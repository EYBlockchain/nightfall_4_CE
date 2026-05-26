use super::*;
use crate::{
    domain::entities::{
        Block, ClientTransactionWithMetaData, DepositDatawithFee, L1Ref, PendingBlock,
        PendingBlockState, RestoreJournalPhase, RestoreJournalStep, SyncState, TxLifecycle,
    },
    driven::db::{
        mongo_db::{StoredBlock, DB, DEPOSIT_COLLECTION},
        snapshot::{
            create_proposer_snapshot, load_proposer_snapshot_into_shadow, restore_proposer_snapshot,
        },
    },
    driven::nightfall_event::get_expected_layer2_blocknumber,
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    ports::{
        contracts::NightfallContract,
        db::{BlockStorageDB, PendingBlockDB, RestoreJournalDB, SyncStateDB, TransactionsDB},
        trees::{CommitmentTree, HistoricRootTree},
    },
};
use alloy::primitives::{Address, Bytes, TxHash, I256};
use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use ark_std::Zero;
use lib::hex_conversion::HexConvertible;
use lib::merkle_trees::trees::{MutableTree, TreeMetadata};
use lib::nf_client_proof::Proof;
use lib::shared_entities::{ClientTransaction, CompressedSecrets, DepositData};
use lib::tests_utils::{get_db_connection, get_mongo};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tokio::sync::{Mutex, OnceCell};

struct MockNightfallContract;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct MockProof;

impl Proof for MockProof {
    fn compress_proof(&self) -> Result<Bytes, SerializationError> {
        Ok(Bytes::new())
    }

    fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
        Ok(Self)
    }
}

#[derive(Debug, PartialEq)]
struct CapturedSyncState {
    last_applied_l2_block: u64,
    fingerprint: String,
    l1_ref: L1Ref,
    schema_version: u32,
}

#[derive(Debug, PartialEq)]
struct CapturedStoredBlock {
    layer2_block_number: u64,
    commitments: Vec<String>,
    proposer_address: Address,
}

#[derive(Debug, PartialEq)]
struct CapturedProposerState {
    sync_state: CapturedSyncState,
    blocks: Vec<CapturedStoredBlock>,
    commitment_root: Fr254,
    historic_root: Fr254,
}

fn mock_onchain_next_block() -> &'static std::sync::atomic::AtomicU64 {
    static ONCHAIN_NEXT_BLOCK: std::sync::OnceLock<std::sync::atomic::AtomicU64> =
        std::sync::OnceLock::new();
    ONCHAIN_NEXT_BLOCK.get_or_init(|| std::sync::atomic::AtomicU64::new(0))
}

impl MockNightfallContract {
    fn set_onchain_next_block(block_number: u64) {
        mock_onchain_next_block().store(block_number, std::sync::atomic::Ordering::SeqCst);
    }
}

#[async_trait::async_trait]
impl NightfallContract for MockNightfallContract {
    async fn propose_block(
        _block: crate::domain::entities::Block,
    ) -> Result<crate::ports::contracts::ProposeBlockOutcome, lib::error::NightfallContractError>
    {
        unreachable!("propose_block is not used in bootstrap tests")
    }

    async fn get_proposal_receipt_status(
        _tx_hash: TxHash,
    ) -> Result<Option<bool>, lib::error::NightfallContractError> {
        unreachable!("get_proposal_receipt_status is not used in bootstrap tests")
    }

    async fn get_current_layer2_blocknumber() -> Result<I256, lib::error::NightfallContractError> {
        Ok(
            I256::try_from(mock_onchain_next_block().load(std::sync::atomic::Ordering::SeqCst))
                .expect("mock block number fits into I256"),
        )
    }
}

async fn bootstrap_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
    static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
    LOCK.get_or_init(|| async { Mutex::new(()) })
        .await
        .lock()
        .await
}

async fn persist_sync_state(client: &mongodb::Client, sync_state: &SyncState) {
    let mut session = client.start_session().await.expect("start session");
    let client_for_write = client.clone();
    let sync_state_for_write = sync_state.clone();
    session
        .start_transaction()
        .and_run2(async move |session| {
            client_for_write
                .update_sync_state_with_session(&sync_state_for_write, session)
                .await?;
            Ok::<(), mongodb::error::Error>(())
        })
        .await
        .expect("write sync_state");

    let stored_block = client
        .get_block_by_number(sync_state.last_applied_l2_block)
        .await
        .expect("stored block should exist for test sync_state");

    let commitment_metadata = client
        .database(DB)
        .collection::<TreeMetadata<Fr254>>(&format!(
            "{}_metadata",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        ))
        .find_one(doc! { "_id": 0 })
        .await
        .expect("read commitment metadata");
    if commitment_metadata
        .as_ref()
        .is_some_and(|metadata| metadata.sub_tree_count == 0)
        && !stored_block.commitments.is_empty()
    {
        <mongodb::Client as MutableTree<Fr254>>::insert_leaf(
            client,
            Fr254::from(sync_state.last_applied_l2_block + 1),
            true,
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("materialize commitment tree state for bootstrap test");
    }

    let target_historic_root_sub_tree_count = sync_state
        .last_applied_l2_block
        .checked_add(2)
        .expect("historic root count should not overflow in bootstrap tests");
    let Some(historic_root_metadata) = client
        .database(DB)
        .collection::<TreeMetadata<Fr254>>(&format!(
            "{}_metadata",
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
        ))
        .find_one(doc! { "_id": 0 })
        .await
        .expect("read historic root metadata")
    else {
        return;
    };
    let mut historic_root_sub_tree_count = historic_root_metadata.sub_tree_count;

    while historic_root_sub_tree_count < target_historic_root_sub_tree_count {
        let commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
            .await
            .expect("read commitment root");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client,
            &commitment_root,
            true,
        )
        .await
        .expect("materialize historic root state for bootstrap test");
        historic_root_sub_tree_count += 1;
    }
}

