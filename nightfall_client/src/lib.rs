pub mod domain;
pub mod driven;
pub mod drivers;
pub mod ports;
pub mod services;
pub mod test_helpers;

use alloy::dyn_abi::abi::encode;
use alloy::primitives::{keccak256, U256};
use alloy::sol_types::SolValue;
use ark_bn254::Fr as Fr254;
use configuration::addresses::get_addresses;
use drivers::derive_key::ZKPKeys;
use num::BigUint;
use std::sync::{Mutex, OnceLock};
use bip32::{Mnemonic};
use bip32::DerivationPath;

/// This function is used to retrieve the zkp keys
pub fn get_zkp_keys() -> &'static Mutex<ZKPKeys> {
    static ZKP_KEYS: OnceLock<Mutex<ZKPKeys>> = OnceLock::new();
    ZKP_KEYS.get_or_init(|| 
        {
        let rng = ark_std::rand::thread_rng();
        let mnemonic = Mnemonic::random(rng,  Default::default());
        let path: DerivationPath = "m/44'/60'/0'/0/0".parse().expect("failed to parse path");
        let zkp_keys = ZKPKeys::derive_from_mnemonic(&mnemonic, &path).expect("Could not derive ZKP keys from mnemonic");
        Mutex::new(zkp_keys)
}
    )
}

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
    use crate::ports::trees::CommitmentTree;
    use ark_bn254::Fr as Fr254;
    use configuration::settings::get_settings;
    use mongodb::Client as MongoClient;
    use reqwest::{Client as HttpClient, ClientBuilder};
    use std::{sync::OnceLock, time::Duration};
    use tokio::sync::OnceCell;
    use url::Url;

    /// This function is used to provide a singleton database connection across the entire application.
    pub async fn get_db_connection() -> &'static MongoClient {
        static DB_CONNECTION: OnceCell<MongoClient> = OnceCell::const_new();
        DB_CONNECTION
            .get_or_init(|| async {
                let client = MongoClient::with_uri_str(&get_settings().nightfall_client.db_url)
                    .await
                    .expect("Could not create database connection");
                // Initialize the commitment tree in the database
                <MongoClient as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
                    .await
                    .expect("Could not create commitment tree");
                client
            })
            .await
    }

    /// This function is used to provide a singleton proposer http connection across the entire application.
    pub fn get_proposer_http_connection() -> &'static (HttpClient, Url) {
        static PROPOSER_HTTP_CONNECTION: OnceLock<(HttpClient, Url)> = OnceLock::new();
        PROPOSER_HTTP_CONNECTION.get_or_init(|| {
            let base_url = &get_settings().nightfall_proposer.url;
            let url = Url::parse(base_url)
                .expect("Could not parse proposer url")
                .join("/v1/transaction")
                .expect("Could not join proposer url with /v1/transaction");

            // Create a new HTTP client with a timeout
            let client = ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Could not build HTTP client with timeout");
            (client, url)
        })
    }
}
