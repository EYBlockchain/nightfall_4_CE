use super::trees::{
    IndexedLeaf, IndexedLeaves, IndexedTree, MerkleTreeError, MutableTree, TreeMetadata,
};
use crate::{merkle_trees::trees::ToStringRep, serialization::serialize_to_padded_hex};
use ark_ff::PrimeField;
use futures::{future::join_all, TryStreamExt};
use jf_primitives::{
    poseidon::{FieldHasher, PoseidonError, PoseidonParams},
    trees::{
        imt::{IMTCircuitInsertionInfo, LeafDBEntry},
        Directions, MembershipProof, PathElement,
    },
};
use log::{debug, error};
use mongodb::bson::doc;

use std::convert::TryFrom; // already in prelude, but explicit is fine

// a small helper for u64 -> i64 with a bound check
fn u64_to_i64_checked(x: u64) -> Result<i64, MerkleTreeError<mongodb::error::Error>> {
    if x > i64::MAX as u64 {
        return Err(MerkleTreeError::Error(format!(
            "Index {} exceeds i64::MAX ({}) for BSON storage",
            x,
            i64::MAX
        )));
    }
    Ok(x as i64)
}

#[async_trait::async_trait]
impl<F> IndexedTree<F> for mongodb::Client
where
    F: PrimeField + PoseidonParams,
{
    async fn new_indexed_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
        tree_id: &str,
    ) -> Result<(), <Self as MutableTree<F>>::Error> {
        <Self as MutableTree<F>>::new_mutable_tree(self, tree_height, sub_tree_height, tree_id)
            .await?;
        // and an associated indexed leaves db
        <Self as IndexedLeaves<F>>::new_indexed_leaves_db(self, tree_id).await?;
        // not sure why we don't just use the default value. I suppose pulling the default value from the
        // lead db catches a write failure
        let entry = <Self as IndexedLeaves<F>>::get_leaf(self, Some(F::zero()), None, tree_id)
            .await?
            .ok_or(MerkleTreeError::NoLeaves)?;
        // Into which we add a zero leaf
        let hasher = <Self as MutableTree<F>>::TreeHasher::new();
        let entry_value = entry.value;
        let entry_next_value = entry.next_value;
        let entry_next_index = F::from(entry.next_index);
        let leaf_value = hasher
            .hash(&[entry_value, entry_next_index, entry_next_value])
            .expect("hashing failed");
        <Self as MutableTree<F>>::insert_leaf(self, leaf_value, true, tree_id).await?;
        Ok(())
    }

    async fn get_non_membership_proof(
        &self,
        leaf: &F,
        tree_id: &str,
    ) -> Result<MembershipProof<F>, <Self as MutableTree<F>>::Error> {
        // asking for a non membership proof for a leaf that is zero should be an error
        if *leaf == F::zero() {
            return Err(MerkleTreeError::LeafExists);
        }

        if <Self as IndexedLeaves<F>>::get_leaf(self, Some(*leaf), None, tree_id)
            .await?
            .is_some()
        {
            return Err(MerkleTreeError::LeafExists);
        }
        // Get the tree metadata
        let collection_name = format!("{}_{}", tree_id, "metadata");
        let db = self.database(<Self as MutableTree<F>>::MUT_DB_NAME);
        let collection = db.collection::<TreeMetadata<F>>(&collection_name);
        let metadata = collection
            .find_one(doc! {})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::ItemNotFound)?;
        let height = metadata.tree_height + metadata.sub_tree_height;

        let low_nullifier = <Self as IndexedLeaves<F>>::get_low_leaf(self, leaf, tree_id)
            .await?
            .ok_or(MerkleTreeError::Error("Could not get low leaf".to_string()))?;
        let hasher = <Self as MutableTree<F>>::TreeHasher::new();
        let ln_index = low_nullifier._id;
        let mut node_index = 2u64.pow(height) - 1 + ln_index;
        let low_nullifier_value: F = low_nullifier.value;
        let low_nullifier_next_index = F::from(low_nullifier.next_index);
        let low_nullifier_next_value: F = low_nullifier.next_value;
        let leaf_value = hasher.hash(&[
            low_nullifier_value,
            low_nullifier_next_index,
            low_nullifier_next_value,
        ])?;
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

        Ok(MembershipProof {
            node_value: leaf_value,
            sibling_path,
            leaf_index: ln_index as usize,
        })
    }

    fn verify_non_membership_proof(
        &self,
        non_membership_proof: &MembershipProof<F>,
        root: &F,
    ) -> Result<(), <Self as MutableTree<F>>::Error> {
        let hasher = <Self as MutableTree<F>>::TreeHasher::new();
        non_membership_proof
            .verify(root, &hasher)
            .map_err(|_| MerkleTreeError::InvalidProof)
    }

    /// Inserts multiple leaves into the tree.
    async fn insert_leaves(
        &self,
        inner_leaf_values: &[F],
        tree_id: &str,
    ) -> Result<F, <Self as MutableTree<F>>::Error> {
        // if we're given an empty list of leaves then we should return the current root
        if inner_leaf_values.is_empty() {
            return <Self as MutableTree<F>>::get_root(self, tree_id).await;
        }
        // check that the leaves are not already in the tree
        //skip the zero leaves
        for leaf in inner_leaf_values {
            if <Self as IndexedLeaves<F>>::get_leaf(self, Some(*leaf), None, tree_id)
                .await?
                .is_some()
                && (!leaf.is_zero())
            {
                error!("Leaf already exists {leaf:?}");
                return Err(MerkleTreeError::LeafExists);
            }
        }
        let hasher = <Self as MutableTree<F>>::TreeHasher::new();
        // find the low nullifiers for the leaves but do it asynchronously
        let low_nullifiers_getters = inner_leaf_values
            .iter()
            .map(|leaf| <Self as IndexedLeaves<F>>::get_low_leaf(self, leaf, tree_id))
            .collect::<Vec<_>>();
        let mut low_nullifiers = join_all(low_nullifiers_getters)
            .await
            .into_iter()
            .collect::<Result<Vec<Option<_>>, _>>()?
            .iter()
            .filter_map(|opt| opt.map(|indexed_leaf| indexed_leaf.value))
            .collect::<Vec<F>>();
        low_nullifiers.sort();
        low_nullifiers.dedup();

        let collection_name = format!("{}_{}", tree_id, "metadata");
        let db = self.database(<Self as MutableTree<F>>::MUT_DB_NAME);
        let collection = db.collection::<TreeMetadata<F>>(&collection_name);
        let metadata = collection
            .find_one(doc! {})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::ItemNotFound)?;
        let mut insert_index = metadata.sub_tree_count as u32 * (1u32 << metadata.sub_tree_height);
        // We cannot store the leaves concurrently because the index gets updated each time we store one and we'll have
        // a data race. We could probably do something clever with a mutex but it's easier to just store them sequentially.
        for inner_leaf in inner_leaf_values.iter() {
            if !inner_leaf.is_zero() {
                let res = <Self as IndexedLeaves<F>>::store_leaf(
                    self,
                    *inner_leaf,
                    Some(insert_index.into()),
                    tree_id,
                )
                .await?;
                if res.is_none() {
                    return Err(MerkleTreeError::Error(format!(
                        "Failed to store leaf {}",
                        inner_leaf.to_string_rep()
                    )));
                }
            }
            insert_index += 1;
        }

        // add the leaves to the tree
        let indexed_leaves_getters = inner_leaf_values
            .iter()
            .map(|leaf| <Self as IndexedLeaves<F>>::get_leaf(self, Some(*leaf), None, tree_id))
            .collect::<Vec<_>>();
        let indexed_leaves = join_all(indexed_leaves_getters)
            .await
            .into_iter()
            .collect::<Result<Vec<Option<IndexedLeaf<F>>>, <Self as MutableTree<F>>::Error>>()?
            .into_iter()
            .enumerate()
            .map(|(index, opt)| {
                opt.ok_or(MerkleTreeError::Error(format!(
                    "failed to get IndexedLeaf struct with value {}",
                    inner_leaf_values[index]
                )))
            })
            .collect::<Result<Vec<IndexedLeaf<F>>, _>>()?;
        let leaf_values = indexed_leaves
            .into_iter()
            .map(|indexed_leaf| {
                if !indexed_leaf.value.is_zero() {
                    let leaf_value: F = indexed_leaf.value;
                    let leaf_next_index = F::from(indexed_leaf.next_index);
                    let leaf_next_value: F = indexed_leaf.next_value;
                    hasher
                        .hash(&[leaf_value, leaf_next_index, leaf_next_value])
                        .map_err(|_| MerkleTreeError::Error("Hashing failed".to_string()))
                } else {
                    Ok(F::zero())
                }
            })
            .collect::<Result<Vec<F>, _>>()?;

        <Self as MutableTree<F>>::append_sub_trees(self, &leaf_values, true, tree_id).await?;

        let update_info_getters = low_nullifiers
            .iter()
            .map(|value| <Self as IndexedLeaves<F>>::get_leaf(self, Some(*value), None, tree_id))
            .collect::<Vec<_>>();
        let update_info = join_all(update_info_getters)
            .await
            .into_iter()
            .collect::<Result<Option<Vec<IndexedLeaf<F>>>, _>>()?
            .ok_or(MerkleTreeError::ItemNotFound)?
            .into_iter()
            .map(|leaf| {
                let leaf_value: F = leaf.value;
                let leaf_next_index = F::from(leaf.next_index);
                let leaf_next_value: F = leaf.next_value;
                Ok((
                    hasher.hash(&[leaf_value, leaf_next_index, leaf_next_value])?,
                    leaf._id as usize,
                ))
            })
            .collect::<Result<Vec<(F, usize)>, MerkleTreeError<mongodb::error::Error>>>()?;

        let mut root = F::zero();
        for info in update_info.into_iter() {
            let leaf_value = info.0;
            let ln_index = info.1 as u64;
            root = <Self as MutableTree<F>>::update_sub_tree(
                self,
                ln_index,
                &[leaf_value],
                true,
                tree_id,
            )
            .await?;
        }
        Ok(root)
    }

    async fn insert_nullifiers_for_circuit(
        &self,
        leaves: &[F],
        tree_id: &str,
    ) -> Result<IMTCircuitInsertionInfo<F>, <Self as MutableTree<F>>::Error> {
        // Get the tree metadata
        let collection_name = format!("{}_{}", tree_id, "metadata");
        let db = self.database(<Self as MutableTree<F>>::MUT_DB_NAME);
        let collection = db.collection::<TreeMetadata<F>>(&collection_name);
        let metadata = collection
            .find_one(doc! {})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::ItemNotFound)?;
        if 1 << metadata.sub_tree_height != leaves.len() {
            return Err(MerkleTreeError::IncorrectBatchSize);
        }

        let old_root = metadata.root;

        // This is the index of the first leaf of the subtree in the main tree, counted from the left, starting at zero
        let mut initial_index = metadata.sub_tree_count * (1u64 << metadata.sub_tree_height);

        let first_index = initial_index;

        let mut pending_inserts = vec![];
        let mut low_nullifiers = vec![];

        let hasher = Self::TreeHasher::new();
        for &inner_value in leaves.iter() {
            // First we get the low nullifier for the leaf we are inserting if the value is non-zero and does not already exist
            if <Self as IndexedLeaves<F>>::get_leaf(self, Some(inner_value), None, tree_id)
                .await?
                .is_some()
                && (!inner_value.is_zero())
            {
                return Err(MerkleTreeError::LeafExists);
            }

            if !inner_value.is_zero() {
                let low_nullifier =
                    <Self as IndexedLeaves<F>>::get_low_leaf(self, &inner_value, tree_id)
                        .await?
                        .ok_or(MerkleTreeError::Error(format!(
                            "Could not get low nullifier for inner value: {inner_value}"
                        )))?;

                // Now we check if the low nullifier is in the tree already, if it is not then it is one of the pending inserts
                let proof = if low_nullifier._id < first_index {
                    let proof = self.get_non_membership_proof(&inner_value, tree_id).await?;
                    <Self as IndexedLeaves<F>>::store_leaf(
                        self,
                        inner_value,
                        Some(initial_index),
                        tree_id,
                    )
                    .await?
                    .ok_or(MerkleTreeError::Error(
                        "Could not store nullifier".to_string(),
                    ))?;

                    initial_index += 1;

                    let updated_nullifier = <Self as IndexedLeaves<F>>::get_leaf(
                        self,
                        Some(low_nullifier.value),
                        None,
                        tree_id,
                    )
                    .await?
                    .ok_or(MerkleTreeError::Error(
                        "Could not get low nullifier".to_string(),
                    ))?;

                    let updated_leaf = hasher.hash(&[
                        updated_nullifier.value,
                        F::from(updated_nullifier.next_index),
                        updated_nullifier.next_value,
                    ])?;
                    let ln_index = updated_nullifier._id;

                    let _ = <Self as MutableTree<F>>::update_sub_tree(
                        self,
                        ln_index,
                        &[updated_leaf],
                        true,
                        tree_id,
                    )
                    .await?;

                    proof
                } else {
                    // If we couldn't get the non-membership proof its because its a pending insert so we return a proof where everything is zero.

                    <Self as IndexedLeaves<F>>::store_leaf(
                        self,
                        inner_value,
                        Some(initial_index),
                        tree_id,
                    )
                    .await?
                    .ok_or(MerkleTreeError::Error("Could not store leaf".to_string()))?;
                    initial_index += 1;
                    let node_value = hasher.hash(&[
                        low_nullifier.value,
                        F::from(low_nullifier.next_index),
                        low_nullifier.next_value,
                    ])?;
                    MembershipProof {
                        node_value,
                        sibling_path: vec![
                            PathElement {
                                value: F::zero(),
                                direction: Directions::HashWithThisNodeOnLeft,
                            };
                            metadata.tree_height as usize
                                + metadata.sub_tree_height as usize
                        ],
                        leaf_index: initial_index as usize - 1,
                    }
                };

                low_nullifiers.push((LeafDBEntry::<F>::from(low_nullifier), proof));
            } else {
                low_nullifiers.push((
                    LeafDBEntry::<F>::from(IndexedLeaf::<F>::default()),
                    MembershipProof {
                        node_value: F::zero(),
                        sibling_path: vec![
                            PathElement {
                                value: F::zero(),
                                direction: Directions::HashWithThisNodeOnLeft,
                            };
                            metadata.tree_height as usize
                                + metadata.sub_tree_height as usize
                        ],
                        leaf_index: 0,
                    },
                ));
                initial_index += 1;
            }
        }

        for inner_value in leaves.iter() {
            if !inner_value.is_zero() {
                let pending_insert =
                    <Self as IndexedLeaves<F>>::get_leaf(self, Some(*inner_value), None, tree_id)
                        .await?
                        .ok_or(MerkleTreeError::Error(
                            "Could not retrieve nullifier".to_string(),
                        ))?;

                pending_inserts.push(LeafDBEntry::<F>::from(pending_insert));
            } else {
                pending_inserts.push(LeafDBEntry::<F>::default());
            }
        }
        // Build the subtree to insert.
        let new_leaf_values = pending_inserts
            .iter()
            .map(|entry| {
                if !entry.value.is_zero() {
                    hasher.hash(&[entry.value, entry.next_index, entry.next_value])
                } else {
                    Ok(F::zero())
                }
            })
            .collect::<Result<Vec<F>, PoseidonError>>()?;

        let circuit_info = self.insert_for_circuit(&new_leaf_values, tree_id).await?;

        Ok(IMTCircuitInsertionInfo {
            old_root,
            circuit_info,
            first_index,
            low_nullifiers,
            pending_inserts,
        })
    }

    async fn batch_insert_nullifiers_with_circuit_info(
        &self,
        nullifiers: &[F],
        tree_id: &str,
    ) -> Result<Vec<IMTCircuitInsertionInfo<F>>, <Self as MutableTree<F>>::Error> {
        if nullifiers.is_empty() {
            return Ok(vec![]);
        }
        // Get the tree metadata
        let collection_name = format!("{}_{}", tree_id, "metadata");
        let db = self.database(<Self as MutableTree<F>>::MUT_DB_NAME);
        let collection = db.collection::<TreeMetadata<F>>(&collection_name);
        let metadata = collection
            .find_one(doc! {})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::ItemNotFound)?;

        let mut circuit_infos = vec![];
        for leaf_chunk in nullifiers.chunks(1 << metadata.sub_tree_height) {
            let circuit_info = self
                .insert_nullifiers_for_circuit(leaf_chunk, tree_id)
                .await?;
            circuit_infos.push(circuit_info);
        }
        Ok(circuit_infos)
    }
}

