use lib::{error::ConversionError, shared_entities::CompressedSecrets};

/// trait which public data (in the form of a struct, e.g. ERC20DepositData) must implement
pub trait PublicData {
    fn get_compressed_secrets(&self) -> Result<CompressedSecrets, ConversionError>;
}
