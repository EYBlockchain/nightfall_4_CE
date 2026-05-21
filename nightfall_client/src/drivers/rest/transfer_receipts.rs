use crate::domain::error::ClientRejection;
use configuration::addresses::get_addresses;
use futures::future::join_all;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, error, warn};
use nightfall_bindings::artifacts::ProposerManager;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;
use warp::{path, Filter, Rejection, Reply};

/// Request body accepted by `POST /v1/transfer-receipts` on the **client**.
/// It is forwarded verbatim to every proposer's matching endpoint.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubmitReceiptRequest {
    pub tx_hash: String,
    pub ciphertext: String,
    pub version: Option<u8>,
    pub receipt_token: String,
}

/// The subset of the proposer's `CreateTransferReceiptResponse` that the
/// client proxies back to the caller.
#[derive(Debug, Serialize, Deserialize)]
struct ProposerReceiptResponse {
    receipt_id: String,
    status: serde_json::Value,
}

#[derive(Debug)]
enum ProposerReceiptError {
    /// 401 from proposer — token mismatch.
    Unauthorized,
    /// 409 from proposer — different ciphertext already stored.
    Conflict,
    /// 400 from proposer — tx not yet in proposer mempool.
    TxNotFound,
    /// 400 from proposer — request body failed validation (e.g. malformed tx_hash).
    ValidationFailed,
    /// Network or unexpected HTTP error.
    Other(String),
}

pub fn submit_transfer_receipt() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
{
    path!("v1" / "transfer-receipts")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .and_then(handle_submit_transfer_receipt)
}

async fn handle_submit_transfer_receipt(
    request: SubmitReceiptRequest,
) -> Result<impl Reply, Rejection> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

    let client = Client::new();

    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let round_robin_instance =
        ProposerManager::new(get_addresses().round_robin, blockchain_client.root());

    let proposers = round_robin_instance
        .get_proposers()
        .call()
        .await
        .map_err(|_| warp::reject::custom(ClientRejection::ProposerError))?;

    let futures: Vec<_> = proposers
        .into_iter()
        .map(|proposer| {
            send_receipt_to_proposer_with_retry(
                client.clone(),
                proposer,
                request.clone(),
                MAX_RETRIES,
                INITIAL_BACKOFF,
            )
        })
        .collect();

    let results = join_all(futures).await;

    // Aggregate: the first success wins.  Track error categories so we can
    // return the most informative failure if every proposer rejects.
    let mut first_success: Option<(ProposerReceiptResponse, warp::http::StatusCode)> = None;
    let mut saw_conflict = false;
    let mut saw_unauthorized = false;
    let mut saw_tx_not_found = false;
    let mut saw_validation_failed = false;

    for result in results {
        match result {
            Ok((resp, http_status)) => {
                if first_success.is_none() {
                    first_success = Some((resp, http_status));
                }
            }
            Err(ProposerReceiptError::Conflict) => saw_conflict = true,
            Err(ProposerReceiptError::Unauthorized) => saw_unauthorized = true,
            Err(ProposerReceiptError::TxNotFound) => saw_tx_not_found = true,
            Err(ProposerReceiptError::ValidationFailed) => saw_validation_failed = true,
            Err(ProposerReceiptError::Other(msg)) => {
                warn!("Proposer receipt error: {msg}");
            }
        }
    }

    if let Some((resp, http_status)) = first_success {
        return Ok(warp::reply::with_status(
            warp::reply::json(&resp),
            http_status,
        ));
    }

    // No proposer accepted — return the most specific error.
    if saw_conflict {
        return Err(warp::reject::custom(ClientRejection::ReceiptConflict));
    }
    if saw_unauthorized {
        return Err(warp::reject::custom(ClientRejection::ReceiptUnauthorized));
    }
    if saw_validation_failed {
        return Err(warp::reject::custom(ClientRejection::ReceiptValidationFailed));
    }
    if saw_tx_not_found {
        return Err(warp::reject::custom(ClientRejection::ReceiptTxNotFound));
    }

    Err(warp::reject::custom(
        ClientRejection::ReceiptSubmissionFailed,
    ))
}

