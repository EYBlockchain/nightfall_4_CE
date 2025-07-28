
use alloy::signers::local::LocalSignerError as WalletError;
use alloy::rpc::json_rpc::RpcError;
use alloy::transports::TransportError;
use std::{
    error::Error,
    fmt::{self, Debug, Display},
};
use warp::reject::Reject;

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

/// Errors that can be throw when working with a blockchain client connector
#[derive(Debug)]
pub enum BlockchainClientConnectionError {
    RpcError(RpcError<String>),
    TransportError(TransportError),
    ProviderError(String),
    WalletError(WalletError),
    AzureError(Box<dyn Error + Send + Sync>),
}

impl Display for BlockchainClientConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockchainClientConnectionError::RpcError(e) => write!(f, "RPC error: {e}"),
            BlockchainClientConnectionError::TransportError(e) => write!(f, "Transport error: {e}"),
            BlockchainClientConnectionError::ProviderError(e) => write!(f, "Provider error: {e}"),
            BlockchainClientConnectionError::WalletError(e) => write!(f, "Wallet error: {e}"),
            BlockchainClientConnectionError::AzureError(e) => write!(f, "Azure error: {e}"),
        }
    }
}

impl Error for BlockchainClientConnectionError {}

impl From<String> for BlockchainClientConnectionError {
    fn from(e: String) -> Self {
        BlockchainClientConnectionError::ProviderError(e)
    }
}
impl From<RpcError<String>> for BlockchainClientConnectionError {
    fn from(e: RpcError<String>) -> Self {
        BlockchainClientConnectionError::RpcError(e)
    }
}

impl From<WalletError> for BlockchainClientConnectionError {
    fn from(e: WalletError) -> Self {
        BlockchainClientConnectionError::WalletError(e)
    }
}

impl From<Box<dyn Error + Send + Sync>> for BlockchainClientConnectionError {
    fn from(e: Box<dyn Error + Send + Sync>) -> Self {
        BlockchainClientConnectionError::AzureError(e)
    }
}
impl From<TransportError> for BlockchainClientConnectionError {
    fn from(e: TransportError) -> Self {
        BlockchainClientConnectionError::TransportError(e)
    }
}
