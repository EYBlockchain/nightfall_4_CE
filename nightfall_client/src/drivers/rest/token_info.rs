use ark_bn254::Fr as Fr254;
use reqwest::StatusCode;
use warp::{
    path, reject,
    reply::{self, Reply},
    Filter,
};
use crate::domain::entities::HexConvertible;
use crate::ports::contracts::NightfallContract;

#[derive(Debug)]
pub struct InvalidQuery;
impl reject::Reject for InvalidQuery {}
#[derive(Debug)]
pub struct NotFound;
impl reject::Reject for NotFound {}

/// GET request for a getting information about a token if you happen to know the nightfall token id
pub fn get_token_info<N: NightfallContract>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "token" / String)
        .and(warp::get())
        .and_then(handle_get_token_info::<N>)
}

/// Handler for the GET request to retrieve token information
async fn handle_get_token_info<N: NightfallContract>(
    nf_token_id: String,
) -> Result<impl Reply, warp::Rejection> {
    let nf_token_id = Fr254::from_hex_string(&nf_token_id)
        .map_err(|_| reject::custom(InvalidQuery))?;
    dbg!(nf_token_id);
    let token_info = N::get_token_info(nf_token_id)
        .await
        .map_err(|_| reject::custom(NotFound))?;
    dbg!(&token_info);
    Ok(reply::with_status(reply::json(&token_info), StatusCode::OK))
}
