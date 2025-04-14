use ark_ff::PrimeField;
use jf_primitives::{poseidon::PoseidonParams, trees::imt::IMTCircuitInsertionInfo};
use lib::merkle_trees::trees::{IndexedTree, MutableTree};

use crate::ports::trees::NullifierTree;

#[async_trait::async_trait]
impl<F> NullifierTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
{
    type CircuitInfo = IMTCircuitInsertionInfo<F>;
    const TREE_NAME: &'static str = "Nullifiers";

    async fn insert_for_circuit(
        &mut self,
        leaves: &[F],
    ) -> Result<Self::CircuitInfo, <Self as MutableTree<F>>::Error> {
        <Self as IndexedTree<F>>::insert_nullifiers_for_circuit(
            self,
            leaves,
            <Self as NullifierTree<F>>::TREE_NAME,
        )
        .await
    }
    /// let's multiple sub trees be added in a single batch - it calls insert_subtree for each sub tree
    async fn batch_insert_with_circuit_info(
        &mut self,
        commitments: &[F],
    ) -> Result<Vec<Self::CircuitInfo>, <Self as MutableTree<F>>::Error> {
        <Self as IndexedTree<F>>::batch_insert_nullifiers_with_circuit_info(
            self,
            commitments,
            <Self as NullifierTree<F>>::TREE_NAME,
        )
        .await
    }
}