async fn materialize_tree_state_for_block(client: &mongodb::Client, block_number: u64) {
    let commitment_leaf = Fr254::from(block_number + 1);
    <mongodb::Client as MutableTree<Fr254>>::insert_leaf(
        client,
        commitment_leaf,
        true,
        <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
    )
    .await
    .expect("append test commitment leaf");

    let commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
        .await
        .expect("read commitment root");
    <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
        client,
        &commitment_root,
        true,
    )
    .await
    .expect("append historic root");
}

async fn create_snapshot_fixture(
    client: &mongodb::Client,
    snapshot_root_prefix: &str,
    layer2_block_number: u64,
    l1_block_number: u64,
) -> (PathBuf, StoredBlock, SyncState) {
    ensure_proposer_db_initialized(client).await;

    let stored_block = StoredBlock {
        layer2_block_number,
        commitments: vec![format!("0x{layer2_block_number:02x}")],
        proposer_address: Address::from([layer2_block_number as u8; 20]),
    };
    client
        .store_block(&stored_block)
        .await
        .expect("store block for snapshot fixture");
    materialize_tree_state_for_block(client, layer2_block_number).await;

    let sync_state = SyncState::new(
        stored_block.layer2_block_number,
        stored_block.hash().to_hex_string(),
        L1Ref {
            block_number: l1_block_number,
            tx_hash: TxHash::from([layer2_block_number as u8; 32]),
            log_index: layer2_block_number,
        },
        mongodb::bson::DateTime::now(),
    );
    persist_sync_state(client, &sync_state).await;

    let snapshot_root = std::env::temp_dir().join(format!(
        "{snapshot_root_prefix}-{}",
        mongodb::bson::DateTime::now().timestamp_millis()
    ));
    create_proposer_snapshot(client, &snapshot_root)
        .await
        .expect("create snapshot fixture");

    (snapshot_root, stored_block, sync_state)
}

async fn set_live_sync_state(
    client: &mongodb::Client,
    layer2_block_number: u64,
    commitment_tag: &str,
    l1_block_number: u64,
) -> SyncState {
    let stored_block = StoredBlock {
        layer2_block_number,
        commitments: vec![commitment_tag.to_string()],
        proposer_address: Address::from([layer2_block_number as u8; 20]),
    };
    client
        .store_block(&stored_block)
        .await
        .expect("store live block");
    materialize_tree_state_for_block(client, layer2_block_number).await;

    let sync_state = SyncState::new(
        stored_block.layer2_block_number,
        stored_block.hash().to_hex_string(),
        L1Ref {
            block_number: l1_block_number,
            tx_hash: TxHash::from([layer2_block_number as u8; 32]),
            log_index: layer2_block_number,
        },
        mongodb::bson::DateTime::now(),
    );
    persist_sync_state(client, &sync_state).await;
    sync_state
}

fn test_reserved_deposit(seed: u64) -> DepositDatawithFee {
    DepositDatawithFee {
        fee: Fr254::from(seed),
        deposit_data: DepositData {
            nf_token_id: Fr254::from(seed + 100),
            nf_slot_id: Fr254::from(seed + 101),
            value: Fr254::from(seed + 102),
            secret_hash: Fr254::from(seed + 103),
        },
        reserved: true,
    }
}

fn test_selected_client_transaction(
    seed: u32,
    block_l2: u64,
) -> ClientTransactionWithMetaData<MockProof> {
    ClientTransactionWithMetaData {
        client_transaction: ClientTransaction {
            commitments: [
                Fr254::from(u64::from(seed) + 200),
                Fr254::zero(),
                Fr254::zero(),
                Fr254::zero(),
            ],
            compressed_secrets: CompressedSecrets::default(),
            proof: MockProof,
            ..Default::default()
        },
        lifecycle: TxLifecycle::Selected { block_l2 },
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
        "selected_or_included" => {
            document.insert("in_mempool", false);
            document.insert("cancelled_explicitly", false);
            document.insert(
                "block_l2",
                mongodb::bson::Bson::Int64(
                    i64::try_from(transaction.lifecycle.block_l2().expect("block_l2"))
                        .expect("block_l2 fits i64"),
                ),
            );
        }
        other => panic!("unexpected legacy kind {other}"),
    }

    document
}

