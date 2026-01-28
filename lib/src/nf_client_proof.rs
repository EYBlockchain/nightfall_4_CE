use alloy::primitives::{Address, Bytes};
use ark_bn254::Fr as Fr254;
use ark_ec::{twisted_edwards::Affine as TEAffine, AffineRepr};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::SerializationError;
use ark_std::{One, Zero};
use jf_primitives::{
    circuit::tree::structs::MembershipProofVar,
    trees::{MembershipProof, PathElement},
};
use jf_relation::{
    errors::CircuitError,
    gadgets::ecc::{Point, PointVariable},
    BoolVar, Circuit, PlonkCircuit, Variable,
};
use nf_curves::ed_on_bn254::{BabyJubjub, Fr as BJJScalar};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Proof:
    Serialize + Debug + Clone + Sync + Send + 'static + for<'a> Deserialize<'a> + Unpin
{
    fn compress_proof(&self) -> Result<Bytes, SerializationError>;
    fn from_compressed(compressed: Bytes) -> Result<Self, SerializationError>
    where
        Self: Sized + Debug;
}
pub trait ProvingEngine<P>
where
    Self: Sized + Debug + Send + Sync + 'static,
    P: Proof,
{
    type Error: std::error::Error + Send + Sync;

    fn prove(
        private_inputs: &mut PrivateInputs,
        public_inputs: &mut PublicInputs,
    ) -> Result<P, Self::Error>;
    fn verify(proof: &P, public_inputs: &PublicInputs) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct PublicInputs {
    pub fee: Fr254,
    pub roots: [Fr254; 4],
    pub commitments: [Fr254; 4],
    pub nullifiers: [Fr254; 4],
    pub compressed_secrets: [Fr254; 5],
}

impl Default for PublicInputs {
    fn default() -> Self {
        Self {
            fee: Fr254::zero(),
            roots: [Fr254::zero(); 4],
            commitments: [Fr254::zero(); 4],
            nullifiers: [Fr254::zero(); 4],
            compressed_secrets: [Fr254::zero(); 5],
        }
    }
}

impl PublicInputs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fee(&mut self, fee: Fr254) -> &mut Self {
        self.fee = fee;
        self
    }

    pub fn commitments(&mut self, commitments: &[Fr254; 4]) -> &mut Self {
        self.commitments = *commitments;
        self
    }

    pub fn nullifiers(&mut self, nullifiers: &[Fr254; 4]) -> &mut Self {
        self.nullifiers = *nullifiers;
        self
    }

    pub fn compressed_secrets(&mut self, secrets: &[Fr254; 5]) -> &mut Self {
        self.compressed_secrets = *secrets;
        self
    }

    pub fn roots(&mut self, roots: &[Fr254; 4]) -> &mut Self {
        self.roots = *roots;
        self
    }

    /// Call this function after all other parameters have been set to build the finished struct.
    pub fn build(&mut self) -> Self {
        Self {
            fee: self.fee,
            roots: self.roots,
            commitments: self.commitments,
            nullifiers: self.nullifiers,
            compressed_secrets: self.compressed_secrets,
        }
    }

    /// Return an iterator over the values of the public inputs.
    pub fn iter(&self) -> impl Iterator<Item = Fr254> {
        Vec::<Fr254>::from(self).into_iter()
    }
}

impl From<&PublicInputs> for Vec<Fr254> {
    fn from(value: &PublicInputs) -> Self {
        // We include the initialisation bytes and length separators
        let mut init_bytes = "public_inputs".as_bytes().to_vec();
        init_bytes.extend_from_slice("version1".as_bytes());
        [
            &[Fr254::from_le_bytes_mod_order(init_bytes.as_slice())],
            &[Fr254::one()],
            &[value.fee],
            &[Fr254::from(value.roots.len() as u8)],
            value.roots.as_slice(),
            &[Fr254::from(value.commitments.len() as u8)],
            value.commitments.as_slice(),
            &[Fr254::from(value.nullifiers.len() as u8)],
            value.nullifiers.as_slice(),
            &[Fr254::from(value.compressed_secrets.len() as u8)],
            value.compressed_secrets.as_slice(),
        ]
        .concat()
    }
}

