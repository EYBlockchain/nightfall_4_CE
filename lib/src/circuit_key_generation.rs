use crate::{
    deposit_circuit::deposit_circuit_builder,
    entities::DepositData,
    nf_client_proof::PublicInputs,
    rollup_circuit_checks::{find_file_with_path, RollupKeyGenerator},
    utils::get_block_size,
};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254, FrConfig};
use ark_ec::bn::Bn;
use ark_ff::{Fp, MontBackend};
use ark_serialize::CanonicalDeserialize;
use ark_std::{path::PathBuf, Zero};
use itertools::izip;
use jf_plonk::{
    errors::PlonkError,
    nightfall::{ipa_structs::ProvingKey, FFTPlonk, UnivariateUniversalIpaParams},
    proof_system::UniversalRecursiveSNARK,
    recursion::RecursiveProver,
    transcript::RescueTranscript,
};
use jf_primitives::{
    pcs::prelude::*,
    poseidon::Poseidon,
    rescue::sponge::RescueCRHF,
    trees::{
        imt::{IndexedMerkleTree, LeafDBEntry},
        timber::Timber,
        MembershipProof,
    },
};
use jf_relation::PlonkCircuit;
use std::collections::HashMap;

pub fn universal_setup_for_production(
    max_kzg_degree: usize,
) -> Result<UnivariateUniversalParams<Bn<ark_bn254::Config>>, PCSError> {
    // locate the configuration directory
    let path = std::env::current_dir()
        .expect("Failed to get current directory")
        .as_path()
        .join("configuration");
    // Download perpetual powers of Tau file if not cached locally, then extract a KZG structured reference string from cached setup if it exists, otherwise create it
    let ptau_file = path.join(format!("bin/ppot_{max_kzg_degree}.ptau"));
    UnivariateKzgPCS::download_ptau_file_if_needed(max_kzg_degree, &ptau_file)
        .expect("Failed to download ptau file");
    let cache_file = path.join(format!("bin/bn254_setup_{max_kzg_degree}.cache"));
    UnivariateKzgPCS::universal_setup_bn254_cached(&ptau_file, 1 << max_kzg_degree, &cache_file)
}

pub fn generate_rollup_keys_for_production(
    deposit_circuit: PlonkCircuit<Fp<MontBackend<FrConfig, 4>, 4>>,
    deposit_pk_path: PathBuf,
    kzg_srs: &UnivariateUniversalParams<Bn<ark_bn254::Config>>,
) -> Result<(), PlonkError> {
    let ipa_srs = UnivariateUniversalIpaParams::gen_srs("Nightfall_4", 1 << 18).unwrap();

    let mut d_proofs = Vec::new();
    let mut public_input_vec = Vec::new();

    let source_file = find_file_with_path(&deposit_pk_path).unwrap();
    let deposit_pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
        &*std::fs::read(source_file).expect("Could not read proving key"),
    )
    .expect("Could not deserialise proving key");

    let output =
        FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<_, _, RescueTranscript<Fr254>>(
            &mut ark_std::rand::thread_rng(),
            &deposit_circuit,
            &deposit_pk,
            None,
            true,
        )
        .unwrap();

    let block_size = match get_block_size() {
        Ok(size) => size,
        Err(e) => {
            log::warn!("Falling back to default block size 64 due to error: {e:?}");
            64
        }
    };

    let deposit_data = [DepositData::default(); 4];
    let mut deposit_public_inputs = PublicInputs::new();
    let mut deposit_circuit = deposit_circuit_builder(&deposit_data, &mut deposit_public_inputs)?;
    deposit_circuit.finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()?;

    (0..block_size).for_each(|_| {
        d_proofs.push((output.clone(), deposit_pk.vk.clone()));
        public_input_vec.push(deposit_public_inputs);
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
    m_proof_vec.push(deposit_public_inputs.roots[0]);
    let root_m_proofs_inner = vec![m_proof_vec.clone(); 4].concat();
    let root_membership_proofs = vec![root_m_proofs_inner.clone(); block_size];

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
        .ok_or(PlonkError::InvalidParameters(
            "Error with historic root path".to_string(),
        ))?;

    let historic_root_proof = MembershipProof::<Fr254> {
        node_value: new_commitment_root,
        sibling_path: historic_root_path,
        leaf_index: 1,
    };

    let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
    let historic_root_proof_length = Fr254::from(extra_info_vec.len() as u64);
    extra_info_vec.insert(0, historic_root_proof_length);
    extra_info_vec.push(old_historic_root);

    RollupKeyGenerator::preprocess(
        &d_proofs,
        &specific_pi,
        &extra_base_info,
        &extra_info_vec,
        &ipa_srs,
        kzg_srs,
    )?;
    Ok(())
}
