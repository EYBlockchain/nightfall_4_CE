use crate::domain::error::EventHandlerError;
use alloy::primitives::TxHash;

use super::contracts::NightfallContract;

// we need a transaction hash associated with the event so that we can find
// the transaction that triggered the event from the blockchain record
#[async_trait::async_trait]
pub trait EventHandler<N: NightfallContract> {
    async fn handle_event(&self, transaction_hash: Option<TxHash>)
        -> Result<(), EventHandlerError>;
}
