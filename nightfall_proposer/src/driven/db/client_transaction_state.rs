use crate::{
    domain::entities::{ClientTransactionWithMetaData, TxLifecycle},
    driven::db::mongo_db::{StoredBlock, DB},
    ports::db::BlockStorageDB,
};
use ark_ff::Zero;
use lib::hex_conversion::HexConvertible;
use mongodb::{
    bson::{doc, Bson, Document},
    Client,
};
use std::collections::{HashMap, HashSet};

pub const CLIENT_TRANSACTIONS_COLLECTION: &str = "ClientTransactions";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LegacyClientTransactionLifecycleMigrationStats {
    pub mempool_backfilled: u64,
    pub selected_backfilled: u64,
    pub included_backfilled: u64,
    pub cancelled_backfilled: u64,
    pub dropped_backfilled: u64,
}

impl LegacyClientTransactionLifecycleMigrationStats {
    pub fn total_backfilled(&self) -> u64 {
        self.mempool_backfilled
            + self.selected_backfilled
            + self.included_backfilled
            + self.cancelled_backfilled
            + self.dropped_backfilled
    }
}

pub(crate) fn lifecycle_bson(lifecycle: &TxLifecycle) -> Bson {
    mongodb::bson::to_bson(lifecycle).expect("TxLifecycle should serialize to BSON")
}

pub(crate) fn mempool_transactions_filter() -> Document {
    doc! { "lifecycle.state": "mempool" }
}

pub(crate) fn selected_transactions_filter() -> Document {
    doc! { "lifecycle.state": "selected" }
}

pub(crate) fn selected_or_included_transactions_filter() -> Document {
    doc! { "lifecycle.state": { "$in": ["selected", "included"] } }
}

pub(crate) fn selected_transactions_filter_for_block(block_l2: i64) -> Document {
    doc! {
        "lifecycle.state": "selected",
        "lifecycle.block_l2": Bson::Int64(block_l2),
    }
}

pub(crate) fn selected_transactions_filter_after_block(block_l2: i64) -> Document {
    doc! {
        "lifecycle.state": "selected",
        "lifecycle.block_l2": { "$gt": Bson::Int64(block_l2) },
    }
}

#[cfg(test)]
fn legacy_transactions_filter() -> Document {
    doc! { "lifecycle": { "$exists": false } }
}

fn legacy_mempool_transactions_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "in_mempool": true,
        "cancelled_explicitly": { "$ne": true }
    }
}

fn legacy_cancelled_transactions_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "cancelled_explicitly": true
    }
}

fn legacy_selected_or_included_transactions_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "in_mempool": { "$ne": true },
        "block_l2": { "$exists": true, "$ne": Bson::Null },
        "cancelled_explicitly": { "$ne": true }
    }
}

fn legacy_dropped_transactions_filter() -> Document {
    doc! {
        "lifecycle": { "$exists": false },
        "in_mempool": { "$ne": true },
        "cancelled_explicitly": { "$ne": true },
        "$or": [
            doc! { "block_l2": { "$exists": false } },
            doc! { "block_l2": Bson::Null }
        ]
    }
}

fn stored_block_commitments_map(stored_blocks: Vec<StoredBlock>) -> HashMap<u64, HashSet<String>> {
    stored_blocks
        .into_iter()
        .map(|block| {
            (
                block.layer2_block_number,
                block.commitments.into_iter().collect::<HashSet<_>>(),
            )
        })
        .collect()
}

fn transaction_commitments_hex<P>(transaction: &ClientTransactionWithMetaData<P>) -> Vec<String> {
    transaction
        .client_transaction
        .commitments
        .iter()
        .filter(|commitment| !commitment.is_zero())
        .map(|commitment| commitment.to_hex_string())
        .collect()
}

fn classify_legacy_active_transaction<P>(
    transaction: &ClientTransactionWithMetaData<P>,
    stored_blocks: &HashMap<u64, HashSet<String>>,
) -> Result<TxLifecycle, String> {
    let block_l2 = transaction.lifecycle.block_l2().ok_or_else(|| {
        format!(
            "Legacy client transaction {:?} is missing block_l2 during lifecycle backfill",
            transaction.hash
        )
    })?;

    let lifecycle = match stored_blocks.get(&block_l2) {
        Some(block_commitments)
            if transaction_commitments_hex(transaction)
                .into_iter()
                .all(|commitment| block_commitments.contains(&commitment)) =>
        {
            TxLifecycle::Included { block_l2 }
        }
        _ => TxLifecycle::Selected { block_l2 },
    };

    Ok(lifecycle)
}

