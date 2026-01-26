use ark_ff::PrimeField;
use jf_primitives::{
    circuit::{poseidon::PoseidonHashGadget, tree::structs::MembershipProofVar},
    poseidon::PoseidonParams,
};
use jf_relation::{
    errors::CircuitError,
    gadgets::ecc::{HasTEForm, PointVariable},
    Circuit, PlonkCircuit, Variable,
};
pub trait VerifyNullifiersCircuit<F>
where
    F: PrimeField,
{
    #[allow(clippy::too_many_arguments)]
    fn verify_nullifiers<P: HasTEForm<BaseField = F>>(
        &mut self,
        fee_token_id: Variable,
        nf_token_id: Variable, // could be nf4_token_id or fee_token_id
        nf_slot_id: Variable,
        nullifiers_key: Variable,
        public_keys: &[PointVariable; 4],
        roots: &[Variable; 4],
        old_commitment_values: &[Variable; 4],
        old_commitment_salts: &[Variable; 4],
        membership_proofs: &[MembershipProofVar],
        secret_preimages: &[[Variable; 3]; 4],
    ) -> Result<[Variable; 4], CircuitError>;
}

impl<F> VerifyNullifiersCircuit<F> for PlonkCircuit<F>
where
    F: PoseidonParams,
{
    #[allow(clippy::too_many_arguments)]
    fn verify_nullifiers<P: HasTEForm<BaseField = F>>(
        &mut self,
        fee_token_id: Variable,
        nf_token_id: Variable,
        nf_slot_id: Variable,
        nullifiers_key: Variable,
        public_keys: &[PointVariable; 4],
        roots: &[Variable; 4],
        old_commitment_values: &[Variable; 4],
        old_commitment_salts: &[Variable; 4],
        membership_proofs: &[MembershipProofVar],
        secret_preimages: &[[Variable; 3]; 4],
    ) -> Result<[Variable; 4], jf_relation::errors::CircuitError> {
        // Check the first nullifier, nullify Withdrawn/Transferred token
        let commitment_hash_1 = self.poseidon_hash(&[
            nf_token_id,
            nf_slot_id,
            old_commitment_values[0],
            public_keys[0].get_x(),
            public_keys[0].get_y(),
            old_commitment_salts[0],
        ])?;

        // Calculate the commitment's nullifier
        let secret_hash = self.poseidon_hash(&secret_preimages[0])?;
        let neutral_point = self.is_neutral_point::<P>(&public_keys[0])?;
        let key_to_use = self.conditional_select(neutral_point, nullifiers_key, secret_hash)?;
        let nullifier_1 = self.poseidon_hash(&[key_to_use, commitment_hash_1])?;

        // Check if the nullifier is equal to the public transaction nullifier hash, or input commitment value is zero
        // Check if the Merkle root is equal to the supplied one.
        let calc_root = membership_proofs[0].calculate_new_root(self, &commitment_hash_1)?;
        self.enforce_equal(calc_root, roots[0])?;

        // Finally we check if the salt is from a secret preimage
        let secret_hash = self.poseidon_hash(&secret_preimages[0])?;

        let neutral_point = self.is_neutral_point::<P>(&public_keys[0])?;

        let salt_to_enforce =
            self.conditional_select(neutral_point, old_commitment_salts[0], secret_hash)?;
        self.enforce_equal(salt_to_enforce, old_commitment_salts[0])?;

        // Check the second nullifier, nullify extra Withdrawn/Transferred token
        let is_zero = self.is_zero(old_commitment_values[1])?;

        let commitment_hash_2 = self.poseidon_hash(&[
            nf_token_id,
            nf_slot_id,
            old_commitment_values[1],
            public_keys[1].get_x(),
            public_keys[1].get_y(),
            old_commitment_salts[1],
        ])?;
        // Calculate the commitment's nullifier
        let secret_hash_2 = self.poseidon_hash(&secret_preimages[1])?;
        let neutral_point_2 = self.is_neutral_point::<P>(&public_keys[1])?;
        let key_to_use_2 = self.conditional_select(neutral_point_2, nullifiers_key, secret_hash_2)?;
        let nullifier_2 = self.poseidon_hash(&[key_to_use_2, commitment_hash_2])?;

        // Check if the nullifier is equal to the public transaction nullifier hash, or input commitment value is zero
        // Check if the Merkle root is equal to the supplied one.

        let calc_root = membership_proofs[1].calculate_new_root(self, &commitment_hash_2)?;
        let root_is_equal = self.is_equal(calc_root, roots[1])?;

        // If commitment value is zero nullifier will directly be considered valid.
        let is_valid = self.conditional_select(is_zero, root_is_equal.into(), self.one())?;
        self.enforce_true(is_valid)?;
        let nullifier_2_out = self.conditional_select(is_zero, nullifier_2, self.zero())?;

        // Finally we check if the salt is from a secret preimage
        let secret_hash = self.poseidon_hash(&secret_preimages[1])?;

        let neutral_point = self.is_neutral_point::<P>(&public_keys[1])?;

        let salt_to_enforce =
            self.conditional_select(neutral_point, old_commitment_salts[1], secret_hash)?;
        let salt_to_enforce = self.conditional_select(is_zero, salt_to_enforce, self.zero())?;
        self.enforce_equal(salt_to_enforce, old_commitment_salts[1])?;

        // Check the third nullifier, nullify fee token used to pay
        let is_zero = self.is_zero(old_commitment_values[2])?;

        let commitment_hash_3 = self.poseidon_hash(&[
            fee_token_id,
            fee_token_id,
            old_commitment_values[2],
            public_keys[2].get_x(),
            public_keys[2].get_y(),
            old_commitment_salts[2],
        ])?;

        // Calculate the commitment's nullifier
        let secret_hash_3 = self.poseidon_hash(&secret_preimages[2])?;
        let neutral_point_3 = self.is_neutral_point::<P>(&public_keys[2])?;
        let key_to_use_3 = self.conditional_select(neutral_point_3, nullifiers_key, secret_hash_3)?;
        let nullifier_3 = self.poseidon_hash(&[key_to_use_3, commitment_hash_3])?;

        // Check if the nullifier is equal to the public transaction nullifier hash, or input commitment value is zero
        // Check if the Merkle root is equal to the supplied one.
        let calc_root = membership_proofs[2].calculate_new_root(self, &commitment_hash_3)?;

        let root_is_equal = self.is_equal(calc_root, roots[2])?;

        // If commitment value is zero nullifier will directly be considered valid.
        let is_valid = self.conditional_select(is_zero, root_is_equal.into(), self.one())?;
        self.enforce_true(is_valid)?;
        let nullifier_3_out = self.conditional_select(is_zero, nullifier_3, self.zero())?;

        // Finally we check if the salt is from a secret preimage
        let secret_hash = self.poseidon_hash(&secret_preimages[2])?;

        let neutral_point = self.is_neutral_point::<P>(&public_keys[2])?;

        let salt_to_enforce =
            self.conditional_select(neutral_point, old_commitment_salts[2], secret_hash)?;
        let salt_to_enforce = self.conditional_select(is_zero, salt_to_enforce, self.zero())?;
        self.enforce_equal(salt_to_enforce, old_commitment_salts[2])?;

        // Check the third nullifier
        let is_zero = self.is_zero(old_commitment_values[3])?;

        let commitment_hash_4 = self.poseidon_hash(&[
            fee_token_id,
            fee_token_id,
            old_commitment_values[3],
            public_keys[3].get_x(),
            public_keys[3].get_y(),
            old_commitment_salts[3],
        ])?;

        // Calculate the commitment's nullifier
        let secret_hash_4 = self.poseidon_hash(&secret_preimages[3])?;
        let neutral_point_4 = self.is_neutral_point::<P>(&public_keys[3])?;
        let key_to_use_4 = self.conditional_select(neutral_point_4, nullifiers_key, secret_hash_4)?;
        let nullifier_4 = self.poseidon_hash(&[key_to_use_4, commitment_hash_4])?;

        // Check if the nullifier is equal to the public transaction nullifier hash, or input commitment value is zero
        // Check if the Merkle root is equal to the supplied one.
        let calc_root = membership_proofs[3].calculate_new_root(self, &commitment_hash_4)?;

        let root_is_equal = self.is_equal(calc_root, roots[3])?;

        // If commitment value is zero nullifier will directly be considered valid.
        let is_valid = self.conditional_select(is_zero, root_is_equal.into(), self.one())?;
        self.enforce_true(is_valid)?;
        let nullifier_4_out = self.conditional_select(is_zero, nullifier_4, self.zero())?;

        // Finally we check if the salt is from a secret preimage
        let secret_hash = self.poseidon_hash(&secret_preimages[3])?;

        let neutral_point = self.is_neutral_point::<P>(&public_keys[3])?;

        let salt_to_enforce =
            self.conditional_select(neutral_point, old_commitment_salts[3], secret_hash)?;
        let salt_to_enforce = self.conditional_select(is_zero, salt_to_enforce, self.zero())?;
        self.enforce_equal(salt_to_enforce, old_commitment_salts[3])?;

        Ok([
            nullifier_1,
            nullifier_2_out,
            nullifier_3_out,
            nullifier_4_out,
        ])
    }
}
