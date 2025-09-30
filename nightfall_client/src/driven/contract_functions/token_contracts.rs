//! Implementations of the [`TokenContract`] interface defined in `ports/contracts.rs`.

use super::contract_type_conversions::{Addr, Uint256};
use crate::{domain::error::TokenContractError, ports::contracts::TokenContract};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, BigInteger256};
use ark_std::Zero;
use configuration::addresses::get_addresses;
use lib::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
    initialisation::get_blockchain_client_connection,
};
use log::debug;
use nightfall_bindings::artifacts::{IERC1155, IERC20, IERC3525, IERC721};

impl TokenContract for IERC20::IERC20Calls {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // ERC-20: token_id must be zero (domain-level guard)
        if token_id != BigInteger256::zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC20 approvals should have a token ID of 0".to_string(),
            ));
        }

        // Type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let spender = get_addresses().nightfall();
        let amount = Uint256::from(value);

        // Resolve client and explicit caller
        let conn_guard = get_blockchain_client_connection().await;
        let read = conn_guard.read().await;
        let provider = read.get_client();
        let client = provider.root();
        let caller = read.get_address();

         /* If your chain doesn't support signing transactions locally, and need to be signed at the client level and need to use send_raw_tansaction uncomment the code below */

        // let nonce = client.get_transaction_count(caller).await.map_err(|e| {
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
        //         .approve(spender, amount.0)
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

        // Send the transaction with explicit `from`
        // ERC-20 approve(spender, amount) never requires token ownership or balance. It just sets the allowance for the caller itself (owner = msg.sender)
        let tx_receipt = IERC20::new(solidity_erc_address.0, client.clone())
            .approve(spender, amount.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|_| {
                BlockchainClientConnectionError::ProviderError(
                    "Failed to get transaction receipt".to_string(),
                )
            })?;

        debug!("ERC20 approval tx mined, from: {:?}", tx_receipt.from);

        if !tx_receipt.status() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "ERC20 SetApproval Transaction reverted (status=0)".to_string(),
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
        // ERC-721: value must be zero (domain-level guard)
        if !value.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC721 approvals should have a value of 0".to_string(),
            ));
        }

        // Type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let spender = get_addresses().nightfall();
        let token_id_u256 = Uint256::from(token_id);

        // Resolve client and explicit caller
        let conn_guard = get_blockchain_client_connection().await;
        let read = conn_guard.read().await;
        let provider = read.get_client();
        let client = provider.root();
        let caller = read.get_address();

        // let nonce = client.get_transaction_count(caller).await.map_err(|e| {
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
        //     .approve(spender, token_id_u256.0)
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


        // Send the transaction with explicit `from`
        let tx_receipt = IERC721::new(solidity_erc_address.0, client.clone())
            .approve(spender, token_id_u256.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|_| {
                BlockchainClientConnectionError::ProviderError(
                    "Failed to get transaction receipt".to_string(),
                )
            })?;

        debug!("ERC721 approval tx mined, from: {:?}", tx_receipt.from);

        if !tx_receipt.status() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "ERC721 SetApproval Transaction reverted (status=0)".to_string(),
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

        if value.is_zero() & token_id.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC1155 approvals should have one of value or token ID non-zero".to_string(),
            ));
        }

        // Type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let operator = get_addresses().nightfall();

        // Resolve client and explicit caller
        let conn_guard = get_blockchain_client_connection().await;
        let read = conn_guard.read().await;
        let provider = read.get_client();
        let client = provider.root();
        let caller = read.get_address();

        // let nonce = client.get_transaction_count(caller).await.map_err(|e| {
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
        //     .setApprovalForAll(operator, true)
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

        // Send the transaction with explicit `from`
        // setApprovalForAll(operator, approved) is per-caller, not per tokenId or value.
        // Any address can toggle operator approval for itself; there is no ownership-of-a-specific-token check and no balance requirement.
        let tx_receipt = IERC1155::new(solidity_erc_address.0, client.clone())
            .setApprovalForAll(operator, true)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|_| {
                BlockchainClientConnectionError::ProviderError(
                    "Failed to get transaction receipt".to_string(),
                )
            })?;

        debug!("ERC1155 approval tx mined, from: {:?}", tx_receipt.from);

        if !tx_receipt.status() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "ERC1155 SetApproval Transaction reverted (status=0)".to_string(),
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
        // Type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let spender = get_addresses().nightfall();
        let token_id_u256 = Uint256::from(token_id);

        // Resolve client and explicit caller
        let conn_guard = get_blockchain_client_connection().await;
        let read = conn_guard.read().await;
        let provider = read.get_client();
        let client = provider.root();
        let caller = read.get_address();

        debug!("ERC3525 caller: {caller:?}");

        // let nonce = client.get_transaction_count(caller).await.map_err(|e| {
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
        //     .approve_0(spender, token_id_u256.0)
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


        // NOTE: IERC3525 has overloaded approve functions in many implementations.
        // Here we use the 2-arg overload approve(address to, uint256 tokenId),
        // which bindings expose as `approve_0`.

        // Send the transaction with explicit `from`
        let tx_receipt = IERC3525::new(solidity_erc_address.0, client.clone())
            .approve_0(spender, token_id_u256.0)
            .from(caller)
            .send()
            .await
            .map_err(|e| {
                BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
            })?
            .get_receipt()
            .await
            .map_err(|_| {
                BlockchainClientConnectionError::ProviderError(
                    "Failed to get transaction receipt".to_string(),
                )
            })?;

        debug!("ERC3525 approval tx mined, from: {:?}", tx_receipt.from);

        if !tx_receipt.status() {
            return Err(BlockchainClientConnectionError::ProviderError(
                "ERC3525 SetApproval Transaction reverted (status=0)".to_string(),
            )
            .into());
        }

        Ok(())
    }
}
