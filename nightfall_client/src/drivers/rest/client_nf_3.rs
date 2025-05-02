use super::{
    client_operation::handle_client_operation,
    models::{NF3DepositRequest, NF3TransferRequest, NF3WithdrawRequest, NullifierKey},
    utils::{reverse_hex_string, to_nf_token_id_from_str},
};
use crate::{
    domain::entities::{CommitmentStatus, RequestStatus},
    driven::{
        db::mongo::CommitmentEntry,
        queue::{get_queue, QueuedRequest, TransactionRequest},
    },
    get_zkp_keys,
    ports::{commitments::Nullifiable, contracts::NightfallContract, db::RequestDB},
};
use crate::{
    domain::{
        entities::{
            DepositSecret, ERCAddress, HexConvertible, Operation, OperationType, Preimage, Salt,
            TokenType, Transport,
        },
        error::FailedClientOperation,
    },
    driven::contract_functions::contract_type_conversions::FrBn254,
    drivers::{
        blockchain::nightfall_event_listener::get_synchronisation_status, derive_key::ZKPKeys,
        rest::models::NF3RequestError,
    },
    get_fee_token_id,
    initialisation::get_db_connection,
    ports::{
        commitments::Commitment,
        db::CommitmentDB,
        db::CommitmentEntryDB,
        keys::KeySpending,
        proof::{Proof, ProvingEngine},
    },
    services::{
        client_operation::deposit_operation, commitment_selection::find_usable_commitments,
    },
};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine;
use ark_ff::{BigInteger256, Zero};
use ark_serialize::CanonicalDeserialize;
use ark_std::{rand::thread_rng, UniformRand};
use configuration::addresses::get_addresses;
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::wallets::LocalWsClient;
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::{BabyJubjub, Fr as BJJScalar};
use nightfall_bindings::{
    ierc1155::IERC1155, ierc20::IERC20, ierc3525::IERC3525, ierc721::IERC721, nightfall::Nightfall,
};
use serde::Serialize;
use uuid::Uuid;
use warp::{
    hyper::StatusCode,
    path, reject,
    reply::{self, json, Json, Reply, WithStatus},
    Filter,
};

#[derive(Serialize)]
struct WithdrawResponse {
    success: bool,
    message: String,
    withdraw_fund_salt: String, // Return the withdraw_fund_salt
}
// A simplified client interface, which provides Deposit, Transfer and Withdraw operations,
// with automated commitment selection, but without the flexibility of the lower-level
// client_operation API.
// It matches the API of NF_3 so it can be used with the NF_3 client, under the hood, it calls
// the client_operation handler

pub fn deposit_request<P, E, N>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    path!("v1" / "deposit")
        .and(warp::body::json())
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_deposit_request::<P, E, N>)
}

pub fn transfer_request<P, E, N>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    path!("v1" / "transfer")
        .and(warp::body::json())
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_transfer_request::<P, E, N>)
}

pub fn withdraw_request<P, E, N>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and(warp::header::optional::<String>("X-Request-ID"))
        .and_then(queue_withdraw_request::<P, E, N>)
}

/// function to queue the deposit requests
async fn queue_deposit_request<P, E, N>(
    deposit_req: NF3DepositRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let transaction_request = TransactionRequest::Deposit(deposit_req);
    queue_request::<P, E, N>(transaction_request, request_id).await
}

/// function to queue the transfer requests
async fn queue_transfer_request<P, E, N>(
    transfer_req: NF3TransferRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let transaction_request = TransactionRequest::Transfer(transfer_req);
    queue_request::<P, E, N>(transaction_request, request_id).await
}

/// function to queue the withdraw requests
async fn queue_withdraw_request<P, E, N>(
    withdraw_req: NF3WithdrawRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let transaction_request = TransactionRequest::Withdraw(withdraw_req);
    queue_request::<P, E, N>(transaction_request, request_id).await
}