async fn capture_proposer_state(
    client: &mongodb::Client,
    block_numbers: &[u64],
) -> CapturedProposerState {
    let sync_state = client
        .get_sync_state()
        .await
        .expect("sync_state should exist for captured state");
    let mut blocks = Vec::with_capacity(block_numbers.len());
    for block_number in block_numbers {
        let stored_block = client
            .get_block_by_number(*block_number)
            .await
            .unwrap_or_else(|| panic!("missing stored block {block_number}"));
        blocks.push(CapturedStoredBlock {
            layer2_block_number: stored_block.layer2_block_number,
            commitments: stored_block.commitments,
            proposer_address: stored_block.proposer_address,
        });
    }

    CapturedProposerState {
        sync_state: CapturedSyncState {
            last_applied_l2_block: sync_state.last_applied_l2_block,
            fingerprint: sync_state.fingerprint,
            l1_ref: sync_state.l1_ref,
            schema_version: sync_state.schema_version,
        },
        blocks,
        commitment_root: <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
            .await
            .expect("read commitment root for captured state"),
        historic_root: <mongodb::Client as MutableTree<Fr254>>::get_root(
            client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("read historic root for captured state"),
    }
}

fn snapshot_dir(snapshot_root: &PathBuf) -> PathBuf {
    fs::read_dir(snapshot_root)
        .expect("read snapshot root")
        .next()
        .expect("snapshot dir should exist")
        .expect("read snapshot dir entry")
        .path()
}

async fn reset_runtime_bootstrap_state() {
    *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
    get_synchronisation_status()
        .await
        .write()
        .await
        .clear_synchronised();
    set_runtime_listener_start_block(0).await;
    set_runtime_listener_resume_cursor(None).await;
}

#[tokio::test]
async fn bootstrap_aborts_when_sync_state_exists_but_trees_are_torn_down() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let stored_block = StoredBlock {
        layer2_block_number: 5,
        commitments: vec!["0xtorn-state".to_string()],
        proposer_address: Address::from([5u8; 20]),
    };
    client
        .store_block(&stored_block)
        .await
        .expect("store torn-state block");
    persist_sync_state(
        &client,
        &SyncState::new(
            stored_block.layer2_block_number,
            stored_block.hash().to_hex_string(),
            L1Ref {
                block_number: 500,
                tx_hash: TxHash::from([5u8; 32]),
                log_index: 5,
            },
            mongodb::bson::DateTime::now(),
        ),
    )
    .await;

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    let error = bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect_err("bootstrap should reject torn proposer tree state");

    assert!(
        error.contains("tree state is inconsistent")
            && error.contains("Manual recovery is required"),
        "unexpected bootstrap error: {error}"
    );
}

#[tokio::test]
async fn bootstrap_aborts_when_sync_state_exists_but_nullifier_metadata_is_missing() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    client
        .database(DB)
        .collection::<Document>("Nullifiers_metadata")
        .drop()
        .await
        .expect("drop nullifier metadata");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    let error = bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect_err("bootstrap should reject missing nullifier metadata with sync_state present");

    assert!(
        error.contains("tree state is inconsistent")
            && error.contains("tree metadata for Nullifiers is missing"),
        "unexpected bootstrap error: {error}"
    );
}

#[tokio::test]
async fn bootstrap_aborts_when_sync_state_exists_but_nullifier_indexed_leaves_are_empty() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    client
        .database(DB)
        .collection::<Document>("Nullifiers_indexed_leaves")
        .delete_many(doc! {})
        .await
        .expect("empty nullifier indexed leaves");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    let error = bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect_err(
            "bootstrap should reject empty nullifier indexed leaves with sync_state present",
        );

    assert!(
        error.contains("tree state is inconsistent")
            && error.contains("nullifier indexed leaves collection is empty"),
        "unexpected bootstrap error: {error}"
    );
}

#[tokio::test]
async fn bootstrap_cleans_stored_blocks_ahead_of_sync_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .expect("store speculative block ahead of sync_state");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean stored blocks ahead of sync_state");

    assert!(client.get_block_by_number(6).await.is_none());
}

#[tokio::test]
async fn bootstrap_aborts_when_no_sync_state_but_stored_blocks_exist() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    client
        .store_block(&StoredBlock {
            layer2_block_number: 0,
            commitments: vec!["0x00".to_string()],
            proposer_address: Address::from([0u8; 20]),
        })
        .await
        .expect("store block without sync_state");

    MockNightfallContract::set_onchain_next_block(0);
    reset_runtime_bootstrap_state().await;

    let error = bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect_err("bootstrap should fail closed on stored blocks without sync_state");

    assert!(
        error.contains("StoredBlocks exist without sync_state")
            && error.contains("Manual recovery is required"),
        "unexpected bootstrap error: {error}"
    );
    assert!(
        client.get_block_by_number(0).await.is_some(),
        "stored block should remain because cleanup must not run after fail-closed validation"
    );
}

