use crate::{
    domain::entities::{Block, ClientTransactionWithMetaData},
    driven::db::mongo_db::StoredBlock,
    ports::db::{BlockStorageDB, TransactionsDB},
};
use ark_ff::Zero;
use lib::{hex_conversion::HexConvertible, nf_client_proof::Proof};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTransactionState {
    InFlight,
    Included,
    Orphaned,
}

#[derive(Debug, Clone)]
pub struct ClassifiedSelectedTransaction<P> {
    pub transaction: ClientTransactionWithMetaData<P>,
    pub state: SelectedTransactionState,
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

fn block_contains_transaction<P>(
    block_commitments: &HashSet<String>,
    transaction: &ClientTransactionWithMetaData<P>,
) -> bool {
    transaction_commitments_hex(transaction)
        .into_iter()
        .all(|commitment| block_commitments.contains(&commitment))
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

fn classify_selected_transaction<P>(
    transaction: ClientTransactionWithMetaData<P>,
    stored_blocks: &HashMap<u64, HashSet<String>>,
    current_layer2_block_number: u64,
    allow_missing_block_inference: bool,
) -> ClassifiedSelectedTransaction<P> {
    let block_l2 = transaction.lifecycle.block_l2().unwrap_or_default();
    let state = match stored_blocks.get(&block_l2) {
        Some(block_commitments) if block_contains_transaction(block_commitments, &transaction) => {
            SelectedTransactionState::Included
        }
        Some(_) => SelectedTransactionState::Orphaned,
        None if allow_missing_block_inference && block_l2 < current_layer2_block_number => {
            SelectedTransactionState::Orphaned
        }
        _ => SelectedTransactionState::InFlight,
    };

    ClassifiedSelectedTransaction { transaction, state }
}

pub async fn get_classified_selected_transactions<P>(
    db: &mongodb::Client,
    current_layer2_block_number: u64,
    allow_missing_block_inference: bool,
) -> Option<Vec<ClassifiedSelectedTransaction<P>>>
where
    P: Proof,
{
    let selected_transactions =
        <mongodb::Client as TransactionsDB<P>>::get_all_selected_client_transactions(db).await?;
    let stored_blocks = stored_block_commitments_map(
        <mongodb::Client as BlockStorageDB>::get_all_blocks(db).await?,
    );

    Some(
        selected_transactions
            .into_iter()
            .map(|(_, transaction)| {
                classify_selected_transaction(
                    transaction,
                    &stored_blocks,
                    current_layer2_block_number,
                    allow_missing_block_inference,
                )
            })
            .collect(),
    )
}

async fn restore_selected_transactions_grouped_by_block<P>(
    db: &mongodb::Client,
    transactions: Vec<ClientTransactionWithMetaData<P>>,
) -> Option<u64>
where
    P: Proof,
{
    let mut transactions_by_block = HashMap::<u64, Vec<ClientTransactionWithMetaData<P>>>::new();
    for transaction in transactions {
        let block_l2 = transaction.lifecycle.block_l2()?;
        transactions_by_block
            .entry(block_l2)
            .or_default()
            .push(transaction);
    }

    let mut restored = 0u64;
    for (block_l2, transactions) in transactions_by_block {
        let modified = <mongodb::Client as TransactionsDB<P>>::restore_transactions_to_mempool(
            db,
            &transactions,
            block_l2,
        )
        .await?;
        if modified != transactions.len() as u64 {
            return None;
        }
        restored += modified;
    }

    Some(restored)
}

pub async fn reconcile_orphaned_selected_transactions<P>(
    db: &mongodb::Client,
    current_layer2_block_number: u64,
) -> Option<u64>
where
    P: Proof,
{
    let classified_transactions =
        get_classified_selected_transactions::<P>(db, current_layer2_block_number, true).await?;
    let orphaned_transactions = classified_transactions
        .into_iter()
        .filter(|classified| classified.state == SelectedTransactionState::Orphaned)
        .map(|classified| classified.transaction)
        .collect::<Vec<_>>();

    restore_selected_transactions_grouped_by_block(db, orphaned_transactions).await
}

pub async fn reconcile_obviously_orphaned_selected_transactions<P>(
    db: &mongodb::Client,
) -> Option<u64>
where
    P: Proof,
{
    let classified_transactions = get_classified_selected_transactions::<P>(db, 0, false).await?;
    let orphaned_transactions = classified_transactions
        .into_iter()
        .filter(|classified| classified.state == SelectedTransactionState::Orphaned)
        .map(|classified| classified.transaction)
        .collect::<Vec<_>>();

    restore_selected_transactions_grouped_by_block(db, orphaned_transactions).await
}

pub async fn restore_selected_transactions_for_failed_block<P>(
    db: &mongodb::Client,
    block: &Block,
) -> Option<u64>
where
    P: Proof,
{
    let block_commitments = block
        .transactions
        .iter()
        .flat_map(|transaction| transaction.commitments.iter())
        .filter(|commitment| !commitment.is_zero())
        .map(|commitment| commitment.to_hex_string())
        .collect::<HashSet<_>>();
    let selected_transactions =
        <mongodb::Client as TransactionsDB<P>>::get_all_selected_client_transactions(db)
            .await
            .unwrap_or_default();
    let matching_transactions = selected_transactions
        .into_iter()
        .map(|(_, transaction)| transaction)
        .filter(|transaction| block_contains_transaction(&block_commitments, transaction))
        .collect::<Vec<_>>();

    restore_selected_transactions_grouped_by_block(db, matching_transactions).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::TxLifecycle,
        driven::db::mongo_db::StoredBlock,
        ports::db::{BlockStorageDB, TransactionsDB},
    };
    use alloy::primitives::Address;
    use alloy::primitives::Bytes;
    use ark_bn254::Fr as Fr254;
    use ark_serialize::SerializationError;
    use lib::{
        nf_client_proof::Proof,
        shared_entities::{ClientTransaction, CompressedSecrets, OnChainTransaction},
        tests_utils::{get_db_connection, get_mongo},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq, Clone)]
    struct MockProof {
        a: Vec<u8>,
        b: Vec<u8>,
        c: Vec<u8>,
    }

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::from_static(b"mock-proof"))
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self {
                a: vec![1],
                b: vec![2],
                c: vec![3],
            })
        }
    }

    fn sample_selected_transaction(
        commitment: Fr254,
        block_l2: u64,
    ) -> ClientTransactionWithMetaData<MockProof> {
        ClientTransactionWithMetaData {
            client_transaction: ClientTransaction {
                commitments: [commitment, Fr254::zero(), Fr254::zero(), Fr254::zero()],
                compressed_secrets: CompressedSecrets::default(),
                proof: MockProof {
                    a: vec![1],
                    b: vec![2],
                    c: vec![3],
                },
                ..Default::default()
            },
            lifecycle: TxLifecycle::Selected { block_l2 },
            hash: vec![1, 2, 3],
            historic_roots: vec![],
        }
    }

    fn sample_block(commitment: Fr254) -> Block {
        Block {
            transactions: vec![OnChainTransaction {
                commitments: [commitment, 0u64.into(), 0u64.into(), 0u64.into()],
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn classifies_selected_transaction_as_included_when_block_contains_commitments() {
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);
        let stored_blocks =
            HashMap::from([(7, HashSet::from([Fr254::from(10u64).to_hex_string()]))]);

        let classified = classify_selected_transaction(tx, &stored_blocks, 8, true);

        assert_eq!(classified.state, SelectedTransactionState::Included);
    }

    #[test]
    fn classifies_selected_transaction_as_in_flight_when_block_is_still_current() {
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);

        let classified = classify_selected_transaction(tx, &HashMap::new(), 7, true);

        assert_eq!(classified.state, SelectedTransactionState::InFlight);
    }

    #[test]
    fn classifies_selected_transaction_as_orphaned_when_chain_has_advanced_without_match() {
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);

        let classified = classify_selected_transaction(tx, &HashMap::new(), 8, true);

        assert_eq!(classified.state, SelectedTransactionState::Orphaned);
    }

    #[test]
    fn does_not_classify_missing_local_block_as_orphaned_when_missing_block_inference_is_disabled()
    {
        let tx = sample_selected_transaction(Fr254::from(10u64), 30);

        let classified = classify_selected_transaction(tx, &HashMap::new(), 100, false);

        assert_eq!(classified.state, SelectedTransactionState::InFlight);
    }

    #[test]
    fn matches_failed_block_transactions_by_commitments() {
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);
        let block = sample_block(Fr254::from(10u64));
        let block_commitments = block
            .transactions
            .iter()
            .flat_map(|transaction| transaction.commitments.iter())
            .filter(|commitment| !commitment.is_zero())
            .map(|commitment| commitment.to_hex_string())
            .collect::<HashSet<_>>();

        assert!(block_contains_transaction(&block_commitments, &tx));
    }

    #[tokio::test]
    async fn restores_selected_transactions_for_failed_block() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);
        db.store_transaction(tx.clone()).await.unwrap();

        let restored = restore_selected_transactions_for_failed_block::<MockProof>(
            &db,
            &sample_block(Fr254::from(10u64)),
        )
        .await
        .unwrap();

        let stored: ClientTransactionWithMetaData<MockProof> =
            db.get_transaction(&tx.hash).await.unwrap();
        assert_eq!(restored, 1);
        assert_eq!(stored.lifecycle, TxLifecycle::Mempool);
    }

    #[tokio::test]
    async fn restart_recovery_restores_only_obviously_orphaned_selected_transactions() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx = sample_selected_transaction(Fr254::from(10u64), 30);
        db.store_transaction(tx.clone()).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: 30,
            commitments: vec![Fr254::from(11u64).to_hex_string()],
            proposer_address: Address::ZERO,
        })
        .await
        .unwrap();

        let restored = reconcile_obviously_orphaned_selected_transactions::<MockProof>(&db)
            .await
            .unwrap();

        let stored: ClientTransactionWithMetaData<MockProof> =
            db.get_transaction(&tx.hash).await.unwrap();
        assert_eq!(restored, 1);
        assert_eq!(stored.lifecycle, TxLifecycle::Mempool);
    }

    #[tokio::test]
    async fn restart_recovery_does_not_restore_when_local_block_is_missing() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx = sample_selected_transaction(Fr254::from(10u64), 30);
        db.store_transaction(tx.clone()).await.unwrap();

        let restored = reconcile_obviously_orphaned_selected_transactions::<MockProof>(&db)
            .await
            .unwrap();

        let stored: ClientTransactionWithMetaData<MockProof> =
            db.get_transaction(&tx.hash).await.unwrap();
        assert_eq!(restored, 0);
        assert_eq!(stored.lifecycle, TxLifecycle::Selected { block_l2: 30 });
    }

    #[tokio::test]
    async fn reorg_reconciliation_restores_selected_transaction_after_block_deletion() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx = sample_selected_transaction(Fr254::from(10u64), 7);
        db.store_transaction(tx.clone()).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: 7,
            commitments: vec![Fr254::from(10u64).to_hex_string()],
            proposer_address: Address::ZERO,
        })
        .await
        .unwrap();

        db.delete_block_by_number(7).await.unwrap();
        let restored = reconcile_orphaned_selected_transactions::<MockProof>(&db, 8)
            .await
            .unwrap();

        let stored: ClientTransactionWithMetaData<MockProof> =
            db.get_transaction(&tx.hash).await.unwrap();
        assert_eq!(restored, 1);
        assert_eq!(stored.lifecycle, TxLifecycle::Mempool);
    }

    #[tokio::test]
    async fn reconcile_does_not_restore_explicitly_cancelled_transactions() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx = ClientTransactionWithMetaData {
            client_transaction: ClientTransaction {
                commitments: [
                    Fr254::from(10u64),
                    Fr254::zero(),
                    Fr254::zero(),
                    Fr254::zero(),
                ],
                compressed_secrets: CompressedSecrets::default(),
                proof: MockProof {
                    a: vec![1],
                    b: vec![2],
                    c: vec![3],
                },
                ..Default::default()
            },
            lifecycle: TxLifecycle::Cancelled,
            hash: vec![9, 9, 9],
            historic_roots: vec![],
        };
        db.store_transaction(tx.clone()).await.unwrap();

        let restored = reconcile_orphaned_selected_transactions::<MockProof>(&db, 8)
            .await
            .unwrap();
        let stored: ClientTransactionWithMetaData<MockProof> =
            db.get_transaction(&tx.hash).await.unwrap();

        assert_eq!(restored, 0);
        assert_eq!(stored.lifecycle, TxLifecycle::Cancelled);
    }
}
