//! This module contains the code for circuit checking. It builds a struct and implements the `RecursiveProver` trait from nightfish_CE and from the `ports` module.

use crate::plonk_prover::get_client_proving_key;
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{path::Path, One, Zero};
use jf_plonk::{
    nightfall::{
        ipa_structs::{ProvingKey, VerifyingKey},
        mle::mle_structs::MLEProvingKey,
    },
    proof_system::structs::{ProvingKey as PlonkProvingKey, VerifyingKey as PlonkVerifyingKey},
    recursion::{
        circuits::{Kzg, Zmorph},
        RecursiveProver,
    },
};
use jf_primitives::{
    circuit::{
        sha256::Sha256HashGadget,
        tree::structs::{CircuitInsertionInfoVar, IMTCircuitInsertionInfoVar, MembershipProofVar},
    },
    pcs::prelude::UnivariateKzgPCS,
};
use jf_relation::{errors::CircuitError, BoolVar, Circuit, PlonkCircuit, Variable};
use std::{env, fs::File, io::Write, path::PathBuf, vec};
/// Function that starts at the current working directory and returns the path to the configuration file.
pub fn get_configuration_path() -> Option<PathBuf> {
    let mut cwd = env::current_dir().ok()?;
    loop {
        let file_path = cwd.join("configuration");
        if file_path.is_dir() {
            return Some(file_path);
        }

        cwd = cwd.parent()?.to_path_buf();
    }
}

/// Function that retrieves the deposit proving key from a local file.
pub fn get_deposit_proving_key_locally() -> ProvingKey<UnivariateKzgPCS<Bn254>> {
    let deposit_pk_path = Path::new("./configuration/bin/deposit_proving_key");

    let source_file = find_file_with_path(deposit_pk_path).unwrap();
    ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
        &*std::fs::read(source_file).expect("Could not read proving key"),
    )
    .expect("Could not deserialise proving key")
}
/// Function that searches for a file starting from the current working directory and moving up the directory tree.
pub fn find_file_with_path(path: &Path) -> Option<std::path::PathBuf> {
    if path.is_absolute() {
        match path.is_file() {
            true => return Some(path.to_path_buf()),
            false => return None,
        }
    }

    let cwd = std::env::current_dir().ok()?;
    let mut cwd = cwd.as_path();
    loop {
        let file_path = cwd.join(path);
        if file_path.is_file() {
            return Some(file_path);
        }

        cwd = cwd.parent()?;
    }
}
#[derive(Debug, Clone)]
/// The struct for the rollup circuits checks needed in key generation.
pub struct RollupKeyGenerator;

impl RecursiveProver for RollupKeyGenerator {
    // these checks are implementation of RecursiveProver in Nightfish and will be called by each corresponding circuit
    fn base_bn254_extra_checks(
        specific_pis: &[Variable],
        root_m_proof_length: usize,
        commitment_info_length: usize,
        nullifier_info_length: usize,
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let (first_pis, second_pis) = specific_pis.split_at(specific_pis.len() / 2);

        let mut start_roots_comm = Vec::new();
        let mut start_roots_null = Vec::new();
        let mut end_roots_comm = Vec::new();
        let mut end_roots_null = Vec::new();

        let total_m_proofs_length = 8 * (root_m_proof_length + 2);

        for pi in [first_pis, second_pis] {
            pi[8..]
                .chunks(root_m_proof_length + 1)
                .take(8)
                .zip(pi[..8].iter())
                .try_for_each(|(chunk, leaf_root)| {
                    let m_proof_var =
                        MembershipProofVar::from_vars(circuit, &chunk[..root_m_proof_length])?;

                    let check_var = m_proof_var.verify_membership_proof(
                        circuit,
                        leaf_root,
                        &chunk[root_m_proof_length],
                    )?;
                    circuit.enforce_true(check_var.into())
                })?;

            let circuit_info = CircuitInsertionInfoVar::from_vars(
                circuit,
                &pi[total_m_proofs_length..total_m_proofs_length + commitment_info_length],
                29,
            )?;

            circuit_info.verify_subtree_insertion_gadget(circuit)?;

            start_roots_comm.push(circuit_info.old_root);
            end_roots_comm.push(circuit_info.new_root);

            let nullifier_info = IMTCircuitInsertionInfoVar::from_vars(
                circuit,
                &pi[total_m_proofs_length + commitment_info_length
                    ..total_m_proofs_length + commitment_info_length + nullifier_info_length],
                32,
                8,
            )?;
            nullifier_info.verify_subtree_insertion_gadget(circuit)?;

            start_roots_null.push(nullifier_info.old_root);
            end_roots_null.push(nullifier_info.circuit_info.new_root);
        }

        circuit.enforce_equal(start_roots_comm[1], end_roots_comm[0])?;
        circuit.enforce_equal(start_roots_null[1], end_roots_null[0])?;

        Ok(vec![
            start_roots_comm[0],
            end_roots_comm[1],
            start_roots_null[0],
            end_roots_null[1],
        ])
    }

