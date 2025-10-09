use crate::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
};
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::ws::WsConnect;
use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use log::{debug, info, warn, error};
use std::{error::Error, sync::Arc};
use url::Url;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug)]
pub enum WalletType {
    Local(PrivateKeySigner),
    Azure(AzureWallet),
}

/// Struct to represent an Azure-based wallet
#[derive(Clone, Debug)]
pub struct AzureWallet {
    key_client: SecretClient,
    key_name: String,
}

impl AzureWallet {
    pub async fn new(
        vault_url: &str,
        key_name: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let credential = azure_identity::create_default_credential()?;
        let key_client = SecretClient::new(vault_url, credential)?;
        Ok(Self {
            key_client,
            key_name: key_name.to_string(),
        })
    }

    /// Fetch the signing key from Azure Key Vault
    pub async fn get_signing_key(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let key = self.key_client.get(self.key_name.clone()).await?;
        Ok(key.value)
    }
}

#[derive(Clone)]
pub struct LocalWsClient {
    provider: Arc<dyn Provider>,
    signer: PrivateKeySigner,
}

impl LocalWsClient {
    /// Create a new WebSocket provider connection.
    async fn connect_provider(
        url: Url,
        signer: PrivateKeySigner,
    ) -> Result<Arc<dyn Provider>, BlockchainClientConnectionError> {
        let ws = WsConnect::new(url);
        let provider = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect_ws(ws)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;
        Ok(Arc::new(provider))
    }

    /// Attempt to reconnect to the WebSocket RPC, retrying up to `max_retries` times.
    async fn reconnect(
        &mut self,
        url: Url,
        max_retries: usize,
    ) -> Result<(), BlockchainClientConnectionError> {
        let mut retries = 0;
        loop {
            retries += 1;
            match Self::connect_provider(url.clone(), self.signer.clone()).await {
                Ok(p) => {
                    self.provider = p;
                    warn!("Reconnected to WebSocket after {} attempt(s)", retries);
                    return Ok(());
                }
                Err(e) => {
                    if retries >= max_retries {
                        error!("Failed to reconnect after {} retries: {}", retries, e);
                        return Err(e);
                    }
                    warn!(
                        "WebSocket reconnect attempt {}/{} failed: {}. Retrying in 5s...",
                        retries,
                        max_retries,
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Periodically check if the provider is still connected and reconnect if needed.
    pub fn spawn_reconnect_task(self, url: Url) -> Arc<Self> {
        let client = Arc::new(self);
        let client_clone = client.clone();
        let client_for_task = client.clone();
        println!("Spawning reconnect task for WebSocket client...");

        tokio::spawn(async move {
            loop {
                let is_connected = client_clone.is_connected().await;
                println!("connection is alive: {}", is_connected);
                if !client_clone.is_connected().await {
                    warn!("WebSocket connection lost, attempting to reconnect...");
                    if let Err(e) = Arc::get_mut(&mut client_for_task.clone())
                        .unwrap()
                        .reconnect(url.clone(), 5)
                        .await
                    {
                        error!("Reconnection failed: {}", e);
                    }
                }
                sleep(Duration::from_secs(3)).await;
            }
        });

        client
    }
}

#[async_trait]
impl BlockchainClientConnection for LocalWsClient {
    type W = PrivateKeySigner;
    type T = WsConnect;
    type S = configuration::settings::Settings;

    async fn new(url: Url, local_signer: Self::W) -> Result<Self, BlockchainClientConnectionError> {
        let ws = WsConnect::new(url);
        let provider = ProviderBuilder::new()
            .wallet(local_signer.clone())
            .connect_ws(ws)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            signer: local_signer,
        })
    }

    async fn is_connected(&self) -> bool {
        self.provider.get_net_version().await.is_ok()
    }

    async fn get_balance(&self) -> Option<alloy::primitives::U256> {
        let address = self.get_address();
        self.provider.get_balance(address).await.ok()
    }

    fn get_address(&self) -> Address {
        self.signer.address()
    }

    fn get_client(&self) -> Arc<dyn Provider> {
        self.provider.clone()
    }

    fn get_signer(&self) -> PrivateKeySigner {
        self.signer.clone()
    }

    async fn try_from_settings(
        settings: &Self::S,
    ) -> Result<Self, BlockchainClientConnectionError> {
        // get the signer
        let local_signer = match settings.nightfall_client.wallet_type.as_str() {
            "local" => settings
                .signing_key
                .parse::<PrivateKeySigner>()
                .map_err(BlockchainClientConnectionError::WalletError)?,
            "azure" => {
                let azure_wallet =
                    AzureWallet::new(&settings.azure_vault_url, &settings.azure_key_name).await?;
                let signing_key = azure_wallet.get_signing_key().await?;
                signing_key
                    .parse::<PrivateKeySigner>()
                    .map_err(BlockchainClientConnectionError::WalletError)?
            }
            "YubiWallet" => todo!(),
            "AwsSigner" => todo!(),
            "EYTransactionManager" => todo!(),
            _ => panic!("Invalid wallet type"),
        };

        info!("Created signer with address: {:?}", local_signer.address());
        debug!("And chain id: {}", settings.network.chain_id);
        debug!("And Ethereum client url: {}", settings.ethereum_client_url);
        let url = Url::parse(&settings.ethereum_client_url)
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        // create provider
        // Attempt to connect with retry logic at startup
        let mut retries = 0;
        let provider: Arc<dyn Provider> = loop {
            match LocalWsClient::connect_provider(url.clone(), local_signer.clone()).await {
                Ok(p) => break p,
                Err(e) => {
                    retries += 1;
                    if retries > 5 {
                        return Err(e);
                    }
                    warn!(
                        "Initial WebSocket connection failed (attempt {}), retrying in 5s: {}",
                        retries,
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        };

        Ok(Self {
            provider,
            signer: local_signer,
        })
    }
}
