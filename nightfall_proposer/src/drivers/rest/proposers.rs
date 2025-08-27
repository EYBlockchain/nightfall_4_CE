use configuration::{addresses::get_addresses, settings::get_settings};
use ethers::types::U256;
use log::{info, warn};
use nightfall_bindings::round_robin::RoundRobin;
/// APIs for managing proposers
use warp::{hyper::StatusCode, path, reply::Reply, Filter};

use crate::{domain::error::ProposerRejection, initialisation::get_blockchain_client_connection};
use lib::blockchain_client::BlockchainClientConnection;

/// Get request for proposer rotation
pub fn rotate_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "rotate")
        .and(warp::get())
        .and_then(handle_rotate_proposer)
}

async fn handle_rotate_proposer() -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    match proposer_manager.proposer_count().call().await {
        Ok(count) => {
            if count <= U256::one() {
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
            tx.await.map_err(|_| ProposerRejection::ProviderError)?;
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
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    // add the proposer
    let tx = proposer_manager
        .add_proposer(url)
        .value(get_settings().nightfall_deployer.proposer_stake)
        .send()
        .await
        .map_err(|_e| {
            warn!("Failed to add proposer");
            ProposerRejection::FailedToAddProposer
        })?
        .await
        .map_err(|_| ProposerRejection::ProviderError)?;
    match tx {
        Some(transaction) => info!("Registered proposer with address: {:?}", transaction.from),
        None => {
            warn!("Failed to add proposer");
            return Err(warp::reject::custom(ProposerRejection::FailedToAddProposer));
        }
    }
    Ok(StatusCode::OK)
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
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    let signer_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .address();

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
            warn!("Could not check current proposer before removal: {e:?}",);
        }
    }

    // remove the proposer
    let tx = proposer_manager
        .remove_proposer()
        .send()
        .await
        .map_err(|_e| {
            warn!("Failed to remove proposer");
            ProposerRejection::FailedToRemoveProposer
        })?
        .await
        .map_err(|_| ProposerRejection::ProviderError)?;
    match tx {
        Some(transaction) => info!("Removed proposer with address: {:?}", transaction.from),
        None => {
            warn!("Failed to remove proposer");
            return Err(warp::reject::custom(
                ProposerRejection::FailedToRemoveProposer,
            ));
        }
    }
    Ok(StatusCode::OK)
}

// Withdraw a proposer's stake after a successful deregistration
pub fn withdraw() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and_then(handle_withdraw)
}

async fn handle_withdraw(amount: u64) -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    // attemp to withdraw the stake
    let tx = proposer_manager
        .withdraw(amount.into())
        .send()
        .await
        .map_err(|_e| {
            warn!("Failed to withdraw stake");
            ProposerRejection::FailedToWithdrawStake
        })?
        .await
        .map_err(|_| ProposerRejection::ProviderError)?;
    match tx {
        Some(transaction) => info!("Withdrew {} to address: {:?}", amount, transaction.from),
        None => {
            warn!("Failed to withdraw funds");
            return Err(warp::reject::custom(
                ProposerRejection::FailedToWithdrawStake,
            ));
        }
    }
    Ok(StatusCode::OK)
}
