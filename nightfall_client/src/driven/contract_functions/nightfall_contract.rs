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
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use ark_std::Zero;
use configuration::addresses::get_addresses;
use ethers::{
    abi::{encode, Tokenizable},
    providers::ProviderError,
    types::I256,
    utils::keccak256,
};
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};
use log::info;
use nightfall_bindings::{
    erc20_mock::ERC20MockErrors,
    ierc3525::IERC3525,
    nightfall::{Nightfall, WithdrawData as NFWithdrawData},
};
use num::BigUint;

impl<M> NightfallContract for Nightfall<M> {
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

        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

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
            .value(Uint256::from(total_fee));

        let receipt = call
            .send()
            .await
            .map_err(|e| {
                if e.is_revert() {
                    ProviderError::CustomError(format!(
                        "Revert when calling escrow: {:?}",
                        e.decode_contract_revert::<ERC20MockErrors>()
                    ))
                } else {
                    ProviderError::CustomError(format!("Contract error: {}", e))
                }
            })?
            .await?;
        info!(
            "Gas used in escrow funds: {:?}",
            receipt.clone().unwrap().gas_used
        );

        let slot_id = if let TokenType::ERC3525 = token_type {
            let erc_contract = IERC3525::new(solidity_token_address.0, client.clone());
            erc_contract
                .slot_of(solidity_token_id.0)
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

        receipt.ok_or(NightfallContractError::EscrowError(
            "Transaction unsuccesful".to_string(),
        ))?;

        // We calculate the the nf_token_id and nf_slot_id here
        let erc_token = solidity_token_address.0.into_token();
        let nf_token_id =
            to_nf_token_id_from_solidity(solidity_token_address.0, solidity_token_id.0);
        if slot_id == solidity_token_id.0 {
            let nf_slot_id = nf_token_id;
            Ok([nf_token_id, nf_slot_id])
        } else {
            let slot_id_token = slot_id.into_token();
            let nf_slot_id_biguint =
                BigUint::from_bytes_be(&keccak256(encode(&[erc_token, slot_id_token]))) >> 4;
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
        let data = NFWithdrawData::from(withdraw_data);

        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let contract = Nightfall::new(get_addresses().nightfall(), client.clone());

        let call = contract.descrow_funds(data, token_type.into());

        let receipt = call
            .send()
            .await
            .map_err(|e| {
                if e.is_revert() {
                    ProviderError::CustomError(format!(
                        "Revert: {:?}",
                        e.decode_contract_revert::<ERC20MockErrors>()
                    ))
                } else {
                    ProviderError::CustomError(format!("Contract error: {}", e))
                }
            })?
            .await?;
        if receipt.clone().unwrap().gas_used.is_some() {
            info!(
                "Gas used in de_escrow_funds: {:?}",
                receipt.clone().unwrap().gas_used.unwrap()
            );
        }
        receipt.ok_or(NightfallContractError::EscrowError(
            "Transaction unsuccesful".to_string(),
        ))?;
        Ok(())
    }

    async fn withdraw_available(
        withdraw_data: WithdrawData,
    ) -> Result<bool, NightfallContractError> {
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let contract = Nightfall::new(get_addresses().nightfall(), client.clone());

        let data = NFWithdrawData::from(withdraw_data);
        contract
            .withdraw_processed(data)
            .call()
            .await
            .map_err(|e| ProviderError::CustomError(format!("Contract error: {}", e)).into())
    }

    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let nightfall_address = get_addresses().nightfall();
        let nightfall = Nightfall::new(nightfall_address, client);

        nightfall
            .layer_2_block_number()
            .call()
            .await
            .map_err(|_| NightfallContractError::TransactionError)
    }

    async fn get_token_info(
        nf_token_id: Fr254,
    ) -> Result<TokenData, NightfallContractError> {
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let contract = Nightfall::new(get_addresses().nightfall(), client);

        let (erc_address, token_id) = contract
            .get_token_info(Uint256::from(nf_token_id).into())
            .call()
            .await
            .map_err(|_| NightfallContractError::TransactionError)?;

        Ok(TokenData {
            erc_address: FrBn254::from(erc_address).into(),
            token_id: BigInteger256::from(Uint256(token_id)),
        })
    }
}
