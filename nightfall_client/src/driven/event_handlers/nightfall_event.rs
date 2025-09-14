use crate::{
    domain::{
        entities::{
            CommitmentStatus, CompressedSecrets, OnChainTransaction, Preimage, RequestStatus, Salt,
        },
        error::EventHandlerError,
        notifications::NotificationPayload,
    },
    driven::{
        contract_functions::contract_type_conversions::FrBn254,
        db::mongo::{BlockStorageDB, CommitmentEntry, StoredBlock},
        notifier::webhook_notifier::WebhookNotifier,
        primitives::kemdem_functions::kemdem_decrypt,
    },
    drivers::derive_key::ZKPKeys,
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        commitments::{Commitment, Nullifiable},
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB, RequestDB},
        events::EventHandler,
        trees::CommitmentTree,
    },
    services::data_publisher::DataPublisher,
};
use alloy::consensus::Transaction;
use alloy::sol_types::SolInterface;
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger;
use configuration::settings::get_settings;

use alloy::primitives::{TxHash, I256, U256};
use lib::{
    blockchain_client::BlockchainClientConnection, hex_conversion::HexConvertible,
    initialisation::get_blockchain_client_connection,
};
use log::{debug, error, info, warn};
use nightfall_bindings::artifacts::Nightfall;
use std::{collections::HashSet, sync::OnceLock};
use tokio::{join, sync::Mutex};

// Define a mutable lazy static to hold the layer 2 blocknumber. We need this to
// check if we're still in sync.
pub fn get_expected_layer2_blocknumber() -> &'static Mutex<I256> {
    static LAYER2_BLOCKNUMBER: OnceLock<Mutex<I256>> = OnceLock::new();
    LAYER2_BLOCKNUMBER.get_or_init(|| Mutex::new(I256::ZERO))
}

/// Implementation of the EventHandler trait for the NightfallEvents enum.
/// This will receive any blockchain events that are emitted by the NIGHTFALL smart contracts
/// and pass them to the appropriate handler.
/// This is similar to the proposers event handler but calls different functions and has different traits ultimately.
/// We could possibly refactor this to use the same event handler in future.
#[async_trait::async_trait]
impl<N> EventHandler<N> for Nightfall::NightfallEvents
where
    N: NightfallContract,
{
    async fn handle_event(&self, tx_hash: Option<TxHash>) -> Result<(), EventHandlerError> {
        info!("handle_event called with event: {:?}", self);
        // we'll split out individual events here in case that's useful later
        match &self {
            Nightfall::NightfallEvents::BlockProposed(filter) => {
                info!("Detected a new block has been proposed");
                process_nightfall_calldata::<N>(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{e}");
                        debug!("{e}");
                        EventHandlerError::InvalidCalldata
                    })?;
            }
            Nightfall::NightfallEvents::DepositEscrowed(filter) => {
                info!("Received DepositEscrowed event");
                process_deposit_escrowed_event(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{e}");
                        EventHandlerError::InvalidCalldata
                    })?;
            }
            Nightfall::NightfallEvents::Initialized(_filter) => {
                info!("Received Initialized event");
            }
            Nightfall::NightfallEvents::Upgraded(_filter) => {
                info!("Received Upgraded event");
            }
            Nightfall::NightfallEvents::AuthoritiesUpdated(_filter) => {
                info!("Received AuthoritiesUpdated event");
            }
            Nightfall::NightfallEvents::OwnershipTransferred(_filter) => {
                info!("Received OwnershipTransferred event");
            }
        }
        Ok(())
    }
}

/// This function gets the calldata associated with a given transaction and decodes it.
/// Once decoded, it passes the decoded calldata to the appropriate function for processing.
pub async fn process_nightfall_calldata<N: NightfallContract>(
    transaction_hash: Option<TxHash>,
    filter: &Nightfall::BlockProposed,
) -> Result<(), EventHandlerError> {
    // get the transaction
    let tx_hash = transaction_hash.ok_or(EventHandlerError::IOError(
        "No transaction hash provided".to_string(),
    ))?;
    let tx = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction_by_hash(tx_hash)
        .await
        .map_err(|e| EventHandlerError::IOError(e.to_string()))?;
    // if there is one, decode it. If not, warn someone.
    match tx {
        Some(tx) => {
            let decoded = Nightfall::NightfallCalls::abi_decode(tx.input())
                .map_err(|e| EventHandlerError::IOError(e.to_string()))?;
            #[allow(clippy::single_match)] // we may add more events later
            match decoded {
                Nightfall::NightfallCalls::propose_block(decode) => {
                    info!("Processing a block proposed event");
                    process_propose_block_event::<N>(decode, tx_hash, filter).await?
                }
                _ => (),
            }
        }
        None => panic!("Transaction not found when looking up calldata"),
    }
    Ok(())
}

