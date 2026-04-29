use super::client_operation::{
    handle_client_operation, request_swap_cancel, submit_client_operation, SwapParams,
};
use crate::{
    domain::{
        entities::{
            should_overwrite_request_status_with_failed, CommitmentStatus, ERCAddress, Operation,
            OperationType, RequestStatus, Transport,
        },
        error::{ClientRejection, DepositError, TokenContractError, TransactionHandlerError},
        notifications::NotificationPayload,
    },
    driven::{
        db::mongo::CommitmentEntry,
        queue::{get_queue, QueuedRequest, TransactionRequest},
    },
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB, RequestDB},
    },
    services::{
        client_operation::deposit_operation,
        commitment_selection::find_usable_commitments,
        swap_expiry::{
            reconcile_expired_swap_request, swap_deadline_has_passed, SwapChildRequestArgs,
            SwapExpiryError, SwapExpiryStore,
        },
    },
};
use alloy::primitives::I256;
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine;
use ark_ff::{BigInteger, BigInteger256, PrimeField, Zero};
use ark_std::{rand::thread_rng, UniformRand};
use async_trait::async_trait;
use configuration::{addresses::get_addresses, settings::get_settings};
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::{
    client_models::{
        DeEscrowDataReq, NF3CancelSwapRequest, NF3DepositRequest, NF3SettleExpiredSwapRequest,
        NF3SwapRequest, NF3TransferRequest, NF3WithdrawRequest,
    },
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
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::{BJJTEAffine as JubJub, BabyJubjub, Fr as BJJScalar};
use nightfall_bindings::artifacts::{Nightfall, IERC1155, IERC20, IERC3525, IERC721};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    time::{SystemTime, UNIX_EPOCH},
};
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

#[derive(Deserialize)]
struct JubJubPubKey(#[serde(deserialize_with = "ark_de_hex")] JubJub);
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
pub fn swap_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "swap")
        .and(warp::body::json())
        .and_then(queue_swap_request)
}

pub(super) fn parse_token_type(token_type: &str) -> Result<TokenType, String> {
    let normalized = token_type
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    let parsed = u8::from_str_radix(normalized, 16)
        .map_err(|_| "Invalid tokenType: expected hex-encoded value".to_string())?;
    match parsed {
        0 => Ok(TokenType::ERC20),
        1 => Ok(TokenType::ERC1155),
        2 => Ok(TokenType::ERC721),
        3 => Ok(TokenType::ERC3525),
        4 => Ok(TokenType::FeeToken),
        _ => Err("Unsupported tokenType".to_string()),
    }
}

fn validate_asset_constraints(
    token_type: TokenType,
    value: Fr254,
    token_id: BigInteger256,
) -> Result<(), String> {
    match token_type {
        TokenType::ERC20 => {
            if value.is_zero() {
                return Err("ERC20 operations require value > 0".to_string());
            }
            if token_id != BigInteger256::zero() {
                return Err("ERC20 operations require tokenId to be 0".to_string());
            }
            Ok(())
        }
        TokenType::ERC721 => {
            if !value.is_zero() {
                return Err("ERC721 operations require value to be 0".to_string());
            }
            Ok(())
        }
        TokenType::ERC1155 => {
            if value.is_zero() && token_id == BigInteger256::zero() {
                return Err(
                    "ERC1155 operations require either value > 0 or tokenId > 0".to_string()
                );
            }
            Ok(())
        }
        TokenType::ERC3525 => {
            if value.is_zero() {
                return Err(format!("{token_type:?} operations require value > 0"));
            }
            Ok(())
        }
        TokenType::FeeToken => Err("FeeToken is not supported for this operation".to_string()),
    }
}

fn validate_deposit_request_payload(req: &NF3DepositRequest) -> Result<(), String> {
    ERCAddress::try_from_hex_string(&req.erc_address)
        .map_err(|e| format!("Invalid ercAddress: {e}"))?;
    let token_id = BigInteger256::from_hex_string(req.token_id.as_str())
        .map_err(|e| format!("Invalid tokenId: {e}"))?;
    let token_type = parse_token_type(req.token_type.as_str())?;
    let value =
        Fr254::from_hex_string(req.value.as_str()).map_err(|e| format!("Invalid value: {e}"))?;
    Fr254::from_hex_string(req.fee.as_str()).map_err(|e| format!("Invalid fee: {e}"))?;
    Fr254::from_hex_string(req.deposit_fee.as_str())
        .map_err(|e| format!("Invalid deposit_fee: {e}"))?;
    validate_asset_constraints(token_type, value, token_id)
}

fn validate_transfer_request_payload(req: &NF3TransferRequest) -> Result<(), String> {
    to_nf_token_id_from_str(req.erc_address.as_str(), req.token_id.as_str())
        .map_err(|e| format!("Invalid ercAddress/tokenId pair: {e}"))?;
    let token_id = BigInteger256::from_hex_string(req.token_id.as_str())
        .map_err(|e| format!("Invalid tokenId: {e}"))?;
    let token_type = parse_token_type(req.token_type.as_str())?;
    Fr254::from_hex_string(req.fee.as_str()).map_err(|e| format!("Invalid fee: {e}"))?;

    if req.recipient_data.values.len() != 1 {
        return Err("Transfer currently supports exactly one recipient value".to_string());
    }
    if req
        .recipient_data
        .recipient_compressed_zkp_public_keys
        .len()
        != 1
    {
        return Err("Transfer currently supports exactly one recipient public key".to_string());
    }

    let value = Fr254::from_hex_string(req.recipient_data.values[0].as_str())
        .map_err(|e| format!("Invalid transfer value: {e}"))?;
    validate_asset_constraints(token_type, value, token_id)?;

    let first_key = &req.recipient_data.recipient_compressed_zkp_public_keys[0];
    let json_wrapped = format!("\"{first_key}\"");
    let deserialized_public_key: JubJubPubKey = serde_json::from_str(&json_wrapped)
        .map_err(|e| format!("Invalid recipient public key: {e}"))?;
    if deserialized_public_key.0.is_zero() {
        return Err("Recipient public key cannot be the identity point".to_string());
    }

    Ok(())
}

fn validate_withdraw_request_payload(req: &NF3WithdrawRequest) -> Result<(), String> {
    ERCAddress::try_from_hex_string(&req.erc_address)
        .map_err(|e| format!("Invalid ercAddress: {e}"))?;
    let token_id = BigInteger256::from_hex_string(req.token_id.as_str())
        .map_err(|e| format!("Invalid tokenId: {e}"))?;
    let token_type = parse_token_type(req.token_type.as_str())?;
    let value =
        Fr254::from_hex_string(req.value.as_str()).map_err(|e| format!("Invalid value: {e}"))?;
    Fr254::from_hex_string(req.fee.as_str()).map_err(|e| format!("Invalid fee: {e}"))?;
    let recipient_address = Fr254::from_hex_string(req.recipient_address.as_str())
        .map_err(|e| format!("Invalid recipientAddress: {e}"))?;
    if recipient_address.is_zero() {
        return Err("Withdraw operations require a non-zero recipientAddress".to_string());
    }
    validate_asset_constraints(token_type, value, token_id)
}

pub fn cancel_swap_request(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "swap" / "cancel-request")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_cancel_swap_request)
}

pub fn settle_expired_swap_request(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "swap" / "settle-expired")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_settle_expired_swap_request)
}

/// function to queue the deposit requests
async fn queue_deposit_request(
    deposit_req: NF3DepositRequest,
) -> Result<warp::reply::Response, warp::Rejection> {
    if let Err(message) = validate_deposit_request_payload(&deposit_req) {
        error!("Rejecting invalid deposit request: {message}");
        return Ok(reply::with_status(
            json(&serde_json::json!({ "error": message })),
            StatusCode::BAD_REQUEST,
        )
        .into_response());
    }

    let transaction_request = TransactionRequest::Deposit(deposit_req);
    let uuid_string = Uuid::new_v4().to_string();

    debug!("Queueing deposit request");
    queue_request(transaction_request, uuid_string).await
}

/// function to queue the transfer requests
async fn queue_transfer_request(
    transfer_req: NF3TransferRequest,
) -> Result<warp::reply::Response, warp::Rejection> {
    if let Err(message) = validate_transfer_request_payload(&transfer_req) {
        error!("Rejecting invalid transfer request: {message}");
        return Ok(reply::with_status(
            json(&serde_json::json!({ "error": message })),
            StatusCode::BAD_REQUEST,
        )
        .into_response());
    }

    let transaction_request = TransactionRequest::Transfer(transfer_req);
    let uuid_string = Uuid::new_v4().to_string();

    queue_request(transaction_request, uuid_string).await
}

/// function to queue the withdraw requests
async fn queue_withdraw_request(
    withdraw_req: NF3WithdrawRequest,
) -> Result<warp::reply::Response, warp::Rejection> {
    if let Err(message) = validate_withdraw_request_payload(&withdraw_req) {
        error!("Rejecting invalid withdraw request: {message}");
        return Ok(reply::with_status(
            json(&serde_json::json!({ "error": message })),
            StatusCode::BAD_REQUEST,
        )
        .into_response());
    }

    let transaction_request = TransactionRequest::Withdraw(withdraw_req);
    let uuid_string = Uuid::new_v4().to_string();

    queue_request(transaction_request, uuid_string).await
}

/// function to queue the swap requests
async fn queue_swap_request(swap_req: NF3SwapRequest) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Swap(swap_req);
    let uuid_string = Uuid::new_v4().to_string();

    queue_request(transaction_request, uuid_string).await
}

#[derive(Debug, PartialEq, Eq)]
struct SwapCancelExecution {
    status_code: StatusCode,
    cancel_request_status: String,
    matched: usize,
    message: String,
}

#[derive(Debug, PartialEq, Eq)]
struct SettleExpiredExecution {
    status_code: StatusCode,
    unlocked: usize,
    skipped: usize,
    message: &'static str,
}

#[async_trait]
trait SwapCancelStore {
    async fn get_request(&self, request_id: &str) -> Option<crate::domain::entities::Request>;
    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()>;
}

#[async_trait]
impl SwapCancelStore for mongodb::Client {
    async fn get_request(&self, request_id: &str) -> Option<crate::domain::entities::Request> {
        RequestDB::get_request(self, request_id).await
    }

    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()> {
        RequestDB::update_request_child_args(self, request_id, child_args).await
    }
}

#[async_trait]
impl SwapCancelStore for &mongodb::Client {
    async fn get_request(&self, request_id: &str) -> Option<crate::domain::entities::Request> {
        RequestDB::get_request(*self, request_id).await
    }

    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()> {
        RequestDB::update_request_child_args(*self, request_id, child_args).await
    }
}

fn no_child_args_execution() -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped: 0,
        message: "No pending swap commitments found for this request",
    }
}

fn no_unlockable_commitments_execution(skipped: usize) -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped,
        message: "No pending commitments could be unlocked",
    }
}

fn invalid_settle_expired_commitments_execution(skipped: usize) -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped,
        message: "Swap commitments are not all pending spend",
    }
}

fn non_settleable_request_status_execution(status: RequestStatus) -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped: 0,
        message: match status {
            RequestStatus::Queued | RequestStatus::Processing => {
                "Swap request is still being processed and cannot be settled yet"
            }
            RequestStatus::Confirmed => "Swap request is already confirmed on-chain",
            _ => "Swap request is not in a settleable state",
        },
    }
}

fn request_status_allows_settle_expired(status: RequestStatus) -> bool {
    matches!(
        status,
        RequestStatus::Submitted
            | RequestStatus::ProposerUnreachable
            | RequestStatus::Failed
            | RequestStatus::Expired
    )
}

fn deadline_not_passed_execution() -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped: 0,
        message: "Swap deadline has not passed yet; use cancel-request for advisory proposer cancellation while commitments remain locked",
    }
}

fn client_not_synchronised_execution() -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::CONFLICT,
        unlocked: 0,
        skipped: 0,
        message: "Client is not synchronised with L2; unlock safety cannot be determined",
    }
}

fn already_expired_execution() -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::OK,
        unlocked: 0,
        skipped: 0,
        message: "Swap was already expired and commitments were already reconciled",
    }
}

fn already_reconciled_execution() -> SettleExpiredExecution {
    SettleExpiredExecution {
        status_code: StatusCode::OK,
        unlocked: 0,
        skipped: 0,
        message: "Swap was already reconciled locally",
    }
}

fn request_status_allows_cancel_request(status: RequestStatus) -> bool {
    matches!(
        status,
        RequestStatus::Submitted | RequestStatus::ProposerUnreachable | RequestStatus::Failed
    )
}

fn non_cancel_requestable_status_execution(status: RequestStatus) -> SwapCancelExecution {
    SwapCancelExecution {
        status_code: StatusCode::CONFLICT,
        cancel_request_status: "unavailable".to_string(),
        matched: 0,
        message: match status {
            RequestStatus::Queued | RequestStatus::Processing => {
                "Swap request is still being processed and cannot be cancelled yet".to_string()
            }
            RequestStatus::Confirmed => "Swap request is already confirmed on-chain".to_string(),
            RequestStatus::Expired => {
                "Swap request is already expired locally; use settle-expired if reconciliation is still needed"
                    .to_string()
            }
            _ => "Swap request is not in a cancellable state".to_string(),
        },
    }
}

