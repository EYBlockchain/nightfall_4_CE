use super::verify::verify_duplicates_gadgets::VerifyDuplicatesCircuit;
use super::DOMAIN_SHARED_SALT;
use crate::{
    derive_key::{NULLIFIER_PREFIX, PRIVATE_KEY_PREFIX},
    nf_client_proof::{PrivateInputs, PrivateInputsVar, PublicInputs},
    plonk_prover::circuits::verify::{
        verify_commitments_gadgets::VerifyCommitmentsCircuit,
        verify_encryption_gadgets::VerifyEncryptionCircuit,
        verify_nullifiers_gadgets::VerifyNullifiersCircuit,
    },
};
use ark_ec::{twisted_edwards::Affine, AffineRepr};
use ark_ff::BigInteger256;
use ark_ff::{One, PrimeField, Zero};
use jf_plonk::errors::PlonkError;
use jf_primitives::circuit::poseidon::sponge::{PoseidonStateVar, SpongePoseidonHashGadget};
use jf_primitives::circuit::poseidon::PoseidonHashGadget;
use jf_relation::{
    errors::CircuitError,
    gadgets::ecc::{Point, PointVariable},
    Circuit, PlonkCircuit, Variable,
};
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use nf_curves::ed_on_bn254::{BabyJubjub, Fq as Fr254};
use num_bigint::BigUint;

/// This trait is used to construct a circuit to verify the integrity of transfer, withdraw and swap operations
pub trait UnifiedCircuit {
    // this function takes PrivateInputs (all except fee_token_id) and PublicInputs (fee and root specifically)
    // checks the integrity of the operation and returns the full PublicInputs and PrivateInputs
    fn assess_operation_integrity(
        &mut self,
        public_input: &PublicInputs,
        private_input: &mut PrivateInputs,
    ) -> Result<(PublicInputs, PrivateInputs), CircuitError>;
}

