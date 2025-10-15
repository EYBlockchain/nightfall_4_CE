use crate::{
    domain::entities::RequestStatus,
    driven::notifier::webhook_notifier::WebhookNotifier,
    drivers::{
        blockchain::{
            event_listener_manager::restart, nightfall_event_listener::get_synchronisation_status,
        },
        rest::{
            client_nf_3::handle_request,
            models::{NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest},
        },
    },
    initialisation::get_db_connection,
    ports::{contracts::NightfallContract, db::RequestDB},
    services::data_publisher::DataPublisher,
};
use configuration::settings::get_settings;
use lib::{
    nf_client_proof::{Proof, ProvingEngine},
    shared_entities::SynchronisationPhase::Desynchronized,
};
use log::{debug, error, info, warn};
use std::{collections::VecDeque, time::Duration};
use tokio::{
    sync::{OnceCell, RwLock},
    time::sleep,
};
/// This module implements a queue of received requests. Requests can be added to the queue
/// asynchronously but are executed with a concurrency of 1.
pub struct QueuedRequest {
    pub transaction_request: TransactionRequest,
    pub uuid: String,
}
pub enum TransactionRequest {
    Deposit(NF3DepositRequest),
    Transfer(NF3TransferRequest),
    Withdraw(NF3WithdrawRequest),
}

/// This function is used to provide a singleton request queue across the entire application.
pub async fn get_queue() -> &'static RwLock<VecDeque<QueuedRequest>> {
    static QUEUE: OnceCell<RwLock<VecDeque<QueuedRequest>>> = OnceCell::const_new();
    QUEUE
        .get_or_init(|| async { RwLock::new(VecDeque::<QueuedRequest>::with_capacity(10)) })
        .await
}

/// This function is used to process the queue. It will run in a loop and process requests
/// as they come in. It will wait for 1 second if the queue is empty before checking again.
/// This function should be run in a separate thread or task.
pub async fn process_queue<P, E, N>()
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    // register a notifier to publish to the webhook URL
    let mut publisher = DataPublisher::new();
    let webhook_url = &get_settings().nightfall_client.webhook_url;
    debug!("Using webhook URL: {webhook_url}");
    let notifier = WebhookNotifier::new(webhook_url);
    publisher.register_notifier(Box::new(notifier));

    loop {
        while let Some(request) = {
            let mut queue = get_queue().await.write().await;
            let request = queue.pop_front();
            drop(queue); // drop the lock here so we don't hold up the queue while processing the request
            request
        } {
            // Process the request here with a concurrency of 1
            // mark request as 'Processing'
            info!("Processing request: {}", request.uuid);
            //first check the sync status
            let sync_state = match get_synchronisation_status::<N>().await {
                Ok(status) => status.phase(),
                Err(e) => {
                    error!("Failed to get synchronisation status: {e:?}");
                    return;
                }
            };
            if sync_state == Desynchronized {
                warn!("Client is not synchronised with the blockchain, restarting event listener on thread {:?}", std::thread::current().id());
                restart::<N>().await;
            }
            let db = get_db_connection().await;
            let _ = db
                .update_request(&request.uuid, RequestStatus::Processing)
                .await; // we'll carry on even if this fails
            match handle_request::<P, E, N>(request.transaction_request, &request.uuid).await {
                Ok(response) => {
                    let db = get_db_connection().await;
                    let _ = db
                        .update_request(&request.uuid, RequestStatus::Submitted)
                        .await;
                    info!("Request {} processed successfully: ", request.uuid);
                    if webhook_url.is_empty() {
                        warn!("No webhook URL provided, skipping notification of successful transaction");
                    } else {
                        // Publish the notification
                        info!("Response: {response:?}");
                        publisher.publish(response).await;
                    }
                }
                Err(e) => {
                    // Handle the error here
                    let db = get_db_connection().await;
                    let _ = db
                        .update_request(&request.uuid, RequestStatus::Failed)
                        .await;
                    warn!("{} Error processing request: {:?}", request.uuid, e);
                }
            }
        }
        // If the queue is empty, wait a bit then try again
        sleep(Duration::from_secs(1)).await;
    }
}
