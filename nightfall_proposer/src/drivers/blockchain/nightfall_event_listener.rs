use crate::{
    domain::entities::{L1Ref, ProposerSnapshotManifest, RestoreJournalPhase, SyncState},
    driven::db::client_transaction_state::restore_all_selected_transactions_to_mempool,
    driven::db::mongo_db::DB,
    driven::db::snapshot::{
        acquire_proposer_state_maintenance_guard, cleanup_after_proposer_shadow_swap,
        find_latest_valid_proposer_snapshot, recover_from_restore_journal,
        restore_proposer_snapshot,
    },
    driven::nightfall_event::get_expected_layer2_blocknumber,
    drivers::blockchain::block_assembly::clear_pending_blocks_queue,
    initialisation::{
        complete_startup_replay_reset, get_block_assembly_status, get_blockchain_client_connection,
        get_db_connection, get_runtime_listener_resume_cursor, get_runtime_listener_start_block,
        set_runtime_listener_resume_cursor, set_runtime_listener_start_block,
        validate_live_proposer_state_consistency,
    },
    ports::{
        contracts::NightfallContract,
        db::{PendingBlockDB, RestoreJournalDB, SyncStateDB, TransactionsDB},
    },
    services::process_events::process_events,
    services::selected_transactions::reconcile_active_client_transaction_lifecycle,
    services::snapshot_scheduler::set_last_snapshot_l2_block,
};
use alloy::{
    primitives::I256,
    rpc::types::{Filter, Log},
    sol_types::{SolEvent, SolEventInterface},
};
use configuration::{addresses::get_addresses, settings::get_settings};
use futures::StreamExt;
use futures::{future::BoxFuture, FutureExt};
use lib::{
    blockchain_client::BlockchainClientConnection,
    error::EventHandlerError,
    log_fetcher::get_logs_paginated,
    nf_client_proof::{Proof, ProvingEngine},
    shared_entities::{SynchronisationPhase::Desynchronized, SynchronisationStatus},
};
use log::{debug, warn};
use mongodb::Client as MongoClient;
use nightfall_bindings::artifacts::Nightfall;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
};
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
            let start_block = if attempts == 1 {
                start_block
            } else {
                get_runtime_listener_start_block().await
            };
            log::info!("Proposer event listener (attempt {attempts}) from L1 block {start_block}...");
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
                    apply_listener_retry_runtime_state().await;
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

fn should_skip_replayed_log(cursor: Option<&L1Ref>, log: &Log) -> bool {
    matches!(
        (cursor, log.block_number, log.log_index),
        (Some(cursor), Some(block_number), Some(log_index))
            if block_number == cursor.block_number && log_index <= cursor.log_index
    )
}

fn listener_start_block_from_sync_state(sync_state: &SyncState) -> usize {
    usize::try_from(sync_state.l1_ref.block_number)
        .expect("Restored L1 block number does not fit into usize")
}

async fn apply_restored_listener_runtime_state(sync_state: &SyncState) -> usize {
    let next_listener_start_block = listener_start_block_from_sync_state(sync_state);
    set_listener_replay_catch_up_pending(true);
    set_runtime_listener_resume_cursor(Some(sync_state.l1_ref.clone())).await;
    set_runtime_listener_start_block(next_listener_start_block).await;
    next_listener_start_block
}

async fn continue_from_restored_sync_state(sync_state: &SyncState) -> usize {
    let next_expected_block = sync_state.last_applied_l2_block.saturating_add(1);
    *get_expected_layer2_blocknumber().await.write().await = I256::try_from(next_expected_block)
        .expect("Restored L2 block number does not fit into I256");
    set_last_snapshot_l2_block(sync_state.last_applied_l2_block).await;
    get_synchronisation_status()
        .await
        .write()
        .await
        .clear_synchronised();
    let next_listener_start_block = apply_restored_listener_runtime_state(sync_state).await;
    warn!(
        "Recovered proposer snapshot state is live. Proposer will remain desynchronised until replay catches up from L1 block {} with next expected L2 block {}",
        sync_state.l1_ref.block_number,
        next_expected_block
    );
    next_listener_start_block
}

fn restored_sync_state_matches_manifest(
    sync_state: &SyncState,
    manifest: &ProposerSnapshotManifest,
) -> bool {
    sync_state.last_applied_l2_block == manifest.last_applied_l2_block
        && sync_state.fingerprint == manifest.fingerprint
        && sync_state.l1_ref == manifest.l1_ref
}

async fn recovered_snapshot_sync_state_if_available(
    db: &MongoClient,
    manifest: &ProposerSnapshotManifest,
) -> Result<Option<SyncState>, String> {
    if let Some(journal) = db.get_restore_journal().await {
        if journal.phase != RestoreJournalPhase::SwapComplete {
            return Ok(None);
        }

        cleanup_after_proposer_shadow_swap(db)
            .await
            .map_err(|error| {
                format!(
                    "restore journal recovery reached swap_complete but cleanup could not finish: {error}"
                )
            })?;
    }

    let Some(sync_state) = db.get_sync_state().await else {
        return Ok(None);
    };

    if !restored_sync_state_matches_manifest(&sync_state, manifest) {
        return Ok(None);
    }

    validate_live_proposer_state_consistency(db)
        .await
        .map_err(|error| {
            format!(
                "restored sync_state matches the snapshot manifest, but live proposer state remains inconsistent: {error}"
            )
        })?;

    Ok(Some(sync_state))
}