impl UnifiedCircuit for PlonkCircuit<Fr254> {
    fn assess_operation_integrity(
        &mut self,
        public_inputs: &PublicInputs,
        private_inputs: &mut PrivateInputs,
    ) -> Result<(PublicInputs, PrivateInputs), CircuitError> {
        // Commitments[0]: transferred value commitment or 0 if withdraw
        // Commitments[1]: change token commitment
        // Commitments[2]: fee paid token commitment
        // Commitments[3]: fee change token commitment
        // Nullifiers[0]: nullify transferred/withdrawn token
        // Nullifiers[1]: nullify extra token (if one not enough, placeholder, can be zero)
        // Nullifiers[2]: nullify fee token
        // Nullifiers[3]: nullify extra fee token (placeholder, can be zero)
        let fee = self.create_variable(public_inputs.fee)?;
        let roots = public_inputs
            .roots
            .iter()
            .map(|root| self.create_variable(*root))
            .collect::<Result<Vec<Variable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;

        let PrivateInputsVar {
            fee_token_id,
            nf_address,
            nf_slot_id,
            nullifiers_values,
            nullifiers_salts,
            membership_proofs,
            commitments_values,
            sender_commitment_salts,
            public_keys,
            root_key,
            ephemeral_key,
            withdraw_address,
            withdraw_flag,
            secret_preimages,
            party_a_public_key,
            party_b_public_key,
            nf_token_a_id,
            value_a,
            nf_token_b_id,
            value_b,
            swap_nonce,
            deadline,
        } = PrivateInputsVar::from_private_inputs(private_inputs, self)?;

        // Check that the withdraw address is in range
        self.enforce_in_range(withdraw_address, 160)?;
        // Check that the nightfall address is in range
        self.enforce_in_range(nf_address, 160)?;

        // KEY DERIVATION
        let pub_point = self
            .create_constant_point_variable(&Point::<Fr254>::from(
                Affine::<BabyJubjub>::generator(),
            ))?;

        // Constrain nullifier_key from root_key
        let nullifier_prefix = self.create_constant_variable(Fr254::from(
            BigUint::parse_bytes(NULLIFIER_PREFIX.as_bytes(), 10).unwrap(),
        ))?;
        let nullifier_key = self.poseidon_hash(&[root_key, nullifier_prefix])?;

        // Derive a dedicated private-key hash from root_key using a fixed domain-separation prefix.
        let private_prefix = self.create_constant_variable(Fr254::from(
            BigUint::parse_bytes(PRIVATE_KEY_PREFIX.as_bytes(), 10).unwrap(),
        ))?;
        let fr_zkp_priv_key = self.poseidon_hash(&[root_key, private_prefix])?;
        let fr_zkp_priv_key_val = self.witness(fr_zkp_priv_key)?;

        // Convert the BN254 hash output into a BabyJubjub scalar by modular reduction.
        // The remainder is required because BabyJubjub scalar multiplication expects scalars
        // in [0, BJJ_ORDER), while fr_zkp_priv_key is an element of the larger BN254 field.
        let hash_bigint = BigUint::from(BigInteger256::from(fr_zkp_priv_key_val));
        let bjj_order_bigint = BigUint::from(BJJScalar::MODULUS);
        let zkp_private_key_val = Fr254::from(&hash_bigint % &bjj_order_bigint);
        let zkp_private_key = self.create_variable(zkp_private_key_val)?;

        // Prove in-circuit that reduction was done correctly:
        // fr_zkp_priv_key = zkp_private_key + lambda * BJJ_ORDER
        let lambda_val = Fr254::from(&hash_bigint / &bjj_order_bigint);
        let lambda = self.create_variable(lambda_val)?;
        let bjj_scalar_order = Fr254::from(BJJScalar::MODULUS);

        self.lin_comb_gate(
            &[Fr254::one(), bjj_scalar_order],
            &Fr254::zero(),
            &[zkp_private_key, lambda],
            &fr_zkp_priv_key,
        )?;
        // Enforce a canonical remainder and a bounded quotient for BN254 -> BJJ reduction.
        // For any BN254 element h, lambda = floor(h / BJJ_ORDER).
        // Since h <= Fr254::MODULUS - 1, the maximum possible quotient is
        // floor((Fr254::MODULUS - 1) / BJJScalar::MODULUS) = 7, so lambda < 8.

        self.enforce_lt_constant(zkp_private_key, bjj_scalar_order)?;
        self.enforce_lt_constant(lambda, Fr254::from(8u64))?;

        // Calculate zkp_public_key from zkp_private_key
        let zkp_pub_key =
            self.variable_base_scalar_mul::<BabyJubjub>(zkp_private_key, &pub_point)?;

        let swap_nonce_is_zero = self.is_zero(swap_nonce)?;
        let is_swap = self.logic_neg(swap_nonce_is_zero)?;
        // ROLE DETECTION & DERIVED VALUES
        // Determines caller's role and derives value, nf_token_id,
        // and recipient_public_key from swap parameters.
        //
        // For swap:
        //   party A → spends token_a/value_a, recipient = party_b
        //   party B → spends token_b/value_b, recipient = party_a
        // For transfer/withdraw:
        //   always party A → spends token_a/value_a, recipient = party_b
        let my_pk_x_eq_a = self.is_equal(zkp_pub_key.get_x(), party_a_public_key.get_x())?;
        let my_pk_y_eq_a = self.is_equal(zkp_pub_key.get_y(), party_a_public_key.get_y())?;
        let is_party_a = self.logic_and(my_pk_x_eq_a, my_pk_y_eq_a)?;

        // Canonicalize non-swap witnesses so transfer/withdraw do not carry
        // free swap-leg variables.
        self.mul_gate(swap_nonce_is_zero.into(), nf_token_b_id, self.zero())?;
        self.mul_gate(swap_nonce_is_zero.into(), value_b, self.zero())?;
        let not_party_a = self.logic_neg(is_party_a)?;
        let invalid_non_swap_party_a = self.logic_and(swap_nonce_is_zero, not_party_a)?;
        self.enforce_false(invalid_non_swap_party_a.into())?;

        // Swap-specific: derive from role
        let swap_value = self.conditional_select(is_party_a, value_b, value_a)?;
        let swap_nf_token_id = self.conditional_select(is_party_a, nf_token_b_id, nf_token_a_id)?;
        let swap_recipient_x = self.conditional_select(
            is_party_a,
            party_a_public_key.get_x(),
            party_b_public_key.get_x(),
        )?;
        let swap_recipient_y = self.conditional_select(
            is_party_a,
            party_a_public_key.get_y(),
            party_b_public_key.get_y(),
        )?;

        // Final: for transfer use value_a/party_b directly, for swap use role-based
        let value = self.conditional_select(is_swap, value_a, swap_value)?;
        let nf_token_id = self.conditional_select(is_swap, nf_token_a_id, swap_nf_token_id)?;
        let recipient_x =
            self.conditional_select(is_swap, party_b_public_key.get_x(), swap_recipient_x)?;
        let recipient_y =
            self.conditional_select(is_swap, party_b_public_key.get_y(), swap_recipient_y)?;

        let recipient_public_key = PointVariable::TE(recipient_x, recipient_y);

        // BALANCE CHECKS
        // commitments_values[0]: transfer/withdraw change value
        // commitments_values[1]: fee change value
        // nullifiers_values[0]: first token's value for transfer/withdraw
        // nullifiers_values[1]: second token's value for transfer/withdraw
        // nullifiers_values[2]: first fee token's value for transfer/withdraw
        // nullifiers_values[3]: second fee token's value for transfer/withdraw

        // value + change = nullifier[0] + nullifier[1]
        self.lc_gate(
            &[
                value,
                commitments_values[0],
                nullifiers_values[0],
                nullifiers_values[1],
                self.zero(),
            ],
            &[Fr254::one(), Fr254::one(), -Fr254::one(), -Fr254::one()],
        )?;
        // fee + fee_change = fee_nullifier[0] + fee_nullifier[1]
        self.lc_gate(
            &[
                fee,
                commitments_values[1],
                nullifiers_values[2],
                nullifiers_values[3],
                self.zero(),
            ],
            &[Fr254::one(), Fr254::one(), -Fr254::one(), -Fr254::one()],
        )?;
        // We range check `value`, `fee`, `commitments_values[0]` and `commitments_values[1]`
        // If we don't do this the client send "negative" values that result in huge
        // change commitments due to a wrap around error.
        // We choose 96 bits, as this seems like a reasonable upper limit for a transfer.
        // In addition 96 is divisible by 8, which makes it slightly cheaper to range check.
        self.enforce_in_range(value, 96)?;
        self.enforce_in_range(fee, 96)?;
        self.enforce_in_range(commitments_values[0], 96)?;
        self.enforce_in_range(commitments_values[1], 96)?;

        // OWNERSHIP VERIFICATION (for all: transfer, withdraw, swap)
        for i in 0..4 {
            let is_neutral = self.is_neutral_point::<BabyJubjub>(&public_keys[i])?;
            let is_zero_value = self.is_zero(nullifiers_salts[i])?;

            let x_matches = self.is_equal(zkp_pub_key.get_x(), public_keys[i].get_x())?;
            let y_matches = self.is_equal(zkp_pub_key.get_y(), public_keys[i].get_y())?;
            let key_matches = self.logic_and(x_matches, y_matches)?;

            let skip = self.logic_or(is_neutral, is_zero_value)?;

            // skip OR key_matches == 1
            self.quad_poly_gate(
                &[
                    skip.into(),
                    key_matches.into(),
                    self.zero(),
                    self.zero(),
                    self.one(),
                ],
                &[Fr254::one(), Fr254::one(), Fr254::zero(), Fr254::zero()],
                &[-Fr254::one(), Fr254::zero()],
                Fr254::one(),
                Fr254::zero(),
            )?;
        }

        // SWAP LOGIC (only enforced if is_swap)
        // 1. Verify swap_link hash
        // Domain separator for swap_link hash.
        // `SWAP_V1` is a protocol constant; keep it fixed for compatibility.
        // Change it only in a coordinated breaking protocol upgrade if swap-link schema changes.
        let swap_domain =
            self.create_constant_variable(Fr254::from_le_bytes_mod_order(b"SWAP_V1"))?;
        let initial_state = PoseidonStateVar([self.zero(), self.zero(), self.zero(), self.zero()]);
        let absorbed_state = self.absorb(
            &initial_state,
            &[
                swap_domain,
                party_a_public_key.get_x(),
                party_a_public_key.get_y(),
                party_b_public_key.get_x(),
                party_b_public_key.get_y(),
                nf_token_a_id,
                value_a,
                nf_token_b_id,
                value_b,
                swap_nonce,
            ],
        )?;
        let computed_swap_link = self.squeeze(&absorbed_state, 1)?[0];

        // Swap-only input constraints:
        // - party keys must be non-neutral
        // - party keys must be distinct
        // - nonce/deadline must fit 64 bits
        let a_is_neutral = self.is_neutral_point::<BabyJubjub>(&party_a_public_key)?;
        let b_is_neutral = self.is_neutral_point::<BabyJubjub>(&party_b_public_key)?;
        let a_neutral_and_swap = self.logic_and(is_swap, a_is_neutral)?;
        self.enforce_false(a_neutral_and_swap.into())?;
        let b_neutral_and_swap = self.logic_and(is_swap, b_is_neutral)?;
        self.enforce_false(b_neutral_and_swap.into())?;

        let pk_x_eq = self.is_equal(party_a_public_key.get_x(), party_b_public_key.get_x())?;
        let pk_y_eq = self.is_equal(party_a_public_key.get_y(), party_b_public_key.get_y())?;
        let pk_equal = self.logic_and(pk_x_eq, pk_y_eq)?;
        let pk_equal_and_swap = self.logic_and(is_swap, pk_equal)?;
        self.enforce_false(pk_equal_and_swap.into())?;

        self.enforce_in_range(swap_nonce, 64)?;
        self.enforce_in_range(deadline, 64)?;
        self.mul_gate(swap_nonce_is_zero.into(), deadline, self.zero())?;

        let final_swap_link = self.conditional_select(is_swap, self.zero(), computed_swap_link)?;

        // 2. Verify exclusive role: my_pk == party_a XOR my_pk == party_b
        let my_pk_x_eq_b = self.is_equal(zkp_pub_key.get_x(), party_b_public_key.get_x())?;
        let my_pk_y_eq_b = self.is_equal(zkp_pub_key.get_y(), party_b_public_key.get_y())?;
        let is_party_b = self.logic_and(my_pk_x_eq_b, my_pk_y_eq_b)?;

        // Exactly one role: is_party_a + is_party_b == 1
        let role_sum = self.add(is_party_a.into(), is_party_b.into())?;
        let role_is_one = self.is_equal(role_sum, self.one())?;
        let role_valid = self.conditional_select(is_swap, self.one(), role_is_one.into())?;
        self.enforce_true(role_valid)?;

        // MUTUAL EXCLUSION: Cannot be both swap and withdraw
        let both_swap_and_withdraw = self.logic_and(is_swap, withdraw_flag)?;
        self.enforce_false(both_swap_and_withdraw.into())?;

        //  SHARED SECRET
        // Calculate shared_secret
        let shared_secret =
            self.variable_base_scalar_mul::<BabyJubjub>(ephemeral_key, &recipient_public_key)?;

        // Calculate new commitments
        let domain_shared_salt = self.create_variable(DOMAIN_SHARED_SALT)?;
        let shared_salt = self.poseidon_hash(&[
            shared_secret.get_x(),
            shared_secret.get_y(),
            domain_shared_salt,
        ])?;

        let commitments = self.verify_commitments(
            fee_token_id,
            nf_address,
            nf_token_id,
            nf_slot_id,
            value,
            fee,
            shared_salt,
            &commitments_values,
            &[recipient_public_key, zkp_pub_key],
            &sender_commitment_salts,
            withdraw_flag,
        )?;

        // Calculate nullifiers
        let nullifiers = self.verify_nullifiers::<BabyJubjub>(
            fee_token_id,
            nf_token_id,
            nf_slot_id,
            nullifier_key,
            &public_keys,
            &roots,
            &nullifiers_values,
            &nullifiers_salts,
            &membership_proofs,
            &secret_preimages,
        )?;

        // No duplications in nullifiers and commitments unless they are zero
        self.verify_duplicates(&nullifiers, &commitments)?;

        // Verify encryption of recipient's commitment preimage
        let public_data = self.verify_encryption(
            nf_token_id,
            nf_slot_id,
            value,
            &shared_secret,
            ephemeral_key,
            withdraw_address,
            withdraw_flag,
        )?;

        // If withdrawing, the recipient public key should be the neutral point
        let is_neutral = self.is_neutral_point::<BabyJubjub>(&recipient_public_key)?;
        self.quad_poly_gate(
            &[
                is_neutral.into(),
                withdraw_flag.into(),
                self.zero(),
                self.zero(),
                self.one(),
            ],
            &[-Fr254::one(), -Fr254::one(), Fr254::zero(), Fr254::zero()],
            &[Fr254::from(2u8), Fr254::zero()],
            Fr254::one(),
            Fr254::one(),
        )?;

        // PUBLIC INPUTS
        // We set the relevant variables to be public here in the order:
        // hash initialisation (domain tag, version)
        // fee
        // roots
        // commitments
        // nullifiers
        // compressed_secrets
        // swap_link
        // deadline
        // swap_side
        let mut init_bytes = "public_inputs".as_bytes().to_vec();
        init_bytes.extend_from_slice("version2".as_bytes());
        let init_pi_var =
            self.create_constant_variable(Fr254::from_le_bytes_mod_order(init_bytes.as_slice()))?;
        self.set_variable_public(init_pi_var)?;

        let fee_len_sep = self.create_constant_variable(Fr254::from(1u8))?;
        self.set_variable_public(fee_len_sep)?;
        self.set_variable_public(fee)?;
        let fee = self.witness(fee)?;

        let roots_len_sep = self.create_constant_variable(Fr254::from(4u8))?;
        self.set_variable_public(roots_len_sep)?;
        let roots: [Fr254; 4] = roots
            .iter()
            .map(|&root| {
                self.set_variable_public(root)?;
                self.witness(root)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert roots to fixed length array".to_string(),
                )
            })?;

