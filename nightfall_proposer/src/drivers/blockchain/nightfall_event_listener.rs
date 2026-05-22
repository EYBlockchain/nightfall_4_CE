use crate::{
    driven::db::snapshot::{
        acquire_proposer_state_maintenance_guard, find_latest_valid_proposer_snapshot,
        recover_from_restore_journal, restore_proposer_snapshot,
    },
    driven::nightfall_event::get_expected_layer2_blocknumber,
    drivers::blockchain::block_assembly::clear_pending_blocks_queue,
    initialisation::{
        get_block_assembly_status, get_blockchain_client_connection, get_db_connection,
        set_runtime_listener_start_block,
    },
    ports::{
        contracts::NightfallContract,
        db::{SyncStateDB, TransactionsDB},
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
    services::process_events::process_events,
    services::selected_transactions::reconcile_obviously_orphaned_selected_transactions,
    services::snapshot_scheduler::set_last_snapshot_l2_block,
};
use alloy::{
    primitives::I256,
    rpc::types::Filter,
    sol_types::{SolEvent, SolEventInterface},
};
use ark_bn254::Fr as Fr254;
use configuration::{addresses::get_addresses, settings::get_settings};
use futures::StreamExt;
use futures::{future::BoxFuture, FutureExt};
use lib::{
    blockchain_client::BlockchainClientConnection,
    error::EventHandlerError,
    log_fetcher::get_logs_paginated,
    merkle_trees::trees::MutableTree,
    nf_client_proof::{Proof, ProvingEngine},
    shared_entities::{SynchronisationPhase::Desynchronized, SynchronisationStatus},
};
use log::{debug, warn};
use mongodb::Client as MongoClient;
use nightfall_bindings::artifacts::Nightfall;
use std::path::PathBuf;
use std::time::Duration;
use tokio::{
    sync::{OnceCell, RwLock},
    time::sleep,
};

#[cfg(test)]
use std::{collections::HashSet, sync::Mutex};

#[cfg(test)]
static ENABLED_REPLAY_RESET_FAILPOINTS: std::sync::OnceLock<Mutex<HashSet<String>>> =
    std::sync::OnceLock::new();

#[cfg(test)]
fn enabled_replay_reset_failpoints() -> &'static Mutex<HashSet<String>> {
    ENABLED_REPLAY_RESET_FAILPOINTS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn maybe_fail_replay_reset(_name: &str) -> Result<(), EventHandlerError> {
    #[cfg(test)]
    {
        if enabled_replay_reset_failpoints()
            .lock()
            .expect("replay reset failpoint lock poisoned")
            .contains(_name)
        {
            return Err(EventHandlerError::IOError(format!(
                "Simulated replay reset failure at {_name}"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
struct TestReplayResetFailpointGuard {
    name: String,
}

#[cfg(test)]
impl TestReplayResetFailpointGuard {
    fn enable(name: &str) -> Self {
        enabled_replay_reset_failpoints()
            .lock()
            .expect("replay reset failpoint lock poisoned")
            .insert(name.to_string());
        Self {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
impl Drop for TestReplayResetFailpointGuard {
    fn drop(&mut self) {
        enabled_replay_reset_failpoints()
            .lock()
            .expect("replay reset failpoint lock poisoned")
            .remove(&self.name);
    }
}

/// This function starts the event handler. It will attempt to restart the event handler in case of errors
/// with an exponential backoff for a configurable number of attempts. If the event handler
/// fails after the maximum number of attempts, it will log an error and send a notification (if configured)
pub fn start_event_listener<P, E, N>(
    start_block: usize,
    max_attempts: u32,
) -> BoxFuture<'static, ()>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    debug!("Starting event listener");
    // we use the async block and the BoxFuture so that we can recurse an async
    async move {
        let mut attempts = 0;
        let mut backoff_delay = Duration::from_secs(2);
        let max_attempts = std::cmp::max(1, max_attempts);

        loop {
            attempts += 1;
            log::info!("Proposer event listener (attempt {attempts})...");
            let result = listen_for_events::<P, E, N>(start_block).await;

            match result {
                Ok(_) => {
                    log::info!("Proposer event listener finished successfully.");
                    break;
                }
                Err(e) => {
                    log::error!(
                        "Proposer event listener terminated with error: {e:?}. Restarting in {backoff_delay:?}"
                    );
                    if attempts >= max_attempts {
                        log::error!("Proposer event listener: max attempts reached. Giving up.");
                        if let Err(err) = notify_failure_proposer(
                            "Proposer event listener failed after max retries",
                        )
                        .await
                        {
                            log::error!(
                                "Failed to send failure notification (proposer): {err:?}"
                            );
                        }
                        break;
                    }
                    sleep(backoff_delay).await;
                    backoff_delay *= 2;
                }
            }
        }
    }
    .boxed()
}
async fn notify_failure_proposer(message: &str) -> Result<(), ()> {
    log::error!("ALERT (Proposer): {message}");
    Ok(())
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<P, E, N>(start_block: usize) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();

    let events_filter = Filter::new()
        .address(get_addresses().nightfall())
        .event_signature(vec![
            Nightfall::BlockProposed::SIGNATURE_HASH,
            Nightfall::DepositEscrowed::SIGNATURE_HASH,
            Nightfall::Initialized::SIGNATURE_HASH,
            Nightfall::Upgraded::SIGNATURE_HASH,
            Nightfall::AuthoritiesUpdated::SIGNATURE_HASH,
            Nightfall::OwnershipTransferred::SIGNATURE_HASH,
        ])
        .from_block(start_block as u64);
    // Subscribe to the combined events filter
    let events_subscription = blockchain_client
        .subscribe_logs(&events_filter)
        .await
        .map_err(|_| EventHandlerError::NoEventStream)?;

    {
        let latest_block = blockchain_client
            .get_block_number()
            .await
            .expect("could not get latest block number");

        if latest_block >= start_block as u64 {
            log::info!("Fetching past events from block {start_block} to {latest_block}");
            let past_events = match get_logs_paginated(
                blockchain_client.root(),
                events_filter.clone(),
                start_block as u64,
                latest_block,
            )
            .await
            {
                Ok(events) => events,
                Err(e) => {
                    log::error!("Failed to fetch past events: {e}. Will retry...");
                    return Err(EventHandlerError::IOError(format!(
                        "Failed to fetch past events: {e}"
                    )));
                }
            };
            log::info!("Found {} past events to process", past_events.len());
            for evt in past_events {
                let event = match Nightfall::NightfallEvents::decode_log(&evt.inner) {
                    Ok(e) => e,
                    Err(e) => {
                        warn!("Failed to decode log: {e:?}");
                        continue; // Skip malformed events
                    }
                };
                let result = process_events::<P, E, N>(event.data, evt).await;
                match result {
                    Ok(_) => continue,
                    Err(e) => {
                        match e {
                            // we're missing blocks, so we need to re-synchronise
                            EventHandlerError::MissingBlocks(n) => {
                                warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
                                restart_event_listener::<P, E, N>(start_block).await;
                                return Err(EventHandlerError::StreamTerminated);
                            }

                            EventHandlerError::BlockHashError(expected, found) => {
                                warn!(
                                    "Block hash mismatch: expected {expected:?}, found {found:?}. Restarting event listener"
                                );
                                restart_event_listener::<P, E, N>(start_block).await;
                                return Err(EventHandlerError::StreamTerminated);
                            }

                            _ => panic!("Error processing event: {e:?}"),
                        }
                    }
                }
            }
        } else {
            println!( "Start block {start_block} is greater than latest block {latest_block}. No past events to process.");
        }
    }

    let mut events_stream = events_subscription.into_stream();

    while let Some(log) = events_stream.next().await {
        let event = match Nightfall::NightfallEvents::decode_log(&log.inner) {
            Ok(e) => e,
            Err(e) => {
                warn!("Failed to decode log: {e:?}");
                continue; // Skip malformed events
            }
        };
        let result = process_events::<P, E, N>(event.data, log).await;
        match result {
            Ok(_) => continue,
            Err(e) => {
                match e {
                    // we're missing blocks, so we need to re-synchronise
                    EventHandlerError::MissingBlocks(n) => {
                        warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
                        restart_event_listener::<P, E, N>(start_block).await;
                        return Err(EventHandlerError::StreamTerminated);
                    }

                    EventHandlerError::BlockHashError(expected, found) => {
                        warn!(
                                "Block hash mismatch: expected {expected:?}, found {found:?}. Restarting event listener"
                            );
                        restart_event_listener::<P, E, N>(start_block).await;
                        return Err(EventHandlerError::StreamTerminated);
                    }

                    _ => panic!("Error processing event: {e:?}"),
                }
            }
        }
    }

    Err(EventHandlerError::StreamTerminated)
}

/// Returns the highest L2 block that is safe to target with snapshot restore during desync
/// recovery, based on the last contiguous block the proposer expected to have applied.
async fn latest_restorable_l2_block() -> usize {
    let expected = *get_expected_layer2_blocknumber().await.read().await;
    if expected <= I256::ZERO {
        return 0;
    }

    expected.as_usize().checked_sub(1).unwrap_or_default()
}

/// Cleans transient proposer-side state that should not survive a desync recovery attempt,
/// without touching the restored or replayed tree state.
///
/// Known limitation: pending client transactions that have not reached `Selected`
/// are dropped during recovery and must be resubmitted by clients afterwards.
async fn cleanup_recovery_side_effects<P>(db: &MongoClient)
where
    P: Proof,
{
    let removed_client_txs = TransactionsDB::<P>::remove_all_mempool_client_transactions(db).await;

    debug!(
        "Mempool cleanup: removed {} client transactions.",
        removed_client_txs.unwrap_or(0)
    );
    if let Some(removed_client_txs) = removed_client_txs.filter(|count| *count > 0) {
        warn!(
            "Recovery dropped {removed_client_txs} pending proposer mempool client transaction(s); clients must resubmit them after recovery"
        );
    }

    let restored_selected = reconcile_obviously_orphaned_selected_transactions::<P>(db).await;
    debug!(
        "Selected transaction recovery after restart restored {} orphaned transaction(s).",
        restored_selected.unwrap_or(0)
    );
}

/// Resets proposer state for the destructive replay fallback, including tree state and
/// persisted sync_state, before historical events are replayed.
async fn reset_proposer_state_for_replay<P>(db: &MongoClient) -> Result<(), EventHandlerError>
where
    P: Proof,
{
    let _maintenance_guard = acquire_proposer_state_maintenance_guard().await;

    let mut session = db.start_session().await.map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not start MongoDB session for proposer replay fallback reset: {error}"
        ))
    })?;
    let db_for_cleanup = db.clone();
    session
        .start_transaction()
        .and_run2(async move |session| {
            maybe_fail_replay_reset("delete_sync_state_before_delete")
                .map_err(mongodb::error::Error::custom)?;
            db_for_cleanup
                .delete_sync_state_with_session(session)
                .await?;
            Ok::<(), mongodb::error::Error>(())
        })
        .await
        .map_err(|error| {
            EventHandlerError::IOError(format!(
                "Could not delete proposer sync_state before replay fallback: {error}"
            ))
        })?;

    reset_commitment_tree_for_replay(db).await?;
    reset_historic_root_tree_for_replay(db).await?;
    reset_nullifier_tree_for_replay(db).await?;

    cleanup_recovery_side_effects::<P>(db).await;
    Ok(())
}

async fn reset_commitment_tree_for_replay(db: &MongoClient) -> Result<(), EventHandlerError> {
    maybe_fail_replay_reset("reset_commitment_tree_before_drop")?;
    <MongoClient as MutableTree<Fr254>>::reset_mutable_tree(
        db,
        <MongoClient as CommitmentTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not reset proposer commitment tree before replay fallback: {error}"
        ))
    })?;

    <MongoClient as CommitmentTree<Fr254>>::new_commitment_tree(db, 29, 3)
        .await
        .map_err(|error| {
            EventHandlerError::IOError(format!(
                "Could not reinitialize proposer commitment tree before replay fallback: {error}"
            ))
        })?;

    Ok(())
}

async fn reset_historic_root_tree_for_replay(db: &MongoClient) -> Result<(), EventHandlerError> {
    <MongoClient as MutableTree<Fr254>>::reset_mutable_tree(
        db,
        <MongoClient as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not reset proposer historic root tree before replay fallback: {error}"
        ))
    })?;

    <MongoClient as HistoricRootTree<Fr254>>::new_historic_root_tree(db, 32)
        .await
        .map_err(|error| {
            EventHandlerError::IOError(format!(
                "Could not reinitialize proposer historic root tree before replay fallback: {error}"
            ))
        })?;
    <MongoClient as HistoricRootTree<Fr254>>::append_historic_commitment_root(
        db,
        &Fr254::from(0u8),
        true,
    )
    .await
    .map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not restore zero historic root before replay fallback: {error}"
        ))
    })?;

    Ok(())
}

