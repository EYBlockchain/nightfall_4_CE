use crate::{
    domain::entities::{ClientTransactionWithMetaData, DepositDatawithFee, HistoricRoot, Node},
    driven::db::mongo_db::StoredBlock,
};
use ark_bn254::Fr as Fr254;
use ark_ff::PrimeField;
use lib::serialization::{ark_de_bytes, ark_se_bytes};
use nightfall_client::domain::entities::ClientTransaction;
use serde::{Deserialize, Serialize};
#[async_trait::async_trait]
pub trait BlockStorageDB {
    async fn store_block(&self, block: &StoredBlock) -> Option<()>;
    async fn get_block_by_number(&self, block_number: u64) -> Option<StoredBlock>;
    async fn get_all_blocks(&self) -> Option<Vec<StoredBlock>>;
    async fn delete_block_by_number(&self, block_number: u64) -> Option<()>;
}
/// Used to store transactions that are on chain. Can be queried to see if a nullifier or commitment is on chain.
#[async_trait::async_trait]
pub trait TransactionsDB<'a, P> {
    async fn store_transaction(&self, transaction: ClientTransactionWithMetaData<P>) -> Option<()>;
    async fn get_transaction(&self, key: &'a [u32]) -> Option<ClientTransactionWithMetaData<P>>;
    async fn get_all_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>>;
    async fn get_all_mempool_client_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>>;
    async fn count_mempool_client_transactions(&self) -> Result<u64, mongodb::error::Error>;
    async fn is_transaction_in_mempool(&self, key: &'a [u32]) -> bool;
    async fn check_nullifier(&self, nullifier: Fr254) -> bool;
    async fn check_commitment(&self, commitment: Fr254) -> bool;
    async fn update_commitment<M>(&self, mutator: M, key: &'a [u32]) -> Option<()>
    where
        M: Fn(&ClientTransactionWithMetaData<P>) -> ClientTransactionWithMetaData<P> + Send;
    async fn set_in_mempool(
        &self,
        transactions: &[ClientTransactionWithMetaData<P>],
        in_mempool: bool,
    ) -> Option<u64>;
    async fn find_transaction(
        &self,
        tx: &ClientTransaction<P>,
    ) -> Option<ClientTransactionWithMetaData<P>>;
    async fn find_deposit(&self, tx: &DepositDatawithFee) -> Option<DepositDatawithFee>;
    async fn set_mempool_deposits(&self, deposits: Vec<DepositDatawithFee>) -> Option<u64>;
    async fn get_mempool_deposits(&self) -> Option<Vec<DepositDatawithFee>>;
    async fn count_mempool_deposits(&self) -> Result<u64, mongodb::error::Error>;
    async fn remove_mempool_deposits(
        &self,
        used_deposits: Vec<Vec<DepositDatawithFee>>,
    ) -> Option<u64>;
    async fn remove_all_mempool_deposits(&self) -> Option<u64>;
    async fn remove_all_mempool_client_transactions(&self) -> Option<u64>;
}

/// A database that stores historic roots of the commitments Merkle tree.
#[async_trait::async_trait]
pub trait HistoricRootsDB {
    async fn store_historic_root(&mut self, historic_root: &HistoricRoot) -> Option<()>;
    async fn get_historic_root(&mut self, k: &Fr254) -> Option<HistoricRoot>;
}

/// A struct that represents an entry in the nullifier database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NullifierEntry {
    /// The value of this nullifier
    #[serde(serialize_with = "ark_se_bytes", deserialize_with = "ark_de_bytes")]
    pub value: Fr254,
    /// The index of the next highest value nullifier in the Indexed Merkle tree
    pub next_index: u32,
    /// The value of the next highest value nullifier in the Indexed Merkle tree
    #[serde(serialize_with = "ark_se_bytes", deserialize_with = "ark_de_bytes")]
    pub next_value: Fr254,
    /// The leaf value of this nullifier in the Indexed Merkle tree
    #[serde(serialize_with = "ark_se_bytes", deserialize_with = "ark_de_bytes")]
    pub key: Fr254,
}

/// Struct storing relevant information about a nullifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NullifierDBEntry<F> {
    /// The value of this nullifier
    pub value: F,
    /// The index of the leaf representing this nullifier in the Indexed Merkle tree
    pub index: u32,
    /// The index of the next highest value nullifier in the Indexed Merkle tree
    pub next_index: F,
    /// The value of the next highest value nullifier in the Indexed Merkle tree
    pub next_value: F,
}

impl<F: PrimeField> Default for NullifierDBEntry<F> {
    fn default() -> Self {
        Self {
            value: F::zero(),
            index: 0,
            next_index: F::zero(),
            next_value: F::zero(),
        }
    }
}

impl<F: PrimeField> NullifierDBEntry<F> {
    /// Creates a new instance of the struct.
    pub fn new(value: F, index: u32, next_index: F, next_value: F) -> Self {
        Self {
            value,
            index,
            next_index,
            next_value,
        }
    }
}

/// A database that stores nullifiers of spent commitments.
///
/// These are stored in the format they appear in the Indexed Merkle tree, that is
/// the "preimage" of the nullifier is stored in the form {nullifier_value, index, next_value} and
/// the key is a hash of this.
pub trait NullifierDB<F> {
    /// Creates a new instance of the database. We have to do this because we need to insert the zero nullifier.
    fn new() -> Self;
    /// Stores a nullifier in the database. This functions works out the low nullifier.
    fn store_nullifier(&mut self, nullifier: F) -> Option<()>;
    /// Searches the database for a nullifier with the supplied fields. If it finds one, it returns it.
    fn get_nullifier(
        &self,
        value: Option<F>,
        next_value: Option<F>,
    ) -> Option<&NullifierDBEntry<F>>;
    /// Searches the database for the nullifier that skips over the supplied value. That is finds the nullifier such that
    /// `low_nullifier.value` < `nullifier_value` < `low_nullifier.next_value`. If it finds one, it returns it.
    fn get_low_nullifier(&self, nullifier_value: &F) -> Option<&NullifierDBEntry<F>>;
    /// Updates the nullifier entry stored with value `nullifier` with the new `next_value`.
    fn update_nullifier(
        &mut self,
        nullifier: F,
        new_next_index: F,
        new_next_value: F,
    ) -> Option<()>;
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
