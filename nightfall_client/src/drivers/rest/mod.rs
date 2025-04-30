use crate::ports::{
    contracts::NightfallContract,
    proof::{Proof, ProvingEngine},
};
use balance::{get_balance, get_fee_balance};
use lib::validate_certificate::certification_validation_request;
use proposers::get_proposers;
use std::fmt::Debug;
use warp::Filter;

use self::{
    client_nf_3::{deposit_request, transfer_request, withdraw_request},
    commitment::{get_all_commitments, get_commitment},
    health_check::health_route,
    keys::derive_key_mnemonic,
    synchronisation::synchronisation,
    withdraw::de_escrow,
    request_status::get_request_status,
};

mod balance;
mod client_nf_3;
pub mod client_operation;
mod commitment;
pub mod health_check;
mod keys;
pub mod models;
pub mod proposers;
mod synchronisation;
pub mod utils;
mod withdraw;
mod request_status;

pub fn routes<P, E, N>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof + Debug + Send + serde::Serialize + Clone + Sync,
    E: ProvingEngine<P> + Send + Sync,
    N: NightfallContract,
{
    health_route()
        .or(deposit_request::<N>())
        .or(transfer_request::<P, E, N>())
        .or(withdraw_request::<P, E, N>())
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
}
