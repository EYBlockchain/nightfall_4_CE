use crate::drivers::rest::{
    block_data::get_block_data, client_transactions::client_transaction,
    proposers::rotate_proposer, synchronisation::synchronisation,
};
use block_assembly::{pause_block_assembly, resume_block_assembly};
use lib::validate_certificate::certification_validation_request;
use nightfall_client::{
    drivers::rest::health_check::health_route,
    ports::proof::{Proof, ProvingEngine},
};
use proposers::{add_proposer, remove_proposer, withdraw};
use warp::{
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

// nightfall_client/src/drivers/rest/health_check.rs
// use::nightfall_client::drivers::rest::health_check::health_route;

// nightfall_client/src/drivers/rest/balance.rs
        // .or(get_l1_balance())
// import get_l1_balance from nightfall_client::drivers::rest::balance
use nightfall_client::drivers::rest::balance::get_l1_balance;

use crate::domain::error::ProposerRejection;

pub mod block_assembly;
pub mod block_data;
pub mod client_transactions;
pub mod proposers;
pub mod synchronisation;

pub fn routes<P, E>() -> impl Filter<Extract = (impl warp::Reply,)> + Clone
where
    P: Proof,
    E: ProvingEngine<P> + Sync + Send + 'static,
{
    health_route()
        .or(client_transaction::<P, E>())
        .or(rotate_proposer())
        .or(get_block_data())
        .or(add_proposer())
        .or(remove_proposer())
        .or(withdraw())
        .or(certification_validation_request())
        .or(synchronisation())
        .or(pause_block_assembly())
        .or(resume_block_assembly())
        // .or(get_fee_balance())
        // nightfall_client/src/drivers/rest/balance.rs
        .or(get_l1_balance())
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if let Some(e) = err.find::<ProposerRejection>() {
        match e {
            ProposerRejection::BlockDataUnavailable => Ok(reply::with_status(
                "Block data unavailable",
                warp::http::StatusCode::SERVICE_UNAVAILABLE,
            )),
            ProposerRejection::ClientTransactionFailed => Ok(reply::with_status(
                "Client transaction failed",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToRotateProposer => Ok(reply::with_status(
                "Failed to rotate proposer",
                warp::http::StatusCode::LOCKED,
            )),
            ProposerRejection::FailedToAddProposer => Ok(reply::with_status(
                "Failed to add proposer",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToRemoveProposer => Ok(reply::with_status(
                "Failed to remove proposer",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToWithdrawStake => Ok(reply::with_status(
                "Failed to withdraw stake",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::ProviderError => Ok(reply::with_status(
                "Provider error",
                warp::http::StatusCode::SERVICE_UNAVAILABLE,
            )),
        }
    } else {
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
