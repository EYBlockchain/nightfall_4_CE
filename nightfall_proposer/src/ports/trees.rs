//! This module contains the interfaces for the three Merkle Trees a Proposer works with.

use ark_bn254::Fr as Fr254;
use ark_ff::PrimeField;
use bson::Document;
use configuration::settings::get_settings;
use jf_primitives::{poseidon::PoseidonParams, trees::MembershipProof};
use lib::merkle_trees::trees::{IndexedTree, MerkleTreeError, MutableTree};
use log::debug;
use mongodb::Client;

/// Trait defining the functionality of a commitment tree.
#[async_trait::async_trait]
pub trait CommitmentTree<F>: MutableTree<F>
where
    F: PrimeField + PoseidonParams,
{
    /// The struct used for Circuit insertion Info.
    type CircuitInfo;
    /// The name of the commitment tree (Nightfall only has one so it can be a constant)
    const TREE_NAME: &'static str;
    /// Add leaves into the tree.
    async fn append_sub_trees(&self, leaves: &[F], update_tree: bool) -> Result<F, Self::Error> {
        let (result, _) =
            <Self as MutableTree<F>>::append_sub_trees(self, leaves, update_tree, Self::TREE_NAME)
                .await?;
        Ok(result)
    }
    /// Inserts leaves into the tree and returns information allowing us to verify in a circuit.
    async fn insert_for_circuit(&mut self, leaves: &[F]) -> Result<Self::CircuitInfo, Self::Error>;
    /// let's multiple sub trees be added in a single batch - it calls insert_subtree for each sub tree
    async fn batch_insert_with_circuit_info(
        &mut self,
        commitments: &[F],
    ) -> Result<Vec<Self::CircuitInfo>, Self::Error>;
    /// get a new commitment tree
    async fn new_commitment_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
    ) -> Result<(), Self::Error> {
        Self::new_mutable_tree(self, tree_height, sub_tree_height, Self::TREE_NAME).await
    }
    /// get the root of the tree
    async fn get_root(&self) -> Result<F, Self::Error>;
    /// reset the tree
    async fn reset_tree(&self) -> Result<(), Self::Error>
    where
        Self: MutableTree<F, Error = MerkleTreeError<mongodb::error::Error>>,
    {
        let _ = <Self as MutableTree<F>>::reset_mutable_tree(self, Self::TREE_NAME).await;
        // select the proposer to use
        use configuration::settings::get_settings;
        use mongodb::Client;
        let uri = &get_settings().nightfall_proposer.db_url;
        let client = Client::with_uri_str(uri)
            .await
            .expect("Could not create database connection");
        // it's not enough just to connect to a database, we need to initialise some trees in it
        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .map_err(|e| {
                log::error!("Could not create commitment tree metadata: {:?}", e);
                e
            })?;
        Ok(())
    }
}

/// Trait defining the functionality of a nullifier tree.
#[async_trait::async_trait]
pub trait NullifierTree<F>: IndexedTree<F>
where
    F: PrimeField + PoseidonParams,
{
    /// The struct used for Circuit insertion Info.
    type CircuitInfo;
    /// The name of the nullifier tree (Nightfall only has one so it can be a constant)
    const TREE_NAME: &'static str;
    /// create a new nullifier tree
    async fn new_nullifier_tree(
        &self,
        tree_height: u32,
        sub_tree_height: u32,
    ) -> Result<(), <Self as MutableTree<F>>::Error> {
        <Self as IndexedTree<F>>::new_indexed_tree(
            self,
            tree_height,
            sub_tree_height,
            Self::TREE_NAME,
        )
        .await
    }

    /// inserts new nullifiers into the tree with the given leaves and returns the new root.
    async fn insert_nullifiers(
        &self,
        nullifiers: &[F],
    ) -> Result<F, <Self as MutableTree<F>>::Error> {
        <Self as IndexedTree<F>>::insert_leaves(self, nullifiers, Self::TREE_NAME).await
    }
    /// gets a non-inclusion proof for a nullifier in the tree.
    async fn get_non_membership_proof(
        &self,
        leaf: &F,
    ) -> Result<MembershipProof<F>, <Self as MutableTree<F>>::Error> {
        <Self as IndexedTree<F>>::get_non_membership_proof(self, leaf, Self::TREE_NAME).await
    }
    /// Inserts leaves into the tree and returns information allowing us to verify in a circuit.
    async fn insert_for_circuit(
        &mut self,
        leaves: &[F],
    ) -> Result<Self::CircuitInfo, <Self as MutableTree<F>>::Error>;
    /// let's multiple sub trees be added in a single batch - it calls insert_subtree for each sub tree
    async fn batch_insert_with_circuit_info(
        &mut self,
        commitments: &[F],
    ) -> Result<Vec<Self::CircuitInfo>, <Self as MutableTree<F>>::Error>;

    // Reset the tree
    async fn reset_tree(&self) -> Result<(), <Self as MutableTree<F>>::Error>
    where
        Self: MutableTree<F, Error = MerkleTreeError<mongodb::error::Error>>,
    {
        debug!("resting NullifierTree");
        // Step 1: Drop mutable tree collections
        let _ = <Self as MutableTree<F>>::reset_mutable_tree(self, Self::TREE_NAME).await;
        // Step 2: Drop indexed_leaves collection
        let uri = &get_settings().nightfall_proposer.db_url;
        let client = Client::with_uri_str(uri)
            .await
            .expect("Could not create database connection");

        let db = client.database("nightfall");

        // Drop collections
        let indexed_collection = db.collection::<Document>("Nullifiers_indexed_leaves");
        if let Err(e) = indexed_collection.drop().await {
            if !e.to_string().contains("ns not found") {
                return Err(MerkleTreeError::DatabaseError(e));
            }
        }
        // select the proposer to use

        let uri = &get_settings().nightfall_proposer.db_url;
        let client = Client::with_uri_str(uri)
            .await
            .expect("Could not create database connection");
        // it's not enough just to connect to a database, we need to initialise some trees in it

        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .map_err(|e| {
                log::error!("Could not create NullifierTree metadata: {:?}", e);
                <Self as MutableTree<F>>::Error::from(e)
            })?;
        Ok(())
    }
}