async fn restore_recovery_state_may_still_be_present(db: &MongoClient) -> Result<bool, String> {
    if db.get_restore_journal().await.is_some() {
        return Ok(true);
    }

    let collection_names = db
        .database(DB)
        .list_collection_names()
        .await
        .map_err(|error| format!("could not inspect restore recovery collections: {error}"))?;

    Ok(collection_names
        .iter()
        .any(|name| name.starts_with("restore_backup__") || name.starts_with("restore_shadow__")))
}

async fn continue_after_failed_snapshot_restore<P>(
    db: &MongoClient,
    snapshot_dir: &std::path::Path,
    manifest: &ProposerSnapshotManifest,
    restore_error: &str,
    replay_start_block: usize,
) -> usize
where
    P: Proof,
{
    match recover_from_restore_journal(db).await {
        Ok(()) => match recovered_snapshot_sync_state_if_available(db, manifest).await {
            Ok(Some(sync_state)) => {
                warn!(
                    "Snapshot restore from {} returned an error ({}), but restore journal recovery completed the requested snapshot successfully",
                    snapshot_dir.display(),
                    restore_error,
                );
                continue_from_restored_sync_state(&sync_state).await
            }
            Ok(None) => {
                warn!(
                    "Snapshot restore from {} failed: {}. Restore journal recovery did not leave the requested snapshot state live. Falling back to destructive reset+replay from L1 block {}",
                    snapshot_dir.display(),
                    restore_error,
                    replay_start_block
                );
                continue_with_destructive_replay_fallback::<P>(db).await
            }
            Err(recovery_error) => {
                continue_after_incomplete_restore_recovery::<P>(
                    db,
                    snapshot_dir,
                    restore_error,
                    &recovery_error,
                    replay_start_block,
                )
                .await
            }
        },
        Err(recovery_error) => {
            continue_after_incomplete_restore_recovery::<P>(
                db,
                snapshot_dir,
                restore_error,
                &recovery_error.to_string(),
                replay_start_block,
            )
            .await
        }
    }
}

async fn continue_after_incomplete_restore_recovery<P>(
    db: &MongoClient,
    snapshot_dir: &std::path::Path,
    restore_error: &str,
    recovery_error: &str,
    replay_start_block: usize,
) -> usize
where
    P: Proof,
{
    match restore_recovery_state_may_still_be_present(db).await {
        Ok(true) => panic!(
            "Snapshot restore from {} failed: {}. Restore journal recovery also failed or \
             could not finalize safely: {}. restore_journal or restore backup/shadow state \
             may still be present. Manual intervention is required before destructive replay.",
            snapshot_dir.display(),
            restore_error,
            recovery_error
        ),
        Ok(false) => {
            warn!(
                "Snapshot restore from {} failed: {}. Restore journal recovery also failed or \
                 could not finalize safely: {}. No residual restore state remains, so \
                 destructive reset+replay from L1 block {} is allowed.",
                snapshot_dir.display(),
                restore_error,
                recovery_error,
                replay_start_block
            );
            continue_with_destructive_replay_fallback::<P>(db).await
        }
        Err(state_error) => panic!(
            "Snapshot restore from {} failed: {}. Restore journal recovery also failed or \
             could not finalize safely: {}. Additionally, residual restore state could not be \
             inspected safely: {}. Manual intervention is required before destructive replay.",
            snapshot_dir.display(),
            restore_error,
            recovery_error,
            state_error
        ),
    }
}

fn destructive_replay_start_block() -> usize {
    get_settings().genesis_block
}

async fn apply_destructive_replay_listener_runtime_state() -> usize {
    let start_block = destructive_replay_start_block();
    set_listener_replay_catch_up_pending(true);
    set_runtime_listener_resume_cursor(None).await;
    set_runtime_listener_start_block(start_block).await;
    start_block
}

async fn continue_with_destructive_replay_fallback<P>(db: &MongoClient) -> usize
where
    P: Proof,
{
    if let Err(reset_error) = reset_proposer_state_for_replay::<P>(db).await {
        panic!(
            "Proposer replay fallback aborted because reset could not complete safely: {reset_error}"
        );
    }
    set_last_snapshot_l2_block(0).await;
    *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
    apply_destructive_replay_listener_runtime_state().await
}

async fn apply_listener_retry_runtime_state() {
    set_listener_replay_catch_up_pending(true);
    get_synchronisation_status()
        .await
        .write()
        .await
        .clear_synchronised();
    get_block_assembly_status().await.write().await.pause();
}

async fn apply_listener_caught_up_runtime_state() {
    set_listener_replay_catch_up_pending(false);
    get_synchronisation_status()
        .await
        .write()
        .await
        .set_synchronised();
    get_block_assembly_status().await.write().await.resume();
}