#[async_trait::async_trait]
impl<F: PrimeField + PoseidonParams> IndexedLeaves<F> for mongodb::Client {
    type Error = MerkleTreeError<mongodb::error::Error>;
    const DB: &'static str = "nightfall";

    async fn new_indexed_leaves_db(&self, tree_id: &str) -> Result<(), Self::Error> {
        // Create a new collection for indexed leaves
        let collection_name = format!("{}_{}", tree_id, "indexed_leaves");
        let db = self.database(<Self as IndexedLeaves<F>>::DB);
        let collection = db.collection::<IndexedLeaf<F>>(&collection_name);
        // Add a zero leaf as the first leaf
        collection
            .insert_one(IndexedLeaf::<F>::default())
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        Ok(())
    }

    async fn store_leaf(
        &self,
        leaf: F,
        index: Option<u64>,
        tree_id: &str,
    ) -> Result<Option<()>, Self::Error> {
        // If the new leaf is already in the db then we shouldn't store it.
        if self.get_leaf(Some(leaf), None, tree_id).await?.is_some() {
            debug!("Leaf already exists {}", leaf.to_string_rep());
            return Err(MerkleTreeError::LeafExists);
        }
        let collection_name = format!("{}_{}", tree_id, "indexed_leaves");
        let db = self.database(<Self as IndexedLeaves<F>>::DB);
        let collection = db.collection::<IndexedLeaf<F>>(&collection_name);
        // If the new leaf is not in the db then we should update the next value of its low nullifier.
        let low_leaf = if let Some(low_leaf) = self.get_low_leaf(&leaf, tree_id).await? {
            low_leaf
        } else {
            debug!("Could not find low leaf for leaf {}", leaf.to_string_rep());
            return Err(MerkleTreeError::LeafExists);
        };
        // internally we represent indices as i64 for compatibility with the Mongo DB Bson types.
        // externally we represent indices as u64 because a negative index doesn't make sense.
        let index = if let Some(i) = index {
            i
        } else {
            // if the index is not provided then we search for the maximum index in the db. If there are no
            // maximal leaves in the db then we set the index to 1. Then we add 1 to the result.
            collection
                .count_documents(doc! {})
                .await
                .map_err(MerkleTreeError::DatabaseError)?
        };
        // Create a new leaf entry
        let entry = IndexedLeaf::<F> {
            value: leaf,
            _id: index,
            next_index: low_leaf.next_index,
            next_value: low_leaf.next_value,
        };
        // Insert the new leaf into the db. This should work as we've already checked that the leaf is not in the db.
        // but that doesn't mean that the index hasn't already been written to the db. If the index is already in the db
        // then this will throw a duplicate key error. So we upsert the entry rather than insert it.
        let padded_leaf = serialize_to_padded_hex(&leaf)?;
        let padded_next_value = serialize_to_padded_hex(&entry.next_value)?;

        let bson_index = u64_to_i64_checked(index)?;
        let bson_next_index = u64_to_i64_checked(low_leaf.next_index)?;

        let updates_result = collection
            .update_one(
                doc! { "_id": bson_index },
                doc! {
                    "$set": {
                        "value": padded_leaf,
                        "next_index": bson_next_index,
                        "next_value": padded_next_value,
                    }
                },
            )
            .upsert(true)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;

        if updates_result.matched_count == 0 && updates_result.upserted_id.is_none() {
            return Err(MerkleTreeError::Error(
                "Failed to update or upsert the node in the database".to_string(),
            ));
        }
        let low_leaf_value: F = low_leaf.value;
        self.update_leaf(low_leaf_value, index, leaf, tree_id)
            .await?;
        Ok(Some(()))
    }

