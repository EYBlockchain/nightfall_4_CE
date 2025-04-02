//! This module contains the interface that a smart contract must work with to be classed as a Nightfall contract by a proposer.

use ethers::types::I256;
use nightfall_client::domain::error::NightfallContractError;

use crate::domain::entities::Block;

#[async_trait::async_trait]
pub trait NightfallContract {
    /// Proposes a block
    async fn propose_block(block: Block) -> Result<(), NightfallContractError>;

    /// Gets the current layer 2 block number from the blockchain
    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError>;
}
