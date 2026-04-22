/// REST handlers for transfer receipt create / resolve / status endpoints.
use crate::{
    domain::{
        entities::{TransferReceipt, TransferReceiptStatus},
        error::ProposerRejection,
    },
    initialisation::get_db_connection,
    ports::db::{TransactionsDB, TransferReceiptDB, TransferReceiptStoreError},
};
use lib::nf_client_proof::Proof;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::{
    hyper::StatusCode,
    path,
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

// ---------------------------------------------------------------------------
// Validation constants
// ---------------------------------------------------------------------------

const MAX_TX_HASH_LEN: usize = 32;
const MAX_RECEIPT_ID_HEX_LEN: usize = 64;

// Receipt v1 ciphertext: [Fr254; 9] = 9 × 32 bytes = 288 bytes = 576 hex chars.
const MIN_CIPHERTEXT_V1_HEX_LEN: usize = 576;
const MAX_CIPHERTEXT_HEX_LEN: usize = 2048;

const SUPPORTED_VERSION: u8 = 1;

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateTransferReceiptRequest {
    pub tx_hash: Vec<u32>,
    pub ciphertext: String,
    pub version: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct CreateTransferReceiptResponse {
    pub receipt_id: String,
    pub status: String,
    pub link_path: String,
}

#[derive(Debug, Serialize)]
pub struct GetTransferReceiptResponse {
    pub receipt_id: String,
    pub tx_hash: Vec<u32>,
    pub ciphertext: String,
    pub version: u8,
    pub status: String,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Debug, Serialize)]
pub struct TransferReceiptStatusResponse {
    pub receipt_id: String,
    pub status: String,
}

// ---------------------------------------------------------------------------
// Route builders
// ---------------------------------------------------------------------------

pub fn create_transfer_receipt<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts")
        .and(warp::post())
        .and(warp::body::content_length_limit(64 * 1024))
        .and(warp::body::json())
        .and_then(|req| handle_create_transfer_receipt::<P>(req))
}

pub fn get_transfer_receipt_status<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts" / String / "status")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(|id: String| handle_get_transfer_receipt_status::<P>(id))
}

pub fn get_transfer_receipt<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts" / String)
        .and(warp::get())
        .and(warp::path::end())
        .and_then(|id: String| handle_get_transfer_receipt::<P>(id))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn generate_receipt_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

fn tx_hash_to_hex(tx_hash: &[u32]) -> String {
    let bytes: Vec<u8> = tx_hash.iter().map(|&v| v as u8).collect();
    hex::encode(bytes)
}

fn validate_receipt_id(id: &str) -> bool {
    id.len() == MAX_RECEIPT_ID_HEX_LEN && id.chars().all(|c| c.is_ascii_hexdigit())
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn status_to_string(s: &TransferReceiptStatus) -> String {
    match s {
        TransferReceiptStatus::Created => "created".to_string(),
        TransferReceiptStatus::Pending => "pending".to_string(),
        TransferReceiptStatus::IncludedL2 => "included_l2".to_string(),
        TransferReceiptStatus::Failed => "failed".to_string(),
    }
}

fn derive_status(block_l2: Option<u64>) -> TransferReceiptStatus {
    if block_l2.is_some() {
        TransferReceiptStatus::IncludedL2
    } else {
        TransferReceiptStatus::Pending
    }
}

// ---------------------------------------------------------------------------
// Create handler
// ---------------------------------------------------------------------------

async fn handle_create_transfer_receipt<P: Proof>(
    request: CreateTransferReceiptRequest,
) -> Result<impl Reply, Rejection> {
    // 1. Validate tx_hash length and byte range.
    if request.tx_hash.len() != MAX_TX_HASH_LEN {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        ));
    }
    if request.tx_hash.iter().any(|&v| v > 255) {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        ));
    }

    // 2. Resolve version.
    let version = request.version.unwrap_or(SUPPORTED_VERSION);

    let tx_hash_hex = tx_hash_to_hex(&request.tx_hash);
    let db = get_db_connection().await;

    // 3. Check if a receipt already exists for this tx_hash.
    if let Some(existing) = db.get_transfer_receipt_by_tx_hash(&tx_hash_hex).await {
        // 4. Idempotent: same ciphertext + same version → 200 OK.
        if existing.ciphertext == request.ciphertext && existing.version == version {
            let resp = CreateTransferReceiptResponse {
                receipt_id: existing.receipt_id.clone(),
                status: status_to_string(&existing.status),
                link_path: format!("/v1/transfer-receipts/{}", existing.receipt_id),
            };
            return Ok(reply::with_status(reply::json(&resp), StatusCode::OK).into_response());
        } else {
            // 4b. Different ciphertext or version → 409 Conflict.
            return Err(warp::reject::custom(ProposerRejection::TransferReceiptConflict));
        }
    }

    // 5. No existing receipt — reject unsupported versions.
    if version != SUPPORTED_VERSION {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        ));
    }

    // 5b. Validate ciphertext: must be even-length hex, within size bounds, decodable.
    let ct_len = request.ciphertext.len();
    if ct_len < MIN_CIPHERTEXT_V1_HEX_LEN
        || ct_len > MAX_CIPHERTEXT_HEX_LEN
        || ct_len % 2 != 0
        || hex::decode(&request.ciphertext).is_err()
    {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        ));
    }

    // 5c. Ensure referenced transaction exists in proposer DB.
    let tx = <mongodb::Client as TransactionsDB<P>>::get_transaction(db, &request.tx_hash).await;
    let Some(tx_meta) = tx else {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptTxNotFound,
        ));
    };

    // 5d. Derive initial status from block_l2.
    let status = derive_status(tx_meta.block_l2);
    let now = now_unix();
    let receipt_id = generate_receipt_id();

    let receipt = TransferReceipt {
        receipt_id: receipt_id.clone(),
        tx_hash: request.tx_hash.clone(),
        tx_hash_hex: tx_hash_hex.clone(),
        ciphertext: request.ciphertext.clone(),
        version,
        status: status.clone(),
        created_at_unix: now,
        updated_at_unix: now,
    };

    // 5e. Insert; handle duplicate-key race.
    match db.store_transfer_receipt(receipt).await {
        Ok(_) => {
            let resp = CreateTransferReceiptResponse {
                receipt_id: receipt_id.clone(),
                status: status_to_string(&status),
                link_path: format!("/v1/transfer-receipts/{receipt_id}"),
            };
            Ok(reply::with_status(reply::json(&resp), StatusCode::CREATED).into_response())
        }
        Err(TransferReceiptStoreError::DuplicateKey) => {
            // Race: re-read and resolve.
            if let Some(existing) = db.get_transfer_receipt_by_tx_hash(&tx_hash_hex).await {
                if existing.ciphertext == request.ciphertext && existing.version == version {
                    let resp = CreateTransferReceiptResponse {
                        receipt_id: existing.receipt_id.clone(),
                        status: status_to_string(&existing.status),
                        link_path: format!("/v1/transfer-receipts/{}", existing.receipt_id),
                    };
                    Ok(reply::with_status(reply::json(&resp), StatusCode::OK).into_response())
                } else {
                    Err(warp::reject::custom(ProposerRejection::TransferReceiptConflict))
                }
            } else {
                Err(warp::reject::custom(
                    ProposerRejection::TransferReceiptCreationFailed,
                ))
            }
        }
        Err(TransferReceiptStoreError::Other(_)) => Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        )),
    }
}

