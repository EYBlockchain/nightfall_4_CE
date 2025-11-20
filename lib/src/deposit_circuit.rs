//! This module contains the code for generating the deposit proofs, these are made by the proposer because they deal with
//! sha256 hashes within a circuit.
use crate::{entities::DepositData, nf_client_proof::PublicInputs};
use ark_bn254::Fr as Fr254;
use ark_ff::{PrimeField, Zero};
use jf_primitives::circuit::{poseidon::PoseidonHashGadget, sha256::Sha256HashGadget};
use jf_relation::{errors::CircuitError, BoolVar, Circuit, PlonkCircuit, Variable};

/// Storing the 'Variable's needed to crete a deposit commitment.
/// Note that if it's a deposit_fee, nf_token_id and nf_slot_id will be fee_token_id,
/// value will be deposit_fee. These are handled onchain automatically in escrow fund function.
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
        // Calculate the commitment.
        let calculated_hash = circuit.poseidon_hash(&[
            self.nf_token_id,
            self.nf_slot_id,
            self.value,
            circuit.zero(),
            circuit.one(),
            self.secret_hash,
        ])?;

        // If flag is true we output the zero commitment, else the calculated commitment
        circuit.conditional_select(flag, calculated_hash, circuit.zero())
    }
    /// Schedules the sha256 hash of this [`DepositDataVar`] to be calculated.
    /// Returns the output of the hash right shifted by 4 so it is in agreement with Nightfall.sol.
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

        // If flag is true then this was a dummy variable and so we want to output the zero hash.
        circuit.conditional_select(flag, sha256_var, circuit.zero())
    }
    /// Returns a [`BoolVar`] indicating whether this real deposit data or dummied to be zero
    pub fn is_real(&self, circuit: &mut PlonkCircuit<Fr254>) -> Result<BoolVar, CircuitError> {
        // We check if both value and token id are zero, if this is the case then the this is a dummy entry and we output true.
        let value_zero = circuit.is_zero(self.value)?;
        let id_zero = circuit.is_zero(self.nf_token_id)?;
        circuit.logic_and(value_zero, id_zero)
    }

    /// Creates a new instance of [`DepositDataVar`] from a [`DepositData`]
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

pub trait DepositCircuitGadget<F>
where
    F: PrimeField,
{
    fn build_deposit_circuit(
        &mut self,
        deposit_data: &[DepositData; 4],
    ) -> Result<PublicInputs, CircuitError>;
}

impl DepositCircuitGadget<Fr254> for PlonkCircuit<Fr254> {
    fn build_deposit_circuit(
        &mut self,
        deposit_data: &[DepositData; 4],
    ) -> Result<PublicInputs, CircuitError> {
        // First we convert the inputs into variable form.
        let data_vars = deposit_data
            .iter()
            .map(|data| DepositDataVar::from_deposit_data(data, self))
            .collect::<Result<Vec<DepositDataVar>, CircuitError>>()?;
        // Now work out if each of the deposit data is real data
        let flags = data_vars
            .iter()
            .map(|var| var.is_real(self))
            .collect::<Result<Vec<BoolVar>, CircuitError>>()?;
        // Next we calculate the output commitment hashes
        let new_commitments = data_vars
            .iter()
            .zip(flags.iter())
            .map(|(var, &flag)| var.to_commitment(self, flag))
            .collect::<Result<Vec<Variable>, CircuitError>>()?;

        // Make the vector of lookup variables to push to and perform the sha hashing.
        let mut lookup_vars = Vec::<(Variable, Variable, Variable)>::new();
        let mut sha_outputs = data_vars
            .iter()
            .zip(flags.iter())
            .map(|(var, &flag)| var.sha256_and_shift(self, &mut lookup_vars, flag))
            .collect::<Result<Vec<Variable>, CircuitError>>()?;
        // We push a zero variable because public data is always 5 field elements (the final two get compressed together but compressing with zero doesn't change the fourth element)
        sha_outputs.push(self.zero());

        // Finalize the sha hash
        self.finalize_for_sha256_hash(&mut lookup_vars)?;

        // Make the relevant variables public
        // fee is special in a deposit proof, it's set to zero on purpose.
        // because fee for deposit transactions are handled on chain,
        // unlike trasnfer/withdraw transactions, where we need create fee commitment.
        // in a deposit proof, we only create commitments from DepositData.
        // if there are deposit_fee, then there will be deposit_fee commitments,
        // otherwise there will only be value commitments.
        let _ = self.create_public_variable(Fr254::zero())?;

        let fee = Fr254::zero();
        let roots: [Fr254; 4] = (0..4)
            .map(|_| {
                let root = self.create_public_variable(Fr254::zero())?;
                self.witness(root)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert roots to fixed length array".to_string(),
                )
            })?;
        let commitments: [Fr254; 4] = new_commitments
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

