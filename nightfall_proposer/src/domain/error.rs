use std::fmt::{Debug, Display, Formatter};

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
pub enum ProposerRejection {
    #[error("Block data unavailable")]
    BlockDataUnavailable,
    #[error("Client transaction failed")]
    ClientTransactionFailed,
    #[error("Failed to rotate proposer")]
    FailedToRotateProposer,
    #[error("Failed to add proposer")]
    FailedToAddProposer,
    #[error("Failed to remove proposer")]
    FailedToRemoveProposer,
    #[error("Failed to withdraw stake")]
    FailedToWithdrawStake,
    #[error("Provider error")]
    ProviderError,
}

impl warp::reject::Reject for ProposerRejection {}
