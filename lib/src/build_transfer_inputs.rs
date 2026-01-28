use crate::{
    commitments::Commitment,
    derive_key::ZKPKeys,
    hex_conversion::HexConvertible,
    nf_client_proof::{PrivateInputs, PublicInputs},
    nf_token_id::to_nf_token_id_from_str,
    secret_hash::SecretHash,
    shared_entities::{DepositSecret, Preimage, Salt},
};
use alloy::primitives::{hex, keccak256};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_std::{rand::Rng, UniformRand};
use jf_primitives::{
    poseidon::Poseidon,
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};
use num_bigint::BigUint;

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
    let erc_address: [u8; 20] = rng.gen();
    let erc_address_string = format!("0x{}", hex::encode(erc_address));
    let token_id_fr = Fr254::rand(rng);
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
