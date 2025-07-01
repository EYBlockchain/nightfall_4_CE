use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

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

impl<E: Display + Debug> Error for MerkleTreeError<E> {}

impl<E: Display> Display for MerkleTreeError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TreeIsFull => write!(f, "The tree is full"),
            Self::IncorrectBatchSize => write!(f, "Incorrect batch size"),
            Self::NoLeaves => write!(f, "No leaves"),
            Self::DatabaseError(e) => write!(f, "Database error {}", e),
            Self::TreeNotFound => write!(f, "Tree not found"),
            Self::TreeAlreadyExists => write!(f, "Tree already exists"),
            Self::SerializationError => write!(f, "Serialization error "),
            Self::InvalidProof => write!(f, "Invalid proof"),
        }
    }
}

#[derive(Debug)]
pub enum ProposerRejection {
    BlockDataUnavailable,
    ClientTransactionFailed,
    FailedToRotateProposer,
    FailedToAddProposer,
    FailedToRemoveProposer,
    FailedToWithdrawStake,
    ProviderError,
}

impl std::fmt::Display for ProposerRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposerRejection::BlockDataUnavailable => write!(f, "Block data unavailable"),
            ProposerRejection::ClientTransactionFailed => write!(f, "Client transaction failed"),
            ProposerRejection::FailedToRotateProposer => write!(f, "Failed to rotate proposer"),
            ProposerRejection::FailedToAddProposer => write!(f, "Failed to add proposer"),
            ProposerRejection::FailedToRemoveProposer => write!(f, "Failed to remove proposer"),
            ProposerRejection::FailedToWithdrawStake => write!(f, "Failed to withdraw stake"),
            ProposerRejection::ProviderError => write!(f, "Provider error"),
        }
    }
}

impl std::error::Error for ProposerRejection {}

impl warp::reject::Reject for ProposerRejection {}
