//! Implementation of the [`NightfallContract`] trait from `nightfall_proposer/src/ports/contracts.rs`.

use crate::{
    domain::entities::Block, initialisation::get_blockchain_client_connection,
    ports::contracts::NightfallContract,
};
use alloy::{primitives::I256, providers::Provider};
use configuration::{addresses::get_addresses, settings::get_settings};
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
        let blk: Nightfall::Block = block.into();

        let nonce = client.get_transaction_count(signer.address()).await.map_err(|e| {
            NightfallContractError::EscrowError(format!("Transaction unsuccesful: {e}"))
        })?;
        let gas_price = client.get_gas_price().await.map_err(|e| {
            NightfallContractError::EscrowError(format!("Transaction unsuccesful: {e}"))
        })?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;
        let gas_limit = 500000000u64;
        let call = nightfall
            .propose_block(blk)
            .nonce(nonce)
            .gas(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
            .build_raw_transaction(signer).await
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Transaction unsuccesful: {e}"))
            })?;

            let receipt = client.send_raw_transaction(&call)
            .await
            .map_err(|_| NightfallContractError::TransactionError)?
            .get_receipt()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;
    
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