impl IntoIterator for PublicInputs {
    type Item = Fr254;
    type IntoIter = std::vec::IntoIter<Fr254>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<Fr254>::from(&self).into_iter()
    }
}
#[derive(Debug, Clone)]
pub struct PrivateInputs {
    // fee_token_id should be similar to nf_token_id_1 etc, so we make it private input even though it's a fixed value
    // which is a hash of nightfall contract address with 0 and get right shifted by 4
    pub fee_token_id: Fr254,
    // nf_address should be similar to recipient_public_key, as we make fee commitment have the fee which is paid to a nightfall address that can't be nullified.
    // so we make it private input
    pub nf_address: Address,
    pub value: Fr254,
    pub nf_token_id: Fr254,
    pub nf_slot_id: Fr254,
    pub nullifiers_values: [Fr254; 4],
    pub nullifiers_salts: [Fr254; 4],
    pub membership_proofs: [MembershipProof<Fr254>; 4],
    /// Values of any change commitments, first for the token second for the fee.
    pub commitments_values: [Fr254; 2],
    /// Only three as the first commitment salt is derived from the shared secret between sender and recipient
    pub commitments_salts: [Fr254; 3],
    /// The public keys of the owners of the old commitments that will be nullified.
    pub public_keys: [TEAffine<BabyJubjub>; 4],
    pub recipient_public_key: TEAffine<BabyJubjub>,
    pub root_key: Fr254,
    pub ephemeral_key: Fr254,
    pub withdraw_address: Fr254,
    pub secret_preimages: [[Fr254; 3]; 4],
}

impl Default for PrivateInputs {
    fn default() -> Self {
        let mproof = MembershipProof {
            node_value: Fr254::zero(),
            sibling_path: vec![
                PathElement {
                    direction: jf_primitives::trees::Directions::HashWithThisNodeOnLeft,
                    value: Fr254::zero()
                };
                32
            ],
            leaf_index: 0usize,
        };

        Self {
            fee_token_id: Fr254::zero(),
            nf_address: Address::ZERO,
            value: Fr254::zero(),
            nf_token_id: Fr254::zero(),
            nf_slot_id: Fr254::zero(),
            nullifiers_values: [Fr254::zero(); 4],
            nullifiers_salts: [Fr254::zero(); 4],
            membership_proofs: [mproof.clone(), mproof.clone(), mproof.clone(), mproof],
            commitments_values: [Fr254::zero(); 2],
            commitments_salts: [Fr254::zero(); 3],
            public_keys: [TEAffine::<BabyJubjub>::default(); 4],
            recipient_public_key: TEAffine::<BabyJubjub>::generator(),
            root_key: Fr254::zero(),
            ephemeral_key: Fr254::zero(),
            withdraw_address: Fr254::zero(),
            secret_preimages: [[Fr254::zero(); 3]; 4],
        }
    }
}