fn advisory_cancel_no_child_args_execution() -> SwapCancelExecution {
    SwapCancelExecution {
        status_code: StatusCode::CONFLICT,
        cancel_request_status: "unavailable".to_string(),
        matched: 0,
        message: "No swap metadata found for advisory cancellation".to_string(),
    }
}

fn advisory_cancel_after_deadline_execution() -> SwapCancelExecution {
    SwapCancelExecution {
        status_code: StatusCode::CONFLICT,
        cancel_request_status: "too_late".to_string(),
        matched: 0,
        message: "Swap deadline has already passed; use settle-expired for trustless local reconciliation".to_string(),
    }
}

fn advisory_cancel_delivery_failed_execution(message: String) -> SwapCancelExecution {
    SwapCancelExecution {
        status_code: StatusCode::OK,
        cancel_request_status: "delivery_failed".to_string(),
        matched: 0,
        message,
    }
}

fn now_unix_timestamp_string() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".to_string(),
    }
}

async fn persist_swap_cancel_metadata(
    db: &impl SwapCancelStore,
    request: &crate::domain::entities::Request,
    cancel_request_status: &str,
    cancel_request_message: &str,
) -> Result<(), ClientRejection> {
    let child_args_json = request
        .child_request_args
        .as_ref()
        .ok_or(ClientRejection::DatabaseError)?;
    let mut child_args: SwapChildRequestArgs =
        serde_json::from_str(child_args_json).map_err(|_| ClientRejection::DatabaseError)?;
    child_args.cancel_requested_at = Some(now_unix_timestamp_string());
    child_args.cancel_request_status = Some(cancel_request_status.to_string());
    child_args.cancel_request_message = Some(cancel_request_message.to_string());

    let child_args_json =
        serde_json::to_string(&child_args).map_err(|_| ClientRejection::DatabaseError)?;
    db.update_request_child_args(&request.uuid, &child_args_json)
        .await
        .ok_or(ClientRejection::DatabaseError)
}

async fn process_cancel_swap_request<F, Fut>(
    db: &impl SwapCancelStore,
    request_id: &str,
    current_l2_block: Option<I256>,
    send_cancel_request: F,
) -> Result<SwapCancelExecution, ClientRejection>
where
    F: FnOnce(Fr254, String) -> Fut,
    Fut: Future<Output = Result<super::client_operation::AggregatedSwapCancelResponse, String>>,
{
    let request = db
        .get_request(request_id)
        .await
        .ok_or(ClientRejection::RequestNotFound)?;

    if !request_status_allows_cancel_request(request.status) {
        return Ok(non_cancel_requestable_status_execution(request.status));
    }

    let Some(child_args_json) = request.child_request_args.as_ref() else {
        return Ok(advisory_cancel_no_child_args_execution());
    };
    let child_args: SwapChildRequestArgs =
        serde_json::from_str(child_args_json).map_err(|_| ClientRejection::DatabaseError)?;
    let swap_link_hex = child_args.swap_link.ok_or(ClientRejection::DatabaseError)?;
    let swap_link =
        Fr254::from_hex_string(&swap_link_hex).map_err(|_| ClientRejection::DatabaseError)?;

    if current_l2_block.is_some_and(|block| swap_deadline_has_passed(&request, block))
        || child_args
            .deadline
            .as_deref()
            .and_then(|deadline| Fr254::from_hex_string(deadline).ok())
            .is_some_and(|deadline| deadline.is_zero())
    {
        return Ok(advisory_cancel_after_deadline_execution());
    }

    let execution = match send_cancel_request(swap_link, request.uuid.clone()).await {
        Ok(response) => SwapCancelExecution {
            status_code: StatusCode::OK,
            cancel_request_status: response.status,
            matched: response.matched,
            message: response.message,
        },
        Err(err) => advisory_cancel_delivery_failed_execution(format!(
            "Advisory cancel request could not be delivered to any proposer: {err}. Commitments remain locked until local trustless settlement"
        )),
    };

    persist_swap_cancel_metadata(
        db,
        &request,
        &execution.cancel_request_status,
        &execution.message,
    )
    .await?;

    Ok(execution)
}

async fn process_settle_expired_swap(
    db: &impl SwapExpiryStore,
    request_id: &str,
    current_l2_block: I256,
) -> Result<SettleExpiredExecution, ClientRejection> {
    let request = db
        .get_request(request_id)
        .await
        .ok_or(ClientRejection::RequestNotFound)?;

    if !request_status_allows_settle_expired(request.status) {
        warn!(
            "{request_id} settle-expired refused: request status {} is not settleable",
            request.status
        );
        return Ok(non_settleable_request_status_execution(request.status));
    }

    if request.child_request_args.is_none() {
        if matches!(request.status, RequestStatus::Expired) {
            info!("{request_id} settle-expired found swap already fully reconciled");
            return Ok(already_expired_execution());
        }
        warn!("{request_id} settle-expired refused: no child_request_args found");
        return Ok(no_child_args_execution());
    }

    if !swap_deadline_has_passed(&request, current_l2_block) {
        warn!(
            "{request_id} settle-expired refused: current L2 block {current_l2_block} has not passed the swap deadline"
        );
        return Ok(deadline_not_passed_execution());
    }

    match reconcile_expired_swap_request(db, &request).await {
        Ok(outcome) => {
            if outcome.unlocked == 0
                && outcome.already_unlocked == 0
                && matches!(request.status, RequestStatus::Expired)
            {
                info!("{request_id} settle-expired found swap already fully reconciled");
                return Ok(already_expired_execution());
            }
            if outcome.unlocked == 0 && outcome.already_unlocked > 0 {
                info!(
                    "{request_id} settle-expired found swap already reconciled by a previous attempt"
                );
                return Ok(already_reconciled_execution());
            }
            info!(
                "{request_id} Trustlessly reconciled expired swap: unlocked={}, already_unlocked={}",
                outcome.unlocked, outcome.already_unlocked
            );
            Ok(SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: outcome.unlocked,
                skipped: 0,
                message: "Swap expired and commitments were unlocked locally",
            })
        }
        Err(SwapExpiryError::MissingChildArgs) => Ok(no_child_args_execution()),
        Err(SwapExpiryError::InvalidChildArgs) => Err(ClientRejection::DatabaseError),
        Err(SwapExpiryError::NoUnlockableCommitments { skipped }) => {
            Ok(no_unlockable_commitments_execution(skipped))
        }
        Err(SwapExpiryError::InvalidCommitmentStates { skipped }) => {
            Ok(invalid_settle_expired_commitments_execution(skipped))
        }
        Err(SwapExpiryError::IncompatibleStatus(status)) => {
            Ok(non_settleable_request_status_execution(status))
        }
        Err(SwapExpiryError::DatabaseError) => Err(ClientRejection::DatabaseError),
    }
}

async fn handle_cancel_swap_request(
    cancel_req: NF3CancelSwapRequest,
) -> Result<impl Reply, warp::Rejection> {
    let request_id = cancel_req.request_id;
    if Uuid::parse_str(&request_id).is_err() {
        return Err(warp::reject::custom(
            crate::domain::error::ClientRejection::InvalidRequestId,
        ));
    }

    let db = get_db_connection().await;
    let current_l2_block =
        <Nightfall::NightfallCalls as NightfallContract>::get_current_layer2_blocknumber()
            .await
            .ok();
    let execution = process_cancel_swap_request(
        &db,
        &request_id,
        current_l2_block,
        |swap_link, id| async move {
            request_swap_cancel(swap_link, &id)
                .await
                .map_err(|e| e.to_string())
        },
    )
    .await
    .map_err(warp::reject::custom)?;

    Ok(reply::with_status(
        json(&serde_json::json!({
            "requestId": request_id,
            "cancelRequestStatus": execution.cancel_request_status,
            "matched": execution.matched,
            "commitmentsRemainLocked": true,
            "message": execution.message
        })),
        execution.status_code,
    ))
}

async fn handle_settle_expired_swap_request(
    settle_req: NF3SettleExpiredSwapRequest,
) -> Result<impl Reply, warp::Rejection> {
    let request_id = settle_req.request_id;
    if Uuid::parse_str(&request_id).is_err() {
        return Err(warp::reject::custom(
            crate::domain::error::ClientRejection::InvalidRequestId,
        ));
    }

    let db = get_db_connection().await;
    if RequestDB::get_request(db, &request_id).await.is_none() {
        return Err(warp::reject::custom(ClientRejection::RequestNotFound));
    }

    let sync_status = get_synchronisation_status::<Nightfall::NightfallCalls>()
        .await
        .map_err(|_| warp::reject::custom(ClientRejection::SynchronisationUnavailable))?;
    if !sync_status.is_synchronised() {
        let execution = client_not_synchronised_execution();
        return Ok(reply::with_status(
            json(&serde_json::json!({
                "requestId": request_id,
                "unlocked": execution.unlocked,
                "skipped": execution.skipped,
                "message": execution.message
            })),
            execution.status_code,
        ));
    }

    let current_l2_block =
        <Nightfall::NightfallCalls as NightfallContract>::get_current_layer2_blocknumber()
            .await
            .map_err(|_| warp::reject::custom(ClientRejection::SynchronisationUnavailable))?;
    let execution = process_settle_expired_swap(&db, &request_id, current_l2_block)
        .await
        .map_err(warp::reject::custom)?;

    Ok(reply::with_status(
        json(&serde_json::json!({
            "requestId": request_id,
            "unlocked": execution.unlocked,
            "skipped": execution.skipped,
            "message": execution.message
        })),
        execution.status_code,
    ))
}

/// This function queues all types of transaction request
async fn queue_request(
    transaction_request: TransactionRequest,
    request_id: String,
) -> Result<warp::reply::Response, warp::Rejection> {
    let settings = get_settings();
    let max_queue_size = settings
        .nightfall_client
        .max_queue_size
        .unwrap_or(1000)
        .try_into()
        .unwrap();

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
        return Ok(queue_full_response(request_id));
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
    Ok(queue_accepted_response(request_id))
}

fn queue_full_response(request_id: String) -> warp::reply::Response {
    reply::with_header(
        reply::with_status(
            json(&"Queue is full".to_string()),
            StatusCode::SERVICE_UNAVAILABLE,
        ),
        "X-Request-ID",
        request_id,
    )
    .into_response()
}

fn queue_accepted_response(request_id: String) -> warp::reply::Response {
    reply::with_header(
        reply::with_status(json(&"Request queued".to_string()), StatusCode::ACCEPTED),
        "X-Request-ID",
        request_id,
    )
    .into_response()
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
            handle_transfer::<P, E, N>(transfer_req, request_id).await
        }
        TransactionRequest::Withdraw(withdraw_req) => {
            handle_withdraw::<P, E, N>(withdraw_req, request_id).await
        }
        TransactionRequest::Swap(swap_req) => handle_swap::<P, E, N>(swap_req, request_id).await,
    }
}

