//! Implementation of the [`NightfallContract`] trait from `nightfall_proposer/src/ports/contracts.rs`.

use crate::{
    domain::entities::Block,
    initialisation::get_blockchain_client_connection,
    ports::contracts::{
        BroadcastUnknownReason, NightfallContract, NotBroadcastReason, ProposeBlockOutcome,
    },
};
use alloy::{
    primitives::{TxHash, I256},
    providers::Provider,
};
use configuration::{addresses::get_addresses, settings::get_settings};
use lib::{
    blockchain_client::BlockchainClientConnection, error::NightfallContractError,
    verify_contract::VerifiedContracts,
};
use log::info;
use nightfall_bindings::artifacts::Nightfall;

#[async_trait::async_trait]
impl NightfallContract for Nightfall::NightfallCalls {
    async fn propose_block(block: Block) -> Result<ProposeBlockOutcome, NightfallContractError> {
        let read_connection = get_blockchain_client_connection().await.read().await;
        let blockchain_client = read_connection.get_client();
        let client = blockchain_client.root();
        let caller = read_connection.get_address();
        let wallet = read_connection.get_wallet_type().clone();
        let verified =
            match VerifiedContracts::resolve_and_verify_contract(client.clone(), get_addresses())
                .await
            {
                Ok(verified) => verified,
                Err(_) => {
                    return Ok(ProposeBlockOutcome::NotBroadcast {
                        reason: NotBroadcastReason::TransactionPreparationFailed,
                    });
                }
            };
        let nightfall = verified.nightfall;

        // Convert the block transactions to the Nightfall format
        let blk: Nightfall::Block = block.into();
        let nonce = match blockchain_client.get_transaction_count(caller).await {
            Ok(nonce) => nonce,
            Err(_) => {
                return Ok(ProposeBlockOutcome::NotBroadcast {
                    reason: NotBroadcastReason::TransactionPreparationFailed,
                });
            }
        };
        let gas_price = match blockchain_client.get_gas_price().await {
            Ok(gas_price) => gas_price,
            Err(_) => {
                return Ok(ProposeBlockOutcome::NotBroadcast {
                    reason: NotBroadcastReason::TransactionPreparationFailed,
                });
            }
        };
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;
        let gas_limit = 5000000u64;

        let raw_tx = match nightfall
            .propose_block(blk)
            .nonce(nonce)
            .gas(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id)
            .build_raw_transaction(wallet)
            .await
        {
            Ok(raw_tx) => raw_tx,
            Err(_) => {
                return Ok(ProposeBlockOutcome::NotBroadcast {
                    reason: NotBroadcastReason::TransactionPreparationFailed,
                });
            }
        };

        let pending_tx = match blockchain_client.send_raw_transaction(&raw_tx).await {
            Ok(pending_tx) => pending_tx,
            Err(_) => {
                return Ok(ProposeBlockOutcome::NotBroadcast {
                    reason: NotBroadcastReason::SendRawTransactionFailed,
                });
            }
        };
        let tx_hash = *pending_tx.tx_hash();

        let receipt = match pending_tx.get_receipt().await {
            Ok(receipt) => receipt,
            Err(_) => {
                return Ok(ProposeBlockOutcome::BroadcastUnknown {
                    tx_hash,
                    reason: BroadcastUnknownReason::ReceiptUnavailable,
                });
            }
        };
        info!(
            "The L2 block was sent to L1. Received receipt for submitted block with hash: {}, gas used was: {}",
            receipt.transaction_hash, receipt.gas_used
        );
        if receipt.status() {
            Ok(ProposeBlockOutcome::Submitted {
                tx_hash: receipt.transaction_hash,
            })
        } else {
            Ok(ProposeBlockOutcome::Reverted {
                tx_hash: receipt.transaction_hash,
            })
        }
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

    async fn get_proposal_receipt_status(
        tx_hash: TxHash,
    ) -> Result<Option<bool>, NightfallContractError> {
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();
        let receipt = client
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(|e| NightfallContractError::ProviderError(e.to_string()))?;

        Ok(receipt.map(|receipt| receipt.status()))
    }
}
