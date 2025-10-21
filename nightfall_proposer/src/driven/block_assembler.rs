use crate::{
    domain::entities::Block,
    initialisation::get_blockchain_client_connection,
    ports::{block_assembly_trigger::BlockAssemblyTrigger, db::TransactionsDB},
};
use async_trait::async_trait;
use nightfall_bindings::artifacts::{Nightfall, ProposerManager};

use alloy::primitives::Bytes;
use configuration::addresses::get_addresses;
use lib::{
    blockchain_client::BlockchainClientConnection,
    contract_conversions::{FrBn254, Uint256},
    error::ConversionError,
    nf_client_proof::Proof,
    shared_entities::OnChainTransaction,
    utils::get_block_size,
};
use log::{debug, error, warn};
use std::marker::PhantomData;
use tokio::{
    sync::RwLock,
    time::{self, Duration, Instant},
};
/// SmartTrigger is responsible for deciding when to trigger block assembly,
/// based on time constraints and mempool state.
///
/// Parameters
///  `interval_secs`: time between periodic checks of the mempool
///  `max_wait_secs`: maximum time to wait before forcing block assembly
///  `status`: shared state indicating if the block assembly is currently active
///  `db`: handle to the database containing the mempool
///  `target_block_fill_ratio`: threshold used to trigger block creation
///
/// Behavior:
/// - A block is triggered if either:
///     1. The current fill ratio of the block (deposits + txs) reaches `target_block_fill_ratio`
///     2. The `max_wait_secs` duration has passed without meeting the threshold
pub struct SmartTrigger<P: Proof> {
    pub interval_secs: u64,
    pub max_wait_secs: u64,
    pub status: &'static RwLock<BlockAssemblyStatus>,
    pub db: &'static mongodb::Client,
    pub target_block_fill_ratio: f32,
    pub phantom: PhantomData<P>,
}