// ---------------------------------------------------------------------------
// Resolve handler
// ---------------------------------------------------------------------------

async fn handle_get_transfer_receipt<P: Proof>(
    receipt_id: String,
) -> Result<impl Reply, Rejection> {
    if !validate_receipt_id(&receipt_id) {
        return Err(warp::reject::custom(ProposerRejection::TransferReceiptNotFound));
    }

    let db = get_db_connection().await;
    let Some(mut receipt) = db.get_transfer_receipt(&receipt_id).await else {
        return Err(warp::reject::custom(ProposerRejection::TransferReceiptNotFound));
    };

    // Refresh status from current transaction metadata.
    if let Some(tx_meta) = <mongodb::Client as TransactionsDB<P>>::get_transaction(db, &receipt.tx_hash).await {
        let refreshed = derive_status(tx_meta.block_l2);
        if refreshed != receipt.status {
            let now = now_unix();
            let _ = db
                .set_transfer_receipt_status(&receipt_id, refreshed.clone(), now)
                .await;
            receipt.status = refreshed;
            receipt.updated_at_unix = now;
        }
    }

    let resp = GetTransferReceiptResponse {
        receipt_id: receipt.receipt_id,
        tx_hash: receipt.tx_hash,
        ciphertext: receipt.ciphertext,
        version: receipt.version,
        status: status_to_string(&receipt.status),
        created_at_unix: receipt.created_at_unix,
        updated_at_unix: receipt.updated_at_unix,
    };
    Ok(reply::json(&resp).into_response())
}

// ---------------------------------------------------------------------------
// Status handler
// ---------------------------------------------------------------------------

async fn handle_get_transfer_receipt_status<P: Proof>(
    receipt_id: String,
) -> Result<impl Reply, Rejection> {
    if !validate_receipt_id(&receipt_id) {
        return Err(warp::reject::custom(ProposerRejection::TransferReceiptNotFound));
    }

    let db = get_db_connection().await;
    let Some(mut receipt) = db.get_transfer_receipt(&receipt_id).await else {
        return Err(warp::reject::custom(ProposerRejection::TransferReceiptNotFound));
    };

    // Refresh status.
    if let Some(tx_meta) = <mongodb::Client as TransactionsDB<P>>::get_transaction(db, &receipt.tx_hash).await {
        let refreshed = derive_status(tx_meta.block_l2);
        if refreshed != receipt.status {
            let now = now_unix();
            let _ = db
                .set_transfer_receipt_status(&receipt_id, refreshed.clone(), now)
                .await;
            receipt.status = refreshed;
        }
    }

    let resp = TransferReceiptStatusResponse {
        receipt_id: receipt.receipt_id,
        status: status_to_string(&receipt.status),
    };
    Ok(reply::json(&resp).into_response())
}
