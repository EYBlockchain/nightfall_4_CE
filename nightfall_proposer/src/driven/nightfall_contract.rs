//! Implementation of the [`NightfallContract`] trait from `nightfall_proposer/src/ports/contracts.rs`.

use crate::{
    domain::entities::Block, initialisation::get_blockchain_client_connection,
    ports::contracts::NightfallContract,
};
use alloy::primitives::I256;
use configuration::addresses::get_addresses;
use lib::blockchain_client::BlockchainClientConnection;
use log::info;
use nightfall_bindings::artifacts::Nightfall;
use nightfall_client::domain::error::NightfallContractError;

#[async_trait::async_trait]
impl NightfallContract for Nightfall::NightfallCalls {
    async fn propose_block(block: Block) -> Result<(), NightfallContractError> {
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();
        let signer = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_signer();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);
        // Convert the block transactions to the Nightfall format
        let blk = Nightfall::Block::from(block);
        let _call =  nightfall
        .propose_block(blk.clone())
        .from(signer.address())
        .call()
        .await.unwrap();

        let receipt = nightfall
            .propose_block(blk)
            .from(signer.address())
            .send()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?
            .get_receipt()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;
        println!("Checking receipt of submitted block...{}", receipt.status());
        info!(
            "Received receipt for submitted block with hash: {}, gas used was: {}",
            receipt.transaction_hash, receipt.gas_used
        );
        Ok(())
    }

    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);

        Ok(nightfall
            .layer2_block_number()
            .call()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?)
    }
}