/// This function is called whenever we receive and decode a valid block
async fn process_propose_block_event<N: NightfallContract>(
    decode: Nightfall::propose_blockCall,
    transaction_hash: TxHash,
    filter: &Nightfall::BlockProposed,
) -> Result<(), EventHandlerError> {
    info!(
        "Decoded Proposed block call from transaction {}, Layer 2 block number {} is now on-chain",
        transaction_hash, filter.layer2_block_number,
    );

    let blk = decode.blk;
    // The first thing to do is to make sure that we've not missed any blocks.
    // If we have, then we'll need to resynchronise with the blockchain.
    // note, the L2 block number on chain increments immediately after the BlockProposed event is emitted (hence adding 1).
    let layer_2_block_number_in_event = filter.layer2_block_number;
    let mut expected_onchain_block_number = get_expected_layer2_blocknumber().lock().await;
    if *expected_onchain_block_number < layer_2_block_number_in_event {
        warn!(
            "Out of sync with blockchain. Blockchain has block number {layer_2_block_number_in_event}, expected {expected_onchain_block_number}"
        );
        return Err(EventHandlerError::MissingBlocks(
            expected_onchain_block_number.as_usize(),
        ));
    }

    // check if we're ahead of the event, this means we've already seen it and we shouldn't process it again
    // This could happen if we've missed some blocks and we're re-synchronising

    if *expected_onchain_block_number > layer_2_block_number_in_event {
        warn!("Already processed layer 2 block {layer_2_block_number_in_event} - skipping");
        return Ok(());
    }
    let layer_2_block_number_in_event_u64: u64 = layer_2_block_number_in_event
        .try_into()
        .expect("I256 to u64 conversion failed");
    let proposer_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction_by_hash(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?
        .ok_or(EventHandlerError::IOError(
            "Could not retrieve transaction".to_string(),
        ))?
        .inner
        .signer();

    let store_block_pending = StoredBlock {
        layer2_block_number: layer_2_block_number_in_event_u64,
        commitments: blk
            .transactions
            .iter()
            .flat_map(|ntx| {
                let tx: OnChainTransaction = (*ntx).clone().into();
                tx.commitments
                    .iter()
                    .map(|c| c.to_hex_string())
                    .collect::<Vec<_>>()
            })
            .collect(),
        proposer_address,
    };
    // get a lock on the db, we don't want anything else updating or reading the DB until
    // we're done here
    let db = &mut get_db_connection().await;
    let layer_2_block_number_in_event_u64: u64 = layer_2_block_number_in_event
        .try_into()
        .expect("I256 to u64 conversion failed");

    if *expected_onchain_block_number == layer_2_block_number_in_event
        && db
            .get_block_by_number(layer_2_block_number_in_event_u64)
            .await
            .is_some()
    {
        // compute the expected block hash and the block hash saved before in other proposed block event

        let existing_block = db
            .get_block_by_number(layer_2_block_number_in_event_u64)
            .await
            .ok_or(EventHandlerError::IOError(
                "Could not retrieve block from database".to_string(),
            ))?;
        let existing_block_stored_hash = existing_block.hash();
        let block_store_pending_hash = store_block_pending.hash();

        if existing_block.hash() != store_block_pending.hash() {
            warn!(
            "Block hash mismatch. Expected {existing_block_stored_hash}, got {block_store_pending_hash} in layer 2 block {layer_2_block_number_in_event}"
        );
            // Delete the invalid block and clear sync status
            db.delete_block_by_number(layer_2_block_number_in_event_u64)
                .await;
            return Err(EventHandlerError::BlockHashError(
                existing_block_stored_hash,
                block_store_pending_hash,
            ));
        } else {
            debug!(
            "Block hash matches for layer 2 block {layer_2_block_number_in_event}: {existing_block_stored_hash}"
        );
        }
    }
    *expected_onchain_block_number += I256::ONE;

    // warn that we're not synced with the blockchain if we're behind
    let current_block_number = N::get_current_layer2_blocknumber().await.map_err(|_| {
        EventHandlerError::IOError("Could not retrieve current block number".to_string())
    })?;

    let delta = current_block_number - filter.layer2_block_number - I256::ONE;
    // if we"re synchronising, we don"t want to check for duplicate keys because we expect to overwrite commitments already in the commitment collection
    let dup_key_check = if delta != I256::ZERO {
        warn!("Synchronising - behind blockchain by {delta} layer 2 blocks ");
        false
    } else {
        debug!("Synchronised with blockchain");
        true
    };

    // Next, we'll attempt to decode the transactions with compressed secrets and, if they're for us,
    // we'll store the commitments. We'll also add all the commitments to our local copy of the commitment tree,
    // whether or not they're ours.

    // get keys from the lazy static global that holds them. We'll use these to decrpyt the compressed secrets
    let ZKPKeys {
        zkp_public_key,
        zkp_private_key,
        nullifier_key,
        ..
    } = *get_zkp_keys().lock().expect("Poisoned lock");
    debug!("Processing transactions");
    let db = &get_db_connection().await;

    // first, add _all_ the commitments to the commitment tree
    // This does mean iterating over the transactions twice, but that's a fast operation and it has the benefit of making
    // the code a bit clearer by seperating the logical operations.

    // get all the commitments in the block into a nice, flat vec
    let commitments = &blk
        .transactions
        .iter()
        .flat_map(|transaction| &transaction.commitments)
        .map(|u| FrBn254::try_from(*u).map(|f| f.into()))
        .collect::<Result<Vec<Fr254>, _>>()
        .map_err(|_| {
            EventHandlerError::IOError("Could not convert commitment to Fr254".to_string())
        })?;
    debug!("Block has {:?} commitments", &commitments.len());
    // add them all to the timber tree, saving the index and membership proof for each commitment that is ours
    // get the old root (not used in calculations, but useful for debugging)
    let old_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(db)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not get current root".to_string()))?;
    // and the new root
    let (root, _) =
        <mongodb::Client as CommitmentTree<Fr254>>::append_sub_trees(db, commitments, true)
            .await
            .map_err(|_| {
                EventHandlerError::IOError("Could not append commitments to tree".to_string())
            })?;
    debug!("New commitments tree root is {root}, old root was {old_root}");
    // The root should be the same as the one in the block. This is worth checking
    let historic_root = FrBn254::try_from(blk.commitments_root)
        .map_err(|_| {
            EventHandlerError::IOError("Could not convert commitment to Fr254".to_string())
        })?
        .into();
    if root != historic_root {
        error!("Commitment root in block does not match calculated root. historic root is {historic_root}, calculated root is {root}");
    } else {
        debug!("Commitment root in block matches calculated root");
    }

    debug!("{} commitments added to commitment tree", commitments.len());

    // Update the state of any commitments and nullifiers that are in our database, which this block has put on chain
    let mut nullifiers = vec![];
    let mut commitment_hashes = vec![];
    for transaction in blk.transactions.iter() {
        // check each commitment and if it's in our commitmentdb, mark it as unspent
        for commitment in transaction.commitments.iter() {
            let commitment_hash = FrBn254::try_from(*commitment)
                .map_err(|_| {
                    EventHandlerError::IOError("Could not convert commitment to Fr254".to_string())
                })?
                .into();
            commitment_hashes.push(commitment_hash);
        }
        // check the spent commitments, if they're ours, mark them as spent in our database
        for nullifier in transaction.nullifiers.iter() {
            let nullifier = FrBn254::try_from(*nullifier)
                .map_err(|_| {
                    EventHandlerError::IOError("Could not convert nullifier to Fr254".to_string())
                })?
                .0;
            nullifiers.push(nullifier);
        }
    }
    debug!("Updating commitment database with on-chain data");
    join!(
        db.mark_commitments_unspent(
            &commitment_hashes,
            Some(transaction_hash),
            Some(filter.layer2_block_number)
        ),
        db.mark_commitments_spent(nullifiers)
    );

    debug!("Updating request status for confirmed commitments");
    for commitment_hash in &commitment_hashes {
        let commitment_hex = commitment_hash.to_hex_string();
        if let Some(request_ids) = db.get_requests_by_commitment(&commitment_hex).await {
            for request_id in request_ids {
                debug!("Marking request {request_id} as confirmed");
                db.update_request(&request_id, RequestStatus::Confirmed)
                    .await;
            }
        }
    }

    // now attempt to decrypt the compressed secrets to see which commitments (if any) we own
    let mut commitment_entries = vec![];
    for transaction in blk.transactions.iter() {
        // If all the nullifiers are zero we can skip to the next transaction
        if transaction.nullifiers == [U256::ZERO; 4] {
            continue;
        }

        // Check to see if the first commitment is zero, in which case this was a withdraw and no decrypting is required
        if transaction.commitments[0].is_zero() && !transaction.nullifiers[0].is_zero() {
            continue;
        }

        // Extract the compressed secrets from the public data
        let compressed_secrets_onchain = transaction.public_data;
        let compressed_secrets: CompressedSecrets = compressed_secrets_onchain.into();

        // Attempt to decrypt the compressed secrets
        let decrypt =
            kemdem_decrypt(zkp_private_key, &compressed_secrets.cipher_text).map_err(|_| {
                EventHandlerError::IOError("Could not decrypt compressed secrets".to_string())
            })?;
        // now we have a candidate decrypt, we need to test if it's really a decrypt by seeing if
        // we can reconstruct the commitment from it.  If we can, then the commitment is ours!
        let test_preimage = Preimage {
            nf_token_id: decrypt[0],
            nf_slot_id: decrypt[1],
            value: decrypt[2],
            salt: Salt::Transfer(decrypt[3]),
            public_key: zkp_public_key,
        };
        let test_hash = test_preimage
            .hash()
            .map_err(|_| EventHandlerError::IOError("Could not hash preimage".to_string()))?;

        let commitment_hash = FrBn254::try_from(transaction.commitments[0])
            .map_err(|_| {
                EventHandlerError::IOError("Could not convert commitment to Fr254".to_string())
            })?
            .into();

        if test_hash != commitment_hash {
            debug!(
                "Commitment {} is not owned by us",
                commitment_hash.to_hex_string()
            );
        } else {
            info!(
                "Received commitment owned by us, with hash {}",
                test_hash.to_hex_string()
            );
            // store our newly received commitment in our commitment db
            let nullifier = test_preimage
                .nullifier_hash(&nullifier_key)
                .map_err(|_| EventHandlerError::HashError)?;
            let commitment_entry =
                CommitmentEntry::new(test_preimage, nullifier, CommitmentStatus::Unspent);
            commitment_entries.push(commitment_entry);
        }
    }

    if (db
        .store_commitments(&commitment_entries, dup_key_check)
        .await)
        .is_none()
    {
        error!("Failed to store commitments");
        return Err(EventHandlerError::IOError(
            "Failed to store commitments".to_string(),
        ));
    };

    // Let's use the Data Publisher to publish notification
    // if the WEBHOOK_URL is set
    let webhook_url = &get_settings().nightfall_client.webhook_url;
    debug!("Using webhook URL: {webhook_url}");
    let mut publisher = DataPublisher::new();
    let notifier = WebhookNotifier::new(webhook_url);

    publisher.register_notifier(Box::new(notifier));

    // Let's get the full hash as it gets truncated otherwise
    let l1_txn_hash = format!("{transaction_hash:#x}");
    let owned_commitment_hashes: Vec<String> = commitment_hashes
        .iter()
        .filter(|&c| !c.0.is_zero())
        .map(|&c| c.to_hex_string())
        .collect();

    // Get request IDs associated with commitments
    let mut request_id_set = HashSet::new();
    for commitment_hash in owned_commitment_hashes.clone() {
        if let Some(ids) = db.get_requests_by_commitment(&commitment_hash).await {
            for id in ids {
                request_id_set.insert(id);
            }
        }
    }

    let request_ids = request_id_set.into_iter().collect();

    let notification = NotificationPayload::BlockchainEvent {
        l1_txn_hash,
        l2_block_number: filter.layer2_block_number.as_u64(),
        commitments: owned_commitment_hashes,
        request_ids,
    };

    publisher.publish(notification).await;

    // If the block is not in the database, we can store it
    db.store_block(&store_block_pending).await;
    Ok(())
}

pub async fn process_deposit_escrowed_event(
    transaction_hash: Option<TxHash>,
    filter: &Nightfall::DepositEscrowed,
) -> Result<(), EventHandlerError> {
    info!(
        "Client: Decoded DepositEscrowed event from transaction {}, Deposit Transaction with nf_slot_id {}, value {}, is now on-chain",
        transaction_hash.unwrap(), filter.nfSlotId, filter.value,
    );

    Ok(())
}
