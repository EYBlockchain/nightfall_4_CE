use crate::{
    domain::{
        entities::{
            CommitmentStatus, CompressedSecrets, HexConvertible, Preimage, RequestStatus, Salt,
        },
        error::EventHandlerError,
        notifications::NotificationPayload,
    },
    driven::{
        contract_functions::contract_type_conversions::FrBn254, db::mongo::CommitmentEntry,
        notifier::webhook_notifier::WebhookNotifier, primitives::kemdem_functions::kemdem_decrypt,
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
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger;
use configuration::settings::get_settings;
use ethers::{
    abi::AbiDecode,
    providers::Middleware,
    types::{TxHash, H256, I256, U256},
};
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, error, info, warn};
use nightfall_bindings::nightfall::{
    BlockProposedFilter, DepositEscrowedFilter, NightfallCalls, NightfallEvents, ProposeBlockCall,
};
use std::{collections::HashSet, error::Error, sync::OnceLock};
use tokio::{join, sync::Mutex};

// Define a mutable lazy static to hold the layer 2 blocknumber. We need this to
// check if we're still in sync.
pub fn get_expected_layer2_blocknumber() -> &'static Mutex<I256> {
    static LAYER2_BLOCKNUMBER: OnceLock<Mutex<I256>> = OnceLock::new();
    LAYER2_BLOCKNUMBER.get_or_init(|| Mutex::new(I256::zero()))
}

/// Implementation of the EventHandler trait for the NightfallEvents enum.
/// This will receive any blockchain events that are emitted by the NIGHTFALL smart contracts
/// and pass them to the appropriate handler.
/// This is similar to the proposers event handler but calls different functions and has different traits ultimately.
/// We could possibly refactor this to use the same event handler in future.
#[async_trait::async_trait]
impl<N> EventHandler<N> for NightfallEvents
where
    N: NightfallContract,
{
    async fn handle_event(&self, tx_hash: TxHash) -> Result<(), EventHandlerError> {
        // we'll split out individual events here in case that's useful later
        match &self {
            NightfallEvents::BlockProposedFilter(filter) => {
                info!("Detected a new block has been proposed");
                process_nightfall_calldata::<N>(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{}", e);
                        EventHandlerError::InvalidCalldata
                    })?;
            }
            NightfallEvents::DepositEscrowedFilter(filter) => {
                info!("Received DepositEscrowed event");
                process_deposit_escrowed_event(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{}", e);
                        EventHandlerError::InvalidCalldata
                    })?;
            }
        }
        Ok(())
    }
}

