use crate::{
    domain::entities::{Block, OnChainTransaction},
    initialisation::get_block_assembly_status,
    ports::block_assembly_trigger::BlockAssemblyTrigger,
};

use async_trait::async_trait;
use ethers::types::Bytes;
use nightfall_bindings::nightfall::{
    Block as NightfallBlockStruct, CompressedSecrets as NightfallCompressedSecrets,
    OnChainTransaction as NightfallOnChainTransaction,
};
use nightfall_client::{
    domain::{entities::ClientTransaction, error::ConversionError},
    driven::contract_functions::contract_type_conversions::{FrBn254, Uint256},
};
use tokio::time::{sleep, Duration};

pub struct IntervalTrigger(pub u64);

// We'll want to replace it with something more sophisticated.
#[async_trait]
impl BlockAssemblyTrigger for IntervalTrigger {
    async fn await_trigger(&self) {
        let mut count = self.0;
        while count > 0 {
            let is_running = get_block_assembly_status().await.read().await.is_running();
            if is_running {
                count -= 1;
            }
            sleep(Duration::from_secs(1)).await;
        }
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
