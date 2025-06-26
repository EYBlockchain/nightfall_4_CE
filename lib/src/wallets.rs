use crate::blockchain_client::BlockchainClientConnection;
use crate::error::BlockchainClientConnectionError;
use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::network::EthereumSigner;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use alloy::transports::Ws;
use log::{debug, info};
use std::error::Error;
use std::sync::Arc;
use url::Url;

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
    provider: Arc<Provider<Ws>>,
    signer: EthereumSigner<PrivateKeySigner>,
    chain_id: u64,
}

#[async_trait]
impl BlockchainClientConnection for LocalWsClient {
    type Signer = PrivateKeySigner;
    type Transport = Ws;
    type Settings = configuration::settings::Settings;

    async fn new(
        url: Url,
        signer: Self::Signer,
        chain_id: u64,
    ) -> Result<Self, BlockchainClientConnectionError> {
        let provider = ProviderBuilder::new()
            .connect_ws(url)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;
            
        Ok(Self {
            provider: Arc::new(provider),
            signer: EthereumSigner::from(signer),
            chain_id,
        })
    }

    async fn is_connected(&self) -> bool {
        self.provider.get_network_info().await.is_ok()
    }

    async fn get_balance(&self) -> Option<alloy::primitives::U256> {
        let address = self.get_address()?;
        self.provider.get_balance(address).await.ok()
    }

    fn get_address(&self) -> Option<alloy::primitives::Address> {
        Some(self.signer.address())
    }

    fn get_provider(&self) -> Arc<Provider<Self::Transport>> {
        self.provider.clone()
    }

    fn get_signer(&self) -> EthereumSigner<Self::Signer> {
        self.signer.clone()
    }

    async fn try_from_settings(settings: &Self::Settings) -> Result<Self, BlockchainClientConnectionError> {
        // get the signer
        let signer = match settings.nightfall_client.wallet_type.as_str() {
            "local" => {
                PrivateKeySigner::from_string(&settings.signing_key)
                    .map_err(BlockchainClientConnectionError::SignerError)?
            }
            "azure" => {
                let azure_wallet =
                    AzureWallet::new(&settings.azure_vault_url, &settings.azure_key_name).await?;
                let signing_key = azure_wallet.get_signing_key().await?;
                PrivateKeySigner::from_string(&signing_key)
                    .map_err(BlockchainClientConnectionError::SignerError)?
            }
            "YubiWallet" => todo!(),
            "AwsSigner" => todo!(),
            "EYTransactionManager" => todo!(),
            _ => panic!("Invalid wallet type"),
        };

        info!("Created signer with address: {:?}", signer.address());
        debug!("And chain id: {}", settings.network.chain_id);
        debug!("And Ethereum client url: {}", settings.ethereum_client_url);

        // create provider
        let provider = ProviderBuilder::new()
            .on_ws(&settings.ethereum_client_url)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            signer: EthereumSigner::from(signer),
            chain_id: settings.network.chain_id,
        })
    }
}