use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::SerializationError;
use lib::serialization::{ark_de_hex, ark_se_hex};
use log::error;
use nightfall_client::domain::entities::{ClientTransaction, CompressedSecrets, HexConvertible};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::{digest::generic_array::GenericArray, Digest, Keccak256};
use std::fmt::Debug;

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

impl OnChainTransaction {
    pub fn hash_commitments(&self) -> Fr254 {
        let mut bytes = Vec::new();
        for c in &self.commitments {
            let c_string = c.to_hex_string();
            bytes.extend_from_slice(c_string.as_bytes());
        }
        let hash = Sha256::digest(&bytes);
        Fr254::from_be_bytes_mod_order(&hash)
    }
    pub fn hash(&self) -> Fr254 {
        let mut bytes = Vec::with_capacity(32 * 14); // 14 field elements: 1 fee + 4 commitments + 4 nullifiers + 5 cipher_text

        // 1. Hash fee
        bytes.extend_from_slice(&self.fee.into_bigint().to_bytes_be());

        // 2. Commitments
        for c in &self.commitments {
            bytes.extend_from_slice(&c.into_bigint().to_bytes_be());
        }

        // 3. Nullifiers
        for n in &self.nullifiers {
            bytes.extend_from_slice(&n.into_bigint().to_bytes_be());
        }

        // 4. Cipher text (CompressedSecrets)
        for ct in &self.public_data.cipher_text {
            bytes.extend_from_slice(&ct.into_bigint().to_bytes_be());
        }

        // 5. Hash using SHA-256
        let hash = Sha256::digest(&bytes);

        // 6. Convert SHA-256 hash to Fr254 (modular reduction)
        Fr254::from_be_bytes_mod_order(&hash)
    }
}

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

impl Block {
    #[allow(dead_code)]
    pub fn hash_commitments(&self) -> Fr254 {
        let mut bytes = Vec::new();
        for tx in &self.transactions {
            let tx_hash = tx.hash_commitments();
            bytes.extend_from_slice(&tx_hash.into_bigint().to_bytes_be());
        }
        bytes.extend_from_slice(&self.rollup_proof);
        let hash = Sha256::digest(&bytes);
        Fr254::from_be_bytes_mod_order(&hash)
    }
    pub fn hash(&self) -> Fr254 {
        let mut bytes = Vec::new();

        // 1. Hash commitments_root
        bytes.extend_from_slice(&self.commitments_root.into_bigint().to_bytes_be());

        // 2. Hash nullifiers_root
        bytes.extend_from_slice(&self.nullifiers_root.into_bigint().to_bytes_be());

        // 3. Hash commitments_root_root
        bytes.extend_from_slice(&self.commitments_root_root.into_bigint().to_bytes_be());

        // 4. Hash transactions
        for tx in &self.transactions {
            let tx_hash = tx.hash();
            bytes.extend_from_slice(&tx_hash.into_bigint().to_bytes_be());
        }
        // 5. Hash rollup_proof
        bytes.extend_from_slice(&self.rollup_proof);
        // 6. Hash using SHA-256
        let hash = Sha256::digest(&bytes);
        // 7. Convert SHA-256 hash to Fr254 (modular reduction)
        Fr254::from_be_bytes_mod_order(&hash)
    }
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
            error!("DepositDatawithFee hash computation error: {}", e);
            SerializationError::InvalidData
        })?;

        // Step 2: Hash the bytes with Keccak256
        let hash: GenericArray<u8, _> = Keccak256::digest(encoding);

        // Step 3: Convert hash bytes to Vec<u32>
        Ok(hash
            .as_slice()
            .iter()
            .map(|h| u32::from(*h))
            .collect::<Vec<u32>>())
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
