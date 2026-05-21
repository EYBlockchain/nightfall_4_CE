use crate::{
    domain::entities::{
        ClientTransactionWithMetaData, DepositDatawithFee, HistoricRoot, PendingBlock,
        TransferReceipt, TransferReceiptStatus, TxHashBytes, TxLifecycle,
    },
    ports::db::{
        BlockStorageDB, HistoricRootsDB, PendingBlockDB, TransactionsDB, TransferReceiptDB,
        TransferReceiptStoreError,
    },
};
use alloy::primitives::Address;
use ark_bn254::Fr as Fr254;
use ark_ff::{PrimeField, Zero};
use futures::TryStreamExt;
use lib::{
    error::ConversionError, hex_conversion::HexConvertible, nf_client_proof::Proof,
    shared_entities::ClientTransaction,
};
use mongodb::bson::{doc, Bson, Document};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

fn lifecycle_bson(lifecycle: &TxLifecycle) -> Bson {
    mongodb::bson::to_bson(lifecycle).expect("TxLifecycle should serialize to BSON")
}

// Temporary migration support: queries must match both the new explicit
// lifecycle document and the legacy triplet {in_mempool, block_l2,
// cancelled_explicitly} until existing proposer data has been backfilled.
fn legacy_mempool_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "in_mempool": true,
        "cancelled_explicitly": { "$ne": true }
    }
}

fn legacy_selected_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "in_mempool": { "$ne": true },
        "block_l2": { "$exists": true, "$ne": Bson::Null },
        "cancelled_explicitly": { "$ne": true }
    }
}

fn mempool_state_filter() -> Document {
    doc! {
        "$or": [
            doc! { "lifecycle.state": "mempool" },
            legacy_mempool_filter()
        ]
    }
}

fn selected_state_filter() -> Document {
    doc! {
        "$or": [
            doc! { "lifecycle.state": "selected" },
            legacy_selected_filter()
        ]
    }
}

fn selected_or_included_state_filter() -> Document {
    doc! {
        "$or": [
            doc! { "lifecycle.state": { "$in": ["selected", "included"] } },
            legacy_selected_filter()
        ]
    }
}

fn selected_state_filter_for_block(block_l2: i64) -> Document {
    doc! {
        "$or": [
            doc! {
                "lifecycle.state": "selected",
                "lifecycle.block_l2": Bson::Int64(block_l2)
            },
            doc! {
                "lifecycle": { "$exists": false },
                "in_mempool": { "$ne": true },
                "block_l2": Bson::Int64(block_l2),
                "cancelled_explicitly": { "$ne": true }
            }
        ]
    }
}

fn swap_link_filter(swap_link: &Fr254) -> Document {
    doc! {
        "client_transaction.swap_link": swap_link.to_hex_string()
    }
}

fn cancelled_state_filter() -> Document {
    doc! {
        "$or": [
            doc! { "lifecycle.state": "cancelled" },
            doc! {
                "lifecycle": { "$exists": false },
                "cancelled_explicitly": true
            }
        ]
    }
}

pub const DB: &str = "nightfall";
const COLLECTION: &str = "ClientTransactions";
const DEPOSIT_COLLECTION: &str = "Deposits";
pub const PROPOSED_BLOCKS_COLLECTION: &str = "ProposedBlocks";
const TRANSFER_RECEIPTS_COLLECTION: &str = "TransferReceipts";
const PENDING_BLOCKS_COLLECTION: &str = "PendingBlocks";

fn deposit_filter(deposit: &DepositDatawithFee) -> Document {
    doc! {
        "deposit_data.secret_hash": deposit.deposit_data.secret_hash.to_hex_string(),
        "deposit_data.nf_slot_id": deposit.deposit_data.nf_slot_id.to_hex_string(),
    }
}

fn deposit_filters(deposits: &[DepositDatawithFee]) -> Vec<Document> {
    deposits.iter().map(deposit_filter).collect()
}

