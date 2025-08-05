//! Implementation of the [`NightfallContract`] trait from `ports/contracts.rs`.
use super::contract_type_conversions::{Addr, FrBn254, Uint256};
use crate::{
    domain::{
        entities::{DepositSecret, TokenData, TokenType, WithdrawData},
        error::NightfallContractError,
    },
    drivers::rest::utils::to_nf_token_id_from_solidity,
    ports::{contracts::NightfallContract, secret_hash::SecretHash},
};
use alloy::primitives::{keccak256, Address, B256, I256};
use alloy::rpc::types::Filter;
use alloy::{
    consensus::Transaction,
    dyn_abi::abi::encode,
    providers::Provider,
    sol_types::{SolInterface, SolValue},
};
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use ark_std::Zero;
use configuration::addresses::get_addresses;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::{debug, info};
use nightfall_bindings::artifacts::{ERC20Mock, Nightfall, IERC3525};
use num::BigUint;

impl NightfallContract for Nightfall::NightfallCalls {
    async fn escrow_funds(
        token_erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
        fee: Fr254,
        deposit_fee: Fr254,
        secret_preimage: DepositSecret,
        token_type: TokenType,
    ) -> Result<[Fr254; 2], NightfallContractError> {
        // Make DepositData
        let solidity_fee = Uint256::from(fee);
        let solidity_erc_address = get_addresses().nightfall();
        let solidity_token_address = Addr::try_from(token_erc_address)?;
        let solidity_value = Uint256::from(value);
        let solidity_token_id = Uint256::from(token_id);
        let solidity_secret_hash = Uint256::from(secret_preimage.hash()?);

        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();

        let contract = Nightfall::new(solidity_erc_address, client.clone());

        // A deposit transaction (value_1, fee_1, deposit_fee_1), means we want to deposit value_1, with fee_1 paid to proposer, and additional deposit_fee_1 for future transactions. Two deposit data are created for a single deposit transaction:
        // 1: (value: value_1, fee: fee_1),
        // 2: (value: deposit_fee_1, fee: fee_1)
        // therefore, the total deposit fee (msg.value) is 2 * fee_1 + deposit_fee_1
        // If a deposit transaction doesn't have a deposit fee, then the total deposit fee is equal to the fee_1
        let total_fee = if deposit_fee == Fr254::zero() {
            fee
        } else {
            fee + fee + deposit_fee
        };

        let call = contract
            .escrow_funds(
                solidity_fee.0,
                solidity_token_address.0,
                solidity_token_id.0,
                solidity_value.0,
                solidity_secret_hash.0,
                token_type.into(),
            )
            .value(Uint256::from(total_fee).0);

        let receipt = call
            .send()
            .await
            .map_err(|e| {
                if e.as_revert_data().is_some() {
                    format!(
                        "Revert when calling escrow: {:?}",
                        e.as_decoded_error::<ERC20Mock::ERC20InsufficientBalance>()
                    )
                } else {
                    format!("Contract error: {e}")
                }
            })
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Error getting receipt: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Transaction unsuccesful: {e}"))
            })?;

        info!("Gas used in escrow funds: {:?}", receipt.gas_used);
        let slot_id = if let TokenType::ERC3525 = token_type {
            let erc_contract = IERC3525::new(solidity_token_address.0, client.clone());
            erc_contract
                .slotOf(solidity_token_id.0)
                .call()
                .await
                .map_err(|_| {
                    NightfallContractError::EscrowError(
                        "Could not retrieve ERC3525 slot".to_string(),
                    )
                })?
        } else {
            solidity_token_id.0
        };

        // We calculate the the nf_token_id and nf_slot_id here
        let erc_token = solidity_token_address.0.tokenize();
        let nf_token_id =
            to_nf_token_id_from_solidity(solidity_token_address.0, solidity_token_id.0);
        if slot_id == solidity_token_id.0 {
            let nf_slot_id = nf_token_id;
            Ok([nf_token_id, nf_slot_id])
        } else {
            let slot_id_token = slot_id.tokenize();
            let nf_slot_id_biguint =
                BigUint::from_bytes_be(keccak256(encode(&(erc_token, slot_id_token))).as_slice())
                    >> 4;
            let nf_slot_id = Fr254::from(nf_slot_id_biguint);
            Ok([nf_token_id, nf_slot_id])
        }
    }

    fn get_address() -> Fr254 {
        FrBn254::from(get_addresses().nightfall()).0
    }

    async fn de_escrow_funds(
        withdraw_data: WithdrawData,
        token_type: TokenType,
    ) -> Result<(), NightfallContractError> {
        let data = Nightfall::WithdrawData::from(withdraw_data);
        let decode_data = Nightfall::WithdrawData {
            nf_token_id: data.nf_token_id,
            recipient_address: data.recipient_address,
            value: data.value,
            withdraw_fund_salt: data.withdraw_fund_salt,
        };
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();

        let contract = Nightfall::new(get_addresses().nightfall(), client.clone());

        let call = contract.descrow_funds(decode_data, token_type.into());

        let receipt = call
            .send()
            .await
            .map_err(|e| {
                if e.as_revert_data().is_some() {
                    format!(
                        "Revert when calling escrow: {:?}",
                        e.as_decoded_error::<ERC20Mock::ERC20InsufficientBalance>()
                    )
                } else {
                    format!("Contract error: {e}")
                }
            })
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Error getting receipt: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Transaction unsuccesful: {e}"))
            })?;

        if !receipt.gas_used.is_zero() {
            info!("Gas used in de_escrow_funds: {:?}", receipt.gas_used);
        }
        Ok(())
    }

    async fn withdraw_available(
        withdraw_data: WithdrawData,
    ) -> Result<bool, NightfallContractError> {
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();
        let nightfall_instance = Nightfall::new(get_addresses().nightfall(), client.clone());

        let data = Nightfall::WithdrawData::from(withdraw_data);
        let decode_data = Nightfall::WithdrawData {
            nf_token_id: data.nf_token_id,
            recipient_address: data.recipient_address,
            value: data.value,
            withdraw_fund_salt: data.withdraw_fund_salt,
        };
        let result = nightfall_instance
            .withdraw_processed(decode_data)
            .call()
            .await
            .map_err(|e| NightfallContractError::EscrowError(format!("{e}")))?;
        Ok(result)
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

        let l2_block = nightfall
            .layer2_block_number()
            .call()
            .await
            .map_err(|e| NightfallContractError::EscrowError(format!("{e}")))?;

        Ok(l2_block)
    }
    async fn get_token_info(nf_token_id: Fr254) -> Result<TokenData, NightfallContractError> {
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let client = blockchain_client.root();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);

        let token_info = nightfall
            .getTokenInfo(Uint256::from(nf_token_id).0)
            .call()
            .await
            .map_err(|e| {
                NightfallContractError::EscrowError(format!("Error getting token info: {e}"))
            })?;

        Ok(TokenData {
            erc_address: FrBn254::from(token_info.ercAddress).into(),
            token_id: BigInteger256::from(Uint256(token_info.tokenId)),
        })
    }

    // given a layer 2 block number, return the layer 2 block and the sender address
    async fn get_layer2_block_by_number(
        block_number: I256,
    ) -> Result<(Address, Nightfall::Block), NightfallContractError> {
        let block_number = block_number - I256::ONE;
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let client = blockchain_client.root();
        let nightfall_address = get_addresses().nightfall();
        let block_topic = B256::from_slice(&block_number.to_be_bytes::<32>());

        let latest_block = client.get_block_number().await.map_err(|e| {
            NightfallContractError::ProviderError(format!("get_block_number error: {e}"))
        })?;

        let event_sig = B256::from(keccak256("BlockProposed(int256)"));
        let filter = Filter::new()
            .address(nightfall_address)
            .from_block(0u64)
            .to_block(latest_block)
            .event_signature(event_sig)
            .topic1(block_topic);

        let logs = client
            .get_logs(&filter)
            .await
            .map_err(|e| NightfallContractError::ProviderError(format!("Provider error: {e}")))?;

        let log = logs
            .first()
            .ok_or_else(|| NightfallContractError::BlockNotFound(block_number.as_u64()))?;

        let tx_hash = log.transaction_hash.ok_or_else(|| {
            NightfallContractError::MissingTransactionHash(
                "Log has no transaction hash".to_string(),
            )
        })?;
        let tx = client
            .get_transaction_by_hash(tx_hash)
            .await
            .map_err(|e| {
                NightfallContractError::ProviderError(format!("get_transaction error: {e}"))
            })?
            .ok_or(NightfallContractError::TransactionNotFound(tx_hash))?;

        let sender_address = tx.inner.signer();
        debug!("Sender of transaction {tx_hash} is {sender_address}");

        let decoded = Nightfall::NightfallCalls::abi_decode(tx.input()).map_err(|e| {
            NightfallContractError::AbiDecodeError(format!("ABI decode error: {e:?}"))
        })?;

        match decoded {
            Nightfall::NightfallCalls::propose_block(decoded) => {
                debug!("Successfully decoded block {block_number} from tx {tx_hash}");
                Ok((sender_address, decoded.blk))
            }
            _ => Err(NightfallContractError::DecodedCallError(
                "Decoded call was not propose_block".to_string(),
            )),
        }
    }
}
