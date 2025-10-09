use ark_bn254::Fr as Fr254;

use jf_plonk::errors::PlonkError;

use jf_relation::PlonkCircuit;

use lib::nf_client_proof::{PrivateInputs, PublicInputs};

use super::circuits::unified_circuit::unified_circuit_builder;

pub trait CircuitBuilder
where
    Self: Sized,
{
    type Error: std::error::Error;

    fn build_circuit(
        public_inputs: &mut PublicInputs,
        private_inputs: &mut PrivateInputs,
    ) -> Result<Self, Self::Error>;
}

impl CircuitBuilder for PlonkCircuit<Fr254> {
    type Error = PlonkError;

    fn build_circuit(
        public_inputs: &mut PublicInputs,
        private_inputs: &mut PrivateInputs,
    ) -> Result<PlonkCircuit<Fr254>, PlonkError> {
        unified_circuit_builder(public_inputs, private_inputs)
    }
}
