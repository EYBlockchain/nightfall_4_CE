use crate::{
    domain::error::EventHandlerError,
    ports::{contracts::NightfallContract, events::EventHandler},
};
use ethers::contract::LogMeta;

// A simple function that passes the Event through to be handled by an implementation in the respository
// This will probably do more as the service develops, otherwise consider calling directly from driver to repository.
pub async fn process_events<N: NightfallContract>(
    e: impl EventHandler<N>,
    log: LogMeta,
) -> Result<(), EventHandlerError> {
    e.handle_event(log.transaction_hash).await
}
