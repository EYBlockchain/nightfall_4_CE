use super::trees::{
    helper_functions::make_complete_tree, MerkleTreeError, MutableTree, TreeMetadata,
};
use crate::{
    merkle_trees::trees::ToStringRep,
    serialization::{deserialize_fr_padded, serialize_fr_padded, serialize_to_padded_hex},
};
use ark_ff::PrimeField;
use futures::{future::join_all, TryStreamExt};
use jf_primitives::{
    poseidon::{Poseidon, PoseidonParams},
    trees::{CircuitInsertionInfo, Directions, MembershipProof, PathElement, TreeHasher},
};
use log::debug;
use mongodb::bson::{doc, to_bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct Node<F: PrimeField> {
    #[serde(
        serialize_with = "serialize_fr_padded",
        deserialize_with = "deserialize_fr_padded"
    )]
    value: F,
    _id: u64,
}

#[async_trait::async_trait]
impl<F> MutableTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
{
    type TreeHasher = Poseidon<F>;
    type Error = MerkleTreeError<mongodb::error::Error>;
    const MUT_DB_NAME: &'static str = "nightfall";

    async fn new_mutable_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), Self::Error> {
        let metadata = TreeMetadata {
            tree_height,
            sub_tree_height,
            sub_tree_count: 0,
            _id: 0,
            root: F::zero(),
        };
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        metadata_collection
            .insert_one(metadata)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        Ok(())
    }

    async fn get_root(&self, tree_id: &str) -> Result<F, Self::Error> {
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        Ok(metadata.root)
    }

    async fn get_node(&self, index: u64, tree_id: &str) -> Result<F, Self::Error> {
        // first, check if the node is in the temporary cache. This is used for when we don't want to write to the db
        let bson_index = to_bson(&index).map_err(|e| MerkleTreeError::DatabaseError(e.into()))?;
        let cache_collection_name = format!("{}_{}", tree_id, "cache");
        let cache_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<Node<F>>(&cache_collection_name);
        let node = cache_collection
            .find_one(doc! {"_id": bson_index.clone()})
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        match node {
            // if the node is in the cache, return it
            Some(node) => Ok(node.value),
            // if the node is not in the cache, check the main database
            None => {
                let node_collection_name = format!("{}_{}", tree_id, "nodes");
                let node_collection = self
                    .database(<Self as MutableTree<F>>::MUT_DB_NAME)
                    .collection::<Node<F>>(&node_collection_name);
                let node = node_collection
                    .find_one(doc! {"_id": bson_index})
                    .await
                    .map_err(MerkleTreeError::DatabaseError)?;
                // nodes that aren't in the database are returned as zero
                match node {
                    Some(node) => Ok(node.value),
                    None => Ok(F::zero()),
                }
            }
        }
    }

    async fn flush_cache(&self, tree_id: &str) -> Result<(), Self::Error> {
        let cache_collection_name = format!("{}_{}", tree_id, "cache");
        let cache_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<Node<F>>(&cache_collection_name);
        let mut cache = cache_collection
            .find(doc! {})
            .await
            .map_err(MerkleTreeError::DatabaseError)?;

        let node_collection_name = format!("{}_{}", tree_id, "nodes");
        let node_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<Node<F>>(&node_collection_name);
        while let Some(node) = cache
            .try_next()
            .await
            .map_err(MerkleTreeError::DatabaseError)?
        {
            let bson_id =
                to_bson(&node._id).map_err(|e| MerkleTreeError::DatabaseError(e.into()))?;
            // a bulk write would be more efficient, check if now supported in Rust driver

            let value_padded_hex = serialize_to_padded_hex(&node.value)?;
            node_collection
                .update_one(
                    doc! {"_id": bson_id},
                    doc! {"$set": {"value": value_padded_hex}},
                )
                .upsert(true)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
        }
        // drop the cache once it's been written to the main database
        cache_collection
            .drop()
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        Ok(())
    }

    async fn is_leaf(&self, value: &F, tree_id: &str) -> Result<bool, Self::Error> {
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let node_collection_name = format!("{}_{}", tree_id, "nodes");
        let node_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<Node<F>>(&node_collection_name);
        let value_padded_hex = serialize_to_padded_hex(value)?;
        let node = node_collection
            .find_one(doc! {"value": value_padded_hex})
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        if node.is_none() {
            return Ok(false);
        }
        let index = node.unwrap()._id; // safe to unwrap as we've checked it's not None
        let height = metadata.tree_height + metadata.sub_tree_height;
        Ok(index >= 2_u64.pow(height) - 1 && index <= 2_u64.pow(height + 1) - 2)
    }

    async fn set_node(
        &self,
        index: u64,
        value: F,
        update_tree: bool,
        tree_id: &str,
    ) -> Result<(), Self::Error> {
        let update_value = serialize_to_padded_hex(&value)?;
        let bson_index = to_bson(&index).map_err(|e| MerkleTreeError::DatabaseError(e.into()))?;
        if !update_tree {
            let cache_collection_name = format!("{}_{}", tree_id, "cache");
            let cache_collection = self
                .database(<Self as MutableTree<F>>::MUT_DB_NAME)
                .collection::<Node<F>>(&cache_collection_name);
            cache_collection
                .update_one(
                    doc! {"_id": bson_index},
                    doc! {"$set": {"value": update_value}},
                )
                .upsert(true)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
            Ok(())
        } else {
            let node_collection_name = format!("{}_{}", tree_id, "nodes");
            let node_collection = self
                .database(<Self as MutableTree<F>>::MUT_DB_NAME)
                .collection::<Node<F>>(&node_collection_name);
            node_collection
                .update_one(
                    doc! {"_id": bson_index},
                    doc! {"$set": {"value": update_value}},
                )
                .upsert(true)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
            Ok(())
        }
    }

    async fn insert_leaf(
        &self,
        leaf: F,
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error> {
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let sub_tree_count = metadata.sub_tree_count;

        // we'll 'add' each sub tree in turn but only write everything to the db at the end. This will be much
        // more efficient than writing to the db for each sub tree
        let mut hash = leaf;

        // first, we'll compute the entire sub tree that we're adding because then we can add its root
        // to the main tree and hence update the main tree's Frontier and then derive sibling paths for all the
        // elements
        let hasher = Self::TreeHasher::new();

        // now hook the sub-tree into the main tree and compute the updated frontier
        // first change our index from a subtree (leaf) index to a node index
        let sub_tree_node_index = 2_u64.pow(metadata.tree_height + metadata.sub_tree_height) - 1
            + sub_tree_count * (1 << metadata.sub_tree_height); // this is the index where we're going to put the sub_tree
        let mut node_index = sub_tree_node_index;
        let mut updates = vec![self.set_node(node_index, leaf, update_tree, tree_id)]; // this will store all the hash values in the path from the leaf to the root

        for _i in 0..metadata.tree_height + metadata.sub_tree_height {
            hash = if node_index % 2 == 0 {
                hasher
                    .tree_hash(&[self.get_node(node_index - 1, tree_id).await?, hash])
                    .expect("Could not hash nodes together")
            } else {
                hasher
                    .tree_hash(&[hash, self.get_node(node_index + 1, tree_id).await?])
                    .expect("Could not hash nodes together")
            };
            node_index = (node_index - 1) / 2;
            updates.push(self.set_node(node_index, hash, update_tree, tree_id));
            // save the updated nodes
        }

        join_all(updates).await;

        // store the updated sub tree count
        if update_tree {
            let new_metadata = TreeMetadata {
                tree_height: metadata.tree_height,
                sub_tree_height: metadata.sub_tree_height,
                sub_tree_count: sub_tree_count + 1,
                _id: 0,
                root: hash,
            };
            metadata_collection
                .replace_one(doc! {"_id": 0}, new_metadata)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
            // save the cached nodes
            <Self as MutableTree<F>>::flush_cache(self, tree_id).await?;
        }
        // return the final root and the new sub tree count (from which leaf indices can be derived)
        Ok(hash)
    }

    async fn append_sub_trees(
        &self,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<(F, u64), Self::Error> {
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let mut sub_tree_count = metadata.sub_tree_count;
        let old_sub_tree_count = sub_tree_count;
        // Basic data validation
        let sub_tree_capacity = 2_usize.pow(metadata.sub_tree_height);
        if leaves.len() % sub_tree_capacity != 0 {
            return Err(Self::Error::IncorrectBatchSize);
        }
        if leaves.is_empty() {
            return Ok((metadata.root, sub_tree_count * sub_tree_capacity as u64));
        }
        if sub_tree_count > 2_usize.pow(metadata.tree_height) as u64 {
            return Err(Self::Error::TreeIsFull);
        }
        // we'll 'add' each sub tree in turn but only write everything to the db at the end. This will be much
        // more efficient than writing to the db for each sub tree
        let mut hash = F::zero();
        for leaf_batch in leaves.chunks(sub_tree_capacity) {
            // first, we'll compute the entire sub tree that we're adding because then we can add its root
            // to the main tree and hence update the main tree's Frontier and then derive sibling paths for all the
            // elements
            let hasher = Self::TreeHasher::new();
            let sub_tree = make_complete_tree(metadata.sub_tree_height, &hasher, leaf_batch);
            // just to make the code easer to follow
            let sub_tree_root = sub_tree[0];
            // now hook the sub-tree into the main tree and compute the updated frontier
            // first change our index from a subtree (leaf) index to a node index
            let sub_tree_node_index = 2_u64.pow(metadata.tree_height) - 1 + sub_tree_count; // this is the index where we're going to put the sub_tree
            let mut node_index = sub_tree_node_index;
            let mut updates = vec![self.set_node(node_index, sub_tree_root, update_tree, tree_id)]; // this will store all the hash values in the path from the leaf to the root
            hash = sub_tree_root; // the main tree leaf value is the starting hash
                                  // hash to get the path up the tree, store the updated nodes as we go
            for _i in 0..metadata.tree_height {
                hash = if node_index % 2 == 0 {
                    hasher
                        .tree_hash(&[self.get_node(node_index - 1, tree_id).await?, hash])
                        .expect("Could not hash nodes together")
                } else {
                    hasher
                        .tree_hash(&[hash, self.get_node(node_index + 1, tree_id).await?])
                        .expect("Could not hash nodes together")
                };
                node_index = (node_index - 1) / 2;
                updates.push(self.set_node(node_index, hash, update_tree, tree_id));
                // save the updated nodes
            }

            // for the nodes in the sub tree we count downwards, ignoring the root because we already counted that
            // i is the row in the sub tree, counting from the top
            let mut node_index = sub_tree_node_index; // this is the index of the root of the sub tree in the main tree
            let mut sub_tree_node_index = 0; // this is the index of the root node in the sub tree
            let mut span = 1;
            for _i in 0..metadata.sub_tree_height {
                node_index = node_index * 2 + 1;
                span *= 2;
                for j in node_index..(node_index + span) {
                    sub_tree_node_index += 1;
                    updates.push(self.set_node(
                        j,
                        sub_tree[sub_tree_node_index],
                        update_tree,
                        tree_id,
                    ));
                }
            }

            // run the set functions concurrently to update the nodes we changed
            join_all(updates).await;
            sub_tree_count += 1;
        }
        // store the updated sub tree count
        if update_tree {
            let new_metadata = TreeMetadata {
                tree_height: metadata.tree_height,
                sub_tree_height: metadata.sub_tree_height,
                sub_tree_count,
                _id: 0,
                root: hash,
            };
            metadata_collection
                .replace_one(doc! {"_id": 0}, new_metadata)
                .await
                .map_err(MerkleTreeError::DatabaseError)?;
            // save the cached nodes
            <Self as MutableTree<F>>::flush_cache(self, tree_id).await?;
        } else {
            sub_tree_count = old_sub_tree_count;
        }
        // return the final root and the new sub tree count (from which leaf indices can be derived)
        Ok((hash, sub_tree_count * sub_tree_capacity as u64))
    }

    async fn get_membership_proof(
        &self,
        leaf: Option<&F>,
        leaf_index: Option<u64>,
        tree_id: &str,
    ) -> Result<MembershipProof<F>, Self::Error> {
        // first read and parse the metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let height = metadata.tree_height + metadata.sub_tree_height;
        // next, find the index of the leaf node
        let node_collection_name = format!("{}_{}", tree_id, "nodes");
        let node_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<Node<F>>(&node_collection_name);
        // note, we will get problems if the leaf value is not unique. It would be too slow to check for this
        // but it's unlikely to happen in practice except for "0".
        let mut node_index = match (leaf, leaf_index) {
            (Some(leaf), None) => {
                if *leaf == F::zero() {
                    return Err(MerkleTreeError::Error(
                        "Cannot search for the index of a zero leaf".to_string(),
                    ));
                }
                let leaf_padded_hex = serialize_to_padded_hex(leaf)?;
                let node = node_collection
                    .find_one(doc! {"value": leaf_padded_hex})
                    .await
                    .map_err(|e| {
                        debug!("{e}");
                        MerkleTreeError::DatabaseError(e)
                    })?;
                if node.is_none() {
                    debug!("Could not find leaf node {leaf} in DB");
                    return Err(MerkleTreeError::ItemNotFound);
                }
                let node = node.unwrap(); // safe to unwrap as we've checked it's not None
                node._id
            }
            (_, Some(leaf_index)) => leaf_index + 2_u64.pow(height) - 1, // ignore the leaf if we have a leaf index
            _ => return Err(MerkleTreeError::ItemNotFound),
        };
        // check that the node is actually a leaf node
        if node_index < 2_u64.pow(height) - 1 || node_index > 2_u64.pow(height + 1) - 2 {
            debug!(
                "Node index {} is not a leaf node. Leaf value was {}",
                node_index,
                leaf.unwrap().to_string_rep()
            );
            return Err(MerkleTreeError::InvalidProof);
        }
        let leaf_node_index = node_index; // save the initial, leaf value for later
                                          // and directly extract the sibling path, storing it as PathElements rather than primitive values
        let mut sibling_path = vec![];
        for _i in 0..usize::try_from(height).unwrap() {
            if node_index % 2 == 0 {
                // sibling is to our left
                let path_element = PathElement {
                    direction: Directions::HashWithThisNodeOnLeft,
                    value: self.get_node(node_index - 1, tree_id).await?,
                };
                sibling_path.push(path_element);
            } else {
                // sibling is to our right
                let path_element = PathElement {
                    direction: Directions::HashWithThisNodeOnRight,
                    value: self.get_node(node_index + 1, tree_id).await?,
                };
                sibling_path.push(path_element);
            }
            node_index = (node_index - 1) / 2;
        }
        let leaf_index = (leaf_node_index - 2_u64.pow(height) + 1) as usize;
        // look up the leaf value if we need to.
        let leaf = match leaf {
            Some(leaf) => *leaf,
            None => self.get_node(leaf_node_index, tree_id).await?,
        };

        Ok(MembershipProof {
            sibling_path,
            leaf_index,
            node_value: leaf,
        })
    }

    async fn update_sub_tree(
        &self,
        sub_tree_index: u64,
        leaves: &[F],
        update_tree: bool,
        tree_id: &str,
    ) -> Result<F, Self::Error> {
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        // Basic data validation

        let tree_capacity = 2_u64.pow(metadata.tree_height + metadata.sub_tree_height);
        if leaves.len().next_power_of_two() != leaves.len() {
            return Err(Self::Error::IncorrectBatchSize);
        }
        if leaves.is_empty() {
            return Err(Self::Error::NoLeaves);
        }
        if sub_tree_index >= tree_capacity {
            return Err(Self::Error::InvalidIndex);
        }

        // first, we'll compute the entire sub tree that we're adding because then we can add its root
        // to the main tree and hence update the main tree's Frontier and then derive sibling paths for all the
        // elements
        let hasher = Self::TreeHasher::new();
        let sub_tree = make_complete_tree(leaves.len().ilog2(), &hasher, leaves);
        // just to make the code easer to follow
        let sub_tree_root = sub_tree[0];
        // now hook the sub-tree into the main tree and compute the updated frontier
        // first change our index from a subtree (leaf) index to a node index

        let height_to_use = if leaves.len().ilog2() == metadata.sub_tree_height {
            metadata.tree_height
        } else {
            metadata.tree_height + metadata.sub_tree_height - leaves.len().ilog2()
        };
        let sub_tree_node_index = 2_u64.pow(height_to_use) - 1 + sub_tree_index; // this is the index where we're going to put the sub_tree
        let mut node_index = sub_tree_node_index;
        let mut updates = vec![self.set_node(node_index, sub_tree_root, update_tree, tree_id)]; // this will store all the hash values in the path from the leaf to the root
        let mut hash = sub_tree_root; // the main tree leaf value is the starting hash
                                      // hash to get the path up the tree, store the updated nodes as we go
        for _i in 0..height_to_use {
            hash = if node_index % 2 == 0 {
                hasher
                    .tree_hash(&[self.get_node(node_index - 1, tree_id).await?, hash])
                    .expect("Could not hash nodes together")
            } else {
                hasher
                    .tree_hash(&[hash, self.get_node(node_index + 1, tree_id).await?])
                    .expect("Could not hash nodes together")
            };
            node_index = (node_index - 1) / 2;
            updates.push(self.set_node(node_index, hash, update_tree, tree_id));
            // save the updated nodes
        }
        // for the nodes in the sub tree we count downwards, ignoring the root because we already counted that
        // i is the row in the sub tree, counting from the top
        let mut node_index = sub_tree_node_index; // this is the index of the root of the sub tree in the main tree
        let mut sub_tree_node_index = 0; // this is the index of the root node in the sub tree
        let mut span = 1;
        if leaves.len() != 1 {
            for _i in 0..metadata.sub_tree_height {
                node_index = node_index * 2 + 1;
                span *= 2;
                for j in node_index..(node_index + span) {
                    sub_tree_node_index += 1;
                    updates.push(self.set_node(
                        j,
                        sub_tree[sub_tree_node_index],
                        update_tree,
                        tree_id,
                    ));
                }
            }
        };
        // run the set functions concurrently to update the nodes we changed
        join_all(updates).await;
        if update_tree {
            // save the cached nodes
            <Self as MutableTree<F>>::flush_cache(self, tree_id).await?
        }
        // save the updated root
        let new_metadata = TreeMetadata {
            tree_height: metadata.tree_height,
            sub_tree_height: metadata.sub_tree_height,
            sub_tree_count: metadata.sub_tree_count,
            _id: 0,
            root: hash,
        };
        metadata_collection
            .replace_one(doc! {"_id": 0}, new_metadata)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        // return the updated root
        Ok(hash)
    }

    async fn batch_insert_with_circuit_info(
        &self,
        commitments: &[F],
        tree_id: &str,
    ) -> Result<Vec<CircuitInsertionInfo<F>>, Self::Error> {
        // Basic data validation
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let sub_tree_leaf_capacity = 2_usize.pow(metadata.sub_tree_height);
        if commitments.len() % sub_tree_leaf_capacity != 0 {
            return Err(Self::Error::IncorrectBatchSize);
        }
        // call insert circuit for each sub_tree
        let mut circuit_infos = vec![];
        for chunk in commitments.chunks(sub_tree_leaf_capacity) {
            let circuit_info = self
                .insert_for_circuit(chunk, tree_id)
                .await
                .expect("Could not insert for circuit");
            circuit_infos.push(circuit_info);
        }
        Ok(circuit_infos)
    }

    async fn insert_for_circuit(
        &self,
        commitments: &[F],
        tree_id: &str,
    ) -> Result<CircuitInsertionInfo<F>, Self::Error> {
        // get the tree metadata
        let metadata_collection_name = format!("{}_{}", tree_id, "metadata");
        let metadata_collection = self
            .database(<Self as MutableTree<F>>::MUT_DB_NAME)
            .collection::<TreeMetadata<F>>(&metadata_collection_name);
        let old_metadata = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        // we should only be adding one sub tree at a time, if we want more, we should use the batch version
        if commitments.len() != 2_usize.pow(old_metadata.sub_tree_height) {
            return Err(Self::Error::IncorrectBatchSize);
        }
        let old_root = old_metadata.root;
        let sub_tree_index = old_metadata.sub_tree_count;
        // The proof that we have an empty sub tree position starts from the bottom of the sub tree. That's just how 'get_membership_proof'
        // is meant to work - it gives a proof for the whole tree height. We only care about the root of the sub tree being zero though,
        // so we can start from any leaf of the sub tree (they're all zero) then remove the sub-tree siblings. If the root is then zero,
        // the proof will verify.
        let sub_tree_height = old_metadata.sub_tree_height;
        let sub_tree_leaf_capacity = 2_usize.pow(sub_tree_height);
        let leaf_index = sub_tree_index * sub_tree_leaf_capacity as u64;
        let mut proof_of_emptiness = self
            .get_membership_proof(None, Some(leaf_index), tree_id)
            .await?;
        let proof = MembershipProof {
            node_value: F::zero(),
            sibling_path: proof_of_emptiness
                .sibling_path
                .split_off(sub_tree_height as usize),
            leaf_index: sub_tree_index as usize,
        };

        // now we can insert the leaves
        let (new_root, _) = self.append_sub_trees(commitments, true, tree_id).await?;
        // we can calculate the new leaf count to save reading the updated metadata from the DB
        Ok(CircuitInsertionInfo::<F> {
            old_root,
            new_root,
            proof,
            leaf_count: old_metadata.sub_tree_count as usize * sub_tree_leaf_capacity
                + commitments.len(),
            leaves: commitments.to_vec(),
        })
    }
    async fn reset_mutable_tree(&self, tree_id: &str) -> Result<(), Self::Error> {
        let db = self.database(<Self as MutableTree<F>>::MUT_DB_NAME);

        // Collection names
        let metadata_collection = format!("{tree_id}_metadata");
        let nodes_collection = format!("{tree_id}_nodes");
        let cache_collection = format!("{tree_id}_cache");
        use mongodb::bson::Document;
        // Drop metadata collection
        if let Err(e) = db.collection::<Document>(&metadata_collection).drop().await {
            if !e.to_string().contains("ns not found") {
                return Err(MerkleTreeError::DatabaseError(e));
            }
        }

        // Drop nodes collection
        if let Err(e) = db.collection::<Document>(&nodes_collection).drop().await {
            if !e.to_string().contains("ns not found") {
                return Err(MerkleTreeError::DatabaseError(e));
            }
        }

        // Drop cache collection
        if let Err(e) = db.collection::<Document>(&cache_collection).drop().await {
            if !e.to_string().contains("ns not found") {
                return Err(MerkleTreeError::DatabaseError(e));
            }
        }
        Ok(())
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
    async fn test_mutable_tree() {
        let mut rng = ark_std::test_rng();
        // get a mongo container and connect to it
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        // make a new tree
        let tree_name = "test_tree";
        const TREE_HEIGHT: u32 = 3;
        const SUB_TREE_HEIGHT: u32 = 3;
        const SUB_TREE_LEAF_CAPACITY: usize = 2_usize.pow(SUB_TREE_HEIGHT);
        <mongodb::Client as MutableTree<Fr254>>::new_mutable_tree(
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
        let leaves_3 = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY, &mut rng);
        let leaves_4 = make_rnd_leaves(2_usize * SUB_TREE_LEAF_CAPACITY, &mut rng);
        let mut leaves = leaves_1.clone();
        leaves.append(&mut leaves_2.clone());
        let mut updated_leaves = leaves_3.clone();
        updated_leaves.append(&mut leaves_2.clone());
        let mut more_leaves = leaves.clone();
        more_leaves.append(&mut leaves_4.clone());

        // insert the leaves
        let (root, sub_tree_count) = client
            .append_sub_trees(&leaves_1, true, tree_name)
            .await
            .unwrap();
        // // now directly compute the entire tree
        let hasher = Poseidon::<Fr254>::new();
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &leaves_1);
        // and check the database tree against the test tree
        assert_eq!(sub_tree_count, leaves_1.len() as u64);
        assert_eq!(root, test_tree[0]);

        // 'add' another subtree but this time, don't update the tree database, just compute what new root would be
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &leaves);
        let (root, sub_tree_count) = client
            .append_sub_trees(&leaves_2, false, tree_name)
            .await
            .unwrap();
        assert_eq!(sub_tree_count, leaves_1.len() as u64);
        assert_eq!(root, test_tree[0]);

        // fully add another sub tree of leaves and check again. This will only succeed if the previous test did not update the tree
        let (root, sub_tree_count) = client
            .append_sub_trees(&leaves_2, true, tree_name)
            .await
            .unwrap();
        assert_eq!(sub_tree_count, leaves.len() as u64);
        assert_eq!(root, test_tree[0]);

        // see if it finds a leaf
        let is_leaf = client.is_leaf(&leaves_2[1], tree_name).await.unwrap();
        assert!(is_leaf);
        let is_leaf = client.is_leaf(&leaves_3[1], tree_name).await.unwrap();
        assert!(!is_leaf);

        // update the first sub tree and test it works by computing the tree from scratch
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &updated_leaves);
        let root = client
            .update_sub_tree(0, &leaves_3, true, tree_name)
            .await
            .unwrap();
        assert_eq!(root, test_tree[0]);

        // update the first subtree back to leaves_2 but this time do it leaf by leaf
        for (i, leaf) in leaves_1.iter().enumerate() {
            let root = client
                .update_sub_tree(i as u64, &[*leaf], true, tree_name)
                .await
                .unwrap();
            updated_leaves[i] = *leaf;
            let test_tree =
                make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &updated_leaves);
            assert_eq!(root, test_tree[0]);
        }

        // test that we can append two sub trees at once and get the correct root
        let test_tree = make_complete_tree(SUB_TREE_HEIGHT + TREE_HEIGHT, &hasher, &more_leaves);
        let (root, _sub_tree_count) = client
            .append_sub_trees(&leaves_4, true, tree_name)
            .await
            .unwrap();
        assert_eq!(root, test_tree[0]);

        // test that we can get a membership proof
        let proof = client
            .get_membership_proof(Some(&leaves_2[2]), None, tree_name)
            .await
            .unwrap();
        let root = client.get_root(tree_name).await.unwrap();
        let hasher = Poseidon::<Fr254>::new();
        assert!(proof.verify(&root, &hasher).is_ok());
    }
}