        let nullifiers: [Fr254; 4] = (0..4)
            .map(|_| {
                let nullifier = self.create_public_variable(Fr254::zero())?;
                self.witness(nullifier)
            })
            .collect::<Result<Vec<Fr254>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError(
                    "Could not convert roots to fixed length array".to_string(),
                )
            })?;
        let compressed_secrets: [Fr254; 5] = sha_outputs
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

        Ok(PublicInputs::new()
            .fee(fee)
            .roots(&roots)
            .commitments(&commitments)
            .nullifiers(&nullifiers)
            .compressed_secrets(&compressed_secrets)
            .build())
    }
}

/// Function called to build a deposit circuit
pub fn deposit_circuit_builder(
    deposit_data: &[DepositData; 4],
    public_inputs: &mut PublicInputs,
) -> Result<PlonkCircuit<Fr254>, CircuitError> {
    let mut circuit = PlonkCircuit::<Fr254>::new_ultra_plonk(8);
    *public_inputs = circuit.build_deposit_circuit(deposit_data)?;

    Ok(circuit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nf_token_id::to_nf_token_id_from_fr254;
    use ark_bn254::Fr as Fr254;
    use ark_ff::BigInteger;
    use ark_std::{One, UniformRand, Zero};
    use jf_primitives::poseidon::{FieldHasher, Poseidon};
    use jf_utils::test_rng;
    use num_bigint::BigUint;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_deposit_circuit() -> Result<(), CircuitError> {
        let rng = &mut test_rng();
        for _ in 0..5 {
            let mut circuit: PlonkCircuit<Fr254> = PlonkCircuit::new_ultra_plonk(8);
            let token_id = Fr254::rand(rng);
            let erc_address = Fr254::rand(rng);
            let nf_token_id = to_nf_token_id_from_fr254(erc_address, token_id);

            let deposit_data: [DepositData; 4] = (0..4)
                .map(|i| {
                    if i.is_zero() {
                        DepositData::default()
                    } else {
                        let nf_slot_id = Fr254::rand(rng);
                        let value = Fr254::rand(rng);
                        let secret_hash = Fr254::rand(rng);
                        DepositData {
                            nf_token_id,
                            nf_slot_id,
                            value,
                            secret_hash,
                        }
                    }
                })
                .collect::<Vec<DepositData>>()
                .try_into()
                .unwrap();

            let public_input = circuit.build_deposit_circuit(&deposit_data).unwrap();
            let pi_vec = Vec::from(&public_input);
            circuit
                .check_circuit_satisfiability(pi_vec.as_slice())
                .unwrap();

            println!("Constraint count: {}", circuit.num_gates());
            let rust_sha_hashes = deposit_data.map(|data| {
                if !data.value.is_zero() && !data.nf_token_id.is_zero() {
                    let token_id_bytes = data.nf_token_id.into_bigint().to_bytes_be();
                    let slot_id_bytes = data.nf_slot_id.into_bigint().to_bytes_be();
                    let value_bytes = data.value.into_bigint().to_bytes_be();
                    let secret_hash_bytes = data.secret_hash.into_bigint().to_bytes_be();

                    let field_bytes = [
                        token_id_bytes,
                        slot_id_bytes,
                        value_bytes,
                        secret_hash_bytes,
                    ]
                    .concat();

                    let mut hasher = Sha256::new();
                    hasher.update(field_bytes);
                    let full_hash_bytes = hasher.finalize();
                    let exp_hash_value = BigUint::from_bytes_be(&full_hash_bytes) >> 4;
                    Fr254::from(exp_hash_value)
                } else {
                    Fr254::zero()
                }
            });

            for (rust_sha, circuit_sha) in rust_sha_hashes
                .iter()
                .zip(public_input.compressed_secrets[..4].iter())
            {
                assert_eq!(*rust_sha, *circuit_sha);
            }

            let poseidon = Poseidon::<Fr254>::new();
            for (dd, circ_comm) in deposit_data.iter().zip(public_input.commitments.iter()) {
                let expect_comm_hash = if !dd.value.is_zero() && !dd.nf_token_id.is_zero() {
                    poseidon
                        .hash(&[
                            dd.nf_token_id,
                            dd.nf_slot_id,
                            dd.value,
                            Fr254::zero(),
                            Fr254::one(),
                            dd.secret_hash,
                        ])
                        .unwrap()
                } else {
                    Fr254::zero()
                };
                assert_eq!(expect_comm_hash, *circ_comm);
            }
        }
        Ok(())
    }
}