#[tokio::test]
async fn bootstrap_cleans_first_pending_block_startup_state_without_sync_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    let canonical_commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
        .await
        .expect("read canonical commitment root before cleanup");
    let canonical_historic_root = <mongodb::Client as MutableTree<Fr254>>::get_root(
        &client,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    .expect("read canonical historic root before cleanup");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 0,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: Vec::new(),
            selected_client_transaction_hashes: Vec::new(),
        })
        .await
        .expect("store first pending block");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 0,
            commitments: vec!["0x00".to_string()],
            proposer_address: Address::from([0u8; 20]),
        })
        .await
        .expect("store first speculative block without sync_state");

    MockNightfallContract::set_onchain_next_block(0);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean first pending startup state without sync_state");

    assert_eq!(client.get_sync_state().await, None);
    assert_eq!(client.get_pending_block(0).await, None);
    assert!(client.get_block_by_number(0).await.is_none());
    assert_eq!(
        <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
            .await
            .expect("read commitment root after cleanup"),
        canonical_commitment_root
    );
    assert_eq!(
        <mongodb::Client as MutableTree<Fr254>>::get_root(
            &client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("read historic root after cleanup"),
        canonical_historic_root
    );
}

#[tokio::test]
async fn bootstrap_aborts_when_no_sync_state_but_trees_are_non_empty() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    materialize_tree_state_for_block(&client, 0).await;

    let commitment_root_before = <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
        .await
        .expect("read commitment root before failed bootstrap");
    let historic_root_before = <mongodb::Client as MutableTree<Fr254>>::get_root(
        &client,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    .expect("read historic root before failed bootstrap");

    MockNightfallContract::set_onchain_next_block(0);
    reset_runtime_bootstrap_state().await;

    let error = bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect_err("bootstrap should fail closed on trees without sync_state");

    assert!(
        error.contains("no proposer sync_state exists, but proposer trees are not empty"),
        "unexpected bootstrap error: {error}"
    );
    assert_eq!(
        <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
            .await
            .expect("read commitment root after failed bootstrap"),
        commitment_root_before
    );
    assert_eq!(
        <mongodb::Client as MutableTree<Fr254>>::get_root(
            &client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("read historic root after failed bootstrap"),
        historic_root_before
    );
}

#[tokio::test]
async fn bootstrap_cleans_ready_to_propose_pending_block_startup_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let canonical_commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
        .await
        .expect("read canonical commitment root before cleanup");
    let canonical_historic_root = <mongodb::Client as MutableTree<Fr254>>::get_root(
        &client,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    .expect("read canonical historic root before cleanup");
    let selected_deposit = test_reserved_deposit(5);
    let selected_transaction = test_selected_client_transaction(51, 6);
    let included_transaction = ClientTransactionWithMetaData {
        lifecycle: TxLifecycle::Included { block_l2: 4 },
        ..test_selected_client_transaction(61, 4)
    };
    let dropped_transaction = ClientTransactionWithMetaData {
        lifecycle: TxLifecycle::Dropped,
        ..test_selected_client_transaction(71, 6)
    };
    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit],
    )
    .await
    .expect("store reserved deposit for ready pending block");
    client
        .store_transaction(selected_transaction.clone())
        .await
        .expect("store selected client transaction for ready pending block");
    client
        .store_transaction(included_transaction.clone())
        .await
        .expect("store included client transaction for ready pending block");
    client
        .store_transaction(dropped_transaction.clone())
        .await
        .expect("store dropped client transaction for ready pending block");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
        })
        .await
        .expect("store ready pending block");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .expect("store speculative block for ready pending block");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean ready pending state");

    assert_eq!(client.get_pending_block(6).await, None);
    assert!(client.get_block_by_number(6).await.is_none());
    assert_eq!(
        client
            .database(DB)
            .collection::<Document>(DEPOSIT_COLLECTION)
            .count_documents(doc! { "reserved": true })
            .await
            .expect("count reserved deposits after ready pending cleanup"),
        0
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
    assert_eq!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &included_transaction.hash,
        )
        .await
        .expect("included transaction should still exist after cleanup")
        .lifecycle,
        TxLifecycle::Included { block_l2: 4 }
    );
    assert_eq!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &dropped_transaction.hash,
        )
        .await
        .expect("dropped transaction should still exist after cleanup")
        .lifecycle,
        TxLifecycle::Dropped
    );
    assert_eq!(
        <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
            .await
            .expect("read commitment root after cleanup"),
        canonical_commitment_root
    );
    assert_eq!(
        <mongodb::Client as MutableTree<Fr254>>::get_root(
            &client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("read historic root after cleanup"),
        canonical_historic_root
    );
}

#[tokio::test]
async fn bootstrap_cleans_reserved_pending_block_recovery_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let selected_deposit = test_reserved_deposit(7);
    let selected_transaction = test_selected_client_transaction(71, 6);
    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit],
    )
    .await
    .expect("store reserved deposit for reserved pending block");
    client
        .store_transaction(selected_transaction.clone())
        .await
        .expect("store selected client transaction for reserved pending block");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::Reserved,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: None,
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: vec![vec![9, 9, 9]],
        })
        .await
        .expect("store reserved pending block");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .expect("store speculative block for reserved pending block");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean reserved pending state");

    assert_eq!(client.get_pending_block(6).await, None);
    assert!(client.get_block_by_number(6).await.is_none());
    assert_eq!(
        client
            .database(DB)
            .collection::<Document>(DEPOSIT_COLLECTION)
            .count_documents(doc! { "reserved": true })
            .await
            .expect("count reserved deposits after reserved pending cleanup"),
        0
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
}

