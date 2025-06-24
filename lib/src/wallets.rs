use crate::blockchain_client::BlockchainClientConnection;
use crate::error::BlockchainClientConnectionError;
use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use k256::ecdsa::SigningKey;
use k256::ecdsa::signature::Signer;
use alloy::primitives::{U256, Address};
use alloy::providers::{ProviderBuilder, Provider};
use alloy::network::EthereumWallet;
use alloy::signers::local::{PrivateKeySigner as LocalWallet, LocalSigner as Wallet};
use alloy::signers::Signer;
use alloy::transports::ws::WsConnect as Ws;
use log::{debug, info};
use std::error::Error;
use std::sync::Arc;
use url::Url;

#[derive(Clone, Debug)]
pub enum WalletType {
    Local(LocalWallet),
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
        // default credential use the environment variables AZURE_CLIENT_ID, AZURE_TENANT_ID, AZURE_CLIENT_SECRET for environment variables
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
    provider: Arc<dyn Provider<Ws>>,
    signer: EthereumWallet,
    chain_id: u64,
}

#[async_trait]
impl BlockchainClientConnection for LocalWsClient {
    type W = LocalWallet;
    type T = Ws;
    type S = configuration::settings::Settings;

    async fn new(
        url: Url,
        wallet: Self::W,
        chain_id: u64,
    ) -> Result<Self, BlockchainClientConnectionError> {
        let provider = ProviderBuilder::new()
            .on_ws(url)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            signer: EthereumWallet::from(wallet),
            chain_id,
        })
    }

    async fn is_connected(&self) -> bool {
        self.get_client().node_info().await.is_ok()
    }

    async fn get_balance(&self) -> Option<U256> {
        let address = self.get_address()?;
        self.get_client().get_balance(address, None).await.ok()
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.get_client().address())
    }

    fn get_client(&self) -> Arc<dyn Provider<Self::T>> {
        self.provider.clone()
    }

    async fn try_from_settings(settings: &Self::S) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized,
    {
        // get the wallet.
        let wallet = match settings.nightfall_client.wallet_type.as_str() {
            // a local wallet has the private key read into the settings from an environment variable NF4_SIGNING_KEY
            "local" => settings
                .signing_key
                .parse::<LocalWallet>()
                .map_err(BlockchainClientConnectionError::WalletError)?,
            "azure" => {
                let azure_wallet =
                    AzureWallet::new(&settings.azure_vault_url, &settings.azure_key_name).await?;
                let signing_key = azure_wallet.get_signing_key().await?;
                signing_key
                    .parse::<LocalWallet>()
                    .map_err(BlockchainClientConnectionError::WalletError)?
            }
            "YubiWallet" => todo!(),
            "AwsSigner" => todo!(),
            "EYTransactionManager" => todo!(),
            _ => panic!("Invalid wallet type"),
        };
        info!("Created wallet with address: {:?}", wallet.address());
        debug!("And chain id: {}", settings.network.chain_id);
        debug!("And Ethereum client url: {}", settings.ethereum_client_url);
        // get the provider
        let provider = ProviderBuilder::new().connect(settings.ethereum_client_url.as_str())
        .await
        .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            signer: EthereumWallet::from(wallet),
            chain_id: settings.network.chain_id,
        })
    }
}
