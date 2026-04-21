/// Module for querying block data
use warp::{
    path,
    reply::{self, Reply},
    Filter,
};

use crate::domain::error::ProposerRejection;
use crate::driven::nightfall_event::get_expected_layer2_blocknumber;

/// GET request for a specific commitment by key
pub fn get_block_data() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "blockdata")
        .and(warp::get())
        .and_then(handle_block_data)
}

async fn handle_block_data() -> Result<impl Reply, warp::Rejection> {
    let result: Result<u64, _> = (*get_expected_layer2_blocknumber().await.read().await).try_into();
    match result {
        Ok(block_number) => Ok(reply::json(&block_number)),
        Err(_) => Err(warp::reject::custom(
            ProposerRejection::BlockDataUnavailable,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driven::nightfall_event::get_expected_layer2_blocknumber;
    use alloy::primitives::{I256, U256};
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_blockdata_route_returns_current_block_number() {
        *get_expected_layer2_blocknumber().await.write().await =
            I256::try_from(U256::from(123u64)).unwrap();

        let filter = get_block_data();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/blockdata")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        let body = serde_json::from_slice::<u64>(res.body()).expect("body should be JSON number");
        assert_eq!(body, 123);
    }
}
