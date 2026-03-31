use alloy::rpc::json_rpc::RpcError;
use alloy::signers::local::LocalSignerError as WalletError;
use alloy::transports::TransportError;
use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use jf_plonk::errors::PlonkError;
use jf_primitives::poseidon::PoseidonError;
use jf_relation::errors::CircuitError;
use std::error::Error;
use std::fmt;
use warp::reject::Reject;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HexError {
    #[error("Invalid string length")]
    InvalidStringLength,
    #[error("Invalid string")]
    InvalidString,
    #[error("Invalid hex format")]
    InvalidHexFormat,
    #[error("Invalid conversion")]
    InvalidConversion,
}

#[derive(Debug)]
pub struct CertificateVerificationError {
    message: String,
}

impl CertificateVerificationError {
    pub fn new(msg: &str) -> CertificateVerificationError {
        CertificateVerificationError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CertificateVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CertificateVerificationError: {}", self.message)
    }
}

impl Error for CertificateVerificationError {}

impl Reject for CertificateVerificationError {}

#[derive(Debug)]
pub struct KeyVerificationError {
    message: String,
}

impl KeyVerificationError {
    pub fn new(msg: &str) -> KeyVerificationError {
        KeyVerificationError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for KeyVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyVerificationError: {}", self.message)
    }
}

impl From<CircuitError> for KeyVerificationError {
    fn from(e: CircuitError) -> Self {
        KeyVerificationError::new(&format!("Circuit error: {e}"))
    }
}

impl From<std::io::Error> for KeyVerificationError {
    fn from(e: std::io::Error) -> Self {
        KeyVerificationError::new(&format!("IO error: {e}"))
    }
}

impl From<PlonkError> for KeyVerificationError {
    fn from(e: PlonkError) -> Self {
        KeyVerificationError::new(&format!("Plonk error: {e}"))
    }
}

impl Error for KeyVerificationError {}

impl Reject for KeyVerificationError {}

/// Errors that can be throw when working with a blockchain client connector
#[derive(Debug, thiserror::Error)]
pub enum BlockchainClientConnectionError {
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError<String>),
    #[error("Transport error: {0}")]
    TransportError(#[from] TransportError),
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Wallet error: {0}")]
    WalletError(#[from] WalletError),
    #[error("Azure error: {0}")]
    AzureError(#[from] Box<dyn Error + Send + Sync>),
    #[error("InvalidWalletType: {0}")]
    InvalidWalletType(String),
}

impl From<String> for BlockchainClientConnectionError {
    fn from(e: String) -> Self {
        BlockchainClientConnectionError::ProviderError(e)
    }
}

/// An error that we can throw during type conversion
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Overflow during conversion. Uints cannot be bigger than (q-1)/2 where q is the modulus of the scalar field")]
    Overflow,
    #[error("Error during proof decompression")]
    ProofDecompression,
    #[error("Error during proof compression: {0}")]
    ProofCompression(SerializationError),
    #[error("Error during serialisation: {0}")]
    SerialisationError(#[from] SerializationError),
    #[error("Could not convert the public data bytes into ERC20 deposit data")]
    NotErc20DepositData,
    #[error("Failed to convert to a fixed length array")]
    FixedLengthArrayError,
    #[error("Failed to parse data")]
    ParseFailed,
    #[error("Poseidon Error: {0}")]
    PoseidonError(#[from] PoseidonError),
    #[error("Invalid token type")]
    InvalidTokenType,
}
impl Reject for ConversionError {}

/// Error type used by the Event Listener, that listens for blockchain events and processes them.
#[derive(Debug, thiserror::Error)]
pub enum EventHandlerError {
    #[error("Could not connect to event stream")]
    NoEventStream,
    #[error("Event stream terminated")]
    StreamTerminated,
    #[error("Invalid calldata")]
    InvalidCalldata,
    #[error("IO Error: {0}")]
    IOError(String),
    #[error("Missing layer 2 blocks. Last processed was: {0}")]
    MissingBlocks(usize),
    #[error("Hashing error")]
    HashError,
    #[error("Block not found: {0}")]
    BlockNotFound(u64),
    #[error("Block hash error, expected block hash: {0}, got block hash: {1}")]
    BlockHashError(Fr254, Fr254),
}
impl Reject for EventHandlerError {}

/// Error type for handling calls to a token contract
#[derive(Debug, thiserror::Error)]
pub enum NightfallContractError {
    #[error("Nightfall Contract Error: Blockchain Client Connection Error: {0}")]
    BlockchainClientConnectionError(#[from] BlockchainClientConnectionError),
    #[error("Nightfall Contract Error: Error while converting to Solidity type: {0}")]
    ConversionError(#[from] ConversionError),
    #[error("Did not receive a transaction receipt")]
    TransactionError,
    #[error("Escrow Funds Error: {0}")]
    EscrowError(String),
    #[error("De-Escrow Funds Error: {0}")]
    DeEscrowError(String),
    #[error("Contract Verification Error: {0}")]
    ContractVerificationError(String),
    #[error("Hashing Error: {0}")]
    PoseidonError(#[from] PoseidonError),
    #[error("Layer 2 block number {0} not found on-chain")]
    BlockNotFound(u64),
    #[error("Blockchain provider error: {0}")]
    ProviderError(String),
    #[error("Missing transaction hash: {0}")]
    MissingTransactionHash(String),
    #[error("Transaction not found: {0}")]
    TransactionNotFound(alloy::primitives::TxHash),
    #[error("ABI decode error: {0}")]
    AbiDecodeError(String),
    #[error("Decoded call error: {0}")]
    DecodedCallError(String),
    #[error("X509 error: {0}")]
    X509Error(String),
    #[error("Block proposal error: {0}")]
    BlockProposalError(String),
}

/// Error type for proposer rotation
#[derive(Debug, thiserror::Error)]
pub enum ProposerError {
    #[error("Failed to get list of Proposers")]
    FailedToGetProposers,
    #[error("Provider error")]
    ProviderError(String),
}

impl warp::reject::Reject for ProposerError {}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid block size: {0}")]
    InvalidBlockSize(String),
    #[error("Configuration error: {0}")]
    Other(String),
}
