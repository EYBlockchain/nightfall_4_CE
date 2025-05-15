use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use ark_serialize::SerializationError;
use ethers::providers::ProviderError;

use jf_primitives::poseidon::PoseidonError;
use lib::error::BlockchainClientConnectionError;
use warp::reject::{self, Reject};

#[derive(Debug)]
pub struct FailedClientOperation;

impl Error for FailedClientOperation {}

impl std::fmt::Display for FailedClientOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to perform client operation")
    }
}

impl reject::Reject for FailedClientOperation {}

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

/// Error type used by the Event Listener, that listens for blockchain events and processes them.
#[derive(Debug)]
pub enum EventHandlerError {
    NoEventStream,
    StreamTerminated,
    InvalidCalldata,
    IOError(String),
    MissingBlocks(usize),
    HashError,
}

impl Display for EventHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EventHandlerError::NoEventStream => write!(f, "Could not connect to event stream"),
            EventHandlerError::StreamTerminated => write!(f, "Event stream terminated"),
            EventHandlerError::InvalidCalldata => write!(f, "Invalid calldata"),
            EventHandlerError::IOError(s) => write!(f, "IO Error: {}", s),
            EventHandlerError::MissingBlocks(n) => {
                write!(f, "Missing layer 2 blocks. Last processed was: {}", n)
            }
            EventHandlerError::HashError => write!(f, "Hashing error"),
        }
    }
}

impl Error for EventHandlerError {}
impl Reject for EventHandlerError {}

#[derive(Debug)]
/// Error type used by the handler that processes deposit, transfer and withdraw transactions
pub enum TransactionHandlerError {
    JsonConversionError(serde_json::Error),
    DepositError(DepositError),
    DatabaseError,
    CustomError(String),
    Error,
    ClientNotSynchronized,
}

impl Display for TransactionHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionHandlerError::JsonConversionError(e) => {
                write!(f, "Json conversion error: {}", e)
            }
            TransactionHandlerError::DepositError(e) => write!(f, "Deposit error: {}", e),
            TransactionHandlerError::DatabaseError => write!(f, "Database error"),
            TransactionHandlerError::CustomError(s) => write!(f, "Transaction error: {}", s),
            TransactionHandlerError::Error => write!(f, "Transaction error"),
            TransactionHandlerError::ClientNotSynchronized => write!(f, "Client not synchronized"),
        }
    }
}

impl Error for TransactionHandlerError {}

/// Error type for handling calls to a token contract
#[derive(Debug)]
pub enum TokenContractError {
    BlockchainClientConnectionError(BlockchainClientConnectionError),
    ConversionError(ConversionError),
    TransactionError,
    TokenTypeError(String),
}

impl Display for TokenContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenContractError::BlockchainClientConnectionError(e) => write!(
                f,
                "Token Contract Error: Blockchain Client Connection Error: {}",
                e
            ),
            TokenContractError::ConversionError(e) => write!(
                f,
                "Token Contract Error: Error while converting to Solidity type: {}",
                e
            ),
            TokenContractError::TransactionError => {
                write!(f, "Did not receive a transaction receipt")
            }
            TokenContractError::TokenTypeError(s) => write!(f, "Token Type Error: {}", s),
        }
    }
}

impl Error for TokenContractError {}

impl From<BlockchainClientConnectionError> for TokenContractError {
    fn from(e: BlockchainClientConnectionError) -> Self {
        Self::BlockchainClientConnectionError(e)
    }
}

impl From<ConversionError> for TokenContractError {
    fn from(e: ConversionError) -> Self {
        Self::ConversionError(e)
    }
}

impl From<ProviderError> for TokenContractError {
    fn from(e: ProviderError) -> Self {
        Self::from(BlockchainClientConnectionError::from(e))
    }
}

/// Error type for handling calls to a token contract
#[derive(Debug)]
pub enum NightfallContractError {
    BlockchainClientConnectionError(BlockchainClientConnectionError),
    ConversionError(ConversionError),
    TransactionError,
    EscrowError(String),
    PoseidonError(PoseidonError),
}

