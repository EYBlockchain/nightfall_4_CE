use crate::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
};
use alloy::signers::utils::public_key_to_address;
use alloy::{
    consensus::SignableTransaction,
    network::{Ethereum, NetworkWallet, TxSigner},
    primitives::{Address, Signature},
    providers::{Provider, ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
};
use async_trait::async_trait;
use azure_identity;
use azure_security_keyvault::{prelude::*, KeyClient};
use base64::prelude::*;
use k256::ecdsa::{RecoveryId, Signature as K256Signature, VerifyingKey};
use k256::EncodedPoint;
use log::{debug, info};
use std::sync::Arc;
use url::Url;

#[derive(Clone, Debug)]
pub enum WalletType {
    Local(PrivateKeySigner),
    Azure(AzureWallet),
}

/// AzureWallet
/// --------------
/// This struct represents an Ethereum wallet whose private key is stored securely in Azure Key Vault.
/// It allows signing Ethereum transactions/messages without ever exposing the private key.
/// The wallet keeps only a reference to the key (key name) and the derived Ethereum address
#[derive(Clone)]
pub struct AzureWallet {
    key_client: Arc<KeyClient>, // Client to interact with Azure Key Vault
    key_name: String,           // Name of the key in Azure Key Vault
    address: Address,           // Derived Ethereum address
    verifying_key: VerifyingKey,
}

impl AzureWallet {
    /// Create a new AzureWallet
    /// -------------------------
    /// 1. Connects to Azure Key Vault
    /// 2. Fetches the public key of the Ethereum key
    /// 3. Derives the Ethereum address
    /// 4. Returns an AzureWallet instance
    pub async fn new(
        vault_url: &str,
        key_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!(" Creating Azure Wallet");
        // Create credential and KeyClient to communicate with Azure
        let credential = azure_identity::create_credential()?;
        let key_client = KeyClient::new(vault_url, credential)?;

        // Fetch the key bundle (contains the public key)
        let key_bundle = key_client.get(key_name).await?;
        debug!("Key bundle fetched successfully");

        // Extract public key and derive Ethereum address
        let verifying_key = Self::extract_public_key_from_jwk(&key_bundle)?;

        // Derive address
        let address = public_key_to_address(&verifying_key);

        Ok(Self {
            key_client: Arc::new(key_client),
            key_name: key_name.to_string(),
            address,
            verifying_key,
        })
    }

    /// Sign a message hash using the Azure Key Vault key
    /// ---------------------------------------------------
    /// The private key never leaves the HSM. The signature returned is Ethereum-compatible.
    pub async fn sign(
        &self,
        message_hash: &[u8; 32],
    ) -> Result<Signature, Box<dyn std::error::Error + Send + Sync>> {
        info!(" Signing with Azure Key Vault");
        // Encode message hash in Base64 (required by Azure
        let digest_base64 = BASE64_STANDARD.encode(message_hash);

        // Request signature from Azure Key Vault
        let sign_result = self
            .key_client
            .sign(&self.key_name, SignatureAlgorithm::ES256K, digest_base64)
            .await?;

        // let signature_bytes = match BASE64_STANDARD.decode(&sign_result.signature)
        let signature_bytes = &sign_result.signature;
        let (r, s) = Self::parse_signature(signature_bytes)?;
        // Recover v value (recovery id) for Ethereum signature
        let mut sig_bytes_64 = [0u8; 64];
        sig_bytes_64[0..32].copy_from_slice(&r);
        sig_bytes_64[32..64].copy_from_slice(&s);
        let sig = K256Signature::from_bytes(&sig_bytes_64.into())?;

        let recid =
            RecoveryId::trial_recovery_from_prehash(&self.verifying_key, message_hash, &sig)?;

        let v = recid.to_byte();

        // Construire signature finale
        let mut sig_bytes = [0u8; 65];
        sig_bytes[0..32].copy_from_slice(&r);
        sig_bytes[32..64].copy_from_slice(&s);
        sig_bytes[64] = v;

        Ok(Signature::from_raw_array(&sig_bytes)?)
    }

    /// Get the Ethereum address associated with this wallet
    pub fn address(&self) -> Address {
        self.address
    }

    // Helper functions
    //================================
    // These private functions encapsulate complex operations such as parsing DER
    // signatures, converting bytes to fixed-size arrays, and recovering Ethereum
    // addresses, providing modularity and clarity to the signing logic.

    /// Extract the uncompressed public key from the JWK (JSON Web Key) in the key bundle
    fn extract_public_key_from_jwk(
        key_bundle: &KeyVaultKey,
    ) -> Result<VerifyingKey, Box<dyn std::error::Error + Send + Sync>> {
        let jwk = &key_bundle.key;

        let curve = jwk.curve_name.as_deref();
        if curve != Some("SECP256K1") && curve != Some("P-256K") {
            return Err(format!("Expected secp256k1 curve, got {curve:?}").into());
        }

        let x = jwk.x.as_ref().ok_or("Missing x coordinate")?;
        let y = jwk.y.as_ref().ok_or("Missing y coordinate")?;
        if x.len() != 32 {
            return Err(format!("Invalid x length: expected 32, got {}", x.len()).into());
        }
        if y.len() != 32 {
            return Err(format!("Invalid y length: expected 32, got {}", y.len()).into());
        }

        // Construct  public key (0x04 || X || Y)
        let mut public_key = vec![0x04];
        public_key.extend_from_slice(x);
        public_key.extend_from_slice(y);

        let encoded_point = EncodedPoint::from_bytes(&public_key)
            .map_err(|e| format!("Invalid encoded point: {e}"))?;

        let verifying_key = VerifyingKey::from_encoded_point(&encoded_point)
            .map_err(|e| format!("Invalid verifying key: {e}"))?;

        Ok(verifying_key)
    }

    /// Parse a DER-encoded ECDSA signature to extract r and s components
    fn parse_signature(
        der: &[u8],
    ) -> Result<([u8; 32], [u8; 32]), Box<dyn std::error::Error + Send + Sync>> {
        if der.len() == 64 {
            let sig = K256Signature::from_slice(der)?;
            let normalized_sig = sig.normalize_s().unwrap_or(sig);

            let r: [u8; 32] = normalized_sig.r().to_bytes().into();
            let s: [u8; 32] = normalized_sig.s().to_bytes().into();
            return Ok((r, s));
        }
        let sig = K256Signature::from_der(der)?;
        let normalized_sig = sig.normalize_s().unwrap_or(sig);

        let r: [u8; 32] = normalized_sig.r().to_bytes().into();
        let s: [u8; 32] = normalized_sig.s().to_bytes().into();

        Ok((r, s))
    }
}

// Implement Debug trait for AzureWallet for better logging
impl std::fmt::Debug for AzureWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AzureWallet")
            .field("key_name", &self.key_name)
            .field("address", &self.address)
            .finish()
    }
}

