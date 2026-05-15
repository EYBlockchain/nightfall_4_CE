use crate::{
    domain::{
        entities::{TransferReceipt, TransferReceiptStatus, TxHashBytes},
        error::ProposerRejection,
    },
    initialisation::get_db_connection,
    ports::db::{TransactionsDB, TransferReceiptDB, TransferReceiptStoreError},
};
use lib::nf_client_proof::Proof;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::{path, Filter, Rejection, Reply};

const MAX_RECEIPT_ID_LEN: usize = 64;
const CIPHERTEXT_V1_HEX_LEN: usize = 576;
const SUPPORTED_VERSION: u8 = 1;

#[derive(Debug, PartialEq, Eq)]
enum ReceiptValidationError {
    InvalidTxHash,
    UnsupportedVersion,
    InvalidCiphertext,
}

impl ReceiptValidationError {
    fn into_rejection(self) -> Rejection {
        warp::reject::custom(ProposerRejection::TransferReceiptCreationFailed)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTransferReceiptRequest {
    pub tx_hash: String,
    pub ciphertext: String,
    pub version: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct CreateTransferReceiptResponse {
    pub receipt_id: String,
    pub status: TransferReceiptStatus,
}

#[derive(Debug, Serialize)]
pub struct TransferReceiptStatusResponse {
    pub receipt_id: String,
    pub status: TransferReceiptStatus,
}

pub fn create_transfer_receipt<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .and_then(handle_create_transfer_receipt::<P>)
}

pub fn get_transfer_receipt_status<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts" / String / "status")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handle_get_transfer_receipt_status::<P>)
}

pub fn get_transfer_receipt<P: Proof>(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("v1" / "transfer-receipts" / String)
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handle_get_transfer_receipt::<P>)
}

async fn handle_create_transfer_receipt<P: Proof>(
    request: CreateTransferReceiptRequest,
) -> Result<impl Reply, Rejection> {
    let tx_hash = validate_tx_hash(request.tx_hash.trim()).map_err(|e| e.into_rejection())?;
    let version = request.version.unwrap_or(SUPPORTED_VERSION);
    let db = get_db_connection().await;

    if let Some(existing) = db.get_transfer_receipt_by_tx_hash(&tx_hash).await {
        if existing.ciphertext != request.ciphertext || existing.version != version {
            return Err(warp::reject::custom(
                ProposerRejection::TransferReceiptConflict,
            ));
        }

        let response = CreateTransferReceiptResponse {
            receipt_id: existing.receipt_id.clone(),
            status: existing.status,
        };

        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ));
    }

    validate_new_receipt_fields(version, &request.ciphertext).map_err(|e| e.into_rejection())?;

    let tx_hash_vec = tx_hash.as_u32_vec();
    let tx_meta = <mongodb::Client as TransactionsDB<P>>::get_transaction(db, &tx_hash_vec)
        .await
        .ok_or_else(|| warp::reject::custom(ProposerRejection::TransferReceiptTxNotFound))?;

    let now = unix_now() as i64;
    let status = derive_status(tx_meta.lifecycle.block_l2());
    let receipt_id = generate_receipt_id();
    let receipt = TransferReceipt {
        receipt_id: receipt_id.clone(),
        tx_hash,
        ciphertext: request.ciphertext.clone(),
        version,
        status: status.clone(),
        created_at_unix: now,
        updated_at_unix: now,
    };

    match db.store_transfer_receipt(receipt).await {
        Ok(()) => {
            let response = CreateTransferReceiptResponse {
                receipt_id: receipt_id.clone(),
                status,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::CREATED,
            ))
        }
        Err(TransferReceiptStoreError::DuplicateKey) => {
            if let Some(existing) = db.get_transfer_receipt_by_tx_hash(&tx_hash).await {
                if existing.ciphertext != request.ciphertext || existing.version != version {
                    return Err(warp::reject::custom(
                        ProposerRejection::TransferReceiptConflict,
                    ));
                }

                let response = CreateTransferReceiptResponse {
                    receipt_id: existing.receipt_id.clone(),
                    status: existing.status,
                };

                return Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    warp::http::StatusCode::OK,
                ));
            }

            Err(warp::reject::custom(
                ProposerRejection::TransferReceiptCreationFailed,
            ))
        }
        Err(TransferReceiptStoreError::Other(_)) => Err(warp::reject::custom(
            ProposerRejection::TransferReceiptCreationFailed,
        )),
    }
}

async fn handle_get_transfer_receipt<P: Proof>(
    receipt_id: String,
) -> Result<impl Reply, Rejection> {
    if !is_valid_receipt_id(&receipt_id) {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptNotFound,
        ));
    }

    let db = get_db_connection().await;
    let mut receipt = db
        .get_transfer_receipt(&receipt_id)
        .await
        .ok_or_else(|| warp::reject::custom(ProposerRejection::TransferReceiptNotFound))?;

    refresh_status::<P>(db, &mut receipt).await;

    Ok(warp::reply::json(&receipt))
}

