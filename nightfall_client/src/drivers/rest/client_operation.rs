use configuration::addresses::get_addresses;
use ethers::types::TransactionReceipt;
use nightfall_bindings::nightfall::{ClientTransaction as NightfallTransactionStruct, Nightfall};
use std::{error::Error, fmt::Debug, time::Duration};
use reqwest::Error as ReqwestError;
use tokio::time::sleep;
use warp::reply::Json;
use warp::reply::WithStatus;
use warp::{hyper::StatusCode, reject, reply};

use crate::domain::entities::CommitmentStatus;
use crate::domain::entities::HexConvertible;
use crate::domain::entities::RequestStatus;
use crate::driven::db::mongo::CommitmentEntry;
use crate::drivers::blockchain::nightfall_event_listener::get_synchronisation_status;
use crate::drivers::derive_key::ZKPKeys;
use crate::drivers::rest::models::NullifierKey;
use crate::get_zkp_keys;
use crate::ports::db::RequestDB;
use crate::{
    domain::{
        entities::{ClientTransaction, Operation, Transport},
        error::FailedClientOperation,
    },
    initialisation::{get_db_connection, get_proposer_http_connection},
    ports::{
        commitments::Nullifiable,
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB},
        proof::{Proof, ProvingEngine},
        secret_hash::SecretHash,
    },
    services::client_operation::client_operation,
};
use ark_bn254::Fr as Fr254;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use serde::Serialize;
#[allow(clippy::too_many_arguments)]
pub async fn handle_client_operation<P, E, N>(
    operation: Operation,
    spend_commitments: [impl Nullifiable; 4],
    new_commitments: [impl Nullifiable; 4],
    ephemeral_private_key: BJJScalar,
    recipient_address: Fr254,
    secret_preimages: [impl SecretHash; 4],
    id: &str,
) -> Result<warp::reply::WithHeader<WithStatus<Json>>, warp::Rejection>
where
    P: Proof + Debug + serde::Serialize + Clone + Send + Sync,
    E: ProvingEngine<P> + Send + Sync,
    N: NightfallContract,
{
    debug!("{id} Handling client operation: {:?}", operation);

    let sync_state = get_synchronisation_status::<N>()
        .await
        .map_err(reject::custom)?
        .is_synchronised();
    if !sync_state {
        warn!("{id} Rejecting request - Proposer is not synchronised with the blockchain");
        return Ok(reply::with_header(
            reply::with_status(
                reply::json(&"Proposer is not synchronised with the blockchain"),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
            "X-Request-ID",
            id,
        ));
    }

    // get the zkp keys from the global state. They will have been created when the keys were requested using a mnemonic
    let ZKPKeys {
        zkp_private_key,
        zkp_public_key,
        nullifier_key,
        ..
    } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

    debug!("{id} Calling client_operation");
    // We should store the change commitments, so that when they appear on-chain, we can add them to the commitments DB.
    // That will mean that they could potentially be spent. Scope it so the lock gets dropped and other processes can access the DB.
    {
        let db = &mut get_db_connection().await.write().await;
        let mut commitment_entries = vec![];
        for commitment in new_commitments.iter() {
            if commitment.get_public_key() == zkp_public_key {
                let commitment_hash = commitment.hash().expect("Commitments must be hashable");
                let nullifier = commitment
                    .nullifier_hash(&nullifier_key)
                    .expect("Nullifiers must be hashable");
                let commitment_entry = CommitmentEntry::new(
                    commitment.get_preimage(),
                    commitment_hash,
                    nullifier,
                    CommitmentStatus::PendingCreation,
                );
                commitment_entries.push(commitment_entry);
            }
        }
        if (db.store_commitments(&commitment_entries, true).await).is_none() {
            error!("{id} Failed to store commitments");
            return Err(reject::custom(FailedClientOperation));
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
        NullifierKey(nullifier_key),
        zkp_private_key,
        ephemeral_private_key,
        recipient_address,
        &secret_preimages,
        id,
    )
    .await
    .map_err(|err| {
        error!("{} {}", err, id);
        reject::custom(FailedClientOperation)
    })?;
    // having done that, we can submit the nighfall transaction, either on or off chain, normally the latter

    let tx_receipt = match operation.transport {
        Transport::OnChain => process_transaction_onchain(&operation_result).await,
        Transport::OffChain => process_transaction_offchain(&operation_result, id).await,
    }
    .map_err(|_| reject::custom(FailedClientOperation))?;
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
    Ok(reply::with_header(
        reply::with_status(
            reply::json(&(operation_result_json, tx_receipt)),
            StatusCode::ACCEPTED,
        ),
        "X-Request-ID",
        id,
    ))
}
async fn process_transaction_offchain<P: Serialize>(
    l2_transaction: &ClientTransaction<P>,
    id: &str,
) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
    info!("{id} Sending client transaction to proposer");
    let (proposer_http_connection, url) = get_proposer_http_connection();
    let db = get_db_connection().await.write().await;

    // we need to update the request in the database to show that it's been sent to the proposer
    //ading BACKOFF
    // let proposer_response = proposer_http_connection
    //     .post(url.clone())
    //     .json(l2_transaction)
    //     .send()
    //     .await;
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

    for attempt in 1..=MAX_RETRIES {
        let proposer_response = proposer_http_connection
            .post(url.clone())
            .json(l2_transaction)
            .send()
            .await;
        match proposer_response {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    db.update_request(id, RequestStatus::Submitted).await;
                    debug!("{id} Successfully sent to proposer (attempt {attempt}).");
                    return Ok(None);
                } else {
                    let body = resp.text().await.unwrap_or_default();
                    error!("{id} Error from proposer: HTTP {} — Body: {}", status, body);

                    if matches!(status, StatusCode::BAD_GATEWAY | StatusCode::SERVICE_UNAVAILABLE | StatusCode::GATEWAY_TIMEOUT) {
                        if attempt < MAX_RETRIES {
                            let backoff = INITIAL_BACKOFF * 2u32.pow(attempt - 1);
                            warn!("{id} Retrying in {:?} (HTTP error {})", backoff, status);
                            sleep(backoff).await;
                            continue;
                        } else {
                            db.update_request(id, RequestStatus::ProposerUnreachable).await;
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::TimedOut,
                                format!("Proposer returned {} after {} attempts", status, MAX_RETRIES),
                            )));
                        }
                    } else {
                        db.update_request(id, RequestStatus::Failed).await;
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Proposer returned non-retryable error {} — {}", status, body),
                        )));
                    }
                }
            }
            Err(err) => {
                error!("{id} Network error sending to proposer: {err}");
                if is_retriable_error(&err) && attempt < MAX_RETRIES {
                    let backoff = INITIAL_BACKOFF * 2u32.pow(attempt - 1);
                    warn!("{id} Retrying after network error in {:?}", backoff);
                    sleep(backoff).await;
                } else {
                    db.update_request(id, RequestStatus::ProposerUnreachable).await;
                    return Err(Box::new(err));
                }
            }
        }
    }

    // All retries exhausted — unreachable should never be hit
    unreachable!()
}

