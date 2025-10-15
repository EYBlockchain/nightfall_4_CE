use crate::{domain::error::ProposerRejection, initialisation::get_blockchain_client_connection};
use alloy::primitives::U256;
use configuration::{addresses::get_addresses, settings::get_settings};
use lib::{blockchain_client::BlockchainClientConnection, error::ProposerError};
use log::{info, warn};
use nightfall_bindings::artifacts::RoundRobin;
/// APIs for managing proposers
use warp::{hyper::StatusCode, path, reply::Reply, Filter};

/// Get request for proposer rotation
pub fn rotate_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "rotate")
        .and(warp::get())
        .and_then(handle_rotate_proposer)
}

async fn handle_rotate_proposer() -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let proposer_manager = RoundRobin::new(get_addresses().round_robin, blockchain_client.root());
    match proposer_manager.proposer_count().call().await {
        Ok(count) => {
            if count <= U256::ONE {
                warn!("Rotation requested, but only one active proposer; rotation will have no effect.");
            }
        }
        Err(_e) => {
            warn!("Failed to fetch proposer count before rotation");
        }
    }
    // rotate the proposer
    let tx_call = proposer_manager.rotate_proposer();
    let tx_result = tx_call.send().await;
    match tx_result {
        Ok(tx) => {
            tx.get_receipt().await.map_err(|e| {
                warn!("Failed to get transaction receipt: {e}");
                ProposerError::ProviderError(e.to_string())
            })?;
            Ok(StatusCode::OK)
        }
        Err(_e) => Err(warp::reject::custom(
            ProposerRejection::FailedToRotateProposer,
        )),
    }
}

// Add a proposer
pub fn add_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "register")
        .and(warp::body::json())
        .and_then(handle_add_proposer)
}

async fn handle_add_proposer(url: String) -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let read_connection = get_blockchain_client_connection().await.read().await;
    let blockchain_client = read_connection.get_client();
    let caller = read_connection.get_address();
    let signer = read_connection.get_signer();
    let client = blockchain_client.root();
    let proposer_manager = RoundRobin::new(get_addresses().round_robin, client.clone());

    let nonce = blockchain_client
        .get_transaction_count(caller)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?;
    let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
        warn!("{e}");
        ProposerRejection::FailedToAddProposer
    })?;
    let max_fee_per_gas = gas_price * 2;
    let max_priority_fee_per_gas = gas_price;
    let gas_limit = 5000000u64;

    let raw_tx = proposer_manager
        .add_proposer(url)
        .value(U256::from(get_settings().nightfall_deployer.proposer_stake))
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        .build_raw_transaction(signer)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?;
    // add the proposer
    let tx = blockchain_client
        .send_raw_transaction(&raw_tx)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            warn!("Failed to get transaction receipt: {e}");
            ProposerError::ProviderError(e.to_string())
        })?;
    if tx.status() {
        info!("Registered proposer with address: {:?}", tx.from);
        Ok(StatusCode::OK)
    } else {
        warn!("Failed to add proposer with address: {:?}", tx.from);
        Err(warp::reject::custom(ProposerRejection::FailedToAddProposer))
    }
}

// remove a proposer
pub fn remove_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "deregister")
        .and(warp::get())
        .and_then(handle_remove_proposer)
}

async fn handle_remove_proposer() -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let read_connection = get_blockchain_client_connection().await.read().await;
    let blockchain_client = read_connection.get_client();
    let signer_address = read_connection.get_address();
    let signer = read_connection.get_signer();
    let client = blockchain_client.root();
    let proposer_manager = RoundRobin::new(get_addresses().round_robin, client.clone());

    // Read penalty + cooling config from settings
    let settings = get_settings();
    let penalty = settings.nightfall_deployer.proposer_exit_penalty;
    let cooling_blocks = settings.nightfall_deployer.proposer_cooling_blocks;

    // Fetch the current proposer address on-chain
    match proposer_manager.get_current_proposer_address().call().await {
        Ok(current_proposer) => {
            if current_proposer == signer_address {
                warn!(
                    "You are removing yourself as the active proposer — this will deduct an exit penalty of {penalty} units and start a cooldown period of {cooling_blocks} L1 blocks before you can re-register."
                );
            } else {
                info!("You are removing yourself, but you are not the active proposer — no penalty will be applied.");
            }
        }
        Err(e) => {
            warn!("Could not check current proposer before removal: {e:?}");
        }
    }

    let nonce = blockchain_client
        .get_transaction_count(signer_address)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToRemoveProposer
        })?;
    let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
        warn!("{e}");
        ProposerRejection::FailedToRemoveProposer
    })?;
    let max_fee_per_gas = gas_price * 2;
    let max_priority_fee_per_gas = gas_price;
    let gas_limit = 5000000u64;

    let raw_tx = proposer_manager
        .remove_proposer()
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        .build_raw_transaction(signer)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToRemoveProposer
        })?;
    // add the proposer
    let tx = blockchain_client
        .send_raw_transaction(&raw_tx)
        .await
        .map_err(|_e| {
            warn!("Failed to remove proposer");
            ProposerRejection::FailedToRemoveProposer
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            warn!("Failed to get transaction receipt: {e}");
            ProposerError::ProviderError(e.to_string())
        })?;
    if tx.status() {
        info!("Removed proposer with address: {:?}", tx.from);
        Ok(StatusCode::OK)
    } else {
        warn!("Failed to remove proposer");
        Err(warp::reject::custom(
            ProposerRejection::FailedToRemoveProposer,
        ))
    }
}

// Withdraw a proposer's stake after a successful deregistration
pub fn withdraw() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and_then(handle_withdraw)
}

async fn handle_withdraw(amount: u64) -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let read_connection = get_blockchain_client_connection().await.read().await;
    let blockchain_client = read_connection.get_client();
    let caller = read_connection.get_address();
    let signer = read_connection.get_signer();
    let proposer_manager = RoundRobin::new(get_addresses().round_robin, blockchain_client.root());
    // attemp to withdraw the stake
    let nonce = blockchain_client
        .get_transaction_count(caller)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?;
    let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
        warn!("{e}");
        ProposerRejection::FailedToWithdrawStake
    })?;
    let max_fee_per_gas = gas_price * 2;
    let max_priority_fee_per_gas = gas_price;
    let gas_limit = 5000000u64;

    let raw_tx = proposer_manager
        .withdraw(U256::from(amount))
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        .build_raw_transaction(signer)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?;
    // add the proposer
    let tx = blockchain_client
        .send_raw_transaction(&raw_tx)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            warn!("Failed to get transaction receipt: {e}");
            ProposerError::ProviderError(e.to_string())
        })?;
    if tx.status() {
        info!("Withdrew {} to address: {:?}", amount, tx.from);
        Ok(StatusCode::OK)
    } else {
        warn!("Failed to withdraw funds");
        Err(warp::reject::custom(
            ProposerRejection::FailedToWithdrawStake,
        ))
    }
}