async fn backfill_legacy_transactions_by_filter(
    client: &Client,
    filter: Document,
    lifecycle: TxLifecycle,
) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .update_many(
            filter,
            doc! {
                "$set": { "lifecycle": lifecycle_bson(&lifecycle) }
            },
        )
        .await
        .map(|result| result.modified_count)
        .map_err(|error| {
            format!("Could not backfill proposer client transaction lifecycle: {error}")
        })
}

pub(crate) async fn backfill_legacy_client_transaction_lifecycle(
    client: &Client,
) -> Result<LegacyClientTransactionLifecycleMigrationStats, String> {
    let mempool_backfilled = backfill_legacy_transactions_by_filter(
        client,
        legacy_mempool_transactions_filter(),
        TxLifecycle::Mempool,
    )
    .await?;
    let cancelled_backfilled = backfill_legacy_transactions_by_filter(
        client,
        legacy_cancelled_transactions_filter(),
        TxLifecycle::Cancelled,
    )
    .await?;
    let dropped_backfilled = backfill_legacy_transactions_by_filter(
        client,
        legacy_dropped_transactions_filter(),
        TxLifecycle::Dropped,
    )
    .await?;

    let active_collection = client
        .database(DB)
        .collection::<ClientTransactionWithMetaData<Bson>>(CLIENT_TRANSACTIONS_COLLECTION);
    let mut cursor = active_collection
        .find(legacy_selected_or_included_transactions_filter())
        .await
        .map_err(|error| {
            format!(
                "Could not query legacy proposer client transactions for lifecycle backfill: {error}"
            )
        })?;

    let stored_blocks = stored_block_commitments_map(
        <Client as BlockStorageDB>::get_all_blocks(client)
            .await
            .ok_or_else(|| {
                "Could not read proposer StoredBlocks during lifecycle backfill".to_string()
            })?,
    );
    let raw_collection = client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION);

    let mut selected_backfilled = 0_u64;
    let mut included_backfilled = 0_u64;

    while cursor.advance().await.map_err(|error| {
        format!(
            "Could not iterate legacy proposer client transactions for lifecycle backfill: {error}"
        )
    })? {
        let transaction = cursor.deserialize_current().map_err(|error| {
            format!(
                "Could not deserialize legacy proposer client transaction for lifecycle backfill: {error}"
            )
        })?;

        let lifecycle = classify_legacy_active_transaction(&transaction, &stored_blocks)?;
        let result = raw_collection
            .update_one(
                doc! {
                    "hash": &transaction.hash,
                    "lifecycle": { "$exists": false }
                },
                doc! { "$set": { "lifecycle": lifecycle_bson(&lifecycle) } },
            )
            .await
            .map_err(|error| {
                format!(
                    "Could not persist lifecycle backfill for proposer client transaction {:?}: {error}",
                    transaction.hash
                )
            })?;

        if result.modified_count != 1 {
            return Err(format!(
                "Lifecycle backfill expected to update exactly one proposer client transaction for hash {:?}, updated {} instead",
                transaction.hash, result.modified_count
            ));
        }

        match lifecycle {
            TxLifecycle::Selected { .. } => selected_backfilled += 1,
            TxLifecycle::Included { .. } => included_backfilled += 1,
            _ => {}
        }
    }

    Ok(LegacyClientTransactionLifecycleMigrationStats {
        mempool_backfilled,
        selected_backfilled,
        included_backfilled,
        cancelled_backfilled,
        dropped_backfilled,
    })
}

pub(crate) async fn selected_client_transaction_count(client: &Client) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .count_documents(selected_transactions_filter())
        .await
        .map_err(|error| {
            format!("Could not inspect proposer selected client transactions: {error}")
        })
}

pub(crate) async fn selected_client_transaction_count_after_block(
    client: &Client,
    last_applied_l2_block: u64,
) -> Result<u64, String> {
    let last_applied_l2_block = i64::try_from(last_applied_l2_block).map_err(|_| {
        format!("Could not convert proposer last applied L2 block {last_applied_l2_block} into i64")
    })?;

    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .count_documents(selected_transactions_filter_after_block(last_applied_l2_block))
        .await
        .map_err(|error| {
            format!(
                "Could not inspect proposer selected client transactions beyond L2 block {last_applied_l2_block}: {error}"
            )
        })
}

