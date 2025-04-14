use crate::domain::{entities::SynchronisationStatus, error::EventHandlerError};
use crate::driven::event_handlers::nightfall_event::get_expected_layer2_blocknumber;
use crate::ports::contracts::NightfallContract;
use crate::services::process_events::process_events;
use configuration::addresses::get_addresses;
use ethers::prelude::*;
use futures::{future::BoxFuture, FutureExt};
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, warn};
use nightfall_bindings::nightfall::Nightfall;
use std::panic;

/// This function starts the event handler. It will attempt to restart the event handler in case of errors
/// for a bit, before giving up and panicing.
pub fn start_event_listener<N: NightfallContract>(
    allowed_failures: u32,
    start_block: usize,
) -> BoxFuture<'static, ()> {
    // we use the async block and the BoxFuture so that we can recurse an async
    async move {
        let result = listen_for_events::<N>(start_block).await;
        match result {
            Ok(_) => {
                panic!("It should not be possible for the event listener to terminate without an error");
            }
            Err(e) => {
                log::error!(
                    "Event listener terminated with error: {:?}. Attempting restart in 10 seconds",
                    e
                );
                if allowed_failures > 50 {
                    panic!("Unable to start event listener");
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                start_event_listener::<N>(allowed_failures + 1, start_block).await;
            }
        }
    }
    .boxed()
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
    start_event_listener::<N>(0, start_block).await;
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
