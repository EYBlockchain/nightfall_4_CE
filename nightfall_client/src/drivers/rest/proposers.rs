use crate::domain::entities::Proposer;
use configuration::addresses::get_addresses;
use futures::Future;
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
    handle_get_proposers_with(fetch_proposers).await
}

async fn handle_get_proposers_with<F, Fut>(fetch: F) -> Result<impl Reply, warp::Rejection>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<Proposer>, warp::Rejection>>,
{
    let list = fetch().await?;
    Ok(reply::json(&list))
}

async fn fetch_proposers() -> Result<Vec<Proposer>, warp::Rejection> {
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
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, U256};
    use serde_json::Value;
    use warp::{http::StatusCode, Filter};

    fn sample_proposers() -> Vec<Proposer> {
        vec![Proposer {
            stake: U256::from(42u64),
            addr: Address::from([0x11; 20]),
            url: "http://proposer-1.example".to_string(),
            next_addr: Address::from([0x22; 20]),
            previous_addr: Address::from([0x33; 20]),
        }]
    }

    #[tokio::test]
    async fn test_proposers_route_returns_ok_with_json_array() {
        let filter = path!("v1" / "proposers")
            .and(warp::get())
            .and_then(|| handle_get_proposers_with(|| async { Ok(sample_proposers()) }));

        let res = warp::test::request()
            .method("GET")
            .path("/v1/proposers")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert!(body.is_array(), "expected JSON array");
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["url"], "http://proposer-1.example");
    }

    #[tokio::test]
    async fn test_proposers_route_returns_service_unavailable_on_provider_error() {
        let filter = path!("v1" / "proposers")
            .and(warp::get())
            .and_then(|| async {
                handle_get_proposers_with(|| async {
                    Err(warp::reject::custom(
                        crate::domain::error::ClientRejection::ProposerError,
                    ))
                })
                .await
            })
            .recover(super::super::handle_rejection);

        let res = warp::test::request()
            .method("GET")
            .path("/v1/proposers")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Failed to get list of Proposers"
        );
    }
}
