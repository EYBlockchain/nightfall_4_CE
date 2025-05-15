use crate::domain::{entities::SynchronisationStatus, error::EventHandlerError};
use crate::driven::event_handlers::nightfall_event::get_expected_layer2_blocknumber;
use crate::ports::contracts::NightfallContract;
use crate::services::process_events::process_events;
use configuration::addresses::get_addresses;
use configuration::settings::get_settings;
use ethers::prelude::*;
use std::time::Duration;
use tokio::time::sleep;
use futures::{future::BoxFuture, FutureExt};
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, warn};
use nightfall_bindings::nightfall::Nightfall;
use std::panic;

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
            log::info!("Client event listener (attempt {})...", attempts);
            let result = listen_for_events::<N>(start_block).await;

            match result {
                Ok(_) => {
                    log::info!("Client event listener finished successfully.");
                    break;
                }
                Err(e) => {
                    log::error!(
                        "Client event listener terminated with error: {:?}. Restarting in {:?}",
                        e,
                        backoff_delay
                    );
                    if attempts >= max_attempts {
                        log::error!("Client event listener: max attempts reached. Giving up.");
                        if let Err(err) = notify_failure_client("Client event listener failed after max retries").await {
                            log::error!("Failed to send failure notification (client): {:?}", err);
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
    log::error!("ALERT: {}", message);
    Ok(())
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<N: NightfallContract>(
    start_block: usize,
) -> Result<(), EventHandlerError> {
    let nightfall_instance = Nightfall::new(
        get_addresses().nightfall(),
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    log::info!(
        "Listening for events on the Nightfall contract at address: {}",
        get_addresses().nightfall()
    );
    let events = nightfall_instance.events().from_block(start_block);
    let mut stream = events
        .subscribe_with_meta()
        .await
        .map_err(|_| EventHandlerError::NoEventStream)?;
    while let Some(Ok(evt)) = stream.next().await {
        // process each event in the stream and handle any errors
        let result = process_events::<N>(evt.0, evt.1).await;
        match result {
            Ok(_) => continue,
            Err(e) => {
                match e {
                    // we're missing blocks, so we need to re-synchronise
                    EventHandlerError::MissingBlocks(n) => {
                        warn!("Missing blocks. Last contiguous block was {}. Restarting event listener", n);
                        restart_event_listener::<N>(start_block).await;
                        return Err(EventHandlerError::StreamTerminated);
                    }
                    _ => panic!("Error processing event: {:?}", e),
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
        .is_synchronised();
    if sync_state {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    let settings = get_settings();
    let max_attempts = settings.nightfall_client.max_event_listener_attempts.unwrap_or(10); 

    start_event_listener::<N>(start_block, max_attempts).await;
}

pub async fn get_synchronisation_status<N: NightfallContract>(
) -> Result<SynchronisationStatus, EventHandlerError> {
    let expected_block = get_expected_layer2_blocknumber().lock().await;
    let current_block = N::get_current_layer2_blocknumber()
        .await
        .map_err(|_| EventHandlerError::IOError("Could not read current block".to_string()))?;
    debug!(
        "Expected block: {}, Current block: {}",
        *expected_block, current_block
    );
    Ok(SynchronisationStatus::new(*expected_block == current_block))
}
