use super::cleanup::{
    cleanup_non_canonical_startup_state, highest_stored_block_number, indexed_leaves_count,
    reserved_deposit_count, tree_sub_tree_count,
};
use crate::{
    domain::entities::SyncState,
    driven::db::{
        client_transaction_state::{
            selected_client_transaction_count, selected_client_transaction_count_after_block,
        },
        mongo_db::StoredBlock,
    },
    ports::{
        db::{BlockStorageDB, SyncStateDB},
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::Fr as Fr254;
use lib::hex_conversion::HexConvertible;
use mongodb::Client;

pub(super) fn missing_stored_block_error(last_applied_l2_block: u64) -> String {
    format!(
        "Proposer startup aborted: sync_state references L2 block {last_applied_l2_block}, \
         but StoredBlock at height {last_applied_l2_block} is missing. Local proposer state \
         is inconsistent. Manual intervention is required before restart."
    )
}

fn fingerprint_mismatch_error(
    last_applied_l2_block: u64,
    stored_fingerprint: &str,
    sync_state_fingerprint: &str,
) -> String {
    format!(
        "Proposer startup aborted: sync_state references L2 block {last_applied_l2_block}, \
         but StoredBlock fingerprint ({stored_fingerprint}) does not match sync_state \
         fingerprint ({sync_state_fingerprint}). Local proposer state is inconsistent. \
         Manual intervention is required before restart."
    )
}

fn unsupported_sync_state_schema_error(schema_version: u32) -> String {
    format!(
        "Proposer startup aborted: sync_state uses unsupported schema version \
         {schema_version}. Manual intervention is required before restart."
    )
}

pub(super) fn ahead_of_chain_error(next_expected_block: u64, onchain_next_block: u64) -> String {
    format!(
        "Proposer startup aborted: local proposer state expects next L2 block \
         {next_expected_block}, but chain reports {onchain_next_block}. Local state is ahead \
         of chain. This may indicate an L1 reorg, chain rollback, or dev chain reset. \
         Manual recovery is required before restart."
    )
}

pub(super) fn tree_state_inconsistency_error(details: &str) -> String {
    format!(
        "Proposer startup aborted: local proposer tree state is inconsistent with persisted \
         sync_state. {details} Manual recovery is required before restart."
    )
}

fn stored_blocks_ahead_of_sync_state_error(
    last_applied_l2_block: u64,
    highest_stored_block: u64,
) -> String {
    format!(
        "Proposer startup aborted: highest StoredBlock ({highest_stored_block}) is ahead of \
         sync_state-applied block {last_applied_l2_block}. Local proposer state is \
         inconsistent. Manual recovery is required before restart."
    )
}

fn reserved_deposits_ahead_of_sync_state_error(
    last_applied_l2_block: u64,
    reserved_deposit_count: u64,
) -> String {
    format!(
        "Proposer startup aborted: Deposits contains {reserved_deposit_count} reserved \
         selection(s) ahead of sync_state-applied block {last_applied_l2_block}. Local \
         proposer state is inconsistent. Manual recovery is required before restart."
    )
}

fn selected_transactions_ahead_of_sync_state_error(
    last_applied_l2_block: u64,
    selected_transaction_count: u64,
) -> String {
    format!(
        "Proposer startup aborted: ClientTransactions contains {selected_transaction_count} \
         selected transaction(s) ahead of sync_state-applied block {last_applied_l2_block}. \
         Local proposer state is inconsistent. Manual recovery is required before restart."
    )
}

pub(super) fn startup_local_state_cleanup_error(details: &str) -> String {
    format!(
        "Proposer startup aborted: non-canonical local proposer state cleanup failed. \
         {details} Manual recovery is required before restart."
    )
}

pub(super) fn startup_legacy_transaction_migration_error(details: &str) -> String {
    format!(
        "Proposer startup aborted: legacy ClientTransactions lifecycle backfill failed. \
         {details} Manual recovery is required before restart."
    )
}

fn expected_historic_root_sub_tree_count(last_applied_l2_block: u64) -> Result<u64, String> {
    last_applied_l2_block.checked_add(2).ok_or_else(|| {
        tree_state_inconsistency_error(
            "sync_state last_applied_l2_block overflowed while validating historic roots.",
        )
    })
}

pub(super) async fn validate_tree_state_against_sync_state(
    client: &Client,
    sync_state: Option<&SyncState>,
    stored_block: Option<&StoredBlock>,
) -> Result<(), String> {
    let commitment_sub_tree_count = tree_sub_tree_count(
        client,
        <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
    )
    .await?;
    let nullifier_sub_tree_count =
        tree_sub_tree_count(client, <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME).await?;
    let nullifier_indexed_leaf_count =
        indexed_leaves_count(client, <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME).await?;
    let historic_root_sub_tree_count = tree_sub_tree_count(
        client,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await?;

    match sync_state {
        Some(sync_state) => {
            let expected_historic_root_sub_tree_count =
                expected_historic_root_sub_tree_count(sync_state.last_applied_l2_block)?;

            if historic_root_sub_tree_count <= 1 {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the historic root tree only \
                     contains the zero leaf.",
                    sync_state.last_applied_l2_block,
                )));
            }

            if historic_root_sub_tree_count > expected_historic_root_sub_tree_count {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the historic root tree \
                     sub_tree_count {} is ahead of the maximum coherent value {}.",
                    sync_state.last_applied_l2_block,
                    historic_root_sub_tree_count,
                    expected_historic_root_sub_tree_count,
                )));
            }

            if historic_root_sub_tree_count < expected_historic_root_sub_tree_count {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the historic root tree \
                     sub_tree_count {} is behind the required coherent value {}.",
                    sync_state.last_applied_l2_block,
                    historic_root_sub_tree_count,
                    expected_historic_root_sub_tree_count,
                )));
            }

            if commitment_sub_tree_count == 0
                && stored_block.is_some_and(|block| !block.commitments.is_empty())
            {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the commitment tree is empty \
                     while the stored block at that height contains commitments.",
                    sync_state.last_applied_l2_block
                )));
            }

            if nullifier_sub_tree_count == 0 {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the nullifier tree metadata \
                     reports sub_tree_count 0.",
                    sync_state.last_applied_l2_block
                )));
            }

            if nullifier_indexed_leaf_count == 0 {
                return Err(tree_state_inconsistency_error(&format!(
                    "sync_state records applied L2 block {}, but the nullifier indexed leaves \
                     collection is empty.",
                    sync_state.last_applied_l2_block
                )));
            }
        }
        None => {
            if commitment_sub_tree_count > 0
                || historic_root_sub_tree_count > 1
                || nullifier_sub_tree_count != 1
                || nullifier_indexed_leaf_count != 1
            {
                return Err(tree_state_inconsistency_error(&format!(
                    "no proposer sync_state exists, but proposer trees are not empty \
                     (commitment_sub_tree_count={commitment_sub_tree_count}, \
                     historic_root_sub_tree_count={historic_root_sub_tree_count}, \
                     nullifier_sub_tree_count={nullifier_sub_tree_count}, \
                     nullifier_indexed_leaf_count={nullifier_indexed_leaf_count})."
                )));
            }
        }
    }

    Ok(())
}
pub(super) async fn validate_startup_proposer_state_consistency(
    client: &Client,
) -> Result<(), String> {
    let sync_state = client.get_sync_state().await;

    match sync_state.as_ref() {
        Some(sync_state) => {
            let stored_block = client
                .get_block_by_number(sync_state.last_applied_l2_block)
                .await
                .ok_or_else(|| missing_stored_block_error(sync_state.last_applied_l2_block))?;

            validate_sync_state_against_block(sync_state, &stored_block)?;
            validate_tree_state_against_sync_state(client, Some(sync_state), Some(&stored_block))
                .await?;
        }
        None => {
            validate_tree_state_against_sync_state(client, None, None).await?;

            if let Some(highest_stored_block) = highest_stored_block_number(client).await? {
                return Err(stored_blocks_ahead_of_sync_state_error(
                    0,
                    highest_stored_block,
                ));
            }
        }
    }

    cleanup_non_canonical_startup_state(client, sync_state.as_ref()).await?;
    validate_live_proposer_state_consistency(client).await
}