    async fn get_leaf(
        &self,
        value: Option<F>,
        next_value: Option<F>,
        tree_id: &str,
    ) -> Result<Option<IndexedLeaf<F>>, Self::Error> {
        let collection_name = format!("{}_{}", tree_id, "indexed_leaves");
        let db = self.database(<Self as IndexedLeaves<F>>::DB);
        let collection = db.collection::<IndexedLeaf<F>>(&collection_name);
        let query = match (value, next_value) {
            (Some(value), Some(next_value)) => {
                let value_padded_hex = serialize_to_padded_hex(&value)?;
                let next_value_padded_hex = serialize_to_padded_hex(&next_value)?;
                doc! {
                    "value": value_padded_hex,
                    "next_value": next_value_padded_hex
                }
            }
            (Some(value), None) => {
                let value_padded_hex = serialize_to_padded_hex(&value)?;

                doc! {
                    "value": value_padded_hex
                }
            }
            (None, Some(next_value)) => {
                let next_value_padded_hex = serialize_to_padded_hex(&next_value)?;

                doc! {
                   "next_value": next_value_padded_hex
                }
            }
            _ => doc! {},
        };

        collection
            .find_one(query)
            .await
            .map_err(MerkleTreeError::DatabaseError)
    }

    async fn get_low_leaf(
        &self,
        leaf_value: &F,
        tree_id: &str,
    ) -> Result<Option<IndexedLeaf<F>>, Self::Error> {
        let collection_name = format!("{}_{}", tree_id, "indexed_leaves");
        let db = self.database(<Self as IndexedLeaves<F>>::DB);
        let collection = db.collection::<IndexedLeaf<F>>(&collection_name);
        let padded_hex = serialize_to_padded_hex(leaf_value)?;
        let mut cursor = collection
            .find(doc! {"value": {"$lt": padded_hex}})
            .sort(doc! {"value": -1})
            .limit(1)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;

        if let Some(result) = cursor
            .try_next()
            .await
            .map_err(MerkleTreeError::DatabaseError)?
        {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    async fn update_leaf(
        &self,
        leaf: F,
        new_next_index: u64,
        new_next_value: F,
        tree_id: &str,
    ) -> Result<(), Self::Error> {
        let collection_name = format!("{}_{}", tree_id, "indexed_leaves");
        let db = self.database(<Self as IndexedLeaves<F>>::DB);
        let collection = db.collection::<IndexedLeaf<F>>(&collection_name);
        let padded_leaf = serialize_to_padded_hex(&leaf)?;
        let padded_next_value = serialize_to_padded_hex(&new_next_value)?;
        let query = doc! {"value": padded_leaf};
        let bson_next_index = u64_to_i64_checked(new_next_index)?;
        let update =
            doc! {"$set": {"next_index": bson_next_index, "next_value": padded_next_value}};
        let result = collection
            .update_one(query, update)
            .await
            .map_err(MerkleTreeError::DatabaseError)?;
        if result.matched_count == 0 && result.upserted_id.is_none() {
            return Err(MerkleTreeError::Error(
                "Failed to update or upsert the node in the database".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::merkle_trees::trees::helper_functions::make_complete_tree;
    use crate::tests_utils::*;
    use ark_bn254::Fr as Fr254;
    use ark_ff::Zero;
    use ark_std::{rand::Rng, UniformRand};

    /// makes a vector of n leaves with random values.
    fn make_rnd_leaves<N: UniformRand>(n: usize, mut rng: impl Rng) -> Vec<N> {
        let mut leaves = vec![];
        for _i in 0..n {
            leaves.push(N::rand(&mut rng));
        }
        leaves
    }

    fn check_pending_inserts(pending_inserts: Vec<LeafDBEntry<Fr254>>) -> bool {
        for insert in pending_inserts {
            if insert.value == Fr254::zero() || insert.next_index == Fr254::zero() {
                continue;
            }
            if insert.value > insert.next_value {
                return false;
            }
        }
        true
    }

    #[tokio::test]
    async fn test_indexed_merkle_tree() {
        let mut rng = ark_std::test_rng();
        // get a mongo container and connect to it
        let tree_name = "test_tree";
        const TREE_HEIGHT: u32 = 4;
        const SUB_TREE_HEIGHT: u32 = 3;
        const SUB_TREE_LEAF_CAPACITY: usize = 2_usize.pow(SUB_TREE_HEIGHT);
        let container = get_mongo().await;

        // generate some leaves for test purposes
        let leaves_1: Vec<Fr254> = vec![
            Fr254::from(0),
            Fr254::from(0),
            Fr254::from(20),
            Fr254::from(50),
            Fr254::from(70),
            Fr254::from(60),
            Fr254::from(90),
            Fr254::from(80),
        ];
        let leaves_2 = vec![
            Fr254::from(25),
            Fr254::from(35),
            Fr254::from(55),
            Fr254::from(5),
            Fr254::from(65),
            Fr254::from(40),
            Fr254::from(0),
            Fr254::from(95),
        ];
        let leaves_3 = make_rnd_leaves(SUB_TREE_LEAF_CAPACITY, &mut rng);
        let mut leaves = leaves_1.clone();
        leaves.append(&mut leaves_2.clone());
        let mut updated_leaves = leaves_3.clone();
        updated_leaves.append(&mut leaves_2.clone());

        // get a Mongo container and connect to it
        let client = get_db_connection(&container).await;
        // make a new tree
        <mongodb::Client as IndexedTree<Fr254>>::new_indexed_tree(
            &client,
            TREE_HEIGHT,
            SUB_TREE_HEIGHT,
            tree_name,
        )
        .await
        .unwrap();

        // now, only the first leaf of the merkle tree should be non-zero, so let's check that the root is correct
        let root: Fr254 = client.get_root(tree_name).await.unwrap();
        let hasher = <mongodb::Client as MutableTree<Fr254>>::TreeHasher::new();
        let leaf_0 = hasher
            .hash(&[Fr254::zero(), Fr254::zero(), Fr254::zero()])
            .unwrap();
        let test_tree = make_complete_tree(TREE_HEIGHT + 3, &hasher, &[leaf_0]);
        assert_eq!(test_tree[0], root);

        // check carefully that insert_leaves is working
        // insert some leaves into the indexed Merkle tree note, the tree doesn't store the leaves directly
        // but rather stores the hash of the leaf value, index and next value.
        // so this root won't derive from the inserted leaves but rather the hash of the hash of the leaf value, index and next value.
        let indexed_root = client.insert_leaves(&leaves_1, tree_name).await.unwrap();
        // compute the expected hashes from leaves_1
        let leaves_1_triples = vec![
            (0, 10, 20),
            (0, 0, 0),
            (0, 0, 0),
            (20, 11, 50),
            (50, 13, 60),
            (70, 15, 80),
            (60, 12, 70),
            (90, 0, 0),
            (80, 14, 90),
        ];
        let leaves_1_hashes = leaves_1_triples
            .into_iter()
            .enumerate()
            .map(|(i, lt)| {
                if lt.0 == 0 && i != 0 {
                    Fr254::zero()
                } else {
                    hasher
                        .hash(&[Fr254::from(lt.0), Fr254::from(lt.1), Fr254::from(lt.2)])
                        .unwrap()
                }
            })
            .collect::<Vec<Fr254>>();
        //put all the tree's leaves into a vec, noting the 'gap' of seven leaves to account for the sub tree size
        let mut all_leaves = vec![leaves_1_hashes[0]];
        all_leaves.append(&mut vec![Fr254::zero(); 7]);
        all_leaves.append(&mut leaves_1_hashes[1..].to_vec());
        let test_tree = make_complete_tree(TREE_HEIGHT + 3, &hasher, &all_leaves);
        assert_eq!(test_tree[0], indexed_root);

        // create a non-membership proof for a leaf (it's improbable that any of leaves_2 are in leaves_1)
        assert!(!leaves_1.contains(&leaves_2[0]));
        let leaf = leaves_2[0];

        let non_membership_proof = client
            .get_non_membership_proof(&leaf, tree_name)
            .await
            .unwrap();

        assert!(client
            .verify_non_membership_proof(&non_membership_proof, &indexed_root)
            .is_ok());

        // now try getting a non-membership proof for a leaf that _is_ in the tree
        // this should fail
        let leaf = leaves_1[0];
        let non_membership_proof = client.get_non_membership_proof(&leaf, tree_name).await;
        assert!(non_membership_proof.is_err());

        // insert the leaves_2 set into the tree and the previously passing test should now fail
        let leaves_2_info = client
            .insert_nullifiers_for_circuit(&leaves_2, tree_name)
            .await
            .unwrap();
        let indexed_root = leaves_2_info.circuit_info.new_root;
        let leaf = leaves_2[0];
        let non_membership_proof = client.get_non_membership_proof(&leaf, tree_name).await;
        assert!(non_membership_proof.is_err());
        assert!(check_pending_inserts(leaves_2_info.pending_inserts));

        // Check that the root has been correctly computed
        let leaves_1_2_triples = vec![
            (0, 19, 5),
            (0, 0, 0),
            (0, 0, 0),
            (20, 16, 25),
            (50, 18, 55),
            (70, 15, 80),
            (60, 20, 65),
            (90, 23, 95),
            (80, 14, 90),
        ];
        let leaves_1_2_hashes = leaves_1_2_triples
            .into_iter()
            .enumerate()
            .map(|(i, lt)| {
                if lt.0 == 0 && i != 0 {
                    Fr254::zero()
                } else {
                    hasher
                        .hash(&[Fr254::from(lt.0), Fr254::from(lt.1), Fr254::from(lt.2)])
                        .unwrap()
                }
            })
            .collect::<Vec<Fr254>>();
        let mut all_leaves = vec![leaves_1_2_hashes[0]];
        all_leaves.append(&mut vec![Fr254::zero(); 7]);
        all_leaves.append(&mut leaves_1_2_hashes[1..].to_vec());
        let test_tree = make_complete_tree(TREE_HEIGHT + 3, &hasher, &all_leaves);

        assert_eq!(test_tree[0], leaves_2_info.circuit_info.old_root);

        let leaves_2_triples = vec![
            (0, 19, 5),
            (0, 0, 0),
            (0, 0, 0),
            (20, 16, 25),
            (50, 18, 55),
            (70, 15, 80),
            (60, 20, 65),
            (90, 23, 95),
            (80, 14, 90),
            (25, 17, 35),
            (35, 21, 40),
            (55, 13, 60),
            (5, 10, 20),
            (65, 12, 70),
            (40, 11, 50),
            (0, 0, 0),
            (95, 0, 0),
        ];
        let leaves_2_hashes = leaves_2_triples
            .into_iter()
            .enumerate()
            .map(|(i, lt)| {
                if lt.0 == 0 && i != 0 {
                    Fr254::zero()
                } else {
                    hasher
                        .hash(&[Fr254::from(lt.0), Fr254::from(lt.1), Fr254::from(lt.2)])
                        .unwrap()
                }
            })
            .collect::<Vec<Fr254>>();
        let mut all_leaves = vec![leaves_2_hashes[0]];
        all_leaves.append(&mut vec![Fr254::zero(); 7]);
        all_leaves.append(&mut leaves_2_hashes[1..].to_vec());
        let test_tree = make_complete_tree(TREE_HEIGHT + 3, &hasher, &all_leaves);
        assert_eq!(test_tree[0], leaves_2_info.circuit_info.new_root);

        // a leaf from leaves_3 should still pass
        let leaf = leaves_3[0];
        let non_membership_proof = client
            .get_non_membership_proof(&leaf, tree_name)
            .await
            .unwrap();
        assert!(client
            .verify_non_membership_proof(&non_membership_proof, &indexed_root)
            .is_ok());

        // insert the leaves_3 set into the tree and the previously passing test should now fail
        print!(" Adding random leaves leaves_3");
        let insert_info = client
            .insert_nullifiers_for_circuit(&leaves_3, tree_name)
            .await
            .unwrap();
        let leaf = leaves_3[0];
        let non_membership_proof = client.get_non_membership_proof(&leaf, tree_name).await;
        assert!(non_membership_proof.is_err());
        assert!(check_pending_inserts(insert_info.pending_inserts));

        // Check that the insert infos line up
        assert_eq!(indexed_root, insert_info.old_root);
    }
}