async fn handle_get_transfer_receipt_status<P: Proof>(
    receipt_id: String,
) -> Result<impl Reply, Rejection> {
    if !is_valid_receipt_id(&receipt_id) {
        return Err(warp::reject::custom(
            ProposerRejection::TransferReceiptNotFound,
        ));
    }

    let db = get_db_connection().await;
    let mut receipt = db
        .get_transfer_receipt(&receipt_id)
        .await
        .ok_or_else(|| warp::reject::custom(ProposerRejection::TransferReceiptNotFound))?;

    refresh_status::<P>(db, &mut receipt).await;

    Ok(warp::reply::json(&TransferReceiptStatusResponse {
        receipt_id: receipt.receipt_id,
        status: receipt.status,
    }))
}

async fn refresh_status<P: Proof>(db: &mongodb::Client, receipt: &mut TransferReceipt) {
    let tx_hash = receipt.tx_hash.as_u32_vec();
    if let Some(tx_meta) =
        <mongodb::Client as TransactionsDB<P>>::get_transaction(db, &tx_hash).await
    {
        let status = derive_status(tx_meta.lifecycle.block_l2());
        if status != receipt.status {
            let updated_at_unix = unix_now() as i64;
            if db
                .set_transfer_receipt_status(&receipt.receipt_id, status.clone(), updated_at_unix)
                .await
                .is_some()
            {
                receipt.status = status;
                receipt.updated_at_unix = updated_at_unix;
            } else {
                log::warn!(
                    "Failed to persist receipt status refresh for id={}",
                    receipt.receipt_id
                );
            }
        }
    }
}

fn is_valid_receipt_id(id: &str) -> bool {
    id.len() == MAX_RECEIPT_ID_LEN
        && id
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

fn derive_status(block_l2: Option<u64>) -> TransferReceiptStatus {
    if block_l2.is_some() {
        TransferReceiptStatus::IncludedL2
    } else {
        TransferReceiptStatus::Pending
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn generate_receipt_id() -> String {
    use rand::{rngs::OsRng, RngCore};

    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn validate_tx_hash(tx_hash: &str) -> Result<TxHashBytes, ReceiptValidationError> {
    TxHashBytes::from_hex(tx_hash).ok_or(ReceiptValidationError::InvalidTxHash)
}

fn validate_new_receipt_fields(
    version: u8,
    ciphertext: &str,
) -> Result<(), ReceiptValidationError> {
    if version != SUPPORTED_VERSION {
        return Err(ReceiptValidationError::UnsupportedVersion);
    }

    if !is_valid_receipt_ciphertext(version, ciphertext) {
        return Err(ReceiptValidationError::InvalidCiphertext);
    }

    Ok(())
}

fn is_valid_receipt_ciphertext(version: u8, ciphertext: &str) -> bool {
    version == SUPPORTED_VERSION
        && ciphertext.len() == CIPHERTEXT_V1_HEX_LEN
        && ciphertext.chars().all(|c| c.is_ascii_hexdigit())
        && hex::decode(ciphertext).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_status_pending_if_no_block() {
        assert_eq!(derive_status(None), TransferReceiptStatus::Pending);
    }

    #[test]
    fn derive_status_included_if_block() {
        assert_eq!(derive_status(Some(1)), TransferReceiptStatus::IncludedL2);
    }

    #[test]
    fn ciphertext_v1_matches_receipt_format() {
        let ciphertext = "ab".repeat(288);
        assert_eq!(ciphertext.len(), 576);
    }

    #[test]
    fn valid_v1_receipt_ciphertext_accepted() {
        assert!(is_valid_receipt_ciphertext(1, &"ab".repeat(288)));
    }

    #[test]
    fn extended_v1_ciphertext_rejected() {
        assert!(!is_valid_receipt_ciphertext(1, &"ab".repeat(400)));
    }

    #[test]
    fn short_ciphertext_rejected() {
        assert!(!is_valid_receipt_ciphertext(1, &"ab".repeat(100)));
    }

    #[test]
    fn oversized_ciphertext_rejected() {
        assert!(!is_valid_receipt_ciphertext(1, &"ab".repeat(1025)));
    }

    #[test]
    fn odd_length_ciphertext_rejected() {
        assert!(!is_valid_receipt_ciphertext(1, &"a".repeat(577)));
    }

    #[test]
    fn non_hex_ciphertext_rejected() {
        assert!(!is_valid_receipt_ciphertext(1, &"zz".repeat(288)));
    }

    #[test]
    fn unsupported_ciphertext_version_rejected() {
        assert!(!is_valid_receipt_ciphertext(2, &"ab".repeat(288)));
    }

    #[test]
    fn valid_receipt_id_accepted() {
        assert!(is_valid_receipt_id(&"a".repeat(64)));
    }

    #[test]
    fn receipt_id_rejects_uppercase() {
        assert!(!is_valid_receipt_id(&format!("{}{}", "a".repeat(63), "A")));
    }

    #[test]
    fn receipt_id_rejects_wrong_length() {
        assert!(!is_valid_receipt_id(&"a".repeat(63)));
    }
}
