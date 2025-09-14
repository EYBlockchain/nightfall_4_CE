use crate::{
    domain::error::EventHandlerError,
    ports::{contracts::NightfallContract, events::EventHandler},
};
use alloy::rpc::types::Log;

// A simple function that passes the Event through to be handled by an implementation in the respository
// This will probably do more as the service develops, otherwise consider calling directly from driver to repository.
pub async fn process_events<N: NightfallContract>(
    e: impl EventHandler<N>,
    log: Log,
) -> Result<(), EventHandlerError> {
    ark_std::println!(
        "Processing event with transaction hash: {:?}",
        log.transaction_hash
    );
    e.handle_event(log.transaction_hash).await
}