fn listener_replay_catch_up_pending_cell() -> &'static AtomicBool {
    static LISTENER_REPLAY_CATCH_UP_PENDING: OnceLock<AtomicBool> = OnceLock::new();
    LISTENER_REPLAY_CATCH_UP_PENDING.get_or_init(|| AtomicBool::new(false))
}

pub(crate) fn is_listener_replay_catch_up_pending() -> bool {
    listener_replay_catch_up_pending_cell().load(Ordering::SeqCst)
}

pub(crate) fn set_listener_replay_catch_up_pending(pending: bool) {
    listener_replay_catch_up_pending_cell().store(pending, Ordering::SeqCst);
}

const LISTENER_CATCH_UP_RETRY_INTERVAL: Duration = Duration::from_secs(5);

async fn reconcile_active_client_transactions_after_listener_catch_up<P>(
    db: &MongoClient,
    current_layer2_block_number: u64,
) where
    P: Proof,
{
    match reconcile_active_client_transaction_lifecycle::<P>(db, current_layer2_block_number).await
    {
        Some(reclassified) if reclassified > 0 => {
            warn!(
                "Listener catch-up reclassified {reclassified} active client transaction(s) \
                 against canonical StoredBlock state at L2 block \
                 {current_layer2_block_number}"
            );
        }
        Some(_) => {}
        None => {
            warn!(
                "Listener catch-up could not reconcile active client transaction lifecycle at \
                 L2 block {current_layer2_block_number}"
            );
        }
    }
}

async fn finalize_listener_catch_up<P, N>()
where
    P: Proof,
    N: NightfallContract,
{
    let current_layer2_block_number =
        match u64::try_from(*get_expected_layer2_blocknumber().await.read().await) {
            Ok(block_number) => block_number,
            Err(_) => {
                warn!(
                    "Listener catch-up could not derive the current L2 block number from \
                 expected_layer2_blocknumber; keeping proposer desynchronised until replay can \
                 continue"
                );
                return;
            }
        };

    let current_onchain_layer2_block_number = match N::get_current_layer2_blocknumber().await {
        Ok(block_number) => block_number,
        Err(error) => {
            warn!(
                "Listener catch-up could not fetch the current on-chain L2 block number: \
                 {error}. Keeping proposer desynchronised until replay can continue"
            );
            return;
        }
    };
    let current_onchain_layer2_block_number =
        match u64::try_from(current_onchain_layer2_block_number) {
            Ok(block_number) => block_number,
            Err(_) => {
                warn!(
                    "Listener catch-up could not convert the current on-chain L2 block number \
                     into u64; keeping proposer desynchronised until replay can continue"
                );
                return;
            }
        };

    if current_layer2_block_number != current_onchain_layer2_block_number {
        warn!(
            "Listener catch-up reached in-memory next expected L2 block \
             {current_layer2_block_number} but on-chain next expected L2 block is \
             {current_onchain_layer2_block_number}. Keeping proposer desynchronised until replay \
             fully catches up"
        );
        return;
    }

    let db = get_db_connection().await;
    reconcile_active_client_transactions_after_listener_catch_up::<P>(
        db,
        current_layer2_block_number,
    )
    .await;
    apply_listener_caught_up_runtime_state().await;
}

async fn retry_listener_catch_up_if_pending<P, N>()
where
    P: Proof,
    N: NightfallContract,
{
    if is_listener_replay_catch_up_pending() {
        finalize_listener_catch_up::<P, N>().await;
    }
}

async fn advance_runtime_listener_state_from_log(log: &Log) {
    let (Some(block_number), Some(tx_hash), Some(log_index)) =
        (log.block_number, log.transaction_hash, log.log_index)
    else {
        return;
    };
    let Ok(start_block) = usize::try_from(block_number) else {
        return;
    };

    set_runtime_listener_resume_cursor(Some(L1Ref {
        block_number,
        tx_hash,
        log_index,
    }))
    .await;
    set_runtime_listener_start_block(start_block).await;
}

