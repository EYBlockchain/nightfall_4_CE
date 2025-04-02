use ark_ff::PrimeField;
use jf_primitives::{poseidon::PoseidonParams, trees::MembershipProof};
use lib::merkle_trees::trees::{MerkleTreeError, MutableTree};

use crate::ports::trees::CommitmentTree;

/// implement a commitment tree for storing commitments. Note that this is really just a
/// special case of a MutableTree
#[async_trait::async_trait]
impl<F> CommitmentTree<F> for mongodb::Client
where
    F: PrimeField + std::marker::Unpin + PoseidonParams,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    const TREE_NAME: &'static str = "commitment_tree";
    type Error = MerkleTreeError<mongodb::error::Error>;

    async fn new_commitment_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
    ) -> Result<(), <Self as CommitmentTree<F>>::Error> {
        <Self as MutableTree<F>>::new_mutable_tree(
            self,
            tree_height,
            sub_tree_height,
            <Self as CommitmentTree<F>>::TREE_NAME,
        )
        .await
    }

    async fn append_sub_trees(
        &self,
        sub_tree_roots: &[F],
        update_tree: bool,
    ) -> Result<(F, u64), <Self as CommitmentTree<F>>::Error> {
        <Self as MutableTree<F>>::append_sub_trees(
            self,
            sub_tree_roots,
            update_tree,
            <Self as CommitmentTree<F>>::TREE_NAME,
        )
        .await
    }

    async fn get_membership_proof(
        &self,
        leaf: Option<&F>,
        leaf_index: Option<u64>,
    ) -> Result<MembershipProof<F>, <Self as CommitmentTree<F>>::Error> {
        <Self as MutableTree<F>>::get_membership_proof(
            self,
            leaf,
            leaf_index,
            <Self as CommitmentTree<F>>::TREE_NAME,
        )
        .await
    }

    async fn get_root(&self) -> Result<F, <Self as CommitmentTree<F>>::Error> {
        <Self as MutableTree<F>>::get_root(self, <Self as CommitmentTree<F>>::TREE_NAME).await
    }
}
