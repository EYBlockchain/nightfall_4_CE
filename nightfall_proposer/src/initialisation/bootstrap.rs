use super::{
    cleanup::resume_startup_replay_reset_if_needed,
    consistency::{
        ahead_of_chain_error, missing_stored_block_error,
        startup_legacy_transaction_migration_error, validate_startup_proposer_state_consistency,
        validate_sync_state_against_block, validate_tree_state_against_sync_state,
    },
    db::ensure_proposer_db_initialized,
    get_raw_db_connection,
    runtime_listener::{get_listener_start_block, set_runtime_listener_resume_cursor},
};
use crate::{
    driven::db::{
        client_transaction_state::backfill_legacy_client_transaction_lifecycle,
        snapshot::recover_from_restore_journal,
    },
    driven::nightfall_event::get_expected_layer2_blocknumber,
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    ports::{
        contracts::NightfallContract,
        db::{BlockStorageDB, RestoreJournalDB, StartupReplayResetMarkerDB, SyncStateDB},
    },
    services::snapshot_scheduler::initialize_snapshot_scheduler_state,
};
use alloy::primitives::I256;
use log::{info, warn};
use mongodb::Client;

pub(super) async fn backfill_legacy_client_transactions_at_startup(
    client: &Client,
) -> Result<(), String> {
    let stats = backfill_legacy_client_transaction_lifecycle(client)
        .await
        .map_err(|error| startup_legacy_transaction_migration_error(&error))?;

    if stats.total_backfilled() > 0 {
        warn!(
            "Backfilled {} legacy proposer client transaction lifecycle field(s) at startup \
             (mempool={}, selected={}, included={}, cancelled={}, dropped={})",
            stats.total_backfilled(),
            stats.mempool_backfilled,
            stats.selected_backfilled,
            stats.included_backfilled,
            stats.cancelled_backfilled,
            stats.dropped_backfilled
        );
    }

    Ok(())
}

pub(super) async fn bootstrap_proposer_startup_state_with_db<N>(
    db: &Client,
    initialize_snapshot_scheduler: bool,
) -> Result<(), String>
where
    N: NightfallContract,
{
    backfill_legacy_client_transactions_at_startup(db).await?;

    if let Err(error) = recover_from_restore_journal(db).await {
        if db.get_restore_journal().await.is_none() {
            warn!(
                "Proposer restore recovery completed by rolling back incomplete restore state before bootstrap: {error}"
            );
        } else {
            return Err(format!(
                "Proposer restore recovery failed before bootstrap: {error}"
            ));
        }
    }
    if resume_startup_replay_reset_if_needed(db).await? {
        warn!(
            "Proposer startup resumed an interrupted startup replay reset before bootstrap validation"
        );
    }
    if initialize_snapshot_scheduler {
        if let Err(error) = initialize_snapshot_scheduler_state().await {
            warn!("Could not initialize proposer snapshot scheduler state: {error}");
        }
    }

    validate_startup_proposer_state_consistency(db).await?;

    let onchain_next_block_i256 = N::get_current_layer2_blocknumber()
        .await
        .map_err(|e| format!("Could not fetch current L2 block number: {e}"))?;

    if onchain_next_block_i256 < I256::ZERO {
        return Err(format!(
            "Nightfall returned negative current L2 block number {onchain_next_block_i256}"
        ));
    }

    let onchain_next_block: u64 = onchain_next_block_i256
        .try_into()
        .map_err(|_| "Current L2 block number does not fit into u64".to_string())?;

    let mut expected_block_number = get_expected_layer2_blocknumber().await.write().await;
    let mut sync_status = get_synchronisation_status().await.write().await;
    let mut listener_start_block = get_listener_start_block().await.write().await;

    *listener_start_block = configuration::settings::get_settings().genesis_block;

    match db.get_sync_state().await {
        Some(sync_state) => {
            let stored_block = db
                .get_block_by_number(sync_state.last_applied_l2_block)
                .await
                .ok_or_else(|| missing_stored_block_error(sync_state.last_applied_l2_block))?;

            validate_sync_state_against_block(&sync_state, &stored_block)?;
            validate_tree_state_against_sync_state(db, Some(&sync_state), Some(&stored_block))
                .await?;

            let next_expected_block =
                sync_state
                    .last_applied_l2_block
                    .checked_add(1)
                    .ok_or_else(|| {
                        "last_applied_l2_block overflowed when computing next expected block"
                            .to_string()
                    })?;

            if next_expected_block > onchain_next_block {
                return Err(ahead_of_chain_error(
                    next_expected_block,
                    onchain_next_block,
                ));
            }

            *expected_block_number = I256::try_from(next_expected_block)
                .map_err(|_| "Next expected L2 block number does not fit into I256".to_string())?;

            let sync_state_l1_block_number: usize =
                sync_state.l1_ref.block_number.try_into().map_err(|_| {
                    format!(
                        "sync_state L1 block number {} does not fit into usize",
                        sync_state.l1_ref.block_number
                    )
                })?;
            *listener_start_block = sync_state_l1_block_number;
            set_runtime_listener_resume_cursor(Some(sync_state.l1_ref.clone())).await;

            sync_status.clear_synchronised();
            if next_expected_block == onchain_next_block {
                info!(
                    "Recovered proposer state at L2 tip {} from sync_state; proposer will remain desynchronised until replay from L1 block {} confirms catch-up",
                    sync_state.last_applied_l2_block,
                    sync_state.l1_ref.block_number
                );
            } else {
                info!(
                    "Recovered proposer state at L2 block {}, behind chain tip {}; listener replay will start from L1 block {}",
                    sync_state.last_applied_l2_block,
                    onchain_next_block.saturating_sub(1),
                    sync_state.l1_ref.block_number
                );
            }
        }
        None => {
            set_runtime_listener_resume_cursor(None).await;
            validate_tree_state_against_sync_state(db, None, None).await?;
            *expected_block_number = I256::ZERO;
            if onchain_next_block == 0 {
                sync_status.set_synchronised();
                info!("No proposer sync_state found and chain is at genesis; starting fresh");
            } else {
                sync_status.clear_synchronised();
                warn!(
                    "No proposer sync_state found while chain is already at L2 block {}; replay is required",
                    onchain_next_block.saturating_sub(1)
                );
            }
        }
    }

    Ok(())
}

pub async fn bootstrap_proposer_startup_state<N>() -> Result<(), String>
where
    N: NightfallContract,
{
    let db = get_raw_db_connection().await;
    recover_then_initialize_proposer_db(db).await?;
    bootstrap_proposer_startup_state_with_db::<N>(db, true).await
}

pub(super) async fn recover_then_initialize_proposer_db(db: &Client) -> Result<(), String> {
    backfill_legacy_client_transactions_at_startup(db).await?;
    if let Err(error) = recover_from_restore_journal(db).await {
        if db.get_restore_journal().await.is_none() {
            warn!(
                "Proposer restore recovery completed by rolling back incomplete restore state before startup initialisation: {error}"
            );
        } else {
            return Err(format!(
                "Proposer restore recovery failed before startup initialisation: {error}"
            ));
        }
    }
    if let Some(marker) = db.get_startup_replay_reset_marker().await {
        warn!(
            "Detected startup replay reset marker in phase {:?} during proposer startup initialisation; bootstrap will resume reset before validation",
            marker.phase
        );
    }
    ensure_proposer_db_initialized(db).await;
    Ok(())
}
