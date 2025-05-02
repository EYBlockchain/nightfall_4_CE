use std::time::Duration;

use std::collections::VecDeque;
use log::info;
use tokio::sync::{OnceCell, RwLock};
use tokio::time::sleep;
use crate::drivers::rest::client_nf_3::handle_request;
use crate::drivers::rest::models::{NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest};
use crate::ports::contracts::NightfallContract;
use crate::ports::proof::{Proof, ProvingEngine};

/// This module implements a queue of received requests. Requests can be added to the queue
/// asynchronously but are executed with a concurrency of 1.
///
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
    QUEUE.get_or_init(|| async {
        RwLock::new(VecDeque::<QueuedRequest>::with_capacity(10))})
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
    let queue = get_queue();
    let mut queue = queue.await.write().await;
    loop {
        while let Some(request) = queue.pop_front() {
            // Process the request here with a concurrency of 1
            match handle_request::<P, E, N>(request.transaction_request, &request.uuid).await {
                Ok(response) => {
                    info!("{} Request processed successfully: ", request.uuid);
                }
                Err(e) => {
                    // Handle the error here
                    eprintln!("Error processing request: {:?}", e);
                }
            }
        }
        // If the queue is empty, wait a bit then try again
        sleep(Duration::from_secs(1)).await;
    }
}
