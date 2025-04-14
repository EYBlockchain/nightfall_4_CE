use ark_ff::PrimeField;
use jf_primitives::{circuit::poseidon::PoseidonHashGadget, poseidon::PoseidonParams};
use jf_relation::{
    errors::CircuitError, gadgets::ecc::PointVariable, BoolVar, Circuit, PlonkCircuit, Variable,
};

pub trait VerifyCommitmentsCircuit<F>
where
    F: PrimeField,
{
    /// Verify the new commitments being created in this transaction. If its a transfer then the first commitment should be non-zero if its a withdraw the first commitment should be zero.
    #[allow(clippy::too_many_arguments)]
    fn verify_commitments(
        &mut self,
        fee_token_id: Variable,
        nf_address: Variable,
        token_id: Variable,
        slot_id: Variable,
        value: Variable,
        fee: Variable,
        shared_secret_y: Variable,
        new_commitments_values: &[Variable; 2],
        recipient_public_key: &[PointVariable; 2],
        new_commitments_salts: &[Variable; 3],
        withdraw_flag: BoolVar,
    ) -> Result<[Variable; 4], CircuitError>;
}

impl<F> VerifyCommitmentsCircuit<F> for PlonkCircuit<F>
where
    F: PoseidonParams,
{
    #[allow(clippy::too_many_arguments)]
    fn verify_commitments(
        &mut self,
        fee_token_id: Variable,
        nf_address: Variable,
        token_id: Variable,
        slot_id: Variable,
        value: Variable,
        fee: Variable,
        shared_secret_y: Variable,
        new_commitments_values: &[Variable; 2],
        recipient_public_key: &[PointVariable; 2],
        new_commitments_salts: &[Variable; 3],
        withdraw_flag: BoolVar,
    ) -> Result<[Variable; 4], CircuitError> {
        // new_commitments_values[0]: transfer/withdraw change value
        // new_commitments_values[1]: fee change value
        // Check the first commitment, Transfered to Token
        let first_commitment_hash = self.poseidon_hash(&[
            token_id,
            slot_id,
            value,
            recipient_public_key[0].get_x(),
            recipient_public_key[0].get_y(),
            shared_secret_y,
        ])?;
        let first_commitment =
            self.conditional_select(withdraw_flag, first_commitment_hash, self.zero())?;

        // Check the second commitment Transfer Change Token
        let is_transfer_change_zero = self.is_zero(new_commitments_values[0])?;
        let second_commitment_hash = self.poseidon_hash(&[
            token_id,
            slot_id,
            new_commitments_values[0],
            recipient_public_key[1].get_x(),
            recipient_public_key[1].get_y(),
            new_commitments_salts[0],
        ])?;
        let second_commitment =
            self.conditional_select(is_transfer_change_zero, second_commitment_hash, self.zero())?;

        // The third commitment should be the fee which is paid to a nightfall address that can't be nullified. Then on block submission the
        // block proof also provides the total fee that has been paid in the block and is now trapped in these un-nullifiable commitments, allowing the
        // smart contract to safely transfer the funds out to proposers.
        let is_fee_zero = self.is_zero(fee)?;
        let third_commitment_hash = self.poseidon_hash(&[
            fee_token_id,
            fee_token_id,
            fee,
            self.zero(),
            nf_address,
            new_commitments_salts[1],
        ])?;
        let third_commitment =
            self.conditional_select(is_fee_zero, third_commitment_hash, self.zero())?;

        // Check the final commitment Fee Change Token
        let is_fee_change_zero = self.is_zero(new_commitments_values[1])?;

        let final_commitment_hash = self.poseidon_hash(&[
            fee_token_id,
            fee_token_id,
            new_commitments_values[1],
            recipient_public_key[1].get_x(),
            recipient_public_key[1].get_y(),
            new_commitments_salts[2],
        ])?;
        let final_commitment =
            self.conditional_select(is_fee_change_zero, final_commitment_hash, self.zero())?;

        Ok([
            first_commitment,
            second_commitment,
            third_commitment,
            final_commitment,
        ])
    }
}
