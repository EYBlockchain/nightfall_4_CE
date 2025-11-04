use alloy::primitives::{hex, keccak256};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_serialize::{CanonicalSerialize, Write};
use ark_std::{
    rand::{self, Rng},
    UniformRand,
};
use configuration::settings::{self, Settings};
use jf_plonk::{
    errors::PlonkError,
    nightfall::{ipa_structs::VerificationKeyId, FFTPlonk},
    proof_system::UniversalSNARK,
};
use jf_primitives::{
    pcs::prelude::*,
    poseidon::Poseidon,
    rescue::sponge::RescueCRHF,
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use lib::{
    circuit_key_generation::{generate_rollup_keys_for_production, universal_setup_for_production},
    constants::MAX_KZG_DEGREE,
    deposit_circuit::deposit_circuit_builder,
    entities::DepositData,
    hex_conversion::HexConvertible,
    nf_client_proof::{PrivateInputs, PublicInputs},
    nf_token_id::to_nf_token_id_from_str,
    plonk_prover::circuits::unified_circuit::unified_circuit_builder,
};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};
use nightfall_client::{
    domain::entities::{DepositSecret, Preimage, Salt},
    drivers::derive_key::ZKPKeys,
    ports::{commitments::Commitment, secret_hash::SecretHash},
};
use num_bigint::BigUint;
use std::fs::File;
fn main() {
    let settings: Settings = settings::Settings::new().unwrap();
    if settings.mock_prover {
        println!("Generating keys for MOCK rollup prover");
    } else {
        println!("Generating keys for REAL rollup prover");
    }
    generate_proving_keys(&settings).unwrap();
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
    )?;
    // deposit pk vk
    let (deposit_pk, _) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
        &kzg_srs,
        Some(VerificationKeyId::Deposit),
        &deposit_circuit,
    )?;

    let pk_path = path.join("bin/proving_key");
    let mut file = File::create(pk_path).map_err(PlonkError::IoError)?;
    let mut compressed_bytes = Vec::new();
    pk.serialize_compressed(&mut compressed_bytes)?;
    file.write_all(&compressed_bytes)
        .map_err(PlonkError::IoError)?;

    let deposit_pk_path = path.join("bin/deposit_proving_key");

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

fn generate_random_path(leaf_value: Fr254, rng: &mut impl Rng) -> (MembershipProof<Fr254>, Fr254) {
    let mut root = leaf_value;
    let poseidon = Poseidon::<Fr254>::new();
    let leaf_index = u32::rand(rng);
    let mut path_elements = Vec::<PathElement<Fr254>>::new();
    for i in 0..32 {
        let dir = leaf_index >> i & 1;
        let value = Fr254::rand(rng);
        if dir == 0 {
            root = poseidon.tree_hash(&[root, value]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnRight,
                value,
            })
        } else {
            root = poseidon.tree_hash(&[value, root]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnLeft,
                value,
            })
        }
    }

    (
        MembershipProof {
            node_value: leaf_value,
            sibling_path: path_elements,
            leaf_index: leaf_index as usize,
        },
        root,
    )
}

// Creates a random 96 bit element of Fr254
fn rand_96_bit(rng: &mut impl Rng) -> Fr254 {
    let random_96_bit = u128::rand(rng) % (1u128 << 96);
    Fr254::from(random_96_bit)
}

