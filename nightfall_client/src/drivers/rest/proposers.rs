use crate::domain::entities::Proposer;
use configuration::addresses::get_addresses;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use nightfall_bindings::artifacts::ProposerManager;
use warp::{path, reply, reply::Reply, Filter};

/// Get request for obtaining a list of proposers
pub fn get_proposers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "proposers")
        .and(warp::get())
        .and_then(handle_get_proposers)
}

async fn handle_get_proposers() -> Result<impl Reply, warp::Rejection> {
    // get a ManageProposers instance
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client(); // returns impl Provider or dyn Provider

    let proposer_manager =
        ProposerManager::new(get_addresses().round_robin, blockchain_client.root());
    // get the proposers
    let proposer_list =
        proposer_manager.get_proposers().call().await.map_err(|_| {
            warp::reject::custom(crate::domain::error::ClientRejection::ProposerError)
        })?;
    let list = proposer_list
        .into_iter()
        .map(|p| Proposer {
            stake: p.stake,
            addr: p.addr,
            url: p.url,
            next_addr: p.next_addr,
            previous_addr: p.previous_addr,
        })
        .collect::<Vec<Proposer>>();
    Ok(reply::json(&list))
}
