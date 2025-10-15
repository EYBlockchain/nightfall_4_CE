use crate::{
    contract_conversions::{FrBn254, Uint256},
    nf_client_proof::{Proof, PublicInputs},
    serialization::{ark_de_hex, ark_se_hex},
};
use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use log::{error, warn};
use nightfall_bindings::artifacts::Nightfall;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::fmt::Debug;

/// A struct representing the synchronisation status of a container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynchronisationPhase {
    /// Client is fully caught up with the on-chain state.
    Synchronized,
    /// Client is ahead of the chain and No need to resync.
    AheadOfChain { blocks_ahead: usize },
    /// Client is out-of-sync and must restart syncing.
    Desynchronized,
}

/// A struct representing the synchronisation status of a container
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SynchronisationStatus {
    phase: SynchronisationPhase,
}

impl SynchronisationStatus {
    /// Create a new instance
    pub fn new(phase: SynchronisationPhase) -> Self {
        Self { phase }
    }
    /// Get the current synchronisation phase
    pub fn phase(&self) -> SynchronisationPhase {
        self.phase
    }
    /// return whether the application is synchronised with the blockchain
    pub fn is_synchronised(&self) -> bool {
        matches!(self.phase, SynchronisationPhase::Synchronized)
    }
    /// Set the synchronisation status to fully synchronised
    pub fn set_synchronised(&mut self) {
        self.phase = SynchronisationPhase::Synchronized;
    }
    /// clear the synchronisation status
    pub fn clear_synchronised(&mut self) {
        self.phase = SynchronisationPhase::Desynchronized;
    }
}

/// A struct representing a node in a Merkle Tree
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub index: usize,
}

/// A struct representing summary data about an append-only Merkle Tree
pub struct AppendOnlyTreeMetadata<F> {
    pub main_tree_height: u32,
    pub sub_tree_height: u32,
    pub sub_tree_count: usize,
    pub frontier: Vec<F>,
    pub root: F,
}

/// Formalises the compressed secrets in a client proof.  This makes the purpose of the data clearer than using
/// the tuple output of the KEM-DEM function
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct CompressedSecrets {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub cipher_text: [Fr254; 5],
}

/// Transaction struct representing NF on chain transaction
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct OnChainTransaction {
    // The fee paid to the proposer.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    // List of new commitments created by this transaction.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments: [Fr254; 4],
    // List of nullifiers consumed by this transaction.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifiers: [Fr254; 4],
    // public data (public inputs) associated with this transaction.
    pub public_data: CompressedSecrets,
}
/// Converts the NF_4 smart contract representation of an on-chain transaction (i.e. a transaction that is
/// rolled up into a block), into a form more sutiable for manipulation in Rust.
impl From<Nightfall::OnChainTransaction> for OnChainTransaction {
    fn from(ntx: Nightfall::OnChainTransaction) -> Self {
        Self {
            fee: FrBn254::try_from(ntx.fee)
                .expect("Conversion of on-chain fee into field element should never fail")
                .0,
            commitments: ntx.commitments.map(|c| {
                FrBn254::try_from(c)
                    .expect(
                        "Conversion of on-chain commitments into field elements should never fail",
                    )
                    .0
            }),
            nullifiers: ntx.nullifiers.map(|n| {
                FrBn254::try_from(n)
                    .expect(
                        "Conversion of on-chain commitments into field elements should never fail",
                    )
                    .0
            }),
            public_data: ntx.public_data.into(),
        }
    }
}

/// Converts the Domain representation of an onchain transaction (i.e. one that is rolled up into a block)
/// into one suitable for interacting with the smart contract
impl From<OnChainTransaction> for Nightfall::OnChainTransaction {
    fn from(otx: OnChainTransaction) -> Self {
        Self {
            fee: Uint256::from(otx.fee).into(),
            commitments: otx.commitments.map(|c| Uint256::from(c).into()),
            nullifiers: otx.nullifiers.map(|n| Uint256::from(n).into()),
            public_data: otx.public_data.into(),
        }
    }
}

/// Converts a ClientTransaction into a form suitable for rolling into a block.
impl<P> From<&ClientTransaction<P>> for OnChainTransaction {
    fn from(client_transaction: &ClientTransaction<P>) -> Self {
        Self {
            fee: client_transaction.fee,
            commitments: client_transaction.commitments,
            nullifiers: client_transaction.nullifiers,
            public_data: client_transaction.compressed_secrets,
        }
    }
}

impl From<&PublicInputs> for OnChainTransaction {
    fn from(p: &PublicInputs) -> Self {
        OnChainTransaction {
            fee: p.fee,
            commitments: p.commitments,
            nullifiers: p.nullifiers,
            public_data: CompressedSecrets {
                cipher_text: p.compressed_secrets,
            },
        }
    }
}

/// Token Type Based on ERC Standards or L2
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    #[default]
    ERC20,
    ERC1155,
    ERC721,
    ERC3525,
}

impl From<TokenType> for u8 {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::ERC20 => 0,
            TokenType::ERC1155 => 1,
            TokenType::ERC721 => 2,
            TokenType::ERC3525 => 3,
        }
    }
}

impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => TokenType::ERC20,
            1 => TokenType::ERC1155,
            2 => TokenType::ERC721,
            3 => TokenType::ERC3525,
            _ => {
                warn!("TokenType value {value} not supported, defaulting to ERC20");
                TokenType::ERC20
            }
        }
    }
}

/// Transaction struct representing NF client transaction
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientTransaction<P> {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub historic_commitment_roots: [Fr254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments: [Fr254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifiers: [Fr254; 4],
    pub compressed_secrets: CompressedSecrets,
    pub proof: P,
}

impl<P: Proof + Debug + Serialize + Clone> ClientTransaction<P> {
    #[allow(dead_code)]
    pub fn hash(&self) -> Result<Vec<u32>, SerializationError> {
        let encoding = serde_json::to_vec(self).map_err(|e| {
            error!("Proof hash computation error {e}");
            SerializationError::InvalidData
        })?;
        let hash = Keccak256::digest(encoding);
        // convert to u32 because the Mongo Rust driver doesn't support u8
        Ok(hash.iter().map(|&b| b as u32).collect())
    }
}

impl<P: Proof + Debug + Serialize + Clone> From<&ClientTransaction<P>> for PublicInputs {
    fn from(tx: &ClientTransaction<P>) -> Self {
        PublicInputs {
            fee: tx.fee,
            commitments: tx.commitments,
            nullifiers: tx.nullifiers,
            compressed_secrets: tx.compressed_secrets.cipher_text,
            roots: tx.historic_commitment_roots,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod token_type_tests {
        use super::*;
        #[test]
        fn return_correct_number() {
            let t_erc20 = TokenType::ERC20;
            let u_erc20 = u8::from(t_erc20);
            assert_eq!(u_erc20, 0);
            let t_erc1155 = TokenType::ERC1155;
            let u_erc1155 = u8::from(t_erc1155);
            assert_eq!(u_erc1155, 1);
            let t_erc721 = TokenType::ERC721;
            let u_erc721 = u8::from(t_erc721);
            assert_eq!(u_erc721, 2);
            let t_erc3525 = TokenType::ERC3525;
            let u_erc3525 = u8::from(t_erc3525);
            assert_eq!(u_erc3525, 3);
        }
    }
}