pub fn build_valid_transfer_inputs(rng: &mut impl Rng) -> (PublicInputs, PrivateInputs) {
    let mut rng_erc_address = rand::thread_rng();
    let erc_address: [u8; 20] = rng_erc_address.gen();
    let erc_address_string = format!("0x{}", hex::encode(erc_address));
    let mut rng_token_id = ark_std::rand::thread_rng();
    let token_id_fr = Fr254::rand(&mut rng_token_id);
    let token_id_string = Fr254::to_hex_string(&token_id_fr);

    let nf_token_id = to_nf_token_id_from_str(&erc_address_string, &token_id_string).unwrap();
    let nf_slot_id = nf_token_id;

    // generate a 'random' fee token ID (we just use the keccak hash of 1)
    let fee_token_id = Fr254::from(BigUint::from_bytes_be(keccak256([1]).as_slice()) >> 4);

    // Random values for fee and value
    let mut nullified_fee_one = rand_96_bit(rng);
    let mut nullified_fee_two = rand_96_bit(rng);
    let mut nullified_value_one = rand_96_bit(rng);
    let mut nullified_value_two = rand_96_bit(rng);

    let mut value = rand_96_bit(rng);
    let mut fee = rand_96_bit(rng);

    // We need to make sure the fee and value are less than the sum of the nullified fee and value.
    // We also need to ensure the change will not exceed 2^96.
    while value > (nullified_value_one + nullified_value_two)
        || (nullified_value_one + nullified_value_two) - value >= Fr254::from(1u128 << 96)
    {
        nullified_value_one = rand_96_bit(rng);
        nullified_value_two = rand_96_bit(rng);
        value = rand_96_bit(rng);
    }

    while fee > (nullified_fee_one + nullified_fee_two)
        || (nullified_fee_one + nullified_fee_two) - fee >= Fr254::from(1u128 << 96)
    {
        nullified_fee_one = rand_96_bit(rng);
        nullified_fee_two = rand_96_bit(rng);
        fee = rand_96_bit(rng);
    }

    // Generate random root key
    let root_key = Fr254::rand(rng);
    let keys = ZKPKeys::new(root_key).unwrap();

    // Generate random recipient public key
    let recipient_public_key = TEAffine::<BabyJubJub>::rand(rng);

    // Generate random ephemeral private key
    let ephemeral_key = BJJScalar::rand(rng);

    // Make commitments for nullified values
    let nullified_one = Preimage::new(
        nullified_value_one,
        nf_token_id,
        nf_slot_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    // The second token commitment nullified will be from a deposit so the public key will be the neutral point
    let deposit_secret = DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_two = Preimage::new(
        nullified_value_two,
        nf_token_id,
        nf_slot_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(deposit_secret),
    );

    // Now nullified fee tokens
    let nullified_three = Preimage::new(
        nullified_fee_one,
        fee_token_id,
        fee_token_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    let fee_deposit_secret =
        DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_four = Preimage::new(
        nullified_fee_two,
        fee_token_id,
        fee_token_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(fee_deposit_secret),
    );

    // Make membership proofs
    let spend_commitments = [
        nullified_one,
        nullified_two,
        nullified_three,
        nullified_four,
    ];
    let mut membership_proofs = vec![];
    let mut roots = vec![];
    for nullifier in spend_commitments.iter() {
        let (membership_proof, root) = generate_random_path(nullifier.hash().unwrap(), rng);
        membership_proofs.push(membership_proof);
        roots.push(root);
    }

    let mem_proofs: [MembershipProof<Fr254>; 4] = membership_proofs.try_into().unwrap();
    let roots: [Fr254; 4] = roots.try_into().unwrap();

    // Work out what the change values will be
    let value_change = nullified_value_one + nullified_value_two - value;
    let fee_change = nullified_fee_one + nullified_fee_two - fee;

    // Salts for new commitments
    let new_salts = [Salt::new_transfer_salt().get_salt(); 3];

    let public_inputs = PublicInputs::new().fee(fee).roots(&roots).build();

    let private_inputs = PrivateInputs::new()
        .value(value)
        .nf_token_id(nf_token_id)
        .nf_slot_id(nf_slot_id)
        .ephemeral_key(ephemeral_key)
        .recipient_public_key(recipient_public_key)
        .nullifiers_values(&[
            nullified_one.get_value(),
            nullified_two.get_value(),
            nullified_three.get_value(),
            nullified_four.get_value(),
        ])
        .nullifiers_salts(&[
            nullified_one.get_salt(),
            nullified_two.get_salt(),
            nullified_three.get_salt(),
            nullified_four.get_salt(),
        ])
        .commitments_values(&[value_change, fee_change])
        .commitments_salts(&new_salts)
        .membership_proofs(&mem_proofs)
        .nullifier_key(keys.nullifier_key)
        .secret_preimages(&[
            nullified_one.get_secret_preimage().to_array(),
            nullified_two.get_secret_preimage().to_array(),
            nullified_three.get_secret_preimage().to_array(),
            nullified_four.get_secret_preimage().to_array(),
        ])
        .zkp_private_key(keys.zkp_private_key)
        .public_keys(&[
            nullified_one.get_public_key(),
            nullified_two.get_public_key(),
            nullified_three.get_public_key(),
            nullified_four.get_public_key(),
        ])
        .build();

    (public_inputs, private_inputs)
}