fn available_deposits_filter() -> Document {
    doc! {
        "$or": [
            { "reserved": false },
            { "reserved": { "$exists": false } }
        ]
    }
}

#[async_trait::async_trait]
impl<'a, P> TransactionsDB<'a, P> for mongodb::Client
where
    P: Proof,
{
    async fn store_transaction(&self, transaction: ClientTransactionWithMetaData<P>) -> Option<()> {
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .insert_one(transaction)
            .await
            .ok()?;
        Some(())
    }

    async fn get_transaction(&self, key: &'a [u32]) -> Option<ClientTransactionWithMetaData<P>> {
        let filter = doc! {"hash": key};
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find_one(filter)
            .await
            .ok()?
    }

    async fn get_all_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> {
        let mut cursor: mongodb::Cursor<ClientTransactionWithMetaData<P>> = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find(doc! {})
            .await
            .ok()?;
        let mut result: Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)> = Vec::new();
        while cursor.advance().await.ok()? {
            let v: ClientTransactionWithMetaData<P> = cursor.deserialize_current().ok()?;
            result.push((v.hash.clone(), v));
        }
        Some(result)
    }

    // add in all the remaining trait items
    async fn get_all_mempool_client_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> {
        let filter = mempool_state_filter();
        let mut cursor: mongodb::Cursor<ClientTransactionWithMetaData<P>> = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find(filter)
            .await
            .ok()?; // propagate DB error as None so the caller can handle it explicitly
        let mut result: Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)> = Vec::new();
        while cursor.advance().await.ok()? {
            let v: ClientTransactionWithMetaData<P> = cursor.deserialize_current().ok()?;
            result.push((v.hash.clone(), v));
        }
        Some(result)
    }

    async fn get_all_selected_client_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> {
        let filter = selected_state_filter();
        let mut cursor: mongodb::Cursor<ClientTransactionWithMetaData<P>> = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find(filter)
            .await
            .ok()?;
        let mut result: Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)> = Vec::new();
        while cursor.advance().await.ok()? {
            let v: ClientTransactionWithMetaData<P> = cursor.deserialize_current().ok()?;
            result.push((v.hash.clone(), v));
        }
        Some(result)
    }

    // Count client_transaction in the mempool
    // This is used to determine if we need to assemble a block
    async fn count_mempool_client_transactions(&self) -> Result<u64, mongodb::error::Error> {
        let filter = mempool_state_filter();
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .count_documents(filter)
            .await
    }

    async fn count_mempool_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error> {
        let mut filter = swap_link_filter(swap_link);
        filter.extend(mempool_state_filter());
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .count_documents(filter)
            .await
    }

    async fn count_selected_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error> {
        let mut filter = swap_link_filter(swap_link);
        filter.extend(selected_or_included_state_filter());
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .count_documents(filter)
            .await
    }

    async fn count_cancelled_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error> {
        let mut filter = swap_link_filter(swap_link);
        filter.extend(cancelled_state_filter());
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .count_documents(filter)
            .await
    }

    async fn cancel_mempool_swap_transactions(&self, swap_link: &Fr254) -> Option<u64> {
        let mut filter = swap_link_filter(swap_link);
        filter.extend(mempool_state_filter());
        let update = doc! {"$set": {
            "lifecycle": lifecycle_bson(&TxLifecycle::Cancelled)
        }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    async fn set_client_transactions_in_mempool_by_hashes(
        &self,
        transaction_hashes: &[Vec<u32>],
        in_mempool: bool,
    ) -> Option<u64> {
        if transaction_hashes.is_empty() {
            return Some(0);
        }

        let lifecycle = if in_mempool {
            TxLifecycle::Mempool
        } else {
            TxLifecycle::Dropped
        };
        let filter = doc! {"hash": { "$in": transaction_hashes }};
        let update = doc! {"$set": {
            "lifecycle": lifecycle_bson(&lifecycle)
        }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    async fn mark_transactions_selected_for_block(
        &self,
        txs: &[ClientTransactionWithMetaData<P>],
        block_l2: u64,
    ) -> Option<u64> {
        let block_l2 = i64::try_from(block_l2).ok()?;
        let k: Vec<_> = txs.iter().map(|t| &t.hash).collect();
        let mut filter = doc! {
            "hash": { "$in": k }
        };
        filter.extend(mempool_state_filter());
        let update = doc! {"$set": {
            "lifecycle": lifecycle_bson(&TxLifecycle::Selected {
                block_l2: u64::try_from(block_l2).ok()?,
            })
        }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    async fn restore_transactions_to_mempool(
        &self,
        txs: &[ClientTransactionWithMetaData<P>],
        block_l2: u64,
    ) -> Option<u64> {
        if txs.is_empty() {
            return Some(0);
        }

        let block_l2 = i64::try_from(block_l2).ok()?;
        let k: Vec<_> = txs.iter().map(|t| &t.hash).collect();
        let mut filter = doc! {
            "hash": { "$in": k }
        };
        filter.extend(selected_state_filter_for_block(block_l2));
        let update = doc! {"$set": {
            "lifecycle": lifecycle_bson(&TxLifecycle::Mempool)
        }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    async fn mark_transactions_included_by_hashes(
        &self,
        transaction_hashes: &[Vec<u32>],
    ) -> Option<u64> {
        if transaction_hashes.is_empty() {
            return Some(0);
        }

        let collection = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION);
        let mut modified = 0u64;

        for hash in transaction_hashes {
            let transaction =
                <mongodb::Client as TransactionsDB<P>>::get_transaction(self, hash).await?;
            let block_l2 = transaction.lifecycle.block_l2()?;
            let block_l2_i64 = i64::try_from(block_l2).ok()?;

            let mut filter = doc! { "hash": hash };
            filter.extend(selected_state_filter_for_block(block_l2_i64));
            let update = doc! {"$set": {
                "lifecycle": lifecycle_bson(&TxLifecycle::Included { block_l2 })
            }};

            let result = collection.update_one(filter, update).await.ok()?;
            modified += result.modified_count;
        }

        Some(modified)
    }

    async fn drop_transactions(&self, txs: &[ClientTransactionWithMetaData<P>]) -> Option<u64> {
        if txs.is_empty() {
            return Some(0);
        }

        let k: Vec<_> = txs.iter().map(|t| &t.hash).collect();
        let mut filter = doc! {
            "hash": { "$in": k }
        };
        filter.extend(mempool_state_filter());
        let update = doc! {"$set": {
            "lifecycle": lifecycle_bson(&TxLifecycle::Dropped)
        }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    async fn find_transaction(
        &self,
        v: &ClientTransaction<P>,
    ) -> Option<ClientTransactionWithMetaData<P>> {
        // we'll compute the hash of the transaction and then look it up in the database
        let hash = v.hash().ok()?;
        let mut filter = doc! {
            "hash": hash
        };
        filter.extend(mempool_state_filter());
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find_one(filter)
            .await
            .expect("Database error") // we can't really proceed at this point
    }

    async fn find_deposit(&self, v: &DepositDatawithFee) -> Option<DepositDatawithFee> {
        let filter = deposit_filter(v);
        self.database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION)
            .find_one(filter)
            .await
            .expect("Database error") // we can't really proceed at this point
    }

    // Store unused deposits in the mempool
    async fn set_mempool_deposits(&self, deposits: Vec<DepositDatawithFee>) -> Option<u64> {
        if deposits.is_empty() {
            return Some(0);
        }

        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);

        // Directly insert Vec<DepositInfo> instead of converting to Document
        let result = collection.insert_many(deposits).await.ok()?;

        Some(result.inserted_ids.len() as u64)
    }

    // Retrieve deposits from the mempool
    async fn get_mempool_deposits(&self) -> Option<Vec<DepositDatawithFee>> {
        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);
        let mut cursor = collection.find(available_deposits_filter()).await.ok()?;

        let mut result: Vec<DepositDatawithFee> = Vec::new();
        while cursor.advance().await.ok()? {
            let deposit: DepositDatawithFee = cursor.deserialize_current().ok()?;
            result.push(deposit);
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    // Count deposits in the mempool
    // This is used to determine if we need to assemble a block
    async fn count_mempool_deposits(&self) -> Result<u64, mongodb::error::Error> {
        self.database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION)
            .count_documents(available_deposits_filter())
            .await
    }

    // Remove used deposits from the mempool
    async fn remove_mempool_deposits(
        &self,
        used_deposits: Vec<Vec<DepositDatawithFee>>,
    ) -> Option<u64> {
        let used_deposits: Vec<DepositDatawithFee> = used_deposits.into_iter().flatten().collect();
        if used_deposits.is_empty() {
            return Some(0);
        }

        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);

        let delete_conditions = deposit_filters(&used_deposits);
        let filter = doc! {
            "$or": delete_conditions
        };
        // Delete matching documents
        let result = collection.delete_many(filter).await.ok()?;
        Some(result.deleted_count)
    }

    async fn set_mempool_deposits_reserved(
        &self,
        deposits: Vec<Vec<DepositDatawithFee>>,
        reserved: bool,
    ) -> Option<u64> {
        let deposits: Vec<DepositDatawithFee> = deposits.into_iter().flatten().collect();
        if deposits.is_empty() {
            return Some(0);
        }

        let filter = doc! {
            "$or": deposit_filters(&deposits)
        };
        let update = doc! {"$set": { "reserved": reserved }};
        let result = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION)
            .update_many(filter, update)
            .await
            .ok()?;
        Some(result.modified_count)
    }

    // Remove all deposits from the mempool
    async fn remove_all_mempool_deposits(&self) -> Option<u64> {
        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);

        let result = collection.delete_many(doc! {}).await.ok()?;
        Some(result.deleted_count)
    }
    async fn remove_all_mempool_client_transactions(&self) -> Option<u64> {
        let collection = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION);

        let result = collection.delete_many(mempool_state_filter()).await.ok()?;
        Some(result.deleted_count)
    }
}

#[async_trait::async_trait]
impl HistoricRootsDB for mongodb::Client {
    async fn store_historic_root(&mut self, historic_root: &HistoricRoot) -> Option<()> {
        let historic_root_entry = HistoricRootEntry::from(historic_root);
        self.database(DB)
            .collection::<HistoricRootEntry>("historic_roots")
            .insert_one(historic_root_entry)
            .await
            .expect("Database error"); // we can't really proceed at this point
        Some(())
    }
    async fn get_historic_root(&mut self, historic_root_hash: &Fr254) -> Option<HistoricRoot> {
        let filter = doc! {"historic_root_hash": historic_root_hash.to_string()};
        let historic_root = self
            .database(DB)
            .collection::<HistoricRootEntry>("historic_roots")
            .find_one(filter)
            .await
            .expect("Database error"); // we can't really proceed at this point
        historic_root.map(|historic_root| {
            historic_root
                .try_into()
                .expect("Conversion should always succeed")
        })
    }
}

// we need to store a slightly different struct because we can't easily turn
// HistoricRoot into a bson object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct HistoricRootEntry {
    historic_root_hash: String,
    index: u32,
}

impl From<&HistoricRoot> for HistoricRootEntry {
    fn from(historic_root: &HistoricRoot) -> Self {
        Self {
            historic_root_hash: historic_root.0.to_string(),
            index: historic_root.1,
        }
    }
}

impl TryFrom<HistoricRootEntry> for HistoricRoot {
    type Error = ConversionError;

    fn try_from(historic_root_entry: HistoricRootEntry) -> Result<Self, Self::Error> {
        // a value of Fr254::zero() gets converted to an empty string, rather than "0"
        // this then fails to parse, so we need to handle this case
        // ...

        if historic_root_entry.historic_root_hash.is_empty() {
            return Ok(Self(Fr254::zero(), 0));
        }
        Ok(Self(
            historic_root_entry
                .historic_root_hash
                .parse::<Fr254>()
                .map_err(|_| ConversionError::ParseFailed)?,
            historic_root_entry.index,
        ))
    }
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

/// Creates unique indexes on `receipt_id` and `tx_hash` in the TransferReceipts collection.
/// Must be called once at proposer startup.
pub async fn ensure_transfer_receipt_indexes(
    client: &mongodb::Client,
) -> Result<(), mongodb::error::Error> {
    use mongodb::options::IndexOptions;
    use mongodb::IndexModel;

    let collection = client
        .database(DB)
        .collection::<TransferReceipt>(TRANSFER_RECEIPTS_COLLECTION);

    let receipt_id_index = IndexModel::builder()
        .keys(doc! { "receipt_id": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    let tx_hash_hex_index = IndexModel::builder()
        .keys(doc! { "tx_hash": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(receipt_id_index).await?;
    collection.create_index(tx_hash_hex_index).await?;
    Ok(())
}

#[async_trait::async_trait]
impl TransferReceiptDB for mongodb::Client {
    async fn store_transfer_receipt(
        &self,
        receipt: TransferReceipt,
    ) -> Result<(), TransferReceiptStoreError> {
        let result = self
            .database(DB)
            .collection::<TransferReceipt>(TRANSFER_RECEIPTS_COLLECTION)
            .insert_one(receipt)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                // MongoDB duplicate key error code is 11000.
                let is_dup = matches!(
                    *e.kind,
                    mongodb::error::ErrorKind::Write(
                        mongodb::error::WriteFailure::WriteError(ref we)
                    ) if we.code == 11000
                );
                if is_dup {
                    Err(TransferReceiptStoreError::DuplicateKey)
                } else {
                    Err(TransferReceiptStoreError::Other(e.to_string()))
                }
            }
        }
    }

    async fn get_transfer_receipt(&self, receipt_id: &str) -> Option<TransferReceipt> {
        let filter = doc! { "receipt_id": receipt_id };
        match self
            .database(DB)
            .collection::<TransferReceipt>(TRANSFER_RECEIPTS_COLLECTION)
            .find_one(filter)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                log::warn!("Failed to query transfer receipt by id={receipt_id}: {e}");
                None
            }
        }
    }

    async fn get_transfer_receipt_by_tx_hash(
        &self,
        tx_hash: &TxHashBytes,
    ) -> Option<TransferReceipt> {
        let filter = doc! { "tx_hash": tx_hash.as_hex() };
        match self
            .database(DB)
            .collection::<TransferReceipt>(TRANSFER_RECEIPTS_COLLECTION)
            .find_one(filter)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                log::warn!(
                    "Failed to query transfer receipt by tx_hash={}: {e}",
                    tx_hash.as_hex()
                );
                None
            }
        }
    }

    async fn set_transfer_receipt_status(
        &self,
        receipt_id: &str,
        status: TransferReceiptStatus,
        updated_at_unix: i64,
    ) -> Option<()> {
        let filter = doc! { "receipt_id": receipt_id };
        let status_bson = match mongodb::bson::to_bson(&status) {
            Ok(b) => b,
            Err(e) => {
                log::warn!("Failed to serialize receipt status for id={receipt_id}: {e}");
                return None;
            }
        };
        let update = doc! {
            "$set": {
                "status": status_bson,
                "updated_at_unix": updated_at_unix,
            }
        };
        match self
            .database(DB)
            .collection::<TransferReceipt>(TRANSFER_RECEIPTS_COLLECTION)
            .update_one(filter, update)
            .await
        {
            Ok(_) => Some(()),
            Err(e) => {
                log::warn!("Failed to update transfer receipt status for id={receipt_id}: {e}");
                None
            }
        }
    }
}

