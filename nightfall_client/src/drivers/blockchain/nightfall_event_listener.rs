use crate::{
    driven::{
        db::mongo::{BlockStorageDB, StoredBlock},
        event_handlers::nightfall_event::get_expected_layer2_blocknumber,
    },
    drivers::blockchain::event_listener_manager::restart,
    initialisation::get_db_connection,
    ports::contracts::NightfallContract,
    services::process_events::process_events,
};
use alloy::{
    primitives::I256,
    rpc::types::Filter,
    sol_types::{SolEvent, SolEventInterface},
};
use configuration::addresses::get_addresses;
use futures::StreamExt;
use futures::{future::BoxFuture, FutureExt};
use lib::log_fetcher::get_logs_paginated;
use lib::{
    blockchain_client::BlockchainClientConnection,
    error::EventHandlerError,
    hex_conversion::HexConvertible,
    initialisation::get_blockchain_client_connection,
    shared_entities::{OnChainTransaction, SynchronisationPhase, SynchronisationStatus},
};
use log::{debug, info, warn};
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
            println!("inside loop to call listen for events on thread");
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
    info!(
        "Listening for events on the Nightfall contract at address: {}",
        get_addresses().nightfall()
    );

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
                let result = process_events::<N>(event.data, evt).await;
                match result {
                    Ok(_) => continue,
                    Err(e) => {
                        match e {
                            // we're missing blocks, so we need to re-synchronise
                            EventHandlerError::MissingBlocks(n) => {
                                warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
                                restart::<N>().await;
                                return Err(EventHandlerError::StreamTerminated);
                            }
                            _ => panic!("Error processing event: {e:?}"),
                        }
                    }
                }
            }
        } else {
            info!(
                "Start block {start_block} is greater than latest block {latest_block}. No past events to process.",
            );
        }
    }
    let mut events_stream = events_subscription.into_stream();
    info!("Subscribed to events.");

    while let Some(evt) = events_stream.next().await {
        // process each event in the stream and handle any errors
        let event = match Nightfall::NightfallEvents::decode_log(&evt.inner) {
            Ok(e) => e,
            Err(e) => {
                warn!("Failed to decode log: {e:?}");
                continue; // Skip malformed events
            }
        };
        let result = process_events::<N>(event.data, evt).await;
        match result {
            Ok(_) => continue,
            Err(e) => {
                match e {
                    // we're missing blocks, so we need to re-synchronise
                    EventHandlerError::MissingBlocks(n) => {
                        warn!("Missing blocks. Last contiguous block was {n}. Restarting event listener");
                        restart::<N>().await;
                        return Err(EventHandlerError::StreamTerminated);
                    }
                    _ => panic!("Error processing event: {e:?}"),
                }
            }
        }
    }

    Err(EventHandlerError::StreamTerminated)
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
