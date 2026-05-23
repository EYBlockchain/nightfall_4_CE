use super::consistency::startup_local_state_cleanup_error;
use crate::{
    domain::entities::SyncState,
    driven::db::{
        client_transaction_state::restore_all_selected_transactions_to_mempool,
        mongo_db::{StoredBlock, DB, DEPOSIT_COLLECTION, PROPOSED_BLOCKS_COLLECTION},
    },
    ports::db::{BlockStorageDB, PendingBlockDB},
};
use ark_bn254::Fr as Fr254;
use lib::merkle_trees::trees::TreeMetadata;
use mongodb::{
    bson::{doc, Document},
    Client,
};

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

pub(super) async fn cleanup_non_canonical_startup_state(
    client: &Client,
    sync_state: Option<&SyncState>,
) -> Result<(), String> {
    let _ = restore_all_selected_transactions_to_mempool(client)
        .await
        .map_err(|error| startup_local_state_cleanup_error(&error))?;
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