#[async_trait::async_trait]
impl PendingBlockDB for mongodb::Client {
    async fn store_pending_block(&self, pending_block: &PendingBlock) -> Option<()> {
        let filter = doc! { "layer2_block_number": pending_block.layer2_block_number as i64 };
        self.database(DB)
            .collection::<PendingBlock>(PENDING_BLOCKS_COLLECTION)
            .replace_one(filter, pending_block)
            .upsert(true)
            .await
            .ok()?;
        Some(())
    }

    async fn get_pending_block(&self, block_number: u64) -> Option<PendingBlock> {
        let filter = doc! { "layer2_block_number": block_number as i64 };
        self.database(DB)
            .collection::<PendingBlock>(PENDING_BLOCKS_COLLECTION)
            .find_one(filter)
            .await
            .ok()?
    }

    async fn get_all_pending_blocks(&self) -> Option<Vec<PendingBlock>> {
        let cursor = self
            .database(DB)
            .collection::<PendingBlock>(PENDING_BLOCKS_COLLECTION)
            .find(doc! {})
            .await
            .ok()?;
        cursor.try_collect().await.ok()
    }

    async fn delete_pending_block(&self, block_number: u64) -> Option<()> {
        let filter = doc! { "layer2_block_number": block_number as i64 };
        self.database(DB)
            .collection::<PendingBlock>(PENDING_BLOCKS_COLLECTION)
            .delete_one(filter)
            .await
            .ok()?;
        Some(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_std::UniformRand;

    #[test]
    fn mempool_filter_matches_new_and_legacy_shapes() {
        let filter = mempool_state_filter();
        let branches = filter
            .get_array("$or")
            .expect("mempool filter should contain transitional branches");

        assert_eq!(branches.len(), 2);
        assert_eq!(
            branches[0]
                .as_document()
                .and_then(|doc| doc.get_str("lifecycle.state").ok()),
            Some("mempool")
        );
        let legacy = branches[1]
            .as_document()
            .expect("legacy mempool branch should be a document");
        assert_eq!(legacy.get_bool("in_mempool"), Ok(true));
        assert_eq!(
            legacy
                .get_document("lifecycle")
                .and_then(|doc| doc.get_bool("$exists")),
            Ok(false)
        );
    }

    #[test]
    fn selected_filter_matches_new_and_legacy_shapes() {
        let filter = selected_state_filter();
        let branches = filter
            .get_array("$or")
            .expect("selected filter should contain transitional branches");

        assert_eq!(branches.len(), 2);
        assert_eq!(
            branches[0]
                .as_document()
                .and_then(|doc| doc.get_str("lifecycle.state").ok()),
            Some("selected")
        );
        let legacy = branches[1]
            .as_document()
            .expect("legacy selected branch should be a document");
        assert_eq!(
            legacy
                .get_document("lifecycle")
                .and_then(|doc| doc.get_bool("$exists")),
            Ok(false)
        );
        assert!(legacy.get("block_l2").is_some());
    }

    #[test]
    fn selected_or_included_filter_matches_active_and_finalized_shapes() {
        let filter = selected_or_included_state_filter();
        let branches = filter
            .get_array("$or")
            .expect("selected-or-included filter should contain transitional branches");

        assert_eq!(branches.len(), 2);
        let lifecycle_state = branches[0]
            .as_document()
            .and_then(|doc| doc.get_document("lifecycle.state").ok())
            .expect("new lifecycle branch should match multiple states");
        assert_eq!(
            lifecycle_state.get_array("$in").expect("$in states").len(),
            2
        );
    }

    #[test]
    fn test_historic_root_type_conversion() {
        let rng = &mut ark_std::test_rng();
        let historic_root = HistoricRoot(Fr254::rand(rng), u32::rand(rng));
        let historic_root_entry = HistoricRootEntry::from(&historic_root);
        let historic_root_2 = HistoricRoot::try_from(historic_root_entry).unwrap();
        assert_eq!(historic_root, historic_root_2);
    }
}
