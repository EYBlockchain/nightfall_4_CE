use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use lib::{
    serialization::{ark_de_hex, ark_se_hex},
    shared_entities::{ClientTransaction, OnChainTransaction},
};
use log::error;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::fmt::Debug;

/// A Block struct representing NF block
/// NOTE: This is not finalised yet, we may need to change fields to this struct
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Block {
    // The root of the merkle tree of all commitments in this block.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments_root: Fr254,
    // The root of the merkle tree of all nullifiers in this block.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifiers_root: Fr254,
    // The new root of the tree of all previous commitments_roots.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments_root_root: Fr254,
    // The hash of the block.
    // The list of transactions in this block.
    pub transactions: Vec<OnChainTransaction>,
    pub rollup_proof: Vec<u8>,
}

/// Struct used to represent deposit data, used in making deposit proofs by the proposer.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct DepositData {
    /// The Nightfall token ID
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_token_id: Fr254,
    /// The Nightfall slot ID
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_slot_id: Fr254,
    /// The value of the deposit
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub value: Fr254,
    /// The secret hash used to redeem the deposit
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub secret_hash: Fr254,
}

/// Struct used to represent deposit data, used in making deposit proofs by the proposer.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct DepositDatawithFee {
    /// The fee paid to the proposer
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    /// deposit data
    pub deposit_data: DepositData,
}

impl DepositDatawithFee {
    #[allow(dead_code)]
    pub fn hash(&self) -> Result<Vec<u32>, SerializationError> {
        // Step 1: Serialize to bytes
        let encoding = serde_json::to_vec(self).map_err(|e| {
            error!("DepositDatawithFee hash computation error: {e}");
            SerializationError::InvalidData
        })?;

        // Step 2: Hash the bytes with Keccak256
        let hash = Keccak256::digest(encoding);

        // Step 3: Convert hash bytes to Vec<u32>
        Ok(hash.iter().map(|&b| b as u32).collect())
    }
}

/// A struct representing a client transaction with added metadata that tells us about its current state.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientTransactionWithMetaData<P> {
    pub client_transaction: ClientTransaction<P>,
    pub block_l2: Option<u64>,
    pub in_mempool: bool,
    pub hash: Vec<u32>,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub historic_roots: Vec<Fr254>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoricRoot(
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")] pub Fr254,
    pub u32,
);
