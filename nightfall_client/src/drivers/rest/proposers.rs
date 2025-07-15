use configuration::addresses::get_addresses;
use alloy::sol;
use warp::reply;
use warp::{path, reply::Reply, Filter};

use crate::domain::entities::Proposer;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};

sol!(
    #[sol(rpc)]     // Add Debug trait to x509CheckReturn
    RoundRobin, "/Users/Swati.Rawal/nightfall_4_PV/blockchain_assets/artifacts/RoundRobin.sol/RoundRobin.json");
/// Error type for proposer rotation
#[derive(Debug)]
pub enum ProposerError {
    FailedToGetProposers,
    ProviderError(String),
}

impl std::fmt::Display for ProposerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposerError::FailedToGetProposers => {
                write!(f, "Failed to get list of Proposers")
            }
            ProposerError::ProviderError(_) => {
                write!(f, "Provider error")
            }
        }
    }
}

impl std::error::Error for ProposerError {}

impl warp::reject::Reject for ProposerError {}

/// Get request for obtaining a list of proposers
pub fn get_proposers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "proposers")
        .and(warp::get())
        .and_then(handle_get_proposers)
}

async fn handle_get_proposers() -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client()  
    );
    // get the proposers
    let proposer_list = proposer_manager.get_proposers()
        .call()
        .await
        .map_err(|_| ProposerError::FailedToGetProposers)?._0;
    let list = proposer_list
        .into_iter()
        .map(Proposer::from)
        .collect::<Vec<RoundRobin::Proposer>>();
    Ok(reply::json(&list))
}
