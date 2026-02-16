
use warp::{hyper::StatusCode, path, reply, Filter, Reply};

use crate::{driven::db::mongo::CommitmentEntry};
use crate::initialisation::get_db_connection;
use crate::ports::db::CommitmentDB;
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, PrimeField, Zero, One};
use lib::{hex_conversion::HexConvertible, shared_entities::TokenType};

/// GET request for a specific commitment by key
pub fn get_commitment(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "commitment" / String)
        .and(warp::get())
        .and_then(handle_get_commitment)
}

pub async fn handle_get_commitment(key: String) -> Result<impl Reply, warp::Rejection> {
    let parsed_key = Fr254::from_hex_string(&key).map_err(|_| {
        warp::reject::custom(crate::domain::error::ClientRejection::InvalidCommitmentKey)
    })?;
    let commitment_db = get_db_connection().await;
    if let Some(res) = commitment_db.get_commitment(&parsed_key).await {
        Ok(reply::with_status(reply::json(&res), StatusCode::OK))
    } else {
        Err(warp::reject::custom(
            crate::domain::error::ClientRejection::CommitmentNotFound,
        ))
    }
}

/// GET request for all commitments
pub fn get_all_commitments(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "commitments")
        .and(warp::get())
        .and_then(handle_get_all_commitments)
}

pub async fn handle_get_all_commitments() -> Result<impl Reply, warp::Rejection> {
    let commitment_db = get_db_connection().await;
    let res = commitment_db
        .get_all_commitments()
        .await
        .map_err(|_| warp::reject::custom(crate::domain::error::ClientRejection::DatabaseError))?;
    let values: Vec<CommitmentEntry> = res.into_iter().map(|c| c.1).collect();
    Ok(reply::with_status(reply::json(&values), StatusCode::OK))
}

// get commitments by token_type
pub fn get_commitments_by_token_type(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "commitments" / "token_type" / String)
        .and(warp::get())
        .and_then(handle_get_commitments_by_token_type)
}

pub async fn handle_get_commitments_by_token_type(
    token_type: String,
) -> Result<impl Reply, warp::Rejection> {
    let commitment_db = get_db_connection().await;
    let res = commitment_db
        .get_commitments_by_token_type(&token_type)
        .await
        .map_err(|_| warp::reject::custom(crate::domain::error::ClientRejection::DatabaseError))?;
    let values: Vec<CommitmentEntry> = res.into_iter().map(|c| c.1).collect();
    Ok(reply::with_status(reply::json(&values), StatusCode::OK))
}

// get the maximum tranferable amount for a given token type, which is the sum of the two maximum value of the commitments of that token type

pub fn get_max_transferable_amount_by_token_type(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "commitments" / "max_transferable_amount" / String / String)
        .and(warp::get())
        .and_then(handle_get_max_transferable_amount_by_token_type)
}

pub async fn handle_get_max_transferable_amount_by_token_type(
    token_type: String,
    nf_token_id: String,
) -> Result<impl Reply, warp::Rejection> {
    let commitment_db = get_db_connection().await;
    let nf_token_id = Fr254::from_hex_string(&nf_token_id)
        .map_err(|_| warp::reject::custom(crate::domain::error::ClientRejection::InvalidTokenId))?;
    let res = commitment_db
        .get_commitments_by_token_type_and_nf_token_id(&token_type, nf_token_id)
        .await
        .map_err(|_| warp::reject::custom(crate::domain::error::ClientRejection::DatabaseError))?;
    let token_type: TokenType = u8::from_str_radix(&token_type, 16)
                                .map_err(|_| warp::reject::custom(crate::domain::error::ClientRejection::InvalidTokenType))?.into();

    let max_transferable_value = |entries: &[(Fr254, CommitmentEntry)]| -> Fr254 {
        let mut values = entries
            .iter()
            .map(|c| c.1.preimage.value)
            .collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        match values.len() {
            0 => Fr254::zero(),
            1 => values[0],
            _ => values[0] + values[1],
        }
    };

    match token_type {
        TokenType::ERC20 | TokenType::ERC1155 | TokenType::ERC3525 | TokenType::FeeToken => {
            // For fungible standards, the maximum transferable amount is the sum of the two highest commitments.
            let max_transferable = max_transferable_value(&res);
            Ok(reply::with_status(hex::encode(max_transferable.into_bigint().to_bytes_be()),
            StatusCode::OK,))
        }
        TokenType::ERC721 => {
            // For ERC-721, the maximum transferable amount is 1 if any commitment exists, otherwise 0.
            let max_transferable = if res.is_empty() { Fr254::zero() } else { Fr254::one() };
            Ok(reply::with_status(hex::encode(max_transferable.into_bigint().to_bytes_be()),
            StatusCode::OK,))
        }
  }
    
}