pub(crate) async fn restore_all_selected_transactions_to_mempool(
    client: &Client,
) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .update_many(
            selected_transactions_filter(),
            doc! {
                "$set": {
                    "lifecycle": lifecycle_bson(&TxLifecycle::Mempool)
                }
            },
        )
        .await
        .map(|result| result.modified_count)
        .map_err(|error| {
            format!(
                "Could not restore proposer selected client transactions to the mempool: {error}"
            )
        })
}

pub(crate) async fn restore_selected_transactions_to_mempool_after_block(
    client: &Client,
    last_applied_l2_block: u64,
) -> Result<u64, String> {
    let last_applied_l2_block = i64::try_from(last_applied_l2_block).map_err(|_| {
        format!("Could not convert proposer last applied L2 block {last_applied_l2_block} into i64")
    })?;

    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .update_many(
            selected_transactions_filter_after_block(last_applied_l2_block),
            doc! {
                "$set": {
                    "lifecycle": lifecycle_bson(&TxLifecycle::Mempool)
                }
            },
        )
        .await
        .map(|result| result.modified_count)
        .map_err(|error| {
            format!(
                "Could not restore proposer selected client transactions beyond L2 block {last_applied_l2_block} to the mempool: {error}"
            )
        })
}

pub(crate) async fn remove_all_mempool_client_transactions(client: &Client) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION)
        .delete_many(mempool_transactions_filter())
        .await
        .map(|result| result.deleted_count)
        .map_err(|error| format!("Could not discard proposer mempool client transactions: {error}"))
}

