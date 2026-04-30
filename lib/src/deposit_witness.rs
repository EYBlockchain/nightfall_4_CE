use crate::shared_entities::DepositData;
use ark_bn254::Fr as Fr254;
use jf_primitives::circuit::{poseidon::PoseidonHashGadget, sha256::Sha256HashGadget};
use jf_relation::{errors::CircuitError, BoolVar, Circuit, PlonkCircuit, Variable};

/// Shared variable representation of deposit witness data.
#[derive(Clone, Copy, Default, Debug)]
pub struct DepositDataVar {
    pub nf_token_id: Variable,
    pub nf_slot_id: Variable,
    pub value: Variable,
    pub secret_hash: Variable,
}

impl DepositDataVar {
    /// Creates the commitment hash from this [`DepositDataVar`].
    pub fn to_commitment(
        &self,
        circuit: &mut PlonkCircuit<Fr254>,
        flag: BoolVar,
    ) -> Result<Variable, CircuitError> {
        let calculated_hash = circuit.poseidon_hash(&[
            self.nf_token_id,
            self.nf_slot_id,
            self.value,
            circuit.zero(),
            circuit.one(),
            self.secret_hash,
        ])?;

        circuit.conditional_select(flag, circuit.zero(), calculated_hash)
    }

    /// Schedules the deposit sha256 output used in public inputs.
    pub fn sha256_and_shift(
        &self,
        circuit: &mut PlonkCircuit<Fr254>,
        lookup_vars: &mut Vec<(Variable, Variable, Variable)>,
        flag: BoolVar,
    ) -> Result<Variable, CircuitError> {
        let (_, sha256_var) = circuit.full_shifted_sha256_hash(
            &[
                self.nf_token_id,
                self.nf_slot_id,
                self.value,
                self.secret_hash,
            ],
            lookup_vars,
        )?;

        circuit.conditional_select(flag, circuit.zero(), sha256_var)
    }

    /// Returns whether this deposit entry is a real deposit entry.
    pub fn is_real(&self, circuit: &mut PlonkCircuit<Fr254>) -> Result<BoolVar, CircuitError> {
        let value_zero = circuit.is_zero(self.value)?;
        let id_zero = circuit.is_zero(self.nf_token_id)?;
        let is_dummy = circuit.logic_and(value_zero, id_zero)?;
        circuit.logic_neg(is_dummy)
    }

    /// Creates a new instance of [`DepositDataVar`] from a [`DepositData`].
    pub fn from_deposit_data(
        data: &DepositData,
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Self, CircuitError> {
        let nf_token_id = circuit.create_variable(data.nf_token_id)?;
        let nf_slot_id = circuit.create_variable(data.nf_slot_id)?;
        let value = circuit.create_variable(data.value)?;
        let secret_hash = circuit.create_variable(data.secret_hash)?;

        Ok(DepositDataVar {
            nf_token_id,
            nf_slot_id,
            value,
            secret_hash,
        })
    }
}
