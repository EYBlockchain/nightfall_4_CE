use std::{error::Error, fmt::Display};

use crate::{
    domain::entities::{Block, ClientTransactionWithMetaData, DepositData},
    drivers::blockchain::block_assembly::BlockAssemblyError,
};
use ark_bn254::{Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::SerializationError;
use lib::error::ConversionError;
use lib::{nf_client_proof::{Proof, ProvingEngine, PublicInputs}, shared_entities::OnChainTransaction};

/// A trait for a proving engine that can recursively prove multiple transactions are valid.
#[allow(async_fn_in_trait)]
pub trait RecursiveProvingEngine<P: Proof> {
    /// This type is defined by the implementation based on how the proving engine proves state transitions.
    type PreppedInfo: std::fmt::Debug;
    /// The error type returned if unable to prove.
    type Error: Error
        + Display
        + Into<BlockAssemblyError>
        + From<SerializationError>
        + From<ConversionError>;
    /// The type of proof output by this recursive proving engine
    type RecursiveProof: Into<Vec<Fq254>>;

    /// This function takes in the list of client transactions to be proved and outputs the formed [`Block`]
    async fn prove_block(
        deposit_transactions: &[(P, PublicInputs)],
        client_transactions: &[ClientTransactionWithMetaData<P>],
    ) -> Result<Block, Self::Error> {
        let (info, [commitments_root, nullifiers_root, commitments_root_root]) =
            Self::prepare_state_transition(deposit_transactions, client_transactions).await?;
        let proof = Self::recursive_prove(info)?;
        let proof_vec: Vec<Fq254> = proof.into();
        let proof_bytes = proof_vec
            .into_iter()
            .flat_map(|x| {
                let mut bytes: Vec<u8> = x.into_bigint().to_bytes_le();
                bytes.resize(32, 0u8);
                bytes.reverse();
                bytes
            })
            .collect::<Vec<u8>>();

        Ok(Block {
            commitments_root,
            nullifiers_root,
            commitments_root_root,
            transactions: deposit_transactions
                .iter()
                .map(|(_, pi)| OnChainTransaction::from(pi))
                .chain(
                    client_transactions
                        .iter()
                        .map(|t| (&t.client_transaction).into()),
                )
                .collect::<_>(),
            rollup_proof: proof_bytes,
        })
    }
    /// The proving engine decides how it handles proving state transition so we make it define how it needs the
    /// commitment and nullifier information structured. This method returns the prepped info ready to be passed to the proving function as well as
    /// the new commitment, nullifier and historic root root in that order.
    async fn prepare_state_transition(
        deposit_transactions: &[(P, PublicInputs)],
        transactions: &[ClientTransactionWithMetaData<P>],
    ) -> Result<(Self::PreppedInfo, [Fr254; 3]), Self::Error>;
    /// This function performs the recursive proving based on the prepped information
    fn recursive_prove(info: Self::PreppedInfo) -> Result<Self::RecursiveProof, Self::Error>;
    fn verify_client_proof<E: ProvingEngine<P>>(
        &self,
        proof: &P,
        public_inputs: &PublicInputs,
    ) -> Result<bool, Self::Error> {
        E::verify(proof, public_inputs).map_err(|_| Self::Error::from(ConversionError::ParseFailed))
    }
    /// Function to create a deposit proof
    fn create_deposit_proof(
        deposit_data: &[DepositData; 4],
        public_inputs: &mut PublicInputs,
    ) -> Result<P, Self::Error>;
}