/// Only retry on network issues or timeouts
fn is_retriable_error(err: &ReqwestError) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}
//     match proposer_response {
//         Ok(_) => {
//             db.update_request(id, RequestStatus::Submitted).await;
//             debug!("{id} Client transaction sent to proposer");
//         }
//         Err(err) => {
//             error!(
//                 "{id} Failed to send client transaction to proposer: {}",
//                 err
//             );
//             db.update_request(id, RequestStatus::Failed).await;
//             return Err(Box::new(err));
//         }
//     }
//     // obvs we won't have a transaction receipt to return if we've sent our transaction to a proposer but we need to return an Option<TransactionReceipt>
//     // for type compatibility with the onchain case
//     Ok(None)
// }

async fn process_transaction_onchain<P>(
    l2_transaction: &ClientTransaction<P>,
) -> Result<Option<TransactionReceipt>, Box<dyn Error>>
where
    P: Proof,
{
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    debug!("Creating contract instance");
    let nightfall_instance = Nightfall::new(get_addresses().nightfall, blockchain_client);
    debug!("Processing client transaction");
    let nightfall_l2_transaction = NightfallTransactionStruct::try_from(l2_transaction.clone())?;
    debug!("Creating nightfall submit_client_transaction function call");
    let call = nightfall_instance.submit_client_transaction(nightfall_l2_transaction);
    debug!("Sending transaction");
    let tx = call.send().await?;
    let tx_receipt = tx.await?;
    Ok(tx_receipt)
}
