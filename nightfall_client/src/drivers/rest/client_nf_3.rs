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
    client_models::{NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest},
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
use log::{debug, error, info};
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
            handle_transfer::<P, E, N>(transfer_req, request_id).await
        }
        TransactionRequest::Withdraw(withdraw_req) => {
            handle_withdraw::<P, E, N>(withdraw_req, request_id).await
        }
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

    let token_type: TokenType = u8::from_str_radix(&token_type, 16)
        .map_err(|err| {
            error!("{id} Could not convert token type");
            TransactionHandlerError::CustomError(err.to_string())
        })?
        .into();

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
    }
    .map_err(TransactionHandlerError::DepositError)?;

    // Insert the preimage into the commitments DB as pending creation
    // TODO remove the blocknumber
    let ZKPKeys { nullifier_key, .. } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");
    let nullifier = preimage_value
        .nullifier_hash(&nullifier_key)
        .expect("Could not hash commitment {}");
    let commitment_hash = preimage_value.hash().expect("Could not hash commitment");
    let commitment_entry = CommitmentEntry::new(
        preimage_value,
        nullifier,
        CommitmentStatus::PendingCreation,
        None,
        None,
    );

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

        let commitment_entry = CommitmentEntry::new(
            preimage_fee,
            nullifier,
            CommitmentStatus::PendingCreation,
            None,
            None,
        );
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

    let value =
        Fr254::from_hex_string(recipient_data.values.first().unwrap().as_str()).map_err(|e| {
            error!("{id} Error when reading value: {e}");
            TransactionHandlerError::CustomError(e.to_string())
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
    let db = get_db_connection().await;
    db.store_request(id, RequestStatus::Queued)
        .await
        .ok_or(TransactionHandlerError::DatabaseError)?;

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

                    for commitment_id in &value_commitment_ids {
                        if let Some(existing) = db.get_commitment(commitment_id).await {
                            let _ = db
                                .mark_commitments_unspent(
                                    &[*commitment_id],
                                    existing.layer_1_transaction_hash,
                                    existing.layer_2_block_number,
                                )
                                .await;
                        }
                    }
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

    dbg!(new_commitments
        .iter()
        .map(|c| c.hash().unwrap().to_hex_string())
        .collect::<Vec<_>>());

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
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();

            info!(
                "{id} Rolling back {} spend commitments",
                commitment_ids.len()
            );

            for commitment_id in &commitment_ids {
                if let Some(existing) = db.get_commitment(commitment_id).await {
                    let _ = db
                        .mark_commitments_unspent(
                            &[*commitment_id],
                            existing.layer_1_transaction_hash,
                            existing.layer_2_block_number,
                        )
                        .await;
                }
            }
            // Delete new commitments
            let new_commitment_ids = new_commitments
                .iter()
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();

            info!("{id} Deleting {} new commitments", new_commitment_ids.len());
            let _ = db.delete_commitments(new_commitment_ids).await;

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
    let db = get_db_connection().await;
    db.store_request(id, RequestStatus::Queued)
        .await
        .ok_or(TransactionHandlerError::DatabaseError)?;

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

    {
        let db = get_db_connection().await;
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
                    for commitment_id in &value_commitment_ids {
                        if let Some(existing) = db.get_commitment(commitment_id).await {
                            let _ = db
                                .mark_commitments_unspent(
                                    &[*commitment_id],
                                    existing.layer_1_transaction_hash,
                                    existing.layer_2_block_number,
                                )
                                .await;
                        }
                    }
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
        id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => {
            // Rollback to UNSPENT status if handle_client_operation fails
            let db = get_db_connection().await;

            // Rollback spend commitments
            let commitment_ids = spend_commitments
                .iter()
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();

            info!(
                "{id} Rolling back {} spend commitments to Unspent",
                commitment_ids.len()
            );
            for commitment_id in &commitment_ids {
                if let Some(existing) = db.get_commitment(commitment_id).await {
                    let _ = db
                        .mark_commitments_unspent(
                            &[*commitment_id],
                            existing.layer_1_transaction_hash,
                            existing.layer_2_block_number,
                        )
                        .await;
                }
            }

            // Delete new commitments
            let new_commitment_ids = new_commitments
                .iter()
                .map(|c| c.hash().unwrap())
                .collect::<Vec<_>>();

            info!("{id} Deleting {} new commitments", new_commitment_ids.len());
            let _ = db.delete_commitments(new_commitment_ids).await;
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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::One;
    use ark_serialize::{CanonicalSerialize, Compress};
    use ark_std::Zero;
    use lib::{
        client_models::NF3RecipientData,
        plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
    };
    use nf_curves::ed_on_bn254::BabyJubjub;
    use nf_curves::ed_on_bn254::Fq;

    /// Tests that transfer API rejects invalid recipient public keys
    #[tokio::test]
    async fn test_transfer_api_rejects_invalid_recipient_keys() {
        // Invalid compressed point (not on the curve) should fail early in handle_transfer
        let invalid_transfer_req = NF3TransferRequest {
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "0x00".to_string(),
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
}
