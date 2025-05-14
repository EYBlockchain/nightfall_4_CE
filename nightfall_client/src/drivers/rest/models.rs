use crate::{
    domain::entities::{DepositSecret, HexConvertible, WithdrawData},
    ports::keys::KeySpending,
};
use ark_bn254::Fr as Fr254;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use warp::reject::Reject;

use super::utils::to_nf_token_id_from_str;

#[derive(Debug)]
pub struct NullifierKey(pub Fr254);

impl KeySpending for NullifierKey {
    fn get_nullifier_key(&self) -> Fr254 {
        self.0
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PreimageReq {
    pub value: String,
    pub erc_address: String,
    pub token_id: String,
    pub public_key: String,
    pub salt: String,
}

impl Default for PreimageReq {
    fn default() -> Self {
        PreimageReq {
            value: "0x00".to_string(),
            erc_address: "0x00".to_string(),
            token_id: "0x00".to_string(),
            public_key: "0x00".to_string(),
            salt: "0x00".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretPreimageReq {
    pub preimage_one: String,
    pub preimage_two: String,
    pub preimage_three: String,
}

impl Default for SecretPreimageReq {
    fn default() -> Self {
        Self {
            preimage_one: "0x00".to_string(),
            preimage_two: "0x00".to_string(),
            preimage_three: "0x00".to_string(),
        }
    }
}

impl From<DepositSecret> for SecretPreimageReq {
    fn from(value: DepositSecret) -> Self {
        SecretPreimageReq {
            preimage_one: value.preimage_one.to_string(),
            preimage_two: value.preimage_two.to_string(),
            preimage_three: value.preimage_three.to_string(),
        }
    }
}

/// structure representing an NF_3 deposit request to provide a simpler,
/// slightly high-level interface for the client to use, and for backwards compatibility with NF_3
#[derive(Debug, Deserialize, Serialize)]
pub struct NF3DepositRequest {
    #[serde(rename = "ercAddress")]
    pub erc_address: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    pub value: String,
    pub fee: String,
    pub deposit_fee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_preimage_one: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_preimage_two: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_preimage_three: Option<String>,
}

/// structure representing an NF_3 deposit request to provide a simpler,
/// slightly high-level interface for the client to use, and for backwards compatibility with NF_3
#[derive(Debug, Deserialize, Serialize)]
pub struct NF3TransferRequest {
    #[serde(rename = "ercAddress")]
    pub erc_address: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "recipientData")]
    pub recipient_data: NF3RecipientData,
    pub fee: String,
}

/// structure representing an NF_3 withdraw request to provide a simpler,
/// slightly high-level interface for the client to use, and for backwards compatibility with NF_3
#[derive(Debug, Deserialize, Serialize)]
pub struct NF3WithdrawRequest {
    #[serde(rename = "ercAddress")]
    pub erc_address: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    pub value: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    pub fee: String,
}

/// structure representing NF_3 recipient data
/// This is a sub-structure of the NF_3 transfer request
#[derive(Debug, Deserialize, Serialize)]
pub struct NF3RecipientData {
    pub values: Vec<String>,
    #[serde(rename = "recipientCompressedZkpPublicKeys")]
    pub recipient_compressed_zkp_public_keys: Vec<String>,
}

/// Struct used for checking that funds are available to withdraw.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithdrawDataReq {
    pub token_id: String,
    pub erc_address: String,
    pub recipient_address: String,
    pub value: String,
    pub fee: String,
    pub token_type: String,
    pub withdraw_fund_salt: String,
}

/// Struct used for checking that funds are available to de-escrow.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeEscrowDataReq {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "ercAddress")]
    pub erc_address: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    pub value: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "withdrawFundSalt")]
    pub withdraw_fund_salt: String,
}


#[derive(Debug)]
pub enum NF3RequestError {
    CouldNotDeserialiseRootKey,
    CouldNotSerialisePublicKey,
    KeyGenerationError,
    TooManyRecipients,
    CouldNotDeserialiseAddress,
    CouldNotDeserialiseValue,
    NoUsableCommitments,
    ConversionError,
}

impl Error for NF3RequestError {}
impl Display for NF3RequestError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            NF3RequestError::CouldNotDeserialiseRootKey => {
                write!(f, "Could not deserialise root key")
            }
            NF3RequestError::CouldNotSerialisePublicKey => {
                write!(f, "Could not serialise public key")
            }
            NF3RequestError::KeyGenerationError => {
                write!(f, "Could not generate keys from root key")
            }
            NF3RequestError::TooManyRecipients => {
                write!(f, "Too many recipients")
            }
            NF3RequestError::CouldNotDeserialiseAddress => {
                write!(f, "Could not deserialise address")
            }
            NF3RequestError::CouldNotDeserialiseValue => {
                write!(f, "Could not deserialise value")
            }
            NF3RequestError::NoUsableCommitments => {
                write!(f, "No usable commitments")
            }
            NF3RequestError::ConversionError => {
                write!(f, "Conversion error")
            }
        }
    }
}
impl Reject for NF3RequestError {}

impl TryFrom<SecretPreimageReq> for DepositSecret {
    type Error = &'static str;
    fn try_from(req: SecretPreimageReq) -> Result<Self, Self::Error> {
        Ok(DepositSecret {
            preimage_one: Fr254::from_hex_string(req.preimage_one.as_str())
                .map_err(|_| "Preimage one failed to convert")?,
            preimage_two: Fr254::from_hex_string(req.preimage_two.as_str())
                .map_err(|_| "Preimage two failed to convert")?,
            preimage_three: Fr254::from_hex_string(req.preimage_three.as_str())
                .map_err(|_| "Preimage three failed to convert")?,
        })
    }
}

impl TryFrom<DeEscrowDataReq> for WithdrawData {
    type Error = &'static str;
    fn try_from(req: DeEscrowDataReq) -> Result<Self, Self::Error> {
        let nf_token_id = to_nf_token_id_from_str(req.erc_address.as_str(), req.token_id.as_str())
            .map_err(|_| "Failed to convert erc address and token id to Nightfall equivalent")?;
        Ok(WithdrawData {
            nf_token_id,
            withdraw_address: Fr254::from_hex_string(req.recipient_address.as_str())
                .map_err(|_| "Withdraw address failed to convert")?,
            value: Fr254::from_hex_string(req.value.as_str())
                .map_err(|_| "Withdraw value failed to convert")?,
            withdraw_fund_salt: Fr254::from_hex_string(req.withdraw_fund_salt.as_str())
                .map_err(|_| "Withdraw withdraw_fund_salt failed to convert")?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyRequest {
    pub mnemonic: String,
    pub child_path: String,
}

/// structure representing a request to escrow some funds
/// The key_id is used to identify the key that will be used to pay the escrow
/// It will be used to look up the key in the wallet that is being used.
#[derive(Deserialize, Serialize, Default)]
pub struct EscrowRequest {
    pub erc_address: String,
    pub token_id: String,
    pub value: String,
    pub key_id: String,
    pub wallet_password: String,
}
