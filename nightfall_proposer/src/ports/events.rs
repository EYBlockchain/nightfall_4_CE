use ethers::types::TxHash;
use nightfall_client::{
    domain::error::EventHandlerError,
    ports::proof::{Proof, ProvingEngine},
};

use super::contracts::NightfallContract;

// we can't reuse client's version because we want to implement on a foreign type
#[async_trait::async_trait]
pub trait EventHandler<P, E, N>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    async fn handle_event(&self, transaction_hash: TxHash) -> Result<(), EventHandlerError>;
}
