use crate::{
    driven::db::snapshot::{
        find_latest_valid_proposer_snapshot, recover_from_restore_journal,
        restore_proposer_snapshot,
    },
    driven::nightfall_event::get_expected_layer2_blocknumber,
    initialisation::{get_block_assembly_status, set_runtime_listener_start_block},
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        contracts::NightfallContract,
        db::{SyncStateDB, TransactionsDB},
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
    services::process_events::process_events,
    services::selected_transactions::reconcile_obviously_orphaned_selected_transactions,
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
async fn cleanup_recovery_side_effects<P>(db: &MongoClient)
where
    P: Proof,
{
    let removed_deposits = TransactionsDB::<P>::remove_all_mempool_deposits(db).await;
    let removed_client_txs = TransactionsDB::<P>::remove_all_mempool_client_transactions(db).await;

    debug!(
        "Mempool cleanup: removed {} deposits and {} client transactions.",
        removed_deposits.unwrap_or(0),
        removed_client_txs.unwrap_or(0)
    );

    let restored_selected = reconcile_obviously_orphaned_selected_transactions::<P>(db).await;
    debug!(
        "Selected transaction recovery after restart restored {} orphaned transaction(s).",
        restored_selected.unwrap_or(0)
    );
}

/// Resets proposer state for the destructive replay fallback, including tree state and
/// persisted sync_state, before historical events are replayed.
async fn reset_proposer_state_for_replay<P>(db: &MongoClient)
where
    P: Proof,
{
    let _ = <MongoClient as CommitmentTree<Fr254>>::reset_tree(db).await;
    let _ = <MongoClient as HistoricRootTree<Fr254>>::reset_tree(db).await;
    let _ = <MongoClient as NullifierTree<Fr254>>::reset_tree(db).await;
    if let Ok(mut session) = db.start_session().await {
        let db_for_cleanup = db.clone();
        let _ = session
            .start_transaction()
            .and_run2(async move |session| {
                db_for_cleanup
                    .delete_sync_state_with_session(session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await;
    }
    cleanup_recovery_side_effects::<P>(db).await;
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
                    reset_proposer_state_for_replay::<P>(db).await;
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
            reset_proposer_state_for_replay::<P>(db).await;
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
            reset_proposer_state_for_replay::<P>(db).await;
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
