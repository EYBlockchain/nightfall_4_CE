pub mod blockchain_client;
pub mod error;
pub mod hex_conversion;
pub mod merkle_trees;
pub mod models;
pub mod serialization;
pub mod utils;
pub mod validate_certificate;
pub mod wallets;

pub mod initialisation {
    use crate::blockchain_client::BlockchainClientConnection;
    use crate::wallets::LocalWsClient;
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
