use crate::serialization::{deserialize_fr_padded, serialize_fr_padded};
use ark_ff::PrimeField;
use jf_primitives::{
    poseidon::{PoseidonError, PoseidonParams},
    trees::{
        imt::{IMTCircuitInsertionInfo, LeafDBEntry},
        CircuitInsertionInfo, MembershipProof, TreeHasher,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

// module containing merkle tree traits. These traits are written on the assumption that one generally
// adds complete, fixed-size subtrees to a tree, rather than individual leaves. A leaf is a special case
// of a subtree with a depth of 0.

/// Metadata for a Merkle Tree.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TreeMetadata<F: PrimeField> {
    pub(crate) tree_height: u32,
    pub(crate) sub_tree_height: u32,
    /// the number of sub-trees in the tree
    pub sub_tree_count: u64, // we really want a u64 here but BSON doesn't handle those.
    pub(crate) _id: u32,
    #[serde(
        serialize_with = "serialize_fr_padded",
        deserialize_with = "deserialize_fr_padded"
    )]
    pub(crate) root: F,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IndexedLeaf<F: PrimeField> {
    #[serde(
        serialize_with = "serialize_fr_padded",
        deserialize_with = "deserialize_fr_padded"
    )]
    pub(crate) value: F,
    pub(crate) _id: u32, // todo, this should ideally be a u64 but BSON doesn't handle those.
    pub(crate) next_index: u32,
    #[serde(
        serialize_with = "serialize_fr_padded",
        deserialize_with = "deserialize_fr_padded"
    )]
    pub(crate) next_value: F,
}

impl<F: PrimeField> From<IndexedLeaf<F>> for LeafDBEntry<F> {
    fn from(leaf: IndexedLeaf<F>) -> Self {
        let IndexedLeaf {
            value,
            _id,
            next_index,
            next_value,
        } = leaf;

        LeafDBEntry {
            value,
            index: _id,
            next_index: F::from(next_index),
            next_value,
        }
    }
}

impl<F: PrimeField> Default for IndexedLeaf<F> {
    fn default() -> Self {
        Self {
            value: F::zero(),
            _id: 0,
            next_index: 0,
            next_value: F::zero(),
        }
    }
}

/// calling to_string() on F is problematic because it gets converted to "" rather than "0" for F::zero()
/// this fails when we want to convert in the opposite direction using .parse(). This trait is a workaround.
pub(crate) trait ToStringRep {
    fn to_string_rep(&self) -> String;
}

impl<F: PrimeField> ToStringRep for F {
    fn to_string_rep(&self) -> String {
        if self == &F::zero() {
            "0".to_string()
        } else {
            self.to_string()
        }
    }
}

/// errors for a merkle tree
#[derive(Debug)]
pub enum MerkleTreeError<E> {
    /// The tree is full
    TreeIsFull,
    IncorrectBatchSize,
    NoLeaves,
    DatabaseError(E),
    TreeNotFound,
    TreeAlreadyExists,
    LeafExists,
    SerializationError,
    InvalidProof,
    ItemNotFound,
    InvalidIndex,
    Error(String),
    HashingError(PoseidonError),
}

impl<E: Display + Debug> Error for MerkleTreeError<E> {}

impl<E: Display> Display for MerkleTreeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TreeIsFull => write!(f, "The tree is full"),
            Self::IncorrectBatchSize => write!(f, "Incorrect batch size"),
            Self::NoLeaves => write!(f, "No leaves"),
            Self::DatabaseError(e) => write!(f, "Database error {}", e),
            Self::TreeNotFound => write!(f, "Tree not found"),
            Self::TreeAlreadyExists => write!(f, "Tree already exists"),
            Self::LeafExists => write!(f, "Leaf already exists"),
            Self::SerializationError => write!(f, "Serialization error "),
            Self::InvalidProof => write!(f, "Invalid proof"),
            Self::ItemNotFound => write!(f, "Item not found"),
            Self::InvalidIndex => write!(f, "Invalid index"),
            Self::Error(e) => write!(f, "Error {}", e),
            Self::HashingError(e) => write!(f, "Hashing error {}", e),
        }
    }
}

impl From<PoseidonError> for MerkleTreeError<mongodb::error::Error> {
    fn from(e: PoseidonError) -> Self {
        MerkleTreeError::HashingError(e)
    }
}
///  A tree whose leaves are not mutable.
///
/// It relies on a Frontier to enable leaves to be appended
/// These trees cannot be used to compute Membership Proofs except at the point when a leaf is inserted
/// As that is usually insecure, provision of Membership Proofs are included in the trait. It's useful if all
/// you need to do is compute an updated root after a subtree has been appended.
#[async_trait::async_trait]
pub trait AppendOnlyTree<F>
where
    F: PrimeField + PoseidonParams,
{
    type Error;
    type TreeHasher: TreeHasher<F>; // type that can hash a pair of tree nodes together, coping with zero value nodes
    const DB: &'static str;

    /// creates a new append only tree with the given tree height and sub tree height
    async fn new_append_only_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), Self::Error>;

    /// appends one or more subtrees to the tree, returning the root of the new tree (the leaves of the sub trees
    /// should be provided in the order that they would be in the tree).
    async fn append_sub_trees(
        &self,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error>;
}

