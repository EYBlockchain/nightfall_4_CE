use crate::ports::{contracts::NightfallContract, proof::Proof};
use balance::{get_balance, get_fee_balance};
use lib::validate_certificate::certification_validation_request;
use log::error;
use proposers::get_proposers;
use reqwest::StatusCode;
use std::fmt::Debug;
use token_info::InvalidQuery;
use warp::{
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

use self::{
    client_nf_3::{deposit_request, transfer_request, withdraw_request},
    commitment::{get_all_commitments, get_commitment},
    health_check::health_route,
    keys::derive_key_mnemonic,
    request_status::{get_queue_length, get_request_status},
    synchronisation::synchronisation,
    token_info::get_token_info,
    withdraw::de_escrow,
};

mod balance;
pub mod client_nf_3;
pub mod client_operation;
mod commitment;
pub mod health_check;
mod keys;
pub mod models;
pub mod proposers;
mod request_status;
mod synchronisation;
mod token_info;
pub mod utils;
mod withdraw;

pub fn routes<P, N>() -> impl Filter<Extract = (impl warp::Reply,)> + Clone
where
    P: Proof + Debug + Send + serde::Serialize + Clone + Sync,
    N: NightfallContract,
{
    health_route()
        .or(deposit_request::<P>())
        .or(transfer_request::<P>())
        .or(withdraw_request::<P>())
        .or(get_commitment())
        .or(get_all_commitments())
        .or(derive_key_mnemonic())
        .or(get_proposers())
        .or(de_escrow())
        .or(certification_validation_request())
        .or(get_balance())
        .or(get_fee_balance())
        .or(synchronisation::<N>())
        .or(get_request_status())
        .or(get_queue_length())
        .or(get_token_info::<N>())
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<InvalidQuery>() {
        error!("Invalid query error: {:?}", e);
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else {
        error!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