impl Display for NightfallContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NightfallContractError::BlockchainClientConnectionError(e) => write!(
                f,
                "Nightfall Contract Error: Blockchain Client Connection Error: {}",
                e
            ),
            NightfallContractError::ConversionError(e) => write!(
                f,
                "Nightfall Contract Error: Error while converting to Solidity type: {}",
                e
            ),
            NightfallContractError::TransactionError => {
                write!(f, "Did not receive a transaction receipt")
            }
            NightfallContractError::EscrowError(s) => write!(f, "Escrow Funds Error: {}", s),
            NightfallContractError::PoseidonError(e) => write!(f, "Hashing Error: {}", e),
        }
    }
}

impl Error for NightfallContractError {}

impl From<BlockchainClientConnectionError> for NightfallContractError {
    fn from(e: BlockchainClientConnectionError) -> Self {
        Self::BlockchainClientConnectionError(e)
    }
}

impl From<ConversionError> for NightfallContractError {
    fn from(e: ConversionError) -> Self {
        Self::ConversionError(e)
    }
}

impl From<ProviderError> for NightfallContractError {
    fn from(e: ProviderError) -> Self {
        Self::from(BlockchainClientConnectionError::from(e))
    }
}

impl From<PoseidonError> for NightfallContractError {
    fn from(e: PoseidonError) -> Self {
        Self::PoseidonError(e)
    }
}

/// An error that we can throw during type conversion
#[derive(Debug)]
pub enum ConversionError {
    Overflow,
    ProofDecompression,
    ProofCompression(SerializationError),
    SerialisationError(SerializationError),
    NotErc20DepositData,
    FixedLengthArrayError,
    ParseFailed,
    PoseidonError(PoseidonError),
}
impl Error for ConversionError {}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::Overflow => write!(f, "Overflow during conversion. Uints cannot be bigger than (q-1)/2 where q is the modulus of the scalar field"),
            ConversionError::ProofDecompression => write!(f, "Error during proof decompression"),
            ConversionError::SerialisationError(e) => write!(f, "Error during serialisation: {}", e),
            ConversionError::NotErc20DepositData => write!(f, "Could not convert the public data bytes into ERC20 deposit data"),
            ConversionError::ProofCompression(e) => write!(f, "Error during proof compression: {}", e),
            ConversionError::FixedLengthArrayError => write!(f, "Failed to convert to a fixed length array"),
            ConversionError::ParseFailed => write!(f, "Failed to parse data"),
            ConversionError::PoseidonError(e) => write!(f, "Poseidon Error: {}", e)
        }
    }
}
impl Reject for ConversionError {}

impl From<SerializationError> for ConversionError {
    fn from(e: SerializationError) -> Self {
        ConversionError::SerialisationError(e)
    }
}

impl From<PoseidonError> for ConversionError {
    fn from(e: PoseidonError) -> Self {
        Self::PoseidonError(e)
    }
}

#[derive(Debug)]
pub enum DepositError {
    TokenError(TokenContractError),
    NightfallError(NightfallContractError),
    PoseidonError(PoseidonError),
}

impl Display for DepositError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepositError::TokenError(e) => write!(f, "Deposit Error: {}", e),
            DepositError::NightfallError(e) => write!(f, "Deposit Error: {}", e),
            DepositError::PoseidonError(e) => write!(f, "Deposit Error: {}", e),
        }
    }
}

impl Error for DepositError {}

impl From<TokenContractError> for DepositError {
    fn from(e: TokenContractError) -> Self {
        Self::TokenError(e)
    }
}

impl From<NightfallContractError> for DepositError {
    fn from(e: NightfallContractError) -> Self {
        Self::NightfallError(e)
    }
}

impl From<PoseidonError> for DepositError {
    fn from(e: PoseidonError) -> Self {
        Self::PoseidonError(e)
    }
}

#[derive(Debug, PartialEq)]
pub enum HexError {
    InvalidStringLength,
    InvalidString,
    InvalidHexFormat,
    InvalidConversion,
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HexError::InvalidStringLength => write!(f, "Invalid string length"),
            HexError::InvalidString => write!(f, "Invalid string"),
            HexError::InvalidHexFormat => write!(f, "Invalid hex format"),
            HexError::InvalidConversion => write!(f, "Invalid conversion"),
        }
    }
}

impl std::error::Error for HexError {}

#[derive(Debug)]
pub struct SyncingError(pub EventHandlerError);

impl Display for SyncingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SyncingError(e) => write!(f, "Could not sync {}", e),
        }
    }
}

impl Error for SyncingError {}
