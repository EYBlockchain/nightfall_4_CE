use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalSerialize;
use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::types::{H256, I256};
use futures::TryStreamExt;
use hex::encode;
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use jf_primitives::trees::{Directions, PathElement};
use log::{debug, error, info};
use mongodb::error::{ErrorKind, WriteFailure::WriteError};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str;

use crate::domain::entities::{
    CommitmentStatus, Request, RequestCommitmentMapping, RequestStatus, WithdrawData,
};
use crate::ports::db::{CommitmentDB, RequestCommitmentMappingDB, RequestDB, WithdrawalDB};
use crate::{
    domain::entities::Preimage,
    driven::contract_functions::contract_type_conversions::FrBn254,
    ports::{commitments::Commitment, db::CommitmentEntryDB},
};
use jf_primitives::{poseidon::PoseidonError, trees::MembershipProof};
use lib::serialization::{ark_de_hex, ark_se_hex};

pub const DB: &str = "nightfall";

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
                info!("{} Got an error inserting request: {}", request.uuid, e);
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
        self.database(DB)
            .collection::<Request>("requests")
            .update_one(filter, update)
            .await
            .ok()?;
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
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub key: Fr254,
    #[serde(
        serialize_with = "ark_se_hex",
        deserialize_with = "ark_de_hex",
        default
    )]
    pub nullifier: Fr254,
    pub layer_1_transaction_hash: Option<H256>,
    pub layer_2_block_number: Option<I256>,
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
    fn new(preimage: Preimage, key: Fr254, nullifier: Fr254, status: CommitmentStatus) -> Self {
        Self {
            preimage,
            status,
            nullifier,
            key,
            layer_1_transaction_hash: None,
            layer_2_block_number: None,
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
                error!("Error adding request-commitment mapping: {}", e);
                Err(format!("DB error: {}", e))
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
    async fn get_available_commitments(&self, nf_token_id: Fr254) -> Option<Vec<CommitmentEntry>> {
        let mut nf_token_id_serialised = Vec::new();
        nf_token_id
            .serialize_compressed(&mut nf_token_id_serialised)
            .ok();
        let filter = doc! {
            "preimage.nf_token_id": hex::encode(nf_token_id_serialised),
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
        let filter = doc! { "key": hex::encode(k.into_bigint().to_bytes_le()) };
        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .find_one(filter)
            .await
            .expect("Database error") // we can't really proceed at this point
    }

    async fn get_balance(&self, nf_token_id: &Fr254) -> Option<Fr254> {
        let filter = doc! {
            "preimage.nf_token_id": hex::encode(nf_token_id.into_bigint().to_bytes_le()),
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

    async fn mark_commitments_pending_spend(&self, commitments: Vec<Fr254>) -> Option<()> {
        let commitment_str = commitments
            .into_iter()
            .map(|c| hex::encode(c.into_bigint().to_bytes_le()))
            .collect::<Vec<_>>();
        let filter = doc! { "key": { "$in": commitment_str }};
        let update = doc! {"$set": { "status": "PendingSpend" }};
        self.database(DB)
            .collection::<CommitmentEntry>("commitments")
            .update_many(filter, update)
            .await
            .ok()?;
        Some(())
    }

    async fn mark_commitments_pending_creation(&self, commitments: Vec<Fr254>) -> Option<()> {
        let commitment_str = commitments
            .into_iter()
            .map(|c| hex::encode(c.into_bigint().to_bytes_le()))
            .collect::<Vec<_>>();
        let filter = doc! { "key": { "$in": commitment_str }};
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
            .map(|c| hex::encode(c.into_bigint().to_bytes_le()))
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
        l1_hash: Option<H256>,
        l2_blocknumber: Option<I256>,
    ) -> Option<()> {
        let commitment_str = commitments
            .iter()
            .map(|c| hex::encode(c.into_bigint().to_bytes_le()))
            .collect::<Vec<_>>();
        let l1_hash = l1_hash.map(|h| h.encode_hex());
        let l2_blocknumber = l2_blocknumber.map(|b| b.encode_hex());
        let filter = doc! { "key": { "$in": commitment_str }};
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
        let filter = doc! { "key": hex::encode(key.into_bigint().to_bytes_le())};
        let update =
            doc! {"$set": { "nullifier": hex::encode(nullifier.into_bigint().to_bytes_le()) }};

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
                debug!("Store commitment result {:#?}", ins);
                Some(())
            }
            Err(e) => {
                error!(
                    "Got an error inserting commitment: {:#?}, {}",
                    commitment, e
                );
                None
            }
        }
    }

    /// function to store multiple commitments in the database, optionally ignoring duplicate key errors
    async fn store_commitments(
        &self,
        commitments: &[CommitmentEntry],
        dup_key_check: bool,
    ) -> Option<()> {
        if commitments.is_empty() {
            return Some(());
        }
        let res = self
            .database(DB)
            .collection::<CommitmentEntry>("commitments")
            .insert_many(commitments)
            .await;
        match res {
            Ok(_) => Some(()),
            // unpack the Mongo error and check if it's a duplicate key error. If so, behave according to dup_key_check
            Err(e) => {
                match e.kind.as_ref() {
                    ErrorKind::Write(WriteError(write_error)) => {
                        if write_error.code == 11000 && !dup_key_check {
                            println!("Duplicate key error: {:?}", write_error);
                            // duplicate key error but we don't care
                            Some(())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
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
                info!("Got an error inserting pending withdrawal: {}", e);
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
        let mut bytes = vec![];
        key.serialize_compressed(&mut bytes).ok()?;

        let hex_str = encode(&bytes);
        let query = doc! {"key": hex_str};

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