/// A tree that is mutable
///
/// These trees remember all of the nodes and so provide much greater functionality,
/// at the expense of having a large database of tree nodes. They can create Membership Proofs
/// and one can update the tree with new leaves as well as append leaves
#[async_trait::async_trait]
pub trait MutableTree<F>
where
    F: PrimeField + PoseidonParams,
{
    type Error;
    type TreeHasher: TreeHasher<F>; // type that can hash a pair of tree nodes together, coping with zero value nodes
    const MUT_DB_NAME: &'static str;

    /// creates a new mutable tree with the given tree height and sub tree height
    async fn new_mutable_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), Self::Error>;
    /// appends one or more subtrees to the tree, returning the root of the new tree and the updated leaf count (the leaves of the sub trees
    /// should be provided in the order that they would be in the tree).
    async fn append_sub_trees(
        &self,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<(F, u64), Self::Error>;
    /// Allows on e to insert a single leaf into the tree regardless of the specified subtree size.
    async fn insert_leaf(
        &self,
        leaf: F,
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error>;
    /// allows one to update a sub-tree
    async fn update_sub_tree(
        &self,
        sub_tree_index: u64,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error>;
    /// get a membership proof
    async fn get_membership_proof(
        &self,
        leaf: Option<&F>,
        leaf_index: Option<u64>,
        tree_id: &str,
    ) -> Result<MembershipProof<F>, Self::Error>;
    /// returns the node value at the given index
    async fn get_node(&self, index: u64, tree_id: &str) -> Result<F, Self::Error>;
    /// sets the node value at the given index
    async fn set_node(
        &self,
        index: u64,
        value: F,
        update_tree: bool,
        tree_id: &str,
    ) -> Result<(), Self::Error>;
    /// determines if a leaf is in the tree
    async fn is_leaf(&self, leaf: &F, tree_id: &str) -> Result<bool, Self::Error>;
    /// writes the temporary node cache to the database and clears the cache. This is normally done automatically.
    async fn flush_cache(&self, tree_id: &str) -> Result<(), Self::Error>;
    /// returns the current root of the tree
    async fn get_root(&self, tree_id: &str) -> Result<F, Self::Error>;
    /// Inserts leaves into the tree and returns information allowing us to verify in a circuit.
    async fn insert_for_circuit(
        &self,
        leaves: &[F],
        tree_id: &str,
    ) -> Result<CircuitInsertionInfo<F>, Self::Error>;
    /// let's multiple sub trees be added in a single batch - it calls insert_subtree for each sub tree
    async fn batch_insert_with_circuit_info(
        &self,
        commitments: &[F],
        tree_id: &str,
    ) -> Result<Vec<CircuitInsertionInfo<F>>, Self::Error>;
    /// Clears all data (nodes, state) for the specified tree.
    async fn reset_mutable_tree(&self, tree_id: &str) -> Result<(), Self::Error>;
}

/// a trait for an indexed merkle tree, which is a mutable tree that can be used to prove non-membership of a leaf
#[async_trait::async_trait]
pub trait IndexedTree<F>: MutableTree<F> + IndexedLeaves<F>
where
    F: PrimeField + PoseidonParams,
{
    /// creates a new indexed tree with the given tree height and sub tree height
    async fn new_indexed_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), <Self as MutableTree<F>>::Error>;
    /// get a non-membership proof
    async fn get_non_membership_proof(
        &self,
        leaf: &F,
        tree_id: &str,
    ) -> Result<MembershipProof<F>, <Self as MutableTree<F>>::Error>;
    /// verifies a non-membership proof
    fn verify_non_membership_proof(
        &self,
        proof: &MembershipProof<F>,
        root: &F,
    ) -> Result<(), <Self as MutableTree<F>>::Error>;
    /// inserts one or more subtrees to the tree, returning the root of the new tree (the leaves of the sub trees
    /// should be provided in the order that they would be in the tree). This function also updates the indexed leaves db
    async fn insert_leaves(
        &self,
        inner_leaf_values: &[F],
        tree_id: &str,
    ) -> Result<F, <Self as MutableTree<F>>::Error>;
    /// Inserts leaves into the tree and returns information allowing us to verify in a circuit.
    async fn insert_nullifiers_for_circuit(
        &self,
        leaves: &[F],
        tree_id: &str,
    ) -> Result<IMTCircuitInsertionInfo<F>, <Self as MutableTree<F>>::Error>;
    /// let's multiple sub trees be added in a single batch - it calls insert_subtree for each sub tree
    async fn batch_insert_nullifiers_with_circuit_info(
        &self,
        commitments: &[F],
        tree_id: &str,
    ) -> Result<Vec<IMTCircuitInsertionInfo<F>>, <Self as MutableTree<F>>::Error>;
}

/// A trait for implementing a database of the leaves of an indexed Merkle tree.
///
/// This is needed because the the leaf preimage is part of the working of the indexed Merkle tree and
/// the tree cannot be correctly updated without the preimage information
#[async_trait::async_trait]
pub trait IndexedLeaves<F: PrimeField> {
    type Error;
    const DB: &'static str;

    /// Creates a new instance of the database. We have to do this because we need to insert the zero leaf.
    async fn new_indexed_leaves_db(&self, tree_id: &str) -> Result<(), Self::Error>;
    /// Stores a leaf in the database. This functions works out the low leaf and optionally takes the index in the tree of the new leaf.
    async fn store_leaf(
        &self,
        leaf: F,
        index: Option<u32>,
        tree_id: &str,
    ) -> Result<Option<()>, Self::Error>;
    /// Searches the database for a leaf with the supplied fields. If it finds one, it returns it.
    async fn get_leaf(
        &self,
        leaf_value: Option<F>,
        next_value: Option<F>,
        tree_id: &str,
    ) -> Result<Option<IndexedLeaf<F>>, Self::Error>;
    /// Searches the database for the leaf that skips over the supplied value. That is finds the leaf such that
    /// `low_leaf.value` < `leaf_value` < `low_leaf.next_value`. If it finds one, it returns it.
    async fn get_low_leaf(
        &self,
        leaf_value: &F,
        tree_id: &str,
    ) -> Result<Option<IndexedLeaf<F>>, Self::Error>;
    /// Updates the leaf entry stored with value `leaf` with the new `next_value`.
    async fn update_leaf(
        &self,
        leaf: F,
        new_next_index: u32,
        new_next_value: F,
        tree_id: &str,
    ) -> Result<(), Self::Error>;
}

pub(crate) mod helper_functions {
    use ark_ff::PrimeField;
    use jf_primitives::trees::{Directions, TreeHasher};

    /// helper function to compute a complete tree (only use for small trees!)
    ///  /// Nodes are numbered thusly:
    ///                                0
    ///             /                                     \
    ///             1                                     2
    ///      /              \                  /                    \
    ///      3              4                  5                     6
    ///   /     \         /     \           /     \               /     \
    /// 7       8        9      10         11     12             13     14
    /// This tree has a height of 3 (4 rows).
    pub(crate) fn make_complete_tree<N>(
        height: u32,
        hasher: &impl TreeHasher<N>,
        leaves: &[N],
    ) -> Vec<N>
    where
        N: PrimeField,
    {
        let n_nodes = 2_usize.pow(height + 1) - 1;
        let n_leaves = 2_usize.pow(height);
        let first_leaf_index = n_nodes - n_leaves;
        let mut nodes = vec![N::zero(); n_nodes];
        let last_leaf_index = first_leaf_index + leaves.len();
        // copy the leaves into the leaf nodes
        nodes[first_leaf_index..last_leaf_index]
            .copy_from_slice(&leaves[..(last_leaf_index - first_leaf_index)]);
        for i in (0..n_nodes - 1).step_by(2) {
            let index = n_nodes - i - 1;
            // we're hashing from the right hand side so the starting node is guaranteed to be even
            nodes[index / 2 - 1] = hasher
                .tree_hash(&[nodes[index - 1], nodes[index]])
                .expect("Could not hash node values together");
        }
        nodes
    }

    /// converts a leaf index into a path up the Merkle tree from the leaf
    pub(crate) fn index_to_directions(index: usize, height: u32) -> Vec<Directions> {
        let mut path = Vec::<Directions>::new();
        for i in 0..height {
            let dir = index >> i & 1;
            if dir == 0 {
                path.push(Directions::HashWithThisNodeOnRight)
            } else {
                path.push(Directions::HashWithThisNodeOnLeft)
            }
        }
        path
    }

    /// Works out which index of the frontier vector should be updated after adding a given leaf
    /// see https://github.com/EYBlockchain/timber
    pub(crate) fn get_frontier_index(leaf_index: usize) -> usize {
        let mut index = 0;
        if leaf_index % 2 == 1 {
            let mut exp1: usize = 1;
            let mut pow1: usize = 2;
            let mut pow2: usize = pow1 << 1;
            while index == 0 {
                if (leaf_index + 1 - pow1) % pow2 == 0 {
                    index = exp1;
                } else {
                    pow1 = pow2;
                    pow2 <<= 1;
                    exp1 += 1;
                }
            }
        }
        index
    }
}