impl<P: Proof> SmartTrigger<P> {
    pub fn new(
        interval_secs: u64,
        max_wait_secs: u64,
        status: &'static RwLock<BlockAssemblyStatus>,
        db: &'static mongodb::Client,
        target_block_fill_ratio: f32,
    ) -> Self {
        Self {
            interval_secs,
            max_wait_secs,
            status,
            db,
            target_block_fill_ratio,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<P: Proof + Send + Sync> BlockAssemblyTrigger for SmartTrigger<P> {
    async fn await_trigger(&self) {
        let interval_duration = Duration::from_secs(self.interval_secs);
        let mut interval = time::interval(interval_duration);
        let short_wait = Duration::from_secs(10);
        let start = Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs();
            let remaining = self.max_wait_secs.saturating_sub(elapsed);
            // Re-check current proposer
            let blockchain_client = get_blockchain_client_connection()
                .await
                .read()
                .await
                .get_client();
            let round_robin_instance =
                ProposerManager::new(get_addresses().round_robin, blockchain_client.root());

            let current_proposer = match round_robin_instance
                .get_current_proposer_address()
                .call()
                .await
            {
                Ok(addr) => addr,
                Err(e) => {
                    error!("Failed to get current proposer: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            let our_address = get_blockchain_client_connection()
                .await
                .read()
                .await
                .get_address();

            if current_proposer != our_address {
                debug!(
                    "Lost proposer status during trigger wait. Current proposer: {current_proposer:?}"
                );
                break; // Exit loop early
            }
            if self.status.read().await.is_running() {
                if self.should_assemble().await {
                    debug!("Trigger activated by mempool check.");
                    break;
                }
                if elapsed >= self.max_wait_secs {
                    debug!(
                        "Max wait time elapsed ({}s). Triggering block assembly.",
                        self.max_wait_secs
                    );
                    break;
                }
            } else {
                if self.break_pause().await {
                    self.status.write().await.resume();
                    debug!("Block assembly resumed as block is full.");
                    break;
                }
                debug!("Block assembly is currently paused. Waiting...");
            }

            // Log status of trigger wait with dynamic information
            warn!(
            "Not enough transactions to assemble a block yet. Elapsed: {}s, remaining: {}s, will wait for more txs or until timeout ({}s).",
            elapsed,
            remaining,
            self.max_wait_secs
        );

            tokio::select! {
                _ = interval.tick() => {
                    if self.status.read().await.is_running() && self.should_assemble().await {
                        debug!("Trigger activated after interval with fill threshold reached.");
                        break;
                    }
                }
                _ = time::sleep(short_wait) => {
                }
            }
        }
    }
}

impl<P: Proof + Send + Sync> SmartTrigger<P> {
    async fn should_assemble(&self) -> bool {
        let db = self.db;

        let num_deposit_groups =
            match <mongodb::Client as TransactionsDB<P>>::count_mempool_deposits(db).await {
                Ok(count) => {
                    let groups = count.div_ceil(4);
                    debug!("Mempool deposits: {count}, grouped into: {groups}");
                    groups
                }
                Err(e) => {
                    error!("Error counting deposits: {e:?}");
                    0
                }
            } as f32;

        let num_client_txs =
            match <mongodb::Client as TransactionsDB<P>>::count_mempool_client_transactions(db)
                .await
            {
                Ok(count) => {
                    debug!("Mempool client transactions: {count}");
                    debug!("Mempool client transactions: {count}");
                    count
                }
                Err(e) => {
                    error!("Error counting client transactions: {e:?}");
                    0
                }
            } as f32;

        let block_size = match get_block_size() {
            Ok(size) => size as f32,
            Err(e) => {
                log::warn!("Falling back to default block size 64 due to error: {e:?}");
                64.0
            }
        };

        let fill_ratio = (num_deposit_groups + num_client_txs) / block_size;
        debug!(
            "Block size: {}, deposits: {}, client txs: {}, fill_ratio: {}, expected ratio: {}",
            block_size,
            num_deposit_groups,
            num_client_txs,
            fill_ratio,
            self.target_block_fill_ratio
        );
        fill_ratio >= self.target_block_fill_ratio
    }

    async fn break_pause(&self) -> bool {
        let db = self.db;

        let num_deposit_groups =
            match <mongodb::Client as TransactionsDB<P>>::count_mempool_deposits(db).await {
                Ok(count) => {
                    let groups = count.div_ceil(4);
                    debug!("Mempool deposits: {count}, grouped into: {groups}");
                    groups
                }
                Err(e) => {
                    error!("Error counting deposits: {e:?}");
                    0
                }
            } as f32;

        let num_client_txs =
            match <mongodb::Client as TransactionsDB<P>>::count_mempool_client_transactions(db)
                .await
            {
                Ok(count) => {
                    debug!("Mempool client transactions: {count}");
                    count
                }
                Err(e) => {
                    error!("Error counting client transactions: {e:?}");
                    0
                }
            } as f32;

        let block_size = match get_block_size() {
            Ok(size) => size as f32,
            Err(e) => {
                log::warn!("Falling back to default block size 64 due to error: {e:?}");
                64.0
            }
        };
        (num_deposit_groups + num_client_txs) >= block_size
    }
}

pub struct BlockAssemblyStatus(bool);

impl BlockAssemblyStatus {
    pub fn new() -> Self {
        Self(true)
    }

    pub fn pause(&mut self) {
        self.0 = false;
    }

    pub fn resume(&mut self) {
        self.0 = true;
    }

    pub fn is_running(&self) -> bool {
        self.0
    }
}

impl Default for BlockAssemblyStatus {
    fn default() -> Self {
        Self::new()
    }
}

// Converts the Block type used in rust to a struct suitable for the Nightfall solidity contract.
// this will need updating as the NightfallBlockStruct type becomes more complex.
impl From<Block> for Nightfall::Block {
    fn from(blk: Block) -> Self {
        Self {
            rollup_proof: Bytes::from(blk.rollup_proof.clone()),
            commitments_root_root: Uint256::from(blk.commitments_root_root).into(),
            commitments_root: Uint256::from(blk.commitments_root).into(),
            nullifier_root: Uint256::from(blk.nullifiers_root).into(),
            transactions: blk
                .transactions
                .into_iter()
                .map(Nightfall::OnChainTransaction::from)
                .collect(),
        }
    }
}

/// Converts the NF_4 smart contract representation of a block into a Domain struct,
/// containing data type more suited to manipulation in Rust.
impl TryFrom<Nightfall::Block> for Block {
    type Error = ConversionError;
    fn try_from(nblk: Nightfall::Block) -> Result<Self, Self::Error> {
        Ok(Self {
            commitments_root: FrBn254::try_from(nblk.commitments_root)?.into(),
            nullifiers_root: FrBn254::try_from(nblk.nullifier_root)?.into(),
            commitments_root_root: FrBn254::try_from(nblk.commitments_root_root)?.into(),
            transactions: nblk
                .transactions
                .into_iter()
                .map(OnChainTransaction::from)
                .collect::<Vec<OnChainTransaction>>(),
            rollup_proof: nblk.rollup_proof.to_vec(),
        })
    }
}