async fn process_listener_log<P, E, N>(
    log: Log,
    restart_start_block: usize,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let event = match Nightfall::NightfallEvents::decode_log(&log.inner) {
        Ok(event) => event,
        Err(error) => {
            warn!("Failed to decode log: {error:?}");
            return Ok(());
        }
    };

    match process_events::<P, E, N>(event.data, log.clone()).await {
        Ok(_) => {
            advance_runtime_listener_state_from_log(&log).await;
            Ok(())
        }
        Err(EventHandlerError::MissingBlocks(n)) => {
            warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
            restart_event_listener::<P, E, N>(restart_start_block).await;
            Err(EventHandlerError::StreamTerminated)
        }
        Err(EventHandlerError::BlockHashError(expected, found)) => {
            warn!(
                "Block hash mismatch: expected {expected:?}, found {found:?}. Restarting event listener"
            );
            restart_event_listener::<P, E, N>(restart_start_block).await;
            Err(EventHandlerError::StreamTerminated)
        }
        Err(error) => {
            warn!("Event listener hit a recoverable processing error: {error:?}");
            Err(error)
        }
    }
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<P, E, N>(start_block: usize) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    set_listener_replay_catch_up_pending(true);
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
    let replay_resume_cursor = get_runtime_listener_resume_cursor().await;

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
                if should_skip_replayed_log(replay_resume_cursor.as_ref(), &evt) {
                    continue;
                }
                process_listener_log::<P, E, N>(evt, start_block).await?;
            }
        } else {
            println!( "Start block {start_block} is greater than latest block {latest_block}. No past events to process.");
        }
    }

    let mut events_stream = events_subscription.into_stream();

    loop {
        match events_stream.next().now_or_never() {
            Some(Some(log)) => {
                process_listener_log::<P, E, N>(log, start_block).await?;
            }
            Some(None) => return Err(EventHandlerError::StreamTerminated),
            None => break,
        }
    }

    finalize_listener_catch_up::<P, N>().await;

    loop {
        tokio::select! {
            maybe_log = events_stream.next() => {
                let Some(log) = maybe_log else {
                    return Err(EventHandlerError::StreamTerminated);
                };
                process_listener_log::<P, E, N>(log, start_block).await?;
                retry_listener_catch_up_if_pending::<P, N>().await;
            }
            _ = sleep(LISTENER_CATCH_UP_RETRY_INTERVAL), if is_listener_replay_catch_up_pending() => {
                retry_listener_catch_up_if_pending::<P, N>().await;
            }
        }
    }
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
/// `Selected` transactions are released back to the mempool so replay can rebuild
/// canonical transaction lifecycle from scratch.
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

    let restored_selected = restore_all_selected_transactions_to_mempool(db).await;
    debug!(
        "Selected transaction recovery after restart restored {} transaction(s) to the mempool.",
        restored_selected.unwrap_or(0)
    );
}

async fn clear_all_reserved_deposits_for_recovery<P>(
    db: &MongoClient,
) -> Result<u64, EventHandlerError>
where
    P: Proof,
{
    TransactionsDB::<P>::clear_all_mempool_deposit_reservations(db)
        .await
        .ok_or_else(|| {
            EventHandlerError::IOError(
                "Could not clear reserved deposits during proposer recovery".to_string(),
            )
        })
}

#[cfg_attr(not(test), allow(dead_code))]
async fn cleanup_persisted_pending_blocks_for_recovery<P>(
    db: &MongoClient,
) -> Result<(u64, u64), EventHandlerError>
where
    P: Proof,
{
    let deleted_pending_blocks = db.delete_all_pending_blocks().await.ok_or_else(|| {
        EventHandlerError::IOError(
            "Could not delete persisted pending blocks during recovery".to_string(),
        )
    })?;
    let unreserved_deposits = clear_all_reserved_deposits_for_recovery::<P>(db).await?;
    if deleted_pending_blocks > 0 || unreserved_deposits > 0 {
        warn!(
            "Recovery discarded {deleted_pending_blocks} persisted pending block(s) and cleared {unreserved_deposits} reserved deposit selection(s)"
        );
    }
    Ok((deleted_pending_blocks, unreserved_deposits))
}

