pub mod blockchain_client;
pub mod circuit_key_generation;
pub mod contract_conversions;
pub mod deposit_circuit;
pub mod entities;
pub mod error;
pub mod health_check;
pub mod hex_conversion;
pub mod merkle_trees;
pub mod models;
pub mod nf_client_proof;
pub mod nf_token_id;
pub mod plonk_prover;
pub mod rollup_circuit_checks;
pub mod serialization;
pub mod shared_entities;
pub mod tests_utils;
pub mod utils;
pub mod validate_certificate;
pub mod wallets;

use alloy::dyn_abi::abi::encode;
use alloy::primitives::{keccak256, U256};
use alloy::sol_types::SolValue;
use ark_bn254::Fr as Fr254;
use configuration::addresses::get_addresses;
use num::BigUint;

/// This function gets the fee token ID based on the current deployment.
/// Fee token ID is the keccak256 hash of the zero address and zero, right shifted by 4 bits.
pub fn get_fee_token_id() -> Fr254 {
    let nf_address = get_addresses().nightfall();

    let nf_address_token = nf_address.tokenize();
    let u256_zero = U256::ZERO.tokenize();
    let fee_token_id_biguint =
        BigUint::from_bytes_be(keccak256(encode(&(nf_address_token, u256_zero))).as_slice()) >> 4;
    Fr254::from(fee_token_id_biguint)
}

pub mod initialisation {
    use crate::{blockchain_client::BlockchainClientConnection, wallets::LocalWsClient};
    use configuration::settings::get_settings;
    use tokio::sync::{OnceCell, RwLock};
    /// This function is used to provide a singleton blockchain client connection across the entire application.
    pub async fn get_blockchain_client_connection() -> &'static RwLock<LocalWsClient> {
        static BLOCKCHAIN_CLIENT_CONNECTION: OnceCell<RwLock<LocalWsClient>> =
            OnceCell::const_new();
        BLOCKCHAIN_CLIENT_CONNECTION
            .get_or_init(|| async {
                RwLock::new(
                    LocalWsClient::try_from_settings(get_settings())
                        .await
                        .expect("Could not create blockchain client connection"),
                )
            })
            .await
    }
}