async fn reset_nullifier_tree_for_replay(db: &MongoClient) -> Result<(), EventHandlerError> {
    <MongoClient as MutableTree<Fr254>>::reset_mutable_tree(
        db,
        <MongoClient as NullifierTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not reset proposer nullifier tree before replay fallback: {error}"
        ))
    })?;

    let indexed_collection = db
        .database("nightfall")
        .collection::<mongodb::bson::Document>("Nullifiers_indexed_leaves");
    indexed_collection.drop().await.map_err(|error| {
        EventHandlerError::IOError(format!(
            "Could not reset proposer nullifier indexed leaves before replay fallback: {error}"
        ))
    })?;

    <MongoClient as NullifierTree<Fr254>>::new_nullifier_tree(db, 29, 3)
        .await
        .map_err(|error| {
            EventHandlerError::IOError(format!(
                "Could not reinitialize proposer nullifier tree before replay fallback: {error}"
            ))
        })?;

    Ok(())
}

/// Returns the configured root directory where proposer snapshots are discovered for restore.
async fn proposer_snapshot_root_dir() -> PathBuf {
    PathBuf::from(&get_settings().nightfall_proposer.snapshot_root_dir)
}

/// Restart the event listener after a desync (missing blocks or hash mismatch).
/// Attempts to restore from the most recent valid proposer snapshot first; falls
/// back to destructive reset+replay if no valid snapshot is found or the restore
/// itself fails. Only the destructive fallback erases already synchronised data.
pub async fn restart_event_listener<P, E, N>(start_block: usize)
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    // if we're restarting the event listener, we definitely shouldn't be in sync, so check that's the case
    let sync_state = get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised();
    if sync_state {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    get_block_assembly_status().await.write().await.pause();
    let dropped_pending_blocks = clear_pending_blocks_queue().await;
    if dropped_pending_blocks > 0 {
        warn!(
            "Recovery cleared {dropped_pending_blocks} queued block assembly candidate(s) before restore/replay"
        );
    }

    let db = get_db_connection().await;
    let max_restorable_l2_block = latest_restorable_l2_block().await as u64;
    let snapshot_root_dir = proposer_snapshot_root_dir().await;
    let mut next_listener_start_block = start_block;

    match find_latest_valid_proposer_snapshot(&snapshot_root_dir, max_restorable_l2_block).await {
        Ok(Some((snapshot_dir, manifest))) => {
            warn!(
                "Desynchronised proposer state detected. Attempting snapshot restore from {} (L2 block {}) before falling back to reset+replay",
                snapshot_dir.display(),
                manifest.last_applied_l2_block
            );
            match restore_proposer_snapshot(db, &snapshot_dir).await {
                Ok(sync_state) => {
                    let next_expected_block = sync_state.last_applied_l2_block.saturating_add(1);
                    *get_expected_layer2_blocknumber().await.write().await =
                        I256::try_from(next_expected_block)
                            .expect("Restored L2 block number does not fit into I256");
                    set_last_snapshot_l2_block(sync_state.last_applied_l2_block).await;
                    let restored_is_at_tip = match N::get_current_layer2_blocknumber().await {
                        Ok(onchain_next_block_i256) if onchain_next_block_i256 >= I256::ZERO => {
                            match u64::try_from(onchain_next_block_i256) {
                                Ok(onchain_next_block) => next_expected_block == onchain_next_block,
                                Err(_) => {
                                    warn!(
                                            "Restored proposer state but could not convert current on-chain L2 block number {} into u64; keeping proposer desynchronised until replay confirms state",
                                            onchain_next_block_i256
                                        );
                                    false
                                }
                            }
                        }
                        Ok(onchain_next_block_i256) => {
                            warn!(
                                    "Restored proposer state but contract returned negative current L2 block number {}; keeping proposer desynchronised until replay confirms state",
                                    onchain_next_block_i256
                                );
                            false
                        }
                        Err(error) => {
                            warn!(
                                    "Restored proposer state but could not fetch current on-chain L2 block number: {}. Keeping proposer desynchronised until replay confirms state",
                                    error
                                );
                            false
                        }
                    };
                    if restored_is_at_tip {
                        get_synchronisation_status()
                            .await
                            .write()
                            .await
                            .set_synchronised();
                    } else {
                        get_synchronisation_status()
                            .await
                            .write()
                            .await
                            .clear_synchronised();
                    }
                    next_listener_start_block = usize::try_from(sync_state.l1_ref.block_number)
                        .expect("Restored L1 block number does not fit into usize");
                    set_runtime_listener_start_block(next_listener_start_block).await;
                    warn!(
                        "Snapshot restore completed successfully. Proposer will replay from L1 block {} with next expected L2 block {}",
                        sync_state.l1_ref.block_number,
                        next_expected_block
                    );
                    cleanup_recovery_side_effects::<P>(db).await;
                }
                Err(error) => {
                    if let Err(recovery_error) = recover_from_restore_journal(db).await {
                        warn!(
                            "Snapshot restore failed and in-process restore journal recovery also failed: {}. Continuing with destructive reset fallback",
                            recovery_error
                        );
                    }
                    warn!(
                        "Snapshot restore from {} failed: {}. Falling back to destructive reset+replay from L1 block {}",
                        snapshot_dir.display(),
                        error,
                        start_block
                    );
                    if let Err(reset_error) = reset_proposer_state_for_replay::<P>(db).await {
                        panic!(
                            "Proposer replay fallback aborted because reset could not complete safely: {reset_error}"
                        );
                    }
                    set_last_snapshot_l2_block(0).await;
                    *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
                    set_runtime_listener_start_block(start_block).await;
                }
            }
        }
        Ok(None) => {
            warn!(
                "No valid proposer snapshot found under {} for L2 recovery point <= {}. Falling back to destructive reset+replay from L1 block {}",
                snapshot_root_dir.display(),
                max_restorable_l2_block,
                start_block
            );
            if let Err(reset_error) = reset_proposer_state_for_replay::<P>(db).await {
                panic!(
                    "Proposer replay fallback aborted because reset could not complete safely: {reset_error}"
                );
            }
            set_last_snapshot_l2_block(0).await;
            *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
            set_runtime_listener_start_block(start_block).await;
        }
        Err(error) => {
            warn!(
                "Snapshot discovery failed under {}: {}. Falling back to destructive reset+replay from L1 block {}",
                snapshot_root_dir.display(),
                error,
                start_block
            );
            if let Err(reset_error) = reset_proposer_state_for_replay::<P>(db).await {
                panic!(
                    "Proposer replay fallback aborted because reset could not complete safely: {reset_error}"
                );
            }
            set_last_snapshot_l2_block(0).await;
            *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
            set_runtime_listener_start_block(start_block).await;
        }
    }

    let settings = get_settings();
    let max_attempts = settings
        .nightfall_proposer
        .max_event_listener_attempts
        .unwrap_or(10);

    get_block_assembly_status().await.write().await.resume();
    start_event_listener::<P, E, N>(next_listener_start_block, max_attempts).await;
}

