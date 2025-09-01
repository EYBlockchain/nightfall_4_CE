//! Implementation of the [`NightfallContract`] trait from `nightfall_proposer/src/ports/contracts.rs`.

use crate::{
    domain::entities::Block, initialisation::get_blockchain_client_connection,
    ports::contracts::NightfallContract,
};
use configuration::addresses::get_addresses;
use ethers::types::{I256, U256};
use lib::blockchain_client::BlockchainClientConnection;
use log::info;
use nightfall_bindings::nightfall::{Block as NightfallBlock, Nightfall};
use nightfall_client::domain::error::NightfallContractError;

#[async_trait::async_trait]
impl<M> NightfallContract for Nightfall<M> {
    async fn propose_block(block: Block) -> Result<(), NightfallContractError> {
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);

        let blk = NightfallBlock::from(block);
        let receipt = nightfall
            .propose_block(blk)
            .send()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;

        match receipt {
            Some(receipt) => {
                info!(
                    "Received receipt for submitted block with hash: {}, gas used was: {}",
                    receipt.transaction_hash,
                    receipt.gas_used.unwrap_or_else(U256::zero)
                );
            }
            None => info!("No receipt received for submitted block"),
        }

        Ok(())
    }

    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);

        Ok(nightfall
            .layer_2_block_number()
            .call()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?)
    }
}
