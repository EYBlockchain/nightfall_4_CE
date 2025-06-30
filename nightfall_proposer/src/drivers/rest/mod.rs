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
use warp::Filter;

pub mod block_assembly;
pub mod block_data;
pub mod client_transactions;
pub mod proposers;
pub mod synchronisation;

pub fn routes<P, E>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
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
}
