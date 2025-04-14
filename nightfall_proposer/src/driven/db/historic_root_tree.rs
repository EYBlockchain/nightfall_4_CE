use ark_ff::PrimeField;
use jf_primitives::poseidon::PoseidonParams;

use crate::ports::trees::HistoricRootTree;

/// Implementation of a historic root tree.  This is a mutable tree because although we don't make use
/// of membership proofs, we do need to be able to tell if a particular leaf is in the tree or not. We can do that
/// by querying the node database, which only a mutable tree can do.

#[async_trait::async_trait]
impl<F> HistoricRootTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
{
    const TREE_NAME: &'static str = "historic_root_tree";
}
