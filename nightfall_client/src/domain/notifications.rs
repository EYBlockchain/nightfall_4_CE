use alloy::primitives::BlockNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum NotificationPayload {
    BlockchainEvent {
        l1_txn_hash: String,
        l2_block_number: BlockNumber,
        commitments: Vec<String>,
        request_ids: Vec<String>,
    },
    TransactionEvent {
        response: String,
        uuid: String,
    },
}
