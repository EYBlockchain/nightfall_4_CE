use crate::ports::contracts::NightfallContract;
use crate::{
    initialisation::get_blockchain_client_connection, services::process_events::process_events,
};
use configuration::addresses::get_addresses;
use ethers::prelude::*;
use futures::{future::BoxFuture, FutureExt};
use lib::blockchain_client::BlockchainClientConnection;
use log::warn;
use nightfall_bindings::nightfall::Nightfall;
use nightfall_client::domain::entities::SynchronisationStatus;
use nightfall_client::{
    domain::error::EventHandlerError,
    ports::proof::{Proof, ProvingEngine},
};
use tokio::sync::{OnceCell, RwLock};

/// This function starts the event handler. It will attempt to restart the event handler in case of errors
/// for a bit, before giving up and panicing.
pub fn start_event_listener<P, E, N>(
    allowed_failures: u32,
    start_block: usize,
) -> BoxFuture<'static, ()>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    ark_std::println!("Starting event listener");
    // we use the async block and the BoxFuture so that we can recurse an async
    async move {
        let result = listen_for_events::<P, E, N>(start_block).await;
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
                start_event_listener::<P, E, N>(allowed_failures + 1, start_block).await;
            }
        }
    }
    .boxed()
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<P, E, N>(start_block: usize) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    ark_std::println!("Listening for events at start_block: {}", start_block);
    let nightfall_instance = Nightfall::new(
        get_addresses().nightfall,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    let events = nightfall_instance.events().from_block(start_block);

    let mut stream = events
        .subscribe_with_meta()
        .await
        .map_err(|_| EventHandlerError::NoEventStream)?;
    while let Some(Ok(evt)) = stream.next().await {
        // process each event in the stream and handle any errors
        let result = process_events::<P, E, N>(evt.0, evt.1).await;
        match result {
            Ok(_) => continue,
            Err(e) => {
                match e {
                    // we're missing blocks, so we need to re-synchronise
                    EventHandlerError::MissingBlocks(n) => {
                        warn!("Missing blocks. Last contiguous block was {}. Restarting event listener", n);
                        restart_event_listener::<P, E, N>(start_block).await;
                        return Err(EventHandlerError::StreamTerminated);
                    }

                    EventHandlerError::BlockHashError(expected, found) => {
                        warn!(
                            "Block hash mismatch: expected {:?}, found {:?}. Restarting event listener",
                            expected, found
                        );
                        restart_event_listener::<P, E, N>(start_block).await;
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
pub async fn restart_event_listener<P, E, N>(start_block: usize)
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    // if we're restarting the event lister, we definitely shouldn't be in sync, so check that's the case
    let sync_state = get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised();
    ark_std::println!("Restarting event listener, synchronisation state: {}", sync_state);
    if sync_state {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    start_event_listener::<P, E, N>(0, start_block).await;
}

pub async fn get_synchronisation_status() -> &'static RwLock<SynchronisationStatus> {
    static SYNCHRONISATION_STATUS: OnceCell<RwLock<SynchronisationStatus>> = OnceCell::const_new();
    SYNCHRONISATION_STATUS
        .get_or_init(|| async { RwLock::new(SynchronisationStatus::new(false)) })
        .await
}
