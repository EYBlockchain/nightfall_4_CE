#[cfg(test)]
mod tests {
    use crate::{
        domain::entities::{DepositSecret, Preimage, Salt},
        driven::primitives::kemdem_functions::kemdem_encrypt,
        drivers::derive_key::ZKPKeys,
        ports::{commitments::Commitment, secret_hash::SecretHash},
    };
    use alloy::{
        dyn_abi::abi::encode,
        primitives::{keccak256, Address, U256},
        sol_types::SolValue,
    };
    use ark_bn254::Bn254;
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
        poseidon::{FieldHasher, Poseidon},
        trees::{Directions, MembershipProof, PathElement, TreeHasher},
    };
    use jf_relation::{Arithmetization, Circuit};
    use lib::{
        hex_conversion::HexConvertible,
        nf_client_proof::{PrivateInputs, PublicInputs},
        nf_token_id::to_nf_token_id_from_str,
        plonk_prover::circuits::{unified_circuit::unified_circuit_builder, DOMAIN_SHARED_SALT},
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

    fn generate_random_path(
        leaf_value: Fr254,
        rng: &mut StdRng,
    ) -> (MembershipProof<Fr254>, Fr254) {
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
        let mut membership_proofs = vec![];
        let mut roots = vec![];
        for nullifier in spend_commitments.iter() {
            let (membership_proof, root) =
                generate_random_path(nullifier.hash().unwrap(), &mut rng);
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
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .zkp_private_key(keys.zkp_private_key)
            .root_key(keys.root_key)
            .lambda(keys.lambda)
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
            (nullified_one, &nullified_one.get_secret_preimage()),
            (nullified_two, &nullified_two.get_secret_preimage()),
            (nullified_three, &nullified_three.get_secret_preimage()),
            (nullified_four, &nullified_four.get_secret_preimage()),
        ]
        .map(|(c, secret)| {
            let commitment_hash = c.hash().unwrap();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                // Deposit: use H(secret_preimage)
                let secret_hash = poseidon.hash(&secret.to_array()).unwrap();
                poseidon.hash(&[secret_hash, commitment_hash]).unwrap()
            } else {
                // Transfer: use nullifier_key
                poseidon.hash(&[keys.nullifier_key, commitment_hash]).unwrap()
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
        let mut membership_proofs = vec![];
        let mut roots = vec![];
        for nullifier in spend_commitments.iter() {
            let (membership_proof, root) =
                generate_random_path(nullifier.hash().unwrap(), &mut rng);
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
            .secret_preimages(&[
                nullified_one.get_secret_preimage().to_array(),
                nullified_two.get_secret_preimage().to_array(),
                nullified_three.get_secret_preimage().to_array(),
                nullified_four.get_secret_preimage().to_array(),
            ])
            .zkp_private_key(keys.zkp_private_key)
            .root_key(keys.root_key)
            .lambda(keys.lambda)
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
            (nullified_one, &nullified_one.get_secret_preimage()),
            (nullified_two, &nullified_two.get_secret_preimage()),
            (nullified_three, &nullified_three.get_secret_preimage()),
            (nullified_four, &nullified_four.get_secret_preimage()),
        ]
        .map(|(c, secret)| {
            let commitment_hash = c.hash().unwrap();
            if c.get_public_key() == Affine::<BabyJubjub>::zero() {
                // Deposit: use H(secret_preimage)
                let secret_hash = poseidon.hash(&secret.to_array()).unwrap();
                poseidon.hash(&[secret_hash, commitment_hash]).unwrap()
            } else {
                // Transfer: use nullifier_key
                poseidon.hash(&[keys.nullifier_key, commitment_hash]).unwrap()
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

            //Incorrect roots
            let mut incorrect_roots = build_valid_transfer_inputs();
            incorrect_roots.public_inputs.roots = [Fr254::one(); 4];

            let circuit = unified_circuit_builder(
                &mut incorrect_roots.public_inputs,
                &mut incorrect_roots.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_roots.public_inputs).as_slice(),)
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
            incorrect_value.private_inputs.value = Fr254::from(1u8);

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

            //Incorrect roots
            let mut incorrect_roots = build_valid_withdraw_inputs();
            incorrect_roots.public_inputs.roots = [Fr254::one(); 4];

            let circuit = unified_circuit_builder(
                &mut incorrect_roots.public_inputs,
                &mut incorrect_roots.private_inputs,
            )
            .unwrap();

            assert!(circuit
                .check_circuit_satisfiability(Vec::from(&incorrect_roots.public_inputs).as_slice(),)
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
            incorrect_value.private_inputs.value = Fr254::from(1u8);

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

        let mut inputs = Vec::new();
        inputs.push(circuit_test_info.public_inputs.fee);
        inputs.extend_from_slice(&circuit_test_info.public_inputs.roots);
        inputs.extend_from_slice(&circuit_test_info.public_inputs.commitments);
        inputs.extend_from_slice(&circuit_test_info.public_inputs.nullifiers);
        inputs.extend_from_slice(&circuit_test_info.public_inputs.compressed_secrets);

        let _ = FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(
            &vk, &inputs, &proof, None, true,
        );
    }
}
