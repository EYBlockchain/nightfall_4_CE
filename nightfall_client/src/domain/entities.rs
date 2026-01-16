use crate::ports::key_provider::KeyProvider;
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use lib::{
    error::HexError,
    serialization::{ark_de_hex, ark_se_hex},
};
use lib::hex_conversion::HexConvertible;
use nf_curves::ed_on_bn254::Fr as BJJScalar;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    str::{self, FromStr},
};
/// A struct representing the status of an HTTP request
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub status: RequestStatus,
    pub uuid: String,
}

/// Struct to represent the realtionship between request and commitment
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCommitmentMapping {
    pub request_id: String,
    pub commitment_hash: String,
}

/// An enum representing the possible statuses of an HTTP request
#[derive(Serialize, Deserialize, Debug)]
pub enum RequestStatus {
    Queued, // This is for tx_request status associated with the X-Request-ID for a request with status: The transaction is waiting to be processed by the client.
    Submitted, // This is for tx_request status associated with the X-Request-ID for a request with status: The Client has successfully processed the transaction and handed off the result, either to the blockchain, in the case of a deposit escrow, or to a Proposer, in the case of a transfer or withdraw transaction.
    Failed, // This is for tx_request status associated with the X-Request-ID for a request with status: The hand off to the next stage did not succeed.
    Processing, // This is for tx_request status associated with the X-Request-ID for a request with status: The Client has taken the transaction out of the queue and is actively working on it, but has not yet completed the hand-off to the next stage.
    ProposerUnreachable, // This is for transfer and withdraw tx_request status when the Client was unable to reach the Proposer at the URL provided in the request.
    Confirmed, // This is for tx_request status associated with the X-Request-ID for a request with status: The life cycle of this tx is finished, aka, commitments are all onchain.
}

impl Display for RequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestStatus::Queued => write!(f, "Queued"),
            RequestStatus::Submitted => write!(f, "Submitted"),
            RequestStatus::Failed => write!(f, "Failed"),
            RequestStatus::Processing => write!(f, "Processing"),
            RequestStatus::ProposerUnreachable => write!(f, "ProposerUnreachable"),
            RequestStatus::Confirmed => write!(f, "Confirmed"),
        }
    }
}

/// a struct representing the states that a commitment can be in
#[derive(Clone, Debug, Deserialize, Serialize, Copy, PartialEq, Default)]
pub enum CommitmentStatus {
    PendingSpend,
    Spent,
    PendingCreation,
    #[default]
    Unspent,
}

/// A struct representing a proposer in a linked list of proposers (used in the ProposerManager contract)
#[derive(Serialize, Deserialize, Debug)]
pub struct Proposer {
    pub stake: ::alloy::primitives::U256,
    pub addr: ::alloy::primitives::Address,
    pub url: ::std::string::String,
    pub next_addr: ::alloy::primitives::Address,
    pub previous_addr: ::alloy::primitives::Address,
}
pub struct EnvironmentKey;

impl KeyProvider<BJJScalar> for EnvironmentKey {
    fn get_key(key_id: &str) -> Option<BJJScalar> {
        let key_string = env::var(key_id).ok()?;
        BJJScalar::from_str(&key_string).ok()
    }
    fn set_key(key_id: &str, key: BJJScalar) -> Result<(), Box<dyn Error>> {
        let key_string = key.to_string();
        // Acknowledge Possible Risks: we're confident that the use of std::env::set_var is indeed safe in this context
        unsafe {
            env::set_var(key_id, key_string);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Transport {
    OnChain,
    OffChain,
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OperationType {
    #[default]
    Deposit,
    Withdraw,
    Transfer,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "Deposit"),
            OperationType::Withdraw => write!(f, "Withdraw"),
            OperationType::Transfer => write!(f, "Transfer"),
        }
    }
}

impl From<OperationType> for u8 {
    fn from(value: OperationType) -> Self {
        match value {
            OperationType::Deposit => 0,
            OperationType::Withdraw => 1,
            OperationType::Transfer => 2,
        }
    }
}

impl From<u8> for OperationType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperationType::Deposit,
            1 => OperationType::Withdraw,
            2 => OperationType::Transfer,
            _ => OperationType::Deposit,
        }
    }
}

/// Struct representing a Nightfall operation, together with the transport mechanism
#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub transport: Transport,
    pub operation_type: OperationType,
}

pub struct ParseOperationError;
impl FromStr for Operation {
    type Err = ParseOperationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deposit-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Deposit,
            }),
            "deposit-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Deposit,
            }),
            "withdraw-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Withdraw,
            }),
            "withdraw-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Withdraw,
            }),
            "transfer-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Transfer,
            }),
            "transfer-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Transfer,
            }),
            _ => Err(ParseOperationError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProofType {
    Groth16,
    Plonk,
}

pub struct ParseProofTypeError;
impl FromStr for ProofType {
    type Err = ParseProofTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Groth16" => Ok(ProofType::Groth16),
            "Plonk" => Ok(ProofType::Plonk),
            _ => Err(ParseProofTypeError),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub erc_address: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub token_id: BigInteger256,
}
pub struct ERCAddress;
impl ERCAddress {
    #[allow(dead_code)]
    pub fn try_from_hex_string(h: &str) -> Result<Fr254, HexError> {
        let bytes = Vec::<u8>::from_hex_string(h)?;
        let uint = BigUint::from_bytes_be(&bytes);
        // Check the address is no more than 20 bytes long
        if uint > BigUint::from_bytes_be(&[255; 20]) {
            return Err(HexError::InvalidStringLength);
        }
        Ok(Fr254::from(uint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod operation_tests {
        use super::*;
        #[test]
        fn enum_correct() {
            let o = Operation::from_str("deposit-onchain");
            assert!(o.is_ok());
        }

        #[test]
        fn enum_incorrect() {
            let o = Operation::from_str("clearly_wrong");
            assert!(o.is_err());
        }
    }
    mod proof_type_tests {
        use super::*;
        #[test]
        fn enum_correct() {
            let o = ProofType::from_str("Groth16");
            assert!(o.is_ok());
        }

        #[test]
        fn enum_incorrect() {
            let o = ProofType::from_str("clearly_wrong");
            assert!(o.is_err());
        }
    }
    mod erc_address_tests {
        use super::*;
        use ark_ff::BigInt;
        #[test]
        fn create_erc_address_from_hex() {
            let test_address = Fr254::new(BigInt([
                0x5fe9b39eaac67dc6,
                0xcff77681312747be,
                0xea730722,
                0x00,
            ]));
            let test_address_2 = Fr254::new(BigInt([
                0x5fe9b39eaac67dc6,
                0xcff77681312747be,
                0x00730722,
                0x00,
            ]));

            let address = "0xea730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address
            );
            assert_eq!(
                ERCAddress::try_from_hex_string(&address[2..]).unwrap(),
                test_address
            );
            let address = "0x00ea730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address
            );
            // make sure leading zeros are correctly handled
            let address = "0x00730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address_2
            );
        }
        #[test]
        fn address_too_big() {
            let address = "0x010000000000000000000000000000000000000000";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap_err(),
                HexError::InvalidStringLength
            );
            let address = "0xffffffffffffffffffffffffffffffffffffffff";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                Fr254::from(BigInt::new([
                    0xffffffffffffffff,
                    0xffffffffffffffff,
                    0xffffffff,
                    0,
                ],))
            );
        }
    }
}
