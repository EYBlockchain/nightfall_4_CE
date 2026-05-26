use super::consistency::startup_local_state_cleanup_error;
use crate::{
    domain::entities::SyncState,
    driven::db::{
        client_transaction_state::{
            restore_all_selected_transactions_to_mempool,
            restore_selected_transactions_to_mempool_after_block,
        },
        mongo_db::{StoredBlock, DB, DEPOSIT_COLLECTION, PROPOSED_BLOCKS_COLLECTION},
    },
    ports::{
        db::{BlockStorageDB, PendingBlockDB, SyncStateDB},
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::Fr as Fr254;
use lib::merkle_trees::trees::{MutableTree, TreeMetadata};
use mongodb::{
    bson::{doc, Document},
    Client,
};
use std::collections::HashSet;

pub(super) async fn tree_sub_tree_count(client: &Client, tree_name: &str) -> Result<u64, String> {
    let metadata_collection_name = format!("{tree_name}_metadata");
    let metadata = client
        .database(DB)
        .collection::<TreeMetadata<Fr254>>(&metadata_collection_name)
        .find_one(mongodb::bson::doc! {})
        .await
        .map_err(|error| format!("Could not read proposer tree metadata for {tree_name}: {error}"))?
        .ok_or_else(|| {
            super::consistency::tree_state_inconsistency_error(&format!(
                "tree metadata for {tree_name} is missing."
            ))
        })?;
    Ok(metadata.sub_tree_count)
}

async fn collection_exists(client: &Client, collection_name: &str) -> Result<bool, String> {
    let names = client
        .database(DB)
        .list_collection_names()
        .await
        .map_err(|error| format!("Could not list proposer collections: {error}"))?;
    Ok(names.iter().any(|name| name == collection_name))
}

pub(super) async fn indexed_leaves_count(client: &Client, tree_name: &str) -> Result<u64, String> {
    let collection_name = format!("{tree_name}_indexed_leaves");
    if !collection_exists(client, &collection_name).await? {
        return Err(super::consistency::tree_state_inconsistency_error(
            &format!("indexed leaves collection for {tree_name} is missing."),
        ));
    }

    client
        .database(DB)
        .collection::<Document>(&collection_name)
        .count_documents(doc! {})
        .await
        .map_err(|error| {
            format!("Could not count proposer indexed leaves collection {collection_name}: {error}")
        })
}

pub(super) async fn highest_stored_block_number(client: &Client) -> Result<Option<u64>, String> {
    client
        .database(DB)
        .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
        .find_one(mongodb::bson::doc! {})
        .sort(mongodb::bson::doc! { "layer2_block_number": -1_i32 })
        .await
        .map_err(|error| format!("Could not inspect proposer StoredBlocks: {error}"))
        .map(|maybe_block| maybe_block.map(|block| block.layer2_block_number))
}

pub(super) async fn reserved_deposit_count(client: &Client) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<mongodb::bson::Document>(DEPOSIT_COLLECTION)
        .count_documents(mongodb::bson::doc! { "reserved": true })
        .await
        .map_err(|error| format!("Could not inspect proposer Deposits reservations: {error}"))
}

pub(crate) async fn clear_all_reserved_deposits(client: &Client) -> Result<u64, String> {
    client
        .database(DB)
        .collection::<Document>(DEPOSIT_COLLECTION)
        .update_many(
            doc! { "reserved": true },
            doc! { "$set": { "reserved": false } },
        )
        .await
        .map(|result| result.modified_count)
        .map_err(|error| format!("Could not clear proposer Deposits reservations: {error}"))
}

async fn pending_block_numbers(client: &Client) -> Result<HashSet<u64>, String> {
    client
        .get_all_pending_blocks()
        .await
        .ok_or_else(|| {
            startup_local_state_cleanup_error(
                "Could not inspect proposer PendingBlocks during startup cleanup.",
            )
        })
        .map(|pending_blocks| {
            pending_blocks
                .into_iter()
                .map(|pending_block| pending_block.layer2_block_number)
                .collect()
        })
}

fn pending_block_numbers_from_pending_blocks(
    pending_blocks: &[crate::domain::entities::PendingBlock],
) -> HashSet<u64> {
    pending_blocks
        .iter()
        .map(|pending_block| pending_block.layer2_block_number)
        .collect()
}

fn speculative_blocks_without_sync_state_are_cleanup_safe(
    stored_blocks: &[StoredBlock],
    pending_block_numbers: &HashSet<u64>,
) -> bool {
    !stored_blocks.is_empty()
        && stored_blocks
            .iter()
            .all(|stored_block| pending_block_numbers.contains(&stored_block.layer2_block_number))
}

pub(super) async fn startup_tree_replay_reset_candidate(
    client: &Client,
    sync_state: Option<&SyncState>,
) -> Result<bool, String> {
    let pending_blocks = client.get_all_pending_blocks().await.ok_or_else(|| {
        startup_local_state_cleanup_error(
            "Could not inspect proposer PendingBlocks while evaluating startup tree recovery.",
        )
    })?;

    match sync_state {
        Some(sync_state) => {
            let has_pending_state_ahead = pending_blocks.iter().any(|pending_block| {
                pending_block.layer2_block_number > sync_state.last_applied_l2_block
            });
            let has_stored_block_ahead = highest_stored_block_number(client)
                .await?
                .is_some_and(|highest_block| highest_block > sync_state.last_applied_l2_block);
            Ok(has_pending_state_ahead || has_stored_block_ahead)
        }
        None => {
            if !pending_blocks.is_empty() {
                return Ok(true);
            }

            let stored_blocks = client.get_all_blocks().await.ok_or_else(|| {
                startup_local_state_cleanup_error(
                    "Could not inspect proposer StoredBlocks while evaluating startup tree recovery.",
                )
            })?;
            if stored_blocks.is_empty() {
                return Ok(false);
            }

            let pending_block_numbers = pending_block_numbers_from_pending_blocks(&pending_blocks);
            Ok(speculative_blocks_without_sync_state_are_cleanup_safe(
                &stored_blocks,
                &pending_block_numbers,
            ))
        }
    }
}

pub(super) async fn reset_proposer_state_for_startup_replay(client: &Client) -> Result<(), String> {
    let mut session = client.start_session().await.map_err(|error| {
        startup_local_state_cleanup_error(&format!(
            "Could not start MongoDB session for startup replay reset: {error}"
        ))
    })?;
    let client_for_cleanup = client.clone();
    session
        .start_transaction()
        .and_run2(async move |session| {
            client_for_cleanup
                .delete_sync_state_with_session(session)
                .await?;
            client_for_cleanup
                .delete_all_blocks_with_session(session)
                .await?;
            client_for_cleanup
                .delete_all_pending_blocks_with_session(session)
                .await?;
            Ok::<(), mongodb::error::Error>(())
        })
        .await
        .map_err(|error| {
            startup_local_state_cleanup_error(&format!(
                "Could not clear proposer canonical state for startup replay reset: {error}"
            ))
        })?;

    <Client as MutableTree<Fr254>>::reset_mutable_tree(
        client,
        <Client as CommitmentTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        startup_local_state_cleanup_error(&format!(
            "Could not reset proposer commitment tree for startup replay reset: {error}"
        ))
    })?;
    <Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
        .await
        .map_err(|error| {
            startup_local_state_cleanup_error(&format!(
                "Could not reinitialize proposer commitment tree for startup replay reset: {error}"
            ))
        })?;

    <Client as MutableTree<Fr254>>::reset_mutable_tree(
        client,
        <Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        startup_local_state_cleanup_error(&format!(
            "Could not reset proposer historic root tree for startup replay reset: {error}"
        ))
    })?;
    <Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
        .await
        .map_err(|error| {
            startup_local_state_cleanup_error(&format!(
                "Could not reinitialize proposer historic root tree for startup replay reset: {error}"
            ))
        })?;
    <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
        client,
        &Fr254::from(0u8),
        true,
    )
    .await
    .map_err(|error| {
        startup_local_state_cleanup_error(&format!(
            "Could not restore zero historic root for startup replay reset: {error}"
        ))
    })?;

    <Client as MutableTree<Fr254>>::reset_mutable_tree(
        client,
        <Client as NullifierTree<Fr254>>::TREE_NAME,
    )
    .await
    .map_err(|error| {
        startup_local_state_cleanup_error(&format!(
            "Could not reset proposer nullifier tree for startup replay reset: {error}"
        ))
    })?;
    let indexed_collection = client
        .database(DB)
        .collection::<Document>("Nullifiers_indexed_leaves");
    if let Err(error) = indexed_collection.drop().await {
        if !error.to_string().contains("ns not found") {
            return Err(startup_local_state_cleanup_error(&format!(
                "Could not reset proposer nullifier indexed leaves for startup replay reset: {error}"
            )));
        }
    }
    <Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
        .await
        .map_err(|error| {
            startup_local_state_cleanup_error(&format!(
                "Could not reinitialize proposer nullifier tree for startup replay reset: {error}"
            ))
        })?;

    Ok(())
}

