pub mod blockchain_client;
pub mod error;
pub mod hex_conversion;
pub mod merkle_trees;
pub mod models;
pub mod serialization;
pub mod tests_utils;
pub mod utils;
pub mod validate_certificate;
pub mod wallets;

pub mod initialisation {
    use crate::{blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError, wallets::LocalWsClient};
    use log::info; // Import the `info!` macro
    use configuration::settings::get_settings;
    use tokio::sync::{OnceCell, RwLock};
    use url::Url;

    /// This function is used to provide a singleton blockchain client connection across the entire application.
    pub async fn get_blockchain_client_connection() -> &'static RwLock<LocalWsClient> {
        static BLOCKCHAIN_CLIENT_CONNECTION: OnceCell<RwLock<LocalWsClient>> =
            OnceCell::const_new();
        BLOCKCHAIN_CLIENT_CONNECTION
            .get_or_init(|| async {

                let settings = get_settings();
            let url = match Url::parse(&settings.ethereum_client_url) {
                Ok(parsed_url) => parsed_url,
                Err(e) => {
                    panic!("Failed to parse Ethereum client URL: {}", e);
                }
            };
                let client = 
                    LocalWsClient::try_from_settings(get_settings())
                        .await.expect("Failed to create blockchain client from settings");
            let spwan = client.clone().spawn_reconnect_task(url.clone());
                            // Spawn a periodic background reconnection loop
                        info!("Blockchain client initialized and reconnect task started.");
                        RwLock::new(client)
            })
            .await
    }
}
