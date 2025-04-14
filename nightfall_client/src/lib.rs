pub mod domain;
pub mod driven;
pub mod drivers;
pub mod ports;
pub mod services;
pub mod test_helpers;

use ark_bn254::{Bn254, Fr as Fr254};
use ark_serialize::CanonicalDeserialize;
use configuration::addresses::get_addresses;
use drivers::derive_key::ZKPKeys;
use ethers::{
    abi::{encode, Tokenizable},
    types::U256,
    utils::keccak256,
};
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
use lib::utils::load_key_from_server;
use log::warn;
use num::BigUint;
use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

/// This function is used to retrieve the zkp keys
pub fn get_zkp_keys() -> &'static Mutex<ZKPKeys> {
    static ZKP_KEYS: OnceLock<Mutex<ZKPKeys>> = OnceLock::new();
    ZKP_KEYS.get_or_init(|| Mutex::new(Default::default()))
}

/// This function gets the fee token ID based on the current deployment.
/// Fee token ID is the keccak256 hash of the zero address and zero, right shifted by 4 bits.
pub fn get_fee_token_id() -> Fr254 {
    let nf_address = get_addresses().nightfall();

    let nf_address_token = nf_address.into_token();
    let u256_zero = U256::zero().into_token();

    let fee_token_id_biguint =
        BigUint::from_bytes_be(&keccak256(encode(&[nf_address_token, u256_zero]))) >> 4;
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
    use mongodb::Client;
    use std::sync::OnceLock;
    use tokio::sync::{OnceCell, RwLock};
    use url::Url;

    /// This function is used to provide a singleton database connection across the entire application.
    pub async fn get_db_connection() -> &'static RwLock<Client> {
        static DB_CONNECTION: OnceCell<RwLock<Client>> = OnceCell::const_new();
        DB_CONNECTION
            .get_or_init(|| async {
                RwLock::new({
                    let client = Client::with_uri_str(&get_settings().nightfall_client.db_url)
                        .await
                        .expect("Could not create database connection");
                    // it's not enough just to connect to a database, we need to initialise some trees in it
                    <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
                        .await
                        .expect("Could not create commitment tree");
                    client
                })
            })
            .await
    }

    /// This function is used to provide a singleton proposer http connection across the entire application.
    /// It's not wrapped in a mutex
    pub fn get_proposer_http_connection() -> &'static (reqwest::Client, Url) {
        static PROPOSER_HTTP_CONNECTION: OnceLock<(reqwest::Client, Url)> = OnceLock::new();
        PROPOSER_HTTP_CONNECTION.get_or_init(|| {
            (
                reqwest::Client::new(),
                Url::parse(&get_settings().nightfall_proposer.url)
                    .expect("Could not parse proposer url")
                    .join("/v1/transaction")
                    .expect("Could not join proposer url with /v1/transaction"),
            )
        })
    }
}
