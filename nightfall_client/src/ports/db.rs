use crate::domain::entities::{CommitmentStatus, Node, Preimage};

use super::{
    commitments::Commitment,
    keys::{ProvingKey, VerifyingKey},
};

use ark_bn254::Fr as Fr254;
use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait CommitmentDB<K, V>
where
    V: CommitmentEntryDB,
{
    async fn store_commitment(&self, commitment_entry: V) -> Option<()>;
    async fn store_commitments(&self, commitment_entries: &[V], dup_key_check: bool) -> Option<()>;
    async fn get_all_commitments(&self) -> Result<Vec<(K, V)>, mongodb::error::Error>;
    async fn get_commitment(&self, k: &K) -> Option<V>;
    async fn get_balance(&self, k: &K) -> Option<Fr254>;
    async fn get_available_commitments(&self, nf_token_id: Fr254) -> Option<Vec<V>>;
    async fn mark_commitments_pending_spend(&self, commitments: Vec<K>) -> Option<()>;
    async fn mark_commitments_pending_creation(&self, commitments: Vec<K>) -> Option<()>;
    async fn mark_commitments_unspent(&self, commitments: &[K]) -> Option<()>;
    async fn mark_commitments_spent(&self, nullifiers: Vec<K>) -> Option<()>;
    async fn add_nullifier(&self, key: &K, nullifier: K) -> Option<()>;
}

pub trait CommitmentEntryDB: Commitment {
    fn new(preimage: Preimage, key: Fr254, nullifier: Fr254, status: CommitmentStatus) -> Self;
    fn get_status(&self) -> CommitmentStatus;
}

#[async_trait]
pub trait CircuitKeyDB<K, V>
where
    V: KeyEntryDB,
{
    async fn store_key(&mut self, compressed_proving_key: V) -> Option<()>;
    async fn get_key(&mut self, k: &K) -> Option<V>;
}

pub trait KeyEntryDB: ProvingKey + VerifyingKey {}
/// This is useful because Mongo serialises things to hex
pub trait ToHexString {
    fn to_hex(&self) -> String;
}

/// Trait for a DB storing a Merkle tree
#[async_trait::async_trait]
pub trait MerkleTreeDB<F> {
    type Error;
    async fn update_nodes(
        &mut self,
        tree_name: &str,
        nodes: Vec<Node<F>>,
    ) -> Result<(), Self::Error>;
    async fn read_node_values(
        &self,
        tree_name: &str,
        indices: Vec<usize>,
    ) -> Result<Vec<F>, Self::Error>;
    async fn new_tree(&mut self, tree_height: u32, tree_name: &str) -> Result<(), Self::Error>;
    async fn get_tree_height(&self, tree_name: &str) -> Result<u32, Self::Error>;
}

/// Database used to store pending withdrawals when they appear on chain.
pub trait WithdrawalDB<K, V> {
    /// Store a pending withdrawal
    fn store_withdrawal(&mut self, data: V) -> impl Future<Output = Option<()>> + Send;
    /// Retrieves all pending withdrawals
    fn get_pending_withdrawals(&self) -> impl Future<Output = Option<Vec<V>>> + Send;
    /// Removes a pending withdrawal from the database
    fn remove_withdrawal(&mut self, key: K) -> impl Future<Output = Option<()>> + Send;
}
