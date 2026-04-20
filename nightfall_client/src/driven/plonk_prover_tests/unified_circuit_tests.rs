#[cfg(test)]
mod tests {
    use std::vec;

    use crate::driven::primitives::kemdem_functions::kemdem_encrypt;
    use alloy::{
        dyn_abi::abi::encode,
        primitives::{keccak256, Address, U256},
        sol_types::SolValue,
    };
    use ark_bn254::Bn254;
    use ark_crypto_primitives::sponge::{CryptographicSponge, FieldBasedCryptographicSponge};
    use ark_ec::{twisted_edwards::Affine, AffineRepr, CurveGroup};
    use ark_ff::{One, PrimeField, Zero};
    use ark_std::{rand::rngs::StdRng, UniformRand};
    use jf_plonk::{
        nightfall::{ipa_structs::VerificationKeyId, FFTPlonk},
        proof_system::UniversalSNARK,
        transcript::StandardTranscript,
    };
    use jf_primitives::{
        pcs::prelude::UnivariateKzgPCS,
        poseidon::{
            sponge::{PoseidonSponge, CRHF_RATE},
            FieldHasher, Poseidon, PoseidonPerm,
        },
        trees::{Directions, MembershipProof, PathElement, TreeHasher},
    };
    use jf_relation::{Arithmetization, Circuit};
    use lib::{
        commitments::Commitment,
        deposit_circuit::deposit_circuit_builder,
        derive_key::ZKPKeys,
        hex_conversion::HexConvertible,
        nf_client_proof::{PrivateInputs, PublicInputs},
        nf_token_id::to_nf_token_id_from_str,
        plonk_prover::circuits::{unified_circuit::unified_circuit_builder, DOMAIN_SHARED_SALT},
        secret_hash::SecretHash,
        shared_entities::{DepositData, DepositSecret, Preimage, Salt},
    };
    use nf_curves::ed_on_bn254::{BabyJubjub, Fq as Fr254, Fr as BJJScalar};
    use num_bigint::BigUint;
    use rand::Rng;
    /// Struct use for handling information in circuit testing
    struct CircuitTestInfo {
        public_inputs: PublicInputs,
        private_inputs: PrivateInputs,
        expected_commitments: [Fr254; 4],
        expected_nullifiers: [Fr254; 4],
        expected_compressed_secrets: [Fr254; 5],
        expected_swap_link: Fr254,
        expected_deadline: Fr254,
        expected_swap_side: Fr254,
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
                expected_swap_link: Fr254::zero(),
                expected_deadline: Fr254::zero(),
                expected_swap_side: Fr254::zero(),
            }
        }

        fn with_swap(mut self, swap_link: Fr254, deadline: Fr254, swap_side: Fr254) -> Self {
            self.expected_swap_link = swap_link;
            self.expected_deadline = deadline;
            self.expected_swap_side = swap_side;
            self
        }
    }

    fn generate_random_paths_with_shared_root(
        leaf_values: [Fr254; 4],
        rng: &mut StdRng,
    ) -> ([MembershipProof<Fr254>; 4], Fr254) {
        let poseidon = Poseidon::<Fr254>::new();
        let leaf_indices = [0usize, 1usize, 2usize, 3usize];
        let mut proofs: [MembershipProof<Fr254>; 4] = std::array::from_fn(|i| MembershipProof {
            node_value: leaf_values[i],
            sibling_path: Vec::with_capacity(32),
            leaf_index: leaf_indices[i],
        });
        let mut current_nodes = std::collections::BTreeMap::from([
            (leaf_indices[0], leaf_values[0]),
            (leaf_indices[1], leaf_values[1]),
            (leaf_indices[2], leaf_values[2]),
            (leaf_indices[3], leaf_values[3]),
        ]);

        for level in 0..32 {
            for i in 0..4 {
                let idx = leaf_indices[i] >> level;
                let sibling_idx = idx ^ 1;
                let sibling_value = *current_nodes
                    .entry(sibling_idx)
                    .or_insert_with(|| Fr254::rand(rng));
                let direction = if idx & 1 == 0 {
                    Directions::HashWithThisNodeOnRight
                } else {
                    Directions::HashWithThisNodeOnLeft
                };
                proofs[i].sibling_path.push(PathElement {
                    direction,
                    value: sibling_value,
                });
            }

            let mut next_nodes = std::collections::BTreeMap::new();
            let mut pairs_done = std::collections::BTreeSet::new();
            let keys: Vec<usize> = current_nodes.keys().copied().collect();
            for idx in keys {
                let pair_base = idx & !1;
                if !pairs_done.insert(pair_base) {
                    continue;
                }
                let left = *current_nodes
                    .get(&pair_base)
                    .expect("Left child should exist when building parent");
                let right = *current_nodes
                    .get(&(pair_base + 1))
                    .expect("Right child should exist when building parent");
                let parent = poseidon.tree_hash(&[left, right]).unwrap();
                next_nodes.insert(pair_base >> 1, parent);
            }
            current_nodes = next_nodes;
        }

        let root = *current_nodes
            .get(&0)
            .expect("Root should exist after building shared Merkle paths");
        (proofs, root)
    }

    // Creates a random 96 bit element of Fr254
    fn rand_96_bit(rng: &mut StdRng) -> Fr254 {
        let random_96_bit = u128::rand(rng) % (1u128 << 96);
        Fr254::from(random_96_bit)
    }

    struct FeesAndValues {
        value: Fr254,
        fee: Fr254,
        nullified_value_one: Fr254,
        nullified_value_two: Fr254,
        nullified_fee_one: Fr254,
        nullified_fee_two: Fr254,
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

        // We return random but fees and values where only `value` is unbounded
        fn rand_invalid_value_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.value = Fr254::rand(rng);
            fees_and_values
        }

        // We return random but fees and values where only `fee` is unbounded
        fn rand_invalid_fee_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.fee = Fr254::rand(rng);
            fees_and_values
        }

        // We return random but fees and values where only `nullified_value_one` is unbounded
        fn rand_invalid_nullified_value_one_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.nullified_value_one = Fr254::rand(rng);
            fees_and_values
        }

        // We return random but fees and values where only `nullified_value_two` is unbounded
        fn rand_invalid_nullified_value_two_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.nullified_value_two = Fr254::rand(rng);
            fees_and_values
        }

        // We return random but fees and values where only `nullified_fee_one` is unbounded
        fn rand_invalid_nullified_fee_one_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.nullified_fee_one = Fr254::rand(rng);
            fees_and_values
        }

        // We return random but fees and values where only `nullified_fee_two` is unbounded
        fn rand_invalid_nullified_fee_two_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            fees_and_values.nullified_fee_two = Fr254::rand(rng);
            fees_and_values
        }

        // We return random fees and values where the change fee is invalid
        fn rand_invalid_change_fee_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            while fees_and_values.fee
                <= (fees_and_values.nullified_fee_one + fees_and_values.nullified_fee_two)
                && (fees_and_values.nullified_fee_one + fees_and_values.nullified_fee_two)
                    - fees_and_values.fee
                    < Fr254::from(1u128 << 96)
            {
                fees_and_values.nullified_fee_one = rand_96_bit(rng);
                fees_and_values.nullified_fee_two = rand_96_bit(rng);
                fees_and_values.fee = rand_96_bit(rng);
            }
            fees_and_values
        }

        // We return random fees and values where the change value is invalid
        fn rand_invalid_change_value_new(rng: &mut StdRng) -> Self {
            let mut fees_and_values = Self::rand_valid_new(rng);
            while fees_and_values.value
                <= (fees_and_values.nullified_value_one + fees_and_values.nullified_value_two)
                && (fees_and_values.nullified_value_one + fees_and_values.nullified_value_two)
                    - fees_and_values.value
                    < Fr254::from(1u128 << 96)
            {
                fees_and_values.nullified_value_one = rand_96_bit(rng);
                fees_and_values.nullified_value_two = rand_96_bit(rng);
                fees_and_values.value = rand_96_bit(rng);
            }
            fees_and_values
        }
    }

    fn build_transfer_inputs(valid: bool) -> CircuitTestInfo {
        let mut rng = rand::thread_rng();

        // Generate 20-byte Ethereum address
        let erc_address: [u8; 20] = rng.gen();
        let erc_address_string = format!("0x{}", hex::encode(erc_address));
        let mut rng = jf_utils::test_rng();
        let token_id_fr = Fr254::rand(&mut rng);
        let token_id_string = Fr254::to_hex_string(&token_id_fr);

        let nf_token_id = to_nf_token_id_from_str(&erc_address_string, &token_id_string).unwrap();
        let nf_slot_id = nf_token_id;

        // make a random Nightfall address
        let mut bytes = rand::thread_rng();
        let nf_address_h160 = Address::new(bytes.gen());
        let nf_address_field = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_slice()));
        let nf_address_token = nf_address_h160.tokenize();
        let u256_zero = U256::ZERO.tokenize();
        let fee_token_id_biguint =
            BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice())
                >> 4;
        let fee_token_id = Fr254::from(fee_token_id_biguint);

        let FeesAndValues {
            value,
            fee,
            nullified_value_one,
            nullified_value_two,
            nullified_fee_one,
            nullified_fee_two,
        } = if valid {
            FeesAndValues::rand_valid_new(&mut rng)
        } else {
            match u8::rand(&mut rng) % 8 {
                0 => FeesAndValues::rand_invalid_value_new(&mut rng),
                1 => FeesAndValues::rand_invalid_fee_new(&mut rng),
                2 => FeesAndValues::rand_invalid_nullified_value_one_new(&mut rng),
                3 => FeesAndValues::rand_invalid_nullified_value_two_new(&mut rng),
                4 => FeesAndValues::rand_invalid_nullified_fee_one_new(&mut rng),
                5 => FeesAndValues::rand_invalid_nullified_fee_two_new(&mut rng),
                6 => FeesAndValues::rand_invalid_change_value_new(&mut rng),
                7 => FeesAndValues::rand_invalid_change_fee_new(&mut rng),
                _ => unreachable!(),
            }
        };

        // Generate random root key
        let root_key = Fr254::rand(&mut rng);
        let keys = ZKPKeys::new(root_key).unwrap();

        // Generate random recipient public key
        let recipient_public_key = Affine::<BabyJubjub>::rand(&mut rng);

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
        let spend_commitment_hashes = spend_commitments.map(|commitment| commitment.hash().unwrap());
        let (mem_proofs, root) =
            generate_random_paths_with_shared_root(spend_commitment_hashes, &mut rng);

        // Work out what the change values will be
        let value_change = nullified_value_one + nullified_value_two - value;
        let fee_change = nullified_fee_one + nullified_fee_two - fee;

        // Salts for new commitments
        let new_salts = [Salt::new_transfer_salt().get_salt(); 3];

        let public_inputs = PublicInputs::new().fee(fee).root(root).build();

        let private_inputs = PrivateInputs::new()
            .fee_token_id(fee_token_id)
            .nf_address(nf_address_h160)
            .value_a(value)
            .nf_token_a_id(nf_token_id)
            .nf_slot_id(nf_slot_id)
            .ephemeral_key(ephemeral_key)
            .party_a_public_key(keys.zkp_public_key)
            .party_b_public_key(recipient_public_key)
            .nf_token_b_id(Fr254::zero())
            .value_b(Fr254::zero())
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
            .sender_commitment_salts(&new_salts)
            .membership_proofs(&mem_proofs)
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .root_key(keys.root_key)
            .public_keys(&[
                nullified_one.get_public_key(),
                nullified_two.get_public_key(),
                nullified_three.get_public_key(),
                nullified_four.get_public_key(),
            ])
            .build();
        // Now we calculate the expected commitments, nullifiers and compressed secrets.
        let shared_secret = (recipient_public_key * ephemeral_key).into_affine();
        let contract_nf_address =
            Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nf_address_field);

        let poseidon = Poseidon::<Fr254>::new();
        // Derive a shared salt from the shared secret using domain-separated Poseidon hash.
        let shared_salt_hash = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
            .unwrap();
        let shared_salt = Salt::Transfer(shared_salt_hash);

        let preimage_one = Preimage::new(
            value,
            nf_token_id,
            nf_slot_id,
            recipient_public_key,
            shared_salt,
        );
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

        let expected_commitments = [
            preimage_one.hash().unwrap(),
            preimage_two.hash().unwrap(),
            preimage_three.hash().unwrap(),
            preimage_four.hash().unwrap(),
        ];
        let poseidon = Poseidon::<Fr254>::new();
        let expected_nullifiers: [Fr254; 4] = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ]
        .map(|c| {
            let commitment_hash = c.hash().unwrap();
            let secret = c.get_secret_preimage();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                // Deposit: use hash(preimage, DOMAIN)
                let deposit_nullifier_key = poseidon
                    .hash(&[
                        secret.to_array()[0],
                        secret.to_array()[1],
                        secret.to_array()[2],
                        Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                    ])
                    .unwrap();
                poseidon
                    .hash(&[deposit_nullifier_key, commitment_hash])
                    .unwrap()
            } else {
                // Transfer: use nullifier_key
                poseidon
                    .hash(&[keys.nullifier_key, commitment_hash])
                    .unwrap()
            }
        });

        let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<false>(
            ephemeral_key,
            recipient_public_key,
            &[nf_token_id, nf_slot_id, value],
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

    fn build_valid_transfer_inputs() -> CircuitTestInfo {
        build_transfer_inputs(true)
    }

    fn build_withdraw_inputs(valid: bool) -> CircuitTestInfo {
        let mut rng = rand::thread_rng();

        // Generate 20-byte Ethereum address
        let erc_address: [u8; 20] = rng.gen();
        let erc_address_string = format!("0x{}", hex::encode(erc_address));
        let mut rng = jf_utils::test_rng();
        let token_id_fr = Fr254::rand(&mut rng);
        let token_id_string = Fr254::to_hex_string(&token_id_fr);

        let nf_token_id = to_nf_token_id_from_str(&erc_address_string, &token_id_string).unwrap();
        let nf_slot_id = nf_token_id;

        // Generate a random withdraw address
        let mut withdraw_address_bytes: [u8; 20] = [0; 20]; // Initialize with zeros
        rand::thread_rng().fill(&mut withdraw_address_bytes);
        if withdraw_address_bytes == [0; 20] {
            withdraw_address_bytes[0] = 1;
        }

        let withdraw_address = Fr254::from_be_bytes_mod_order(&withdraw_address_bytes);
        // make a random Nightfall address, and create fee_token_id from it
        let mut bytes = rand::thread_rng();

        let nf_address_h160 = Address::new(bytes.gen());
        let nf_address = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_slice()));
        let nf_address_token = nf_address_h160.tokenize();
        let u256_zero = U256::ZERO.tokenize();
        let fee_token_id_biguint =
            BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice())
                >> 4;
        let fee_token_id = Fr254::from(fee_token_id_biguint);

        let FeesAndValues {
            value,
            fee,
            nullified_value_one,
            nullified_value_two,
            nullified_fee_one,
            nullified_fee_two,
        } = if valid {
            FeesAndValues::rand_valid_new(&mut rng)
        } else {
            match u8::rand(&mut rng) % 8 {
                0 => FeesAndValues::rand_invalid_change_value_new(&mut rng),
                1 => FeesAndValues::rand_invalid_change_fee_new(&mut rng),
                2 => FeesAndValues::rand_invalid_nullified_value_one_new(&mut rng),
                3 => FeesAndValues::rand_invalid_nullified_value_two_new(&mut rng),
                4 => FeesAndValues::rand_invalid_nullified_fee_one_new(&mut rng),
                5 => FeesAndValues::rand_invalid_nullified_fee_two_new(&mut rng),
                6 => FeesAndValues::rand_invalid_change_value_new(&mut rng),
                7 => FeesAndValues::rand_invalid_change_fee_new(&mut rng),
                _ => unreachable!(),
            }
        };

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
        let spend_commitment_hashes = spend_commitments.map(|commitment| commitment.hash().unwrap());
        let (mem_proofs, root) =
            generate_random_paths_with_shared_root(spend_commitment_hashes, &mut rng);

        // Work out what the change values will be
        let value_change = nullified_value_one + nullified_value_two - value;
        let fee_change = nullified_fee_one + nullified_fee_two - fee;

        // Salts for new commitments
        let new_salts = [Salt::new_transfer_salt().get_salt(); 3];

        let public_inputs = PublicInputs::new().fee(fee).root(root).build();

        let private_inputs = PrivateInputs::new()
            .fee_token_id(fee_token_id)
            .nf_address(nf_address_h160)
            .value_a(value)
            .nf_token_a_id(nf_token_id)
            .nf_slot_id(nf_slot_id)
            .ephemeral_key(ephemeral_key)
            .party_a_public_key(keys.zkp_public_key)
            .party_b_public_key(recipient_public_key)
            .nf_token_b_id(Fr254::zero())
            .value_b(Fr254::zero())
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
            .sender_commitment_salts(&new_salts)
            .membership_proofs(&mem_proofs)
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .root_key(keys.root_key)
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
        let expected_nullifiers: [Fr254; 4] = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ]
        .map(|c| {
            let commitment_hash = c.hash().unwrap();
            let secret = c.get_secret_preimage();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                // Deposit: use hash(preimage, DOMAIN)
                let deposit_nullifier_key = poseidon
                    .hash(&[
                        secret.to_array()[0],
                        secret.to_array()[1],
                        secret.to_array()[2],
                        Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                    ])
                    .unwrap();
                poseidon
                    .hash(&[deposit_nullifier_key, commitment_hash])
                    .unwrap()
            } else {
                // Transfer: use nullifier_key
                poseidon
                    .hash(&[keys.nullifier_key, commitment_hash])
                    .unwrap()
            }
        });

        let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<true>(
            ephemeral_key,
            recipient_public_key,
            &[nf_token_id, withdraw_address, value],
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

    fn build_valid_withdraw_inputs() -> CircuitTestInfo {
        build_withdraw_inputs(true)
    }

    fn build_swap_inputs(valid: bool) -> CircuitTestInfo {
        let mut rng = jf_utils::test_rng();
        let root_key = Fr254::rand(&mut rng);
        let keys = ZKPKeys::new(root_key).unwrap();

        // Party B
        let root_key_b = Fr254::rand(&mut rng);
        let keys_b = ZKPKeys::new(root_key_b).unwrap();

        // Generate token IDs
        let erc_address_a: [u8; 20] = rand::thread_rng().gen();
        let erc_address_a_string = format!("0x{}", hex::encode(erc_address_a));
        let token_id_a_fr = Fr254::rand(&mut rng);
        let token_id_a_string = Fr254::to_hex_string(&token_id_a_fr);
        let nf_token_a_id =
            to_nf_token_id_from_str(&erc_address_a_string, &token_id_a_string).unwrap();

        let erc_address_b: [u8; 20] = rand::thread_rng().gen();
        let erc_address_b_string = format!("0x{}", hex::encode(erc_address_b));
        let token_id_b_fr = Fr254::rand(&mut rng);
        let token_id_b_string = Fr254::to_hex_string(&token_id_b_fr);
        let nf_token_b_id =
            to_nf_token_id_from_str(&erc_address_b_string, &token_id_b_string).unwrap();

        // Fee setup
        let nf_address_h160 = Address::new(rand::thread_rng().gen());
        let nf_address = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_slice()));
        let nf_address_token = nf_address_h160.tokenize();
        let u256_zero = U256::ZERO.tokenize();
        let fee_token_id_biguint =
            BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice())
                >> 4;
        let fee_token_id = Fr254::from(fee_token_id_biguint);

        // Use FeesAndValues like transfer/withdraw
        let FeesAndValues {
            value,
            fee,
            nullified_value_one,
            nullified_value_two,
            nullified_fee_one,
            nullified_fee_two,
        } = if valid {
            FeesAndValues::rand_valid_new(&mut rng)
        } else {
            match u8::rand(&mut rng) % 8 {
                0 => FeesAndValues::rand_invalid_change_value_new(&mut rng),
                1 => FeesAndValues::rand_invalid_change_fee_new(&mut rng),
                2 => FeesAndValues::rand_invalid_nullified_value_one_new(&mut rng),
                3 => FeesAndValues::rand_invalid_nullified_value_two_new(&mut rng),
                4 => FeesAndValues::rand_invalid_nullified_fee_one_new(&mut rng),
                5 => FeesAndValues::rand_invalid_nullified_fee_two_new(&mut rng),
                6 => FeesAndValues::rand_invalid_change_value_new(&mut rng),
                7 => FeesAndValues::rand_invalid_change_fee_new(&mut rng),
                _ => unreachable!(),
            }
        };
        // Values
        let value_a = value;
        let value_b = Fr254::from(200u64);
        let swap_nonce = Fr254::from(u64::rand(&mut rng));
        let deadline = Fr254::from(1000u64);

        // I am Party A, so I send token_a with value_a
        let nf_token_id = nf_token_a_id;
        let nf_slot_id = nf_token_id;

        let nullified_one = Preimage::new(
            nullified_value_one,
            nf_token_id,
            nf_slot_id,
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        );

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

        // Fee commitments
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

        let spend_commitments = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ];
        let spend_commitment_hashes = spend_commitments.map(|commitment| commitment.hash().unwrap());
        let (mem_proofs, root) =
            generate_random_paths_with_shared_root(spend_commitment_hashes, &mut rng);

        let value_change = nullified_value_one + nullified_value_two - value;
        let fee_change = nullified_fee_one + nullified_fee_two - fee;

        let new_salts = [Salt::new_transfer_salt().get_salt(); 3];
        let ephemeral_key = BJJScalar::rand(&mut rng);

        let public_inputs = PublicInputs::new().fee(fee).root(root).build();

        let private_inputs = PrivateInputs::new()
            .fee_token_id(fee_token_id)
            .nf_address(nf_address_h160)
            .value_a(value)
            .nf_token_a_id(nf_token_a_id)
            .nf_slot_id(nf_slot_id)
            .ephemeral_key(ephemeral_key)
            .party_a_public_key(keys.zkp_public_key)
            .party_b_public_key(keys_b.zkp_public_key)
            .nf_token_b_id(nf_token_b_id)
            .value_b(value_b)
            .swap_nonce(swap_nonce)
            .deadline(deadline)
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
            .sender_commitment_salts(&new_salts)
            .membership_proofs(&mem_proofs)
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .root_key(keys.root_key)
            .public_keys(&[
                nullified_one.get_public_key(),
                nullified_two.get_public_key(),
                nullified_three.get_public_key(),
                nullified_four.get_public_key(),
            ])
            .withdraw_address(Fr254::zero()) // No withdraw for swap
            .build();

        let poseidon = Poseidon::<Fr254>::new();

        // Compute expected swap_link
        let sponge_param = PoseidonPerm::<Fr254>::perm().unwrap();
        let mut sponge = PoseidonSponge::<Fr254, CRHF_RATE>::new(&sponge_param);
        let swap_domain = Fr254::from_le_bytes_mod_order(b"SWAP_V1");
        sponge.absorb(&vec![
            swap_domain,
            keys.zkp_public_key.x,
            keys.zkp_public_key.y,
            keys_b.zkp_public_key.x,
            keys_b.zkp_public_key.y,
            nf_token_a_id,
            value_a,
            nf_token_b_id,
            value_b,
            swap_nonce,
        ]);
        let expected_swap_link = sponge.squeeze_native_field_elements(1)[0];

        // Shared secret with counterparty
        let shared_secret: Affine<BabyJubjub> = (keys_b.zkp_public_key * ephemeral_key).into();
        let shared_salt = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
            .unwrap();

        // Contract address for fee
        let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nf_address);

        // Expected commitments
        // [0]: Counterparty receives my tokens
        let preimage_one = Preimage::new(
            value,
            nf_token_id,
            nf_slot_id,
            keys_b.zkp_public_key,
            Salt::Transfer(shared_salt),
        );
        // [1]: My change
        let preimage_two = Preimage::new(
            value_change,
            nf_token_id,
            nf_slot_id,
            keys.zkp_public_key,
            Salt::Transfer(new_salts[0]),
        );
        // [2]: Fee to contract
        let preimage_three = Preimage::new(
            fee,
            fee_token_id,
            fee_token_id,
            contract_nf_address,
            Salt::Transfer(new_salts[1]),
        );
        // [3]: Fee change to me
        let preimage_four = Preimage::new(
            fee_change,
            fee_token_id,
            fee_token_id,
            keys.zkp_public_key,
            Salt::Transfer(new_salts[2]),
        );

        let expected_commitments = [
            preimage_one.hash().unwrap(),
            preimage_two.hash().unwrap(),
            preimage_three.hash().unwrap(),
            preimage_four.hash().unwrap(),
        ];

        // Expected nullifiers
        let expected_nullifiers: [Fr254; 4] = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ]
        .map(|c| {
            let commitment_hash = c.hash().unwrap();
            let secret = c.get_secret_preimage();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                // Deposit: use hash(preimage, DOMAIN)
                let deposit_nullifier_key = poseidon
                    .hash(&[
                        secret.to_array()[0],
                        secret.to_array()[1],
                        secret.to_array()[2],
                        Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                    ])
                    .unwrap();
                poseidon
                    .hash(&[deposit_nullifier_key, commitment_hash])
                    .unwrap()
            } else {
                // Transfer: use nullifier_key
                poseidon
                    .hash(&[keys.nullifier_key, commitment_hash])
                    .unwrap()
            }
        });

        // Expected compressed secrets (encryption for counterparty)
        let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<false>(
            ephemeral_key,
            keys_b.zkp_public_key,
            &[nf_token_id, nf_slot_id, value],
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
        .with_swap(expected_swap_link, deadline, Fr254::one())
    }

    fn build_valid_swap_inputs() -> CircuitTestInfo {
        build_swap_inputs(true)
    }

    fn build_swap_inputs_as_party_b() -> CircuitTestInfo {
        let mut rng = jf_utils::test_rng();
        let root_key_a = Fr254::rand(&mut rng);
        let keys_a = ZKPKeys::new(root_key_a).unwrap();

        // Prover key is party B for this scenario.
        let root_key_b = Fr254::rand(&mut rng);
        let keys_b = ZKPKeys::new(root_key_b).unwrap();

        // Generate token IDs
        let erc_address_a: [u8; 20] = rand::thread_rng().gen();
        let erc_address_a_string = format!("0x{}", hex::encode(erc_address_a));
        let token_id_a_fr = Fr254::rand(&mut rng);
        let token_id_a_string = Fr254::to_hex_string(&token_id_a_fr);
        let nf_token_a_id =
            to_nf_token_id_from_str(&erc_address_a_string, &token_id_a_string).unwrap();

        let erc_address_b: [u8; 20] = rand::thread_rng().gen();
        let erc_address_b_string = format!("0x{}", hex::encode(erc_address_b));
        let token_id_b_fr = Fr254::rand(&mut rng);
        let token_id_b_string = Fr254::to_hex_string(&token_id_b_fr);
        let nf_token_b_id =
            to_nf_token_id_from_str(&erc_address_b_string, &token_id_b_string).unwrap();

        // Fee setup
        let nf_address_h160 = Address::new(rand::thread_rng().gen());
        let nf_address = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_slice()));
        let nf_address_token = nf_address_h160.tokenize();
        let u256_zero = U256::ZERO.tokenize();
        let fee_token_id_biguint =
            BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice())
                >> 4;
        let fee_token_id = Fr254::from(fee_token_id_biguint);

        let FeesAndValues {
            value,
            fee,
            nullified_value_one,
            nullified_value_two,
            nullified_fee_one,
            nullified_fee_two,
        } = FeesAndValues::rand_valid_new(&mut rng);

        // I am party B, so I spend token_b/value_b.
        let value_a = Fr254::from(200u64);
        let value_b = value;
        let mut raw_nonce = u64::rand(&mut rng);
        if raw_nonce == 0 {
            raw_nonce = 1;
        }
        let swap_nonce = Fr254::from(raw_nonce);
        let deadline = Fr254::from(1000u64);

        let nf_token_id = nf_token_b_id;
        let nf_slot_id = nf_token_id;

        let nullified_one = Preimage::new(
            nullified_value_one,
            nf_token_id,
            nf_slot_id,
            keys_b.zkp_public_key,
            Salt::new_transfer_salt(),
        );

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

        let nullified_three = Preimage::new(
            nullified_fee_one,
            fee_token_id,
            fee_token_id,
            keys_b.zkp_public_key,
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

        let spend_commitments = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ];
        let spend_commitment_hashes = spend_commitments.map(|commitment| commitment.hash().unwrap());
        let (mem_proofs, root) =
            generate_random_paths_with_shared_root(spend_commitment_hashes, &mut rng);

        let value_change = nullified_value_one + nullified_value_two - value;
        let fee_change = nullified_fee_one + nullified_fee_two - fee;
        let new_salts = [Salt::new_transfer_salt().get_salt(); 3];
        let ephemeral_key = BJJScalar::rand(&mut rng);

        let public_inputs = PublicInputs::new().fee(fee).root(root).build();

        let private_inputs = PrivateInputs::new()
            .fee_token_id(fee_token_id)
            .nf_address(nf_address_h160)
            .value_a(value_a)
            .nf_token_a_id(nf_token_a_id)
            .nf_slot_id(nf_slot_id)
            .ephemeral_key(ephemeral_key)
            .party_a_public_key(keys_a.zkp_public_key)
            .party_b_public_key(keys_b.zkp_public_key)
            .nf_token_b_id(nf_token_b_id)
            .value_b(value_b)
            .swap_nonce(swap_nonce)
            .deadline(deadline)
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
            .sender_commitment_salts(&new_salts)
            .membership_proofs(&mem_proofs)
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .root_key(keys_b.root_key)
            .public_keys(&[
                nullified_one.get_public_key(),
                nullified_two.get_public_key(),
                nullified_three.get_public_key(),
                nullified_four.get_public_key(),
            ])
            .withdraw_address(Fr254::zero())
            .build();

        let poseidon = Poseidon::<Fr254>::new();
        let sponge_param = PoseidonPerm::<Fr254>::perm().unwrap();
        let mut sponge = PoseidonSponge::<Fr254, CRHF_RATE>::new(&sponge_param);
        let swap_domain = Fr254::from_le_bytes_mod_order(b"SWAP_V1");
        sponge.absorb(&vec![
            swap_domain,
            keys_a.zkp_public_key.x,
            keys_a.zkp_public_key.y,
            keys_b.zkp_public_key.x,
            keys_b.zkp_public_key.y,
            nf_token_a_id,
            value_a,
            nf_token_b_id,
            value_b,
            swap_nonce,
        ]);
        let expected_swap_link = sponge.squeeze_native_field_elements(1)[0];

        let shared_secret: Affine<BabyJubjub> = (keys_a.zkp_public_key * ephemeral_key).into();
        let shared_salt = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
            .unwrap();

        let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nf_address);
        let preimage_one = Preimage::new(
            value,
            nf_token_id,
            nf_slot_id,
            keys_a.zkp_public_key,
            Salt::Transfer(shared_salt),
        );
        let preimage_two = Preimage::new(
            value_change,
            nf_token_id,
            nf_slot_id,
            keys_b.zkp_public_key,
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
            keys_b.zkp_public_key,
            Salt::Transfer(new_salts[2]),
        );

        let expected_commitments = [
            preimage_one.hash().unwrap(),
            preimage_two.hash().unwrap(),
            preimage_three.hash().unwrap(),
            preimage_four.hash().unwrap(),
        ];

        let expected_nullifiers: [Fr254; 4] = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ]
        .map(|c| {
            let commitment_hash = c.hash().unwrap();
            let secret = c.get_secret_preimage();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                let deposit_nullifier_key = poseidon
                    .hash(&[
                        secret.to_array()[0],
                        secret.to_array()[1],
                        secret.to_array()[2],
                        Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                    ])
                    .unwrap();
                poseidon
                    .hash(&[deposit_nullifier_key, commitment_hash])
                    .unwrap()
            } else {
                poseidon
                    .hash(&[keys_b.nullifier_key, commitment_hash])
                    .unwrap()
            }
        });

        let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<false>(
            ephemeral_key,
            keys_a.zkp_public_key,
            &[nf_token_id, nf_slot_id, value],
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
        .with_swap(expected_swap_link, deadline, Fr254::zero())
    }

    #[allow(clippy::too_many_arguments)]
    fn build_matching_swap_leg_inputs(
        rng: &mut StdRng,
        keys_a: &ZKPKeys,
        keys_b: &ZKPKeys,
        nf_token_a_id: Fr254,
        nf_token_b_id: Fr254,
        value_a: Fr254,
        value_b: Fr254,
        swap_nonce: Fr254,
        deadline: Fr254,
        fee: Fr254,
        fee_token_id: Fr254,
        nf_address_h160: Address,
        prover_is_party_a: bool,
    ) -> CircuitTestInfo {
        let poseidon = Poseidon::<Fr254>::new();
        let nf_address_field = Fr254::from(BigUint::from_bytes_be(nf_address_h160.as_slice()));

        let (prover_keys, recipient_pk, nf_token_id, value, expected_side) = if prover_is_party_a {
            (
                keys_a,
                keys_b.zkp_public_key,
                nf_token_a_id,
                value_a,
                Fr254::one(),
            )
        } else {
            (
                keys_b,
                keys_a.zkp_public_key,
                nf_token_b_id,
                value_b,
                Fr254::zero(),
            )
        };
        let nf_slot_id = nf_token_id;

        let mut nullified_value_one = rand_96_bit(rng);
        let mut nullified_value_two = rand_96_bit(rng);
        while value > (nullified_value_one + nullified_value_two)
            || (nullified_value_one + nullified_value_two) - value >= Fr254::from(1u128 << 96)
        {
            nullified_value_one = rand_96_bit(rng);
            nullified_value_two = rand_96_bit(rng);
        }
        let value_change = nullified_value_one + nullified_value_two - value;

        let mut nullified_fee_one = rand_96_bit(rng);
        let mut nullified_fee_two = rand_96_bit(rng);
        while fee > (nullified_fee_one + nullified_fee_two)
            || (nullified_fee_one + nullified_fee_two) - fee >= Fr254::from(1u128 << 96)
        {
            nullified_fee_one = rand_96_bit(rng);
            nullified_fee_two = rand_96_bit(rng);
        }
        let fee_change = nullified_fee_one + nullified_fee_two - fee;

        let nullified_one = Preimage::new(
            nullified_value_one,
            nf_token_id,
            nf_slot_id,
            prover_keys.zkp_public_key,
            Salt::new_transfer_salt(),
        );
        let nullified_two = Preimage::new(
            nullified_value_two,
            nf_token_id,
            nf_slot_id,
            Affine::<BabyJubjub>::zero(),
            Salt::Deposit(DepositSecret::new(
                Fr254::rand(rng),
                Fr254::rand(rng),
                Fr254::rand(rng),
            )),
        );
        let nullified_three = Preimage::new(
            nullified_fee_one,
            fee_token_id,
            fee_token_id,
            prover_keys.zkp_public_key,
            Salt::new_transfer_salt(),
        );
        let nullified_four = Preimage::new(
            nullified_fee_two,
            fee_token_id,
            fee_token_id,
            Affine::<BabyJubjub>::zero(),
            Salt::Deposit(DepositSecret::new(
                Fr254::rand(rng),
                Fr254::rand(rng),
                Fr254::rand(rng),
            )),
        );

        let spend_commitments = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ];
        let spend_commitment_hashes = spend_commitments.map(|commitment| commitment.hash().unwrap());
        let (mem_proofs, root) = generate_random_paths_with_shared_root(spend_commitment_hashes, rng);

        let new_salts = [Salt::new_transfer_salt().get_salt(); 3];
        let ephemeral_key = BJJScalar::rand(rng);

        let public_inputs = PublicInputs::new().fee(fee).root(root).build();

        let private_inputs = PrivateInputs::new()
            .fee_token_id(fee_token_id)
            .nf_address(nf_address_h160)
            .value_a(value_a)
            .nf_token_a_id(nf_token_a_id)
            .nf_slot_id(nf_slot_id)
            .ephemeral_key(ephemeral_key)
            .party_a_public_key(keys_a.zkp_public_key)
            .party_b_public_key(keys_b.zkp_public_key)
            .nf_token_b_id(nf_token_b_id)
            .value_b(value_b)
            .swap_nonce(swap_nonce)
            .deadline(deadline)
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
            .sender_commitment_salts(&new_salts)
            .membership_proofs(&mem_proofs)
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .root_key(prover_keys.root_key)
            .public_keys(&[
                nullified_one.get_public_key(),
                nullified_two.get_public_key(),
                nullified_three.get_public_key(),
                nullified_four.get_public_key(),
            ])
            .withdraw_address(Fr254::zero())
            .build();

        let shared_secret: Affine<BabyJubjub> = (recipient_pk * ephemeral_key).into();
        let shared_salt = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
            .unwrap();

        let contract_nf_address =
            Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nf_address_field);
        let preimage_one = Preimage::new(
            value,
            nf_token_id,
            nf_slot_id,
            recipient_pk,
            Salt::Transfer(shared_salt),
        );
        let preimage_two = Preimage::new(
            value_change,
            nf_token_id,
            nf_slot_id,
            prover_keys.zkp_public_key,
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
            prover_keys.zkp_public_key,
            Salt::Transfer(new_salts[2]),
        );

        let expected_commitments = [
            preimage_one.hash().unwrap(),
            preimage_two.hash().unwrap(),
            preimage_three.hash().unwrap(),
            preimage_four.hash().unwrap(),
        ];

        let expected_nullifiers: [Fr254; 4] = [
            nullified_one,
            nullified_two,
            nullified_three,
            nullified_four,
        ]
        .map(|c| {
            let commitment_hash = c.hash().unwrap();
            let secret = c.get_secret_preimage();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                let deposit_nullifier_key = poseidon
                    .hash(&[
                        secret.to_array()[0],
                        secret.to_array()[1],
                        secret.to_array()[2],
                        Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                    ])
                    .unwrap();
                poseidon
                    .hash(&[deposit_nullifier_key, commitment_hash])
                    .unwrap()
            } else {
                poseidon
                    .hash(&[prover_keys.nullifier_key, commitment_hash])
                    .unwrap()
            }
        });

        let expected_compressed_secrets: [Fr254; 5] = kemdem_encrypt::<false>(
            ephemeral_key,
            recipient_pk,
            &[nf_token_id, nf_slot_id, value],
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
        .with_swap(Fr254::zero(), deadline, expected_side)
    }

    fn assert_valid_circuit(mut info: CircuitTestInfo) {
        let circuit = unified_circuit_builder(&mut info.public_inputs, &mut info.private_inputs)
            .expect("circuit should build for valid inputs");
        circuit
            .check_circuit_satisfiability(Vec::from(&info.public_inputs).as_slice())
            .expect("circuit should be satisfiable for valid inputs");
    }

    fn assert_rejected_circuit(mut info: CircuitTestInfo) {
        match unified_circuit_builder(&mut info.public_inputs, &mut info.private_inputs) {
            Ok(circuit) => {
                assert!(
                    circuit
                        .check_circuit_satisfiability(Vec::from(&info.public_inputs).as_slice(),)
                        .is_err(),
                    "invalid inputs should not satisfy the circuit"
                );
            }
            Err(_) => {
                // Invalid inputs can be rejected during circuit construction.
            }
        }
    }

    fn inject_deposit_commitment_as_first_spend(info: &mut CircuitTestInfo) {
        let mut rng = jf_utils::test_rng();
        let deposit_secret = DepositSecret::new(
            Fr254::rand(&mut rng),
            Fr254::rand(&mut rng),
            Fr254::rand(&mut rng),
        );
        let secret_hash = deposit_secret.hash().expect("deposit secret hash");
        let deposit_data = [
            DepositData {
                nf_token_id: info.private_inputs.nf_token_a_id,
                nf_slot_id: info.private_inputs.nf_slot_id,
                value: info.private_inputs.nullifiers_values[0],
                secret_hash,
            },
            DepositData::default(),
            DepositData::default(),
            DepositData::default(),
        ];
        let mut deposit_public_inputs = PublicInputs::default();
        let deposit_circuit = deposit_circuit_builder(&deposit_data, &mut deposit_public_inputs)
            .expect("deposit circuit should build");
        deposit_circuit
            .check_circuit_satisfiability(Vec::from(&deposit_public_inputs).as_slice())
            .expect("deposit circuit should be satisfiable");

        let deposit_commitment = deposit_public_inputs.commitments[0];
        let leaf_values = [
            deposit_commitment,
            info.private_inputs.membership_proofs[1].node_value,
            info.private_inputs.membership_proofs[2].node_value,
            info.private_inputs.membership_proofs[3].node_value,
        ];
        let (membership_proofs, root) = generate_random_paths_with_shared_root(leaf_values, &mut rng);

        info.private_inputs.public_keys[0] = Affine::<BabyJubjub>::zero();
        info.private_inputs.nullifiers_values[0] = deposit_data[0].value;
        info.private_inputs.nullifiers_salts[0] = secret_hash;
        info.private_inputs.secret_preimages[0] = deposit_secret.to_array();
        info.private_inputs.membership_proofs = membership_proofs;
        info.public_inputs.root = root;
    }

    #[test]
    fn test_transfer() {
        for _ in 0..10 {
            let mut circuit_test_info = build_valid_transfer_inputs();
            let circuit = unified_circuit_builder(
                &mut circuit_test_info.public_inputs,
                &mut circuit_test_info.private_inputs,
            )
            .unwrap();

            circuit
                .check_circuit_satisfiability(
                    Vec::from(&circuit_test_info.public_inputs).as_slice(),
                )
                .unwrap();

            for (circuit_comm, expected_comm) in circuit_test_info
                .public_inputs
                .commitments
                .iter()
                .zip(circuit_test_info.expected_commitments.iter())
            {
                assert_eq!(*circuit_comm, *expected_comm);
            }
            for (circuit_null, expected_null) in circuit_test_info
                .public_inputs
                .nullifiers
                .iter()
                .zip(circuit_test_info.expected_nullifiers.iter())
            {
                assert_eq!(*circuit_null, *expected_null);
            }
            for (i, (circuit_secret, expected_secret)) in circuit_test_info
                .public_inputs
                .compressed_secrets
                .iter()
                .zip(circuit_test_info.expected_compressed_secrets.iter())
                .enumerate()
            {
                assert_eq!(
                    *circuit_secret, *expected_secret,
                    "failed on secret number {} with left {} and right {}",
                    i, *circuit_secret, *expected_secret
                );
            }
            assert_eq!(
                circuit_test_info.public_inputs.swap_side,
                Fr254::zero(),
                "transfer must output swap_side=0"
            );

            // Now we run checks on incorrect information
            // Incorrect fee
            let mut incorrect_fee = build_valid_transfer_inputs();
            incorrect_fee.public_inputs.fee += Fr254::one();

            let circuit = unified_circuit_builder(
                &mut incorrect_fee.public_inputs,
                &mut incorrect_fee.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_fee.public_inputs).as_slice(),)
                .is_err());

            // Wrap around errors. We generate invalid transfer inputs
            for _ in 0..4 {
                let mut wrap_around_error = build_transfer_inputs(false);

                let circuit = unified_circuit_builder(
                    &mut wrap_around_error.public_inputs,
                    &mut wrap_around_error.private_inputs,
                )
                .unwrap();

                assert!(circuit
                    .check_circuit_satisfiability(
                        Vec::from(&wrap_around_error.public_inputs).as_slice(),
                    )
                    .is_err());
            }

            //Incorrect root
            let mut incorrect_root = build_valid_transfer_inputs();
            incorrect_root.public_inputs.root = Fr254::one();

            let circuit = unified_circuit_builder(
                &mut incorrect_root.public_inputs,
                &mut incorrect_root.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(
                    Vec::from(&incorrect_root.public_inputs).as_slice(),
                )
                .is_err());

            // If the wirthdraw address is non-zero we should fail
            let mut incorrect_withdraw_address = build_valid_transfer_inputs();
            incorrect_withdraw_address.private_inputs.withdraw_address = Fr254::from(1u8);

            let circuit = unified_circuit_builder(
                &mut incorrect_withdraw_address.public_inputs,
                &mut incorrect_withdraw_address.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(
                    Vec::from(&incorrect_withdraw_address.public_inputs).as_slice(),
                )
                .is_err());

            // If the value is incorrect we should fail
            let mut incorrect_value = build_valid_transfer_inputs();
            incorrect_value.private_inputs.value_a = Fr254::from(1u8);

            let circuit = unified_circuit_builder(
                &mut incorrect_value.public_inputs,
                &mut incorrect_value.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_value.public_inputs).as_slice(),)
                .is_err());
        }
    }

    #[test]
    fn test_withdraw() {
        for _ in 0..10 {
            let mut circuit_test_info = build_valid_withdraw_inputs();
            let circuit = unified_circuit_builder(
                &mut circuit_test_info.public_inputs,
                &mut circuit_test_info.private_inputs,
            )
            .expect("Failed to build circuit");

            circuit
                .check_circuit_satisfiability(
                    Vec::from(&circuit_test_info.public_inputs).as_slice(),
                )
                .expect("Circuit should be satisfiable with valid inputs");

            for (circuit_comm, expected_comm) in circuit_test_info
                .public_inputs
                .commitments
                .iter()
                .zip(circuit_test_info.expected_commitments.iter())
            {
                assert_eq!(*circuit_comm, *expected_comm);
            }
            for (circuit_null, expected_null) in circuit_test_info
                .public_inputs
                .nullifiers
                .iter()
                .zip(circuit_test_info.expected_nullifiers.iter())
            {
                assert_eq!(*circuit_null, *expected_null);
            }
            for (i, (circuit_secret, expected_secret)) in circuit_test_info
                .public_inputs
                .compressed_secrets
                .iter()
                .zip(circuit_test_info.expected_compressed_secrets.iter())
                .enumerate()
            {
                assert_eq!(
                    *circuit_secret, *expected_secret,
                    "failed on secret number {} with left {} and right {}",
                    i, *circuit_secret, *expected_secret
                );
            }
            assert_eq!(
                circuit_test_info.public_inputs.swap_side,
                Fr254::zero(),
                "withdraw must output swap_side=0"
            );
            // Now we run checks on incorrect information
            // Incorrect fee
            let mut incorrect_fee = build_valid_withdraw_inputs();
            incorrect_fee.public_inputs.fee += Fr254::one();

            let circuit = unified_circuit_builder(
                &mut incorrect_fee.public_inputs,
                &mut incorrect_fee.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_fee.public_inputs).as_slice(),)
                .is_err());

            // Wrap around errors. We generate invalid withdraw inputs
            for _ in 0..4 {
                let mut wrap_around_error = build_withdraw_inputs(false);

                let circuit = unified_circuit_builder(
                    &mut wrap_around_error.public_inputs,
                    &mut wrap_around_error.private_inputs,
                )
                .unwrap();

                assert!(circuit
                    .check_circuit_satisfiability(
                        Vec::from(&wrap_around_error.public_inputs).as_slice(),
                    )
                    .is_err());
            }

            //Incorrect root
            let mut incorrect_root = build_valid_withdraw_inputs();
            incorrect_root.public_inputs.root = Fr254::one();

            let circuit = unified_circuit_builder(
                &mut incorrect_root.public_inputs,
                &mut incorrect_root.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(
                    Vec::from(&incorrect_root.public_inputs).as_slice(),
                )
                .is_err());

            // If the wirthdraw address is zero we should fail
            let mut incorrect_withdraw_address = build_valid_withdraw_inputs();
            incorrect_withdraw_address.private_inputs.withdraw_address = Fr254::from(0u8);

            let circuit = unified_circuit_builder(
                &mut incorrect_withdraw_address.public_inputs,
                &mut incorrect_withdraw_address.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(
                    Vec::from(&incorrect_withdraw_address.public_inputs).as_slice(),
                )
                .is_err());

            // If the value is incorrect we should fail
            let mut incorrect_value = build_valid_withdraw_inputs();
            incorrect_value.private_inputs.value_a = Fr254::from(1u8);

            let circuit = unified_circuit_builder(
                &mut incorrect_value.public_inputs,
                &mut incorrect_value.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_value.public_inputs).as_slice(),)
                .is_err());
        }
    }

    #[test]
    fn test_full_transfer() {
        let mut circuit_test_info = build_valid_transfer_inputs();
        let mut circuit = unified_circuit_builder(
            &mut circuit_test_info.public_inputs,
            &mut circuit_test_info.private_inputs,
        )
        .unwrap();
        circuit
            .check_circuit_satisfiability(Vec::from(&circuit_test_info.public_inputs).as_slice())
            .unwrap();
        circuit.finalize_for_arithmetization().unwrap();
        let mut rng = ark_std::rand::thread_rng();
        let srs_size = circuit.srs_size(true).unwrap();
        let srs =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(srs_size, &mut rng)
                .unwrap();
        let (pk, vk) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
            &srs,
            Some(VerificationKeyId::Client),
            &circuit,
            true,
        )
        .unwrap();
        let proof = FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
            &mut rng, &circuit, &pk, None, true,
        )
        .unwrap();

        let inputs = Vec::from(&circuit_test_info.public_inputs);

        let _ = FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(
            &vk, &inputs, &proof, None, true,
        );
    }

    #[test]
    fn test_transfer_rejects_mismatched_party_a_key_when_not_swap() {
        let mut info = build_valid_transfer_inputs();

        let original = info.private_inputs.party_a_public_key;
        let mut new_root = info.private_inputs.root_key + Fr254::one();
        let mut different_key = ZKPKeys::new(new_root).unwrap();
        while different_key.zkp_public_key == original {
            new_root += Fr254::one();
            different_key = ZKPKeys::new(new_root).unwrap();
        }

        info.private_inputs.party_a_public_key = different_key.zkp_public_key;
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_transfer_rejects_non_zero_counterparty_offer_when_not_swap() {
        let mut token_info = build_valid_transfer_inputs();
        token_info.private_inputs.nf_token_b_id = Fr254::from(1u64);
        assert_rejected_circuit(token_info);

        let mut value_info = build_valid_transfer_inputs();
        value_info.private_inputs.value_b = Fr254::from(1u64);
        assert_rejected_circuit(value_info);
    }

    #[test]
    fn test_withdraw_rejects_non_zero_counterparty_offer_when_not_swap() {
        let mut token_info = build_valid_withdraw_inputs();
        token_info.private_inputs.nf_token_b_id = Fr254::from(1u64);
        assert_rejected_circuit(token_info);

        let mut value_info = build_valid_withdraw_inputs();
        value_info.private_inputs.value_b = Fr254::from(1u64);
        assert_rejected_circuit(value_info);
    }

    #[test]
    fn test_swap() {
        for _ in 0..10 {
            let mut circuit_test_info = build_valid_swap_inputs();
            let circuit = unified_circuit_builder(
                &mut circuit_test_info.public_inputs,
                &mut circuit_test_info.private_inputs,
            )
            .unwrap();

            circuit
                .check_circuit_satisfiability(
                    Vec::from(&circuit_test_info.public_inputs).as_slice(),
                )
                .unwrap();

            for (circuit_comm, expected_comm) in circuit_test_info
                .public_inputs
                .commitments
                .iter()
                .zip(circuit_test_info.expected_commitments.iter())
            {
                assert_eq!(*circuit_comm, *expected_comm);
            }

            for (circuit_null, expected_null) in circuit_test_info
                .public_inputs
                .nullifiers
                .iter()
                .zip(circuit_test_info.expected_nullifiers.iter())
            {
                assert_eq!(*circuit_null, *expected_null);
            }

            for (i, (circuit_secret, expected_secret)) in circuit_test_info
                .public_inputs
                .compressed_secrets
                .iter()
                .zip(circuit_test_info.expected_compressed_secrets.iter())
                .enumerate()
            {
                assert_eq!(
                    *circuit_secret, *expected_secret,
                    "failed on secret number {} with left {} and right {}",
                    i, *circuit_secret, *expected_secret
                );
            }

            assert_eq!(
                circuit_test_info.public_inputs.swap_link, circuit_test_info.expected_swap_link,
                "swap_link mismatch"
            );

            assert_eq!(
                circuit_test_info.public_inputs.deadline, circuit_test_info.expected_deadline,
                "deadline mismatch"
            );
            assert_eq!(
                circuit_test_info.public_inputs.swap_side, circuit_test_info.expected_swap_side,
                "swap_side mismatch"
            );

            for _ in 0..4 {
                let mut wrap_around_error = build_swap_inputs(false);
                let circuit = unified_circuit_builder(
                    &mut wrap_around_error.public_inputs,
                    &mut wrap_around_error.private_inputs,
                )
                .unwrap();
                assert!(circuit
                    .check_circuit_satisfiability(
                        Vec::from(&wrap_around_error.public_inputs).as_slice(),
                    )
                    .is_err());
            }

            let invalid_cases: Vec<Box<dyn Fn() -> CircuitTestInfo>> = vec![
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.public_inputs.fee += Fr254::one();
                    info
                }),
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.public_inputs.root = Fr254::one();
                    info
                }),
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.private_inputs.nf_token_a_id = Fr254::from(999u64);
                    info
                }),
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.private_inputs.value_a = Fr254::from(999u64);
                    info
                }),
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.private_inputs.party_b_public_key = info.private_inputs.party_a_public_key;
                    info
                }),
                Box::new(|| {
                    let mut info = build_valid_swap_inputs();
                    info.private_inputs.value_a = Fr254::from(1u64);
                    info
                }),
            ];

            for create_invalid in invalid_cases {
                let mut info = create_invalid();
                let circuit =
                    unified_circuit_builder(&mut info.public_inputs, &mut info.private_inputs)
                        .unwrap();

                assert!(circuit
                    .check_circuit_satisfiability(Vec::from(&info.public_inputs).as_slice())
                    .is_err());
            }
        }
    }

    #[test]
    fn test_swap_as_party_b() {
        for _ in 0..5 {
            let mut circuit_test_info = build_swap_inputs_as_party_b();
            let circuit = unified_circuit_builder(
                &mut circuit_test_info.public_inputs,
                &mut circuit_test_info.private_inputs,
            )
            .unwrap();

            circuit
                .check_circuit_satisfiability(
                    Vec::from(&circuit_test_info.public_inputs).as_slice(),
                )
                .unwrap();

            assert_eq!(
                circuit_test_info.public_inputs.swap_link, circuit_test_info.expected_swap_link,
                "swap_link mismatch"
            );
            assert_eq!(
                circuit_test_info.public_inputs.deadline, circuit_test_info.expected_deadline,
                "deadline mismatch"
            );
            assert_eq!(
                circuit_test_info.public_inputs.swap_side,
                Fr254::zero(),
                "party B swap should output swap_side=0"
            );
        }
    }

    #[test]
    fn test_matching_swap_legs_produce_same_link_and_opposite_sides() {
        let mut rng = jf_utils::test_rng();
        let keys_a = ZKPKeys::new(Fr254::rand(&mut rng)).unwrap();
        let keys_b = ZKPKeys::new(Fr254::rand(&mut rng)).unwrap();

        let erc_address_a: [u8; 20] = rand::thread_rng().gen();
        let erc_address_a_string = format!("0x{}", hex::encode(erc_address_a));
        let token_id_a = Fr254::rand(&mut rng);
        let nf_token_a_id =
            to_nf_token_id_from_str(&erc_address_a_string, &Fr254::to_hex_string(&token_id_a))
                .unwrap();

        let erc_address_b: [u8; 20] = rand::thread_rng().gen();
        let erc_address_b_string = format!("0x{}", hex::encode(erc_address_b));
        let token_id_b = Fr254::rand(&mut rng);
        let nf_token_b_id =
            to_nf_token_id_from_str(&erc_address_b_string, &Fr254::to_hex_string(&token_id_b))
                .unwrap();

        let nf_address_h160 = Address::new(rand::thread_rng().gen());
        let nf_address_token = nf_address_h160.tokenize();
        let u256_zero = U256::ZERO.tokenize();
        let fee_token_id_biguint =
            BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice())
                >> 4;
        let fee_token_id = Fr254::from(fee_token_id_biguint);

        let value_a = Fr254::from(123u64);
        let value_b = Fr254::from(456u64);
        let swap_nonce = Fr254::from(42u64);
        let deadline = Fr254::from(1000u64);
        let fee = Fr254::from(5u64);
        let sponge_param = PoseidonPerm::<Fr254>::perm().unwrap();
        let mut sponge = PoseidonSponge::<Fr254, CRHF_RATE>::new(&sponge_param);
        let swap_domain = Fr254::from_le_bytes_mod_order(b"SWAP_V1");
        sponge.absorb(&vec![
            swap_domain,
            keys_a.zkp_public_key.x,
            keys_a.zkp_public_key.y,
            keys_b.zkp_public_key.x,
            keys_b.zkp_public_key.y,
            nf_token_a_id,
            value_a,
            nf_token_b_id,
            value_b,
            swap_nonce,
        ]);
        let expected_swap_link = sponge.squeeze_native_field_elements(1)[0];

        let mut party_a_leg = build_matching_swap_leg_inputs(
            &mut rng,
            &keys_a,
            &keys_b,
            nf_token_a_id,
            nf_token_b_id,
            value_a,
            value_b,
            swap_nonce,
            deadline,
            fee,
            fee_token_id,
            nf_address_h160,
            true,
        );
        let mut party_b_leg = build_matching_swap_leg_inputs(
            &mut rng,
            &keys_a,
            &keys_b,
            nf_token_a_id,
            nf_token_b_id,
            value_a,
            value_b,
            swap_nonce,
            deadline,
            fee,
            fee_token_id,
            nf_address_h160,
            false,
        );

        let circuit_a = unified_circuit_builder(
            &mut party_a_leg.public_inputs,
            &mut party_a_leg.private_inputs,
        )
        .unwrap();
        circuit_a
            .check_circuit_satisfiability(Vec::from(&party_a_leg.public_inputs).as_slice())
            .unwrap();

        let circuit_b = unified_circuit_builder(
            &mut party_b_leg.public_inputs,
            &mut party_b_leg.private_inputs,
        )
        .unwrap();
        circuit_b
            .check_circuit_satisfiability(Vec::from(&party_b_leg.public_inputs).as_slice())
            .unwrap();

        assert_eq!(
            party_a_leg.public_inputs.swap_link, party_b_leg.public_inputs.swap_link,
            "complementary swap legs must emit the same swap_link"
        );
        assert_eq!(
            party_a_leg.public_inputs.swap_link, expected_swap_link,
            "party A leg swap_link must match independently computed value"
        );
        assert_eq!(
            party_b_leg.public_inputs.swap_link, expected_swap_link,
            "party B leg swap_link must match independently computed value"
        );
        assert_eq!(
            party_a_leg.public_inputs.swap_side,
            Fr254::one(),
            "party A leg must emit swap_side=1"
        );
        assert_eq!(
            party_b_leg.public_inputs.swap_side,
            Fr254::zero(),
            "party B leg must emit swap_side=0"
        );
    }

    #[test]
    fn test_deposit_commitment_spendable_in_transfer() {
        let mut info = build_valid_transfer_inputs();
        inject_deposit_commitment_as_first_spend(&mut info);
        assert_valid_circuit(info);
    }

    #[test]
    fn test_deposit_commitment_spendable_in_swap() {
        let mut info = build_valid_swap_inputs();
        inject_deposit_commitment_as_first_spend(&mut info);
        assert_valid_circuit(info);
    }

    #[test]
    fn test_swap_rejects_when_prover_matches_neither_party() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.root_key += Fr254::one();
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_swap_rejects_neutral_party_keys() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.party_a_public_key = Affine::<BabyJubjub>::zero();
        assert_rejected_circuit(info);

        let mut info_b = build_valid_swap_inputs();
        info_b.private_inputs.party_b_public_key = Affine::<BabyJubjub>::zero();
        assert_rejected_circuit(info_b);
    }

    #[test]
    fn test_swap_rejects_swap_and_withdraw_combination() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.withdraw_address = Fr254::one();
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_swap_rejects_duplicate_non_zero_nullifiers() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.nullifiers_values[1] = info.private_inputs.nullifiers_values[0];
        info.private_inputs.nullifiers_salts[1] = info.private_inputs.nullifiers_salts[0];
        info.private_inputs.public_keys[1] = info.private_inputs.public_keys[0];
        info.private_inputs.secret_preimages[1] = info.private_inputs.secret_preimages[0];
        info.private_inputs.membership_proofs[1] = info.private_inputs.membership_proofs[0].clone();
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_swap_rejects_swap_nonce_over_64_bits() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.swap_nonce = Fr254::from((1u128 << 65) + 1);
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_swap_rejects_deadline_over_64_bits() {
        let mut info = build_valid_swap_inputs();
        info.private_inputs.deadline = Fr254::from((1u128 << 65) + 1);
        assert_rejected_circuit(info);
    }

    #[test]
    fn test_swap_rejects_tampered_public_swap_outputs() {
        let mut info = build_valid_swap_inputs();
        let circuit = unified_circuit_builder(&mut info.public_inputs, &mut info.private_inputs)
            .expect("valid swap should build");
        circuit
            .check_circuit_satisfiability(Vec::from(&info.public_inputs).as_slice())
            .expect("valid swap should satisfy circuit");

        let mut tampered = info.public_inputs;
        tampered.swap_link += Fr254::one();
        assert!(
            circuit
                .check_circuit_satisfiability(Vec::from(&tampered).as_slice())
                .is_err(),
            "tampered swap_link should be rejected"
        );

        let mut tampered_deadline = info.public_inputs;
        tampered_deadline.deadline += Fr254::one();
        assert!(
            circuit
                .check_circuit_satisfiability(Vec::from(&tampered_deadline).as_slice())
                .is_err(),
            "tampered deadline should be rejected"
        );

        let mut tampered_side = info.public_inputs;
        tampered_side.swap_side = Fr254::one() - tampered_side.swap_side;
        assert!(
            circuit
                .check_circuit_satisfiability(Vec::from(&tampered_side).as_slice())
                .is_err(),
            "tampered swap_side should be rejected"
        );
    }

    #[test]
    fn test_full_swap() {
        let mut circuit_test_info = build_valid_swap_inputs();
        let mut circuit = unified_circuit_builder(
            &mut circuit_test_info.public_inputs,
            &mut circuit_test_info.private_inputs,
        )
        .unwrap();
        circuit
            .check_circuit_satisfiability(Vec::from(&circuit_test_info.public_inputs).as_slice())
            .unwrap();
        circuit.finalize_for_arithmetization().unwrap();
        let mut rng = ark_std::rand::thread_rng();
        let srs_size = circuit.srs_size(true).unwrap();
        let srs =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(srs_size, &mut rng)
                .unwrap();
        let (pk, vk) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
            &srs,
            Some(VerificationKeyId::Client),
            &circuit,
            true,
        )
        .unwrap();
        let proof = FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
            &mut rng, &circuit, &pk, None, true,
        )
        .unwrap();

        // Utilise Vec::from comme check_circuit_satisfiability
        let inputs = Vec::from(&circuit_test_info.public_inputs);

        assert!(
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(
                &vk, &inputs, &proof, None, true,
            )
            .is_ok()
        );
    }
}
