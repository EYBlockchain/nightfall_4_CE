use ark_bn254::Bn254;
use ark_ec::{twisted_edwards::Affine, AffineRepr};
use ark_ff::{PrimeField, Zero};
use ark_std::{rand::rngs::StdRng, UniformRand};
use criterion::{criterion_group, criterion_main, Criterion};
use ethers::{
    abi::{encode, Tokenizable},
    types::{H160, U256},
    utils::keccak256,
};
use jf_plonk::{nightfall::FFTPlonk, proof_system::UniversalSNARK, transcript::StandardTranscript};
use jf_primitives::{
    pcs::prelude::UnivariateKzgPCS,
    poseidon::{FieldHasher, Poseidon},
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use jf_relation::{Arithmetization, Circuit};
use nf_curves::ed_on_bn254::{BabyJubjub, Fq as Fr254, Fr as BJJScalar};
use nightfall_client::{
    domain::entities::HexConvertible, drivers::rest::utils::to_nf_token_id_from_str,
};
use nightfall_client::{
    domain::entities::{DepositSecret, Preimage, Salt},
    driven::{
        plonk_prover::circuits::unified_circuit::unified_circuit_builder,
        primitives::kemdem_functions::kemdem_encrypt,
    },
    drivers::derive_key::ZKPKeys,
    ports::{
        commitments::Commitment,
        proof::{PrivateInputs, PublicInputs},
        secret_hash::SecretHash,
    },
};
use num_bigint::BigUint;
use rand::Rng;
use std::time::{Duration, Instant};

struct FeesAndValues {
    value: Fr254,
    fee: Fr254,
    nullified_value_one: Fr254,
    nullified_value_two: Fr254,
    nullified_fee_one: Fr254,
    nullified_fee_two: Fr254,
}
// Creates a random 96 bit element of Fr254
fn rand_96_bit(rng: &mut StdRng) -> Fr254 {
    let random_96_bit = u128::rand(rng) % (1u128 << 96);
    Fr254::from(random_96_bit)
}
impl FeesAndValues {
    // We return random but valid fees and values
    fn rand_valid_new(rng: &mut StdRng) -> Self {
        let mut nullified_value_one = rand_96_bit(rng);
        let mut nullified_value_two = rand_96_bit(rng);
        let mut nullified_fee_one = rand_96_bit(rng);
        let mut nullified_fee_two = rand_96_bit(rng);

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

        Self {
            value,
            fee,
            nullified_value_one,
            nullified_value_two,
            nullified_fee_one,
            nullified_fee_two,
        }
    }
}
#[allow(dead_code)]
struct CircuitTestInfo {
    public_inputs: PublicInputs,
    private_inputs: PrivateInputs,
    expected_commitments: [Fr254; 4],
    expected_nullifiers: [Fr254; 4],
    expected_compressed_secrets: [Fr254; 5],
}

impl CircuitTestInfo {
    fn new(
        public_inputs: PublicInputs,
        private_inputs: PrivateInputs,
        expected_commitments: [Fr254; 4],
        expected_nullifiers: [Fr254; 4],
        expected_compressed_secrets: [Fr254; 5],
    ) -> Self {
        Self {
            public_inputs,
            private_inputs,
            expected_commitments,
            expected_nullifiers,
            expected_compressed_secrets,
        }
    }
}

fn generate_random_path(leaf_value: Fr254, rng: &mut StdRng) -> (MembershipProof<Fr254>, Fr254) {
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

fn build_valid_transfer_inputs() -> CircuitTestInfo {
    let mut rng = rand::thread_rng();

    // Generate 20-byte Ethereum address
    let erc_address: [u8; 20] = rng.gen();
    let erc_address_string = format!("0x{}", hex::encode(erc_address));
    let mut rng = jf_utils::test_rng();
    let token_id_fr = Fr254::rand(&mut rng);
    let token_id_string = Fr254::to_hex_string(&token_id_fr);

    let nf_token_id = to_nf_token_id_from_str(&erc_address_string, &token_id_string).unwrap();
    let nf_slot_id = nf_token_id;

    let token_id = Fr254::from_hex_string(&token_id_string).unwrap();

    let withdraw_address_bytes: [u8; 20] = [rand::thread_rng().gen(); 20];

    let withdraw_address = Fr254::from_be_bytes_mod_order(&withdraw_address_bytes);
    // make a random Nightfall address, and create fee_token_id from it
    let nf_address_h160 = H160::random();
    let nf_address = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_bytes()));
    let nf_address_token = nf_address_h160.into_token();
    let u256_zero = U256::zero().into_token();
    let fee_token_id_biguint =
        BigUint::from_bytes_be(&keccak256(encode(&[nf_address_token, u256_zero]))) >> 4;
    let fee_token_id = Fr254::from(fee_token_id_biguint);

    let FeesAndValues {
        value,
        fee,
        nullified_value_one,
        nullified_value_two,
        nullified_fee_one,
        nullified_fee_two,
    } = FeesAndValues::rand_valid_new(&mut rng);

    // Generate random root key
    let root_key = Fr254::rand(&mut rng);
    let keys = ZKPKeys::new(root_key).unwrap();

    // Set recipient public key to neutral point
    let recipient_public_key = Affine::<BabyJubjub>::zero();

    // Generate random ephemeral private key
    let ephemeral_key = BJJScalar::rand(&mut rng);

    // Make commitments for nullified values
    let nullified_one = Preimage::new(
        nullified_value_one,
        nf_token_id,
        nf_slot_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    // The second token commitment nullified will be from a deposit so the public key will be the neutral point
    let deposit_secret = DepositSecret::new(
        Fr254::rand(&mut rng),
        Fr254::rand(&mut rng),
        Fr254::rand(&mut rng),
    );
    let nullified_two = Preimage::new(
        nullified_value_two,
        nf_token_id,
        nf_slot_id,
        Affine::<BabyJubjub>::zero(),
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
    let fee_deposit_secret = DepositSecret::new(
        Fr254::rand(&mut rng),
        Fr254::rand(&mut rng),
        Fr254::rand(&mut rng),
    );
    let nullified_four = Preimage::new(
        nullified_fee_two,
        fee_token_id,
        fee_token_id,
        Affine::<BabyJubjub>::zero(),
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
        let (membership_proof, root) = generate_random_path(nullifier.hash().unwrap(), &mut rng);
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
        .fee_token_id(fee_token_id)
        .nf_address(nf_address_h160)
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
        .withdraw_address(withdraw_address)
        .build();

    // Now we calculate the expected commitments, nullifiers and compressed secrets.
    let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nf_address);

    let preimage_two = Preimage::new(
        value_change,
        nf_token_id,
        nf_slot_id,
        keys.zkp_public_key,
        Salt::Transfer(new_salts[0]),
    );
    let preimage_three = Preimage::new(
        fee,
        fee_token_id,
        fee_token_id,
        contract_nf_address,
        Salt::Transfer(new_salts[1]),
    );
    let preimage_four = Preimage::new(
        fee_change,
        fee_token_id,
        fee_token_id,
        keys.zkp_public_key,
        Salt::Transfer(new_salts[2]),
    );
    let poseidon = Poseidon::<Fr254>::new();
    let expected_commitments = [
        Fr254::zero(),
        preimage_two.hash().unwrap(),
        preimage_three.hash().unwrap(),
        preimage_four.hash().unwrap(),
    ];
    let expected_nullifiers = spend_commitments.map(|c| {
        poseidon
            .hash(&[keys.nullifier_key, c.hash().unwrap()])
            .unwrap()
    });

    let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<true>(
        ephemeral_key,
        recipient_public_key,
        &[token_id, nf_slot_id, value],
        Affine::<BabyJubjub>::generator(),
    )
    .unwrap()
    .try_into()
    .unwrap();

    CircuitTestInfo::new(
        public_inputs,
        private_inputs,
        expected_commitments,
        expected_nullifiers,
        expected_compressed_secrets,
    )
}

fn benchmark_unified_circuit(c: &mut Criterion) {
    let mut circuit_test_info = build_valid_transfer_inputs();
    let mut circuit = unified_circuit_builder(
        &mut circuit_test_info.public_inputs,
        &mut circuit_test_info.private_inputs,
    )
    .unwrap();
    circuit
        .check_circuit_satisfiability(Vec::from(&circuit_test_info.public_inputs).as_slice())
        .unwrap();
    println!(
        "transfer : {} constraints before padding",
        circuit.num_gates()
    );
    circuit.finalize_for_arithmetization().unwrap();
    let mut rng = ark_std::rand::thread_rng();
    let srs_size = circuit.srs_size().unwrap();
    ark_std::println!("SRS size: {}", srs_size);
    let srs = FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(srs_size, &mut rng)
        .unwrap();
    let (pk, vk) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(&srs, &circuit).unwrap();
    let start = Instant::now();
    let proof = FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
        &mut rng, &circuit, &pk, None,
    )
    .unwrap();
    println!(
        "Unified Circuit Proving time:{} ms",
        start.elapsed().as_millis()
    );
    c.bench_function("Unified Circuit Proving time:", |b| {
        b.iter(|| {
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
                &mut rng, &circuit, &pk, None,
            )
            .unwrap();
        })
    });
    let mut inputs = Vec::new();
    inputs.push(circuit_test_info.public_inputs.fee);
    inputs.extend_from_slice(&circuit_test_info.public_inputs.roots);
    inputs.extend_from_slice(&circuit_test_info.public_inputs.commitments);
    inputs.extend_from_slice(&circuit_test_info.public_inputs.nullifiers);
    inputs.extend_from_slice(&circuit_test_info.public_inputs.compressed_secrets);
    let start = Instant::now();
    FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(&vk, &inputs, &proof, None)
        .unwrap();
    println!(
        "Unified Circuits Verifying time:{} ms",
        start.elapsed().as_millis()
    );
    c.bench_function("Unified Circuits Verifying time:", |b| {
        b.iter(|| {
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(
                &vk, &inputs, &proof, None,
            )
            .unwrap();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(2)).warm_up_time(Duration::from_secs(1));
    targets = benchmark_unified_circuit
}
criterion_main!(benches);
