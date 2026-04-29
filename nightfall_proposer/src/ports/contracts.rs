//! This module contains the interface that a smart contract must work with to be classed as a Nightfall contract by a proposer.

use alloy::primitives::{I256, TxHash};
use lib::error::NightfallContractError;

use crate::domain::entities::Block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotBroadcastReason {
    TransactionPreparationFailed,
    SendRawTransactionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BroadcastUnknownReason {
    ReceiptUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposeBlockOutcome {
    Submitted {
        tx_hash: TxHash,
    },
    NotBroadcast {
        reason: NotBroadcastReason,
    },
    BroadcastUnknown {
        tx_hash: TxHash,
        reason: BroadcastUnknownReason,
    },
    Reverted {
        tx_hash: TxHash,
    },
}

#[async_trait::async_trait]
pub trait NightfallContract {
    /// Proposes a block
    async fn propose_block(block: Block) -> Result<ProposeBlockOutcome, NightfallContractError>;

    /// Gets the current layer 2 block number from the blockchain
    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError>;
}
