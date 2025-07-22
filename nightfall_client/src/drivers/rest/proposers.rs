use configuration::addresses::get_addresses;
use warp::reply;
use warp::{path, reply::Reply, Filter};

use crate::domain::entities::Proposer;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use nightfall_bindings::bindings::RoundRobin;
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
    let blcokchain_client = get_blockchain_client_connection()
    .await
    .read()
    .await
    .get_client();
    let proposer_manager = RoundRobin::new(
        get_addresses().round_robin,
        blcokchain_client.root() 
    );
    // get the proposers
    let proposer_list = proposer_manager.get_proposers()
        .call()
        .await
        .map_err(|_| ProposerError::FailedToGetProposers)?._0;
    let list  = proposer_list
        .into_iter()
        .map(|p| Proposer {
           stake: p.stake,
           addr: p.addr,
           url: p.url,
           next_addr: p.next_addr,
           previous_addr: p.previous_addr
        })
        .collect::<Vec<Proposer>>();
    Ok(reply::json(&list))
}