pub async fn get_synchronisation_status() -> &'static RwLock<SynchronisationStatus> {
    static SYNCHRONISATION_STATUS: OnceCell<RwLock<SynchronisationStatus>> = OnceCell::const_new();
    SYNCHRONISATION_STATUS
        .get_or_init(|| async { RwLock::new(SynchronisationStatus::new(Desynchronized)) })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::{Block, DepositDatawithFee, L1Ref, SyncState},
        drivers::blockchain::block_assembly::{
            pending_blocks_queue_len_for_test, push_pending_block_for_test,
        },
        driven::db::mongo_db::{ensure_deposit_indexes, StoredBlock, DB},
        ports::db::{BlockStorageDB, SyncStateDB, TransactionsDB},
    };
    use alloy::primitives::Bytes;
    use alloy::primitives::{Address, TxHash};
    use ark_ff::Zero;
    use ark_serialize::SerializationError;
    use lib::hex_conversion::HexConvertible;
    use lib::nf_client_proof::Proof;
    use lib::shared_entities::DepositData;
    use lib::tests_utils::{get_db_connection, get_mongo};
    use mongodb::bson::{doc, Document};
    use serde::{Deserialize, Serialize};
    use tokio::sync::{Mutex, OnceCell};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct MockProof;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::new())
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self)
        }
    }

    async fn event_listener_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) })
            .await
            .lock()
            .await
    }

    async fn persist_sync_state(client: &mongodb::Client, sync_state: &SyncState) {
        let mut session = client.start_session().await.expect("start session");
        let client = client.clone();
        let sync_state = sync_state.clone();
        session
            .start_transaction()
            .and_run2(async move |session| {
                client
                    .update_sync_state_with_session(&sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write sync_state");
    }

    async fn initialize_test_trees(client: &mongodb::Client) {
        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(client)
            .await
            .expect("create deposit indexes");
    }

    #[tokio::test]
    async fn reset_proposer_state_for_replay_returns_error_when_tree_reset_fails() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let block = StoredBlock {
            layer2_block_number: 30,
            commitments: vec!["0xreset-tree".to_string()],
            proposer_address: Address::from([30u8; 20]),
        };
        client.store_block(&block).await.expect("store block");
        let sync_state = SyncState::new(
            30,
            block.hash().to_hex_string(),
            L1Ref {
                block_number: 3000,
                tx_hash: TxHash::from([30u8; 32]),
                log_index: 30,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &sync_state).await;

        let _failpoint = TestReplayResetFailpointGuard::enable("reset_commitment_tree_before_drop");
        let error = reset_proposer_state_for_replay::<MockProof>(&client)
            .await
            .expect_err("tree reset failure should abort replay fallback");

        assert!(
            matches!(error, EventHandlerError::IOError(message) if message.contains("reset_commitment_tree_before_drop"))
        );
        assert_eq!(client.get_sync_state().await, None);

        let metadata_count = client
            .database(DB)
            .collection::<Document>("Commitments_metadata")
            .count_documents(doc! {})
            .await
            .expect("count commitment metadata");
        assert!(metadata_count > 0);
    }

    #[tokio::test]
    async fn recovery_entry_clears_pending_block_queue() {
        let _lock = event_listener_test_lock().await;

        get_block_assembly_status().await.write().await.resume();
        push_pending_block_for_test(Block::default()).await;
        assert_eq!(pending_blocks_queue_len_for_test().await, 1);

        get_synchronisation_status()
            .await
            .write()
            .await
            .clear_synchronised();
        get_block_assembly_status().await.write().await.pause();
        let dropped_pending_blocks = clear_pending_blocks_queue().await;

        assert_eq!(dropped_pending_blocks, 1);
        assert_eq!(pending_blocks_queue_len_for_test().await, 0);
    }

    #[tokio::test]
    async fn cleanup_recovery_side_effects_preserves_pending_deposits() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let pending_deposit = DepositDatawithFee {
            fee: Fr254::from(9u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(41u64),
                nf_slot_id: Fr254::from(42u64),
                value: Fr254::from(43u64),
                secret_hash: Fr254::from(44u64),
            },
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![pending_deposit.clone()],
        )
        .await
        .expect("store pending deposit");

        cleanup_recovery_side_effects::<MockProof>(&client).await;

        let restored_deposits = <mongodb::Client as TransactionsDB<MockProof>>::get_mempool_deposits(
            &client,
        )
        .await
        .expect("pending deposits should remain after recovery cleanup");
        assert_eq!(restored_deposits, vec![pending_deposit]);
    }

    #[tokio::test]
    async fn reset_proposer_state_for_replay_returns_error_when_sync_state_delete_fails() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let block = StoredBlock {
            layer2_block_number: 31,
            commitments: vec!["0xreset-delete".to_string()],
            proposer_address: Address::from([31u8; 20]),
        };
        client.store_block(&block).await.expect("store block");
        let sync_state = SyncState::new(
            31,
            block.hash().to_hex_string(),
            L1Ref {
                block_number: 3100,
                tx_hash: TxHash::from([31u8; 32]),
                log_index: 31,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &sync_state).await;

        let _failpoint = TestReplayResetFailpointGuard::enable("delete_sync_state_before_delete");
        let error = reset_proposer_state_for_replay::<MockProof>(&client)
            .await
            .expect_err("sync_state delete failure should abort replay fallback");

        assert!(
            matches!(error, EventHandlerError::IOError(message) if message.contains("sync_state"))
        );
        assert_eq!(client.get_sync_state().await, Some(sync_state));

        let nullifier_metadata_count = client
            .database(DB)
            .collection::<Document>("Nullifiers_metadata")
            .count_documents(doc! {})
            .await
            .expect("count nullifier metadata");
        assert!(nullifier_metadata_count > 0);
    }

    #[tokio::test]
    async fn reset_proposer_state_for_replay_restores_zero_historic_root() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let block = StoredBlock {
            layer2_block_number: 32,
            commitments: vec!["0xreset-zero-root".to_string()],
            proposer_address: Address::from([32u8; 20]),
        };
        client.store_block(&block).await.expect("store block");
        persist_sync_state(
            &client,
            &SyncState::new(
                32,
                block.hash().to_hex_string(),
                L1Ref {
                    block_number: 3200,
                    tx_hash: TxHash::from([32u8; 32]),
                    log_index: 32,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        reset_proposer_state_for_replay::<MockProof>(&client)
            .await
            .expect("replay reset should succeed");

        let historic_root_metadata = client
            .database(DB)
            .collection::<Document>(&format!(
                "{}_metadata",
                <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
            ))
            .find_one(doc! { "_id": 0 })
            .await
            .expect("read historic root metadata")
            .expect("historic root metadata should exist");
        assert_eq!(
            historic_root_metadata.get_i64("sub_tree_count").unwrap_or_default(),
            1,
            "reset replay should restore the zero historic root invariant"
        );
        assert_eq!(client.get_sync_state().await, None);
    }
}
