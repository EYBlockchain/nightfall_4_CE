use crate::ports::{contracts::NightfallContract, events::EventHandler};
use alloy::rpc::types::Log;
use nightfall_client::{
    domain::error::EventHandlerError,
    ports::proof::{Proof, ProvingEngine},
};

// A simple function that passes the Event through to be handled by an implementation in the respository
// This will probably do more as the service develops, otherwise consider calling directly from driver to repository.
pub async fn process_events<P, E, N>(
    e: impl EventHandler<P, E, N>,
    log: Log,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    e.handle_event(log.transaction_hash.unwrap()).await
}
