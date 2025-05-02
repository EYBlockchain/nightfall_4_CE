use crate::{
    domain::entities::{Block, OnChainTransaction},
    ports::block_assembly_trigger::BlockAssemblyTrigger,
};
use nightfall_client::ports::proof::Proof;
use crate::services::assemble_block::get_block_size;
use crate::ports::db::TransactionsDB;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio::time;
use ethers::types::Bytes;
use nightfall_bindings::nightfall::{
    Block as NightfallBlockStruct, CompressedSecrets as NightfallCompressedSecrets,
    OnChainTransaction as NightfallOnChainTransaction,
};
use nightfall_client::{
    domain::{entities::ClientTransaction, error::ConversionError},
    driven::contract_functions::contract_type_conversions::{FrBn254, Uint256},
};
use tokio::time::{Instant, Duration};
use std::marker::PhantomData;
use log::info as tracing_info;
use log::error;


pub struct SmartTrigger<P: Proof> {
    pub interval_secs: u64,
    pub max_wait_secs: u64,
    pub status: &'static RwLock<BlockAssemblyStatus>,
    pub db: &'static RwLock<mongodb::Client>,
    pub target_block_fill_ratio: f32,
    pub phantom: PhantomData<P>,
}

impl<P: Proof> SmartTrigger<P> {
    pub fn new(
        interval_secs: u64,
        max_wait_secs: u64,
        status: &'static RwLock<BlockAssemblyStatus>,
        db: &'static RwLock<mongodb::Client>, 
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
        let short_wait = Duration::from_secs(5); 

        let start = Instant::now();

        loop {
            if self.status.read().await.is_running() && self.should_assemble().await {
                tracing_info!("Trigger activated by mempool check.");
                break;
            }

            if start.elapsed().as_secs() >= self.max_wait_secs {
                tracing_info!(
                    "Max wait time elapsed ({}s). Triggering block assembly.",
                    self.max_wait_secs
                );
                break;
            }

            tokio::select! {
                _ = interval.tick() => {
                    tracing_info!("Interval elapsed, checking threshold.");
                    if self.status.read().await.is_running() && self.should_assemble().await {
                        tracing_info!("Trigger activated after interval with fill threshold reached.");
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
        let mut db = self.db.write().await;

       
        // let num_deposit_groups = match <mongodb::Client as TransactionsDB<P>>::count_mempool_deposits(&mut db).await {
        //     Ok(count) => (count + 3) / 4,
        //     Err(e) => {
        //         error!("Error counting deposits: {:?}", e);
        //         0 
        //     }
        // } as f32;

        // let num_client_txs = match <mongodb::Client as TransactionsDB<P>>::count_mempool_transactions(&mut db).await {
        //     Ok(count) => count as f32,
        //     Err(e) => {
        //         error!("Error counting client transactions: {:?}", e);
        //         0.0
        //     }
        // };
        let num_deposit_groups = match <mongodb::Client as TransactionsDB<P>>::count_mempool_deposits(&mut db).await {
            Ok(count) => {
                let groups = (count + 3) / 4;
                tracing_info!("Mempool deposits: {}, grouped into: {}", count, groups);
                groups
            }
            Err(e) => {
                error!("Error counting deposits: {:?}", e);
                0
            }
        } as f32;

        let num_client_txs = match <mongodb::Client as TransactionsDB<P>>::count_mempool_transactions(&mut db).await {
            Ok(count) => {
                tracing_info!("Mempool client transactions: {}", count);
                count as f32
            }
            Err(e) => {
                error!("Error counting client transactions: {:?}", e);
                0.0
            }
        };

        let block_size = get_block_size().unwrap_or(64) as f32;
        // let num_deposit_groups = deposits.as_ref().map_or(0, |d| (d.len() + 3) / 4) as f32;
        // let num_client_txs = client_txs.as_ref().map_or(0, |txs| txs.len()) as f32;
        let fill_ratio = (num_deposit_groups + num_client_txs) / block_size;
        

        tracing_info!("Calculated fill ratio: {:.2}", fill_ratio);

        fill_ratio >= self.target_block_fill_ratio
        

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
impl From<Block> for NightfallBlockStruct {
    fn from(blk: Block) -> Self {
        Self {
            rollup_proof: Bytes::from(blk.rollup_proof.clone()),
            commitments_root_root: Uint256::from(blk.commitments_root_root).into(),
            commitments_root: Uint256::from(blk.commitments_root).into(),
            nullifier_root: Uint256::from(blk.nullifiers_root).into(),
            transactions: blk
                .transactions
                .into_iter()
                .map(NightfallOnChainTransaction::from)
                .collect(),
        }
    }
}

/// Converts the Domain representation of an onchain transaction (i.e. one that is rolled up into a block)
/// into one suitable for interacting with the smart contract
impl From<OnChainTransaction> for NightfallOnChainTransaction {
    fn from(otx: OnChainTransaction) -> Self {
        Self {
            fee: Uint256::from(otx.fee).into(),
            commitments: otx.commitments.map(|c| Uint256::from(c).into()),
            nullifiers: otx.nullifiers.map(|n| Uint256::from(n).into()),
            public_data: NightfallCompressedSecrets::from(otx.public_data).cipher_text,
        }
    }
}

/// Converts the NF_4 smart contract representation of an on-chain transaction (i.e. a transaction that is
/// rolled up into a block), into a form more sutiable for manipulation in Rust.
impl From<NightfallOnChainTransaction> for OnChainTransaction {
    fn from(ntx: NightfallOnChainTransaction) -> Self {
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
            public_data: NightfallCompressedSecrets {
                cipher_text: ntx.public_data,
            }
            .into(),
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

/// Converts the NF_4 smart contract representation of a block into a Domain struct,
/// containing data type more suited to manipulation in Rust.
impl TryFrom<NightfallBlockStruct> for Block {
    type Error = ConversionError;
    fn try_from(nblk: NightfallBlockStruct) -> Result<Self, Self::Error> {
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
