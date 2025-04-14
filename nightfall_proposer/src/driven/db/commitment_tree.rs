use ark_ff::PrimeField;
use jf_primitives::{poseidon::PoseidonParams, trees::CircuitInsertionInfo};
use lib::merkle_trees::trees::MutableTree;

use crate::ports::trees::CommitmentTree;

/// Implementation of a commitment tree.  This is bascially an append-only tree because we don't make use
/// of membership proofs, unlike the client crate, which needs to prove membership of a historic commitment tree in order
/// to spend commitments.

#[async_trait::async_trait]
impl<F> CommitmentTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
{
    type CircuitInfo = CircuitInsertionInfo<F>;
    const TREE_NAME: &'static str = "Commitments";

    async fn batch_insert_with_circuit_info(
        &mut self,
        commitments: &[F],
    ) -> Result<Vec<Self::CircuitInfo>, Self::Error> {
        <Self as MutableTree<F>>::batch_insert_with_circuit_info(
            self,
            commitments,
            <Self as CommitmentTree<F>>::TREE_NAME,
        )
        .await
    }

    async fn insert_for_circuit(
        &mut self,
        commitments: &[F],
    ) -> Result<Self::CircuitInfo, Self::Error> {
        <Self as MutableTree<F>>::insert_for_circuit(
            self,
            commitments,
            <Self as CommitmentTree<F>>::TREE_NAME,
        )
        .await
    }

    async fn get_root(&self) -> Result<F, Self::Error> {
        <Self as MutableTree<F>>::get_root(self, <Self as CommitmentTree<F>>::TREE_NAME).await
    }
}
