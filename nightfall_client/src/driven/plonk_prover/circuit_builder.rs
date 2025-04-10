use ark_bn254::Fr as Fr254;

use jf_plonk::errors::PlonkError;

use jf_relation::PlonkCircuit;

use crate::ports::proof::{CircuitBuilder, PrivateInputs, PublicInputs};

use super::circuits::unified_circuit::unified_circuit_builder;

impl CircuitBuilder for PlonkCircuit<Fr254> {
    type Error = PlonkError;

    fn build_circuit(
        public_inputs: &mut PublicInputs,
        private_inputs: &mut PrivateInputs,
    ) -> Result<PlonkCircuit<Fr254>, PlonkError> {
        unified_circuit_builder(public_inputs, private_inputs)
    }
}