/// function to queue the transfer requests. This function queues all types of transaction request
async fn queue_request<P, E, N>(
    transaction_request: TransactionRequest,
    request_id: Option<String>,
) -> Result<impl Reply, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    // extract the request ID
    let id = request_id.unwrap_or_default();
    // check if the id is a valid uuid
    if Uuid::parse_str(&id).is_err() {
        return Ok(reply::with_header(
            reply::with_status(
                json(&"Invalid request id".to_string()),
                StatusCode::BAD_REQUEST,
            ),
            "X-Request-ID",
            id,
        ));
    };

    // add the request to the queue
    let mut q = get_queue().await.write().await;
    q.push_back(QueuedRequest {
        transaction_request,
        uuid: id.clone(),
    });
    drop(q); // drop the lock so other processes can access the queue

    // record the request as queued
    let db = get_db_connection().await.write().await;
    if let None = db.store_request(&id, RequestStatus::Queued).await {
        return Ok(reply::with_header(
            reply::with_status(
                json(&"Database error or duplicate transaction".to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            "X-Request-ID",
            id,
        ));
    }
    drop(db); // drop the lock so other processes can access the DB

    // return a 202 Accepted response with the request ID
    Ok(reply::with_header(
        reply::with_status(
            json(&"Deposit request queued".to_string()),
            StatusCode::ACCEPTED,
        ),
        "X-Request-ID",
        id,
    ))
}

/// This function wraps the various transaction handlers, so that the queue can call the correct handler
/// based on the request type.
pub async fn handle_request<P, E, N>(
    request: TransactionRequest,
    request_id: &str,
) -> Result<warp::reply::WithHeader<WithStatus<Json>>, warp::Rejection>
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
) -> Result<warp::reply::WithHeader<WithStatus<Json>>, warp::Rejection> {
    let sync_state = get_synchronisation_status::<N>()
        .await
        .map_err(reject::custom)?
        .is_synchronised();
    if !sync_state {
        warn!("{id} Rejecting request - Client is not synchronised with the blockchain");
        return Ok(reply::with_header(
            reply::with_status(
                reply::json(&"Client is not synchronised with the blockchain"),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
            "X-Request-ID",
            id,
        ));
    }

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
        error!("{id} Could not convert ERC address {}", err);
        reject::custom(FailedClientOperation)
    })?;

    let token_id: BigInteger256 =
        BigInteger256::from_hex_string(token_id.as_str()).map_err(|_| {
            error!("{id} Could not convert hex string to BigInteger256");
            reject::custom(FailedClientOperation)
        })?;

    let token_type: TokenType = u8::from_str_radix(&token_type, 16)
        .map_err(|_| {
            error!("{id} Could not convert token type");
            reject::custom(FailedClientOperation)
        })?
        .into();

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|_| {
        error!("{id} Could not convert fee");
        reject::custom(FailedClientOperation)
    })?;

    let deposit_fee: Fr254 = Fr254::from_hex_string(deposit_fee.as_str()).map_err(|_| {
        error!("{id} Could not convert deposit fee");
        reject::custom(FailedClientOperation)
    })?;

    let value: Fr254 = Fr254::from_hex_string(value.as_str()).map_err(|err| {
        error!("{id} Could not wrangle value {}", err);
        reject::custom(FailedClientOperation)
    })?;
    let (secret_preimage_one, secret_preimage_two, secret_preimage_three) = if let (
        Some(p1),
        Some(p2),
        Some(p3),
    ) = (
        secret_preimage_one,
        secret_preimage_two,
        secret_preimage_three,
    ) {
        let secret_preimage_one: Fr254 = Fr254::from_hex_string(p1.as_str()).map_err(|err| {
            error!("{id} Could not wrangle secret preimage one {}", err);
            reject::custom(FailedClientOperation)
        })?;
        let secret_preimage_two: Fr254 = Fr254::from_hex_string(p2.as_str()).map_err(|err| {
            error!("{id} Could not wrangle secret preimage two {}", err);
            reject::custom(FailedClientOperation)
        })?;
        let secret_preimage_three: Fr254 = Fr254::from_hex_string(p3.as_str()).map_err(|err| {
            error!("{id} Could not wrangle secret preimage three {}", err);
            reject::custom(FailedClientOperation)
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
    let db: tokio::sync::RwLockWriteGuard<'_, mongodb::Client> =
        get_db_connection().await.write().await;
    // Then match on the token type and call the correct function
    let deposit_result = match token_type {
        TokenType::ERC20 => {
            deposit_operation::<IERC20<LocalWsClient>, Nightfall<LocalWsClient>>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                &id,
            )
            .await
        }
        TokenType::ERC721 => {
            deposit_operation::<IERC721<LocalWsClient>, Nightfall<LocalWsClient>>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                &id,
            )
            .await
        }
        TokenType::ERC1155 => {
            deposit_operation::<IERC1155<LocalWsClient>, Nightfall<LocalWsClient>>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                &id,
            )
            .await
        }
        TokenType::ERC3525 => {
            deposit_operation::<IERC3525<LocalWsClient>, Nightfall<LocalWsClient>>(
                erc_address,
                value,
                fee,
                deposit_fee,
                token_id,
                secret_preimage,
                token_type,
                &id,
            )
            .await
        }
    };

    let (preimage_value, preimage_fee_option) = match deposit_result {
        Ok(dr) => {
            debug!("{id} Deposit operation completed successfully");
            db.update_request(&id, RequestStatus::Submitted).await;
            dr
        }
        Err(e) => {
            error!("{id} Error with deposit operation: {}", e);
            db.update_request(&id, RequestStatus::Failed).await;
            return Err(reject::custom(FailedClientOperation));
        }
    };

    // Insert the preimage into the commitments DB as pending creation
    // TODO remove the blocknumber
    let ZKPKeys { nullifier_key, .. } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");
    let nullifier = preimage_value
        .nullifier_hash(&nullifier_key)
        .expect("Could not hash commitment {}");
    let commitment_hash = preimage_value.hash().expect("Could not hash commitment");
    let commitment_entry = CommitmentEntry::new(
        preimage_value,
        commitment_hash,
        nullifier,
        CommitmentStatus::PendingCreation,
    );

    db.store_commitment(commitment_entry).await.ok_or_else(|| {
        error!("{id} Error while storing pending deposit commitment");
        reject::custom(FailedClientOperation)
    })?;
    debug!("{id} Deposit commitment stored successfully");

    // Check if preimage_fee_option is Some, and store it in the DB if it exists
    if let Some(preimage_fee) = preimage_fee_option {
        let nullifier = preimage_fee
            .nullifier_hash(&nullifier_key)
            .expect("Could not hash commitment");
        let commitment_hash = preimage_fee.hash().expect("Could not hash commitment");

        let commitment_entry = CommitmentEntry::new(
            preimage_fee,
            commitment_hash,
            nullifier,
            CommitmentStatus::PendingCreation,
        );

        db.store_commitment(commitment_entry).await.ok_or_else(|| {
            error!("{id} Error while storing pending deposit_fee commitment");
            reject::custom(FailedClientOperation)
        })?;
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
    Ok(reply::with_header(
        reply::with_status(
            reply::json(&response_data), // Send the appropriate number of Preimages
            StatusCode::ACCEPTED,
        ),
        "X-Request-ID",
        id,
    ))
}

