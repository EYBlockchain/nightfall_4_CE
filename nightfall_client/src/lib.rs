pub mod domain;
pub mod driven;
pub mod drivers;
pub mod ports;
pub mod services;
pub mod test_helpers;

use alloy::dyn_abi::abi::encode;
use alloy::primitives::{keccak256, U256};
use alloy::sol_types::SolValue;
use ark_bn254::{Bn254, Fr as Fr254};
use ark_serialize::CanonicalDeserialize;
use configuration::addresses::get_addresses;
use drivers::derive_key::ZKPKeys;
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
use lib::utils::load_key_from_server;
use log::warn;
use num::BigUint;
use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};
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

/// This function is used to retrieve the client proving key.
pub fn get_client_proving_key() -> &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> {
    static PK: OnceLock<Arc<ProvingKey<UnivariateKzgPCS<Bn254>>>> = OnceLock::new();
    PK.get_or_init(|| {
        fn find(path: &Path) -> Option<std::path::PathBuf> {
            if path.is_absolute() {
                match path.is_file() {
                    true => return Some(path.to_path_buf()),
                    false => return None,
                }
            }

            let cwd = std::env::current_dir().ok()?;
            let mut cwd = cwd.as_path();
            loop {
                let file_path = cwd.join(path);
                if file_path.is_file() {
                    return Some(file_path);
                }

                cwd = cwd.parent()?;
            }
        }

        // We'll try to load from the configuration server first.
        if let Some(key_bytes) = load_key_from_server("proving_key") {
            let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                &*key_bytes,
            )
            .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        // If that fails, we'll try to load from a local file
        warn!("Could not load proving key from server. Loading from local file");
        let path = Path::new("./configuration/bin/proving_key");
        let source_file = find(path).expect("Could not find path");
        let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read proving key"),
        )
        .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
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