/// Trait defining the functionality of a historic root tree.
#[async_trait::async_trait]
pub trait HistoricRootTree<F>: MutableTree<F>
where
    F: PrimeField + PoseidonParams,
{
    /// The name of the historic root tree (Nightfall only has one so it can be a constant)
    const TREE_NAME: &'static str;
    /// Add leaves into the tree.
    async fn append_historic_commitment_root(
        &self,
        historic_commitment_root: &F,
        update_tree: bool,
    ) -> Result<F, Self::Error> {
        let (result, _) = <Self as MutableTree<F>>::append_sub_trees(
            self,
            &[*historic_commitment_root],
            update_tree,
            Self::TREE_NAME,
        )
        .await?;
        Ok(result)
    }
    /// get a new historic root tree
    async fn new_historic_root_tree(&self, tree_height: u32) -> Result<(), Self::Error> {
        <Self as MutableTree<F>>::new_mutable_tree(self, tree_height, 0, Self::TREE_NAME).await
    }
    /// check if a historic root is in the tree
    async fn is_historic_root(&self, leaf: &F) -> Result<bool, Self::Error> {
        <Self as MutableTree<F>>::is_leaf(self, leaf, Self::TREE_NAME).await
    }
    /// Get a membership proof for a leaf in the tree.
    async fn get_membership_proof(
        &self,
        leaf: Option<&F>,
        leaf_index: Option<u64>,
    ) -> Result<MembershipProof<F>, Self::Error> {
        if let Some(leaf_value) = leaf {
            if leaf_value.is_zero() {
                <Self as MutableTree<F>>::get_membership_proof(
                    self,
                    None,
                    Some(0u64),
                    Self::TREE_NAME,
                )
                .await
            } else {
                <Self as MutableTree<F>>::get_membership_proof(
                    self,
                    leaf,
                    leaf_index,
                    Self::TREE_NAME,
                )
                .await
            }
        } else {
            <Self as MutableTree<F>>::get_membership_proof(self, leaf, leaf_index, Self::TREE_NAME)
                .await
        }
    }
    // Reset the tree
    async fn reset_tree(&self) -> Result<(), Self::Error>
    where
        Self: MutableTree<F, Error = MerkleTreeError<mongodb::error::Error>>,
    {
        debug!("resting HistoricRootTree");
        let _ = <Self as MutableTree<F>>::reset_mutable_tree(self, Self::TREE_NAME).await;
        // select the proposer to use
        use configuration::settings::get_settings;
        use mongodb::Client;
        let uri = &get_settings().nightfall_proposer.db_url;
        let client = Client::with_uri_str(uri)
            .await
            .expect("Could not create database connection");
        // it's not enough just to connect to a database, we need to initialise some trees in it
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .map_err(|e| {
                log::error!("Could not create HistoricRootTree metadata: {:?}", e);
                e
            })?;
        Ok(())
    }
}