async fn validate_snapshotted_live_proposer_state_with_sync_state(
    client: &Client,
    sync_state: Option<&SyncState>,
) -> Result<(), String> {
    match sync_state {
        Some(sync_state) => {
            let stored_block = client
                .get_block_by_number(sync_state.last_applied_l2_block)
                .await
                .ok_or_else(|| missing_stored_block_error(sync_state.last_applied_l2_block))?;

            validate_sync_state_against_block(sync_state, &stored_block)?;
            validate_tree_state_against_sync_state(client, Some(sync_state), Some(&stored_block))
                .await?;

            if let Some(highest_stored_block) = highest_stored_block_number(client).await? {
                if highest_stored_block > sync_state.last_applied_l2_block {
                    return Err(stored_blocks_ahead_of_sync_state_error(
                        sync_state.last_applied_l2_block,
                        highest_stored_block,
                    ));
                }
            }

            Ok(())
        }
        None => {
            validate_tree_state_against_sync_state(client, None, None).await?;

            if let Some(highest_stored_block) = highest_stored_block_number(client).await? {
                return Err(stored_blocks_ahead_of_sync_state_error(
                    0,
                    highest_stored_block,
                ));
            }

            Ok(())
        }
    }
}

pub(crate) async fn validate_snapshotted_live_proposer_state_consistency(
    client: &Client,
) -> Result<(), String> {
    let sync_state = client.get_sync_state().await;
    validate_snapshotted_live_proposer_state_with_sync_state(client, sync_state.as_ref()).await
}

