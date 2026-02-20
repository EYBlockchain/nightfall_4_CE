use ark_bn254::{Bn254, Fq as Fq254};
use ark_serialize::{CanonicalSerialize, Write};
use ark_std::rand;
use configuration::settings::{self, Settings};
use jf_plonk::{
    errors::PlonkError,
    nightfall::{ipa_structs::VerificationKeyId, FFTPlonk},
    proof_system::UniversalSNARK,
};
use jf_primitives::{pcs::prelude::*, rescue::sponge::RescueCRHF};
use lib::{
    build_transfer_inputs::build_valid_transfer_inputs,
    circuit_key_generation::{generate_rollup_keys_for_production, universal_setup_for_production},
    constants::MAX_KZG_DEGREE,
    deposit_circuit::deposit_circuit_builder,
    nf_client_proof::PublicInputs,
    plonk_prover::circuits::unified_circuit::unified_circuit_builder,
    shared_entities::DepositData,
};
use std::fs::File;
fn main() {
    let settings: Settings = settings::Settings::new().unwrap();
    if settings.mock_prover {
        println!("Generating keys for MOCK rollup prover");
    } else {
        println!("Generating keys for REAL rollup prover");
    }
    generate_proving_keys(&settings).unwrap();
    println!("Generating keys for rollup prover finished.");
}

/// Generates the proving key and writes it to file.
pub fn generate_proving_keys(settings: &Settings) -> Result<(), PlonkError> {
    // Generate a dummy circuit.
    let (mut public_inputs, mut private_inputs) =
        build_valid_transfer_inputs(&mut ark_std::rand::thread_rng());
    let mut circuit = unified_circuit_builder(&mut public_inputs, &mut private_inputs)?;

    circuit.finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()?;

    let deposit_data = [DepositData::default(); 4];
    let mut deposit_public_inputs = PublicInputs::new();
    let mut deposit_circuit = deposit_circuit_builder(&deposit_data, &mut deposit_public_inputs)?;
    deposit_circuit.finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()?;
    let mut rng = rand::thread_rng();

    // locate the configuration directory
    let path = std::env::current_dir()?.as_path().join("configuration");

    // if we're using a mock prover, we won't waste time downloading a real Perpetual Powers of Tau file
    // and generating a structured reference string
    let kzg_srs = if settings.mock_prover {
        FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(
            1 << MAX_KZG_DEGREE,
            &mut rng,
        )
        .unwrap()
    } else {
        // Unless we already have a local copy, read a remote perpetual powers of Tau file and save, then extract a KZG structured reference string
        universal_setup_for_production(MAX_KZG_DEGREE)
            .expect("Failed to perform universal trusted setup for production.")
    };
    // transfer/withdraw pk vk
    let (pk, _) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
        &kzg_srs,
        Some(VerificationKeyId::Client),
        &circuit,
        true,
    )?;
    // deposit pk vk
    let (deposit_pk, _) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
        &kzg_srs,
        Some(VerificationKeyId::Deposit),
        &deposit_circuit,
        true,
    )?;

    let pk_path = path.join("bin/keys/proving_key");
    ark_std::println!("path for proving_key: {}", pk_path.display());
    let mut file = File::create(pk_path).map_err(PlonkError::IoError)?;
    let mut compressed_bytes = Vec::new();
    pk.serialize_compressed(&mut compressed_bytes)?;
    file.write_all(&compressed_bytes)
        .map_err(PlonkError::IoError)?;

    let deposit_pk_path = path.join("bin/keys/deposit_proving_key");
    ark_std::println!("path for deposit_proving_key: {}", deposit_pk_path.display());
    let mut file = File::create(deposit_pk_path.clone()).map_err(PlonkError::IoError)?;
    let mut deposit_compressed_bytes = Vec::new();
    deposit_pk.serialize_compressed(&mut deposit_compressed_bytes)?;
    file.write_all(&deposit_compressed_bytes)
        .map_err(PlonkError::IoError)?;

    // if we're using a mock prover, we don't need an IPA proof at all, if we are using a real prover then we'll generate a real IPA SRS
    if !settings.mock_prover {
        // this part will generate base_grumpkin_pk, base_bn254_pk, merge_grumpkin_pk, merge_bn254_pk, decider_vk, decider_pk in fn preprocess() located in nightfall_proposer/src/driven/rollup_prover.rs
        generate_rollup_keys_for_production(deposit_circuit, deposit_pk_path, &kzg_srs)?;
    }
    Ok(())
}