    fn base_bn254_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let first_pis = &specific_pis[0];
        let second_pis = &specific_pis[1];

        let pi_slices = [
            &first_pis[..18],
            &first_pis[18..],
            &second_pis[..18],
            &second_pis[18..],
        ];
        let fee_sum = pi_slices
            .iter()
            .try_fold(circuit.zero(), |acc, slice| circuit.add(acc, slice[0]))?;

        let mut lookup_vars = Vec::<(Variable, Variable, Variable)>::new();
        let mut sha_vars = Vec::<Variable>::new();
        for pi_slice in pi_slices {
            let field_vars = [
                pi_slice[5],
                pi_slice[6],
                pi_slice[7],
                pi_slice[8],
                pi_slice[9],
                pi_slice[10],
                pi_slice[11],
                pi_slice[12],
                pi_slice[13],
                pi_slice[14],
                pi_slice[15],
                pi_slice[16],
            ];

            let pi_slice_17 = circuit.witness(pi_slice[17])?;
            let bit_var: BoolVar = circuit.create_boolean_variable(pi_slice_17 == Fr254::one())?;
            let (_, sha256_var) = circuit.full_shifted_sha256_hash_with_bit(
                &field_vars,
                &bit_var,
                &mut lookup_vars,
            )?;
            sha_vars.push(sha256_var);
        }

        let mut second_sha_vars = Vec::<Variable>::new();
        for chunk in sha_vars.chunks(2) {
            let (_, sha_var) =
                circuit.full_shifted_sha256_hash(&[chunk[0], chunk[1]], &mut lookup_vars)?;
            second_sha_vars.push(sha_var);
        }

        let (_, final_sha_var) =
            circuit.full_shifted_sha256_hash(&second_sha_vars, &mut lookup_vars)?;

        circuit.finalize_for_sha256_hash(&mut lookup_vars)?;

