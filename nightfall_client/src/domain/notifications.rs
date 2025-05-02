use ethers::types::I256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum NotificationPayload {
    BlockchainEvent {
        l1_txn_hash: String,
        l2_block_number: I256,
        commitments: Vec<String>,
    },
    TransactionEvent(String)
}