/// Implement TxSigner for AzureWallet to sign transactions
#[async_trait]
impl TxSigner<Signature> for AzureWallet {
    fn address(&self) -> Address {
        self.address
    }
    async fn sign_transaction(
        &self,
        tx: &mut dyn alloy::consensus::SignableTransaction<Signature>,
    ) -> Result<Signature, alloy::signers::Error> {
        let signature_hash = tx.signature_hash();
        let hash_bytes: [u8; 32] = signature_hash.0;
        let signature = self
            .sign(&hash_bytes)
            .await
            .map_err(alloy::signers::Error::other)?;

        Ok(signature)
    }
}
/// Implement NetworkWallet for AzureWallet to integrate with Alloy's network layer
#[async_trait]
impl NetworkWallet<Ethereum> for AzureWallet {
    fn default_signer_address(&self) -> Address {
        self.address
    }

    fn has_signer_for(&self, address: &Address) -> bool {
        address == &self.address
    }

    fn signer_addresses(&self) -> impl Iterator<Item = Address> {
        std::iter::once(self.address)
    }
    #[allow(clippy::manual_async_fn)]
    fn sign_transaction_from(
        &self,
        sender: Address,
        tx: <Ethereum as alloy::network::Network>::UnsignedTx,
    ) -> impl std::future::Future<
        Output = Result<<Ethereum as alloy::network::Network>::TxEnvelope, alloy::signers::Error>,
    > + Send {
        async move {
            if sender != self.address {
                return Err(alloy::signers::Error::other(format!(
                    "Signer address mismatch: expected {}, got {}",
                    self.address, sender
                )));
            }
            let tx_hash = tx.signature_hash();
            let hash_bytes: [u8; 32] = tx_hash.0;

            let signature = self
                .sign(&hash_bytes)
                .await
                .map_err(alloy::signers::Error::other)?;
            let signed = tx.into_signed(signature);
            Ok(<Ethereum as alloy::network::Network>::TxEnvelope::from(
                signed,
            ))
        }
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
        // Create WebSocket provider with local signer
        let provider = ProviderBuilder::new()
            .wallet(local_signer.clone())
            .connect_ws(WsConnect::new(url.clone()))
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
    /// Get the balance of the wallet's address
    async fn get_balance(&self) -> Option<alloy::primitives::U256> {
        let address = self.get_address();
        self.provider.get_balance(address).await.ok()
    }
    /// Get the address associated with the wallet
    fn get_address(&self) -> Address {
        match &self.wallet {
            WalletType::Local(signer) => signer.address(),
            WalletType::Azure(wallet) => wallet.address(),
        }
    }
    /// Get the underlying blockchain client provider
    fn get_client(&self) -> Arc<dyn Provider> {
        self.provider.clone()
    }

    fn get_wallet_type(&self) -> &WalletType {
        &self.wallet
    }

    /// Get the PrivateKeySigner if using a local wallet
    fn get_signer(&self) -> PrivateKeySigner {
        match &self.wallet {
            WalletType::Local(signer) => signer.clone(),
            WalletType::Azure(_) => {
                panic!(
                    "Cannot get PrivateKeySigner for Azure wallet - use provider methods instead"
                )
            }
        }
    }

    /// Create a new instance from configuration settings
    async fn try_from_settings(
        settings: &Self::S,
    ) -> Result<Self, BlockchainClientConnectionError> {
        match settings.nightfall_client.wallet_type.as_str() {
            // Handle different wallet types
            "local" => {
                info!("Creating local wallet");
                // Parse the private key from settings
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
                // Initialize AzureWallet
                let azure_wallet =
                    AzureWallet::new(&settings.azure_vault_url, &settings.azure_key_name).await?;

                let ws = WsConnect::new(settings.ethereum_client_url.clone());
                let provider = ProviderBuilder::new()
                    .wallet(azure_wallet.clone())
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
                    settings.nightfall_client.wallet_type.clone(),
                ))
            }
        }
    }
}
