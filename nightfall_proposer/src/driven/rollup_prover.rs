//! This module contains the code for block proving. It builds a struct and implements the `RecursiveProver` trait from nightfish_CE and from the `ports` module.

use crate::{
    domain::entities::{ClientTransactionWithMetaData, DepositData},
    drivers::blockchain::block_assembly::BlockAssemblyError,
    get_deposit_proving_key,
    initialisation::get_db_connection,
    ports::{
        proving::RecursiveProvingEngine,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};

use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{cfg_chunks, cfg_into_iter, cfg_iter, One, Zero};
use itertools::{izip, Itertools};
use jf_plonk::{
    errors::PlonkError,
    nightfall::{
        accumulation::accumulation_structs::AtomicInstance,
        ipa_structs::{ProvingKey, VerifyingKey, VK},
        mle::{mle_structs::MLEProvingKey, MLEPlonk},
        FFTPlonk,
    },
    proof_system::{
        structs::{ProvingKey as PlonkProvingKey, VerifyingKey as PlonkVerifyingKey},
        PlonkKzgSnark, RecursiveOutput, UniversalSNARK,
    },
    recursion::{
        circuits::{Kzg, Zmorph},
        merge_functions::{Bn254CircuitOutput, GrumpkinCircuitOutput},
        RecursiveProof, RecursiveProver,
    },
    transcript::RescueTranscript,
};
use jf_primitives::{
    circuit::{
        sha256::Sha256HashGadget,
        tree::structs::{CircuitInsertionInfoVar, IMTCircuitInsertionInfoVar, MembershipProofVar},
    },
    pcs::prelude::UnivariateKzgPCS,
    rescue::sponge::RescueCRHF,
    trees::{imt::IMTCircuitInsertionInfo, CircuitInsertionInfo, MembershipProof},
};
use jf_relation::{errors::CircuitError, Circuit, PlonkCircuit, Variable};
use log::{debug, warn};
use mongodb::{bson::doc, Client};

use super::deposit_circuit::deposit_circuit_builder;
use lib::{
    merkle_trees::trees::{MerkleTreeError, MutableTree, TreeMetadata},
    serialization::{ark_de_hex, ark_se_hex},
    utils::load_key_from_server,
};
use nightfall_client::{
    domain::error::ConversionError, driven::plonk_prover::plonk_proof::PlonkProof,
    get_client_proving_key, ports::proof::PublicInputs,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{Read, Write},
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    vec,
};

#[derive(Debug)]
pub enum RollupProofError {
    ConversionError(ConversionError),
    SerializationError(SerializationError),
    ProvingError(PlonkError),
    MerkleTreeError(MerkleTreeError<mongodb::error::Error>),
    ParameterError(String),
}

impl Error for RollupProofError {}
impl Display for RollupProofError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RollupProofError::ConversionError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::SerializationError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::ProvingError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::MerkleTreeError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::ParameterError(s) => {
                write!(f, "RollupProofError: ParameterError: {s}")
            }
        }
    }
}

impl From<ConversionError> for RollupProofError {
    fn from(e: ConversionError) -> Self {
        RollupProofError::ConversionError(e)
    }
}

impl From<SerializationError> for RollupProofError {
    fn from(e: SerializationError) -> Self {
        RollupProofError::SerializationError(e)
    }
}

impl From<PlonkError> for RollupProofError {
    fn from(e: PlonkError) -> Self {
        RollupProofError::ProvingError(e)
    }
}

impl From<MerkleTreeError<mongodb::error::Error>> for RollupProofError {
    fn from(e: MerkleTreeError<mongodb::error::Error>) -> Self {
        RollupProofError::MerkleTreeError(e)
    }
}

impl From<RollupProofError> for BlockAssemblyError {
    fn from(e: RollupProofError) -> Self {
        BlockAssemblyError::ProvingError(e.to_string())
    }
}

/// Function that starts at the current working directory and returns the path to the configuration file.
fn get_configuration_path() -> Option<PathBuf> {
    let mut cwd = env::current_dir().ok()?;
    loop {
        let file_path = cwd.join("configuration");
        if file_path.is_dir() {
            return Some(file_path);
        }

        cwd = cwd.parent()?.to_path_buf();
    }
}