#[allow(dead_code)]
impl PrivateInputs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fee_token_id(&mut self, fee_token_id: Fr254) -> &mut Self {
        self.fee_token_id = fee_token_id;
        self
    }

    pub fn nf_address(&mut self, nf_address: Address) -> &mut Self {
        self.nf_address = nf_address;
        self
    }

    pub fn value(&mut self, value: Fr254) -> &mut Self {
        self.value = value;
        self
    }

    pub fn nf_token_id(&mut self, token_id: Fr254) -> &mut Self {
        self.nf_token_id = token_id;
        self
    }

    pub fn nf_slot_id(&mut self, nf_slot_id: Fr254) -> &mut Self {
        self.nf_slot_id = nf_slot_id;
        self
    }

    pub fn root_key(&mut self, root_key: Fr254) -> &mut Self {
        self.root_key = root_key;
        self
    }

    pub fn nullifiers_values(&mut self, nullifiers_values: &[Fr254; 4]) -> &mut Self {
        self.nullifiers_values = *nullifiers_values;
        self
    }

    pub fn nullifiers_salts(&mut self, nullifiers_salts: &[Fr254; 4]) -> &mut Self {
        self.nullifiers_salts = *nullifiers_salts;
        self
    }

    pub fn membership_proofs(
        &mut self,
        membership_proofs: &[MembershipProof<Fr254>; 4],
    ) -> &mut Self {
        self.membership_proofs.clone_from(membership_proofs);
        self
    }

    pub fn commitments_values(&mut self, commitments_values: &[Fr254; 2]) -> &mut Self {
        self.commitments_values = *commitments_values;
        self
    }

    pub fn commitments_salts(&mut self, commitments_salts: &[Fr254; 3]) -> &mut Self {
        self.commitments_salts = *commitments_salts;
        self
    }

    pub fn public_keys(&mut self, public_keys: &[TEAffine<BabyJubjub>; 4]) -> &mut Self {
        self.public_keys = *public_keys;
        self
    }

    pub fn recipient_public_key(
        &mut self,
        recipient_public_key: TEAffine<BabyJubjub>,
    ) -> &mut Self {
        self.recipient_public_key = recipient_public_key;
        self
    }

    pub fn ephemeral_key(&mut self, ephemeral_key: BJJScalar) -> &mut Self {
        let correct_field =
            Fr254::from_le_bytes_mod_order(&ephemeral_key.into_bigint().to_bytes_le());
        self.ephemeral_key = correct_field;
        self
    }

    pub fn withdraw_address(&mut self, withdraw_address: Fr254) -> &mut Self {
        self.withdraw_address = withdraw_address;
        self
    }

    pub fn secret_preimages(&mut self, secret_preimages: &[[Fr254; 3]; 4]) -> &mut Self {
        self.secret_preimages = *secret_preimages;
        self
    }

    pub fn build(&mut self) -> Self {
        Self {
            fee_token_id: self.fee_token_id,
            nf_address: self.nf_address,
            value: self.value,
            nf_token_id: self.nf_token_id,
            nf_slot_id: self.nf_slot_id,
            nullifiers_values: self.nullifiers_values,
            nullifiers_salts: self.nullifiers_salts,
            membership_proofs: self.membership_proofs.clone(),
            commitments_values: self.commitments_values,
            commitments_salts: self.commitments_salts,
            public_keys: self.public_keys,
            recipient_public_key: self.recipient_public_key,
            root_key: self.root_key,
            ephemeral_key: self.ephemeral_key,
            withdraw_address: self.withdraw_address,
            secret_preimages: self.secret_preimages,
        }
    }
}

/// Variable version of [`PrivateInputs`].
pub struct PrivateInputsVar {
    /// Token Id for the fee
    pub fee_token_id: Variable,
    /// Address of the Nightfall contract
    pub nf_address: Variable,
    /// Transaction value,
    pub value: Variable,
    /// nf_token_id of the transaction
    pub nf_token_id: Variable,
    /// Slot Id of transaction tokens,
    pub nf_slot_id: Variable,
    /// Nullifiers values
    pub nullifiers_values: [Variable; 4],
    /// Nullifiers salts
    pub nullifiers_salts: [Variable; 4],
    /// Merkle paths
    pub membership_proofs: [MembershipProofVar; 4],
    /// Commitments values, the values of any change
    pub commitments_values: [Variable; 2],
    /// Commitments salts
    pub commitments_salts: [Variable; 3],
    /// Public keys for the commitments being nullified
    pub public_keys: [PointVariable; 4],
    /// Recipient public key
    pub recipient_public_key: PointVariable,
    /// Root key
    pub root_key: Variable,
    /// Ephemeral key
    pub ephemeral_key: Variable,
    /// The address to withdraw to in a withdraw
    pub withdraw_address: Variable,
    /// A flag to indicate whether this is a withdraw or not
    pub withdraw_flag: BoolVar,
    /// The preimages to the secret hashes used for deposits
    pub secret_preimages: [[Variable; 3]; 4],
}

