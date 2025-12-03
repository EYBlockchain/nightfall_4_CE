use std::str::FromStr;

use ark_ff::PrimeField;
use jf_primitives::{
    poseidon::{Poseidon, PoseidonParams},
    trees::{Directions, TreeHasher},
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::trees::{
    helper_functions::{get_frontier_index, index_to_directions, make_complete_tree},
    AppendOnlyTree, TreeMetadata,
};
use crate::merkle_trees::trees::{
    helper_functions::{pow2_u64, pow2_usize},
    MerkleTreeError, ToStringRep,
};

#[derive(Serialize, Deserialize)]
struct Frontier {
    frontier: Vec<String>,
    _id: u32,
}
#[async_trait::async_trait]
impl<F> AppendOnlyTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
    <F as FromStr>::Err: std::fmt::Debug,
{
    type Error = MerkleTreeError<mongodb::error::Error>;
    type TreeHasher = Poseidon<F>;
    const DB: &'static str = "nightfall";

    async fn new_append_only_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), Self::Error> {
        // An early check to ensure that total tree height fits within
        // u64 index space so that we can safely compute powers of 2
        if tree_height >= u64::BITS || sub_tree_height >= u64::BITS {
            return Err(Self::Error::Error(
                "tree_height or sub_tree_height too large to represent safely".to_string(),
            ));
        }
        if tree_height
            .checked_add(sub_tree_height)
            .filter(|h| *h < u64::BITS)
            .is_none()
        {
            return Err(Self::Error::Error(
                "combined tree height is too large".to_string(),
            ));
        }
        // get a collection where we can write the tree metadata
        let metadata = TreeMetadata {
            tree_height,
            sub_tree_height,
            sub_tree_count: 0,
            root: F::zero(),
            _id: 0,
        };
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as AppendOnlyTree<F>>::DB)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        // and initialise the metadata
        metadata_collection
            .insert_one(metadata)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        // initialise a collection to store the frontier
        let frontier_collection_name = format!("{}_{}", tree_id, "frontier");
        let frontier_collection = self
            .database(<Self as AppendOnlyTree<F>>::DB)
            .collection::<Frontier>(&frontier_collection_name);
        // and initialise the frontier
        frontier_collection
            .insert_one(Frontier {
                frontier: vec![F::zero().to_string_rep(); tree_height as usize],
                _id: 0,
            })
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        Ok(())
    }

    async fn append_sub_trees(
        &self,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error> {
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as AppendOnlyTree<F>>::DB)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let mut metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let mut sub_tree_count = metadata.sub_tree_count;
        // get the frontier
        let frontier_collection_name = format!("{}_{}", tree_id, "frontier");
        let frontier_collection = self
            .database(<Self as AppendOnlyTree<F>>::DB)
            .collection::<Frontier>(&frontier_collection_name);
        let mut frontier = frontier_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::ItemNotFound)?
            .frontier
            .iter()
            .map(|s| {
                if s.is_empty() {
                    log::error!("Empty string found in frontier entry for tree '{tree_id}' - potential database corruption");
                    Err(Self::Error::DatabaseCorruption)
                } else {
                    F::from_str(s).map_err(|_| Self::Error::SerializationError)
                }
            })
            .collect::<Result<Vec<F>, Self::Error>>()?;
        // Basic data validation
        let sub_tree_capacity = pow2_usize(metadata.sub_tree_height).ok_or_else(|| {
            Self::Error::Error("sub_tree_capacity too large to compute capacity safely".to_string())
        })?;
        if leaves.len() % sub_tree_capacity != 0 {
            return Err(Self::Error::IncorrectBatchSize);
        }
        if leaves.is_empty() {
            return Ok(metadata.root);
        }
        let new_sub_trees = leaves.len() / sub_tree_capacity;
        // Prevent the tree from exceeding its maximum capacity (2^tree_height leaves).
        let capacity_subtrees = pow2_u64(metadata.tree_height).ok_or_else(|| {
            Self::Error::Error("tree_height to large to compute capacity safely".to_string())
        })?;
        let new_sub_trees_u64 = u64::try_from(new_sub_trees).map_err(|_| {
            Self::Error::Error("number of new sub trees does not fit in u64".to_string())
        })?;
        if sub_tree_count
            .checked_add(new_sub_trees_u64)
            .map(|total| total > capacity_subtrees)
            .unwrap_or(true)
        {
            return Err(Self::Error::TreeIsFull);
        }
        if frontier.len() != metadata.tree_height as usize {
            return Err(Self::Error::SerializationError);
        }
        // we'll 'add' each sub tree in turn but only write everything to the db at the end. This will be
        // more efficient than writing to the db for each sub tree
        let mut root = F::zero();
        let hasher = Self::TreeHasher::new();
        for leaf_batch in leaves.chunks(sub_tree_capacity) {
            // first, we'll compute the entire sub tree that we're adding because then we can add its root
            // to the main tree and hence update the main tree's Frontier and then derive sibling paths for all the
            // elements
            let sub_tree = make_complete_tree(metadata.sub_tree_height, &hasher, leaf_batch);
            let sub_tree_root = sub_tree[0];
            // now hook the sub-tree into the main tree and compute the updated frontier
            // first, compute the path up from the sub-tree's root. We can also get the upper part of the
            // sibling path at the same time
            let hashing_directions =
                index_to_directions(sub_tree_count as usize, metadata.tree_height); // the height of the tree DOES NOT include the sub-trees!
            let mut path = vec![sub_tree_root];
            let mut hash = sub_tree_root; // the main tree leaf value is the starting hash
                                          // hash to get the path up the tree
            for (level, direction) in hashing_directions.iter().enumerate() {
                hash = match direction {
                    Directions::HashWithThisNodeOnLeft => hasher
                        .tree_hash(&[frontier[level], hash])
                        .expect("Could not hash nodes together"),
                    Directions::HashWithThisNodeOnRight => hasher
                        .tree_hash(&[hash, F::zero()])
                        .expect("Could not hash nodes together"),
                };
                path.push(hash); // store the current hash in the path
            }
            // next update the frontier
            let index = get_frontier_index(sub_tree_count as usize);
            let p = path[index];
            frontier[index] = p;

            sub_tree_count += 1;

            // and root
            root = path.pop().expect("Path is empty. This should never happen");
        }
        let frontier = Frontier {
            frontier: frontier
                .iter()
                .map(|f| f.to_string_rep())
                .collect::<Vec<_>>(),
            _id: 0,
        };
        // just time to update the sub-tree count and the root in the metadata
        metadata.sub_tree_count = sub_tree_count;
        metadata.root = root;
        // then store it all in the database
        if update_tree {
            metadata_collection
                .replace_one(doc! {"_id": 0}, metadata)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
            frontier_collection
                .replace_one(doc! {"_id": 0}, frontier)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
        }
        Ok(root)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests_utils::*;
    use ark_bn254::Fr as Fr254;
    use ark_std::{rand::Rng, UniformRand};
    /// makes a vector of n leaves with random values.
    fn make_rnd_leaves<N: UniformRand>(n: usize, mut rng: impl Rng) -> Vec<N> {
        let mut leaves = vec![];
        for _i in 0..n {
            leaves.push(N::rand(&mut rng));
        }
        leaves
    }

    #[tokio::test]
    async fn test_append_only_tree() {
        let mut rng = ark_std::test_rng();
        // get a mongo container and connect to it
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        // make a new tree
        let tree_name = "test_tree";
        const TREE_HEIGHT: u32 = 3;
        const SUB_TREE_HEIGHT: u32 = 3;
        const SUB_TREE_LEAF_CAPACITY: usize = 2_usize.pow(SUB_TREE_HEIGHT);
        <mongodb::Client as AppendOnlyTree<Fr254>>::new_append_only_tree(
            &client,
            TREE_HEIGHT,
            SUB_TREE_HEIGHT,
            tree_name,
        )
        .await
        .unwrap();

        // generate some leaves for test purposes
        let leaves_1 = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY, &mut rng);
        let leaves_2 = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY, &mut rng);
        let leaves_4 = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY * 2_usize, &mut rng);
        let mut leaves = leaves_1.clone();
        leaves.append(&mut leaves_2.clone());
        let mut more_leaves = leaves.clone();
        more_leaves.append(&mut leaves_4.clone());

        // insert the leaves
        let root = client
            .append_sub_trees(&leaves_1, true, tree_name)
            .await
            .unwrap();

        // // now directly compute the entire tree
        let hasher = Poseidon::<Fr254>::new();
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &leaves_1);

        // and check the database tree against the test tree
        assert_eq!(root, test_tree[0]);

        // 'add' some more leaves but don't update the tree
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &leaves);
        let root = client
            .append_sub_trees(&leaves_2, false, tree_name)
            .await
            .unwrap();
        assert_eq!(root, test_tree[0]);

        // fully add another sub tree of leaves and check again. This will fail if the previous test added any state to the tree
        let root = client
            .append_sub_trees(&leaves_2, true, tree_name)
            .await
            .unwrap();
        assert_eq!(root, test_tree[0]);

        // test that we can add two sub trees at once and get the correct root
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &more_leaves);
        let root = client
            .append_sub_trees(&leaves_4, true, tree_name)
            .await
            .unwrap();
        assert_eq!(root, test_tree[0]);

        // test that we get an error if we try add too many sub trees, we have already 4 subtrees
        let leaves_5 = make_rnd_leaves(5_usize * SUB_TREE_LEAF_CAPACITY, &mut rng);
        let mut too_many_leaves = leaves.clone();
        too_many_leaves.append(&mut leaves_5.clone());

        let result = client.append_sub_trees(&leaves_5, true, tree_name).await;
        use crate::merkle_trees::trees::MerkleTreeError;
        assert!(matches!(result, Err(MerkleTreeError::TreeIsFull)));
    }

    #[tokio::test]
    async fn new_append_only_tree_rejects_too_large_heights() {
        use crate::merkle_trees::trees::MerkleTreeError;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let tree_name = "bad_height_tree";
        let bad_height = u64::BITS; // guaranteed to be rejected by constructor

        let result = <mongodb::Client as AppendOnlyTree<Fr254>>::new_append_only_tree(
            &client, bad_height, 0, tree_name,
        )
        .await;

        assert!(matches!(result, Err(MerkleTreeError::Error(_))));
    }

    #[tokio::test]
    async fn append_sub_trees_rejects_non_multiple_batch_size() {
        use crate::merkle_trees::trees::MerkleTreeError;
        let mut rng = ark_std::test_rng();
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let tree_name = "bad_batch_tree";
        const TREE_HEIGHT: u32 = 3;
        const SUB_TREE_HEIGHT: u32 = 3;
        const SUB_TREE_LEAF_CAPACITY: usize = 2_usize.pow(SUB_TREE_HEIGHT);

        <mongodb::Client as AppendOnlyTree<Fr254>>::new_append_only_tree(
            &client,
            TREE_HEIGHT,
            SUB_TREE_HEIGHT,
            tree_name,
        )
        .await
        .unwrap();

        // One more leaf than a full subtree capacity
        let bad_leaves: Vec<Fr254> = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY + 1, &mut rng);

        let result = client.append_sub_trees(&bad_leaves, true, tree_name).await;

        assert!(matches!(result, Err(MerkleTreeError::IncorrectBatchSize)));
    }
}
