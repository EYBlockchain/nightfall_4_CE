use crate::{
    domain::{
        entities::CommitmentStatus,
        error::TransactionHandlerError,
        notifications::NotificationPayload,
    },
    driven::db::mongo::CommitmentEntry,
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestCommitmentMappingDB},
    },
    services::client_operation::deposit_operation,
};
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use ark_std::{rand::thread_rng, UniformRand};
use lib::{
    client_models::NF3DepositRequest,
    commitments::{Commitment, Nullifiable},
    derive_key::ZKPKeys,
    hex_conversion::HexConvertible,
    shared_entities::{DepositSecret, TokenType},
};
use tracing::{debug, error, info};
use nightfall_bindings::artifacts::{Nightfall, IERC1155, IERC20, IERC3525, IERC721};

use super::ERCAddress;

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
        TokenType::FeeToken => todo!(),
    }
    .map_err(TransactionHandlerError::DepositError)?;

    // Insert the preimage into the commitments DB as pending creation
    let ZKPKeys { nullifier_key, .. } = *get_zkp_keys().lock().expect("Poisoned Mutex lock");
    let nullifier = preimage_value
        .nullifier_hash(&nullifier_key)
        .map_err(|e| {
            error!("{id} Could not compute nullifier hash: {e}");
            TransactionHandlerError::CustomError(format!("Could not compute nullifier hash: {e}"))
        })?;
    let commitment_hash = preimage_value.hash().map_err(|e| {
        error!("{id} Could not hash commitment: {e}");
        TransactionHandlerError::CustomError(format!("Could not hash commitment: {e}"))
    })?;
    let commitment_entry = CommitmentEntry::new(
        preimage_value,
        nullifier,
        CommitmentStatus::PendingCreation,
        token_type,
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
            .map_err(|e| {
                error!("{id} Could not compute fee nullifier hash: {e}");
                TransactionHandlerError::CustomError(format!(
                    "Could not compute fee nullifier hash: {e}"
                ))
            })?;
        let commitment_hash = preimage_fee.hash().map_err(|e| {
            error!("{id} Could not hash fee commitment: {e}");
            TransactionHandlerError::CustomError(format!("Could not hash fee commitment: {e}"))
        })?;

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
            TokenType::FeeToken,
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
                .map_err(|e| {
                    error!("{id} Could not hash preimage value: {e}");
                    TransactionHandlerError::CustomError(format!(
                        "Could not hash preimage value: {e}"
                    ))
                })?
                .to_hex_string(),
            preimage_fee
                .hash()
                .map_err(|e| {
                    error!("{id} Could not hash preimage fee: {e}");
                    TransactionHandlerError::CustomError(format!(
                        "Could not hash preimage fee: {e}"
                    ))
                })?
                .to_hex_string(),
        ],
        None => vec![preimage_value
            .hash()
            .map_err(|e| {
                error!("{id} Could not hash preimage value: {e}");
                TransactionHandlerError::CustomError(format!(
                    "Could not hash preimage value: {e}"
                ))
            })?
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
