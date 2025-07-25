use crate::domain::{entities::CompressedSecrets, error::ConversionError};

/// trait which public data (in the form of a struct, e.g. ERC20DepositData) must implement
pub trait PublicData {
    fn get_compressed_secrets(&self) -> Result<CompressedSecrets, ConversionError>;
}
