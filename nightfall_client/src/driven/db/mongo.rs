use crate::{
    domain::entities::{
        CommitmentStatus, Preimage, Request, RequestCommitmentMapping, RequestStatus, WithdrawData,
    },
    ports::db::{CommitmentDB, RequestCommitmentMappingDB, RequestDB, WithdrawalDB},
    ports::{commitments::Commitment, db::CommitmentEntryDB},
};
use alloy::primitives::Address;
use alloy::primitives::TxHash;
use ark_bn254::Fr as Fr254;
use ark_ff::PrimeField;
use async_trait::async_trait;
use futures::TryStreamExt;
use jf_primitives::{poseidon::PoseidonError, trees::MembershipProof};
use jf_primitives::{
    poseidon::{FieldHasher, Poseidon},
    trees::{Directions, PathElement},
};
use lib::hex_conversion::HexConvertible;
use lib::{
    contract_conversions::FrBn254,
    serialization::{ark_de_hex, ark_se_hex},
};
use log::{debug, error, info};
use mongodb::{
    bson::doc,
    error::{ErrorKind, WriteFailure::WriteError},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Client,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Digest;
use std::{fmt::Debug, str};

pub const DB: &str = "nightfall";
pub const PROPOSED_BLOCKS_COLLECTION: &str = "ProposedBlocks";

// To do, move this to lib, and change proposer to use it as well.
#[async_trait::async_trait]
pub trait BlockStorageDB {
    async fn store_block(&self, block: &StoredBlock) -> Option<()>;
    async fn get_block_by_number(&self, block_number: u64) -> Option<StoredBlock>;
    async fn get_all_blocks(&self) -> Option<Vec<StoredBlock>>;
    async fn delete_block_by_number(&self, block_number: u64) -> Option<()>;
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// A struct representing a stored block in the database
/// To update local mempool so that proposers won't assemble the block with the same transactions onchain, proposers can just check if the commitments for deposit/client_transactions in mempool have appeared in the stored block.
/// To sync the status, proposers need to check if block is the same as the one it remembers when layer 2 block number expected and onchain are the same, since commitments are unique, it's enought to check the hash of commitments in block.
/// So we only store commitments and layer2_block_number in the block database.
pub struct StoredBlock {
    pub layer2_block_number: u64,
    pub commitments: Vec<String>,
    pub proposer_address: Address,
}
impl StoredBlock {
    pub fn hash(&self) -> Fr254 {
        let mut bytes = Vec::new();
        for c in &self.commitments {
            bytes.extend_from_slice(c.as_bytes());
        }
        bytes.extend_from_slice(self.proposer_address.as_slice());
        let hash = Sha256::digest(&bytes);
        Fr254::from_be_bytes_mod_order(&hash)
    }
}
#[async_trait::async_trait]
impl BlockStorageDB for mongodb::Client {
    async fn store_block(&self, block: &StoredBlock) -> Option<()> {
        // check if the block already exists
        let filter = doc! { "layer2_block_number": block.layer2_block_number as i64 };
        let existing_block = self
            .database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .find_one(filter.clone())
            .await
            .ok()?;
        if existing_block.is_some() {
            // if the block already exists, we need to update it
            let update = doc! { "$set": { "commitments": block.commitments.clone() } };
            self.database(DB)
                .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
                .update_one(filter, update)
                .await
                .ok()?;
            return Some(());
        }
        // if the block doesn't exist, we need to insert it
        self.database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .insert_one(block)
            .await
            .ok()?;
        Some(())
    }

    async fn get_block_by_number(&self, block_number: u64) -> Option<StoredBlock> {
        let filter = doc! { "layer2_block_number": block_number as i64 };
        self.database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .find_one(filter)
            .await
            .ok()?
    }

    async fn get_all_blocks(&self) -> Option<Vec<StoredBlock>> {
        let cursor = self
            .database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .find(doc! {})
            .await
            .ok()?;
        cursor.try_collect().await.ok()
    }
    async fn delete_block_by_number(&self, block_number: u64) -> Option<()> {
        let filter = doc! { "layer2_block_number": block_number as i64 };
        self.database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .delete_one(filter)
            .await
            .ok()?;
        Some(())
    }
}

/// Utility function to convert a MembershipProof<Fr254> to a MemershipProof<FrBn254>
///
/// This will then be serialisable with Serde. This is needed to be able to store it in a Mongo Db (and possibly others).
/// FrBn254 is newtype wrapper around Fr254, so this function is just a cast really.
// the need for this function. It will do for now though.
pub fn to_frbn254_proof(proof: MembershipProof<Fr254>) -> MembershipProof<FrBn254> {
    MembershipProof {
        node_value: FrBn254(proof.node_value),
        sibling_path: proof
            .sibling_path
            .into_iter()
            .map(|p| PathElement {
                direction: p.direction,
                value: FrBn254(p.value),
            })
            .collect(),
        leaf_index: proof.leaf_index,
    }
}

pub fn to_fr254_proof(proof: MembershipProof<FrBn254>) -> MembershipProof<Fr254> {
    MembershipProof {
        node_value: proof.node_value.0,
        sibling_path: proof
            .sibling_path
            .into_iter()
            .map(|p| PathElement {
                direction: p.direction,
                value: p.value.0,
            })
            .collect(),
        leaf_index: proof.leaf_index,
    }
}

#[async_trait]
impl RequestDB for Client {
    async fn store_request(&self, request_id: &str, status: RequestStatus) -> Option<()> {
        let request = Request {
            uuid: request_id.to_string(),
            status,
        };
        let result = self
            .database(DB)
            .collection::<Request>("requests")
            .insert_one(&request)
            .await;
        match result {
            Ok(_) => Some(()),
            Err(e) => {
                error!("{} Got an error inserting request: {}", request.uuid, e);
                None
            }
        }
    }

    async fn get_request(&self, request_id: &str) -> Option<Request> {
        let filter = doc! { "uuid": request_id };
        self.database(DB)
            .collection::<Request>("requests")
            .find_one(filter)
            .await
            .ok()?
    }

    async fn update_request(&self, request_id: &str, status: RequestStatus) -> Option<()> {
        let filter = doc! { "uuid": request_id };
        let update = doc! {"$set": { "status": status.to_string() }};
        let result = self
            .database(DB)
            .collection::<Request>("requests")
            .update_one(filter, update)
            .await;
        if let Err(e) = result {
            error!("{request_id} Got an error updating request: {e}");
            return None;
        }
        Some(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DBMembershipProof {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    node_value: Fr254,
    sibling_path: Vec<bool>,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    sibling_values: Vec<Fr254>,
    leaf_index: usize,
}

impl From<MembershipProof<Fr254>> for DBMembershipProof {
    fn from(proof: MembershipProof<Fr254>) -> Self {
        let (sibling_path, sibling_values): (Vec<bool>, Vec<Fr254>) = proof
            .sibling_path
            .iter()
            .map(|path_element| {
                let conversion = match path_element.direction {
                    Directions::HashWithThisNodeOnLeft => false,
                    Directions::HashWithThisNodeOnRight => true,
                };
                (conversion, path_element.value)
            })
            .unzip();
        Self {
            node_value: proof.node_value,
            sibling_path,
            sibling_values,
            leaf_index: proof.leaf_index,
        }
    }
}

impl From<DBMembershipProof> for MembershipProof<Fr254> {
    fn from(proof: DBMembershipProof) -> Self {
        let sibling_path = proof
            .sibling_path
            .iter()
            .zip(proof.sibling_values.iter())
            .map(|(&boolean, &value)| {
                let direction = match boolean {
                    false => Directions::HashWithThisNodeOnLeft,
                    true => Directions::HashWithThisNodeOnRight,
                };
                PathElement::<Fr254> { direction, value }
            })
            .collect::<Vec<PathElement<Fr254>>>();

        Self {
            node_value: proof.node_value,
            sibling_path,
            leaf_index: proof.leaf_index,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct CommitmentEntry {
    pub preimage: Preimage,
    pub status: CommitmentStatus,
    #[serde(
        rename = "_id",
        serialize_with = "ark_se_hex",
        deserialize_with = "ark_de_hex"
    )]
    pub key: Fr254,
    #[serde(
        serialize_with = "ark_se_hex",
        deserialize_with = "ark_de_hex",
        default
    )]
    pub nullifier: Fr254,
    pub layer_1_transaction_hash: Option<TxHash>, // hash of the L1 transaction that created this commitment
    pub layer_2_block_number: Option<i64>, // block number of the L2 block that created this commitment
}

impl Commitment for CommitmentEntry {
    fn get_preimage(&self) -> Preimage {
        self.preimage
    }
    fn get_nf_token_id(&self) -> Fr254 {
        self.preimage.nf_token_id
    }
    fn get_public_key(&self) -> nf_curves::ed_on_bn254::BJJTEAffine {
        self.preimage.public_key
    }
    fn get_salt(&self) -> Fr254 {
        self.preimage.get_salt()
    }
    fn get_nf_slot_id(&self) -> Fr254 {
        self.preimage.nf_slot_id
    }
    fn get_value(&self) -> Fr254 {
        self.preimage.value
    }
    fn hash(&self) -> Result<Fr254, PoseidonError> {
        self.preimage.hash()
    }
    fn get_secret_preimage(&self) -> crate::domain::entities::DepositSecret {
        self.preimage.get_secret_preimage()
    }
}
impl CommitmentEntryDB for CommitmentEntry {
    fn new(
        preimage: Preimage,
        nullifier: Fr254,
        status: CommitmentStatus,
        layer_1_transaction_hash: Option<TxHash>,
        layer_2_block_number: Option<i64>,
    ) -> Self {
        let key = preimage.hash().expect("failed to hash preimage");
        Self {
            preimage,
            status,
            nullifier,
            key,
            layer_1_transaction_hash,
            layer_2_block_number,
        }
    }
    fn get_status(&self) -> CommitmentStatus {
        self.status
    }
}

#[async_trait]
impl RequestCommitmentMappingDB for Client {
    async fn add_mapping(&self, request_id: &str, commitment_hash: &str) -> Result<(), String> {
        let mapping = RequestCommitmentMapping {
            request_id: request_id.to_owned(),
            commitment_hash: commitment_hash.to_owned(),
        };

        let result = self
            .database(DB)
            .collection::<RequestCommitmentMapping>("request_commitment_mappings")
            .insert_one(&mapping)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error adding request-commitment mapping: {e}");
                Err(format!("DB error: {e}"))
            }
        }
    }

    async fn get_requests_by_commitment(&self, commitment_hash: &str) -> Option<Vec<String>> {
        let filter = doc! { "commitment_hash": commitment_hash };
        let cursor = self
            .database(DB)
            .collection::<RequestCommitmentMapping>("request_commitment_mappings")
            .find(filter)
            .await
            .ok()?;

        let mappings = cursor
            .try_collect::<Vec<_>>()
            .await
            .ok()?
            .into_iter()
            .map(|doc| doc.request_id)
            .collect();

        Some(mappings)
    }

    async fn get_commitments_by_request(&self, request_id: &str) -> Option<Vec<String>> {
        let filter = doc! { "request_id": request_id };
        let cursor = self
            .database(DB)
            .collection::<RequestCommitmentMapping>("request_commitment_mappings")
            .find(filter)
            .await
            .ok()?;

        let mappings = cursor
            .try_collect::<Vec<_>>()
            .await
            .ok()?
            .into_iter()
            .map(|doc| doc.commitment_hash)
            .collect();

        Some(mappings)
    }
}

#[async_trait]
impl CommitmentDB<Fr254, CommitmentEntry> for Client {
    async fn get_all_commitments(
        &self,
    ) -> Result<Vec<(Fr254, CommitmentEntry)>, mongodb::error::Error> {
        let mut cursor = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .find(doc! {})
            .await?;
        let mut result: Vec<(Fr254, CommitmentEntry)> = Vec::new();
        while cursor.advance().await? {
            let v = cursor.deserialize_current()?;
            result.push((v.key, v))
        }
        Ok(result)
    }
    /// Atomically reserves commitments by changing their status from `Unspent` to `PendingSpend`.
    ///
    /// This prevents race conditions where multiple processes try to spend the same commitments
    /// at the same time. Only commitments that are still `Unspent` will be updated and returned.
    async fn reserve_commitments_atomic(
        &self,
        commitment_ids: Vec<Fr254>,
    ) -> Result<Vec<CommitmentEntry>, &'static str> {
        let mut reserved_commitments = Vec::new();

        for commitment_id in commitment_ids {
            let filter = doc! {
                "_id": commitment_id.to_hex_string(),
                "status": "Unspent"
            };

            let update = doc! {
                "$set": { "status": "PendingSpend" }
            };

            let options = FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build();
            // Atomically find and update the commitment
            if let Some(updated_commitment) = self
                .database(DB)
                .collection::<CommitmentEntry>("commitments")
                .find_one_and_update(filter, update)
                .with_options(options)
                .await
                .map_err(|_| "Database update failed")?
            {
                reserved_commitments.push(updated_commitment);
            } else {
                debug!("Failed to reserve commitment: {commitment_id:?}");
            }
        }
        Ok(reserved_commitments)
    }

    async fn get_available_commitments(&self, nf_token_id: Fr254) -> Option<Vec<CommitmentEntry>> {
        let filter = doc! {
            "preimage.nf_token_id": nf_token_id.to_hex_string(),
            "status": "Unspent"
        };
        let mut cursor = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .find(filter)
            .await
            .expect("Database error"); // we can't really proceed at this point
        let mut result: Vec<CommitmentEntry> = Vec::new();
        while cursor.advance().await.expect("Database error")
        // we can't really proceed at this point
        {
            let v = cursor.deserialize_current().expect("Deserialisation error"); // we can't really proceed at this point
            result.push(v)
        }
        if result.is_empty() {
            return None;
        };
        Some(result)
    }

    async fn get_commitment(&self, k: &Fr254) -> Option<CommitmentEntry> {
        let k_string = k.to_hex_string();
        debug!("Getting commitment with key: {k_string}");
        let commitment_1 = self
            .get_all_commitments()
            .await
            .expect("Database error")
            .into_iter()
            .find(|(key, _)| key.to_hex_string() == k_string);
        // now we can check if we found the commitment
        commitment_1.map(|(_, entry)| entry)
    }

    async fn get_balance(&self, nf_token_id: &Fr254) -> Option<Fr254> {
        let filter = doc! {
            "preimage.nf_token_id": nf_token_id.to_hex_string(),
            "status": "Unspent",
        };
        let mut cursor = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .find(filter.clone())
            .await
            .expect("Database error");
        let mut result: Vec<CommitmentEntry> = Vec::new();
        while cursor.advance().await.expect("Database error") {
            let v = cursor.deserialize_current().expect("Database error");
            result.push(v)
        }
        if result.is_empty() {
            return None;
        };
        // we need to sum the values
        let balance: Fr254 = result.iter().map(|entry| entry.preimage.value).sum();
        Some(balance)
    }

    async fn mark_commitments_pending_creation(&self, commitments: Vec<Fr254>) -> Option<()> {
        let commitment_str = commitments
            .into_iter()
            .map(|c| c.to_hex_string())
            .collect::<Vec<_>>();
        let filter = doc! { "_id": { "$in": commitment_str }};
        let update = doc! {"$set": { "status": "PendingCreation" }};
        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .update_many(filter, update)
            .await
            .ok()?;
        Some(())
    }

    // to mark commitments as nullified we search by nullifier, not commitment: when we receive a nullifier from
    // the blockchain, we won't know which commitment it corresponds to.
    async fn mark_commitments_spent(&self, nullifiers: Vec<Fr254>) -> Option<()> {
        let nullifiers_str = nullifiers
            .into_iter()
            .map(|c| c.to_hex_string())
            .collect::<Vec<_>>();
        let filter = doc! { "nullifier": { "$in": nullifiers_str }};
        let update = doc! {"$set": { "status": "Spent"}};
        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .update_many(filter, update)
            .await
            .ok()?;
        Some(())
    }

    async fn mark_commitments_unspent(
        &self,
        commitments: &[Fr254],
        l1_hash: Option<TxHash>,
        l2_blocknumber: Option<i64>,
    ) -> Option<()> {
        let commitment_str = commitments
            .iter()
            .map(|c| c.to_hex_string())
            .collect::<Vec<_>>();
        let l1_hash = l1_hash.map(|h| h.to_string());
        let filter = doc! { "_id": { "$in": commitment_str }};
        let update = doc! {"$set": { "status": "Unspent", "layer_1_transaction_hash": l1_hash, "layer_2_block_number": l2_blocknumber }};
        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .update_many(filter, update)
            .await
            .ok()?;
        Some(())
    }

    // we compute a nullifier for each spend commitment that we process.
    async fn add_nullifier(&self, key: &Fr254, nullifier: Fr254) -> Option<()> {
        let filter = doc! { "_id": key.to_hex_string() };
        let update = doc! {"$set": { "nullifier": nullifier.to_hex_string() }};

        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .update_one(filter, update)
            .await
            .ok()?;
        Some(())
    }

    async fn store_commitment(&self, commitment: CommitmentEntry) -> Option<()> {
        let result = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .insert_one(&commitment)
            .await;
        match result {
            Ok(ins) => {
                debug!("Store commitment result {ins:#?}");
                Some(())
            }
            Err(e) => {
                error!("Got an error inserting commitment: {commitment:#?}, {e}");
                None
            }
        }
    }

    /// function to store multiple commitments in the database, optionally ignoring duplicate _id errors
    async fn store_commitments(
        &self,
        commitments: &[CommitmentEntry],
        dup_key_check: bool,
    ) -> Option<()> {
        if commitments.is_empty() {
            return Some(());
        }
        debug!(
            "Storing commitments with hashes{:?} ",
            commitments
                .iter()
                .map(|c| c.key.to_hex_string())
                .collect::<Vec<_>>()
        );
        let res = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .insert_many(commitments)
            .await;
        match res {
            Ok(_) => Some(()),
            // unpack the Mongo errors and check if it's a duplicate _id error. If so, behave according to dup_key_check
            Err(e) => {
                match e.kind.as_ref() {
                    ErrorKind::Write(WriteError(write_error)) => {
                        if write_error.code == 11000 && !dup_key_check {
                            debug!("Duplicate _id error: {write_error:?}");
                            // duplicate _id error but we don't care
                            Some(())
                        } else {
                            error!("Unhandled Write Error storing commitments: {e} duplicate key check {dup_key_check}");
                            None
                        }
                    }
                    _ => {
                        error!(
                            "Unhandled Error storing commitments: {e} duplicate key check {dup_key_check}"
                        );
                        None
                    }
                }
            }
        }
    }

    /// Delete commitments by their IDs (hashes)
    async fn delete_commitments(&self, commitment_ids: Vec<Fr254>) -> Option<()> {
        if commitment_ids.is_empty() {
            return Some(());
        }

        let commitment_strs: Vec<String> = commitment_ids
            .into_iter()
            .map(|c| c.to_hex_string())
            .collect();

        debug!("Deleting commitments with hashes {commitment_strs:?}");

        let filter = doc! { "_id": { "$in": &commitment_strs }};

        let res = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .delete_many(filter)
            .await;

        match res {
            Ok(del_res) => {
                debug!("Deleted {} commitments", del_res.deleted_count);
                Some(())
            }
            Err(e) => {
                error!("Error deleting commitments {commitment_strs:?}: {e}");
                None
            }
        }
    }
}

/// Struct stored in the pending withdrawal database
#[derive(Debug, Deserialize, Serialize)]
pub struct PendingWithdrawal {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub key: Fr254,
    pub data: WithdrawData,
}

impl PendingWithdrawal {
    /// Create a new instance
    pub fn new(data: WithdrawData) -> Self {
        let poseidon = Poseidon::<Fr254>::new();
        // Unwrap is safe because this is a permitted size for the hash.
        let key = poseidon
            .hash(&[
                data.nf_token_id,
                data.withdraw_address,
                data.value,
                data.withdraw_fund_salt,
            ])
            .unwrap();
        Self { key, data }
    }
}

impl From<WithdrawData> for PendingWithdrawal {
    fn from(data: WithdrawData) -> Self {
        PendingWithdrawal::new(data)
    }
}

impl From<PendingWithdrawal> for WithdrawData {
    fn from(data: PendingWithdrawal) -> Self {
        data.data
    }
}

impl WithdrawalDB<Fr254, PendingWithdrawal> for Client {
    async fn store_withdrawal(&mut self, data: PendingWithdrawal) -> Option<()> {
        let result = self
            .database(DB)
            .collection::<PendingWithdrawal>("withdrawals")
            .insert_one(&data)
            .await;
        match result {
            Ok(_) => Some(()),
            Err(e) => {
                info!("Got an error inserting pending withdrawal: {e}");
                None
            }
        }
    }

    async fn get_pending_withdrawals(&self) -> Option<Vec<PendingWithdrawal>> {
        let mut cursor = self
            .database(DB)
            .collection::<PendingWithdrawal>("withdrawals")
            .find(doc! {})
            .await
            .ok()?;
        let mut result: Vec<PendingWithdrawal> = Vec::new();
        while cursor.advance().await.ok()? {
            let v = cursor.deserialize_current().ok()?;
            result.push(v)
        }
        Some(result)
    }

    async fn remove_withdrawal(&mut self, key: Fr254) -> Option<()> {
        let query = doc! {"key": key.to_hex_string()};

        let delete_count = self
            .database(DB)
            .collection::<PendingWithdrawal>("withdrawals")
            .delete_one(query)
            .await
            .ok()?;

        if delete_count.deleted_count == 1 {
            Some(())
        } else {
            None
        }
    }
}