/// This function gets the calldata associated with a given transaction and decodes it.
/// Once decoded, it passes the decoded calldata to the appropriate function for processing.
pub async fn process_nightfall_calldata<N: NightfallContract>(
    transaction_hash: H256,
    filter: &BlockProposedFilter,
) -> Result<(), Box<dyn Error>> {
    // get the transaction
    let tx = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction(transaction_hash)
        .await?;
    // if there is one, decode it. If not, warn someone.
    match tx {
        Some(tx) => {
            let decoded = NightfallCalls::decode(tx.input)?;
            #[allow(clippy::single_match)] // we may add more events later
            match decoded {
                NightfallCalls::ProposeBlock(decode) => {
                    info!("Processing a block proposed event");
                    process_propose_block_event::<N>(decode, transaction_hash, filter).await?
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
    decode: ProposeBlockCall,
    transaction_hash: H256,
    filter: &BlockProposedFilter,
) -> Result<(), EventHandlerError> {
    info!(
        "Decoded Proposed block call from transaction {}, Layer 2 block number {} is now on-chain",
        transaction_hash, filter.layer_2_block_number,
    );
    let blk = decode.blk;
    // The first thing to do is to make sure that we've not missed any blocks.
    // If we have, then we'll need to resynchronise with the blockchain.
    // note, the L2 block number on chain increments immediately after the BlockProposed event is emitted (hence adding 1).
    let mut expected_onchain_block_number = get_expected_layer2_blocknumber().lock().await;
    if *expected_onchain_block_number < filter.layer_2_block_number {
        warn!(
            "Out of sync with blockchain. Blockchain has block number {}, expected {}",
            filter.layer_2_block_number, expected_onchain_block_number
        );
        return Err(EventHandlerError::MissingBlocks(
            expected_onchain_block_number.as_usize(),
        ));
    }

    // check if we're ahead of the event, this means we've already seen it and we shouldn't process it again
    // This could happen if we've missed some blocks and we're re-synchronising or even during chain reorg.
    if *expected_onchain_block_number > filter.layer_2_block_number {
        warn!(
            "Already processed layer 2 block {} - skipping",
            filter.layer_2_block_number
        );
        return Ok(());
    }

    *expected_onchain_block_number += 1;

    // warn that we're not synced with the blockchain if we're behind
    let current_block_number = N::get_current_layer2_blocknumber().await.map_err(|_| {
        EventHandlerError::IOError("Could not retrieve current block number".to_string())
    })?;

    let delta = current_block_number - filter.layer_2_block_number - I256::one();
    println!(
        "Current block number is {}, delta is {}",
        current_block_number, delta
    );
    // if we"re synchronising, we don"t want to check for duplicate keys because we expect to overwrite commitments already in the commitment collection
    let dup_key_check = if delta != I256::zero() {
        warn!(
            "Synchronising - behind blockchain by {} layer 2 blocks ",
            delta
        );
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
    debug!(
        "New commitments tree root is {}, old root was {}",
        root, old_root
    );
    // The root should be the same as the one in the block. This is worth checking
    let historic_root = FrBn254::try_from(blk.commitments_root)
        .map_err(|_| {
            EventHandlerError::IOError("Could not convert commitment to Fr254".to_string())
        })?
        .into();
    if root != historic_root {
        error!("Commitment root in block does not match calculated root. historic root is {}, calculated root is {}", historic_root, root);
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
            Some(filter.layer_2_block_number)
        ),
        db.mark_commitments_spent(nullifiers)
    );

    debug!("Updating request status for confirmed commitments");
    for commitment_hash in &commitment_hashes {
        let commitment_hex = commitment_hash.to_hex_string();
        if let Some(request_ids) = db.get_requests_by_commitment(&commitment_hex).await {
            for request_id in request_ids {
                debug!("Marking request {} as confirmed", request_id);
                db.update_request(&request_id, RequestStatus::Confirmed)
                    .await;
            }
        }
    }

    // now attempt to decrypt the compressed secrets to see which commitments (if any) we own
    let mut commitment_entries = vec![];
    for transaction in blk.transactions.iter() {
        // If all the nullifiers are zero we can skip to the next transaction
        if transaction.nullifiers == [U256::zero(); 4] {
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
            debug!("Commitment {} is not owned by us", commitment_hash);
        } else {
            info!("Received commitment owned by us, with hash {}", test_hash);
            // store our newly received commitment in our commitment db
            let nullifier = test_preimage
                .nullifier_hash(&nullifier_key)
                .map_err(|_| EventHandlerError::HashError)?;
            let commitment_entry = CommitmentEntry::new(
                test_preimage,
                nullifier,
                CommitmentStatus::Unspent,
            );
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
    debug!("Using webhook URL: {}", webhook_url);
    let mut publisher = DataPublisher::new();
    let notifier = WebhookNotifier::new(webhook_url);

    publisher.register_notifier(Box::new(notifier));

    // Let's get the full hash as it gets truncated otherwise
    let l1_txn_hash = format!("{:#x}", transaction_hash);
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
        l2_block_number: filter.layer_2_block_number,
        commitments: owned_commitment_hashes,
        request_ids,
    };

    publisher.publish(notification).await;
    Ok(())
}

async fn process_deposit_escrowed_event(
    transaction_hash: H256,
    filter: &DepositEscrowedFilter,
) -> Result<(), EventHandlerError> {
    info!(
        "Client: Decoded DepositEscrowed event from transaction {}, Deposit Transaction with nf_slot_id {}, value {}, is now on-chain",
        transaction_hash, filter.nf_slot_id, filter.value,
    );

    Ok(())
}