#[cfg(test)]
pub(crate) fn count_legacy_client_transactions_filter() -> Document {
    legacy_transactions_filter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::TxLifecycle,
        ports::db::{BlockStorageDB, TransactionsDB},
    };
    use alloy::primitives::Address;
    use ark_bn254::Fr as Fr254;
    use ark_serialize::SerializationError;
    use lib::{
        nf_client_proof::Proof,
        shared_entities::{ClientTransaction, CompressedSecrets},
        tests_utils::{get_db_connection, get_mongo},
    };
    use mongodb::bson::doc;
    use serde::{Deserialize, Serialize};
    use tokio::sync::{Mutex, OnceCell};

    async fn client_transaction_state_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) })
            .await
            .lock()
            .await
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    struct MockProof;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<alloy::primitives::Bytes, SerializationError> {
            Ok(alloy::primitives::Bytes::new())
        }

        fn from_compressed(
            _compressed: alloy::primitives::Bytes,
        ) -> Result<Self, SerializationError> {
            Ok(Self)
        }
    }

    fn test_transaction(
        seed: u32,
        lifecycle: TxLifecycle,
        commitment: Fr254,
    ) -> ClientTransactionWithMetaData<MockProof> {
        ClientTransactionWithMetaData {
            client_transaction: ClientTransaction {
                commitments: [commitment, Fr254::zero(), Fr254::zero(), Fr254::zero()],
                compressed_secrets: CompressedSecrets::default(),
                proof: MockProof,
                ..Default::default()
            },
            lifecycle,
            hash: vec![seed, seed + 1, seed + 2],
            historic_roots: vec![],
            receipt_token: None,
        }
    }

    fn legacy_document_from_transaction(
        transaction: &ClientTransactionWithMetaData<MockProof>,
        legacy_kind: &str,
    ) -> Document {
        let mut document =
            mongodb::bson::to_document(transaction).expect("serialize client transaction");
        document.remove("lifecycle");

        match legacy_kind {
            "mempool" => {
                document.insert("in_mempool", true);
                document.insert("cancelled_explicitly", false);
            }
            "cancelled" => {
                document.insert("in_mempool", false);
                document.insert("cancelled_explicitly", true);
            }
            "selected_or_included" => {
                document.insert("in_mempool", false);
                document.insert("cancelled_explicitly", false);
                document.insert(
                    "block_l2",
                    Bson::Int64(
                        i64::try_from(transaction.lifecycle.block_l2().expect("block_l2"))
                            .expect("block_l2 fits i64"),
                    ),
                );
            }
            "dropped" => {
                document.insert("in_mempool", false);
                document.insert("cancelled_explicitly", false);
            }
            other => panic!("unexpected legacy kind {other}"),
        }

        document
    }

    #[tokio::test]
    async fn selected_filter_excludes_included_new_and_legacy_shapes() {
        let _lock = client_transaction_state_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let collection = client
            .database(DB)
            .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION);
        collection
            .insert_many(vec![
                doc! {
                    "hash": [1_i32, 2_i32, 3_i32],
                    "lifecycle": { "state": "selected", "block_l2": 7_i64 }
                },
                doc! {
                    "hash": [4_i32, 5_i32, 6_i32],
                    "lifecycle": { "state": "included", "block_l2": 7_i64 }
                },
                doc! {
                    "hash": [7_i32, 8_i32, 9_i32],
                    "in_mempool": false,
                    "block_l2": 7_i64,
                    "cancelled_explicitly": false
                },
            ])
            .await
            .expect("insert lifecycle filter fixtures");

        let matched = collection
            .count_documents(selected_transactions_filter())
            .await
            .expect("count selected filter matches");
        assert_eq!(
            matched, 1,
            "selected filter should only match the explicit Selected lifecycle state"
        );
    }

    #[tokio::test]
    async fn legacy_client_transaction_lifecycle_backfill_is_idempotent() {
        let _lock = client_transaction_state_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let selected_transaction = test_transaction(
            10,
            TxLifecycle::Selected { block_l2: 30 },
            Fr254::from(310_u64),
        );
        let included_transaction = test_transaction(
            20,
            TxLifecycle::Selected { block_l2: 31 },
            Fr254::from(311_u64),
        );
        let mempool_transaction = test_transaction(30, TxLifecycle::Mempool, Fr254::from(312_u64));
        let cancelled_transaction =
            test_transaction(40, TxLifecycle::Cancelled, Fr254::from(313_u64));
        let dropped_transaction = test_transaction(50, TxLifecycle::Dropped, Fr254::from(314_u64));

        let collection = client
            .database(DB)
            .collection::<Document>(CLIENT_TRANSACTIONS_COLLECTION);
        collection
            .insert_many(vec![
                legacy_document_from_transaction(&selected_transaction, "selected_or_included"),
                legacy_document_from_transaction(&included_transaction, "selected_or_included"),
                legacy_document_from_transaction(&mempool_transaction, "mempool"),
                legacy_document_from_transaction(&cancelled_transaction, "cancelled"),
                legacy_document_from_transaction(&dropped_transaction, "dropped"),
            ])
            .await
            .expect("insert legacy lifecycle fixtures");

        client
            .store_block(&StoredBlock {
                layer2_block_number: 31,
                commitments: transaction_commitments_hex(&included_transaction),
                proposer_address: Address::from([31_u8; 20]),
            })
            .await
            .expect("store included block fixture");

        let first_stats = backfill_legacy_client_transaction_lifecycle(&client)
            .await
            .expect("first lifecycle backfill should succeed");
        assert_eq!(
            first_stats,
            LegacyClientTransactionLifecycleMigrationStats {
                mempool_backfilled: 1,
                selected_backfilled: 1,
                included_backfilled: 1,
                cancelled_backfilled: 1,
                dropped_backfilled: 1,
            }
        );
        assert_eq!(
            collection
                .count_documents(count_legacy_client_transactions_filter())
                .await
                .expect("count legacy rows after first migration"),
            0
        );

        assert_eq!(
            <Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash
            )
            .await
            .expect("read selected transaction after first migration")
            .lifecycle,
            TxLifecycle::Selected { block_l2: 30 }
        );
        assert_eq!(
            <Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &included_transaction.hash
            )
            .await
            .expect("read included transaction after first migration")
            .lifecycle,
            TxLifecycle::Included { block_l2: 31 }
        );
        assert!(<Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &mempool_transaction.hash
        )
        .await
        .expect("read mempool transaction after first migration")
        .lifecycle
        .is_mempool());
        assert!(<Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &cancelled_transaction.hash
        )
        .await
        .expect("read cancelled transaction after first migration")
        .lifecycle
        .is_cancelled());
        assert_eq!(
            <Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &dropped_transaction.hash
            )
            .await
            .expect("read dropped transaction after first migration")
            .lifecycle,
            TxLifecycle::Dropped
        );

        let second_stats = backfill_legacy_client_transaction_lifecycle(&client)
            .await
            .expect("second lifecycle backfill should be a no-op");
        assert_eq!(
            second_stats,
            LegacyClientTransactionLifecycleMigrationStats::default()
        );
        assert_eq!(
            collection
                .count_documents(count_legacy_client_transactions_filter())
                .await
                .expect("count legacy rows after second migration"),
            0
        );
    }
}
