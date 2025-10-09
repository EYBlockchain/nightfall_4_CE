use ark_ff::PrimeField;

use jf_relation::{errors::CircuitError, Circuit, PlonkCircuit, Variable};

pub trait VerifyDuplicatesCircuit<F>
where
    F: PrimeField,
{
    fn verify_duplicates(
        &mut self,
        nullifiers_vars: &[Variable; 4],
        commitments_vars: &[Variable; 4],
    ) -> Result<(), CircuitError>;
}

impl<F> VerifyDuplicatesCircuit<F> for PlonkCircuit<F>
where
    F: PrimeField,
{
    fn verify_duplicates(
        &mut self,
        nullifiers_vars: &[Variable; 4],
        commitments_vars: &[Variable; 4],
    ) -> Result<(), CircuitError> {
        // Check that no nullifiers are duplicated.
        for i in 0..4 {
            for j in i + 1..4 {
                let tmp_one = self.is_equal(nullifiers_vars[j], self.zero())?;
                let tmp_two = self.is_equal(nullifiers_vars[j], nullifiers_vars[i])?;
                let tmp_three = self.logic_neg(tmp_two)?;
                self.logic_or_gate(tmp_one, tmp_three)?;
            }
        }

        // Check that no commitments are duplicated.
        for i in 0..4 {
            for j in i + 1..4 {
                let tmp_one = self.is_equal(commitments_vars[j], self.zero())?;
                let tmp_two = self.is_equal(commitments_vars[j], commitments_vars[i])?;
                let tmp_three = self.logic_neg(tmp_two)?;
                self.logic_or_gate(tmp_one, tmp_three)?;
            }
        }

        Ok(())
    }
}
