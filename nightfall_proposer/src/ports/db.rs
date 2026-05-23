use crate::{
    domain::entities::{
        ClientTransactionWithMetaData, DepositDatawithFee, HistoricRoot, PendingBlock,
        RestoreJournal, SyncState, TransferReceipt, TransferReceiptStatus, TxHashBytes,
    },
    driven::db::mongo_db::StoredBlock,
};
use ark_bn254::Fr as Fr254;
use ark_ff::PrimeField;
use lib::{
    serialization::{ark_de_bytes, ark_se_bytes},
    shared_entities::{ClientTransaction, Node},
};
use serde::{Deserialize, Serialize};
#[async_trait::async_trait]
pub trait BlockStorageDB {
    async fn store_block(&self, block: &StoredBlock) -> Option<()>;
    async fn store_block_with_session(
        &self,
        block: &StoredBlock,
        _session: &mut mongodb::ClientSession,
    ) -> Result<(), mongodb::error::Error> {
        self.store_block(block)
            .await
            .ok_or_else(|| mongodb::error::Error::custom("Could not store proposed block"))
    }
    async fn get_block_by_number(&self, block_number: u64) -> Option<StoredBlock>;
    async fn get_all_blocks(&self) -> Option<Vec<StoredBlock>>;
    async fn delete_block_by_number(&self, block_number: u64) -> Option<()>;
    async fn delete_all_blocks(&self) -> Option<u64> {
        let blocks = self.get_all_blocks().await?;
        let mut deleted = 0_u64;
        for block in blocks {
            self.delete_block_by_number(block.layer2_block_number)
                .await?;
            deleted += 1;
        }
        Some(deleted)
    }
    async fn delete_block_by_number_with_session(
        &self,
        block_number: u64,
        _session: &mut mongodb::ClientSession,
    ) -> Result<(), mongodb::error::Error> {
        self.delete_block_by_number(block_number)
            .await
            .ok_or_else(|| mongodb::error::Error::custom("Could not delete proposed block"))
    }
    async fn delete_all_blocks_with_session(
        &self,
        session: &mut mongodb::ClientSession,
    ) -> Result<u64, mongodb::error::Error> {
        let blocks = self
            .get_all_blocks()
            .await
            .ok_or_else(|| mongodb::error::Error::custom("Could not list proposed blocks"))?;
        let mut deleted = 0_u64;
        for block in blocks {
            self.delete_block_by_number_with_session(block.layer2_block_number, session)
                .await?;
            deleted += 1;
        }
        Ok(deleted)
    }
}

#[async_trait::async_trait]
pub trait SyncStateDB {
    async fn update_sync_state_with_session(
        &self,
        state: &SyncState,
        session: &mut mongodb::ClientSession,
    ) -> Result<(), mongodb::error::Error>;

    async fn get_sync_state(&self) -> Option<SyncState>;

    async fn delete_sync_state_with_session(
        &self,
        _session: &mut mongodb::ClientSession,
    ) -> Result<(), mongodb::error::Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait RestoreJournalDB {
    async fn upsert_restore_journal(
        &self,
        journal: &RestoreJournal,
    ) -> Result<(), mongodb::error::Error>;

    async fn get_restore_journal(&self) -> Option<RestoreJournal>;

    async fn delete_restore_journal(&self) -> Result<(), mongodb::error::Error>;
}

#[async_trait::async_trait]
pub trait PendingBlockDB {
    async fn store_pending_block(&self, pending_block: &PendingBlock) -> Option<()>;
    async fn get_pending_block(&self, block_number: u64) -> Option<PendingBlock>;
    async fn get_all_pending_blocks(&self) -> Option<Vec<PendingBlock>>;
    async fn delete_pending_block(&self, block_number: u64) -> Option<()>;
    async fn delete_all_pending_blocks(&self) -> Option<u64> {
        let pending_blocks = self.get_all_pending_blocks().await?;
        let mut deleted = 0_u64;
        for pending_block in pending_blocks {
            self.delete_pending_block(pending_block.layer2_block_number)
                .await?;
            deleted += 1;
        }
        Some(deleted)
    }
    async fn delete_all_pending_blocks_with_session(
        &self,
        session: &mut mongodb::ClientSession,
    ) -> Result<u64, mongodb::error::Error> {
        let pending_blocks = self
            .get_all_pending_blocks()
            .await
            .ok_or_else(|| mongodb::error::Error::custom("Could not list pending blocks"))?;
        let mut deleted = 0_u64;
        for pending_block in pending_blocks {
            self.delete_pending_block(pending_block.layer2_block_number)
                .await
                .ok_or_else(|| mongodb::error::Error::custom("Could not delete pending block"))?;
            deleted += 1;
        }
        let _ = session;
        Ok(deleted)
    }
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
    async fn get_all_selected_client_transactions(
        &self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>>;
    async fn count_mempool_client_transactions(&self) -> Result<u64, mongodb::error::Error>;
    async fn count_mempool_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error>;
    async fn count_selected_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error>;
    async fn count_cancelled_swap_transactions(
        &self,
        swap_link: &Fr254,
    ) -> Result<u64, mongodb::error::Error>;
    async fn cancel_mempool_swap_transactions(&self, swap_link: &Fr254) -> Option<u64>;
    async fn mark_transactions_selected_for_block(
        &self,
        transactions: &[ClientTransactionWithMetaData<P>],
        block_l2: u64,
    ) -> Option<u64>;
    async fn restore_transactions_to_mempool(
        &self,
        transactions: &[ClientTransactionWithMetaData<P>],
        block_l2: u64,
    ) -> Option<u64>;
    async fn mark_transactions_included_by_hashes(
        &self,
        transaction_hashes: &[Vec<u32>],
    ) -> Option<u64>;
    async fn drop_transactions(
        &self,
        transactions: &[ClientTransactionWithMetaData<P>],
    ) -> Option<u64>;
    async fn set_client_transactions_in_mempool_by_hashes(
        &self,
        transaction_hashes: &[Vec<u32>],
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
    async fn set_mempool_deposits_reserved(
        &self,
        deposits: Vec<Vec<DepositDatawithFee>>,
        reserved: bool,
    ) -> Option<u64>;
    async fn clear_all_mempool_deposit_reservations(&self) -> Option<u64>;
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

/// Error type for transfer receipt storage operations.
#[derive(Debug)]
pub enum TransferReceiptStoreError {
    /// Duplicate unique key (receipt_id or tx_hash already exists).
    DuplicateKey,
    /// Any other storage error.
    Other(String),
}

/// Trait for a DB that stores and retrieves transfer receipts.
#[async_trait::async_trait]
pub trait TransferReceiptDB {
    async fn store_transfer_receipt(
        &self,
        receipt: TransferReceipt,
    ) -> Result<(), TransferReceiptStoreError>;

    async fn get_transfer_receipt(&self, receipt_id: &str) -> Option<TransferReceipt>;

    async fn get_transfer_receipt_by_tx_hash(
        &self,
        tx_hash: &TxHashBytes,
    ) -> Option<TransferReceipt>;

    async fn set_transfer_receipt_status(
        &self,
        receipt_id: &str,
        status: TransferReceiptStatus,
        updated_at_unix: i64,
    ) -> Option<()>;
}