#[tokio::test]
async fn bootstrap_cleans_broadcast_pending_block_startup_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let selected_deposit = test_reserved_deposit(9);
    let selected_transaction = test_selected_client_transaction(91, 6);
    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit],
    )
    .await
    .expect("store reserved deposit for broadcast pending block");
    client
        .store_transaction(selected_transaction.clone())
        .await
        .expect("store selected client transaction for broadcast pending block");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::BroadcastPending,
            broadcast_tx_hash: Some(TxHash::from([9u8; 32])),
            broadcast_receipt_checks: 3,
            block: Some(Block::default()),
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
        })
        .await
        .expect("store broadcast-pending block");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([9u8; 20]),
        })
        .await
        .expect("store speculative block for broadcast-pending block");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean broadcast-pending state");

    assert_eq!(client.get_pending_block(6).await, None);
    assert!(client.get_block_by_number(6).await.is_none());
    assert_eq!(
        client
            .database(DB)
            .collection::<Document>(DEPOSIT_COLLECTION)
            .count_documents(doc! { "reserved": true })
            .await
            .expect("count reserved deposits after broadcast cleanup"),
        0
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
}

#[tokio::test]
async fn bootstrap_backfills_legacy_client_transaction_lifecycle_before_cleanup() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    let included_transaction = test_selected_client_transaction(201, 5);
    let selected_transaction = test_selected_client_transaction(211, 6);
    let canonical_commitment =
        included_transaction.client_transaction.commitments[0].to_hex_string();
    set_live_sync_state(&client, 5, &canonical_commitment, 500).await;
    client
        .database(DB)
        .collection::<Document>("ClientTransactions")
        .insert_many(vec![
            legacy_document_from_transaction(&included_transaction, "selected_or_included"),
            legacy_document_from_transaction(&selected_transaction, "selected_or_included"),
        ])
        .await
        .expect("store legacy lifecycle fixtures");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should backfill legacy lifecycle fields before cleanup");

    assert_eq!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &included_transaction.hash,
        )
        .await
        .expect("included transaction should still exist after bootstrap")
        .lifecycle,
        TxLifecycle::Included { block_l2: 5 }
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after bootstrap")
        .lifecycle
        .is_mempool(),
        "startup cleanup should only demote the legacy Selected transaction after backfill"
    );
}

#[tokio::test]
async fn bootstrap_preserves_selected_transactions_at_sync_state_height() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    let canonical_selected_transaction = test_selected_client_transaction(221, 5);
    let speculative_selected_transaction = test_selected_client_transaction(231, 6);
    client
        .store_transaction(canonical_selected_transaction.clone())
        .await
        .expect("store canonical selected transaction");
    client
        .store_transaction(speculative_selected_transaction.clone())
        .await
        .expect("store speculative selected transaction");
    set_live_sync_state(&client, 5, "0x05", 500).await;

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should preserve canonical selected transactions at sync_state height");

    assert_eq!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &canonical_selected_transaction.hash,
        )
        .await
        .expect("canonical selected transaction should still exist after bootstrap")
        .lifecycle,
        TxLifecycle::Selected { block_l2: 5 }
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &speculative_selected_transaction.hash,
        )
        .await
        .expect("speculative selected transaction should still exist after bootstrap")
        .lifecycle
        .is_mempool(),
        "startup cleanup should only demote Selected transactions ahead of sync_state"
    );
}

#[tokio::test]
async fn bootstrap_cleans_reserved_deposits_ahead_of_sync_state_without_pending_block() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![DepositDatawithFee {
            fee: Fr254::from(5u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(51u64),
                nf_slot_id: Fr254::from(52u64),
                value: Fr254::from(53u64),
                secret_hash: Fr254::from(54u64),
            },
            reserved: true,
        }],
    )
    .await
    .expect("store reserved deposit");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean reserved deposits ahead of sync_state");

    assert_eq!(
        client
            .database(DB)
            .collection::<Document>(DEPOSIT_COLLECTION)
            .count_documents(doc! { "reserved": true })
            .await
            .expect("count reserved deposits after cleanup"),
        0
    );
}

#[tokio::test]
async fn bootstrap_cleans_reserved_pending_block_with_mismatched_deposit_identities() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let selected_deposit = DepositDatawithFee {
        reserved: false,
        ..test_reserved_deposit(11)
    };
    let orphan_reserved_deposit = test_reserved_deposit(12);
    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit, orphan_reserved_deposit],
    )
    .await
    .expect("store mismatched reserved deposits for reserved cleanup");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::Reserved,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: None,
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: Vec::new(),
        })
        .await
        .expect("store reserved pending block");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean mismatched reserved deposit identities");

    assert_eq!(
        client
            .database(DB)
            .collection::<Document>(DEPOSIT_COLLECTION)
            .count_documents(doc! { "reserved": true })
            .await
            .expect("count reserved deposits after cleanup"),
        0
    );
    assert_eq!(client.get_pending_block(6).await, None);
}

