/// Module for querying block data
use warp::{
    path,
    reply::{self, Reply},
    Filter,
};

use crate::driven::nightfall_event::get_expected_layer2_blocknumber;
use crate::domain::error::ProposerRejection;

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
        Err(_) => Err(warp::reject::custom(ProposerRejection::BlockDataUnavailable)),
    }
}
