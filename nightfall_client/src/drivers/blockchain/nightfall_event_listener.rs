use crate::{
    domain::{
        entities::{OnChainTransaction, SynchronisationPhase, SynchronisationStatus},
        error::EventHandlerError,
    },
    driven::db::mongo::{BlockStorageDB, StoredBlock},
    driven::event_handlers::nightfall_event::get_expected_layer2_blocknumber,
    drivers::blockchain::nightfall_event_listener::SynchronisationPhase::Synchronized,
    initialisation::get_db_connection,
    ports::{contracts::NightfallContract, trees::CommitmentTree},
    services::process_events::process_events,
};
use alloy::{primitives::I256, rpc::types::Filter, sol_types::{SolEvent, SolEventInterface}};
use ark_bn254::Fr as Fr254;
use configuration::{addresses::get_addresses, settings::get_settings};
use futures::{StreamExt};
use futures::{future::BoxFuture, FutureExt};
use lib::{
    blockchain_client::BlockchainClientConnection, hex_conversion::HexConvertible,
    initialisation::get_blockchain_client_connection,
};
use log::{debug, warn};
use mongodb::Client as MongoClient;
use nightfall_bindings::artifacts::Nightfall;
use std::{panic, time::Duration};
use tokio::time::sleep;
/// This function starts the event handler. It will attempt to restart the event handler in case of errors
/// with an exponential backoff  for a configurable number of attempts. If the event handler
/// fails after the maximum number of attempts, it will log an error and send a notification (if configured).
pub fn start_event_listener<N: NightfallContract>(
    start_block: usize,
    max_attempts: u32, //max attempts to restart the event listener
) -> BoxFuture<'static, ()> {
    async move {
        let mut attempts = 0;
        let mut backoff_delay = Duration::from_secs(2);
        let max_attempts = std::cmp::max(1, max_attempts);

        loop {
            attempts += 1;
            log::info!("Client event listener (attempt {attempts})...");
            let result = listen_for_events::<N>(start_block).await;
            match result {
                Ok(_) => {
                    log::info!("Client event listener finished successfully.");
                    break;
                }
                Err(e) => {
                    log::error!(
                        "Client event listener terminated with error: {e:?}. Restarting in {backoff_delay:?}"
                    );
                    if attempts >= max_attempts {
                        log::error!("Client event listener: max attempts reached. Giving up.");
                        if let Err(err) =
                            notify_failure_client("Client event listener failed after max retries")
                                .await
                        {
                            log::error!("Failed to send failure notification (client): {err:?}");
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
async fn notify_failure_client(message: &str) -> Result<(), ()> {
    // Here we can implement the logic to notify the failure, e.g, sending a message or an alert
    // for now, we'll just log the error
    log::error!("ALERT: {message}");
    Ok(())
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<N: NightfallContract>(
    start_block: usize,
) -> Result<(), EventHandlerError> {
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    log::info!(
        "Listening for events on the Nightfall contract at address: {}",
        get_addresses().nightfall()
    );

    // get the events from the Nightfall contract from the specified start block
    // Subscribe to the combined events filter
    let events_filter = Filter::new().address(get_addresses().nightfall())
    .event_signature(vec![
        Nightfall::BlockProposed::SIGNATURE_HASH,
        Nightfall::DepositEscrowed::SIGNATURE_HASH,
    ])
        .from_block(start_block as u64);
    
    // Subscribe to the combined events filter
    let events_subscription = blockchain_client.subscribe_logs(&events_filter).await
        .map_err(|_| EventHandlerError::NoEventStream)?;
    
    let mut events_stream = events_subscription.into_stream();
    while let Some(evt) = events_stream.next().await {
        // process each event in the stream and handle any errors
        let event = Nightfall::NightfallEvents::decode_log(&evt.inner).unwrap();
        let result = process_events::<N>(event.data, evt).await;
        match result {
            Ok(_) => continue,
            Err(e) => {
                match e {
                    // we're missing blocks, so we need to re-synchronise
                    EventHandlerError::MissingBlocks(n) => {
                        warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
                        restart_event_listener::<N>(start_block).await;
                        return Err(EventHandlerError::StreamTerminated);
                    }
                    _ => panic!("Error processing event: {e:?}"),
                }
            }
        }
    }

    Err(EventHandlerError::StreamTerminated)
}

// We might need to restart the event listener if we fall out of sync and lose blocks
// This does not erase aleady synchronised data
pub async fn restart_event_listener<N>(start_block: usize)
where
    N: NightfallContract,
{
    // if we're restarting the event lister, we definitely shouldn't be in sync, so check that's the case
    let sync_state = get_synchronisation_status::<N>()
        .await
        .expect("Could not check synchronisation state")
        .phase();
    if sync_state == Synchronized {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    let settings = get_settings();
    let max_attempts = settings
        .nightfall_client
        .max_event_listener_attempts
        .unwrap_or(10);

    // clean the database and reset the trees
    // this is a bit of a hack, but we need to reset the trees to get them back in sync
    // with the blockchain. We should probably do this in a more elegant way, but this works for now
    // and we can improve it later
    {
        let db = get_db_connection().await;
        let _ = <MongoClient as CommitmentTree<Fr254>>::reset_tree(db).await;
    }

    start_event_listener::<N>(start_block, max_attempts).await;
}

pub async fn get_synchronisation_status<N: NightfallContract>(
) -> Result<SynchronisationStatus, EventHandlerError> {
    let expected_block_number = get_expected_layer2_blocknumber().lock().await;
    let current_block_number = N::get_current_layer2_blocknumber()
        .await
        .map_err(|_| EventHandlerError::IOError("Could not read current block".to_string()))?;

    if *expected_block_number < current_block_number {
        warn!(
            "Client is behind chain: expected block {} < current block {}",
            *expected_block_number, current_block_number
        );
        return Ok(SynchronisationStatus::new(
            SynchronisationPhase::Desynchronized,
        ));
    }

    if *expected_block_number > current_block_number {
        let delta = *expected_block_number - current_block_number;
        warn!(
            "Client is ahead of chain: expected block {} > current block {}",
            *expected_block_number, current_block_number
        );
        return Ok(SynchronisationStatus::new(
            SynchronisationPhase::AheadOfChain {
                blocks_ahead: delta.as_usize(),
            },
        ));
    }

    // expected == current
    let i256_val = *expected_block_number;
    assert!(
        i256_val >= I256::ZERO,
        "expected_block_number is negative: {i256_val}"
    );

    let expected_u64: u64 = i256_val
        .try_into()
        .expect("expected_block_number must be within u64 range");

    let db = get_db_connection().await;

    match db.get_block_by_number(expected_u64).await {
        Some(stored_block) => {
            let stored_hash = stored_block.hash();
            let (proposer_address, block_onchain) =
                N::get_layer2_block_by_number(current_block_number)
                    .await
                    .map_err(|_| {
                        EventHandlerError::IOError(
                            "Could not read block from blockchain".to_string(),
                        )
                    })?;
println!("we are here inside get_synchronisation_status");
            let store_block_pending = StoredBlock {
                layer2_block_number: expected_u64,
                commitments: block_onchain
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

            let expected_hash = store_block_pending.hash();

            if expected_hash != stored_hash {
                warn!(
                    "Hash mismatch at block {expected_u64}: expected {expected_hash}, found {stored_hash}"
                );
                return Ok(SynchronisationStatus::new(
                    SynchronisationPhase::Desynchronized,
                ));
            }
            // If hashes match, fall through and return Synchronized
            debug!("Block {expected_u64} verified in local DB with matching hash.");
            Ok(SynchronisationStatus::new(
                SynchronisationPhase::Synchronized,
            ))
        }
        None => {
            debug!("Block {expected_u64} not found in local DB. Assuming client is still in sync.");
            Ok(SynchronisationStatus::new(
                SynchronisationPhase::Synchronized,
            ))
        }
    }
}
