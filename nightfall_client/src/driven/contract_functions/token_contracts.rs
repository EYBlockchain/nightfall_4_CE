//! Implementations of the [`TokenContract`] interface defined in `ports/contracts.rs`.

use super::contract_type_conversions::{Addr, Uint256};
use crate::{domain::error::TokenContractError, drivers::blockchain, ports::contracts::TokenContract};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, BigInteger256};
use ark_std::Zero;
use configuration::addresses::get_addresses;
use lib::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
    initialisation::get_blockchain_client_connection,
};
use nightfall_bindings::artifacts::{IERC1155, IERC20, IERC3525, IERC721};

impl TokenContract for IERC20::IERC20Calls {
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
        let read_connection = get_blockchain_client_connection()
            .await
            .read()
            .await;
        let blockchain_client = read_connection.get_client();
        let caller = read_connection.get_address();
        let client = blockchain_client.root();

        /* If your chain doesn't support signing transactions locally, and need to be signed at the client level and need to use send_raw_tansaction uncomment the code below */

        // let nonce = client.get_transaction_count(signer.address()).await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let gas_price = client.get_gas_price().await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let max_fee_per_gas = gas_price * 2;
        // let max_priority_fee_per_gas = gas_price;
        // let gas_limit = 5000000u64;

        // let raw_tx = IERC20::new(solidity_erc_address.0, client.clone())
        //         .approve(solidity_approval_address, solidity_value.0)
        //         .nonce(nonce)
        //         .gas(gas_limit)
        //         .max_fee_per_gas(max_fee_per_gas)
        //         .max_priority_fee_per_gas(max_priority_fee_per_gas)
        //         .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        //         .build_raw_transaction(caller).await
        //         .map_err(|e| {
        //             BlockchainClientConnectionError::ProviderError(format!(
        //                 "Contract error: {e}"
        //             ))
        //         })?;

        //         let tx_receipt = client.send_raw_transaction(&raw_tx).await
        //         .map_err(|e| {
        //             BlockchainClientConnectionError::ProviderError(format!(
        //                 "Contract error: {e}"
        //             ))
        //         })?
        //         .get_receipt()
        //         .await;

        let tx_receipt = IERC20::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_value.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await;

        if tx_receipt.is_err() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "Failed to get transaction receipt".to_string(),
            )
            .into());
        }
        Ok(())
    }
}

impl TokenContract for IERC721::IERC721Calls {
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
        let read_connection = get_blockchain_client_connection()
            .await
            .read()
            .await;
        let blockchain_client = read_connection.get_client();
        let caller = read_connection.get_address();
        let client = blockchain_client.root();

        // let nonce = client.get_transaction_count(signer.address()).await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let gas_price = client.get_gas_price().await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let max_fee_per_gas = gas_price * 2;
        // let max_priority_fee_per_gas = gas_price;
        // let gas_limit = 500000000u64;
        // let raw_tx = IERC721::new(solidity_erc_address.0, client.clone())
        //     .approve(solidity_approval_address, solidity_token_id.0)
        //     .nonce(nonce)
        //     .gas(gas_limit)
        //     .max_fee_per_gas(max_fee_per_gas)
        //     .max_priority_fee_per_gas(max_priority_fee_per_gas)
        //     .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        //     .build_raw_transaction(caller).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?;


        // let tx_receipt = client.send_raw_transaction(&raw_tx).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?
        //     .get_receipt()
        //     .await;

        let tx_receipt = IERC721::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_token_id.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await;

        if tx_receipt.is_err() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "Failed to get transaction receipt".to_string(),
            )
            .into());
        }
        Ok(())
    }
}

impl TokenContract for IERC1155::IERC1155Calls {
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
        let read_connection = get_blockchain_client_connection()
            .await
            .read()
            .await;
        let blockchain_client = read_connection.get_client();
        let caller = read_connection.get_address();
        let client = blockchain_client.root();

        let client = blockchain_client.root();
        let signer = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_signer();
        // let nonce = client.get_transaction_count(signer.address()).await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let gas_price = client.get_gas_price().await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let max_fee_per_gas = gas_price * 2;
        // let max_priority_fee_per_gas = gas_price;
        // let gas_limit = 500000000u64;
        // let raw_tx = IERC1155::new(solidity_erc_address.0, client.clone())
        //     .setApprovalForAll(solidity_approval_address, true)
        //     .nonce(nonce)
        //     .gas(gas_limit)
        //     .max_fee_per_gas(max_fee_per_gas)
        //     .max_priority_fee_per_gas(max_priority_fee_per_gas)
        //     .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        //     .build_raw_transaction(caller).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?;

        // let tx_receipt = client.send_raw_transaction(&raw_tx).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?
        //     .get_receipt()
        //     .await;


        let tx_receipt = IERC1155::new(solidity_erc_address.0, client.clone())
            .setApprovalForAll(solidity_approval_address, true)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await;

        if tx_receipt.is_err() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "Failed to get transaction receipt".to_string(),
            )
            .into());
        }
        Ok(())
    }
}

impl TokenContract for IERC3525::IERC3525Calls {
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
        let read_connection = get_blockchain_client_connection()
            .await
            .read()
            .await;
        let blockchain_client = read_connection.get_client();
        let caller = read_connection.get_address();
        let client = blockchain_client.root();

        // let nonce = client.get_transaction_count(signer.address()).await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let gas_price = client.get_gas_price().await.map_err(|e| {
        //     BlockchainClientConnectionError::ProviderError(format!(
        //         "Contract error: {e}"
        //     ))
        // })?;
        // let max_fee_per_gas = gas_price * 2;
        // let max_priority_fee_per_gas = gas_price;
        // let gas_limit = 500000000u64;
        // let raw_tx = erc3525
        //     .approve_0(solidity_approval_address, solidity_token_id.0)
        //     .nonce(nonce)
        //     .gas(gas_limit)
        //     .max_fee_per_gas(max_fee_per_gas)
        //     .max_priority_fee_per_gas(max_priority_fee_per_gas)
        //     .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        //     .build_raw_transaction(caller).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?;

        
        // let tx_receipt = client.send_raw_transaction(&raw_tx).await
        //     .map_err(|e| {
        //         BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
        //     })?
        //     .get_receipt()
        //     .await;


        let tx_receipt = IERC3525::new(solidity_erc_address.0, client.clone())
            .approve_0(solidity_approval_address, solidity_token_id.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await;

        if tx_receipt.is_err() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "Failed to get transaction receipt".to_string(),
            )
            .into());
        }
        Ok(())
    }
}