/// Resets proposer state for the destructive replay fallback, including tree state and
/// persisted sync_state, before historical events are replayed.
async fn reset_proposer_state_for_replay<P>(db: &MongoClient) -> Result<(), EventHandlerError>
where
    P: Proof,
{
    let _maintenance_guard = acquire_proposer_state_maintenance_guard().await;
    maybe_fail_replay_reset("delete_sync_state_before_delete")?;
    complete_startup_replay_reset(db)
        .await
        .map_err(EventHandlerError::IOError)?;
    cleanup_recovery_side_effects::<P>(db).await;
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
pub async fn restart_event_listener<P, E, N>(_start_block: usize)
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
    let replay_start_block = destructive_replay_start_block();
    let next_listener_start_block = match find_latest_valid_proposer_snapshot(
        &snapshot_root_dir,
        max_restorable_l2_block,
    )
    .await
    {
        Ok(Some((snapshot_dir, manifest))) => {
            warn!(
                "Desynchronised proposer state detected. Attempting snapshot restore from {} (L2 block {}) before falling back to reset+replay",
                snapshot_dir.display(),
                manifest.last_applied_l2_block
            );
            match restore_proposer_snapshot(db, &snapshot_dir).await {
                Ok(sync_state) => continue_from_restored_sync_state(&sync_state).await,
                Err(error) => {
                    continue_after_failed_snapshot_restore::<P>(
                        db,
                        &snapshot_dir,
                        &manifest,
                        &error.to_string(),
                        replay_start_block,
                    )
                    .await
                }
            }
        }
        Ok(None) => {
            warn!(
                "No valid proposer snapshot found under {} for L2 recovery point <= {}. Falling back to destructive reset+replay from L1 block {}",
                snapshot_root_dir.display(),
                max_restorable_l2_block,
                replay_start_block
            );
            continue_with_destructive_replay_fallback::<P>(db).await
        }
        Err(error) => {
            warn!(
                "Snapshot discovery failed under {}: {}. Falling back to destructive reset+replay from L1 block {}",
                snapshot_root_dir.display(),
                error,
                replay_start_block
            );
            continue_with_destructive_replay_fallback::<P>(db).await
        }
    };
    debug!(
        "Prepared proposer listener recovery runtime state. Outer retry loop will resume from \
         L1 block {next_listener_start_block}"
    );
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
        domain::entities::{
            Block, ClientTransactionWithMetaData, DepositDatawithFee, PendingBlock,
            PendingBlockState, RestoreJournalPhase, TxLifecycle,
        },
        driven::db::mongo_db::{
            ensure_deposit_indexes, StoredBlock, DB, PROPOSED_BLOCKS_COLLECTION,
        },
        driven::db::snapshot::{
            create_proposer_snapshot, load_proposer_snapshot_into_shadow, snapshot_test_lock,
            swap_proposer_shadow_into_live,
        },
        drivers::blockchain::block_assembly::{
            pending_blocks_queue_len_for_test, push_pending_block_for_test,
        },
        initialisation::get_runtime_listener_start_block,
        ports::db::{
            BlockStorageDB, PendingBlockDB, RestoreJournalDB, SyncStateDB, TransactionsDB,
        },
        ports::trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    };
    use alloy::primitives::Bytes;
    use alloy::primitives::{Address, TxHash};
    use alloy::rpc::types::Log as RpcLog;
    use ark_bn254::Fr as Fr254;
    use ark_ff::Zero;
    use ark_serialize::SerializationError;
    use lib::hex_conversion::HexConvertible;
    use lib::merkle_trees::trees::{MutableTree, TreeMetadata};
    use lib::nf_client_proof::Proof;
    use lib::shared_entities::{ClientTransaction, CompressedSecrets, DepositData};
    use lib::tests_utils::{get_db_connection, get_mongo};
    use mongodb::bson::{doc, Document};
    use serde::{Deserialize, Serialize};
    use tokio::sync::{Mutex, OnceCell};

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

    async fn event_listener_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
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
            .expect("stored block should exist for event listener test sync_state");

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
            .expect("materialize commitment tree state for event listener test");
        }

        let target_historic_root_sub_tree_count = sync_state
            .last_applied_l2_block
            .checked_add(2)
            .expect("historic root count should not overflow in event listener tests");
        let mut historic_root_sub_tree_count = client
            .database(DB)
            .collection::<TreeMetadata<Fr254>>(&format!(
                "{}_metadata",
                <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
            ))
            .find_one(doc! { "_id": 0 })
            .await
            .expect("read historic root metadata")
            .expect("historic root metadata should exist for event listener tests")
            .sub_tree_count;
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
            .expect("materialize historic root state for event listener test");
            historic_root_sub_tree_count += 1;
        }
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

    #[test]
    fn should_skip_replayed_log_skips_lower_log_index_in_same_block() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([1u8; 32]),
            log_index: 8,
        };
        let log = RpcLog {
            block_number: Some(100),
            log_index: Some(7),
            ..RpcLog::default()
        };

        assert!(should_skip_replayed_log(Some(&cursor), &log));
    }

    #[test]
    fn should_skip_replayed_log_skips_equal_log_index_in_same_block() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([2u8; 32]),
            log_index: 8,
        };
        let log = RpcLog {
            block_number: Some(100),
            log_index: Some(8),
            ..RpcLog::default()
        };

        assert!(should_skip_replayed_log(Some(&cursor), &log));
    }

    #[test]
    fn should_skip_replayed_log_keeps_higher_log_index_in_same_block() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([3u8; 32]),
            log_index: 8,
        };
        let log = RpcLog {
            block_number: Some(100),
            log_index: Some(9),
            ..RpcLog::default()
        };

        assert!(!should_skip_replayed_log(Some(&cursor), &log));
    }

    #[test]
    fn should_skip_replayed_log_keeps_logs_from_other_blocks() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([4u8; 32]),
            log_index: 8,
        };
        let earlier_log = RpcLog {
            block_number: Some(99),
            log_index: Some(99),
            ..RpcLog::default()
        };
        let later_log = RpcLog {
            block_number: Some(101),
            log_index: Some(0),
            ..RpcLog::default()
        };

        assert!(!should_skip_replayed_log(Some(&cursor), &earlier_log));
        assert!(!should_skip_replayed_log(Some(&cursor), &later_log));
    }

    #[test]
    fn should_skip_replayed_log_keeps_logs_with_missing_log_index() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([5u8; 32]),
            log_index: 8,
        };
        let log = RpcLog {
            block_number: Some(100),
            log_index: None,
            ..RpcLog::default()
        };

        assert!(!should_skip_replayed_log(Some(&cursor), &log));
    }

    #[test]
    fn should_skip_replayed_log_keeps_logs_with_missing_block_number() {
        let cursor = L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([6u8; 32]),
            log_index: 8,
        };
        let log = RpcLog {
            block_number: None,
            log_index: Some(8),
            ..RpcLog::default()
        };

        assert!(!should_skip_replayed_log(Some(&cursor), &log));
    }

    #[test]
    fn should_skip_replayed_log_keeps_logs_without_cursor() {
        let log = RpcLog {
            block_number: Some(100),
            log_index: Some(8),
            ..RpcLog::default()
        };

        assert!(!should_skip_replayed_log(None, &log));
    }

    #[tokio::test]
    async fn apply_restored_listener_runtime_state_sets_resume_cursor_from_sync_state() {
        let _lock = event_listener_test_lock().await;
        set_listener_replay_catch_up_pending(false);
        set_runtime_listener_resume_cursor(None).await;
        set_runtime_listener_start_block(0).await;

        let sync_state = SyncState::new(
            8,
            "fingerprint".to_string(),
            L1Ref {
                block_number: 100,
                tx_hash: TxHash::from([7u8; 32]),
                log_index: 8,
            },
            mongodb::bson::DateTime::now(),
        );

        let start_block = apply_restored_listener_runtime_state(&sync_state).await;

        assert_eq!(start_block, 100);
        assert_eq!(get_runtime_listener_start_block().await, 100);
        assert_eq!(
            get_runtime_listener_resume_cursor().await,
            Some(sync_state.l1_ref.clone())
        );
        assert!(is_listener_replay_catch_up_pending());
    }

    #[tokio::test]
    async fn apply_destructive_replay_listener_runtime_state_clears_resume_cursor() {
        let _lock = event_listener_test_lock().await;
        set_listener_replay_catch_up_pending(false);
        set_runtime_listener_resume_cursor(Some(L1Ref {
            block_number: 100,
            tx_hash: TxHash::from([8u8; 32]),
            log_index: 8,
        }))
        .await;
        set_runtime_listener_start_block(100).await;

        let start_block = apply_destructive_replay_listener_runtime_state().await;

        assert_eq!(start_block, destructive_replay_start_block());
        assert_eq!(get_runtime_listener_start_block().await, start_block);
        assert_eq!(get_runtime_listener_resume_cursor().await, None);
        assert!(is_listener_replay_catch_up_pending());
    }

    #[tokio::test]
    async fn advance_runtime_listener_state_from_log_updates_cursor_and_start_block() {
        let _lock = event_listener_test_lock().await;
        set_runtime_listener_resume_cursor(None).await;
        set_runtime_listener_start_block(0).await;

        let log = RpcLog {
            block_number: Some(321),
            transaction_hash: Some(TxHash::from([9u8; 32])),
            log_index: Some(12),
            ..RpcLog::default()
        };

        advance_runtime_listener_state_from_log(&log).await;

        assert_eq!(get_runtime_listener_start_block().await, 321);
        assert_eq!(
            get_runtime_listener_resume_cursor().await,
            Some(L1Ref {
                block_number: 321,
                tx_hash: TxHash::from([9u8; 32]),
                log_index: 12,
            })
        );
    }

    #[tokio::test]
    async fn advance_runtime_listener_state_from_log_ignores_incomplete_logs() {
        let _lock = event_listener_test_lock().await;
        let original_cursor = L1Ref {
            block_number: 400,
            tx_hash: TxHash::from([4u8; 32]),
            log_index: 4,
        };
        set_runtime_listener_resume_cursor(Some(original_cursor.clone())).await;
        set_runtime_listener_start_block(400).await;

        let incomplete_log = RpcLog {
            block_number: Some(401),
            transaction_hash: None,
            log_index: Some(5),
            ..RpcLog::default()
        };

        advance_runtime_listener_state_from_log(&incomplete_log).await;

        assert_eq!(get_runtime_listener_start_block().await, 400);
        assert_eq!(
            get_runtime_listener_resume_cursor().await,
            Some(original_cursor)
        );
    }

    #[tokio::test]
    async fn apply_listener_retry_runtime_state_clears_sync_and_pauses_assembly() {
        let _lock = event_listener_test_lock().await;
        set_listener_replay_catch_up_pending(false);
        get_synchronisation_status()
            .await
            .write()
            .await
            .set_synchronised();
        get_block_assembly_status().await.write().await.resume();

        apply_listener_retry_runtime_state().await;

        assert!(!get_synchronisation_status()
            .await
            .read()
            .await
            .is_synchronised());
        assert!(!get_block_assembly_status().await.read().await.is_running());
        assert!(is_listener_replay_catch_up_pending());
    }

    #[tokio::test]
    async fn apply_listener_caught_up_runtime_state_sets_sync_and_resumes_assembly() {
        let _lock = event_listener_test_lock().await;
        set_listener_replay_catch_up_pending(true);
        get_synchronisation_status()
            .await
            .write()
            .await
            .clear_synchronised();
        get_block_assembly_status().await.write().await.pause();

        apply_listener_caught_up_runtime_state().await;

        assert!(get_synchronisation_status()
            .await
            .read()
            .await
            .is_synchronised());
        assert!(get_block_assembly_status().await.read().await.is_running());
        assert!(!is_listener_replay_catch_up_pending());
    }

    fn test_selected_client_transaction(
        seed: u32,
        block_l2: u64,
    ) -> ClientTransactionWithMetaData<MockProof> {
        ClientTransactionWithMetaData {
            client_transaction: ClientTransaction {
                commitments: [
                    Fr254::from(u64::from(seed) + 100),
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

    fn test_pending_deposit(seed: u64, reserved: bool) -> DepositDatawithFee {
        DepositDatawithFee {
            fee: Fr254::from(seed),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(seed + 1),
                nf_slot_id: Fr254::from(seed + 2),
                value: Fr254::from(seed + 3),
                secret_hash: Fr254::from(seed + 4),
            },
            reserved,
        }
    }

    #[test]
    fn restored_sync_state_matches_manifest_requires_exact_restore_identity() {
        let sync_state = SyncState::new(
            11,
            "fingerprint-11".to_string(),
            L1Ref {
                block_number: 1100,
                tx_hash: TxHash::from([11u8; 32]),
                log_index: 11,
            },
            mongodb::bson::DateTime::now(),
        );
        let manifest = ProposerSnapshotManifest::new(
            "snapshot-11".to_string(),
            mongodb::bson::DateTime::now(),
            DB.to_string(),
            &sync_state,
            Vec::new(),
            "overall".to_string(),
        );

        assert!(restored_sync_state_matches_manifest(&sync_state, &manifest));

        let different_fingerprint = SyncState {
            fingerprint: "different".to_string(),
            ..sync_state.clone()
        };
        assert!(!restored_sync_state_matches_manifest(
            &different_fingerprint,
            &manifest
        ));

        let different_l1_ref = SyncState {
            l1_ref: L1Ref {
                log_index: 12,
                ..sync_state.l1_ref.clone()
            },
            ..sync_state.clone()
        };
        assert!(!restored_sync_state_matches_manifest(
            &different_l1_ref,
            &manifest
        ));
    }

    #[tokio::test]
    async fn recovered_snapshot_sync_state_if_available_finishes_swap_complete_cleanup() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let block = StoredBlock {
            layer2_block_number: 25,
            commitments: vec!["0xswap-complete-recovery".to_string()],
            proposer_address: Address::from([25u8; 20]),
        };
        client.store_block(&block).await.expect("store block");
        let sync_state = SyncState::new(
            25,
            block.hash().to_hex_string(),
            L1Ref {
                block_number: 2500,
                tx_hash: TxHash::from([25u8; 32]),
                log_index: 25,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-event-listener-snapshot-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap snapshot into live");

        let journal = client
            .get_restore_journal()
            .await
            .expect("swap_complete journal should exist");
        assert_eq!(journal.phase, RestoreJournalPhase::SwapComplete);

        let recovered_sync_state = recovered_snapshot_sync_state_if_available(&client, &manifest)
            .await
            .expect("recover helper should succeed")
            .expect("recover helper should surface restored sync_state");

        assert_eq!(recovered_sync_state, sync_state);
        assert_eq!(client.get_restore_journal().await, None);
    }

    #[tokio::test]
    async fn recovered_snapshot_sync_state_if_available_rejects_inconsistent_live_state() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let block = StoredBlock {
            layer2_block_number: 31,
            commitments: vec!["0xinconsistent-live-state".to_string()],
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

        let manifest = ProposerSnapshotManifest::new(
            "snapshot-31".to_string(),
            mongodb::bson::DateTime::now(),
            DB.to_string(),
            &sync_state,
            Vec::new(),
            "overall".to_string(),
        );

        client
            .database(DB)
            .collection::<Document>(&format!(
                "{}_metadata",
                <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME
            ))
            .drop()
            .await
            .expect("drop nullifier metadata to simulate inconsistent live state");

        let error = recovered_snapshot_sync_state_if_available(&client, &manifest)
            .await
            .expect_err("helper should reject matching sync_state when live state is inconsistent");

        assert!(error.contains("live proposer state remains inconsistent"));
    }

    #[tokio::test]
    async fn continue_after_failed_snapshot_restore_aborts_when_recovery_leaves_restore_state() {
        let _lock = event_listener_test_lock().await;
        let _snapshot_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let snapshot_block = StoredBlock {
            layer2_block_number: 31,
            commitments: vec!["0xlistener-restore-abort".to_string()],
            proposer_address: Address::from([31u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 3100,
                tx_hash: TxHash::from([31u8; 32]),
                log_index: 31,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-event-listener-restart-abort-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 32,
            commitments: Vec::new(),
            proposer_address: Address::from([32u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 3200,
                tx_hash: TxHash::from([32u8; 32]),
                log_index: 32,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap snapshot into live");

        client
            .delete_block_by_number(snapshot_block.layer2_block_number)
            .await
            .expect("delete restored stored block");
        client
            .database(DB)
            .collection::<Document>(&format!("restore_backup__{PROPOSED_BLOCKS_COLLECTION}"))
            .drop()
            .await
            .expect("drop required backup collection");

        let client_for_restart = client.clone();
        let snapshot_dir_for_restart = snapshot_dir.clone();
        let manifest_for_restart = manifest.clone();
        let join_error = tokio::spawn(async move {
            continue_after_failed_snapshot_restore::<MockProof>(
                &client_for_restart,
                &snapshot_dir_for_restart,
                &manifest_for_restart,
                "forced restore failure",
                destructive_replay_start_block(),
            )
            .await
        })
        .await
        .expect_err("restart helper should abort instead of entering destructive replay");

        assert!(join_error.is_panic(), "expected panic, got {join_error:?}");
        assert!(
            client.get_restore_journal().await.is_some(),
            "stale restore_journal should remain for manual inspection"
        );
        assert!(
            client.get_sync_state().await.is_some(),
            "destructive replay must not wipe sync_state after fail-closed abort"
        );

        tokio::fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
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

        clear_pending_blocks_queue().await;
        get_block_assembly_status().await.write().await.resume();
        push_pending_block_for_test(PendingBlock {
            layer2_block_number: 0,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: Vec::new(),
            selected_client_transaction_hashes: Vec::new(),
        })
        .await;
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
            reserved: false,
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![pending_deposit],
        )
        .await
        .expect("store pending deposit");

        cleanup_recovery_side_effects::<MockProof>(&client).await;

        let restored_deposits =
            <mongodb::Client as TransactionsDB<MockProof>>::get_mempool_deposits(&client)
                .await
                .expect("pending deposits should remain after recovery cleanup");
        assert_eq!(restored_deposits, vec![pending_deposit]);
    }

    #[tokio::test]
    async fn cleanup_recovery_side_effects_restores_selected_transactions_to_mempool() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let selected_transaction = test_selected_client_transaction(77, 12);
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store selected transaction");

        cleanup_recovery_side_effects::<MockProof>(&client).await;

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
    async fn listener_catch_up_reconciliation_restores_orphaned_selected_transactions() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let selected_transaction = test_selected_client_transaction(88, 7);
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store selected transaction");

        reconcile_active_client_transactions_after_listener_catch_up::<MockProof>(&client, 8).await;

        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect(
                "selected transaction should still exist after listener catch-up reconciliation"
            )
            .lifecycle
            .is_mempool()
        );
    }

    #[tokio::test]
    async fn cleanup_persisted_pending_blocks_for_recovery_deletes_rows_and_unreserves_deposits() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let reserved_deposit = test_pending_deposit(50, true);
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![reserved_deposit],
        )
        .await
        .expect("store reserved deposit");
        client
            .store_pending_block(&PendingBlock {
                layer2_block_number: 50,
                state: PendingBlockState::ReadyToPropose,
                broadcast_tx_hash: None,
                broadcast_receipt_checks: 0,
                block: None,
                selected_deposits: vec![vec![reserved_deposit]],
                selected_client_transaction_hashes: Vec::new(),
            })
            .await
            .expect("store stale pending block");

        let (deleted_count, unreserved_count) =
            cleanup_persisted_pending_blocks_for_recovery::<MockProof>(&client)
                .await
                .expect("cleanup stale persisted pending blocks");
        assert_eq!(deleted_count, 1);
        assert_eq!(unreserved_count, 1);
        assert!(client
            .get_all_pending_blocks()
            .await
            .expect("read pending blocks after cleanup")
            .is_empty());

        let restored_deposits =
            <mongodb::Client as TransactionsDB<MockProof>>::get_mempool_deposits(&client)
                .await
                .expect("reserved deposit should be visible again");
        assert_eq!(
            restored_deposits,
            vec![DepositDatawithFee {
                reserved: false,
                ..reserved_deposit
            }]
        );
    }

    #[tokio::test]
    async fn cleanup_persisted_pending_blocks_for_recovery_unreserves_without_pending_rows() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let reserved_deposit = test_pending_deposit(60, true);
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![reserved_deposit],
        )
        .await
        .expect("store stranded reserved deposit");

        let (deleted_count, unreserved_count) =
            cleanup_persisted_pending_blocks_for_recovery::<MockProof>(&client)
                .await
                .expect("cleanup should clear stranded reservations");
        assert_eq!(deleted_count, 0);
        assert_eq!(unreserved_count, 1);

        let restored_deposits =
            <mongodb::Client as TransactionsDB<MockProof>>::get_mempool_deposits(&client)
                .await
                .expect("reserved deposit should be visible again");
        assert_eq!(
            restored_deposits,
            vec![DepositDatawithFee {
                reserved: false,
                ..reserved_deposit
            }]
        );
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
            matches!(error, EventHandlerError::IOError(message) if message.contains("canonical state"))
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
            historic_root_metadata
                .get_i64("sub_tree_count")
                .unwrap_or_default(),
            1,
            "reset replay should restore the zero historic root invariant"
        );
        assert_eq!(client.get_sync_state().await, None);
    }

    #[tokio::test]
    async fn reset_proposer_state_for_replay_removes_all_stale_proposed_blocks() {
        let _lock = event_listener_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_test_trees(&client).await;
        for block_number in [40_u64, 41_u64, 42_u64] {
            let block = StoredBlock {
                layer2_block_number: block_number,
                commitments: vec![format!("0xstale-{block_number}")],
                proposer_address: Address::from([block_number as u8; 20]),
            };
            client.store_block(&block).await.expect("store stale block");
            if block_number == 42 {
                persist_sync_state(
                    &client,
                    &SyncState::new(
                        block_number,
                        block.hash().to_hex_string(),
                        L1Ref {
                            block_number: 4200,
                            tx_hash: TxHash::from([42_u8; 32]),
                            log_index: 42,
                        },
                        mongodb::bson::DateTime::now(),
                    ),
                )
                .await;
            }
        }

        reset_proposer_state_for_replay::<MockProof>(&client)
            .await
            .expect("replay reset should clear stale stored blocks");

        assert!(
            client
                .get_all_blocks()
                .await
                .expect("read stored blocks after reset")
                .is_empty(),
            "replay reset should remove all stale stored blocks before replay begins"
        );
    }
}
