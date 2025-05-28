use log::trace;
use warp::{hyper::StatusCode, path, reject, reply, Filter, Reply};

use crate::domain::entities::HexConvertible;
use crate::driven::db::mongo::CommitmentEntry;
use crate::initialisation::get_db_connection;
use crate::ports::db::CommitmentDB;
use ark_bn254::Fr as Fr254;
use std::fmt::Debug;

#[derive(Debug)]
pub enum CommitmentError {
    KeyParsingError,
    DatabaseError(()),
}

impl reject::Reject for CommitmentError {}

/// GET request for a specific commitment by key
pub fn get_commitment(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "commitment" / String)
        .and(warp::get())
        .and_then(handle_get_commitment)
}

pub async fn handle_get_commitment(key: String) -> Result<impl Reply, warp::Rejection> {
    let parsed_key = Fr254::from_hex_string(&key)
        .map_err(|_| reject::custom(CommitmentError::KeyParsingError))?;
    let commitment_db = get_db_connection().await;
    trace!("Looking up commitment in DB, with key {}", &key);
    if let Some(res) = commitment_db.get_commitment(&parsed_key).await {
        Ok(reply::with_status(reply::json(&res), StatusCode::OK))
    } else {
        let resp = "Commitment not found";
        Ok(reply::with_status(
            reply::json(&resp),
            StatusCode::NOT_FOUND,
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
        .map_err(|_| reject::custom(CommitmentError::DatabaseError(())))?;
    let values: Vec<CommitmentEntry> = res.into_iter().map(|c| c.1).collect();
    Ok(reply::with_status(reply::json(&values), StatusCode::OK))
}
