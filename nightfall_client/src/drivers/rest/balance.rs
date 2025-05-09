use super::utils::{reverse_hex_string, to_nf_token_id_from_str};
use crate::{get_fee_token_id, initialisation::get_db_connection, ports::db::CommitmentDB};
use ark_ff::{BigInteger, PrimeField};
use warp::{http::StatusCode, path, reply::Reply, Filter};
// use super::utils::to_nf_token_id_2;
/// Endpoint to get a token balance
/// NB for consistency with the rest of the API,
/// the value is returned as a *hex* string.
pub fn get_balance() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    path!("v1" / "balance" / String / String)
        .and(warp::get())
        .and_then(handle_get_balance)
}

pub async fn handle_get_balance(
    erc_address: String,
    token_id: String,
) -> Result<impl Reply, warp::Rejection> {
    // need to make the token ID little endian because that's what the to_nf_token_id_from_str function expects
    let nf_token_id = to_nf_token_id_from_str(&erc_address, &reverse_hex_string(&token_id));
    // see if we can decode the input
    if let Ok(nf_token_id) = nf_token_id {
        // search the commitment db for a preimage with the correct nf_token_id
        let db = get_db_connection().await.read().await;
        // get the balance
        let balance = db.get_balance(&nf_token_id).await;
        if let Some(balance) = balance {
            Ok(warp::reply::with_status(
                hex::encode(balance.into_bigint().to_bytes_be()),
                StatusCode::OK,
            ))
        } else {
            // if we don't find a balance, return an 404 error
            Ok(warp::reply::with_status(
                "No such token".to_string(),
                StatusCode::NOT_FOUND,
            ))
        }
    } else {
        Ok(warp::reply::with_status(
            "Invalid token id".to_string(),
            StatusCode::BAD_REQUEST,
        ))
    }
}

/// Endpoint to get a fee balance
/// the value is returned as a *hex* string.
pub fn get_fee_balance(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "fee_balance")
        .and(warp::get())
        .and_then(handle_get_fee_balance)
}

pub async fn handle_get_fee_balance() -> Result<impl Reply, warp::Rejection> {
    let fee_token_id = get_fee_token_id();
    // search the commitment db for a preimage with the correct nf_token_id
    let db = get_db_connection().await.read().await;
    // get the balance
    let balance = db.get_balance(&fee_token_id).await;
    if let Some(balance) = balance {
        Ok(warp::reply::with_status(
            hex::encode(balance.into_bigint().to_bytes_be()),
            StatusCode::OK,
        ))
    } else {
        // if we don't find a balance, return an 404 error
        Ok(warp::reply::with_status(
            "No such token".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