async fn handle_transfer<P, E, N>(
    transfer_req: NF3TransferRequest,
    id: &str,
) -> Result<warp::reply::WithHeader<WithStatus<Json>>, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let NF3TransferRequest {
        erc_address,
        token_id,
        recipient_data,
        fee,
        ..
    } = transfer_req;

    // add the id to the request database
    let db = get_db_connection().await.write().await;
    match db.store_request(&id, RequestStatus::Queued).await {
        Some(_) => {}
        None => {
            return Ok(reply::with_header(
                reply::with_status(
                    json(&"Database error".to_string()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
                "X-Request-ID",
                id,
            ))
        }
    }
    drop(db); // drop the lock so other processes can access the DB

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), reverse_hex_string(token_id.as_str()).as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {}",
                e
            );
            reject::custom(e)
        })?;
    let keys = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

    let value =
        Fr254::from_hex_string(recipient_data.values.first().unwrap().as_str()).map_err(|e| {
            error!("{id} Error when reading value: {}", e);
            reject::custom(NF3RequestError::ConversionError)
        })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|e| {
        error!("{id} Error when reading fee: {}", e);
        reject::custom(NF3RequestError::ConversionError)
    })?;

    let decoded_recipient_key = hex::decode(
        recipient_data
            .recipient_compressed_zkp_public_keys
            .first()
            .unwrap(),
    )
    .map_err(|e| {
        error!(
            "{id} Could not parse compressed recipient public key from String: {}",
            e
        );
        reject::custom(NF3RequestError::ConversionError)
    })?;

    let recipient_public_key = Affine::<BabyJubjub>::deserialize_compressed(
        decoded_recipient_key.as_slice(),
    )
    .map_err(|e| {
        error!("{id} Could not deserialize recipient public key: {}", e);
        reject::custom(NF3RequestError::CouldNotSerialisePublicKey)
    })?;

    let ephemeral_private_key = {
        let mut rng = ark_std::rand::thread_rng(); // TODO initialise in main and pass around as a rwlock
        BJJScalar::rand(&mut rng)
    };
    let shared_secret: Affine<BabyJubjub> = (recipient_public_key * ephemeral_private_key).into();

    // TODO: update APIs so that we allow passing in specific commitments.
    // For now we just use the commitment selection algorithm to minimise change.
    let spend_commitments;
    {
        let db = &mut get_db_connection().await.write().await;
        let fee_token_id = get_fee_token_id();
        let spend_value_commitments = find_usable_commitments(nf_token_id, value,db)
        .await.map_err(|e|{error!("{id} Could not find enough usable value commitments to complete this transfer, suggest depositing more tokens: {}", e); reject::custom(NF3RequestError::NoUsableCommitments)})?;
        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            find_usable_commitments(fee_token_id, fee, db)
            .await
            .map_err(|e| {
                error!(
                    "Could not find enough usable fee commitments to complete this transfer, suggest depositing more fee: {} {}",
                    e, id
                );
                reject::custom(NF3RequestError::NoUsableCommitments)
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

    // transferred value commitment, salt is the y-coordinate of the shared secret
    let new_commitment_one = Preimage::new(
        value,
        nf_token_id,
        spend_commitments[0].get_nf_slot_id(),
        recipient_public_key,
        Salt::Transfer(Fr254::new((shared_secret.y).into())),
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
        &id,
    )
    .await
}

async fn handle_withdraw<P, E, N>(
    withdraw_req: NF3WithdrawRequest,
    id: &str,
) -> Result<warp::reply::WithHeader<WithStatus<Json>>, warp::Rejection>
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
    let db = get_db_connection().await.write().await;
    match db.store_request(&id, RequestStatus::Queued).await {
        Some(_) => {}
        None => {
            return Ok(reply::with_header(
                reply::with_status(
                    json(&"Database error".to_string()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
                "X-Request-ID",
                id,
            ))
        }
    }
    drop(db); // drop the lock so other processes can access the DB

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), reverse_hex_string(token_id.as_str()).as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {}",
                e
            );
            reject::custom(e)
        })?;

    let keys = *get_zkp_keys().lock().expect("Poisoned Mutex lock");

    let value = Fr254::from_hex_string(value.as_str()).map_err(|e| {
        error!("{id} Error when reading value: {}", e);
        reject::custom(NF3RequestError::ConversionError)
    })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|e| {
        error!("{id} Error when reading fee: {}", e);
        reject::custom(NF3RequestError::ConversionError)
    })?;

    let recipient_address: Fr254 =
        Fr254::from_hex_string(recipient_address.as_str()).map_err(|e| {
            error!("{id} Error when reading recipeint address: {}", e);
            reject::custom(NF3RequestError::ConversionError)
        })?;
    // TODO: update APIs so that we allow passing in specific commitments.
    // For now we just use the commitment selection algorithm to minimise change.
    let spend_commitments;

    {
        let db = &mut get_db_connection().await.write().await;
        let fee_token_id = get_fee_token_id();
        let spend_value_commitments = find_usable_commitments(nf_token_id, value,db)
        .await.map_err(|e|{error!("{id} Could not find enough usable value commitments to complete this withdraw, suggest depositing more tokens: {}", e); reject::custom(NF3RequestError::NoUsableCommitments)})?;
        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            find_usable_commitments(fee_token_id, fee, db)
            .await
            .map_err(|e| {
                error!(
                    "{id} Could not find enough usable fee commitments to complete this withdraw, suggest depositing more fee: {}",
                    e
                );
                reject::custom(NF3RequestError::NoUsableCommitments)
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
        &id,
    )
    .await?;
    // Build the response
    let response = WithdrawResponse {
        success: true,
        message: "Withdraw operation completed successfully".to_string(),
        withdraw_fund_salt: withdraw_fund_salt.to_hex_string(),
    };
    // Return the response as JSON
    Ok(reply::with_header(
        reply::with_status(json(&response), StatusCode::ACCEPTED),
        "X-Request-ID",
        id,
    ))
}