pub(super) async fn cleanup_non_canonical_startup_state(
    client: &Client,
    sync_state: Option<&SyncState>,
) -> Result<(), String> {
    match sync_state {
        Some(sync_state) => {
            let _ = restore_selected_transactions_to_mempool_after_block(
                client,
                sync_state.last_applied_l2_block,
            )
            .await
            .map_err(|error| startup_local_state_cleanup_error(&error))?;
        }
        None => {
            let _ = restore_all_selected_transactions_to_mempool(client)
                .await
                .map_err(|error| startup_local_state_cleanup_error(&error))?;
        }
    }

    let _ = clear_all_reserved_deposits(client)
        .await
        .map_err(|error| startup_local_state_cleanup_error(&error))?;

    match sync_state {
        Some(sync_state) => {
            let stored_blocks = client.get_all_blocks().await.ok_or_else(|| {
                startup_local_state_cleanup_error(
                    "Could not inspect proposer StoredBlocks during startup cleanup.",
                )
            })?;

            for stored_block in stored_blocks.into_iter().filter(|stored_block| {
                stored_block.layer2_block_number > sync_state.last_applied_l2_block
            }) {
                client
                    .delete_block_by_number(stored_block.layer2_block_number)
                    .await
                    .ok_or_else(|| {
                        startup_local_state_cleanup_error(&format!(
                            "Could not delete speculative StoredBlock {} during startup cleanup.",
                            stored_block.layer2_block_number
                        ))
                    })?;
            }
        }
        None => {
            let stored_blocks = client.get_all_blocks().await.ok_or_else(|| {
                startup_local_state_cleanup_error(
                    "Could not inspect proposer StoredBlocks during startup cleanup.",
                )
            })?;
            if !stored_blocks.is_empty() {
                let pending_block_numbers = pending_block_numbers(client).await?;
                if !speculative_blocks_without_sync_state_are_cleanup_safe(
                    &stored_blocks,
                    &pending_block_numbers,
                ) {
                    return Err(startup_local_state_cleanup_error(
                        "StoredBlocks exist without sync_state, but they are not fully explained by persisted PendingBlocks. Manual recovery is required.",
                    ));
                }
            }

            let _ = client.delete_all_blocks().await.ok_or_else(|| {
                startup_local_state_cleanup_error(
                    "Could not delete speculative StoredBlocks during startup cleanup.",
                )
            })?;
        }
    }

    let _ = client.delete_all_pending_blocks().await.ok_or_else(|| {
        startup_local_state_cleanup_error(
            "Could not delete persisted PendingBlocks during startup cleanup.",
        )
    })?;

    Ok(())
}
