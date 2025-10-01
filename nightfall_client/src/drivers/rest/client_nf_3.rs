use super::{
    client_operation::handle_client_operation,
    models::{NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest, NullifierKey},
    utils::to_nf_token_id_from_str,
};
use crate::{
    domain::{
        entities::{
            CommitmentStatus, DepositSecret, ERCAddress, Operation, OperationType, Preimage,
            RequestStatus, Salt, TokenType, Transport,
        },
        error::TransactionHandlerError,
        notifications::NotificationPayload,
    },
    driven::{
        contract_functions::contract_type_conversions::FrBn254,
        db::mongo::CommitmentEntry,
        queue::{get_queue, QueuedRequest, TransactionRequest},
    },
    drivers::{derive_key::ZKPKeys, DOMAIN_SHARED_SALT},
    get_fee_token_id, get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        commitments::{Commitment, Nullifiable},
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB, RequestDB},
        keys::KeySpending,
        proof::{Proof, ProvingEngine},
    },
    services::{
        client_operation::deposit_operation, commitment_selection::find_usable_commitments,
    },
};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine;
use ark_serialize::Valid;
use ark_ff::{BigInteger256, Zero};
use ark_std::{rand::thread_rng, UniformRand};
use configuration::{addresses::get_addresses, settings::get_settings};
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::{hex_conversion::HexConvertible, serialization::ark_de_hex};
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
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_deposit_request)
}

pub fn transfer_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "transfer")
        .and(warp::body::json())
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_transfer_request)
}

pub fn withdraw_request<P>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
{
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_withdraw_request)
}

/// function to queue the deposit requests
async fn queue_deposit_request(
    deposit_req: NF3DepositRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Deposit(deposit_req);
    debug!("Queueing deposit request");
    queue_request(transaction_request, request_id).await
}

/// function to queue the transfer requests
async fn queue_transfer_request(
    transfer_req: NF3TransferRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Transfer(transfer_req);
    queue_request(transaction_request, request_id).await
}

/// function to queue the withdraw requests
async fn queue_withdraw_request(
    withdraw_req: NF3WithdrawRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection> {
    let transaction_request = TransactionRequest::Withdraw(withdraw_req);
    queue_request(transaction_request, request_id).await
}

/// This function queues all types of transaction request
async fn queue_request(
    transaction_request: TransactionRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection> {
    let settings = get_settings();
    let max_queue_size = settings
        .nightfall_client
        .max_queue_size
        .unwrap_or(1000)
        .try_into()
        .unwrap();
    // extract the request ID
    let id = request_id.unwrap_or_default();
    // check if the id is a valid uuid
    if Uuid::parse_str(&id).is_err() {
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
            id,
        ));
    }
    debug!("got lock on queue");
    q.push_back(QueuedRequest {
        transaction_request,
        uuid: id.clone(),
    });
    drop(q); // drop the lock so other processes can access the queue
    debug!("Added request to queue");
    // record the request as queued
    let db = get_db_connection().await;
    if db.store_request(&id, RequestStatus::Queued).await.is_none() {
        return Err(warp::reject::custom(
            crate::domain::error::ClientRejection::DatabaseError,
        ));
    }
    debug!("Stored request status in database");

    // return a 202 Accepted response with the request ID
    Ok(reply::with_header(
        reply::with_status(json(&"Request queued".to_string()), StatusCode::ACCEPTED),
        "X-Request-ID",
        id,
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
    info!("Deposit raw request: {req:?}");

    // We convert the request into values
    let NF3DepositRequest {
        erc_address,
        token_id,
        token_type,
        value,
        fee,
        deposit_fee,
        secret_preimage_one,
        secret_preimage_two,
        secret_preimage_three,
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
    let (secret_preimage_one, secret_preimage_two, secret_preimage_three) =
        if let (Some(p1), Some(p2), Some(p3)) = (
            secret_preimage_one,
            secret_preimage_two,
            secret_preimage_three,
        ) {
            let secret_preimage_one: Fr254 =
                Fr254::from_hex_string(p1.as_str()).map_err(|err| {
                    error!("{id} Could not wrangle secret preimage one {err}");
                    TransactionHandlerError::CustomError(err.to_string())
                })?;
            let secret_preimage_two: Fr254 = Fr254::from_hex_string(p2.as_str())
                .map_err(|err| TransactionHandlerError::CustomError(err.to_string()))?;
            let secret_preimage_three: Fr254 =
                Fr254::from_hex_string(p3.as_str()).map_err(|err| {
                    error!("{id} Could not wrangle secret preimage three {err}");
                    TransactionHandlerError::CustomError(err.to_string())
                })?;
            (
                secret_preimage_one,
                secret_preimage_two,
                secret_preimage_three,
            )
        } else {
            info!("{id} No secret preimage found for deposit request, generating");
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

    let commitment_entry =
        CommitmentEntry::new(preimage_value, nullifier, CommitmentStatus::PendingCreation);

    db.store_commitment(commitment_entry.clone())
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

        let commitment_entry =
            CommitmentEntry::new(preimage_fee, nullifier, CommitmentStatus::PendingCreation);
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

    // add the id to the request database
    let db = get_db_connection().await;
    db.store_request(id, RequestStatus::Queued)
        .await
        .ok_or(TransactionHandlerError::DatabaseError)?;

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), token_id.as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {e}"
            );
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    let keys = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

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
    let deserialized_public_key: JubJubPubKey =
        serde_json::from_str(&json_wrapped).map_err(|e| {
            error!("{id} Could not deserialize recipient public key: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;

    let recipient_public_key = deserialized_public_key.0;

    let ephemeral_private_key = {
        let mut rng = ark_std::rand::thread_rng(); // TODO initialise in main and pass around as a rwlock
        BJJScalar::rand(&mut rng)
    };
    let shared_secret: Affine<BabyJubjub> = (recipient_public_key * ephemeral_private_key).into();

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
            find_usable_commitments(fee_token_id, fee, db)
            .await
            .map_err(|e| {
                error!(
                    "{id} Could not find enough usable fee commitments to complete this transfer, suggest depositing more fee: {e}");
                TransactionHandlerError::CustomError(e.to_string())
            })?
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

    // transferred value commitment, salt is the y-coordinate of the shared secret
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
    handle_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        ephemeral_private_key,
        Fr254::zero(),
        secret_preimages,
        id,
    )
    .await
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

    let keys = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

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
    // TODO: update APIs so that we allow passing in specific commitments.
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
            find_usable_commitments(fee_token_id, fee, db)
            .await
            .map_err(|e| {
                error!(
                    "{id} Could not find enough usable fee commitments to complete this withdraw, suggest depositing more fee: {e}"
                );
                TransactionHandlerError::CustomError(e.to_string())
            })?
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
    let poseidon = Poseidon::<Fr254>::new();
    let withdraw_fund_salt = poseidon
        .hash(&[
            NullifierKey(keys.nullifier_key).get_nullifier_key(),
            spend_commitments[0]
                .hash()
                .expect("Failed to hash spend_commitments[0]"),
        ])
        .unwrap();
    handle_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        BJJScalar::zero(),
        recipient_address,
        secret_preimages,
        id,
    )
    .await?;
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