#[tokio::test]
async fn bootstrap_cleans_ready_pending_selected_client_transaction_hash_mismatch() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let selected_deposit = test_reserved_deposit(13);
    let selected_transaction = test_selected_client_transaction(131, 6);
    let orphan_selected_transaction = test_selected_client_transaction(141, 6);

    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit],
    )
    .await
    .expect("store reserved deposit for ready pending state");
    client
        .store_transaction(selected_transaction.clone())
        .await
        .expect("store selected client transaction for ready pending block");
    client
        .store_transaction(orphan_selected_transaction.clone())
        .await
        .expect("store orphan selected transaction for ready pending block");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
        })
        .await
        .expect("store ready pending block with mismatched selected tx identities");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .expect("store speculative block for ready pending block");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean mismatched selected transaction identities");

    assert_eq!(client.get_pending_block(6).await, None);
    assert!(client.get_block_by_number(6).await.is_none());
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &orphan_selected_transaction.hash,
        )
        .await
        .expect("orphan selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
}

#[tokio::test]
async fn bootstrap_cleans_ready_pending_selected_client_transaction_block_mismatch() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    ensure_proposer_db_initialized(&client).await;
    set_live_sync_state(&client, 5, "0x05", 500).await;
    let selected_deposit = test_reserved_deposit(15);
    let selected_transaction = test_selected_client_transaction(151, 7);

    <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
        &client,
        vec![selected_deposit],
    )
    .await
    .expect("store reserved deposit for wrong-block selected transaction");
    client
        .store_transaction(selected_transaction.clone())
        .await
        .expect("store selected transaction with wrong block");

    client
        .store_pending_block(&PendingBlock {
            layer2_block_number: 6,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
        })
        .await
        .expect("store ready pending block");
    client
        .store_block(&StoredBlock {
            layer2_block_number: 6,
            commitments: vec!["0x06".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .expect("store speculative block for wrong-block selected tx");

    MockNightfallContract::set_onchain_next_block(6);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean selected transaction bound to wrong block");

    assert_eq!(client.get_pending_block(6).await, None);
    assert!(client.get_block_by_number(6).await.is_none());
    assert!(
        <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
            &client,
            &selected_transaction.hash,
        )
        .await
        .expect("selected transaction should still exist after cleanup")
        .lifecycle
        .is_mempool()
    );
}

async fn rename_collection(
    client: &mongodb::Client,
    from: &str,
    to: &str,
) -> Result<(), mongodb::error::Error> {
    client
        .database("admin")
        .run_command(doc! {
            "renameCollection": format!("{DB}.{from}"),
            "to": format!("{DB}.{to}"),
            "dropTarget": false,
        })
        .await?;
    Ok(())
}

fn ordered_restore_collections(
    journal: &[crate::domain::entities::RestoreJournalCollection],
) -> Vec<crate::domain::entities::RestoreJournalCollection> {
    let mut ordered = journal.to_vec();
    ordered.sort_by(|left, right| {
        match (
            left.live == crate::driven::db::mongo_db::SYNC_STATE_COLLECTION,
            right.live == crate::driven::db::mongo_db::SYNC_STATE_COLLECTION,
        ) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => left.live.cmp(&right.live),
        }
    });
    ordered
}

async fn first_existing_live_restore_collection(
    client: &mongodb::Client,
    journal: &[crate::domain::entities::RestoreJournalCollection],
) -> (usize, crate::domain::entities::RestoreJournalCollection) {
    let existing_collections = client
        .database(DB)
        .list_collection_names()
        .await
        .expect("list collections");
    ordered_restore_collections(journal)
        .into_iter()
        .enumerate()
        .find(|(_, collection)| {
            existing_collections
                .iter()
                .any(|name| name == &collection.live)
        })
        .expect("at least one live restore collection should exist")
}

async fn next_required_live_restore_collection(
    client: &mongodb::Client,
    journal: &[crate::domain::entities::RestoreJournalCollection],
    after_index: usize,
) -> (usize, crate::domain::entities::RestoreJournalCollection) {
    let existing_collections = client
        .database(DB)
        .list_collection_names()
        .await
        .expect("list collections");
    ordered_restore_collections(journal)
        .into_iter()
        .enumerate()
        .skip(after_index + 1)
        .find(|(_, collection)| {
            existing_collections
                .iter()
                .any(|name| name == &collection.live)
                && crate::driven::db::snapshot::required_proposer_snapshot_collection_names()
                    .iter()
                    .any(|name| name == &collection.live)
        })
        .expect("expected another required live restore collection after the first processed one")
}

