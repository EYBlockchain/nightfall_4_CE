use std::fmt::{Debug, Display, Formatter};

use jf_primitives::poseidon::PoseidonError;
use lib::error::ConversionError;
use lib::error::{BlockchainClientConnectionError, EventHandlerError, NightfallContractError};

#[derive(Debug, thiserror::Error)]
#[error("Failed to perform client operation")]
pub struct FailedClientOperation;

impl warp::reject::Reject for FailedClientOperation {}

/// errors for a merkle tree
#[derive(Debug)]
pub enum MerkleTreeError<E> {
    /// The tree is full
    TreeIsFull,
    IncorrectBatchSize,
    NoLeaves,
    DatabaseError(E),
    TreeNotFound,
    TreeAlreadyExists,
    SerializationError,
    InvalidProof,
}

impl<E: Display + Debug> std::error::Error for MerkleTreeError<E> {}

impl<E: Display> Display for MerkleTreeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TreeIsFull => write!(f, "The tree is full"),
            Self::IncorrectBatchSize => write!(f, "Incorrect batch size"),
            Self::NoLeaves => write!(f, "No leaves"),
            Self::DatabaseError(e) => write!(f, "Database error {e}"),
            Self::TreeNotFound => write!(f, "Tree not found"),
            Self::TreeAlreadyExists => write!(f, "Tree already exists"),
            Self::SerializationError => write!(f, "Serialization error "),
            Self::InvalidProof => write!(f, "Invalid proof"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
/// Error type used by the handler that processes deposit, transfer and withdraw transactions
pub enum TransactionHandlerError {
    #[error("Json conversion error: {0}")]
    JsonConversionError(serde_json::Error),
    #[error("Deposit error: {0}")]
    DepositError(#[from] DepositError),
    #[error("Database error")]
    DatabaseError,
    #[error("Transaction error: {0}")]
    CustomError(String),
    #[error("Transaction error")]
    Error,
    #[error("Client not synchronized")]
    ClientNotSynchronized,
}

/// Error type for handling calls to a token contract
#[derive(Debug, thiserror::Error)]
pub enum TokenContractError {
    #[error("Token Contract Error: Blockchain Client Connection Error: {0}")]
    BlockchainClientConnectionError(#[from] BlockchainClientConnectionError),
    #[error("Token Contract Error: Error while converting to Solidity type: {0}")]
    ConversionError(#[from] ConversionError),
    #[error("Did not receive a transaction receipt")]
    TransactionError,
    #[error("Token Type Error: {0}")]
    TokenTypeError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DepositError {
    #[error("Deposit Error: {0}")]
    TokenError(#[from] TokenContractError),
    #[error("Deposit Error: {0}")]
    NightfallError(#[from] NightfallContractError),
    #[error("Deposit Error: {0}")]
    PoseidonError(#[from] PoseidonError),
}

#[derive(Debug, thiserror::Error)]
#[error("Could not sync {0}")]
pub struct SyncingError(pub EventHandlerError);

/// Custom rejection type for REST API errors
#[derive(Debug, thiserror::Error)]
pub enum ClientRejection {
    #[error("No such token found")]
    NoSuchToken,
    #[error("Invalid token id")]
    InvalidTokenId,
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("Invalid request id")]
    InvalidRequestId,
    #[error("Queue is full")]
    QueueFull,
    #[error("Database error or duplicate transaction")]
    DatabaseError,
    #[error("Invalid commitment key")]
    InvalidCommitmentKey,
    #[error("Commitment not found")]
    CommitmentNotFound,
    #[error("Failed to get list of Proposers")]
    ProposerError,
    #[error("No such request")]
    RequestNotFound,
    #[error("Failed to de-escrow funds")]
    FailedDeEscrow,
    #[error("Synchronisation service unavailable")]
    SynchronisationUnavailable,
}

impl warp::reject::Reject for ClientRejection {}