        let comms_len_sep = self.create_constant_variable(Fr254::from(4u8))?;
        self.set_variable_public(comms_len_sep)?;
        let commitments: [Fr254; 4] = commitments
            .iter()
            .map(|&commitment| {
                self.set_variable_public(commitment)?;
                self.witness(commitment)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert commitments to fixed length array".to_string(),
                )
            })?;

        let nulls_len_sep = self.create_constant_variable(Fr254::from(4u8))?;
        self.set_variable_public(nulls_len_sep)?;
        let nullifiers: [Fr254; 4] = nullifiers
            .iter()
            .map(|&nullifier| {
                self.set_variable_public(nullifier)?;
                self.witness(nullifier)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert nullifiers to fixed length array".to_string(),
                )
            })?;

        let comp_secs_len_sep = self.create_constant_variable(Fr254::from(5u8))?;
        self.set_variable_public(comp_secs_len_sep)?;
        let compressed_secrets: [Fr254; 5] = public_data
            .iter()
            .map(|&pd| {
                self.set_variable_public(pd)?;
                self.witness(pd)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert public data to fixed length array".to_string(),
                )
            })?;

        // === Swap link ===
        let swap_link_len_sep = self.create_constant_variable(Fr254::from(1u8))?;
        self.set_variable_public(swap_link_len_sep)?;
        self.set_variable_public(final_swap_link)?;
        let swap_link_out = self.witness(final_swap_link)?;

        // === Deadline ===
        let deadline_len_sep = self.create_constant_variable(Fr254::from(1u8))?;
        self.set_variable_public(deadline_len_sep)?;
        let final_deadline = self.conditional_select(is_swap, self.zero(), deadline)?;
        self.set_variable_public(final_deadline)?;
        let deadline_out = self.witness(final_deadline)?;

        // swap_side is a public role bit for swap matching:
        // - 1 => prover is party A
        // - 0 => prover is party B
        // Proposer checks complementary sides (A+B) for the same swap_link to avoid
        // pairing two same-side swap legs.
        let swap_side_len_sep = self.create_constant_variable(Fr254::from(1u8))?;
        self.set_variable_public(swap_side_len_sep)?;
        let final_side = self.conditional_select(is_swap, self.zero(), is_party_a.into())?;
        self.set_variable_public(final_side)?;
        let swap_side_out = self.witness(final_side)?;

        let full_public_inputs = private_inputs.clone();
        Ok((
            PublicInputs::new()
                .fee(fee)
                .roots(&roots)
                .commitments(&commitments)
                .nullifiers(&nullifiers)
                .compressed_secrets(&compressed_secrets)
                .swap_link(swap_link_out)
                .deadline(deadline_out)
                .swap_side(swap_side_out)
                .build(),
            full_public_inputs,
        ))
    }
}

/// This function takes mutable references to the public_input (only need fee and roots values)
/// and private inputs and returns a PlonkCircuit
/// It will modify public_input and fill correct values for the rest of public_input
pub fn unified_circuit_builder(
    public_input: &mut PublicInputs,
    private_input: &mut PrivateInputs,
) -> Result<PlonkCircuit<Fr254>, PlonkError> {
    let mut circuit = PlonkCircuit::<Fr254>::new_ultra_plonk(8);
    (*public_input, *private_input) =
        circuit.assess_operation_integrity(public_input, private_input)?;
    Ok(circuit)
}
