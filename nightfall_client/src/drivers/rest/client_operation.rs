use crate::{
    domain::{
        entities::{
            ClientTransaction, CommitmentStatus, HexConvertible, Operation, RequestStatus,
            Transport,
        },
        error::FailedClientOperation,
    },
    driven::db::mongo::CommitmentEntry,
    drivers::{
        blockchain::nightfall_event_listener::get_synchronisation_status, derive_key::ZKPKeys,
        rest::models::NullifierKey,
    },
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        commitments::Nullifiable,
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestDB},
        proof::{Proof, ProvingEngine},
        secret_hash::SecretHash,
    },
    services::client_operation::client_operation,
};
use ark_bn254::Fr as Fr254;
use configuration::addresses::get_addresses;
use ethers::types::TransactionReceipt;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use nightfall_bindings::{
    nightfall::{ClientTransaction as NightfallTransactionStruct, Nightfall},
    round_robin::RoundRobin,
    x509::Proposer,
};
use serde::Serialize;
use std::{error::Error, fmt::Debug};
use url::Url;
use warp::{
    hyper::StatusCode,
    reject, reply,
    reply::{Json, WithStatus},
};
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
    let client = reqwest::Client::new();
    let round_robin_instance = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    let proposers_struct: Vec<Proposer> = round_robin_instance.get_proposers().call().await?;
    info!("{id} Sending client transaction to proposer");
    let db = get_db_connection().await.write().await;
    // Send to each proposer’s /v1/transaction endpoint
    for proposer in proposers_struct {
        let url = match Url::parse(&proposer.url).and_then(|base| base.join("/v1/transaction")) {
            Ok(u) => u,
            Err(e) => {
                log::warn!("Skipping proposer with invalid URL {}: {}", proposer.url, e);
                continue;
            }
        };

        let result = client.post(url.clone()).json(l2_transaction).send().await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    db.update_request(id, RequestStatus::Submitted).await;
                    debug!("{id} Successfully sent transaction to proposer at {}", url);
                } else {
                    error!(
                        "Failed to send transaction to proposer at {}. HTTP status: {}",
                        url,
                        response.status()
                    );
                }
            }
            Err(e) => {
                error!("Failed to send transaction to proposer at {}: {:?}", url, e);
            }
        }
    }

    Ok(None) // As per current off-chain flow, return no receipt
}