async fn send_receipt_to_proposer_with_retry(
    client: Client,
    proposer: ProposerManager::Proposer,
    request: SubmitReceiptRequest,
    max_retries: u32,
    initial_backoff: Duration,
) -> Result<(ProposerReceiptResponse, warp::http::StatusCode), ProposerReceiptError> {
    let url = Url::parse(&proposer.url)
        .and_then(|base| base.join("/v1/transfer-receipts"))
        .map_err(|e| ProposerReceiptError::Other(format!("Invalid URL: {e}")))?;

    for attempt in 1..=max_retries {
        let body = serde_json::to_string(&request)
            .expect("SubmitReceiptRequest serialization cannot fail");

        let resp = client
            .post(url.clone())
            .header("Content-Type", "application/json")
            .header("Content-Length", body.len().to_string())
            .body(body)
            .send()
            .await;

        match resp {
            Ok(response) => {
                let status = response.status();
                match status {
                    s if s == reqwest::StatusCode::CREATED => {
                        let body = response.text().await.unwrap_or_default();
                        let parsed = serde_json::from_str::<ProposerReceiptResponse>(&body)
                            .map_err(|e| {
                                ProposerReceiptError::Other(format!("Parse error: {e}"))
                            })?;
                        debug!("Receipt stored (201 Created) at proposer {}", proposer.url);
                        return Ok((parsed, warp::http::StatusCode::CREATED));
                    }
                    s if s == reqwest::StatusCode::OK => {
                        let body = response.text().await.unwrap_or_default();
                        let parsed = serde_json::from_str::<ProposerReceiptResponse>(&body)
                            .map_err(|e| {
                                ProposerReceiptError::Other(format!("Parse error: {e}"))
                            })?;
                        debug!(
                            "Receipt idempotent replay (200 OK) at proposer {}",
                            proposer.url
                        );
                        return Ok((parsed, warp::http::StatusCode::OK));
                    }
                    s if s == reqwest::StatusCode::UNAUTHORIZED => {
                        error!("Receipt unauthorized (401) at proposer {}", proposer.url);
                        return Err(ProposerReceiptError::Unauthorized);
                    }
                    s if s == reqwest::StatusCode::CONFLICT => {
                        error!("Receipt conflict (409) at proposer {}", proposer.url);
                        return Err(ProposerReceiptError::Conflict);
                    }
                    s if s == reqwest::StatusCode::BAD_REQUEST => {
                        let body = response.text().await.unwrap_or_default();
                        warn!(
                            "Receipt bad request (400) at proposer {}: {}",
                            proposer.url, body
                        );
                        // Proposer uses 400 for two distinct cases:
                        //   "Transfer receipt transaction not found" → tx not yet in mempool (retriable)
                        //   "Transfer receipt creation failed"        → validation/input error (fatal)
                        if body.contains("not found") {
                            return Err(ProposerReceiptError::TxNotFound);
                        } else {
                            return Err(ProposerReceiptError::ValidationFailed);
                        }
                    }
                    s if matches!(
                        s,
                        reqwest::StatusCode::BAD_GATEWAY
                            | reqwest::StatusCode::SERVICE_UNAVAILABLE
                            | reqwest::StatusCode::GATEWAY_TIMEOUT
                    ) && attempt < max_retries =>
                    {
                        let backoff = initial_backoff * 2u32.pow(attempt - 1);
                        warn!(
                            "Proposer {} returned {s}, retrying in {backoff:?}",
                            proposer.url
                        );
                        sleep(backoff).await;
                        continue;
                    }
                    s => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(ProposerReceiptError::Other(format!(
                            "Proposer {} returned HTTP {s}: {body}",
                            proposer.url
                        )));
                    }
                }
            }
            Err(err) => {
                if attempt < max_retries {
                    let backoff = initial_backoff * 2u32.pow(attempt - 1);
                    warn!(
                        "Network error sending receipt to proposer {}, retrying in {backoff:?}: {err}",
                        proposer.url
                    );
                    sleep(backoff).await;
                    continue;
                }
                return Err(ProposerReceiptError::Other(format!(
                    "Network error for proposer {}: {err}",
                    proposer.url
                )));
            }
        }
    }

    Err(ProposerReceiptError::Other(format!(
        "Max retries exhausted for proposer {}",
        proposer.url
    )))
}