impl PrivateInputsVar {
    /// Creates an instance of [`PrivateInputsVar`] from an instance of [`PrivateInputs`].
    pub fn from_private_inputs(
        private_inputs: &PrivateInputs,
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<PrivateInputsVar, CircuitError> {
        let fee_token_id = circuit.create_variable(private_inputs.fee_token_id)?;
        let nf_address_field =
            Fr254::from(BigUint::from_bytes_be(private_inputs.nf_address.as_slice()));
        let nf_address = circuit.create_variable(nf_address_field)?;
        let value = circuit.create_variable(private_inputs.value)?;
        let nf_token_id = circuit.create_variable(private_inputs.nf_token_id)?;
        let nf_slot_id = circuit.create_variable(private_inputs.nf_slot_id)?;
        let nullifiers_values = private_inputs
            .nullifiers_values
            .iter()
            .map(|nv| circuit.create_variable(*nv))
            .collect::<Result<Vec<Variable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let nullifiers_salts = private_inputs
            .nullifiers_salts
            .iter()
            .map(|ns| circuit.create_variable(*ns))
            .collect::<Result<Vec<Variable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let membership_proofs = private_inputs
            .membership_proofs
            .iter()
            .map(|mp| MembershipProofVar::from_membership_proof(circuit, mp))
            .collect::<Result<Vec<MembershipProofVar>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let commitments_values = private_inputs
            .commitments_values
            .iter()
            .map(|cv| circuit.create_variable(*cv))
            .collect::<Result<Vec<Variable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let commitments_salts = private_inputs
            .commitments_salts
            .iter()
            .map(|cs| circuit.create_variable(*cs))
            .collect::<Result<Vec<Variable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let public_keys: [PointVariable; 4] = private_inputs
            .public_keys
            .iter()
            .map(|point| circuit.create_point_variable(&Point::<Fr254>::from(*point)))
            .collect::<Result<Vec<PointVariable>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;
        let recipient_public_key = circuit
            .create_point_variable(&Point::<Fr254>::from(private_inputs.recipient_public_key))?;
        // The recipient_public_key should also not be in the small subgroup in the case of a transfer
        // and constrained to be the neutral point in the case of a withdraw.
        let root_key = circuit.create_variable(private_inputs.root_key)?;
        let ephemeral_key = circuit.create_variable(private_inputs.ephemeral_key)?;
        let withdraw_address = circuit.create_variable(private_inputs.withdraw_address)?;
        let withdraw_flag = circuit.is_zero(withdraw_address)?;
        let withdraw_flag = circuit.logic_neg(withdraw_flag)?;
        let secret_preimages = private_inputs
            .secret_preimages
            .iter()
            .map(|secret| {
                secret
                    .iter()
                    .map(|&s| circuit.create_variable(s))
                    .collect::<Result<Vec<Variable>, CircuitError>>()?
                    .try_into()
                    .map_err(|_| {
                        CircuitError::ParameterError(
                            "Couldn't convert to fixed length array".to_string(),
                        )
                    })
            })
            .collect::<Result<Vec<[Variable; 3]>, CircuitError>>()?
            .try_into()
            .map_err(|_| {
                CircuitError::ParameterError("Couldn't convert to fixed length array".to_string())
            })?;

        // We enforce all the public keys to be either neutral or not in the small subgroup.
        // To ensure they're not in the small subgroup we check that [8]P != O.
        for point_var in public_keys.iter() {
            let neutral_check_var = circuit.is_neutral_point::<BabyJubjub>(point_var)?;
            // Compute [8]P by doubling 3 times.
            let check_point_var = std::iter::repeat_n((), 3).try_fold(*point_var, |acc, _| {
                circuit.ecc_add::<BabyJubjub>(&acc, &acc)
            })?;
            let subgroup_check_var = circuit.is_neutral_point::<BabyJubjub>(&check_point_var)?;
            // Finally, we enforce either the point is neutral or not in the small subgroup.
            circuit.mul_add_gate(
                &[
                    neutral_check_var.into(),
                    subgroup_check_var.into(),
                    circuit.one(),
                    subgroup_check_var.into(),
                    circuit.zero(),
                ],
                &[-Fr254::one(), Fr254::one()],
            )?;
        }
        // In the case of a transfer we enforce the recipient_public_key to not be in the small subgroup.
        // In the case of a withwraw we enforce the recipient_public_key to be neutral.
        let neutral_check_var = circuit.is_neutral_point::<BabyJubjub>(&recipient_public_key)?;
        // Compute [8]P by doubling 3 times.
        let check_point_var = std::iter::repeat_n((), 3)
            .try_fold(recipient_public_key, |acc, _| {
                circuit.ecc_add::<BabyJubjub>(&acc, &acc)
            })?;
        let subgroup_check_var = circuit.is_neutral_point::<BabyJubjub>(&check_point_var)?;
        // Finally, we enforce either the point is neutral or not in the small subgroup, depending on whether we have a transfer or withdraw.
        // We do this by enforcing that 2 * withdraw_flag == neutral_check_var + subgroup_check_var
        circuit.lin_comb_gate(
            &[Fr254::from(2u8), -Fr254::one(), -Fr254::one()],
            &Fr254::zero(),
            &[
                withdraw_flag.into(),
                neutral_check_var.into(),
                subgroup_check_var.into(),
            ],
            &circuit.zero(),
        )?;

        Ok(PrivateInputsVar {
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
            root_key,
            ephemeral_key,
            withdraw_address,
            withdraw_flag,
            secret_preimages,
        })
    }
}
