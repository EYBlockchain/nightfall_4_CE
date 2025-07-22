use crate::blockchain_client::BlockchainClientConnection;
use crate::error::BlockchainClientConnectionError;
use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy::transports::ws::WsConnect;
use alloy::signers::local::PrivateKeySigner;
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
    provider: Arc<dyn Provider>,
    signer: PrivateKeySigner,
}

#[async_trait]
impl BlockchainClientConnection for LocalWsClient {
    type W = PrivateKeySigner;
    type T = WsConnect;
    type S = configuration::settings::Settings;

    async fn new(
        url: Url,
        signer: Self::W,
    ) -> Result<Self, BlockchainClientConnectionError> {
        let ws = WsConnect::new(url);
        let provider = ProviderBuilder::new().wallet(signer.clone())
        .on_ws(ws)
        .await
        .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;
            
        Ok(Self {
            provider: Arc::new(provider),
            signer: signer,
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

    async fn try_from_settings(settings: &Self::S) -> Result<Self, BlockchainClientConnectionError> {
        // get the signer
        let signer = match settings.nightfall_client.wallet_type.as_str() {
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

        info!("Created signer with address: {:?}", signer.address());
        debug!("And chain id: {}", settings.network.chain_id);
        debug!("And Ethereum client url: {}", settings.ethereum_client_url);

        // create provider
        let ws = WsConnect::new(settings.ethereum_client_url.clone());
        let provider = ProviderBuilder::new().wallet(signer.clone())
            .on_ws(ws)
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            signer: signer,
        })
    }
}