#[tokio::test]
async fn bootstrap_cleans_loading_shadow_restore_journal_and_keeps_live_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, _, _) =
        create_snapshot_fixture(&client, "nf4-bootstrap-loading-shadow", 8, 800).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    let newer_live_sync_state = set_live_sync_state(&client, 9, "0xlive", 900).await;
    let journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    assert_eq!(journal.phase, RestoreJournalPhase::LoadingShadow);

    MockNightfallContract::set_onchain_next_block(10);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should clean loading shadow and continue");

    assert_eq!(client.get_restore_journal().await, None);
    assert_eq!(
        client.get_sync_state().await,
        Some(newer_live_sync_state.clone())
    );
    assert_eq!(
        *get_expected_layer2_blocknumber().await.read().await,
        I256::try_from(10_u64).expect("10 fits into I256")
    );
    assert_eq!(get_runtime_listener_start_block().await, 900);
    assert_eq!(
        get_runtime_listener_resume_cursor().await,
        Some(newer_live_sync_state.l1_ref.clone())
    );
    assert!(!get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised());

    let collection_names = client
        .database(DB)
        .list_collection_names()
        .await
        .expect("list collections");
    assert!(!collection_names
        .iter()
        .any(|name| name.starts_with("restore_shadow__")));
    assert!(!collection_names
        .iter()
        .any(|name| name.starts_with("restore_backup__")));

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn bootstrap_resumes_swap_in_progress_restore_and_keeps_tip_desynchronised_until_replay() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, snapshot_block, _) =
        create_snapshot_fixture(&client, "nf4-bootstrap-swap-resume", 12, 1200).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    set_live_sync_state(&client, 13, "0xnewer-live", 1300).await;
    let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    let ordered = ordered_restore_collections(&journal.collections);
    let (first_live_index, first_live) =
        first_existing_live_restore_collection(&client, &journal.collections).await;
    rename_collection(&client, &first_live.live, &first_live.backup)
        .await
        .expect("simulate backup rename");
    journal.collections = ordered;
    journal.phase = RestoreJournalPhase::SwapInProgress;
    journal.current_index = Some(first_live_index as u32);
    journal.current_step = Some(RestoreJournalStep::BackupCreated);
    journal.updated_at = mongodb::bson::DateTime::now();
    client
        .upsert_restore_journal(&journal)
        .await
        .expect("persist mid-swap journal");

    MockNightfallContract::set_onchain_next_block(snapshot_block.layer2_block_number + 1);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should resume swap and continue");

    let resumed = client
        .get_restore_journal()
        .await
        .expect("swap_complete journal should remain for cleanup");
    assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);

    let live_sync_state = client
        .get_sync_state()
        .await
        .expect("restored sync_state should exist");
    assert_eq!(live_sync_state.last_applied_l2_block, 12);
    assert_eq!(live_sync_state.l1_ref.block_number, 1200);
    assert_eq!(
        *get_expected_layer2_blocknumber().await.read().await,
        I256::try_from(13_u64).expect("13 fits into I256")
    );
    assert_eq!(get_runtime_listener_start_block().await, 1200);
    assert_eq!(
        get_runtime_listener_resume_cursor().await,
        Some(live_sync_state.l1_ref.clone())
    );
    assert!(!get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised());

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn bootstrap_keeps_resumed_restore_desynchronised_when_one_block_behind_tip() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, snapshot_block, snapshot_sync_state) =
        create_snapshot_fixture(&client, "nf4-bootstrap-one-behind", 14, 1400).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    set_live_sync_state(&client, 15, "0xnewer-live", 1500).await;
    let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    let ordered = ordered_restore_collections(&journal.collections);
    let (first_live_index, first_live) =
        first_existing_live_restore_collection(&client, &journal.collections).await;
    rename_collection(&client, &first_live.live, &first_live.backup)
        .await
        .expect("simulate backup rename");
    journal.collections = ordered;
    journal.phase = RestoreJournalPhase::SwapInProgress;
    journal.current_index = Some(first_live_index as u32);
    journal.current_step = Some(RestoreJournalStep::BackupCreated);
    journal.updated_at = mongodb::bson::DateTime::now();
    client
        .upsert_restore_journal(&journal)
        .await
        .expect("persist mid-swap journal");

    MockNightfallContract::set_onchain_next_block(snapshot_block.layer2_block_number + 2);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should resume swap and remain desynchronised");

    assert_eq!(
        *get_expected_layer2_blocknumber().await.read().await,
        I256::try_from(15_u64).expect("15 fits into I256")
    );
    assert_eq!(get_runtime_listener_start_block().await, 1400);
    assert_eq!(
        get_runtime_listener_resume_cursor().await,
        Some(snapshot_sync_state.l1_ref.clone())
    );
    assert!(!get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised());

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn bootstrap_continues_after_restore_rollback_clears_the_journal() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, _, _) =
        create_snapshot_fixture(&client, "nf4-bootstrap-rollback", 20, 2000).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    let newer_live_sync_state = set_live_sync_state(&client, 21, "0xnewer-live", 2100).await;
    let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    let ordered = ordered_restore_collections(&journal.collections);
    let (first_live_index, first_live) =
        first_existing_live_restore_collection(&client, &journal.collections).await;
    rename_collection(&client, &first_live.live, &first_live.backup)
        .await
        .expect("rename first live to backup");
    rename_collection(&client, &first_live.shadow, &first_live.live)
        .await
        .expect("rename first shadow to live");
    let (next_live_index, next_live) =
        next_required_live_restore_collection(&client, &journal.collections, first_live_index)
            .await;

    journal.collections = ordered;
    journal.phase = RestoreJournalPhase::SwapInProgress;
    journal.current_index = Some(next_live_index as u32);
    journal.current_step = Some(RestoreJournalStep::BackupPending);
    journal.updated_at = mongodb::bson::DateTime::now();
    client
        .upsert_restore_journal(&journal)
        .await
        .expect("persist rollback fixture journal");

    client
        .database(DB)
        .collection::<Document>(&next_live.live)
        .drop()
        .await
        .expect("drop live collection to force rollback");

    MockNightfallContract::set_onchain_next_block(22);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should continue after rollback recovery");

    assert_eq!(client.get_restore_journal().await, None);
    assert_eq!(
        client.get_sync_state().await,
        Some(newer_live_sync_state.clone())
    );
    assert_eq!(
        *get_expected_layer2_blocknumber().await.read().await,
        I256::try_from(22_u64).expect("22 fits into I256")
    );
    assert_eq!(get_runtime_listener_start_block().await, 2100);
    assert!(!get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised());

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn bootstrap_continues_after_swap_complete_rollback_and_restores_pre_snapshot_runtime_state()
{
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, _, _) =
        create_snapshot_fixture(&client, "nf4-bootstrap-swap-complete-rollback", 22, 2200).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    let newer_live_sync_state = set_live_sync_state(&client, 23, "0xnewer-live", 2300).await;
    load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    crate::driven::db::snapshot::swap_proposer_shadow_into_live(&client)
        .await
        .expect("swap shadow into live");

    client
        .delete_block_by_number(22)
        .await
        .expect("delete restored stored block to force swap_complete rollback");

    MockNightfallContract::set_onchain_next_block(24);
    reset_runtime_bootstrap_state().await;

    bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
        .await
        .expect("bootstrap should continue after swap_complete rollback recovery");

    assert_eq!(client.get_restore_journal().await, None);
    assert_eq!(
        client.get_sync_state().await,
        Some(newer_live_sync_state.clone())
    );
    assert_eq!(
        *get_expected_layer2_blocknumber().await.read().await,
        I256::try_from(24_u64).expect("24 fits into I256")
    );
    assert_eq!(get_runtime_listener_start_block().await, 2300);
    assert!(!get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised());

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn startup_recovers_restore_journal_before_reinitializing_live_tree_collections() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, snapshot_block, _) =
        create_snapshot_fixture(&client, "nf4-bootstrap-ordering", 18, 1800).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    set_live_sync_state(&client, 19, "0xnewer-live", 1900).await;
    let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
        .await
        .expect("load snapshot into shadow");
    let ordered = ordered_restore_collections(&journal.collections);
    let (first_live_index, first_live) =
        first_existing_live_restore_collection(&client, &journal.collections).await;
    rename_collection(&client, &first_live.live, &first_live.backup)
        .await
        .expect("simulate missing live collection mid-swap");
    journal.collections = ordered;
    journal.phase = RestoreJournalPhase::SwapInProgress;
    journal.current_index = Some(first_live_index as u32);
    journal.current_step = Some(RestoreJournalStep::BackupCreated);
    journal.updated_at = mongodb::bson::DateTime::now();
    client
        .upsert_restore_journal(&journal)
        .await
        .expect("persist mid-swap journal");

    recover_then_initialize_proposer_db(&client)
        .await
        .expect("startup recovery should complete before init recreates live trees");

    let resumed = client
        .get_restore_journal()
        .await
        .expect("swap_complete journal should remain after recovery");
    assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);

    let restored_sync_state = client
        .get_sync_state()
        .await
        .expect("restored sync_state should exist");
    assert_eq!(
        restored_sync_state.last_applied_l2_block,
        snapshot_block.layer2_block_number
    );
    assert_eq!(restored_sync_state.l1_ref.block_number, 1800);

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot directory");
}

