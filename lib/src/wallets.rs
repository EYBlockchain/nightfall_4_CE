use crate::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
};
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use async_trait::async_trait;
use alloy::providers::WsConnect;
use log::{debug, info};
use std::{error::Error, sync::Arc};

use azure_security_keyvault::KeyClient;
use azure_identity;
use url::Url;


#[derive(Clone, Debug)]
pub enum WalletType {
    Local(PrivateKeySigner),
    Azure(AzureWallet),
}

/// Struct to represent an Azure-based wallet
#[derive(Clone)]
pub struct AzureWallet {
    key_client: Arc<KeyClient>,  
    key_name: String,
    address: Address,  
}

impl AzureWallet {
    pub async fn new(
        vault_url: &str,
        key_name: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
       // let credential = azure_identity::create_default_credential()?;
       let credential = azure_identity::create_credential()?;
       let key_client = KeyClient::new(vault_url, credential)?;
       
       let address = Address::ZERO;
        
        Ok(Self {
            key_client: Arc::new(key_client),
            key_name: key_name.to_string(),
            address,
        })
    }
    pub fn address(&self) -> Address {
        self.address
    }

}

impl std::fmt::Debug for AzureWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AzureWallet")
            .field("key_name", &self.key_name)
            .field("address", &self.address)
            .finish()
    }
}


#[derive(Clone)]
pub struct LocalWsClient {
    provider: Arc<dyn Provider>,
    wallet: WalletType,
}

#[async_trait]
impl BlockchainClientConnection for LocalWsClient {
    type W = PrivateKeySigner;
    type T = WsConnect;
    type S = configuration::settings::Settings;

    async fn new(url: Url, local_signer: Self::W) -> Result<Self, BlockchainClientConnectionError> {
        // let ws = WsConnect::new(url);
        // let wallet = WalletType::Local(local_signer.clone());

        let provider = ProviderBuilder::new()
            .wallet(local_signer.clone()) 
            .connect_ws(WsConnect::new(url))
            .await
            .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;

        Ok(Self {
            provider: Arc::new(provider),
            wallet: WalletType::Local(local_signer),
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
        match &self.wallet {
            WalletType::Local(signer) => signer.address(),
            WalletType::Azure(wallet) => wallet.address(),
        }
    }

    fn get_client(&self) -> Arc<dyn Provider> {
        self.provider.clone()
    }

    fn get_signer(&self) -> PrivateKeySigner {
        match &self.wallet {
            WalletType::Local(signer) => signer.clone(),
            WalletType::Azure(_) => {
                panic!("Cannot get PrivateKeySigner for Azure wallet - key never leaves HSM")
            }
        }
    }

    async fn try_from_settings(
        settings: &Self::S,
    ) -> Result<Self, BlockchainClientConnectionError> {
        match settings.nightfall_client.wallet_type.as_str() {
    
            "local" => {
                info!("Creating local wallet");
                
                let local_signer = settings
                    .signing_key
                    .parse::<PrivateKeySigner>()
                    .map_err(BlockchainClientConnectionError::WalletError)?;
                
                let ws = WsConnect::new(settings.ethereum_client_url.clone());
                let provider = ProviderBuilder::new()
                    .wallet(local_signer.clone())
                    .connect_ws(ws)
                    .await
                    .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;
                
                Ok(Self {
                    provider: Arc::new(provider),
                    wallet: WalletType::Local(local_signer),
                })
            }
            "azure" => {
                info!("Creating Azure Key Vault wallet");

                let azure_wallet = AzureWallet::new(
                    &settings.azure_vault_url,
                    &settings.azure_key_name
                ).await?;

                let ws = WsConnect::new(settings.ethereum_client_url.clone());
                let provider = ProviderBuilder::new()
                    .connect_ws(ws)
                    .await
                    .map_err(|e| BlockchainClientConnectionError::ProviderError(e.to_string()))?;
                
                Ok(Self {
                    provider: Arc::new(provider),
                    wallet: WalletType::Azure(azure_wallet),
                })
            }
            "YubiWallet" => todo!(),
            "AwsSigner" => todo!(),
            "EYTransactionManager" => todo!(),
            _ => {
                return Err(BlockchainClientConnectionError::InvalidWalletType(
                    settings.nightfall_client.wallet_type.clone()
                )) 
            }
      }
    }
}