        Ok(vec![fee_sum, final_sha_var])
    }

    fn base_grumpkin_checks(
        specific_pis: &[Vec<Variable>],
        _circuit: &mut PlonkCircuit<Fq254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        Ok(specific_pis.concat())
    }

    fn bn254_merge_circuit_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let start_root_comm_one = specific_pis[0][0];
        let start_root_comm_two = specific_pis[1][0];
        let end_root_comm_one = specific_pis[0][1];
        let end_root_comm_two = specific_pis[1][1];
        let start_root_null_one = specific_pis[0][2];
        let start_root_null_two = specific_pis[1][2];
        let end_root_null_one = specific_pis[0][3];
        let end_root_null_two = specific_pis[1][3];
        let fee_sum_one = specific_pis[0][4];
        let fee_sum_two = specific_pis[1][4];
        let sha_one = specific_pis[0][5];
        let sha_two = specific_pis[0][6];
        let sha_three = specific_pis[1][5];
        let sha_four = specific_pis[1][6];

        circuit.enforce_equal(end_root_comm_one, start_root_comm_two)?;
        circuit.enforce_equal(end_root_null_one, start_root_null_two)?;

        let fee_sum = circuit.add(fee_sum_one, fee_sum_two)?;
        let mut lookup_vars = Vec::<(Variable, Variable, Variable)>::new();

        let (_, sha_left) =
            circuit.full_shifted_sha256_hash(&[sha_one, sha_two], &mut lookup_vars)?;

        let (_, sha_right) =
            circuit.full_shifted_sha256_hash(&[sha_three, sha_four], &mut lookup_vars)?;

        let (_, final_sha) =
            circuit.full_shifted_sha256_hash(&[sha_left, sha_right], &mut lookup_vars)?;

        circuit.finalize_for_sha256_hash(&mut lookup_vars)?;
        Ok(vec![
            start_root_comm_one,
            end_root_comm_two,
            start_root_null_one,
            end_root_null_two,
            fee_sum,
            final_sha,
        ])
    }

    fn grumpkin_merge_circuit_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fq254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let start_root_comm_one = specific_pis[0][0];
        let start_root_comm_two = specific_pis[1][0];
        let end_root_comm_one = specific_pis[0][1];
        let end_root_comm_two = specific_pis[1][1];
        let start_root_null_one = specific_pis[0][2];
        let start_root_null_two = specific_pis[1][2];
        let end_root_null_one = specific_pis[0][3];
        let end_root_null_two = specific_pis[1][3];
        let fee_sum_one = specific_pis[0][4];
        let fee_sum_two = specific_pis[1][4];
        let sha_one = specific_pis[0][5];
        let sha_two = specific_pis[1][5];

        circuit.enforce_equal(end_root_comm_one, start_root_comm_two)?;
        circuit.enforce_equal(end_root_null_one, start_root_null_two)?;

        let fee_sum = circuit.add(fee_sum_one, fee_sum_two)?;
        Ok(vec![
            start_root_comm_one,
            end_root_comm_two,
            start_root_null_one,
            end_root_null_two,
            fee_sum,
            sha_one,
            sha_two,
        ])
    }

    fn decider_circuit_checks(
        specific_pis: &[Vec<Variable>],
        root_m_proof_length: usize,
        circuit: &mut PlonkCircuit<Fr254>,
        lookup_vars: &mut Vec<(Variable, Variable, Variable)>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let fee_sum_one = specific_pis[0][4];
        let fee_sum_two = specific_pis[1][4];
        let sha_one = specific_pis[0][5];
        let sha_two = specific_pis[0][6];
        let sha_three = specific_pis[1][5];
        let sha_four = specific_pis[1][6];
        let start_root_comm_one = specific_pis[0][0];
        let start_root_comm_two = specific_pis[1][0];
        let end_root_comm_one = specific_pis[0][1];
        let end_root_comm_two = specific_pis[1][1];
        let start_root_null_one = specific_pis[0][2];
        let start_root_null_two = specific_pis[1][2];
        let end_root_null_one = specific_pis[0][3];
        let end_root_null_two = specific_pis[1][3];

        circuit.enforce_equal(end_root_comm_one, start_root_comm_two)?;
        circuit.enforce_equal(end_root_null_one, start_root_null_two)?;

        let fee_sum = circuit.add(fee_sum_one, fee_sum_two)?;

        let (_, sha_left) = circuit.full_shifted_sha256_hash(&[sha_one, sha_two], lookup_vars)?;

        let (_, sha_right) =
            circuit.full_shifted_sha256_hash(&[sha_three, sha_four], lookup_vars)?;

        let (_, final_sha) =
            circuit.full_shifted_sha256_hash(&[sha_left, sha_right], lookup_vars)?;

        circuit.finalize_for_sha256_hash(lookup_vars)?;

        let m_proof_var =
            MembershipProofVar::from_vars(circuit, &specific_pis[2][..root_m_proof_length])?;

        let new_historic_root = m_proof_var.calculate_new_root(circuit, &end_root_comm_two)?;

        let old_root_calc = m_proof_var.calculate_new_root(circuit, &circuit.zero())?;

        circuit.enforce_equal(old_root_calc, specific_pis[2][root_m_proof_length])?;
        Ok(vec![
            fee_sum,
            final_sha,
            start_root_comm_one,
            end_root_comm_two,
            start_root_null_one,
            end_root_null_two,
            specific_pis[2][root_m_proof_length],
            new_historic_root,
        ])
    }

    fn store_base_grumpkin_pk(pk: MLEProvingKey<Zmorph>) -> Option<()> {
        let config_path = get_configuration_path()?;
        let file_path = config_path.join("bin/base_grumpkin_pk");

        let mut buf = Vec::<u8>::new();
        pk.serialize_compressed(&mut buf).ok()?;
        let mut file = File::create(file_path).ok()?;

        file.write_all(&buf).ok()
    }

    fn store_base_bn254_pk(pk: ProvingKey<Kzg>) -> Option<()> {
        let config_path = get_configuration_path()?;
        let file_path = config_path.join("bin/base_bn254_pk");

        let mut buf = Vec::<u8>::new();
        pk.serialize_compressed(&mut buf).ok()?;
        let mut file = File::create(file_path).ok()?;

        file.write_all(&buf).ok()
    }

    fn store_merge_grumpkin_pks(pks: Vec<MLEProvingKey<Zmorph>>) -> Option<()> {
        let config_path = get_configuration_path()?;
        for (i, pk) in pks.into_iter().enumerate() {
            let file_path = config_path.join(format!("bin/merge_grumpkin_pk_{i}"));

            let mut buf = Vec::<u8>::new();
            pk.serialize_compressed(&mut buf).ok()?;

            let mut file = File::create(file_path).ok()?;
            file.write_all(&buf).ok()?;
        }

        Some(())
    }

    fn store_merge_bn254_pks(pks: Vec<ProvingKey<Kzg>>) -> Option<()> {
        let config_path = get_configuration_path()?;
        for (i, pk) in pks.into_iter().enumerate() {
            let file_path: PathBuf = config_path.join(format!("bin/merge_bn254_pk_{i}"));

            let mut buf = Vec::<u8>::new();
            pk.serialize_compressed(&mut buf).ok()?; // serialize the proving key

            let mut file = File::create(file_path).ok()?; // create the file
            file.write_all(&buf).ok()?; // write to the file
        }

        Some(())
    }

    fn store_decider_pk(pk: PlonkProvingKey<Bn254>) -> Option<()> {
        let config_path = get_configuration_path()?;
        let file_path = config_path.join("bin/decider_pk");

        let mut buf = Vec::<u8>::new();
        pk.serialize_compressed(&mut buf).ok()?;
        let mut file = File::create(file_path).ok()?;

        file.write_all(&buf).ok()
    }

    fn store_decider_vk(vk: &PlonkVerifyingKey<Bn254>) {
        let path = get_configuration_path().unwrap().join("bin/decider_vk");
        let mut file = File::create(path).unwrap();
        let mut compressed_bytes = Vec::new();
        vk.serialize_compressed(&mut compressed_bytes).unwrap();
        file.write_all(&compressed_bytes).unwrap();
    }

    fn generate_vk_check_constraint(
        check_hash: Fr254,
        vk_hashes: &[Fr254],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<(), CircuitError> {
        let constant_vars = vk_hashes
            .iter()
            .map(|hash| circuit.create_constant_variable(*hash))
            .collect::<Result<Vec<Variable>, CircuitError>>()?;
        let check_var = circuit.create_variable(check_hash)?;
        let prod = constant_vars
            .iter()
            .try_fold(circuit.one(), |acc, &const_var| {
                circuit.gen_quad_poly(
                    &[acc, check_var, acc, const_var],
                    &[Fr254::zero(); 4],
                    &[Fr254::one(), -Fr254::one()],
                    Fr254::zero(),
                )
            })?;
        circuit.enforce_equal(prod, circuit.zero())
    }
    fn get_vk_list() -> Vec<VerifyingKey<Kzg>> {
        let client_vk = get_client_proving_key().vk.clone();
        let deposit_vk = get_deposit_proving_key_locally().vk.clone();
        vec![client_vk, deposit_vk]
    }

    fn get_base_grumpkin_pk() -> MLEProvingKey<Zmorph> {
        unimplemented!("get_base_grumpkin_pk not needed for key generation API")
    }

    fn get_base_bn254_pk() -> ProvingKey<Kzg> {
        unimplemented!("get_base_bn254_pk not needed for key generation API")
    }

    fn get_merge_grumpkin_pks() -> Vec<MLEProvingKey<Zmorph>> {
        unimplemented!("get_merge_grumpkin_pks not needed for key generation API")
    }

    fn get_merge_bn254_pks() -> Vec<ProvingKey<Kzg>> {
        unimplemented!("get_merge_bn254_pks not needed for key generation API")
    }

    fn get_decider_pk() -> PlonkProvingKey<Bn254> {
        unimplemented!("get_decider_pk not needed for key generation API")
    }

    fn get_decider_vk() -> PlonkVerifyingKey<Bn254> {
        unimplemented!("get_decider_vk not needed for key generation API")
    }
}
