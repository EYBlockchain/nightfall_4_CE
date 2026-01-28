use super::verify::verify_duplicates_gadgets::VerifyDuplicatesCircuit;
use super::DOMAIN_SHARED_SALT;
use crate::{
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
use jf_primitives::circuit::poseidon::PoseidonHashGadget;
use jf_relation::{errors::CircuitError, gadgets::ecc::Point, Circuit, PlonkCircuit, Variable};
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use nf_curves::ed_on_bn254::{BabyJubjub, Fq as Fr254};
use num_bigint::BigUint;

/// This trait is used to construct a circuit verify the integrity of withdraw and transfer operations
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
        // Withdraw is considered a special case of Transfer
        // Commitments[0]:transferred value commitment or 0 if withdraw
        // Commitments[1]: Withdraw/Transfer Change Token commitment
        // Commitments[2]: fee paid token commitment
        // Commitments[3]: fee change token commitment
        // Nullifiers[0]: nullify Withdrawn/Transferred token
        // Nullifiers[1]: nullify extra Withdrawn/Transferred token (if one token is not enough for withdrawing, placeholder, can be zero)
        // Nullifiers[2]: nullify fee token used to pay
        // Nullifiers[3]: nullify fee token used to pay(if one token is not enough for paying the fee, placeholder, can be zero)
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
            value,
            nf_token_id,
            nf_slot_id,
            nullifiers_values,
            nullifiers_salts,
            membership_proofs,
            commitments_values,
            commitments_salts,
            public_keys,
            recipient_public_key,
            zkp_private_key,
            root_key,
            ephemeral_key,
            withdraw_address,
            withdraw_flag,
            secret_preimages,
        } = PrivateInputsVar::from_private_inputs(private_inputs, self)?;
        // Check that the withdraw address is in range
        self.enforce_in_range(withdraw_address, 160)?;
        // Check that the nightfall address is in range
        self.enforce_in_range(nf_address, 160)?;

        // commitments_values[0]: transfer/withdraw change value
        // commitments_values[1]: fee change value
        // nullifiers_values[0]: first token's value for transfer/withdraw
        // nullifiers_values[1]: second token's value for transfer/withdraw
        // nullifiers_values[2]: first fee token's value for transfer/withdraw
        // nullifiers_values[3]: second token's value for transfer/withdraw

        // We check that the first two commitments and first two nullifiers have the same value
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
        // Now we do the same with the fee related commitments
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

        let pub_point =
            self.create_point_variable(&Point::<Fr254>::from(Affine::<BabyJubjub>::generator()))?;

        // Calculate zkpPublicKey and constrain nullifier_key and zkp_private_key_fr254 from root_key
        let zkp_pub_key =
            self.variable_base_scalar_mul::<BabyJubjub>(zkp_private_key, &pub_point)?;

        // Verify that one of the public keys matches the zkp public key unless the public key is neutral point or nullifier value is zero
        for i in 0..4 {
            let is_neutral = self.is_neutral_point::<BabyJubjub>(&public_keys[i])?;
            let is_zero_value = self.is_zero(nullifiers_values[i])?;

            let x_matches = self.is_equal(zkp_pub_key.get_x(), public_keys[i].get_x())?;
            let y_matches = self.is_equal(zkp_pub_key.get_y(), public_keys[i].get_y())?;
            let key_matches = self.logic_and(x_matches, y_matches)?;

            let skip = self.logic_or(is_neutral, is_zero_value)?;
            let valid = self.logic_or(skip, key_matches)?;
            self.enforce_true(valid.into())?;
        }

        // Constrain nullifier_key from root_key
        let nullifier_prefix = self.create_constant_variable(Fr254::from(
            BigUint::parse_bytes(
                b"7805187439118198468809896822299973897593108379494079213870562208229492109015",
                10,
            )
            .unwrap(),
        ))?;
        let nullifier_key = self.poseidon_hash(&[root_key, nullifier_prefix])?;

        // Constrain zkp_private_key_fr254 from root_key
        let private_prefix = self.create_constant_variable(Fr254::from(
            BigUint::parse_bytes(
                b"2708019456231621178814538244712057499818649907582893776052749473028258908910",
                10,
            )
            .unwrap(),
        ))?;
        let expected_zkp_priv = self.poseidon_hash(&[root_key, private_prefix])?;

        let expected_zkp_priv_val = self.witness(expected_zkp_priv)?;
        let zkp_private_key_val = self.witness(zkp_private_key)?;

        let hash_bigint = BigUint::from(BigInteger256::from(expected_zkp_priv_val));
        let zkp_bigint = BigUint::from(BigInteger256::from(zkp_private_key_val));
        let bjj_order_bigint = BigUint::from(BJJScalar::MODULUS);
        let lambda_val = Fr254::from((&hash_bigint - &zkp_bigint) / &bjj_order_bigint);

        let lambda = self.create_variable(lambda_val)?;

        // BJJ Scalar Order constant
        let bjj_scalar_order = Fr254::from(BJJScalar::MODULUS);

        self.lin_comb_gate(
            &[Fr254::one(), bjj_scalar_order],
            &Fr254::zero(),
            &[zkp_private_key, lambda],
            &expected_zkp_priv,
        )?;
        self.enforce_lt_constant(zkp_private_key, bjj_scalar_order)?;
        self.enforce_lt_constant(lambda, Fr254::from(8u64))?; // Verify BiiScaler Lambda is small

        // Calculate the shared secret for the encryption/first commitment
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
            &commitments_salts,
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

        // no duplications in nullifiers and commitments unless they are zero

        self.verify_duplicates(&nullifiers, &commitments)?;

        // Perform the encryption of the recipient's commitment preimage was performed appropriately
        let public_data = self.verify_encryption(
            nf_token_id,
            nf_slot_id,
            value,
            &shared_secret,
            ephemeral_key,
            withdraw_address,
            withdraw_flag,
        )?;

        // If we are withdrawing the recipient public key should be the neutral point.
        let is_neutral = self.is_neutral_point::<BabyJubjub>(&recipient_public_key)?;
        // We achieve this by using the withdraw flag and neutral point check.

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

        // We set the relevant variables to be public here in the order:
        // hash initialisation (domain tag, version)
        // fee
        // roots
        // commitments
        // nullifiers
        // compressed_secrets
        let mut init_bytes = "public_inputs".as_bytes().to_vec();
        init_bytes.extend_from_slice("version1".as_bytes());
        let init_pi_var =
            self.create_constant_variable(Fr254::from_le_bytes_mod_order(init_bytes.as_slice()))?;
        self.set_variable_public(init_pi_var)?;

        //We insert length separators for each section of the public inputs

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

        // return full PublicInputs
        let full_public_inputs = private_inputs.clone();
        Ok((
            PublicInputs::new()
                .fee(fee)
                .roots(&roots)
                .commitments(&commitments)
                .nullifiers(&nullifiers)
                .compressed_secrets(&compressed_secrets)
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