/// Function to find an absolute path to a file.
fn find(path: &Path) -> Option<std::path::PathBuf> {
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

/// This function is used to retrieve the base grumpkin proving key.
pub fn get_base_grumpkin_proving_key() -> &'static Arc<MLEProvingKey<Zmorph>> {
    static PK: OnceLock<Arc<MLEProvingKey<Zmorph>>> = OnceLock::new();
    PK.get_or_init(|| {
        // We'll try to load from the configuration directory first.
        if let Some(key_bytes) = load_key_from_server("base_grumpkin_pk") {
            let pk = MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(&*key_bytes)
                .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        // If that fails, we'll try to load from a local file
        warn!("Could not load deposit proving key from server. Loading from local file");
        let path = Path::new("./configuration/bin/base_grumpkin_pk");
        let source_file = find(path).unwrap();
        let pk = MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read proving key"),
        )
        .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
}

/// This function is used to retrieve the base bn254 proving key.
pub fn get_base_bn254_proving_key() -> &'static Arc<ProvingKey<Kzg>> {
    static PK: OnceLock<Arc<ProvingKey<Kzg>>> = OnceLock::new();
    PK.get_or_init(|| {
        if let Some(key_bytes) = load_key_from_server("base_bn254_pk") {
            let pk = ProvingKey::<Kzg>::deserialize_compressed_unchecked(&*key_bytes)
                .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        // If that fails, we'll try to load from a local file
        warn!("Could not load deposit proving key from server. Loading from local file");
        let path = Path::new("./configuration/bin/base_bn254_pk");
        let source_file = find(path).unwrap();
        let pk = ProvingKey::<Kzg>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read proving key"),
        )
        .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
}

/// This function is used to retrieve the merge grumpkin proving key.
pub fn get_merge_grumpkin_proving_key() -> &'static Arc<MLEProvingKey<Zmorph>> {
    static PK: OnceLock<Arc<MLEProvingKey<Zmorph>>> = OnceLock::new();
    PK.get_or_init(|| {
        if let Some(key_bytes) = load_key_from_server("merge_grumpkin_pk") {
            let pk = MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(&*key_bytes)
                .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        // If that fails, we'll try to load from a local file
        warn!("Could not load deposit proving key from server. Loading from local file");
        let path = Path::new("./configuration/bin/merge_grumpkin_pk");
        let source_file = find(path).unwrap();
        let pk = MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read proving key"),
        )
        .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
}

/// This function is used to retrieve the decider proving key.
pub fn get_decider_proving_key() -> &'static Arc<PlonkProvingKey<Bn254>> {
    static PK: OnceLock<Arc<PlonkProvingKey<Bn254>>> = OnceLock::new();
    PK.get_or_init(|| {
        if let Some(key_bytes) = load_key_from_server("decider_pk") {
            let pk = PlonkProvingKey::<Bn254>::deserialize_compressed_unchecked(&*key_bytes)
                .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        let path = Path::new("./configuration/bin/decider_pk");
        let source_file = find(path).unwrap();
        let mut file = std::fs::File::open(source_file).unwrap();
        let file_metadata = file.metadata().unwrap();
        let size = file_metadata.len();
        let mut buf = vec![0u8; size as usize];
        file.read_exact(&mut buf).unwrap();
        let pk = PlonkProvingKey::<Bn254>::deserialize_compressed_unchecked(buf.as_slice())
            .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
}

type GrumpkinOut = (PlonkCircuit<Fq254>, GrumpkinCircuitOutput);
type Bn254Out = (PlonkCircuit<Fr254>, Bn254CircuitOutput);

#[derive(Debug, Clone)]
/// The prover struct for the rollup prover. It contains the vk_hash_list and the key_store.
pub struct RollupProver;

impl RollupProver {
    pub fn get_decider_vk() -> PlonkVerifyingKey<Bn254> {
        let path = Path::new("./configuration/bin/decider_vk");
        let source_file = find(path).unwrap();
        PlonkVerifyingKey::<Bn254>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read verifying key"),
        )
        .expect("Could not deserialise verifying key")
    }

    pub fn store_decider_vk(vk: &PlonkVerifyingKey<Bn254>) {
        let path = get_configuration_path().unwrap().join("bin/decider_vk");
        let mut file = File::create(path).unwrap();
        let mut compressed_bytes = Vec::new();
        vk.serialize_compressed(&mut compressed_bytes).unwrap();
        file.write_all(&compressed_bytes).unwrap();
    }
}

impl RecursiveProver for RollupProver {
    // these checks are implementation of RecursiveProver in Nightfish and will be called by each corresponding circuit
    fn base_bn254_extra_checks(
        specific_pis: &[Variable],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        let (first_pis, second_pis) = specific_pis.split_at(specific_pis.len() / 2);
        let root_m_proof_length =
            BigUint::from(circuit.witness(first_pis[0])?).to_u32_digits()[0] as usize;
        let commitment_info_length =
            BigUint::from(circuit.witness(first_pis[1])?).to_u32_digits()[0] as usize;
        let nullifier_info_length =
            BigUint::from(circuit.witness(first_pis[2])?).to_u32_digits()[0] as usize;

        let mut start_roots_comm = Vec::new();
        let mut start_roots_null = Vec::new();
        let mut end_roots_comm = Vec::new();
        let mut end_roots_null = Vec::new();
        for pi in [first_pis, second_pis] {
            pi[11..]
                .chunks(root_m_proof_length + 1)
                .take(8)
                .zip(pi[3..11].iter())
                .try_for_each(|(chunk, leaf_root)| {
                    let field_elems = chunk[..root_m_proof_length]
                        .iter()
                        .map(|var| circuit.witness(*var))
                        .collect::<Result<Vec<Fr254>, _>>()?;
                    let membership_proof = MembershipProof::<Fr254>::try_from(field_elems)
                        .map_err(|_| {
                            CircuitError::ParameterError(
                                "Could not convert to MembershipProof".to_string(),
                            )
                        })?;
                    let m_proof_var =
                        MembershipProofVar::from_membership_proof(circuit, &membership_proof)?;

                    let check_var = m_proof_var.verify_membership_proof(
                        circuit,
                        leaf_root,
                        &chunk[root_m_proof_length],
                    )?;
                    circuit.enforce_true(check_var.into())
                })?;

            let comm_insert_vec = pi[11 + 8 * (root_m_proof_length + 1)..]
                .iter()
                .take(commitment_info_length)
                .map(|var| circuit.witness(*var))
                .collect::<Result<Vec<Fr254>, _>>()?;
            let comm_insert_info = CircuitInsertionInfo::<Fr254>::from_slice(&comm_insert_vec, 29)
                .map_err(|_| {
                    CircuitError::ParameterError(
                        "Could not convert to CircuitInsertionInfo".to_string(),
                    )
                })?;
            let circuit_info =
                CircuitInsertionInfoVar::from_circuit_insertion_info(circuit, &comm_insert_info)?;
            circuit_info.verify_subtree_insertion_gadget(circuit)?;

            start_roots_comm.push(circuit_info.old_root);
            end_roots_comm.push(circuit_info.new_root);

            let nullifier_insert_vec = pi
                [11 + 8 * (root_m_proof_length + 1) + commitment_info_length..]
                .iter()
                .take(nullifier_info_length)
                .map(|var| circuit.witness(*var))
                .collect::<Result<Vec<Fr254>, _>>()?;
            let nullifier_insert_info =
                IMTCircuitInsertionInfo::<Fr254>::from_slice(&nullifier_insert_vec, 32, 8)
                    .map_err(|_| {
                        CircuitError::ParameterError(
                            "Could not convert to IMTCircuitInsertionInfo".to_string(),
                        )
                    })?;
            let nullifier_info = IMTCircuitInsertionInfoVar::from_imt_circuit_insertion_info(
                circuit,
                &nullifier_insert_info,
            )?;
            nullifier_info.verify_subtree_insertion_gadget(circuit)?;

            start_roots_null.push(nullifier_info.old_root);
            end_roots_null.push(nullifier_info.circuit_info.new_root);
        }
        circuit.enforce_equal(start_roots_comm[1], end_roots_comm[0])?;
        circuit.enforce_equal(start_roots_null[1], end_roots_null[0])?;
        circuit.check_circuit_satisfiability(&[])?;
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
            let (_, sha256_var) = circuit.full_shifted_sha256_hash(
                &[
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
                ],
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
        circuit: &mut PlonkCircuit<Fr254>,
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
        let mut lookup_vars = Vec::<(Variable, Variable, Variable)>::new();

        let (_, sha_left) =
            circuit.full_shifted_sha256_hash(&[sha_one, sha_two], &mut lookup_vars)?;

        let (_, sha_right) =
            circuit.full_shifted_sha256_hash(&[sha_three, sha_four], &mut lookup_vars)?;

        let (_, final_sha) =
            circuit.full_shifted_sha256_hash(&[sha_left, sha_right], &mut lookup_vars)?;

        circuit.finalize_for_sha256_hash(&mut lookup_vars)?;
        let root_m_proof_length =
            BigUint::from(circuit.witness(specific_pis[2][0])?).to_u32_digits()[0] as usize;

        let field_elems = specific_pis[2][1..1 + root_m_proof_length]
            .iter()
            .map(|var| circuit.witness(*var))
            .collect::<Result<Vec<Fr254>, _>>()?;
        let membership_proof = MembershipProof::<Fr254>::try_from(field_elems).map_err(|_| {
            CircuitError::ParameterError("Could not convert to MembershipProof".to_string())
        })?;
        let m_proof_var = MembershipProofVar::from_membership_proof(circuit, &membership_proof)?;

        let new_historic_root = m_proof_var.calculate_new_root(circuit, &end_root_comm_two)?;

        let old_root_calc = m_proof_var.calculate_new_root(circuit, &circuit.zero())?;

        circuit.enforce_equal(old_root_calc, specific_pis[2][1 + root_m_proof_length])?;

        Ok(vec![
            fee_sum,
            final_sha,
            start_root_comm_one,
            end_root_comm_two,
            start_root_null_one,
            end_root_null_two,
            specific_pis[2][1 + root_m_proof_length],
            new_historic_root,
        ])
    }

    fn get_vk_hash_list() -> Vec<Fr254> {
        let client_vk = get_client_proving_key().vk.hash();
        let deposit_vk = get_deposit_proving_key().vk.hash();
        vec![client_vk, deposit_vk]
    }

    fn get_base_grumpkin_pk() -> MLEProvingKey<Zmorph> {
        get_base_grumpkin_proving_key().deref().clone()
    }

    fn get_base_bn254_pk() -> ProvingKey<Kzg> {
        get_base_bn254_proving_key().deref().clone()
    }

    fn get_merge_grumpkin_pk() -> MLEProvingKey<Zmorph> {
        get_merge_grumpkin_proving_key().deref().clone()
    }

    fn get_merge_bn254_pks() -> Vec<ProvingKey<Kzg>> {
        static MERGE_PKS: OnceLock<Vec<Arc<ProvingKey<Kzg>>>> = OnceLock::new();

        MERGE_PKS
            .get_or_init(|| {
                let config_path = get_configuration_path()
                    .expect("Configuration path not found")
                    .join("bin");

                let mut pks = Vec::new();
                let mut i = 0;
                loop {
                    let filename = format!("merge_bn254_pk_{i}");
                    let path: PathBuf = config_path.join(&filename);
                    if let Some(source_file) = find(&path) {
                        let pk = ProvingKey::<Kzg>::deserialize_compressed_unchecked(
                            &*std::fs::read(source_file).expect("Could not read proving key"),
                        )
                        .expect("Could not deserialise proving key");
                        pks.push(Arc::new(pk));
                        i += 1;
                    } else {
                        break;
                    }
                }
                pks
            })
            .iter()
            .map(|arc_pk| (**arc_pk).clone())
            .collect()
    }

    fn get_decider_pk() -> PlonkProvingKey<Bn254> {
        get_decider_proving_key().deref().clone()
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

    fn store_merge_grumpkin_pk(pk: MLEProvingKey<Zmorph>) -> Option<()> {
        let config_path = get_configuration_path()?;
        let file_path = config_path.join("bin/merge_grumpkin_pk");

        let mut buf = Vec::<u8>::new();
        pk.serialize_compressed(&mut buf).ok()?;
        let mut file = File::create(file_path).ok()?;

        file.write_all(&buf).ok()
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

    fn preprocess(
        outputs: &[(Bn254Output, VerifyingKey<Kzg>)],
        specific_pi: &[Vec<Fr254>],
        extra_base_info: &[Vec<Fr254>],
        extra_decider_info: &[Fr254],
        ipa_srs: &jf_plonk::nightfall::UnivariateUniversalIpaParams<nf_curves::grumpkin::Grumpkin>,
        kzg_srs: &jf_primitives::pcs::prelude::UnivariateUniversalParams<Bn254>,
    ) -> Result<(), PlonkError> {
        // First check that we have the same number of outputs and pi's and that they are also non-zero in length
        if outputs.len() != specific_pi.len() {
            return Err(PlonkError::InvalidParameters(format!(
                "The number of outputs: {} does not equal the number of public input lists: {}",
                outputs.len(),
                specific_pi.len()
            )));
        }

        if outputs.is_empty() {
            return Err(PlonkError::InvalidParameters(
                "Need a non-zero number of proofs".to_string(),
            ));
        }

        if 4 * extra_base_info.len() != outputs.len() {
            return Err(PlonkError::InvalidParameters(format!(
                "The number of outputs: {} does not equal 4 times the number of extra base info: {}",
                extra_base_info.len(),
                outputs.len()
            )));
        }

        if (outputs.len().next_power_of_two() != outputs.len()) || (outputs.len().ilog2() % 2 != 0)
        {
            return Err(PlonkError::InvalidParameters(
                "Outputs length is not a power of four".to_string(),
            ));
        }

        let (outputs, vks): (Vec<Bn254Output>, Vec<VerifyingKey<Kzg>>) =
            outputs.iter().map(|(o, c)| (o.clone(), c.clone())).unzip();

        // === Base Layer ===
        let base_grumpkin_out = cfg_chunks!(outputs, 2)
            .zip(cfg_chunks!(specific_pi, 2))
            .zip(cfg_chunks!(vks, 2))
            .map(|((chunk_one, chunk_two), chunk_three)| {
                let out_slice: &[Bn254Output; 2] = chunk_one.try_into().map_err(|_| {
                    PlonkError::InvalidParameters(
                        "Could not convert to fixed length slice".to_string(),
                    )
                })?;
                let pi_slice: &[Vec<Fr254>; 2] = chunk_two.try_into().map_err(|_| {
                    PlonkError::InvalidParameters(
                        "Could not convert to fixed length slice".to_string(),
                    )
                })?;
                let vk_slice: &[VerifyingKey<Kzg>; 2] = chunk_three.try_into().map_err(|_| {
                    PlonkError::InvalidParameters(
                        "Could not convert to fixed length slice".to_string(),
                    )
                })?;
                Self::base_grumpkin_circuit(out_slice, pi_slice, vk_slice, kzg_srs)
            })
            .collect::<Result<Vec<GrumpkinOut>, PlonkError>>()?;

        // We know the outputs is non-zero so we can safely unwrap here
        let base_grumpkin_circuit = &base_grumpkin_out[0].0;
        let base_pi = base_grumpkin_circuit.public_input().unwrap();
        base_grumpkin_circuit.check_circuit_satisfiability(&base_pi)?;
        let (base_grumpkin_pk, _) = MLEPlonk::<Zmorph>::preprocess(ipa_srs, base_grumpkin_circuit)?;
        Self::store_base_grumpkin_pk(base_grumpkin_pk.clone()).ok_or(
            PlonkError::InvalidParameters("Could not store base Grumpkin proving key".to_string()),
        )?;
        // Produce and store the base Bn254 proving key
        let base_grumpkin_chunks: Vec<[GrumpkinOut; 2]> = base_grumpkin_out
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                chunk.collect::<Vec<GrumpkinOut>>().try_into().map_err(|_| {
                    PlonkError::InvalidParameters(
                        "Could not convert to fixed length array".to_string(),
                    )
                })
            })
            .collect::<Result<Vec<[GrumpkinOut; 2]>, PlonkError>>()?;
        let vk_chunks = vks
            .into_iter()
            .chunks(4)
            .into_iter()
            .map(|chunk| {
                chunk
                    .collect::<Vec<VerifyingKey<Kzg>>>()
                    .try_into()
                    .map_err(|_| {
                        PlonkError::InvalidParameters(
                            "Could not convert to fixed length array".to_string(),
                        )
                    })
            })
            .collect::<Result<Vec<[VerifyingKey<Kzg>; 4]>, PlonkError>>()?;
        let base_bn254_out = cfg_into_iter!(base_grumpkin_chunks)
            .zip(cfg_into_iter!(vk_chunks))
            .zip(cfg_into_iter!(extra_base_info))
            .map(|((chunk, vk_chunk), extra_info)| {
                Self::base_bn254_circuit(chunk, &base_grumpkin_pk, &vk_chunk, extra_info)
            })
            .collect::<Result<Vec<Bn254Out>, PlonkError>>()?;
        let base_bn254_circuit = &base_bn254_out[0].0;
        let base_pi = base_bn254_circuit.public_input().unwrap();
        base_bn254_circuit.check_circuit_satisfiability(&base_pi)?;
        let (base_bn254_pk, _) = FFTPlonk::<Kzg>::preprocess(kzg_srs, base_bn254_circuit)?;
        Self::store_base_bn254_pk(base_bn254_pk.clone()).ok_or(PlonkError::InvalidParameters(
            "Could not store base Bn254 proving key".to_string(),
        ))?;

          // Rollup structure:
        // base_grumpkin_circuit   ← always present
        // base_bn254_circuit      ← always present
        // [...]                     ← variable intermediate merge layers
        // merge_grumpkin_circuit  ← always present (final merge before decider)
        // decider_circuit         ← always present

        // For 64 transactions:
        // base_grumpkin
        // base_bn254
        // [merge_grumpkin]
        // [merge_bn254]
        // merge_grumpkin ← fixed
        // decider
        // → Only 1 pair of merge_{grumpkin, bn254} before the fixed final merge_grumpkin.

        // For 256 transactions:
        // base_grumpkin
        // base_bn254
        // [merge_grumpkin]
        // [merge_bn254]
        // [merge_grumpkin]
        // [merge_bn254]
        // merge_grumpkin ← fixed
        // decider
        // → 2 pairs of intermediate merge layers, followed by the final fixed merge_grumpkin.

        // === Generalized Merge ===
        let mut current_bn254_out = base_bn254_out;
        let mut current_bn254_pk = base_bn254_pk;
        let mut current_grumpkin_pk = base_grumpkin_pk;
        let mut merge_bn254_pks = vec![];
        let intermediate_group = (outputs.len().ilog2() - 4) / 2;

        for _ in 0..intermediate_group {
            // 1. Merge Bn254 → Grumpkin
            let bn254_chunks: Vec<[Bn254Out; 2]> = current_bn254_out
                .into_iter()
                .chunks(2)
                .into_iter()
                .map(|c| {
                    c.collect::<Vec<_>>().try_into().map_err(|_| {
                        PlonkError::InvalidParameters(
                            "Could not convert Bn254Out chunk to array".to_string(),
                        )
                    })
                })
                .collect::<Result<_, _>>()?;

            let grumpkin_out = cfg_into_iter!(bn254_chunks)
                .map(|chunk| {
                    Self::merge_grumpkin_circuit(chunk, &current_bn254_pk, &current_grumpkin_pk)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let grumpkin_circuit = &grumpkin_out[0].0;
            let (new_grumpkin_pk, _) = MLEPlonk::<Zmorph>::preprocess(ipa_srs, grumpkin_circuit)?;
            current_grumpkin_pk = new_grumpkin_pk;

            // 2. Merge Grumpkin → Bn254
            let grumpkin_chunks: Vec<[GrumpkinOut; 2]> = grumpkin_out
                .into_iter()
                .chunks(2)
                .into_iter()
                .map(|c| {
                    c.collect::<Vec<_>>().try_into().map_err(|_| {
                        PlonkError::InvalidParameters(
                            "Could not convert GrumpkinOut chunk to array".to_string(),
                        )
                    })
                })
                .collect::<Result<_, _>>()?;

            current_bn254_out = cfg_into_iter!(grumpkin_chunks)
                .map(|chunk| {
                    Self::merge_bn254_circuit(chunk, &current_grumpkin_pk, &current_bn254_pk)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let bn254_circuit = &current_bn254_out[0].0;
            let (new_bn254_pk, _) = FFTPlonk::<Kzg>::preprocess(kzg_srs, bn254_circuit)?;
            merge_bn254_pks.push(new_bn254_pk.clone());
            current_bn254_pk = new_bn254_pk;
        }
        // Now we need to run merge grumpkin one more time
        let merge_bn254_chunks: Vec<[Bn254Out; 2]> = current_bn254_out
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                chunk.collect::<Vec<Bn254Out>>().try_into().map_err(|_| {
                    PlonkError::InvalidParameters(
                        "Could not convert to fixed length array".to_string(),
                    )
                })
            })
            .collect::<Result<Vec<[Bn254Out; 2]>, PlonkError>>()?;
        let decider_input = cfg_into_iter!(merge_bn254_chunks)
            .map(|chunk| Self::merge_grumpkin_circuit(chunk, &current_bn254_pk, &current_grumpkin_pk))
            .collect::<Result<Vec<GrumpkinOut>, PlonkError>>()?;

        // Check the length is exactly 2
        if decider_input.len() != 2 {
            return Err(PlonkError::InvalidParameters(format!(
                "Decider function input should be length 2, it has length: {}",
                decider_input.len()
            )));
        }
        let decider_input_exact: [GrumpkinOut; 2] =
            [decider_input[0].clone(), decider_input[1].clone()];

        let decider_out = Self::decider_circuit(
            decider_input_exact,
            extra_decider_info,
            &current_grumpkin_pk,
            &current_bn254_pk,
        )?;

        let (decider_pk, decider_vk) = PlonkKzgSnark::<Bn254>::preprocess(kzg_srs, &decider_out.circuit)?;

        Self::store_merge_grumpkin_pk(current_grumpkin_pk).ok_or(PlonkError::InvalidParameters(
            "Could not store merge Grumpkin proving key".to_string(),
        ))?;

        Self::store_merge_bn254_pks(merge_bn254_pks).ok_or(PlonkError::InvalidParameters(
            "Could not store merge Bn254 proving keys".to_string(),
        ))?;

        RollupProver::store_decider_vk(&decider_vk);

        Self::store_decider_pk(decider_pk).ok_or(PlonkError::InvalidParameters(
            "Could not store decider proving key".to_string(),
        ))
    }
}

/// This struct is used for the recursive proving of the rollup prover.
/// It is the result of running the `prepare_state_transition` function.
/// 
/// 
#[derive(Debug)]
pub struct RollupPreppedInfo {
    pub outputs_and_circuit_type: Vec<(Bn254Output, VerifyingKey<Kzg>)>,
    pub specific_pi: Vec<Vec<Fr254>>,
    pub extra_info: Vec<Vec<Fr254>>,
    pub extra_decider_info: Vec<Fr254>,
}

/// The output of the rollup prover, it contains the UltraPlonk proof and the KZG accumulators.
#[derive(Debug, Clone, Serialize, Deserialize, CanonicalSerialize, CanonicalDeserialize)]
pub struct RollupProof {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee_sum: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub accumulator_one: [Fq254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub accumulator_two: [Fq254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub proof: Vec<Fq254>,
}

impl From<RecursiveProof> for RollupProof {
    fn from(proof: RecursiveProof) -> Self {
        let RecursiveProof {
            proof,
            accumulators,
            pi,
        } = proof;
        let [AtomicInstance {
            comm: comm_1,
            opening_proof: op_1,
            ..
        }, AtomicInstance {
            comm: comm_2,
            opening_proof: op_2,
            ..
        }] = accumulators;

        RollupProof {
            fee_sum: pi[0],
            accumulator_one: [comm_1.x, comm_1.y, op_1.proof.x, op_1.proof.y],
            accumulator_two: [comm_2.x, comm_2.y, op_2.proof.x, op_2.proof.y],
            proof: Vec::<Fq254>::from(proof),
        }
    }
}

impl From<RollupProof> for Vec<Fq254> {
    fn from(proof: RollupProof) -> Self {
        let RollupProof {
            fee_sum,
            accumulator_one,
            accumulator_two,
            proof,
        } = proof;
        let mut vec = vec![Fq254::from_le_bytes_mod_order(
            &fee_sum.into_bigint().to_bytes_le(),
        )];
        vec.extend_from_slice(&accumulator_one);
        vec.extend_from_slice(&accumulator_two);
        vec.extend_from_slice(&proof);
        vec
    }
}

pub(crate) type Bn254Output = RecursiveOutput<Kzg, FFTPlonk<Kzg>, RescueTranscript<Fr254>>;

impl RecursiveProvingEngine<PlonkProof> for RollupProver {
    type PreppedInfo = RollupPreppedInfo;

    type Error = RollupProofError;

    type RecursiveProof = RollupProof;

    async fn prepare_state_transition(
        deposit_transactions: &[(PlonkProof, PublicInputs)],
        transactions: &[ClientTransactionWithMetaData<PlonkProof>],
    ) -> Result<(Self::PreppedInfo, [Fr254; 3]), Self::Error> {
        // We retrieve both types of proving keys
        let deposit_pk = get_deposit_proving_key();
        let client_pk = get_client_proving_key();

        // First lets get all the public inputs from the deposit transactions and the client transactions
        // RecursiveOutput {
        //     proof,
        //     pi_hash,
        //     transcript,
        // }
        // get <outputs_and_circuit_type({proofs, pi_hashes, transcriptes},vks), public inputs> from the deposit transactions and client transactions
        let (outputs_and_circuit_type, public_inputs): (
            Vec<(Bn254Output, VerifyingKey<Kzg>)>,
            Vec<PublicInputs>,
        ) = cfg_iter!(deposit_transactions)
            .map(|(proof, pi)| {
                let output = RecursiveOutput::try_from(proof.clone())?;
                Result::<_, PlonkError>::Ok((output, deposit_pk.vk.clone(), *pi))
            })
            .chain(cfg_iter!(transactions).map(|tx| {
                let output = RecursiveOutput::try_from(tx.client_transaction.proof.clone())?;
                Result::<_, PlonkError>::Ok((
                    output,
                    client_pk.vk.clone(),
                    PublicInputs::from(&tx.client_transaction),
                ))
            }))
            .collect::<Result<Vec<_>, PlonkError>>()?
            .into_iter()
            .map(|(output, vk, pi)| ((output, vk), pi))
            .unzip();

        // Get all the commitments and nullifiers from the public inputs
        // Flattens all commitments and nullifiers from the public inputs into vectors.
        let new_commitments = public_inputs
            .iter()
            .flat_map(|pi| pi.commitments)
            .collect::<Vec<Fr254>>();

        let insert_nullifiers = public_inputs
            .iter()
            .flat_map(|pi| pi.nullifiers)
            .collect::<Vec<Fr254>>();

        // work out what the new historic root would be if we were to add these new commitments
        let db = get_db_connection().await;

        // get the current historic root
        let current_historic_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await?;
        // Create the commitments circuit info
        debug!("Inserting commitments");
        let commitment_circuit_info =
            <Client as CommitmentTree<Fr254>>::batch_insert_with_circuit_info(db, &new_commitments)
                .await?;
        // Create the nullifier circuit info
        debug!("Inserting nullifiers");
        let nullifier_circuit_info =
            <Client as NullifierTree<Fr254>>::batch_insert_with_circuit_info(
                db,
                &insert_nullifiers,
            )
            .await?;
        // use the final commitment circuit info to get the new root of the commitment tree.
        let new_commitment_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as CommitmentTree<Fr254>>::TREE_NAME,
        )
        .await?;

        // We also need check each of the roots in the client proofs is valid so we construct the membership proofs for them here.
        let mut root_proofs = HashMap::<Fr254, Vec<Fr254>>::new();
        let mut root_membership_proofs = Vec::<Vec<Fr254>>::new();
        let mut root_m_proof_len = 0;
        for pi in public_inputs.iter() {
            let mut m_proofs = Vec::<Fr254>::new();
            for root in pi.roots.iter() {
                if !root_proofs.contains_key(root) {
                    let proof = <Client as HistoricRootTree<Fr254>>::get_membership_proof(
                        db,
                        Some(root),
                        None,
                    )
                    .await?;
                    let mut proof_vec = Vec::<Fr254>::from(proof);
                    root_m_proof_len = proof_vec.len();
                    proof_vec.push(current_historic_root);
                    root_proofs.insert(*root, proof_vec.clone());
                    m_proofs.extend(proof_vec.iter());
                } else {
                    let proof_vec =
                        root_proofs
                            .get(root)
                            .ok_or(RollupProofError::ParameterError(
                                "Error retrieving Historic root Membership proof from temporary DB"
                                    .to_string(),
                            ))?;
                    m_proofs.extend(proof_vec.iter());
                }
            }
            root_membership_proofs.push(m_proofs);
        }

        let nullifier_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as NullifierTree<Fr254>>::TREE_NAME,
        )
        .await?;

        // work out what the new historic root tree root would be if we were to add this new historic root
        let old_historic_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await?;

        let metadata_collection_name = format!(
            "{}_{}",
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
            "metadata"
        );
        let metadata_collection = db
            .database(<Client as MutableTree<Fr254>>::MUT_DB_NAME)
            .collection::<TreeMetadata<Fr254>>(&metadata_collection_name);
        let metadata: TreeMetadata<Fr254> = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let updated_historic_root =
            <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
                db,
                &new_commitment_root,
                false,
            )
            .await?;

        // Historic Root Membership Proof
        let historic_root_proof = <Client as HistoricRootTree<Fr254>>::get_membership_proof(
            db,
            None,
            Some(metadata.sub_tree_count),
        )
        .await?;
        let root_proof_len_field = Fr254::from(root_m_proof_len as u64);

        //Zips together chunks of public inputs, membership proofs, and circuit info to build the extra info needed for the recursive circuits.
        // Structure of extra_info: Vec<Vec<Fr254>>
        // The outer Vec: Each element represents a chunk (typically a group of 4 transactions, matching the recursion tree's arity).
        // The inner Vec<Fr254>: This is not just 4 elements!
        // It is a flattened, concatenated vector containing all the auxiliary data needed for that chunk of transactions.
        // This includes:
        // Length fields (for parsing)
        // Roots for the transactions in the chunk
        // Membership proofs for those roots
        // Commitment insertion info
        // Nullifier insertion info

        let extra_info = izip!(
            public_inputs.chunks(4),
            root_membership_proofs.chunks(4),
            commitment_circuit_info.chunks(2),
            nullifier_circuit_info.into_iter().chunks(2).into_iter()
        )
        .map(
            |(pis, root_m_proof_chunk, commitment_info, nullifier_info)| {
                let commitment_info_vec_0 = Vec::<Fr254>::from(commitment_info[0].clone());
                let commitment_info_vec_1 = Vec::<Fr254>::from(commitment_info[1].clone());
                let nullifier_info_vecs = nullifier_info
                    .into_iter()
                    .map(|info| info.into())
                    .collect::<Vec<Vec<Fr254>>>();
                let commitment_info_len = Fr254::from(commitment_info_vec_0.len() as u64);
                let nullifier_info_len = Fr254::from(nullifier_info_vecs[0].len() as u64);
                [
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[0].roots, pis[1].roots].concat(),
                    root_m_proof_chunk[0]
                        .iter()
                        .chain(root_m_proof_chunk[1].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_0,
                    nullifier_info_vecs[0].clone(),
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[2].roots, pis[3].roots].concat(),
                    root_m_proof_chunk[2]
                        .iter()
                        .chain(root_m_proof_chunk[3].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_1,
                    nullifier_info_vecs[1].clone(),
                ]
                .concat()
            },
        )
        .collect::<Vec<Vec<Fr254>>>();

        // Format Public Inputs for Circuits
        // Converts each public input into a vector of field elements for use in the circuits.
        let specific_pi = public_inputs
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<Fr254>>>();
        // Prepares the extra info for the decider circuit, including the historic_root_proof and old_historic_root.
        let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
        let historic_root_proof_length = Fr254::from(extra_info_vec.len() as u64);
        extra_info_vec.insert(0, historic_root_proof_length);
        extra_info_vec.push(old_historic_root);
        // 1. RollupPreppedInfo
        // This struct contains all the data needed to generate the recursive rollup proof for a block. It includes:

        // outputs_and_circuit_type:
        // A vector of tuples, each containing:

        // The recursive proof output for a transaction (used in recursive aggregation).
        // The verifying key for the circuit that produced the proof. This is used to verify and aggregate all the individual transaction proofs into a single block proof.
        // specific_pi:
        // A vector of vectors, where each inner vector contains the public inputs for a transaction, formatted as field elements.
        // These are the public data (like commitments, nullifiers, roots, etc.) that the circuits use as inputs.

        // extra_info:
        // A vector of vectors, each containing additional circuit-specific data needed for the recursive circuits.
        // This typically includes membership proofs, insertion info, and other auxiliary data required by the circuits to validate the state transitions.

        // extra_decider_info:
        // A vector containing extra data for the final "decider" circuit, such as the historic root membership proof and the old root.
        // This is used in the final aggregation step to ensure the block’s state transition is valid.
        Ok((
            RollupPreppedInfo {
                outputs_and_circuit_type,
                specific_pi,
                extra_info,
                extra_decider_info: extra_info_vec,
            },
            [new_commitment_root, nullifier_root, updated_historic_root],
        ))
    }

    fn recursive_prove(info: Self::PreppedInfo) -> Result<Self::RecursiveProof, Self::Error> {
        Ok(RollupProof::from(
            <RollupProver as RecursiveProver>::prove(
                &info.outputs_and_circuit_type,
                &info.specific_pi,
                &info.extra_decider_info,
                &info.extra_info,
            )
            .map_err(RollupProofError::from)?,
        ))
    }

    fn create_deposit_proof(
        deposit_data: &[DepositData; 4],
        public_inputs: &mut PublicInputs,
    ) -> Result<PlonkProof, Self::Error> {
        let mut circuit =
            deposit_circuit_builder(deposit_data, public_inputs).map_err(PlonkError::from)?;
        circuit
            .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
            .map_err(PlonkError::from)?;
        let pk = get_deposit_proving_key();

        let output = FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<
            _,
            _,
            RescueTranscript<Fr254>,
        >(&mut ark_std::rand::thread_rng(), &circuit, pk, None)?;
        Ok(PlonkProof::from_recursive_output(output, &pk.vk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::Zero;
    use jf_plonk::nightfall::mle::MLEPlonk;
    use jf_primitives::{
        poseidon::Poseidon,
        trees::{
            imt::{IndexedMerkleTree, LeafDBEntry},
            timber::Timber,
        },
    };
    use std::collections::HashMap;
    #[test]
    #[ignore = "Very long test"]
    fn test_preprocess() {
        let mut rng = ark_std::rand::thread_rng();

        let ipa_srs = MLEPlonk::<Zmorph>::universal_setup_for_testing(1 << 18, &mut rng).unwrap();
        let kzg_srs =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(1 << 25, &mut rng)
                .unwrap();

        let mut d_proofs = Vec::new();
        let mut public_input_vec = Vec::new();
        let mut public_inputs = PublicInputs::new();
        let deposit_array = [DepositData::default(); 4];

        let mut circuit = deposit_circuit_builder(&deposit_array, &mut public_inputs).unwrap();
        circuit
            .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
            .unwrap();
        let deposit_pk = get_deposit_proving_key();

        let output = FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<
            _,
            _,
            RescueTranscript<Fr254>,
        >(&mut ark_std::rand::thread_rng(), &circuit, deposit_pk, None)
        .unwrap();

        (0..64).for_each(|_| {
            d_proofs.push((output.clone(), deposit_pk.vk.clone()));
            public_input_vec.push(public_inputs);
        });
        // We need to make dummy trees for to build circuit insertion info.
        let poseidon = Poseidon::<Fr254>::new();
        let mut timber: Timber<Fr254, Poseidon<Fr254>> =
            Timber::<Fr254, Poseidon<Fr254>>::new(poseidon, 32);
        let mut imt: IndexedMerkleTree<Fr254, Poseidon<Fr254>, _> =
            IndexedMerkleTree::<Fr254, Poseidon<Fr254>, HashMap<Fr254, LeafDBEntry<Fr254>>>::new(
                poseidon, 32,
            )
            .unwrap();
        let mut historic_root_tree: Timber<Fr254, Poseidon<Fr254>> =
            Timber::<Fr254, Poseidon<Fr254>>::new(poseidon, 32);

        // Get all the commitments and nullifiers from the public inputs
        let new_commitments = public_input_vec
            .iter()
            .flat_map(|pi| pi.commitments)
            .collect::<Vec<Fr254>>();

        let insert_nullifiers = public_input_vec
            .iter()
            .flat_map(|pi| pi.nullifiers)
            .collect::<Vec<Fr254>>();

        historic_root_tree.insert_leaf(Fr254::zero()).unwrap();

        let commitment_circuit_info = timber.batch_insert_for_circuit(&new_commitments).unwrap();

        let nullifier_circuit_info = imt.batch_insert_for_circuit(&insert_nullifiers).unwrap();

        let path = historic_root_tree
            .get_sibling_path(Fr254::zero(), 0)
            .unwrap();

        let m_proof = MembershipProof::<Fr254> {
            node_value: Fr254::zero(),
            sibling_path: path,
            leaf_index: 0,
        };

        let mut m_proof_vec = Vec::<Fr254>::from(m_proof);
        let root_proof_len_field = Fr254::from(m_proof_vec.len() as u64);
        m_proof_vec.push(public_inputs.roots[0]);
        let root_m_proofs_inner = vec![m_proof_vec.clone(); 4].concat();
        let root_membership_proofs = vec![root_m_proofs_inner.clone(); 64];

        let extra_base_info = izip!(
            public_input_vec.chunks(4),
            root_membership_proofs.chunks(4),
            commitment_circuit_info.chunks(2),
            nullifier_circuit_info.chunks(2)
        )
        .map(
            |(pis, root_m_proof_chunk, commitment_info, nullifier_info)| {
                let commitment_info_vec_0 = Vec::<Fr254>::from(commitment_info[0].clone());
                let commitment_info_vec_1 = Vec::<Fr254>::from(commitment_info[1].clone());
                let nullifier_info_vec_0: Vec<Fr254> = nullifier_info[0].clone().into();
                let nullifier_info_vec_1: Vec<Fr254> = nullifier_info[1].clone().into();
                let commitment_info_len = Fr254::from(commitment_info_vec_0.len() as u64);
                let nullifier_info_len = Fr254::from(nullifier_info_vec_0.len() as u64);
                [
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[0].roots, pis[1].roots].concat(),
                    root_m_proof_chunk[0]
                        .iter()
                        .chain(root_m_proof_chunk[1].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_0,
                    nullifier_info_vec_0,
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[2].roots, pis[3].roots].concat(),
                    root_m_proof_chunk[2]
                        .iter()
                        .chain(root_m_proof_chunk[3].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_1,
                    nullifier_info_vec_1,
                ]
                .concat()
            },
        )
        .collect::<Vec<Vec<Fr254>>>();

        let specific_pi = public_input_vec
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<Fr254>>>();

        let new_commitment_root = timber.root;

        let old_historic_root = historic_root_tree.root;

        historic_root_tree.insert_leaf(new_commitment_root).unwrap();

        let historic_root_path = historic_root_tree
            .get_sibling_path(new_commitment_root, 1)
            .unwrap();

        let historic_root_proof = MembershipProof::<Fr254> {
            node_value: new_commitment_root,
            sibling_path: historic_root_path,
            leaf_index: 1,
        };

        let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
        extra_info_vec.insert(0, root_proof_len_field);
        extra_info_vec.push(old_historic_root);

        RollupProver::preprocess(
            &d_proofs,
            &specific_pi,
            &extra_base_info,
            &extra_info_vec,
            &ipa_srs,
            &kzg_srs,
        )
        .unwrap();
    }
}