#[tokio::test]
async fn snapshot_restore_then_replay_reaches_canonical_state() {
    let _lock = bootstrap_test_lock().await;
    let container = get_mongo().await;
    let client = get_db_connection(&container).await;

    let (snapshot_root, _, snapshot_sync_state) =
        create_snapshot_fixture(&client, "nf4-restore-replay-e2e", 10, 1000).await;
    let snapshot_dir = snapshot_dir(&snapshot_root);

    set_live_sync_state(&client, 11, "0x0b", 1100).await;
    set_live_sync_state(&client, 12, "0x0c", 1200).await;
    let canonical_state = capture_proposer_state(&client, &[10, 11, 12]).await;

    set_live_sync_state(&client, 12, "0xbad", 1299).await;
    materialize_tree_state_for_block(&client, 99).await;

    let restored_sync_state = restore_proposer_snapshot(&client, &snapshot_dir).await;
    assert_eq!(
        restored_sync_state.expect("restore should succeed"),
        snapshot_sync_state
    );
    assert_eq!(
        client
            .get_sync_state()
            .await
            .expect("restore should reinstall snapshot sync_state"),
        snapshot_sync_state
    );

    set_live_sync_state(&client, 11, "0x0b", 1100).await;
    set_live_sync_state(&client, 12, "0x0c", 1200).await;
    let replayed_state = capture_proposer_state(&client, &[10, 11, 12]).await;

    assert_eq!(replayed_state, canonical_state);

    tokio::fs::remove_dir_all(snapshot_root)
        .await
        .expect("cleanup snapshot root");
}
