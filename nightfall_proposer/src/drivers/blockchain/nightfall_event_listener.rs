use crate::{
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        contracts::NightfallContract,
        db::TransactionsDB,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
    services::process_events::process_events,
};
use ark_bn254::Fr as Fr254;
use configuration::{addresses::get_addresses, settings::get_settings};
use ethers::prelude::*;
use futures::{future::BoxFuture, FutureExt};
use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, warn};
use mongodb::Client as MongoClient;
use nightfall_bindings::nightfall::Nightfall;
use nightfall_client::{
    domain::{
        entities::{SynchronisationPhase::Desynchronized, SynchronisationStatus},
        error::EventHandlerError,
    },
    ports::proof::{Proof, ProvingEngine},
};
use std::time::Duration;
use tokio::{
    sync::{OnceCell, RwLock},
    time::sleep,
};

/// This function starts the event handler. It will attempt to restart the event handler in case of errors
/// with an exponential backoff for a configurable number of attempts. If the event handler
/// fails after the maximum number of attempts, it will log an error and send a notification (if configured)
pub fn start_event_listener<P, E, N>(
    start_block: usize,
    max_attempts: u32,
) -> BoxFuture<'static, ()>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    debug!("Starting event listener");
    // we use the async block and the BoxFuture so that we can recurse an async
    async move {
        let mut attempts = 0;
        let mut backoff_delay = Duration::from_secs(2);
        let max_attempts = std::cmp::max(1, max_attempts);

        loop {
            attempts += 1;
            log::info!("Proposer event listener (attempt {})...", attempts);
            let result = listen_for_events::<P, E, N>(start_block).await;

            match result {
                Ok(_) => {
                    log::info!("Proposer event listener finished successfully.");
                    break;
                }
                Err(e) => {
                    log::error!(
                        "Proposer event listener terminated with error: {:?}. Restarting in {:?}",
                        e,
                        backoff_delay
                    );
                    if attempts >= max_attempts {
                        log::error!("Proposer event listener: max attempts reached. Giving up.");
                        if let Err(err) = notify_failure_proposer(
                            "Proposer event listener failed after max retries",
                        )
                        .await
                        {
                            log::error!(
                                "Failed to send failure notification (proposer): {:?}",
                                err
                            );
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
async fn notify_failure_proposer(message: &str) -> Result<(), ()> {
    log::error!("ALERT (Proposer): {}", message);
    Ok(())
}

// This function listens for events and processes them. It's started by the start_event_listener function
pub async fn listen_for_events<P, E, N>(start_block: usize) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
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
    if sync_state {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    // clean the database and reset the trees
    // this is a bit of a hack, but we need to reset the trees to get them back in sync
    // with the blockchain. We should probably do this in a more elegant way, but this works for now
    // and we can improve it later
    {
        let db = get_db_connection().await;
        let _ = <MongoClient as CommitmentTree<Fr254>>::reset_tree(db).await;
        let _ = <MongoClient as HistoricRootTree<Fr254>>::reset_tree(db).await;
        let _ = <MongoClient as NullifierTree<Fr254>>::reset_tree(db).await;
        // clean up the mempool, otherwise proposer gets duplicated transactions everytime it syncs

        let removed_deposits = TransactionsDB::<P>::remove_all_mempool_deposits(db).await;
        let removed_client_txs =
            TransactionsDB::<P>::remove_all_mempool_client_transactions(db).await;

        debug!(
            "Mempool cleanup: removed {} deposits and {} client transactions.",
            removed_deposits.unwrap_or(0),
            removed_client_txs.unwrap_or(0)
        );
    }

    let settings = get_settings();
    let max_attempts = settings
        .nightfall_proposer
        .max_event_listener_attempts
        .unwrap_or(10);

    start_event_listener::<P, E, N>(start_block, max_attempts).await;
}

pub async fn get_synchronisation_status() -> &'static RwLock<SynchronisationStatus> {
    static SYNCHRONISATION_STATUS: OnceCell<RwLock<SynchronisationStatus>> = OnceCell::const_new();
    SYNCHRONISATION_STATUS
        .get_or_init(|| async { RwLock::new(SynchronisationStatus::new(Desynchronized)) })
        .await
}
