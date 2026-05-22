use alloy::primitives::TxHash;
use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use lib::{
    serialization::{ark_de_hex, ark_se_hex},
    shared_entities::DepositData,
    shared_entities::{ClientTransaction, OnChainTransaction},
};
use log::error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Keccak256};
use std::{fmt, fmt::Debug};

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
    #[serde(default)]
    pub block_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PendingBlockState {
    Reserved,
    ReadyToPropose,
    BroadcastPending,
}

fn default_pending_block_state() -> PendingBlockState {
    PendingBlockState::ReadyToPropose
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PendingBlock {
    pub layer2_block_number: u64,
    #[serde(default = "default_pending_block_state")]
    pub state: PendingBlockState,
    #[serde(default)]
    pub broadcast_tx_hash: Option<TxHash>,
    #[serde(default)]
    pub broadcast_receipt_checks: u32,
    pub block: Option<Block>,
    pub selected_deposits: Vec<Vec<DepositDatawithFee>>,
    pub selected_client_transaction_hashes: Vec<Vec<u32>>,
}

/// Struct used to represent deposit data, used in making deposit proofs by the proposer.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct DepositDatawithFee {
    /// The fee paid to the proposer
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    /// deposit data
    pub deposit_data: DepositData,
    #[serde(default)]
    pub reserved: bool,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum TxLifecycle {
    Mempool,
    Selected { block_l2: u64 },
    Included { block_l2: u64 },
    Cancelled,
    Dropped,
}

impl Default for TxLifecycle {
    fn default() -> Self {
        Self::Mempool
    }
}

impl TxLifecycle {
    pub fn block_l2(&self) -> Option<u64> {
        match self {
            Self::Selected { block_l2 } => Some(*block_l2),
            Self::Included { block_l2 } => Some(*block_l2),
            _ => None,
        }
    }

    pub fn is_mempool(&self) -> bool {
        matches!(self, Self::Mempool)
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected { .. })
    }

    pub fn is_included(&self) -> bool {
        matches!(self, Self::Included { .. })
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ClientTransactionWithMetaData<P> {
    pub client_transaction: ClientTransaction<P>,
    pub lifecycle: TxLifecycle,
    pub hash: Vec<u32>,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub historic_roots: Vec<Fr254>,
    /// Capability token required to create a transfer receipt for this transaction.
    /// Generated at submission time and returned to the submitter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_token: Option<String>,
}

#[derive(Deserialize)]
struct ClientTransactionWithMetaDataSerde<P> {
    pub client_transaction: ClientTransaction<P>,
    #[serde(default)]
    pub lifecycle: Option<TxLifecycle>,
    pub block_l2: Option<u64>,
    pub in_mempool: Option<bool>,
    #[serde(default)]
    pub cancelled_explicitly: bool,
    #[serde(default)]
    pub receipt_token: Option<String>,
    pub hash: Vec<u32>,
    #[serde(deserialize_with = "ark_de_hex")]
    pub historic_roots: Vec<Fr254>,
}

impl<'de, P> Deserialize<'de> for ClientTransactionWithMetaData<P>
where
    P: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ClientTransactionWithMetaDataSerde::<P>::deserialize(deserializer)?;
        // Legacy migration: a document with cancelled_explicitly=false,
        // in_mempool=false, block_l2=None corresponds to a stale/expired
        // removal in the pre-lifecycle codebase. All persisted transactions
        // historically entered the mempool with in_mempool=true first
        // (see nightfall_client_transaction.rs), so this triplet
        // unambiguously represents a Dropped state. If a future code
        // path persists transactions in this triplet for a different
        // reason, this assumption must be revisited.
        let lifecycle = helper.lifecycle.unwrap_or_else(|| {
            if helper.cancelled_explicitly {
                TxLifecycle::Cancelled
            } else if helper.in_mempool.unwrap_or(false) {
                TxLifecycle::Mempool
            } else if let Some(block_l2) = helper.block_l2 {
                TxLifecycle::Selected { block_l2 }
            } else {
                TxLifecycle::Dropped
            }
        });

        Ok(Self {
            client_transaction: helper.client_transaction,
            lifecycle,
            hash: helper.hash,
            historic_roots: helper.historic_roots,
            receipt_token: helper.receipt_token,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferReceiptStatus {
    Pending,
    IncludedL2,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TxHashBytes([u8; 32]);

impl TxHashBytes {
    pub fn from_u32_slice(bytes: &[u32]) -> Option<Self> {
        if bytes.len() != 32 || bytes.iter().any(|&byte| byte > u8::MAX as u32) {
            return None;
        }

        let mut tx_hash = [0u8; 32];
        for (target, source) in tx_hash.iter_mut().zip(bytes.iter()) {
            *target = *source as u8;
        }
        Some(Self(tx_hash))
    }

    pub fn from_hex(hex_str: &str) -> Option<Self> {
        if hex_str.len() != 64 || !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let bytes = hex::decode(hex_str).ok()?;
        let arr: [u8; 32] = bytes.try_into().ok()?;
        Some(Self(arr))
    }

    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn as_u32_vec(&self) -> Vec<u32> {
        self.0.iter().map(|&byte| byte as u32).collect()
    }
}

impl Serialize for TxHashBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_hex())
    }
}

impl<'de> Deserialize<'de> for TxHashBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TxHashBytesVisitor;

        impl<'de> de::Visitor<'de> for TxHashBytesVisitor {
            type Value = TxHashBytes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a 64-character hex string or 32 byte values")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.len() != 64 || value.chars().any(|c| !c.is_ascii_hexdigit()) {
                    return Err(E::custom("tx_hash must be a 64-character hex string"));
                }

                let decoded = hex::decode(value).map_err(E::custom)?;
                let bytes: [u8; 32] = decoded
                    .try_into()
                    .map_err(|_| E::custom("tx_hash must decode to 32 bytes"))?;
                Ok(TxHashBytes(bytes))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 32];
                for (index, target) in bytes.iter_mut().enumerate() {
                    let Some(byte) = seq.next_element::<u32>()? else {
                        return Err(de::Error::invalid_length(index, &self));
                    };
                    if byte > u8::MAX as u32 {
                        return Err(de::Error::custom("tx_hash byte value exceeds 255"));
                    }
                    *target = byte as u8;
                }

                if seq.next_element::<u32>()?.is_some() {
                    return Err(de::Error::custom("tx_hash must contain exactly 32 bytes"));
                }

                Ok(TxHashBytes(bytes))
            }
        }

        deserializer.deserialize_any(TxHashBytesVisitor)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferReceipt {
    /// Deterministic unique identifier for this receipt, derived as
    /// `HMAC-SHA256(receipt_token, tx_hash)`, hex-encoded (64 lowercase hex chars).
    /// Identical across all proposers for the same transfer, allowing the receiver
    /// to query any available proposer using the same `receipt_id`.
    pub receipt_id: String,
    /// The canonical proposer-side transaction hash of the transfer this receipt is
    /// associated with, stored as 32 raw bytes. Used as the primary lookup key when
    /// checking for duplicate receipts on the same transaction.
    pub tx_hash: TxHashBytes,
    /// The opaque KEM-DEM encrypted receipt payload supplied by the sender wallet,
    /// hex-encoded. For `version = 1` this is exactly 576 hex characters
    /// (9 × 32-byte BN254 field elements: 7 encrypted plaintext fields followed by
    /// the ephemeral public key y-coordinate and x-sign flag). The proposer stores
    /// this without interpreting or decrypting it.
    pub ciphertext: String,
    /// Identifies the ciphertext format and encoding rules. Currently only `1` is
    /// accepted, corresponding to the fixed `[Fr254; 9]` layout. Persisting the
    /// version with each receipt allows safe format evolution in future versions
    /// without breaking existing stored receipts, and is also used during idempotent
    /// replay to detect conflicting re-submissions.
    pub version: u8,
    /// Lifecycle status of the associated transfer transaction. In the current
    /// transfer-receipt flow this is typically refreshed from live transaction state
    /// on read and returned as `Pending` or `IncludedL2`; the enum also retains
    /// `Failed` for compatibility with the broader receipt status model.
    pub status: TransferReceiptStatus,
    /// Unix timestamp (seconds) when this receipt was first stored by the proposer.
    pub created_at_unix: i64,
    /// Unix timestamp (seconds) when this receipt was last updated, for example
    /// when `status` was refreshed after the referenced transaction was included
    /// in a Layer 2 block.
    pub updated_at_unix: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoricRoot(
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")] pub Fr254,
    pub u32,
);

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use serde_json::{from_str, to_string, to_value, Value};

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq, Clone)]
    struct MockProof {
        a: Vec<u8>,
        b: Vec<u8>,
        c: Vec<u8>,
    }

    impl lib::nf_client_proof::Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::from_static(b"mock-proof"))
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self::default())
        }
    }

    #[test]
    fn deserializes_legacy_selected_document_into_lifecycle() {
        let tx = ClientTransactionWithMetaData {
            client_transaction: ClientTransaction::<MockProof>::default(),
            lifecycle: TxLifecycle::Selected { block_l2: 7 },
            hash: vec![1, 2, 3],
            historic_roots: vec![],
            receipt_token: None,
        };
        let mut value = to_value(&tx).expect("serialize tx");
        let map = value
            .as_object_mut()
            .expect("transaction should serialize as an object");
        map.remove("lifecycle");
        map.insert("block_l2".to_string(), Value::from(7u64));
        map.insert("in_mempool".to_string(), Value::from(false));
        map.insert("cancelled_explicitly".to_string(), Value::from(false));

        let serialized = serde_json::to_string(&value).expect("serialize legacy value");
        let deserialized: ClientTransactionWithMetaData<MockProof> =
            from_str(&serialized).expect("deserialize legacy tx");

        assert_eq!(
            deserialized.lifecycle,
            TxLifecycle::Selected { block_l2: 7 }
        );
    }

    #[test]
    fn round_trips_new_lifecycle_document() {
        let tx = ClientTransactionWithMetaData {
            client_transaction: ClientTransaction::<MockProof>::default(),
            lifecycle: TxLifecycle::Cancelled,
            hash: vec![4, 5, 6],
            historic_roots: vec![Fr254::from(9u64)],
            receipt_token: None,
        };

        let value = to_value(&tx).expect("serialize tx");
        let serialized = to_string(&value).expect("serialize json value");
        let deserialized: ClientTransactionWithMetaData<MockProof> =
            from_str(&serialized).expect("deserialize tx");

        assert_eq!(deserialized.lifecycle, TxLifecycle::Cancelled);
        assert_eq!(deserialized.hash, vec![4, 5, 6]);
        assert_eq!(deserialized.historic_roots, vec![Fr254::from(9u64)]);
    }
}
