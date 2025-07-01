use crate::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
};
use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::U256,
};
use log::{debug, info};
use std::{error::Error, sync::Arc};
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
pub struct LocalWsClient(Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>);

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
        let provider = Provider::<Self::T>::connect(url)
            .await
            .map_err(BlockchainClientConnectionError::ProviderError)?;
        let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
        Ok(Self(Arc::new(client)))
    }

    async fn is_connected(&self) -> bool {
        self.get_client().node_info().await.is_ok()
    }

    async fn get_balance(&self) -> Option<U256> {
        let address = self.get_address()?;
        self.get_client().get_balance(address, None).await.ok()
    }

    fn get_address(&self) -> Option<ethers::types::Address> {
        Some(self.get_client().address())
    }

    fn get_client(&self) -> Arc<SignerMiddleware<Provider<Self::T>, Self::W>> {
        self.0.clone()
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
        let provider = Provider::<Ws>::connect(&settings.ethereum_client_url)
            .await
            .map_err(BlockchainClientConnectionError::ProviderError)?;
        // get the client
        let client =
            SignerMiddleware::new(provider, wallet.with_chain_id(settings.network.chain_id));
        let client = Arc::new(client);
        Ok(Self(client))
    }
}
