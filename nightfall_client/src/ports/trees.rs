//! This module contains the interfaces for the three Merkle Trees a Proposer works with.

use ark_ff::PrimeField;
use jf_primitives::{poseidon::PoseidonParams, trees::MembershipProof};
use lib::merkle_trees::trees::MutableTree;

/// Trait defining the functionality of a commitment tree.
#[async_trait::async_trait]
pub trait CommitmentTree<F>: MutableTree<F>
where
    F: PrimeField + PoseidonParams + Unpin,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    /// The name of the commitment tree (Nightfall only has one so it can be a constant)
    const TREE_NAME: &'static str;
    type Error;
    /// get a new commitment tree
    async fn new_commitment_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
    ) -> Result<(), <Self as CommitmentTree<F>>::Error>;

    async fn append_sub_trees(
        &self,
        sub_tree_roots: &[F],
        update_tree: bool,
    ) -> Result<(F, u64), <Self as CommitmentTree<F>>::Error>;

    async fn get_membership_proof(
        &self,
        leaf: Option<&F>,
        leaf_index: Option<u64>,
    ) -> Result<MembershipProof<F>, <Self as CommitmentTree<F>>::Error>;

    async fn get_root(&self) -> Result<F, <Self as CommitmentTree<F>>::Error>;
}
