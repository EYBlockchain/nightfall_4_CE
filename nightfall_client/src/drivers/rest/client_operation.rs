use crate::{
    domain::{
        entities::{CommitmentStatus, Operation, Preimage, RequestStatus},
        error::TransactionHandlerError,
        notifications::NotificationPayload,
    },
    driven::db::mongo::CommitmentEntry,
    drivers::derive_key::ZKPKeys,
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        commitments::Nullifiable,
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB, RequestDB},
        secret_hash::SecretHash,
    },
    services::client_operation::client_operation,
};
use alloy::rpc::types::TransactionReceipt;
use ark_bn254::Fr as Fr254;
use configuration::addresses::get_addresses;
use futures::future::join_all;
use lib::{
    blockchain_client::BlockchainClientConnection,
    hex_conversion::HexConvertible,
    initialisation::get_blockchain_client_connection,
    nf_client_proof::{Proof, ProvingEngine},
    shared_entities::ClientTransaction,
};
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use nightfall_bindings::artifacts::ProposerManager;
use reqwest::{Client, Error as ReqwestError};
use serde::Serialize;
use std::{error::Error, fmt::Debug, time::Duration};
use tokio::time::sleep;
use url::Url;
use warp::hyper::StatusCode;

#[allow(clippy::too_many_arguments)]
pub async fn handle_client_operation<P, E, N>(
    operation: Operation,
    spend_commitments: [impl Nullifiable; 4],
    new_commitments: [impl Nullifiable; 4],
    ephemeral_private_key: BJJScalar,
    recipient_address: Fr254,
    secret_preimages: [impl SecretHash; 4],
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof + Debug + serde::Serialize + Clone + Send + Sync,
    E: ProvingEngine<P> + Send + Sync,
    N: NightfallContract,
{
    debug!("{id} Handling client operation: {operation:?}");

    // get the zkp keys from the global state. They will have been created when the keys were requested using a mnemonic
    let ZKPKeys {
        zkp_private_key,
        root_key,
        zkp_public_key,
        nullifier_key,
        ..
    } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

    // We should store the change commitments, so that when they appear on-chain, we can add them to the commitments DB.
    // That will mean that they could potentially be spent.
    {
        let db = get_db_connection().await;
        let mut commitment_entries = vec![];
        for (i, commitment) in new_commitments.iter().enumerate() {
            let commitment_hash = commitment.hash().expect("Commitments must be hashable");
            let commitment_hex = commitment_hash.to_hex_string();
            // Add mapping between request and commitment
            match db.add_mapping(id, &commitment_hex).await {
                Ok(_) => debug!("{id} Mapped commitment to request"),
                Err(e) => error!("{id} Failed to  map commitment to request: {e}"),
            }
            // we only store the change commitments and only the ones that aren't default
            if commitment.get_public_key() == zkp_public_key
                && i != 0
                && commitment.get_preimage() != Preimage::default()
            {
                let nullifier = commitment
                    .nullifier_hash(&nullifier_key)
                    .expect("Nullifiers must be hashable");
                let commitment_entry = CommitmentEntry::new(
                    commitment.get_preimage(),
                    nullifier,
                    CommitmentStatus::PendingCreation,
                    None,
                    None,
                );
                commitment_entries.push(commitment_entry);
            }
        }
        if !commitment_entries.is_empty() {
            db.store_commitments(&commitment_entries, true)
                .await
                .ok_or(TransactionHandlerError::DatabaseError)?;
        }
    }
    // we should now have a situation where:
    // new_commitments[0] is the token commitment
    // new_commitments[1] is the token change commitment
    // new_commitments[2] is the fee commitment
    // new_commitments[3] is the fee change commitment

    // spend_commitments[0] is the first token spend commitment
    // spend_commitments[1] is the second token spend commitment
    // spend_commitments[2] is the first fee spend commitment
    // spend_commitments[3] is the second fee spend commitment

    let mut operation_result: ClientTransaction<P> = client_operation::<P, E>(
        &spend_commitments,
        &new_commitments,
        zkp_private_key,
        root_key,
        ephemeral_private_key,
        recipient_address,
        &secret_preimages,
        id,
    )
    .await
    .map_err(|e| {
        error!("{id} {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;
    // having done that, we can submit the nighfall transaction to proposer offchain

    let tx_receipt = process_transaction_offchain(&operation_result, id)
        .await
        .map_err(|e| TransactionHandlerError::CustomError(e.to_string()))?;
    info!("{id} {} transaction submitted", operation.operation_type);
    let mut operation_result_json = serde_json::to_value(&operation_result)
        .expect("Failed to serialize operation_result to JSON");
    for (key, items) in [
        (
            "historic_commitment_roots",
            &mut operation_result.historic_commitment_roots,
        ),
        ("commitments", &mut operation_result.commitments),
        ("nullifiers", &mut operation_result.nullifiers),
    ] {
        if let Some(field) = operation_result_json.get_mut(key) {
            *field = serde_json::json!(items
                .iter_mut()
                .map(|item| serde_json::to_value(item.to_hex_string()).unwrap())
                .collect::<Vec<_>>());
        }
    }

    if let Some(compressed_secret) = operation_result_json.get_mut("compressed_secrets") {
        let mut value_array = Vec::new();
        if let Some(ciphertext) = compressed_secret.get_mut("cipher_text") {
            for ciphertexts in operation_result.compressed_secrets.cipher_text.iter_mut() {
                let chunks = serde_json::to_value(ciphertexts.to_hex_string()).unwrap();
                //collect the chunks into a historic_commitment_roots array
                value_array.push(chunks);
            }
            *ciphertext = serde_json::json!(value_array);
        }
    }

    let response = serde_json::to_string(&(operation_result_json, tx_receipt))
        .map_err(TransactionHandlerError::JsonConversionError)?;

    let uuid = serde_json::to_string(id).map_err(TransactionHandlerError::JsonConversionError)?;

    Ok(NotificationPayload::TransactionEvent { response, uuid })
}

/// Only retry on network issues or timeouts
fn is_retriable_error(err: &ReqwestError) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

async fn send_to_proposer_with_retry<P: Serialize + Sync>(
    client: &Client,
    proposer: ProposerManager::Proposer,
    l2_transaction: &ClientTransaction<P>,
    id: &str,
    max_retries: u32,
    initial_backoff: Duration,
) -> Result<(), (String, bool)> {
    let url = match Url::parse(&proposer.url).and_then(|base| base.join("/v1/transaction")) {
        Ok(u) => u,
        Err(e) => {
            warn!("Skipping proposer with invalid URL {}: {}", proposer.url, e);
            return Err((format!("Invalid URL: {e}"), false));
        }
    };

    for attempt in 1..=max_retries {
        let body = serde_json::to_string(l2_transaction).unwrap();
        let resp = client
            .post(url.clone())
            .header("Content-Type", "application/json")
            .header("Content-Length", body.len().to_string()) // force length
            .body(body)
            .send()
            .await;
        match resp {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    debug!("{id} Successfully sent transaction to proposer at {url}");
                    return Ok(());
                } else {
                    let body = response.text().await.unwrap_or_default();
                    error!("{id} Error from proposer: HTTP {status} — Body: {body}");
                    if matches!(
                        status,
                        StatusCode::BAD_GATEWAY
                            | StatusCode::SERVICE_UNAVAILABLE
                            | StatusCode::GATEWAY_TIMEOUT
                    ) && attempt < max_retries
                    {
                        let backoff = initial_backoff * 2u32.pow(attempt - 1);
                        warn!("{id} Retrying proposer {} in {backoff:?}", proposer.url);
                        sleep(backoff).await;
                        continue;
                    } else {
                        let retriable = matches!(
                            status,
                            StatusCode::BAD_GATEWAY
                                | StatusCode::SERVICE_UNAVAILABLE
                                | StatusCode::GATEWAY_TIMEOUT
                        );
                        return Err((
                            format!("Proposer returned HTTP {status}: {body}"),
                            retriable,
                        ));
                    }
                }
            }
            Err(err) => {
                error!("{id} Network error sending to proposer {url}: {err:?}");
                if is_retriable_error(&err) && attempt < max_retries {
                    let backoff = initial_backoff * 2u32.pow(attempt - 1);
                    warn!("{id} Retrying network error in {backoff:?}");
                    sleep(backoff).await;
                    continue;
                } else {
                    return Err((format!("Network error: {err}"), true));
                }
            }
        }
    }

    Err((
        format!("Max retries exhausted for proposer {}", proposer.url),
        true,
    ))
}

pub async fn process_transaction_offchain<P: Serialize + Sync>(
    l2_transaction: &ClientTransaction<P>,
    id: &str,
) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
    info!("{id} Sending client transaction to all proposers concurrently.");
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
    let proposers_struct: Vec<ProposerManager::Proposer> =
        round_robin_instance.get_proposers().call().await?;
    let db = get_db_connection().await;

    let futures: Vec<_> = proposers_struct
        .into_iter()
        .map(|proposer| {
            send_to_proposer_with_retry(
                &client,
                proposer,
                l2_transaction,
                id,
                MAX_RETRIES,
                INITIAL_BACKOFF,
            )
        })
        .collect();

    let results = join_all(futures).await;

    let mut any_success = false;
    let mut any_retriable_failures = false;

    for result in results {
        match result {
            Ok(_) => {
                any_success = true;
            }
            Err((msg, retriable)) => {
                warn!("{id} Proposer error: {msg}");
                if retriable {
                    any_retriable_failures = true;
                }
            }
        }
    }

    if any_success {
        db.update_request(id, RequestStatus::Submitted).await;
        Ok(None)
    } else if any_retriable_failures {
        db.update_request(id, RequestStatus::ProposerUnreachable)
            .await;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            format!("{id} All proposers unreachable after retries."),
        )))
    } else {
        db.update_request(id, RequestStatus::Failed).await;
        Err(Box::new(std::io::Error::other(format!(
            "{id} All proposers rejected the transaction."
        ))))
    }
}
