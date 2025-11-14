//! Implementation of the [`NightfallContract`] trait from `nightfall_proposer/src/ports/contracts.rs`.

use crate::{
    domain::entities::Block, initialisation::get_blockchain_client_connection,
    ports::contracts::NightfallContract,
};
use alloy::primitives::I256;
use configuration::{addresses::get_addresses, settings::get_settings};
use lib::{
    blockchain_client::BlockchainClientConnection, error::NightfallContractError,
    verify_contract::VerifiedContracts,
};
use log::info;
use nightfall_bindings::artifacts::Nightfall;

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
        let verified =
            VerifiedContracts::resolve_and_verify_contract(client.clone(), get_addresses())
                .await
                .map_err(|e| {
                    NightfallContractError::ContractVerificationError(format!(
                        "Contract verification failed during get_token_info: {e}"
                    ))
                })?;
        let nightfall = verified.nightfall;

        // Convert the block transactions to the Nightfall format
        let blk: Nightfall::Block = block.into();
        let nonce = blockchain_client
            .get_transaction_count(signer.address())
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;
        let gas_price = blockchain_client
            .get_gas_price()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;
        let gas_limit = 5000000u64;

        let raw_tx = nightfall
            .propose_block(blk)
            .nonce(nonce)
            .gas(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
            .build_raw_transaction(signer)
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;

        let receipt = blockchain_client
            .send_raw_transaction(&raw_tx)
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
        let verified =
            VerifiedContracts::resolve_and_verify_contract(client.clone(), get_addresses())
                .await
                .map_err(|e| {
                    NightfallContractError::ContractVerificationError(format!(
                        "Contract verification failed during get_current_layer2_blocknumber: {e}"
                    ))
                })?;
        let nightfall = verified.nightfall;
        Ok(nightfall
            .layer2_block_number()
            .call()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?)
    }
}
