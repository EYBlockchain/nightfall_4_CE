use ark_bn254::Fr as Fr254;
use ark_serialize::SerializationError;
use lib::{
    serialization::{ark_de_hex, ark_se_hex},
    shared_entities::DepositData,
    shared_entities::{ClientTransaction, OnChainTransaction},
};
use log::error;
use serde::{Deserialize, Deserializer, Serialize};
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
    #[serde(default)]
    pub block_number: u64,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum TxLifecycle {
    Mempool,
    Selected { block_l2: u64 },
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
            _ => None,
        }
    }

    pub fn is_mempool(&self) -> bool {
        matches!(self, Self::Mempool)
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected { .. })
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
        })
    }
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