pub(crate) async fn validate_live_proposer_state_consistency(
    client: &Client,
) -> Result<(), String> {
    let sync_state = client.get_sync_state().await;
    validate_snapshotted_live_proposer_state_with_sync_state(client, sync_state.as_ref()).await?;

    match sync_state {
        Some(sync_state) => {
            let reserved_deposit_count = reserved_deposit_count(client).await?;
            if reserved_deposit_count > 0 {
                return Err(reserved_deposits_ahead_of_sync_state_error(
                    sync_state.last_applied_l2_block,
                    reserved_deposit_count,
                ));
            }

            let selected_transaction_count = selected_client_transaction_count_after_block(
                client,
                sync_state.last_applied_l2_block,
            )
            .await?;
            if selected_transaction_count > 0 {
                return Err(selected_transactions_ahead_of_sync_state_error(
                    sync_state.last_applied_l2_block,
                    selected_transaction_count,
                ));
            }

            Ok(())
        }
        None => {
            validate_tree_state_against_sync_state(client, None, None).await?;

            if let Some(highest_stored_block) = highest_stored_block_number(client).await? {
                return Err(stored_blocks_ahead_of_sync_state_error(
                    0,
                    highest_stored_block,
                ));
            }

            let reserved_deposit_count = reserved_deposit_count(client).await?;
            if reserved_deposit_count > 0 {
                return Err(reserved_deposits_ahead_of_sync_state_error(
                    0,
                    reserved_deposit_count,
                ));
            }

            let selected_transaction_count = selected_client_transaction_count(client).await?;
            if selected_transaction_count > 0 {
                return Err(selected_transactions_ahead_of_sync_state_error(
                    0,
                    selected_transaction_count,
                ));
            }

            Ok(())
        }
    }
}

pub(super) fn validate_sync_state_against_block(
    sync_state: &SyncState,
    stored_block: &StoredBlock,
) -> Result<(), String> {
    let stored_fingerprint = stored_block.hash().to_hex_string();
    if stored_fingerprint != sync_state.fingerprint {
        return Err(fingerprint_mismatch_error(
            sync_state.last_applied_l2_block,
            &stored_fingerprint,
            &sync_state.fingerprint,
        ));
    }

    if sync_state.schema_version != SyncState::SCHEMA_VERSION {
        return Err(unsupported_sync_state_schema_error(
            sync_state.schema_version,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_historic_root_sub_tree_count_tracks_last_applied_block_exactly() {
        assert_eq!(expected_historic_root_sub_tree_count(0).unwrap(), 2);
        assert_eq!(expected_historic_root_sub_tree_count(10).unwrap(), 12);
    }
}
