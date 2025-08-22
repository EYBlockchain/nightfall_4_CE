//! Implementations of the [`TokenContract`] interface defined in `ports/contracts.rs`.

use super::contract_type_conversions::{Addr, Uint256};
use crate::{domain::error::TokenContractError, ports::contracts::TokenContract};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, BigInteger256};
use ark_std::Zero;
use configuration::addresses::get_addresses;
use ethers::providers::ProviderError;
use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};

use nightfall_bindings::{
    ierc1155::IERC1155, ierc20::IERC20, ierc3525::IERC3525, ierc721::IERC721,
};

impl<M> TokenContract for IERC20<M> {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the token ID is zero
        if token_id != BigInteger256::zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC20 approvals should have a token ID of 0".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_value = Uint256::from(value);

        // Send the transaction.
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let tx_receipt = IERC20::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_value.0)
            .send()
            .await
            .map_err(|e| ProviderError::CustomError(format!("Contract error: {e}")))?
            .await?;

        tx_receipt.ok_or(TokenContractError::TransactionError)?;
        Ok(())
    }
}

impl<M> TokenContract for IERC721<M> {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the value is zero
        if !value.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC721 approvals should have a value of 0".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_token_id = Uint256::from(token_id);

        // Send the transaction.
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let tx_receipt = IERC721::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_token_id.0)
            .send()
            .await
            .map_err(|e| ProviderError::CustomError(format!("Contract error: {e}")))?
            .await?;

        tx_receipt.ok_or(TokenContractError::TransactionError)?;
        Ok(())
    }
}

impl<M> TokenContract for IERC1155<M> {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the value is zero
        if value.is_zero() & token_id.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC1155 approvals should have one of value or token ID non-zero".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();

        // Send the transaction.
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let tx_receipt = IERC1155::new(solidity_erc_address.0, client.clone())
            .set_approval_for_all(solidity_approval_address, true)
            .send()
            .await
            .map_err(|e| ProviderError::CustomError(format!("Contract error: {e}")))?
            .await?;

        tx_receipt.ok_or(TokenContractError::TransactionError)?;
        Ok(())
    }
}

impl<M> TokenContract for IERC3525<M> {
    async fn set_approval(
        erc_address: Fr254,
        _value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_token_id = Uint256::from(token_id);

        // Send the transaction.
        let client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();

        let tx_receipt = IERC3525::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_token_id.0)
            .send()
            .await
            .map_err(|e| ProviderError::CustomError(format!("Contract error: {e}")))?
            .await?;

        tx_receipt.ok_or(TokenContractError::TransactionError)?;
        Ok(())
    }
}
