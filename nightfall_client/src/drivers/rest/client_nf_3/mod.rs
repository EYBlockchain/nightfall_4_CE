mod deposit;
mod transfer;
mod withdraw;

pub use deposit::handle_deposit;

use super::client_operation::handle_client_operation;
use crate::{
    domain::{
        entities::{
            CommitmentStatus, ERCAddress, Operation, OperationType, RequestStatus, Transport,
        },
        error::TransactionHandlerError,
        notifications::NotificationPayload,
    },
    driven::{
        db::mongo::CommitmentEntry,
        queue::{get_queue, QueuedRequest, TransactionRequest},
    },
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB, RequestDB},
    },
    services::{
        client_operation::deposit_operation, commitment_selection::find_usable_commitments,
    },
};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine;
use ark_ff::{BigInteger256, Zero};
use ark_std::{rand::thread_rng, UniformRand};
use configuration::{addresses::get_addresses, settings::get_settings};
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::{
    client_models::{DeEscrowDataReq, NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest},
    commitments::{Commitment, Nullifiable},
    contract_conversions::FrBn254,
    derive_key::ZKPKeys,
    get_fee_token_id,
    hex_conversion::HexConvertible,
    nf_client_proof::{Proof, ProvingEngine},
    nf_token_id::to_nf_token_id_from_str,
    plonk_prover::circuits::DOMAIN_SHARED_SALT,
    serialization::ark_de_hex,
    shared_entities::{DepositSecret, Preimage, Salt, TokenType},
};
use tracing::{debug, error, info, warn};
use nf_curves::ed_on_bn254::{BJJTEAffine as JubJub, BabyJubjub, Fr as BJJScalar};
use nightfall_bindings::artifacts::{Nightfall, IERC1155, IERC20, IERC3525, IERC721};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{
    hyper::StatusCode,
    path,
    reply::{self, json, Reply},
    Filter,
};
#[derive(Serialize, Deserialize)]
pub struct WithdrawResponse {
    success: bool,
    message: String,
    pub withdraw_fund_salt: String, // Return the withdraw_fund_salt
}
// A simplified client interface, which provides Deposit, Transfer and Withdraw operations,
// with automated commitment selection, but without the flexibility of the lower-level
// client_operation API.
// It matches the API of NF_3 so it can be used with the NF_3 client, under the hood, it calls
// the client_operation handler

pub fn deposit_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "deposit")
        .and(warp::body::json())
        .and_then(queue_deposit_request)
}

pub fn transfer_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "transfer")
        .and(warp::body::json())
        .and_then(queue_transfer_request)
}

pub fn withdraw_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and_then(queue_withdraw_request)
}

/// function to queue the deposit requests
async fn queue_deposit_request(
    deposit_req: NF3DepositRequest,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Deposit(deposit_req);
    let uuid_string = Uuid::new_v4().to_string();

    debug!("Queueing deposit request");
    queue_request(transaction_request, uuid_string).await
}

/// function to queue the transfer requests
async fn queue_transfer_request(
    transfer_req: NF3TransferRequest,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Transfer(transfer_req);
    let uuid_string = Uuid::new_v4().to_string();

    queue_request(transaction_request, uuid_string).await
}

/// function to queue the withdraw requests
async fn queue_withdraw_request(
    withdraw_req: NF3WithdrawRequest,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Withdraw(withdraw_req);
    let uuid_string = Uuid::new_v4().to_string();

    queue_request(transaction_request, uuid_string).await
}

/// This function queues all types of transaction request
async fn queue_request(
    transaction_request: TransactionRequest,
    request_id: String,
) -> Result<impl Reply, warp::Rejection> {
    let settings = get_settings();
    let max_queue_size: usize = settings
        .nightfall_client
        .max_queue_size
        .unwrap_or(1000)
        .try_into()
        .map_err(|_| {
            warp::reject::custom(crate::domain::error::ClientRejection::DatabaseError)
        })?;

    // check if the id is a valid uuid
    if Uuid::parse_str(&request_id).is_err() {
        return Err(warp::reject::custom(
            crate::domain::error::ClientRejection::InvalidRequestId,
        ));
    };

    // add the request to the queue
    debug!("Adding request to queue");
    let mut q = get_queue().await.write().await;
    // check if the queue is full
    if q.len() >= max_queue_size {
        return Ok(reply::with_header(
            reply::with_status(
                json(&"Queue is full".to_string()),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
            "X-Request-ID",
            request_id,
        ));
    }
    debug!("got lock on queue");
    q.push_back(QueuedRequest {
        transaction_request,
        uuid: request_id.clone(),
    });
    drop(q); // drop the lock so other processes can access the queue
    debug!("Added request to queue");
    // record the request as queued
    let db = get_db_connection().await;
    if db
        .store_request(&request_id, RequestStatus::Queued)
        .await
        .is_none()
    {
        return Err(warp::reject::custom(
            crate::domain::error::ClientRejection::DatabaseError,
        ));
    }
    debug!("Stored request status in database");

    // return a 202 Accepted response with the request ID
    Ok(reply::with_header(
        reply::with_status(json(&"Request queued".to_string()), StatusCode::ACCEPTED),
        "X-Request-ID",
        request_id,
    ))
}

/// This function wraps the various transaction handlers, so that the queue can call the correct handler
/// based on the request type.
pub async fn handle_request<P, E, N>(
    request: TransactionRequest,
    request_id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    match request {
        TransactionRequest::Deposit(deposit_req) => {
            handle_deposit::<N>(deposit_req, request_id).await
        }
        TransactionRequest::Transfer(transfer_req) => {
            transfer::handle_transfer::<P, E, N>(transfer_req, request_id).await
        }
        TransactionRequest::Withdraw(withdraw_req) => {
            withdraw::handle_withdraw::<P, E, N>(withdraw_req, request_id).await
        }
    }
}