/// handle_client_deposit_request is the entry point for deposit requests from the client.
pub async fn handle_deposit<N: NightfallContract>(
    req: NF3DepositRequest,
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError> {
    info!("Deposit raw request: {req:?}");

    // We convert the request into values
    let NF3DepositRequest {
        erc_address,
        token_id,
        token_type,
        value,
        fee,
        deposit_fee,
        ..
    } = req;

    let erc_address = ERCAddress::try_from_hex_string(&erc_address).map_err(|err| {
        error!("{id} Could not convert ERC address {err}");
        TransactionHandlerError::CustomError(err.to_string())
    })?;

    let token_id: BigInteger256 =
        BigInteger256::from_hex_string(token_id.as_str()).map_err(|err| {
            error!("{id} Could not convert hex string to BigInteger256");
            TransactionHandlerError::CustomError(err.to_string())
        })?;

    let token_type = parse_token_type(token_type.as_str()).map_err(|err| {
        error!("{id} Could not convert token type");
        TransactionHandlerError::CustomError(err)
    })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|err| {
        error!("{id} Could not convert fee");
        TransactionHandlerError::CustomError(err.to_string())
    })?;

    let deposit_fee: Fr254 = Fr254::from_hex_string(deposit_fee.as_str()).map_err(|err| {
        error!("{id} Could not convert deposit fee");
        TransactionHandlerError::CustomError(err.to_string())
    })?;

    let value: Fr254 = Fr254::from_hex_string(value.as_str()).map_err(|err| {
        error!("{id} Could not wrangle value {err}");
        TransactionHandlerError::CustomError(err.to_string())
    })?;

    let (secret_preimage_one, secret_preimage_two, secret_preimage_three) = {
        // RNG is Send and scoped to this block
        let mut rng = thread_rng();
        (
            Fr254::rand(&mut rng),
            Fr254::rand(&mut rng),
            Fr254::rand(&mut rng),
        )
    };

    let secret_preimage = DepositSecret::new(
        secret_preimage_one,
        secret_preimage_two,
        secret_preimage_three,
    );

    let db: &'static mongodb::Client = get_db_connection().await;

    // Then match on the token type and call the correct function
    let (preimage_value, preimage_fee_option) = match token_type {
        TokenType::ERC20 => {
            deposit_operation::<IERC20::IERC20Calls, Nightfall::NightfallCalls>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                id,
            )
            .await
        }
        TokenType::ERC721 => {
            deposit_operation::<IERC721::IERC721Calls, Nightfall::NightfallCalls>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                id,
            )
            .await
        }
        TokenType::ERC1155 => {
            deposit_operation::<IERC1155::IERC1155Calls, Nightfall::NightfallCalls>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                id,
            )
            .await
        }
        TokenType::ERC3525 => {
            deposit_operation::<IERC3525::IERC3525Calls, Nightfall::NightfallCalls>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                id,
            )
            .await
        }
        TokenType::FeeToken => Err(DepositError::TokenError(
            TokenContractError::TokenTypeError("FeeToken is not supported for deposit".to_string()),
        )),
    }
    .map_err(TransactionHandlerError::DepositError)?;

    // Insert the preimage into the commitments DB as pending creation
    // TODO remove the blocknumber
    let ZKPKeys { nullifier_key, .. } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");
    let nullifier = preimage_value
        .nullifier_hash(&nullifier_key)
        .expect("Could not hash commitment {}");
    let commitment_hash = preimage_value.hash().expect("Could not hash commitment");
    let mut commitment_entry = CommitmentEntry::new(
        preimage_value,
        nullifier,
        CommitmentStatus::PendingCreation,
        token_type,
        None,
        None,
    );

    match (
        N::get_token_info(preimage_value.nf_token_id).await,
        N::get_slot_info(preimage_value.nf_slot_id).await,
    ) {
        (Ok(token_info), Ok(slot_info)) => {
            commitment_entry.native_token_id = Some(token_info.token_id.to_hex_string());
            commitment_entry.native_slot_id = Some(slot_info.slot_id.to_hex_string());
        }
        (token_result, slot_result) => {
            warn!(
                "{id} Could not enrich commitment with native token/slot metadata. token_info_ok={}, slot_info_ok={}",
                token_result.is_ok(),
                slot_result.is_ok()
            );
        }
    }

    db.store_commitment(commitment_entry)
        .await
        .ok_or(TransactionHandlerError::DatabaseError)?;

    debug!("{id} Deposit commitment stored successfully");

    // Add the mapping between request and commitment
    let commitment_hex = commitment_hash.to_hex_string();
    match db.add_mapping(id, &commitment_hex).await {
        Ok(_) => debug!("{id} Mapped commitment to request"),
        Err(e) => error!("{id} Failed to  map commitment to request: {e}"),
    }

    // Check if preimage_fee_option is Some, and store it in the DB if it exists
    if let Some(preimage_fee) = preimage_fee_option {
        let nullifier = preimage_fee
            .nullifier_hash(&nullifier_key)
            .expect("Could not hash commitment");
        let commitment_hash = preimage_fee.hash().expect("Could not hash commitment");

        // Add the mapping for fee commitment as well
        let commitment_hex = commitment_hash.to_hex_string();
        match db.add_mapping(id, &commitment_hex).await {
            Ok(_) => debug!("{id} Mapped deposit fee commitment to request"),
            Err(e) => error!("{id} Failed to  map deposit fee commitment to request: {e}"),
        }

        let mut commitment_entry = CommitmentEntry::new(
            preimage_fee,
            nullifier,
            CommitmentStatus::PendingCreation,
            TokenType::FeeToken,
            None,
            None,
        );

        match (
            N::get_token_info(preimage_fee.nf_token_id).await,
            N::get_slot_info(preimage_fee.nf_slot_id).await,
        ) {
            (Ok(token_info), Ok(slot_info)) => {
                commitment_entry.native_token_id = Some(token_info.token_id.to_hex_string());
                commitment_entry.native_slot_id = Some(slot_info.slot_id.to_hex_string());
            }
            (token_result, slot_result) => {
                warn!(
                    "{id} Could not enrich fee commitment with native token/slot metadata. token_info_ok={}, slot_info_ok={}",
                    token_result.is_ok(),
                    slot_result.is_ok()
                );
            }
        }
        // Store the fee commitment in the database, error if storage fails
        db.store_commitment(commitment_entry)
            .await
            .ok_or(TransactionHandlerError::DatabaseError)?;
    }

    debug!("{id} Deposit fee commitment stored successfully");

    let response_data = match preimage_fee_option {
        Some(preimage_fee) => vec![
            preimage_value
                .hash()
                .expect("Preimage must be hashable - this should not happen")
                .to_hex_string(),
            preimage_fee
                .hash()
                .expect("Preimage must be hashable - this should not happen")
                .to_hex_string(),
        ],
        None => vec![preimage_value
            .hash()
            .expect("Preimage must be hashable - this should not happen")
            .to_hex_string()],
    };
    debug!("{id} Deposit request completed successfully - returning reply to caller");

    let response = serde_json::to_string(&response_data).map_err(|e| {
        error!("{id} Error when serialising response: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;
    let uuid = serde_json::to_string(&id).map_err(|e| {
        error!("{id} Error when serialising request ID: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;

    Ok(NotificationPayload::TransactionEvent { response, uuid })
}

async fn rollback_commitments<DB>(db: &DB, commitment_ids: &[Fr254], id: &str)
where
    DB: CommitmentDB<Fr254, CommitmentEntry>,
{
    info!(
        "{id} Rolling back {} spend commitments",
        commitment_ids.len()
    );

    for commitment_id in commitment_ids {
        if let Some(existing) = db.get_commitment(commitment_id).await {
            let _ = db
                .mark_commitments_unspent(
                    &[*commitment_id],
                    existing.layer_1_transaction_hash,
                    existing.layer_2_block_number,
                )
                .await;
        } else {
            warn!(
                "{id} Could not rollback value commitment {}: commitment not found",
                commitment_id.to_hex_string()
            );
        }
    }
}

fn parse_bounded_swap_field(
    id: &str,
    field_name: &str,
    hex_value: &str,
    max_bits: u64,
) -> Result<Fr254, TransactionHandlerError> {
    let normalized_hex = hex_value.strip_prefix("0x").unwrap_or(hex_value);
    let decoded_bytes = hex::decode(normalized_hex).map_err(|e| {
        error!("{id} Error when reading {field_name}: {e}");
        TransactionHandlerError::CustomError(format!("{field_name} must be a valid hex string"))
    })?;

    let raw_value = BigUint::from_bytes_be(&decoded_bytes);
    if raw_value.bits() > max_bits {
        error!(
            "{id} Invalid swap request: {field_name} exceeds {max_bits} bits before field conversion"
        );
        return Err(TransactionHandlerError::CustomError(format!(
            "{field_name} must fit in {max_bits} bits"
        )));
    }

    let field_modulus = BigUint::from_bytes_be(&Fr254::MODULUS.to_bytes_be());
    if raw_value >= field_modulus {
        error!("{id} Invalid swap request: {field_name} exceeds the BN254 field modulus");
        return Err(TransactionHandlerError::CustomError(format!(
            "{field_name} must be less than the BN254 field modulus"
        )));
    }

    Ok(Fr254::from_be_bytes_mod_order(&decoded_bytes))
}

fn parse_supported_swap_token_type(
    party: &str,
    token_type_hex: &str,
    id: &str,
) -> Result<TokenType, TransactionHandlerError> {
    let token_type_value = u8::from_str_radix(token_type_hex.trim_start_matches("0x"), 16)
        .map_err(|e| {
            error!("{id} Error when reading {party} token_type: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;

    match token_type_value {
        0 => Ok(TokenType::ERC20),
        1 => Ok(TokenType::ERC1155),
        2 => Ok(TokenType::ERC721),
        3 => Ok(TokenType::ERC3525),
        _ => {
            error!("{id} Invalid swap request: unsupported {party} token_type {token_type_hex}");
            Err(TransactionHandlerError::CustomError(format!(
                "{party}.token_type must be one of 0x00, 0x01, 0x02, or 0x03"
            )))
        }
    }
}

async fn store_swap_child_request_args(
    id: &str,
    deadline: Fr254,
    swap_link: Fr254,
    spend_commitment_ids: &[Fr254],
) -> Result<(), TransactionHandlerError> {
    let child_args = SwapChildRequestArgs {
        deadline: Some(deadline.to_hex_string()),
        swap_link: Some(swap_link.to_hex_string()),
        spend_commitment_ids: spend_commitment_ids
            .iter()
            .map(|commitment_id| commitment_id.to_hex_string())
            .collect(),
        cancel_requested_at: None,
        cancel_request_status: None,
        cancel_request_message: None,
    };

    let child_args_json = serde_json::to_string(&child_args).map_err(|e| {
        error!("{id} Failed to serialize swap child_request_args: {e}");
        TransactionHandlerError::CustomError("failed to persist swap request metadata".to_string())
    })?;

    let db = get_db_connection().await;
    if RequestDB::update_request_child_args(db, id, &child_args_json)
        .await
        .is_none()
    {
        error!("{id} Failed to store swap child_request_args in database");
        return Err(TransactionHandlerError::CustomError(
            "failed to persist swap request metadata".to_string(),
        ));
    }

    Ok(())
}

async fn rollback_swap_request(
    db: &mongodb::Client,
    spend_commitment_ids: &[Fr254],
    new_commitment_ids: &[Fr254],
    id: &str,
) {
    rollback_commitments(db, spend_commitment_ids, id).await;

    if !new_commitment_ids.is_empty() {
        info!("{id} Deleting {} new commitments", new_commitment_ids.len());
        let _ = db.delete_commitments(new_commitment_ids.to_vec()).await;
    }

    // Failure rollback returns the request to its pre-swap state, so we intentionally clear
    // transient swap metadata here. This differs from settle-expired, which preserves
    // cancel-request UX metadata after a successful local reconciliation.
    let _ = RequestDB::clear_request_child_args(db, id).await;
    let existing_request = RequestDB::get_request(db, id).await;
    if should_overwrite_request_status_with_failed(existing_request.as_ref()) {
        let _ = db.update_request(id, RequestStatus::Failed).await;
    }
}

async fn handle_transfer<P, E, N>(
    transfer_req: NF3TransferRequest,
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    debug!("Handling transfer request: {transfer_req:?}");
    let NF3TransferRequest {
        erc_address,
        token_id,
        token_type,
        recipient_data,
        fee,
        ..
    } = transfer_req;

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), token_id.as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {e}"
            );
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    let keys = get_zkp_keys().lock().expect("Poisoned Mutex lock").clone();

    let first_value = recipient_data.values.first().ok_or_else(|| {
        error!("{id} No recipient value provided");
        TransactionHandlerError::CustomError("missing recipient value".into())
    })?;

    let value = Fr254::from_hex_string(first_value.as_str()).map_err(|e| {
        error!("{id} Error when reading value: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let token_id_bigint = BigInteger256::from_hex_string(token_id.as_str()).map_err(|e| {
        error!("{id} Error when reading token id: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let parsed_token_type = parse_token_type(token_type.as_str()).map_err(|e| {
        error!("{id} Error when reading token type: {e}");
        TransactionHandlerError::CustomError(e)
    })?;

    validate_asset_constraints(parsed_token_type, value, token_id_bigint).map_err(|e| {
        error!("{id} Transfer asset constraint validation failed: {e}");
        TransactionHandlerError::CustomError(e)
    })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|e| {
        error!("{id} Error when reading fee: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let first_key = recipient_data
        .recipient_compressed_zkp_public_keys
        .first()
        .ok_or_else(|| {
            error!("{id} No recipient public key provided");
            TransactionHandlerError::CustomError("missing recipient public key".into())
        })?;

    // Create a JSON string that represents the tuple struct content
    let json_wrapped = format!("\"{first_key}\"");

    // Note: ark_de_hex deserialization additionally ensures the point is on-curve and in correct subgroup
    // Unit tests verify this validation behavior remains consistent
    let deserialized_public_key: JubJubPubKey =
        serde_json::from_str(&json_wrapped).map_err(|e| {
            error!("{id} Could not deserialize recipient public key: {e}");
            TransactionHandlerError::CustomError(format!(
                "Could not deserialize recipient public key: {e}"
            ))
        })?;

    let recipient_public_key = deserialized_public_key.0;

    // Check that the recipient public key is not the identity point
    if recipient_public_key.is_zero() {
        error!("{id} Recipient public key cannot be the identity point");
        return Err(TransactionHandlerError::CustomError(
            "Recipient public key cannot be the identity point".to_string(),
        ));
    }

    let ephemeral_private_key = {
        let mut rng = ark_std::rand::thread_rng(); // TODO initialise in main and pass around as a rwlock
        BJJScalar::rand(&mut rng)
    };
    let shared_secret: Affine<BabyJubjub> = (recipient_public_key * ephemeral_private_key).into();

    // add the id to the request database

    // Select the commitments to be spent.
    let spend_commitments;
    {
        let db = get_db_connection().await;
        let fee_token_id = get_fee_token_id();
        let spend_value_commitments = find_usable_commitments(nf_token_id, value,db)
        .await.map_err(|e|{
            error!("{id} Could not find enough usable value commitments to complete this transfer, suggest depositing more tokens: {e}"); 
            TransactionHandlerError::CustomError(e.to_string())})?;
        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            match find_usable_commitments(fee_token_id, fee, db).await {
                Ok(commitments) => commitments,
                Err(e) => {
                    debug!("{id} Could not find enough usable fee commitments, suggest depositing more fee: {e}");
                    // rollback the value commitments to unspent if fails to find fee commitments
                    let value_commitment_ids = spend_value_commitments
                        .iter()
                        .filter_map(|c| c.hash().ok())
                        .collect::<Vec<_>>();
                    rollback_commitments(db, &value_commitment_ids, id).await;
                    let _ = db.update_request(id, RequestStatus::Failed).await;
                    return Err(TransactionHandlerError::CustomError(e.to_string()));
                }
            }
        };
        spend_commitments = [
            spend_value_commitments[0],
            spend_value_commitments[1],
            spend_fee_commitments[0],
            spend_fee_commitments[1],
        ];
    }

    // Work out how much change is needed.
    let total_token_value = spend_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();

    let token_change = total_token_value - value;
    let total_fee_value = spend_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let fee_change = total_fee_value - fee;

    let poseidon = Poseidon::<Fr254>::new();
    // Derive a shared salt from the shared secret using domain-separated Poseidon hash.
    let shared_salt_hash = poseidon
        .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
        .map_err(|e| {
            error!("{id} Failed to derive shared salt with Poseidon: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    let shared_salt = Salt::Transfer(shared_salt_hash);

    // transferred value commitment, salt is derived from the shared secret
    let new_commitment_one = Preimage::new(
        value,
        nf_token_id,
        spend_commitments[0].get_nf_slot_id(),
        recipient_public_key,
        shared_salt,
    );

    let new_commitment_two = if !token_change.is_zero() {
        Preimage::new(
            token_change,
            nf_token_id,
            spend_commitments[0].get_nf_slot_id(),
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let nightfall_address = FrBn254::from(get_addresses().nightfall()).0;
    let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nightfall_address);

    let fee_token_id = get_fee_token_id();
    // if fee is zero, then no fee commitment is needed
    let new_commitment_three = if !fee.is_zero() {
        Preimage::new(
            fee,
            fee_token_id,
            fee_token_id,
            contract_nf_address,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let new_commitment_four = if !fee_change.is_zero() {
        Preimage::new(
            fee_change,
            fee_token_id,
            fee_token_id,
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let new_commitments = [
        new_commitment_one,
        new_commitment_two,
        new_commitment_three,
        new_commitment_four,
    ];

    let new_commitment_hashes = new_commitments
        .iter()
        .filter_map(|c| c.hash().ok().map(|h| h.to_hex_string()))
        .collect::<Vec<_>>();
    debug!("{id} New commitments prepared: {new_commitment_hashes:?}");

    let secret_preimages = [
        spend_commitments[0].get_secret_preimage(),
        spend_commitments[1].get_secret_preimage(),
        spend_commitments[2].get_secret_preimage(),
        spend_commitments[3].get_secret_preimage(),
    ];
    let op = Operation {
        transport: Transport::OffChain,
        operation_type: OperationType::Transfer,
    };
    match handle_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        ephemeral_private_key,
        Fr254::zero(),
        secret_preimages,
        None,
        id,
    )
    .await
    {
        Ok(res) => Ok(res),
        Err(e) => {
            //  rollback to UNSPENT status if handle_client_operation fails
            let db = get_db_connection().await;

            // Rollback the spend commitments to unspent
            let commitment_ids = spend_commitments
                .iter()
                .filter_map(|c| c.hash().ok())
                .collect::<Vec<_>>();
            rollback_commitments(db, &commitment_ids, id).await;
            // Delete new commitments
            let new_commitment_ids = new_commitments
                .iter()
                .filter_map(|c| c.hash().ok())
                .collect::<Vec<_>>();

            info!("{id} Deleting {} new commitments", new_commitment_ids.len());
            let _ = db.delete_commitments(new_commitment_ids).await;
            let _ = db.update_request(id, RequestStatus::Failed).await;

            Err(TransactionHandlerError::CustomError(e.to_string()))
        }
    }
}

async fn handle_withdraw<P, E, N>(
    withdraw_req: NF3WithdrawRequest,
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let NF3WithdrawRequest {
        erc_address,
        token_id,
        value,
        recipient_address,
        fee,
        ..
    } = withdraw_req;

    // add the id to the request database

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), token_id.as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;

    let keys = get_zkp_keys().lock().expect("Poisoned Mutex lock").clone();

    let value = Fr254::from_hex_string(value.as_str()).map_err(|e| {
        error!("{id} Error when reading value: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|e| {
        error!("{id} Error when reading fee: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let recipient_address: Fr254 =
        Fr254::from_hex_string(recipient_address.as_str()).map_err(|e| {
            error!("{id} Error when reading recipeint address: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    // For now we just use the commitment selection algorithm to minimise change.
    let spend_commitments;
    let db = get_db_connection().await;

    {
        let fee_token_id = get_fee_token_id();
        let spend_value_commitments = find_usable_commitments(nf_token_id, value,db)
        .await.map_err(|e|{
            error!("{id} Could not find enough usable value commitments to complete this withdraw, suggest depositing more tokens: {e}"); 
            TransactionHandlerError::CustomError(e.to_string())})?;
        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            match find_usable_commitments(fee_token_id, fee, db).await {
                Ok(commitments) => commitments,
                Err(e) => {
                    error!("{id} Could not find enough usable fee commitments to complete this withdraw, suggest depositing more fee: {e}");
                    // rollback the value commitments to unspent if fails to find fee commitments
                    let value_commitment_ids = spend_value_commitments
                        .iter()
                        .filter_map(|c| c.hash().ok())
                        .collect::<Vec<_>>();
                    rollback_commitments(db, &value_commitment_ids, id).await;
                    let _ = db.update_request(id, RequestStatus::Failed).await;
                    return Err(TransactionHandlerError::CustomError(e.to_string()));
                }
            }
        };
        spend_commitments = [
            spend_value_commitments[0],
            spend_value_commitments[1],
            spend_fee_commitments[0],
            spend_fee_commitments[1],
        ];
    }
    // Work out how much change is needed.
    let total_token_value = spend_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let token_change = total_token_value - value;

    let total_fee_value = spend_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let fee_change = total_fee_value - fee;

    let nightfall_address = FrBn254::from(get_addresses().nightfall()).0;
    let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nightfall_address);

    // The first commitment of the withdraw is 0, which will be calculated in the circuit
    // here, we set new_commitment_one to have the withdraw value so we can check that value is conserved for transfer and withdraw in client_operation services.
    // We set public_key of this preimage to the contract_nf_address, so that it won't be added in PendingCommitment later (as we only add preimages in PendingCommitment iff commitment.get_public_key() == zkp_public_key).
    let new_commitment_one = Preimage::new(
        value,
        nf_token_id,
        spend_commitments[0].get_nf_slot_id(),
        contract_nf_address,
        Salt::new_transfer_salt(),
    );

    let new_commitment_two = if !token_change.is_zero() {
        Preimage::new(
            token_change,
            nf_token_id,
            spend_commitments[0].get_nf_slot_id(),
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let fee_token_id = get_fee_token_id();

    let new_commitment_three = if !fee.is_zero() {
        Preimage::new(
            fee,
            fee_token_id,
            fee_token_id,
            contract_nf_address,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };
    let new_commitment_four = if !fee_change.is_zero() {
        Preimage::new(
            fee_change,
            fee_token_id,
            fee_token_id,
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let new_commitments = [
        new_commitment_one,
        new_commitment_two,
        new_commitment_three,
        new_commitment_four,
    ];

    let secret_preimages = [
        spend_commitments[0].get_secret_preimage(),
        spend_commitments[1].get_secret_preimage(),
        spend_commitments[2].get_secret_preimage(),
        spend_commitments[3].get_secret_preimage(),
    ];
    let op = Operation {
        transport: Transport::OffChain,
        operation_type: OperationType::Withdraw,
    };
    let withdraw_fund_salt = spend_commitments[0]
        .nullifier_hash(&keys.nullifier_key)
        .expect("Failed to compute nullifier hash");
    match handle_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        BJJScalar::zero(),
        recipient_address,
        secret_preimages,
        None,
        id,
    )
    .await
    {
        Ok(res) => {
            let de_escrow_req = DeEscrowDataReq {
                token_id: token_id.clone(),
                erc_address: erc_address.clone(),
                recipient_address: recipient_address.to_hex_string(),
                value: value.to_hex_string(),
                token_type: withdraw_req.token_type.clone(),
                withdraw_fund_salt: withdraw_fund_salt.to_hex_string(),
            };
            match serde_json::to_string(&de_escrow_req) {
                Ok(child_args_json) => {
                    if RequestDB::update_request_child_args(db, id, &child_args_json)
                        .await
                        .is_none()
                    {
                        error!("{id} Failed to store child_request_args in database");
                    } else {
                        debug!("{id} Successfully stored child_request_args in request collection");
                    }
                }
                Err(e) => {
                    error!("{id} Failed to serialize de_escrow_req: {e}");
                }
            }
            res
        }
        Err(e) => {
            // Rollback to UNSPENT status if handle_client_operation fails
            let db = get_db_connection().await;

            // Rollback spend commitments
            let commitment_ids = spend_commitments
                .iter()
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();
            rollback_commitments(db, &commitment_ids, id).await;

            // Delete new commitments
            let new_commitment_ids = new_commitments
                .iter()
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();

            info!("{id} Deleting {} new commitments", new_commitment_ids.len());
            let _ = db.delete_commitments(new_commitment_ids).await;
            let _ = db.update_request(id, RequestStatus::Failed).await;
            return Err(e);
        }
    };

    // Build the response
    let withdraw_response = WithdrawResponse {
        success: true,
        message: "Withdraw operation completed successfully".to_string(),
        withdraw_fund_salt: withdraw_fund_salt.to_hex_string(),
    };

    let response = serde_json::to_string(&withdraw_response).map_err(|e| {
        error!("{id} Error when serialising response: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;
    let uuid = serde_json::to_string(&id).map_err(|e| {
        error!("{id} Error when serialising request ID: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;

    // Return the response as JSON
    Ok(NotificationPayload::TransactionEvent { response, uuid })
}

async fn handle_swap<P, E, N>(
    swap_req: NF3SwapRequest,
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    debug!("{id} Handling swap request: {swap_req:?}");

    let NF3SwapRequest {
        party_a,
        party_b,
        swap_nonce,
        deadline,
        fee,
    } = swap_req;

    let _token_type_a = parse_supported_swap_token_type("party_a", &party_a.token_type, id)?;
    let _token_type_b = parse_supported_swap_token_type("party_b", &party_b.token_type, id)?;

    // Convert request fields to appropriate types
    let nf_token_a_id =
        to_nf_token_id_from_str(party_a.erc_address.as_str(), party_a.token_id.as_str()).map_err(
            |e| {
                error!("{id} Error when retrieving the Nightfall token id for token A: {e}");
                TransactionHandlerError::CustomError(e.to_string())
            },
        )?;

    let nf_token_b_id =
        to_nf_token_id_from_str(party_b.erc_address.as_str(), party_b.token_id.as_str()).map_err(
            |e| {
                error!("{id} Error when retrieving the Nightfall token id for token B: {e}");
                TransactionHandlerError::CustomError(e.to_string())
            },
        )?;

    let keys = get_zkp_keys().lock().expect("Poisoned Mutex lock").clone();

    let fee = parse_bounded_swap_field(id, "fee", fee.as_str(), 96)?;
    let deadline_fr = parse_bounded_swap_field(id, "deadline", deadline.as_str(), 64)?;

    let json_wrapped = format!("\"{}\"", party_a.public_key);
    let party_a_pk: JubJub = serde_json::from_str::<JubJubPubKey>(&json_wrapped)
        .map_err(|e| {
            error!("{id} Could not deserialize party A public key: {e}");
            TransactionHandlerError::CustomError(format!(
                "Could not deserialize party A public key: {e}"
            ))
        })?
        .0;

    let json_wrapped = format!("\"{}\"", party_b.public_key);
    let party_b_pk: JubJub = serde_json::from_str::<JubJubPubKey>(&json_wrapped)
        .map_err(|e| {
            error!("{id} Could not deserialize party B public key: {e}");
            TransactionHandlerError::CustomError(format!(
                "Could not deserialize party B public key: {e}"
            ))
        })?
        .0;

    // Parse swap values
    let value_a_fr = parse_bounded_swap_field(id, "party_a.value", party_a.value.as_str(), 96)?;
    let value_b_fr = parse_bounded_swap_field(id, "party_b.value", party_b.value.as_str(), 96)?;
    let swap_nonce_fr = parse_bounded_swap_field(id, "swap_nonce", swap_nonce.as_str(), 64)?;

    if swap_nonce_fr.is_zero() {
        error!("{id} Invalid swap request: swap_nonce must be non-zero");
        return Err(TransactionHandlerError::CustomError(
            "swap_nonce must be non-zero".to_string(),
        ));
    }

    if deadline_fr.is_zero() {
        error!("{id} Invalid swap request: deadline must be non-zero");
        return Err(TransactionHandlerError::CustomError(
            "deadline must be non-zero".to_string(),
        ));
    }

    // Determine my role: am I party A or party B?
    let is_party_a = keys.zkp_public_key == party_a_pk;
    let is_party_b = keys.zkp_public_key == party_b_pk;

    if !is_party_a && !is_party_b {
        error!("{id} My public key doesn't match party A or party B");
        return Err(TransactionHandlerError::CustomError(
            "My public key doesn't match party A or party B".to_string(),
        ));
    }

    if is_party_a && is_party_b {
        error!("{id} Party A and party B cannot be the same");
        return Err(TransactionHandlerError::CustomError(
            "Party A and party B cannot be the same".to_string(),
        ));
    }

    // Derive my token/value and counterparty from role
    let (nf_token_id, value) = if is_party_a {
        (nf_token_a_id, value_a_fr)
    } else {
        (nf_token_b_id, value_b_fr)
    };

    let counterparty_pk = if is_party_a { party_b_pk } else { party_a_pk };

    // Generate ephemeral key for encryption
    let ephemeral_private_key = {
        let mut rng = ark_std::rand::thread_rng();
        BJJScalar::rand(&mut rng)
    };

    // Compute shared secret with counterparty
    let shared_secret: Affine<BabyJubjub> = (counterparty_pk * ephemeral_private_key).into();

    // Select commitments to spend
    let spend_commitments;
    {
        let db = get_db_connection().await;
        let fee_token_id = get_fee_token_id();

        let spend_value_commitments = find_usable_commitments(nf_token_id, value, db)
            .await
            .map_err(|e| {
                error!("{id} Could not find enough usable value commitments for swap: {e}");
                TransactionHandlerError::CustomError(e.to_string())
            })?;

        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            match find_usable_commitments(fee_token_id, fee, db).await {
                Ok(commitments) => commitments,
                Err(e) => {
                    debug!("{id} Could not find enough usable fee commitments: {e}");
                    // Rollback value commitments
                    let value_commitment_ids = spend_value_commitments
                        .iter()
                        .filter_map(|c| c.hash().ok())
                        .collect::<Vec<_>>();
                    rollback_commitments(db, &value_commitment_ids, id).await;
                    let _ = db.update_request(id, RequestStatus::Failed).await;
                    return Err(TransactionHandlerError::CustomError(e.to_string()));
                }
            }
        };

        spend_commitments = [
            spend_value_commitments[0],
            spend_value_commitments[1],
            spend_fee_commitments[0],
            spend_fee_commitments[1],
        ];
    }

    // Persist spend commitments for this swap request so the client can later
    // settle-expired and unlock only these locally reserved commitments.
    let spend_commitment_ids = spend_commitments
        .iter()
        .filter(|c| c.get_preimage() != Preimage::default())
        .filter_map(|c| c.hash().ok())
        .collect::<Vec<_>>();
    // Calculate change
    let total_token_value = spend_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let token_change = total_token_value - value;

    let total_fee_value = spend_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let fee_change = total_fee_value - fee;

    // Derive shared salt
    let poseidon = Poseidon::<Fr254>::new();
    let shared_salt_hash = poseidon
        .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
        .map_err(|e| {
            error!("{id} Failed to derive shared salt: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    let shared_salt = Salt::Transfer(shared_salt_hash);

    // Create new commitments
    // Commitment 0: Counterparty receives my tokens (uses shared salt)
    let new_commitment_one = Preimage::new(
        value,
        nf_token_id,
        spend_commitments[0].get_nf_slot_id(),
        counterparty_pk,
        shared_salt,
    );

    // Commitment 1: My change (if any)
    let new_commitment_two = if !token_change.is_zero() {
        Preimage::new(
            token_change,
            nf_token_id,
            spend_commitments[0].get_nf_slot_id(),
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let nightfall_address = FrBn254::from(get_addresses().nightfall()).0;
    let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nightfall_address);

    let fee_token_id = get_fee_token_id();

    // Commitment 2: Fee to contract
    let new_commitment_three = if !fee.is_zero() {
        Preimage::new(
            fee,
            fee_token_id,
            fee_token_id,
            contract_nf_address,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    // Commitment 3: Fee change
    let new_commitment_four = if !fee_change.is_zero() {
        Preimage::new(
            fee_change,
            fee_token_id,
            fee_token_id,
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let new_commitments = [
        new_commitment_one,
        new_commitment_two,
        new_commitment_three,
        new_commitment_four,
    ];

    let secret_preimages = [
        spend_commitments[0].get_secret_preimage(),
        spend_commitments[1].get_secret_preimage(),
        spend_commitments[2].get_secret_preimage(),
        spend_commitments[3].get_secret_preimage(),
    ];

    let op = Operation {
        transport: Transport::OffChain,
        operation_type: OperationType::Swap,
    };

    match submit_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        ephemeral_private_key,
        Fr254::zero(),
        secret_preimages,
        Some(SwapParams {
            party_a_public_key: party_a_pk,
            party_b_public_key: party_b_pk,
            token_a_id: nf_token_a_id,
            value_a: value_a_fr,
            token_b_id: nf_token_b_id,
            value_b: value_b_fr,
            swap_nonce: swap_nonce_fr,
            deadline: deadline_fr,
        }),
        id,
    )
    .await
    {
        Ok(submitted) => {
            let new_commitment_ids = new_commitments
                .iter()
                .filter_map(|c| c.hash().ok())
                .collect::<Vec<_>>();
            if let Err(e) = store_swap_child_request_args(
                id,
                deadline_fr,
                submitted.transaction.swap_link,
                &spend_commitment_ids,
            )
            .await
            {
                let db = get_db_connection().await;
                rollback_swap_request(db, &spend_commitment_ids, &new_commitment_ids, id).await;
                return Err(e);
            }
            Ok(submitted.payload)
        }
        Err(e) => {
            // Rollback on failure
            let db = get_db_connection().await;

            let commitment_ids = spend_commitments
                .iter()
                .filter_map(|c| c.hash().ok())
                .collect::<Vec<_>>();

            let new_commitment_ids = new_commitments
                .iter()
                .filter_map(|c| c.hash().ok())
                .collect::<Vec<_>>();
            rollback_swap_request(db, &commitment_ids, &new_commitment_ids, id).await;

            Err(TransactionHandlerError::CustomError(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::{Request, RequestStatus},
        drivers::rest::client_operation,
    };
    use alloy::primitives::TxHash;
    use ark_ec::AffineRepr;
    use ark_ff::One;
    use ark_serialize::{CanonicalSerialize, Compress};
    use ark_std::Zero;
    use async_trait::async_trait;
    use lib::{
        client_models::{
            NF3CancelSwapRequest, NF3DepositRequest, NF3RecipientData, NF3SettleExpiredSwapRequest,
            NF3SwapRequest, NF3TransferRequest, NF3WithdrawRequest, SwapParty,
        },
        derive_key::ZKPKeys,
        plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
        shared_entities::{Preimage, TokenType},
    };
    use nf_curves::ed_on_bn254::BabyJubjub;
    use nf_curves::ed_on_bn254::Fq;
    use serde_json::{json, Value};
    use tokio::sync::Mutex;

    fn sample_deposit_request() -> NF3DepositRequest {
        NF3DepositRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            value: "0x01".to_string(),
            fee: "0x00".to_string(),
            deposit_fee: "0x00".to_string(),
        }
    }

    fn sample_withdraw_request() -> NF3WithdrawRequest {
        NF3WithdrawRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            value: "0x01".to_string(),
            recipient_address: "0x01".to_string(),
            fee: "0x00".to_string(),
        }
    }

    fn sample_transfer_request() -> NF3TransferRequest {
        let mut compressed_public_key = ZKPKeys::new(Fr254::one())
            .expect("should derive zkp keys")
            .compressed_public_key()
            .expect("should compress zkp public key");
        compressed_public_key.reverse(); // Convert to big-endian to match ark_de_hex format

        NF3TransferRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            recipient_data: NF3RecipientData {
                values: vec!["0x01".to_string()],
                recipient_compressed_zkp_public_keys: vec![format!(
                    "0x{}",
                    hex::encode(compressed_public_key)
                )],
            },
            fee: "0x00".to_string(),
        }
    }

    struct MockQuitSwapStore {
        requests: Mutex<Vec<Request>>,
        commitments: Mutex<Vec<CommitmentEntry>>,
        fail_mark_unspent: bool,
    }

    impl Default for MockQuitSwapStore {
        fn default() -> Self {
            Self {
                requests: Mutex::new(Vec::new()),
                commitments: Mutex::new(Vec::new()),
                fail_mark_unspent: false,
            }
        }
    }

    impl MockQuitSwapStore {
        async fn push_request(&self, request: Request) {
            self.requests.lock().await.push(request);
        }

        async fn push_commitment(&self, commitment: CommitmentEntry) {
            self.commitments.lock().await.push(commitment);
        }

        async fn get_commitment_status(&self, commitment_id: Fr254) -> Option<CommitmentStatus> {
            self.commitments
                .lock()
                .await
                .iter()
                .find(|c| c.key == commitment_id)
                .map(|c| c.status)
        }

        async fn request_child_args(&self, request_id: &str) -> Option<Option<String>> {
            self.requests
                .lock()
                .await
                .iter()
                .find(|r| r.uuid == request_id)
                .map(|r| r.child_request_args.clone())
        }

        async fn request_status(&self, request_id: &str) -> Option<RequestStatus> {
            self.requests
                .lock()
                .await
                .iter()
                .find(|r| r.uuid == request_id)
                .map(|r| r.status)
        }
    }

    #[async_trait]
    impl SwapExpiryStore for MockQuitSwapStore {
        async fn get_request(&self, request_id: &str) -> Option<Request> {
            self.requests
                .lock()
                .await
                .iter()
                .find(|r| r.uuid == request_id)
                .cloned()
        }

        async fn get_commitment(&self, commitment_id: &Fr254) -> Option<CommitmentEntry> {
            self.commitments
                .lock()
                .await
                .iter()
                .find(|c| c.key == *commitment_id)
                .cloned()
        }

        async fn set_request_status(&self, request_id: &str, status: RequestStatus) -> Option<()> {
            let mut requests = self.requests.lock().await;
            let request = requests.iter_mut().find(|r| r.uuid == request_id)?;
            request.status = status;
            Some(())
        }

        async fn update_request_child_args(
            &self,
            request_id: &str,
            child_args: &str,
        ) -> Option<()> {
            let mut requests = self.requests.lock().await;
            let request = requests.iter_mut().find(|r| r.uuid == request_id)?;
            request.child_request_args = Some(child_args.to_string());
            Some(())
        }

        async fn mark_commitments_unspent(
            &self,
            commitments: &[Fr254],
            layer_1_transaction_hash: Option<TxHash>,
            layer_2_block_number: Option<i64>,
        ) -> Option<()> {
            if self.fail_mark_unspent {
                return None;
            }

            let mut entries = self.commitments.lock().await;
            let mut updated = false;
            for commitment_id in commitments {
                if let Some(entry) = entries.iter_mut().find(|c| c.key == *commitment_id) {
                    entry.status = CommitmentStatus::Unspent;
                    entry.layer_1_transaction_hash = layer_1_transaction_hash;
                    entry.layer_2_block_number = layer_2_block_number;
                    updated = true;
                }
            }
            if updated {
                Some(())
            } else {
                None
            }
        }
    }

    #[async_trait]
    impl SwapCancelStore for MockQuitSwapStore {
        async fn get_request(&self, request_id: &str) -> Option<Request> {
            self.requests
                .lock()
                .await
                .iter()
                .find(|r| r.uuid == request_id)
                .cloned()
        }

        async fn update_request_child_args(
            &self,
            request_id: &str,
            child_args: &str,
        ) -> Option<()> {
            let mut requests = self.requests.lock().await;
            let request = requests.iter_mut().find(|r| r.uuid == request_id)?;
            request.child_request_args = Some(child_args.to_string());
            Some(())
        }
    }

    fn mock_commitment(key: Fr254, status: CommitmentStatus) -> CommitmentEntry {
        CommitmentEntry {
            preimage: Preimage::default(),
            status,
            key,
            nullifier: Fr254::zero(),
            token_type: TokenType::ERC20,
            native_token_id: None,
            native_slot_id: None,
            layer_1_transaction_hash: None,
            layer_2_block_number: Some(7),
        }
    }

    fn mock_request(request_id: &str, child_request_args: Option<String>) -> Request {
        Request {
            status: RequestStatus::Submitted,
            uuid: request_id.to_string(),
            child_request_args,
        }
    }

    async fn parsed_request_child_args(
        db: &MockQuitSwapStore,
        request_id: &str,
    ) -> Option<SwapChildRequestArgs> {
        let stored = db.request_child_args(request_id).await.flatten()?;
        serde_json::from_str(&stored).ok()
    }

    #[test]
    fn test_parse_token_type_accepts_hex_with_prefix() {
        let parsed = parse_token_type("0x02").expect("should parse token type");
        assert_eq!(parsed, TokenType::ERC721);
    }

    #[test]
    fn test_parse_token_type_rejects_unsupported_value() {
        let err = parse_token_type("0x09").expect_err("should reject unsupported token type");
        assert_eq!(err, "Unsupported tokenType");
    }

    #[test]
    fn test_validate_deposit_rejects_erc721_non_zero_value() {
        let mut req = sample_deposit_request();
        req.token_type = "02".to_string();
        req.token_id = "0x2a".to_string();
        req.value = "0x01".to_string();
        let err = validate_deposit_request_payload(&req).expect_err("validation should fail");
        assert_eq!(err, "ERC721 operations require value to be 0");
    }

    #[test]
    fn test_validate_deposit_accepts_erc721_zero_token_id_when_value_zero() {
        let mut req = sample_deposit_request();
        req.token_type = "02".to_string();
        req.token_id = "0x00".to_string();
        req.value = "0x00".to_string();
        validate_deposit_request_payload(&req).expect("validation should pass");
    }

    #[test]
    fn test_validate_deposit_rejects_erc20_non_zero_token_id() {
        let mut req = sample_deposit_request();
        req.token_type = "00".to_string();
        req.token_id = "0x2a".to_string();
        let err = validate_deposit_request_payload(&req).expect_err("validation should fail");
        assert_eq!(err, "ERC20 operations require tokenId to be 0");
    }

    #[test]
    fn test_validate_deposit_accepts_non_fungible_erc1155_zero_value() {
        let mut req = sample_deposit_request();
        req.token_type = "01".to_string();
        req.value = "0x00".to_string();
        req.token_id = "0x2a".to_string();
        validate_deposit_request_payload(&req).expect("validation should pass");
    }

    #[test]
    fn test_validate_deposit_rejects_erc1155_when_value_and_token_id_are_zero() {
        let mut req = sample_deposit_request();
        req.token_type = "01".to_string();
        req.value = "0x00".to_string();
        req.token_id = "0x00".to_string();
        let err = validate_deposit_request_payload(&req).expect_err("validation should fail");
        assert_eq!(
            err,
            "ERC1155 operations require either value > 0 or tokenId > 0"
        );
    }

    #[test]
    fn test_validate_deposit_rejects_erc3525_zero_value() {
        let mut req = sample_deposit_request();
        req.token_type = "03".to_string();
        req.value = "0x00".to_string();
        let err = validate_deposit_request_payload(&req).expect_err("validation should fail");
        assert_eq!(err, "ERC3525 operations require value > 0");
    }

    #[test]
    fn test_validate_withdraw_rejects_erc721_non_zero_value() {
        let mut req = sample_withdraw_request();
        req.token_type = "02".to_string();
        req.value = "0x01".to_string();
        let err = validate_withdraw_request_payload(&req).expect_err("validation should fail");
        assert_eq!(err, "ERC721 operations require value to be 0");
    }

    #[test]
    fn test_validate_withdraw_rejects_zero_recipient() {
        let mut req = sample_withdraw_request();
        req.recipient_address = "0x00".to_string();
        let err = validate_withdraw_request_payload(&req).expect_err("validation should fail");
        assert_eq!(
            err,
            "Withdraw operations require a non-zero recipientAddress"
        );
    }

    #[test]
    fn test_validate_withdraw_accepts_non_fungible_erc1155_zero_value() {
        let mut req = sample_withdraw_request();
        req.token_type = "01".to_string();
        req.value = "0x00".to_string();
        req.token_id = "0x2a".to_string();
        validate_withdraw_request_payload(&req).expect("validation should pass");
    }

    #[test]
    fn test_validate_withdraw_rejects_erc1155_when_value_and_token_id_are_zero() {
        let mut req = sample_withdraw_request();
        req.token_type = "01".to_string();
        req.value = "0x00".to_string();
        req.token_id = "0x00".to_string();
        let err = validate_withdraw_request_payload(&req).expect_err("validation should fail");
        assert_eq!(
            err,
            "ERC1155 operations require either value > 0 or tokenId > 0"
        );
    }

    #[test]
    fn test_validate_transfer_rejects_empty_values() {
        let mut req = sample_transfer_request();
        req.recipient_data.values = vec![];
        let err = validate_transfer_request_payload(&req).expect_err("validation should fail");
        assert_eq!(
            err,
            "Transfer currently supports exactly one recipient value"
        );
    }

    #[test]
    fn test_validate_transfer_rejects_zero_value() {
        let mut req = sample_transfer_request();
        req.recipient_data.values = vec!["0x00".to_string()];
        let err = validate_transfer_request_payload(&req).expect_err("validation should fail");
        assert_eq!(err, "ERC20 operations require value > 0");
    }

    #[test]
    fn test_validate_transfer_accepts_non_fungible_erc1155_zero_value() {
        let mut req = sample_transfer_request();
        req.token_type = "01".to_string();
        req.token_id = "0x2a".to_string();
        req.recipient_data.values = vec!["0x00".to_string()];
        validate_transfer_request_payload(&req).expect("validation should pass");
    }

    #[test]
    fn test_validate_transfer_rejects_erc1155_when_value_and_token_id_are_zero() {
        let mut req = sample_transfer_request();
        req.token_type = "01".to_string();
        req.token_id = "0x00".to_string();
        req.recipient_data.values = vec!["0x00".to_string()];
        let err = validate_transfer_request_payload(&req).expect_err("validation should fail");
        assert_eq!(
            err,
            "ERC1155 operations require either value > 0 or tokenId > 0"
        );
    }

    #[tokio::test]
    async fn test_queue_full_response_preserves_legacy_json_string_shape() {
        let filter = warp::any().map(|| queue_full_response("req-123".to_string()));
        let res = warp::test::request().reply(&filter).await;

        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(res.headers()["x-request-id"], "req-123");

        let body = serde_json::from_slice::<String>(res.body()).expect("body should be JSON");
        assert_eq!(body, "Queue is full");
    }

    #[tokio::test]
    async fn test_queue_accepted_response_preserves_legacy_json_string_shape() {
        let filter = warp::any().map(|| queue_accepted_response("req-456".to_string()));
        let res = warp::test::request().reply(&filter).await;

        assert_eq!(res.status(), StatusCode::ACCEPTED);
        assert_eq!(res.headers()["x-request-id"], "req-456");

        let body = serde_json::from_slice::<String>(res.body()).expect("body should be JSON");
        assert_eq!(body, "Request queued");
    }

    #[tokio::test]
    async fn test_deposit_route_rejects_invalid_payload_with_bad_request() {
        let mut req = sample_deposit_request();
        req.token_id = "0x2a".to_string();

        let filter = deposit_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/deposit")
            .json(&req)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert_eq!(
            body,
            json!({ "error": "ERC20 operations require tokenId to be 0" })
        );
    }

    #[tokio::test]
    async fn test_transfer_route_rejects_invalid_payload_with_bad_request() {
        let mut req = sample_transfer_request();
        req.recipient_data.values = vec![];

        let filter = transfer_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/transfer")
            .json(&req)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert_eq!(
            body,
            json!({ "error": "Transfer currently supports exactly one recipient value" })
        );
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_invalid_payload_with_bad_request() {
        let mut req = sample_withdraw_request();
        req.recipient_address = "0x00".to_string();

        let filter = withdraw_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .json(&req)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert_eq!(
            body,
            json!({ "error": "Withdraw operations require a non-zero recipientAddress" })
        );
    }

    #[tokio::test]
    async fn test_deposit_route_rejects_malformed_hex_with_bad_request() {
        let mut req = sample_deposit_request();
        req.value = "not-hex".to_string();

        let filter = deposit_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/deposit")
            .json(&req)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert_eq!(
            body,
            json!({ "error": "Invalid value: Invalid hex format" })
        );
    }

    #[tokio::test]
    async fn test_transfer_route_rejects_missing_required_fields() {
        let filter = transfer_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/transfer")
            .body(
                r#"{
                    "ercAddress":"0x1234567890123456789012345678901234567890",
                    "tokenId":"0x00",
                    "tokenType":"00",
                    "fee":"0x00"
                }"#,
            )
            .header("content-type", "application/json")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_missing_required_fields() {
        let filter = withdraw_request::<PlonkProof>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .body(
                r#"{
                    "ercAddress":"0x1234567890123456789012345678901234567890",
                    "tokenId":"0x00",
                    "tokenType":"00",
                    "value":"0x01"
                }"#,
            )
            .header("content-type", "application/json")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_parse_bounded_swap_field_rejects_field_modulus_plus_one() {
        let field_modulus = BigUint::from_bytes_be(&Fr254::MODULUS.to_bytes_be());
        let oversized_hex = format!(
            "0x{}",
            (field_modulus + BigUint::from(1u8)).to_str_radix(16)
        );

        let result = parse_bounded_swap_field("test-id", "party_a.value", &oversized_hex, 256);

        match result {
            Err(TransactionHandlerError::CustomError(msg)) => {
                assert!(msg.contains("BN254 field modulus"), "got: {msg}");
            }
            other => panic!("Expected field modulus rejection, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_bounded_swap_field_accepts_exact_96_bit_limit() {
        let result =
            parse_bounded_swap_field("test-id", "party_a.value", "0xFFFFFFFFFFFFFFFFFFFFFFFF", 96);

        assert!(result.is_ok(), "expected exact 96-bit limit to be accepted");
    }

    #[test]
    fn test_parse_bounded_swap_field_rejects_97_bit_value() {
        let result = parse_bounded_swap_field(
            "test-id",
            "party_a.value",
            "0x01FFFFFFFFFFFFFFFFFFFFFFFF",
            96,
        );

        match result {
            Err(TransactionHandlerError::CustomError(msg)) => {
                assert!(
                    msg.contains("party_a.value must fit in 96 bits"),
                    "got: {msg}"
                );
            }
            other => panic!("Expected 97-bit rejection, got {other:?}"),
        }
    }

    /// Tests that transfer API rejects invalid recipient public keys
    #[tokio::test]
    async fn test_transfer_api_rejects_invalid_recipient_keys() {
        // Invalid compressed point (not on the curve) should fail early in handle_transfer
        let invalid_transfer_req = NF3TransferRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            recipient_data: NF3RecipientData {
                values: vec!["0x04".to_string()],
                recipient_compressed_zkp_public_keys: vec![
                    "0x000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000"
                        .to_string(),
                ],
            },
            fee: "0x00".to_string(),
        };

        let result = handle_transfer::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            invalid_transfer_req,
            "test-id-1",
        )
        .await;

        // This should fail at the recipient key validation stage, demonstrating the API validates keys
        assert!(
            result.is_err(),
            "Transfer API should reject invalid recipient public key"
        );
        if let Err(TransactionHandlerError::CustomError(msg)) = result {
            assert!(
                msg.contains("Could not deserialize recipient public key: the input buffer contained invalid data"),
                "Error should indicate recipient public key deserialization failure, got: {msg}"
            );
        } else {
            panic!("Expected TransactionHandlerError::CustomError with recipient public key deserialization failure");
        }
    }

    #[tokio::test]
    async fn test_transfer_api_rejects_identity_recipient_keys() {
        // Identity point should fail early in handle_transfer
        let identity_point = Affine::<BabyJubjub>::zero();
        let mut compressed_bytes = Vec::new();
        identity_point
            .serialize_with_mode(&mut compressed_bytes, Compress::Yes)
            .unwrap();
        compressed_bytes.reverse(); // Convert to big-endian to match ark_se_hex format
        let identity_point_hex = format!("0x{}", hex::encode(compressed_bytes));

        let identity_point_transfer_req = NF3TransferRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            recipient_data: NF3RecipientData {
                values: vec!["0x04".to_string()],
                recipient_compressed_zkp_public_keys: vec![identity_point_hex],
            },
            fee: "0x00".to_string(),
        };

        let result = handle_transfer::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            identity_point_transfer_req,
            "test-id-2",
        )
        .await;

        assert!(
            result.is_err(),
            "Transfer API should reject recipient public key if it is the identity"
        );
        if let Err(TransactionHandlerError::CustomError(msg)) = result {
            assert!(
                msg.contains("Recipient public key cannot be the identity point"),
                "Error should indicate recipient public key cannot be the identity point, got: {msg}"
            );
        } else {
            panic!("Expected TransactionHandlerError::CustomError with recipient public key cannot be the identity point");
        }
    }

    #[tokio::test]
    async fn test_transfer_api_rejects_low_order_recipient_keys() {
        // A point that is low order but on the curve should fail early in handle_transfer
        // We use point (0, -1) which is order 2 on BabyJubJub
        let zero_x = Fq::zero();
        let neg_one_y = -Fq::one();

        let low_order_point = Affine::<BabyJubjub>::new_unchecked(zero_x, neg_one_y);

        let mut compressed_bytes = Vec::new();
        low_order_point
            .serialize_with_mode(&mut compressed_bytes, Compress::Yes)
            .unwrap();
        compressed_bytes.reverse(); // Convert to big-endian to match ark_se_hex format
        let low_order_hex = format!("0x{}", hex::encode(compressed_bytes));

        let low_order_transfer_req = NF3TransferRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
            token_type: "00".to_string(),
            recipient_data: NF3RecipientData {
                values: vec!["0x04".to_string()],
                recipient_compressed_zkp_public_keys: vec![low_order_hex],
            },
            fee: "0x00".to_string(),
        };

        let result = handle_transfer::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            low_order_transfer_req,
            "test-id-3",
        )
        .await;

        // This should fail at the explicit .check() stage since zero point is low-order
        assert!(
            result.is_err(),
            "Transfer API should reject low-order recipient public key"
        );
        if let Err(TransactionHandlerError::CustomError(msg)) = result {
            assert!(
                msg.contains("Could not deserialize recipient public key: the input buffer contained invalid data"),
                "Error should indicate recipient public key deserialization failure, got: {msg}"
            );
        } else {
            panic!("Expected TransactionHandlerError::CustomError with recipient public key deserialization failure");
        }
    }

    /// Test that handle_swap rejects invalid counterparty public key
    #[tokio::test]
    async fn test_swap_api_rejects_invalid_party_keys() {
        let invalid_hex =
            "0x000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000".to_string();
        let invalid_swap_req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: invalid_hex.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: invalid_hex.clone(),
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };
        let result = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            invalid_swap_req,
            "test-swap-invalid-party-keys",
        )
        .await;
        assert!(
            result.is_err(),
            "Swap API should reject invalid party public keys"
        );
        if let Err(TransactionHandlerError::CustomError(msg)) = result {
            assert!(
                msg.contains("Could not deserialize party A public key")
                    || msg.contains("Could not deserialize party B public key"),
                "Expected deserialization failure, got: {msg}"
            );
        } else {
            panic!("Expected TransactionHandlerError::CustomError for party public key");
        }
    }

    /// Test that handle_swap rejects when my_public_key doesn't match party A or B
    #[tokio::test]
    async fn test_swap_api_rejects_mismatched_party_keys() {
        let other_point = Affine::<BabyJubjub>::generator();
        let mut other_bytes = Vec::new();
        other_point
            .serialize_with_mode(&mut other_bytes, Compress::Yes)
            .unwrap();
        other_bytes.reverse();
        let other_point_hex = format!("0x{}", hex::encode(other_bytes));

        let swap_req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: other_point_hex.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: other_point_hex.clone(),
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let result = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            swap_req,
            "test-swap-mismatch",
        )
        .await;
        assert!(
            result.is_err(),
            "Swap API should reject when my_public_key doesn't match party A or B"
        );
        if let Err(TransactionHandlerError::CustomError(msg)) = result {
            assert!(
                msg.contains("My public key doesn't match party A or party B"),
                "Expected key mismatch error, got: {msg}"
            );
        } else {
            panic!("Expected TransactionHandlerError::CustomError for key mismatch");
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_nonce_over_64_bits() {
        let valid_pk = {
            let p = Affine::<BabyJubjub>::generator();
            let mut b = Vec::new();
            p.serialize_with_mode(&mut b, Compress::Yes).unwrap();
            b.reverse();
            format!("0x{}", hex::encode(b))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: valid_pk.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: valid_pk,
            },
            swap_nonce: "0x010000000000000000".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-nonce-over-64",
        )
        .await;

        assert!(res.is_err());
        if let Err(TransactionHandlerError::CustomError(msg)) = res {
            assert!(msg.contains("swap_nonce must fit in 64 bits"), "got: {msg}");
        } else {
            panic!("Expected CustomError");
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_deadline_over_64_bits() {
        let valid_pk = {
            let p = Affine::<BabyJubjub>::generator();
            let mut b = Vec::new();
            p.serialize_with_mode(&mut b, Compress::Yes).unwrap();
            b.reverse();
            format!("0x{}", hex::encode(b))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: valid_pk.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: valid_pk,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x010000000000000000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-deadline-over-64",
        )
        .await;

        assert!(res.is_err());
        if let Err(TransactionHandlerError::CustomError(msg)) = res {
            assert!(msg.contains("deadline must fit in 64 bits"), "got: {msg}");
        } else {
            panic!("Expected CustomError");
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_party_a_value_over_96_bits() {
        let valid_pk = {
            let p = Affine::<BabyJubjub>::generator();
            let mut b = Vec::new();
            p.serialize_with_mode(&mut b, Compress::Yes).unwrap();
            b.reverse();
            format!("0x{}", hex::encode(b))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x01000000000000000000000000".to_string(),
                public_key: valid_pk.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: valid_pk,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-party-a-value-over-96",
        )
        .await;

        assert!(res.is_err());
        if let Err(TransactionHandlerError::CustomError(msg)) = res {
            assert!(
                msg.contains("party_a.value must fit in 96 bits"),
                "got: {msg}"
            );
        } else {
            panic!("Expected CustomError");
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_party_b_value_over_96_bits() {
        let valid_pk = {
            let p = Affine::<BabyJubjub>::generator();
            let mut b = Vec::new();
            p.serialize_with_mode(&mut b, Compress::Yes).unwrap();
            b.reverse();
            format!("0x{}", hex::encode(b))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: valid_pk.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x01000000000000000000000000".to_string(),
                public_key: valid_pk,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-party-b-value-over-96",
        )
        .await;

        assert!(res.is_err());
        if let Err(TransactionHandlerError::CustomError(msg)) = res {
            assert!(
                msg.contains("party_b.value must fit in 96 bits"),
                "got: {msg}"
            );
        } else {
            panic!("Expected CustomError");
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_fee_over_96_bits() {
        let valid_pk = {
            let p = Affine::<BabyJubjub>::generator();
            let mut b = Vec::new();
            p.serialize_with_mode(&mut b, Compress::Yes).unwrap();
            b.reverse();
            format!("0x{}", hex::encode(b))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: valid_pk.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: valid_pk,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x01000000000000000000000000".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-fee-over-96",
        )
        .await;

        assert!(res.is_err());
        if let Err(TransactionHandlerError::CustomError(msg)) = res {
            assert!(msg.contains("fee must fit in 96 bits"), "got: {msg}");
        } else {
            panic!("Expected CustomError");
        }
    }

    #[tokio::test]
    async fn test_swap_api_accepts_all_supported_token_types_for_party_a() {
        let my_public_key_hex = {
            let my_key = crate::get_zkp_keys()
                .lock()
                .expect("Poisoned Mutex lock")
                .zkp_public_key;
            let mut bytes = Vec::new();
            my_key
                .serialize_with_mode(&mut bytes, Compress::Yes)
                .unwrap();
            bytes.reverse();
            format!("0x{}", hex::encode(bytes))
        };

        // Use identical party keys so we always fail at a deterministic swap validation
        // point after token-type parsing, independent of DB/prover state.
        for token_type_a in ["0x00", "0x01", "0x02", "0x03"] {
            let req = NF3SwapRequest {
                party_a: SwapParty {
                    erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                    token_id: "0x00".to_string(),
                    token_type: token_type_a.to_string(),
                    value: "0x04".to_string(),
                    public_key: my_public_key_hex.clone(),
                },
                party_b: SwapParty {
                    erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                    token_id: "0x00".to_string(),
                    token_type: "0x00".to_string(),
                    value: "0x05".to_string(),
                    public_key: my_public_key_hex.clone(),
                },
                swap_nonce: "0x01".to_string(),
                deadline: "0x1000".to_string(),
                fee: "0x00".to_string(),
            };

            let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
                req,
                "test-swap-token-types-party-a",
            )
            .await;

            assert!(
                res.is_err(),
                "expected validation error for token_type_a={token_type_a}"
            );
            if let Err(TransactionHandlerError::CustomError(msg)) = res {
                assert!(
                    msg.contains("Party A and party B cannot be the same"),
                    "unexpected error for token_type_a={token_type_a}: {msg}"
                );
            } else {
                panic!("Expected TransactionHandlerError::CustomError");
            }
        }
    }

    #[tokio::test]
    async fn test_swap_api_accepts_all_supported_token_types_for_party_b() {
        let my_public_key_hex = {
            let my_key = crate::get_zkp_keys()
                .lock()
                .expect("Poisoned Mutex lock")
                .zkp_public_key;
            let mut bytes = Vec::new();
            my_key
                .serialize_with_mode(&mut bytes, Compress::Yes)
                .unwrap();
            bytes.reverse();
            format!("0x{}", hex::encode(bytes))
        };

        for token_type_b in ["0x00", "0x01", "0x02", "0x03"] {
            let req = NF3SwapRequest {
                party_a: SwapParty {
                    erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                    token_id: "0x00".to_string(),
                    token_type: "0x00".to_string(),
                    value: "0x04".to_string(),
                    public_key: my_public_key_hex.clone(),
                },
                party_b: SwapParty {
                    erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                    token_id: "0x00".to_string(),
                    token_type: token_type_b.to_string(),
                    value: "0x05".to_string(),
                    public_key: my_public_key_hex.clone(),
                },
                swap_nonce: "0x01".to_string(),
                deadline: "0x1000".to_string(),
                fee: "0x00".to_string(),
            };

            let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
                req,
                "test-swap-token-types-party-b",
            )
            .await;

            assert!(
                res.is_err(),
                "expected validation error for token_type_b={token_type_b}"
            );
            if let Err(TransactionHandlerError::CustomError(msg)) = res {
                assert!(
                    msg.contains("Party A and party B cannot be the same"),
                    "unexpected error for token_type_b={token_type_b}: {msg}"
                );
            } else {
                panic!("Expected TransactionHandlerError::CustomError");
            }
        }
    }

    #[tokio::test]
    async fn test_swap_api_rejects_unsupported_token_type_for_party_a() {
        let my_public_key_hex = {
            let my_key = crate::get_zkp_keys()
                .lock()
                .expect("Poisoned Mutex lock")
                .zkp_public_key;
            let mut bytes = Vec::new();
            my_key
                .serialize_with_mode(&mut bytes, Compress::Yes)
                .unwrap();
            bytes.reverse();
            format!("0x{}", hex::encode(bytes))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x09".to_string(),
                value: "0x04".to_string(),
                public_key: my_public_key_hex.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x05".to_string(),
                public_key: my_public_key_hex,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-invalid-token-type-party-a",
        )
        .await;

        match res {
            Err(TransactionHandlerError::CustomError(msg)) => {
                assert!(msg.contains("party_a.token_type"));
            }
            other => panic!("Expected unsupported token_type error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_settle_expired_request_rejects_invalid_uuid() {
        let result = handle_settle_expired_swap_request(NF3SettleExpiredSwapRequest {
            request_id: "not-a-uuid".to_string(),
        })
        .await;

        match result {
            Ok(_) => panic!("invalid UUID should be rejected"),
            Err(rejection) => {
                assert!(matches!(
                    rejection.find::<crate::domain::error::ClientRejection>(),
                    Some(crate::domain::error::ClientRejection::InvalidRequestId)
                ));
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_swap_request_rejects_invalid_uuid() {
        let result = handle_cancel_swap_request(NF3CancelSwapRequest {
            request_id: "not-a-uuid".to_string(),
        })
        .await;

        match result {
            Ok(_) => panic!("invalid UUID should be rejected"),
            Err(rejection) => {
                assert!(matches!(
                    rejection.find::<crate::domain::error::ClientRejection>(),
                    Some(crate::domain::error::ClientRejection::InvalidRequestId)
                ));
            }
        }
    }

    #[tokio::test]
    async fn test_process_cancel_swap_request_records_advisory_metadata() {
        let db = MockQuitSwapStore::default();
        let request_id = "21212121-2121-2121-2121-212121212121";
        let swap_link = Fr254::from(701u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x20".to_string()),
            swap_link: Some(swap_link.to_hex_string()),
            spend_commitment_ids: vec![Fr254::from(77u64).to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");
        db.push_request(mock_request(request_id, Some(child_args)))
            .await;

        let result = process_cancel_swap_request(
            &db,
            request_id,
            Some(I256::try_from(16u64).unwrap()),
            |actual_swap_link, _| async move {
                assert_eq!(actual_swap_link, swap_link);
                Ok(client_operation::AggregatedSwapCancelResponse {
                    status: "accepted".to_string(),
                    message: "advisory cancel accepted".to_string(),
                    matched: 2,
                })
            },
        )
        .await
        .expect("cancel-request should succeed");

        assert_eq!(
            result,
            SwapCancelExecution {
                status_code: StatusCode::OK,
                cancel_request_status: "accepted".to_string(),
                matched: 2,
                message: "advisory cancel accepted".to_string(),
            }
        );

        let stored = db
            .request_child_args(request_id)
            .await
            .flatten()
            .expect("child args should still be present");
        let parsed: SwapChildRequestArgs =
            serde_json::from_str(&stored).expect("stored child args should deserialize");
        assert_eq!(parsed.cancel_request_status.as_deref(), Some("accepted"));
        assert_eq!(
            parsed.cancel_request_message.as_deref(),
            Some("advisory cancel accepted")
        );
        assert!(parsed.cancel_requested_at.is_some());
    }

    #[tokio::test]
    async fn test_process_cancel_swap_request_refuses_after_deadline() {
        let db = MockQuitSwapStore::default();
        let request_id = "31313131-3131-3131-3131-313131313131";
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(702u64).to_hex_string()),
            spend_commitment_ids: vec![Fr254::from(78u64).to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");
        db.push_request(mock_request(request_id, Some(child_args)))
            .await;

        let result = process_cancel_swap_request(
            &db,
            request_id,
            Some(I256::try_from(17u64).unwrap()),
            |_swap_link, _| async move { panic!("cancel-request should not contact proposers") },
        )
        .await
        .expect("cancel-request should return conflict");

        assert_eq!(result, advisory_cancel_after_deadline_execution());
    }

    #[tokio::test]
    async fn test_swap_api_rejects_unsupported_token_type_for_party_b() {
        let my_public_key_hex = {
            let my_key = crate::get_zkp_keys()
                .lock()
                .expect("Poisoned Mutex lock")
                .zkp_public_key;
            let mut bytes = Vec::new();
            my_key
                .serialize_with_mode(&mut bytes, Compress::Yes)
                .unwrap();
            bytes.reverse();
            format!("0x{}", hex::encode(bytes))
        };

        let req = NF3SwapRequest {
            party_a: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567890".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x00".to_string(),
                value: "0x04".to_string(),
                public_key: my_public_key_hex.clone(),
            },
            party_b: SwapParty {
                erc_address: "0x1234567890123456789012345678901234567891".to_string(),
                token_id: "0x00".to_string(),
                token_type: "0x09".to_string(),
                value: "0x05".to_string(),
                public_key: my_public_key_hex,
            },
            swap_nonce: "0x01".to_string(),
            deadline: "0x1000".to_string(),
            fee: "0x00".to_string(),
        };

        let res = handle_swap::<PlonkProof, PlonkProvingEngine, Nightfall::NightfallCalls>(
            req,
            "test-swap-invalid-token-type-party-b",
        )
        .await;

        match res {
            Err(TransactionHandlerError::CustomError(msg)) => {
                assert!(msg.contains("party_b.token_type"));
            }
            other => panic!("Expected unsupported token_type error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_returns_request_not_found() {
        let db = MockQuitSwapStore::default();
        let request_id = "11111111-1111-1111-1111-111111111111";

        let result =
            process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap()).await;

        assert!(matches!(
            result,
            Err(crate::domain::error::ClientRejection::RequestNotFound)
        ));
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_returns_conflict_when_no_child_args() {
        let db = MockQuitSwapStore::default();
        let request_id = "22222222-2222-2222-2222-222222222222";
        db.push_request(mock_request(request_id, None)).await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return an execution result");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 0,
                message: "No pending swap commitments found for this request",
            }
        );
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_unlocks_pending_commitment_after_deadline() {
        let db = MockQuitSwapStore::default();
        let request_id = "33333333-3333-3333-3333-333333333333";
        let commitment_id = Fr254::from(42u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(77u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(
            commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should succeed");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 1,
                skipped: 0,
                message: "Swap expired and commitments were unlocked locally",
            }
        );
        assert_eq!(
            db.get_commitment_status(commitment_id).await,
            Some(CommitmentStatus::Unspent)
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Expired)
        );
        let parsed = parsed_request_child_args(&db, request_id)
            .await
            .expect("settled child args should remain present");
        assert_eq!(parsed.deadline.as_deref(), Some("0x10"));
        assert_eq!(parsed.swap_link, Some(Fr254::from(77u64).to_hex_string()));
        assert!(parsed.spend_commitment_ids.is_empty());
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_refuses_before_deadline() {
        let db = MockQuitSwapStore::default();
        let request_id = "33333333-3333-3333-3333-333333333334";
        let commitment_id = Fr254::from(43u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(78u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(
            commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(16u64).unwrap())
            .await
            .expect("settle-expired should return conflict");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 0,
                message: "Swap deadline has not passed yet; use cancel-request for advisory proposer cancellation while commitments remain locked",
            }
        );
        assert_eq!(
            db.get_commitment_status(commitment_id).await,
            Some(CommitmentStatus::PendingSpend)
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Submitted)
        );
        assert!(db.request_child_args(request_id).await.flatten().is_some());
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_conflict_when_nothing_unlockable() {
        let db = MockQuitSwapStore::default();
        let request_id = "44444444-4444-4444-4444-444444444444";
        let commitment_id = Fr254::from(99u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(88u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(commitment_id, CommitmentStatus::Spent))
            .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return conflict");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 1,
                message: "No pending commitments could be unlocked",
            }
        );
        assert_eq!(
            db.get_commitment_status(commitment_id).await,
            Some(CommitmentStatus::Spent)
        );
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_conflict_when_any_commitment_is_not_pending_spend() {
        let db = MockQuitSwapStore::default();
        let request_id = "55555555-5555-5555-5555-555555555555";
        let pending_commitment_id = Fr254::from(100u64);
        let spent_commitment_id = Fr254::from(101u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(89u64).to_hex_string()),
            spend_commitment_ids: vec![
                pending_commitment_id.to_hex_string(),
                spent_commitment_id.to_hex_string(),
            ],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(
            pending_commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;
        db.push_commitment(mock_commitment(
            spent_commitment_id,
            CommitmentStatus::Spent,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return conflict");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 1,
                message: "Swap commitments are not all pending spend",
            }
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Submitted)
        );
        assert_eq!(
            db.get_commitment_status(pending_commitment_id).await,
            Some(CommitmentStatus::PendingSpend)
        );
        assert_eq!(
            db.get_commitment_status(spent_commitment_id).await,
            Some(CommitmentStatus::Spent)
        );
        assert!(db.request_child_args(request_id).await.flatten().is_some());
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_returns_conflict_before_deadline_even_if_metadata_exists(
    ) {
        let db = MockQuitSwapStore::default();
        let request_id = "66666666-6666-6666-6666-666666666666";
        let commitment_id = Fr254::from(102u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x20".to_string()),
            swap_link: Some(Fr254::from(90u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(
            commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(16u64).unwrap())
            .await
            .expect("settle-expired should return conflict");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 0,
                message: "Swap deadline has not passed yet; use cancel-request for advisory proposer cancellation while commitments remain locked",
            }
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Submitted)
        );
        assert_eq!(
            db.get_commitment_status(commitment_id).await,
            Some(CommitmentStatus::PendingSpend)
        );
        assert!(db.request_child_args(request_id).await.flatten().is_some());
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_rejects_processing_request_status() {
        let db = MockQuitSwapStore::default();
        let request_id = "77777777-7777-7777-7777-777777777777";
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(91u64).to_hex_string()),
            spend_commitment_ids: vec![Fr254::from(103u64).to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(Request {
            status: RequestStatus::Processing,
            uuid: request_id.to_string(),
            child_request_args: Some(child_args),
        })
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return conflict");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::CONFLICT,
                unlocked: 0,
                skipped: 0,
                message: "Swap request is still being processed and cannot be settled yet",
            }
        );
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_returns_ok_when_expired_request_is_already_reconciled(
    ) {
        let db = MockQuitSwapStore::default();
        let request_id = "88888888-8888-8888-8888-888888888888";
        db.push_request(Request {
            status: RequestStatus::Expired,
            uuid: request_id.to_string(),
            child_request_args: None,
        })
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return success");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 0,
                skipped: 0,
                message: "Swap was already expired and commitments were already reconciled",
            }
        );
        assert_eq!(db.request_child_args(request_id).await, Some(None));
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_retries_cleanly_when_request_is_already_expired() {
        let db = MockQuitSwapStore::default();
        let request_id = "12121212-1212-1212-1212-121212121212";
        let commitment_id = Fr254::from(107u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(94u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(Request {
            status: RequestStatus::Expired,
            uuid: request_id.to_string(),
            child_request_args: Some(child_args),
        })
        .await;
        db.push_commitment(mock_commitment(
            commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should return success");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 1,
                skipped: 0,
                message: "Swap expired and commitments were unlocked locally",
            }
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Expired)
        );
        assert_eq!(
            db.get_commitment_status(commitment_id).await,
            Some(CommitmentStatus::Unspent)
        );
        let parsed = parsed_request_child_args(&db, request_id)
            .await
            .expect("settled child args should remain present");
        assert!(parsed.spend_commitment_ids.is_empty());
        assert_eq!(parsed.deadline.as_deref(), Some("0x10"));
        assert_eq!(parsed.swap_link, Some(Fr254::from(94u64).to_hex_string()));
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_retries_cleanly_when_one_commitment_is_already_unlocked(
    ) {
        let db = MockQuitSwapStore::default();
        let request_id = "99999999-9999-9999-9999-999999999999";
        let pending_commitment_id = Fr254::from(105u64);
        let already_unlocked_commitment_id = Fr254::from(106u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(93u64).to_hex_string()),
            spend_commitment_ids: vec![
                pending_commitment_id.to_hex_string(),
                already_unlocked_commitment_id.to_hex_string(),
            ],
            cancel_requested_at: None,
            cancel_request_status: None,
            cancel_request_message: None,
        })
        .expect("serialize child args");

        db.push_request(mock_request(request_id, Some(child_args)))
            .await;
        db.push_commitment(mock_commitment(
            pending_commitment_id,
            CommitmentStatus::PendingSpend,
        ))
        .await;
        db.push_commitment(mock_commitment(
            already_unlocked_commitment_id,
            CommitmentStatus::Unspent,
        ))
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired retry should converge");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 1,
                skipped: 0,
                message: "Swap expired and commitments were unlocked locally",
            }
        );
        assert_eq!(
            db.request_status(request_id).await,
            Some(RequestStatus::Expired)
        );
        let parsed = parsed_request_child_args(&db, request_id)
            .await
            .expect("settled child args should remain present");
        assert!(parsed.spend_commitment_ids.is_empty());
        assert_eq!(parsed.deadline.as_deref(), Some("0x10"));
        assert_eq!(parsed.swap_link, Some(Fr254::from(93u64).to_hex_string()));
        assert_eq!(
            db.get_commitment_status(pending_commitment_id).await,
            Some(CommitmentStatus::Unspent)
        );
        assert_eq!(
            db.get_commitment_status(already_unlocked_commitment_id)
                .await,
            Some(CommitmentStatus::Unspent)
        );
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_is_idempotent_when_expired_and_spend_ids_empty() {
        let db = MockQuitSwapStore::default();
        let request_id = "45454545-4545-4545-4545-454545454545";
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(303u64).to_hex_string()),
            spend_commitment_ids: vec![],
            cancel_requested_at: Some("123".to_string()),
            cancel_request_status: Some("accepted".to_string()),
            cancel_request_message: Some("advisory cancel accepted".to_string()),
        })
        .expect("serialize child args");

        db.push_request(Request {
            status: RequestStatus::Expired,
            uuid: request_id.to_string(),
            child_request_args: Some(child_args),
        })
        .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should be idempotent");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 0,
                skipped: 0,
                message: "Swap was already expired and commitments were already reconciled",
            }
        );

        let parsed = parsed_request_child_args(&db, request_id)
            .await
            .expect("child args should remain present");
        assert!(parsed.spend_commitment_ids.is_empty());
        assert_eq!(parsed.cancel_requested_at.as_deref(), Some("123"));
        assert_eq!(parsed.cancel_request_status.as_deref(), Some("accepted"));
        assert_eq!(
            parsed.cancel_request_message.as_deref(),
            Some("advisory cancel accepted")
        );
    }

    #[tokio::test]
    async fn test_process_settle_expired_swap_reports_already_reconciled_when_all_commitments_are_already_unspent(
    ) {
        let db = MockQuitSwapStore::default();
        let request_id = "56565656-5656-5656-5656-565656565656";
        let commitment_id = Fr254::from(808u64);
        let child_args = serde_json::to_string(&SwapChildRequestArgs {
            deadline: Some("0x10".to_string()),
            swap_link: Some(Fr254::from(404u64).to_hex_string()),
            spend_commitment_ids: vec![commitment_id.to_hex_string()],
            cancel_requested_at: Some("123".to_string()),
            cancel_request_status: Some("accepted".to_string()),
            cancel_request_message: Some("advisory cancel accepted".to_string()),
        })
        .expect("serialize child args");

        db.push_request(Request {
            status: RequestStatus::Expired,
            uuid: request_id.to_string(),
            child_request_args: Some(child_args),
        })
        .await;
        db.push_commitment(mock_commitment(commitment_id, CommitmentStatus::Unspent))
            .await;

        let result = process_settle_expired_swap(&db, request_id, I256::try_from(17u64).unwrap())
            .await
            .expect("settle-expired should report prior reconciliation");

        assert_eq!(
            result,
            SettleExpiredExecution {
                status_code: StatusCode::OK,
                unlocked: 0,
                skipped: 0,
                message: "Swap was already reconciled locally",
            }
        );

        let parsed = parsed_request_child_args(&db, request_id)
            .await
            .expect("child args should remain present");
        assert!(parsed.spend_commitment_ids.is_empty());
        assert_eq!(parsed.cancel_request_status.as_deref(), Some("accepted"));
    }
}
