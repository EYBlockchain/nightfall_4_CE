use crate::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
};
use alloy::providers::WsConnect;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use async_trait::async_trait;
use k256::ecdsa::{RecoveryId, Signature as K256Signature, VerifyingKey};
use std::sync::Arc;
use url::Url;

use alloy::primitives::{keccak256, Address, Signature};
use azure_identity;
use azure_security_keyvault::prelude::*;
use azure_security_keyvault::KeyClient;
use base64::prelude::*;
use log::{debug, info};

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
        debug!("Key bundle received: {key_bundle:#?}");

        // Extract public key and derive Ethereum address
        let public_key = Self::extract_public_key_from_jwk(&key_bundle)?;
        let address = Self::derive_ethereum_address(&public_key)?;

        info!(" Azure Wallet initialized, address: {address:?}");

        Ok(Self {
            key_client: Arc::new(key_client),
            key_name: key_name.to_string(),
            address,
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
        debug!("Signature result: {sign_result:#?}");

        // Decode DER signature from Base64
        let signature_bytes = BASE64_STANDARD.decode(&sign_result.signature)?;
        // Parse DER signature to obtain r and s
        let (r, s) = Self::parse_der_signature(&signature_bytes)?;
        // Calculate recovery ID v
        let v = Self::calculate_recovery_id(message_hash, &r, &s, &self.address)?;
        // Construct the final Ethereum signature (r, s, v)
        let mut sig_bytes = [0u8; 65];
        sig_bytes[0..32].copy_from_slice(&r);
        sig_bytes[32..64].copy_from_slice(&s);
        sig_bytes[64] = v;

        info!("Signature created");
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
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let jwk = &key_bundle.key;

        // Ensure the key is on the SECP256K1 curve
        if jwk.curve_name.as_deref() != Some("SECP256K1") {
            return Err("Expected SECP256K1 curve".into());
        }

        let x = jwk.x.as_ref().ok_or("Missing x coordinate")?;
        let y = jwk.y.as_ref().ok_or("Missing y coordinate")?;

        // Decode Base64URL coordinates
        let x_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(x)?;
        let y_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(y)?;

        // Construct  public key (0x04 || X || Y)
        let mut public_key = vec![0x04];
        public_key.extend_from_slice(&x_bytes);
        public_key.extend_from_slice(&y_bytes);

        Ok(public_key)
    }

    /// Derive the Ethereum address from the  public key
    fn derive_ethereum_address(
        public_key: &[u8],
    ) -> Result<Address, Box<dyn std::error::Error + Send + Sync>> {
        if public_key.len() != 65 || public_key[0] != 0x04 {
            return Err("Invalid public key format".into());
        }

        let hash = keccak256(&public_key[1..]);
        let address_bytes = &hash[12..32];

        Ok(Address::from_slice(address_bytes))
    }
    /// Parse a DER-encoded ECDSA signature to extract r and s components
    fn parse_der_signature(
        der: &[u8],
    ) -> Result<([u8; 32], [u8; 32]), Box<dyn std::error::Error + Send + Sync>> {
        // Basic DER structure check
        if der.len() < 8 || der[0] != 0x30 {
            return Err("Invalid DER signature".into());
        }

        let mut pos = 2;
        // Parse r
        if der[pos] != 0x02 {
            return Err("Expected INTEGER for r".into());
        }
        pos += 1;

        let r_len = der[pos] as usize;
        pos += 1;

        let r_bytes = &der[pos..pos + r_len];
        pos += r_len;

        if der[pos] != 0x02 {
            return Err("Expected INTEGER for s".into());
        }
        pos += 1;

        let s_len = der[pos] as usize;
        pos += 1;

        let s_bytes = &der[pos..pos + s_len];

        let r = Self::to_32_bytes(r_bytes)?;
        let mut s = Self::to_32_bytes(s_bytes)?;
        s = Self::normalize_s(&s);

        Ok((r, s))
    }

    /// Normalize s to prevent signature malleability
    /// 
    /// Ethereum requires that s is in the lower half of the curve order
    /// to prevent signature malleability attacks (EIP-2).
    fn normalize_s(s: &[u8; 32]) -> [u8; 32] {
    use k256::elliptic_curve::ops::Reduce;
    use k256::{Scalar, U256};
    
    let s_scalar = Scalar::reduce(U256::from_be_slice(s));
    
    // n/2 for secp256k1
    let half_n = Scalar::reduce(U256::from_be_hex(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0"
    ));
    
    if s_scalar > half_n {
        // Return n - s
        let n = Scalar::reduce(U256::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"
        ));
        let normalized = n - s_scalar;
        
        let bytes = normalized.to_bytes();
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes);
        result
    } else {
        *s
    }
}

    /// Convert a byte slice to a 32-byte array, padding or truncating as necessary
    fn to_32_bytes(bytes: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error + Send + Sync>> {
        let mut result = [0u8; 32];

        if bytes.len() > 32 {
            result.copy_from_slice(&bytes[bytes.len() - 32..]);
        } else {
            let offset = 32 - bytes.len();
            result[offset..].copy_from_slice(bytes);
        }

        Ok(result)
    }
    /// Calculate the recovery ID (v) for the given signature and message hash
    fn calculate_recovery_id(
        message_hash: &[u8; 32],
        r: &[u8; 32],
        s: &[u8; 32],
        expected_address: &Address,
    ) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        let sig = K256Signature::from_scalars(*r, *s)?;

        for recovery_id in [0u8, 1u8] {
            if let Ok(recid) = RecoveryId::try_from(recovery_id) {
                if let Ok(verifying_key) =
                    VerifyingKey::recover_from_prehash(message_hash, &sig, recid)
                {
                    let public_key = verifying_key.to_encoded_point(false);
                    let public_key_bytes = public_key.as_bytes();

                    if let Ok(recovered_address) = Self::derive_ethereum_address(public_key_bytes) {
                        if recovered_address == *expected_address {
                            debug!("Recovery ID found: {recovery_id}");
                            return Ok(recovery_id + 27); // Ethereum expects v = 27 or 28
                        }
                    }
                }
            }
        }
        Err("Could not calculate recovery ID".into())
    }
    #[allow(dead_code)]
    fn recover_address(
        message_hash: &[u8; 32],
        r: &[u8; 32],
        s: &[u8; 32],
        recovery_id: u8,
    ) -> Result<Address, Box<dyn std::error::Error + Send + Sync>> {
        // Reconstruct the signature
        let sig = K256Signature::from_scalars(*r, *s)?;
        // Convert recovery_id to RecoveryId
        let recid = RecoveryId::try_from(recovery_id - 27).map_err(|_| "Invalid recovery ID")?;

        let verifying_key = VerifyingKey::recover_from_prehash(message_hash, &sig, recid)?;
        // Derive Ethereum address from the public key
        let public_key = verifying_key.to_encoded_point(false);
        let public_key_bytes = public_key.as_bytes();

        Self::derive_ethereum_address(public_key_bytes)
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
    /// Get the PrivateKeySigner if using a local wallet
    fn get_signer(&self) -> PrivateKeySigner {
        match &self.wallet {
            WalletType::Local(signer) => signer.clone(),
            WalletType::Azure(_) => {
                panic!("Cannot get PrivateKeySigner for Azure wallet - key never leaves HSM")
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
                info!("Creating Azure Key Vault wallet");
                // Initialize AzureWallet
                let azure_wallet =
                    AzureWallet::new(&settings.azure_vault_url, &settings.azure_key_name).await?;

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
                    settings.nightfall_client.wallet_type.clone(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ecdsa::signature::hazmat::PrehashSigner;
    use k256::ecdsa::Signature as K256Signature;
    use k256::ecdsa::SigningKey;
    use rand::rngs::OsRng;

    /// Mock AzureWallet for testing purposes
    struct MockAzureWallet {
        address: Address,
    }
    impl MockAzureWallet {
        async fn sign(&self, _msg: &[u8; 32]) -> Result<[u8; 65], Box<dyn std::error::Error>> {
            Ok([0u8; 65])
        }
        fn address(&self) -> Address {
            self.address
        }
    }

    #[tokio::test]
    async fn test_azure_wallet_signing() {
        let expected_address: Address = "0x1234567890abcdef1234567890abcdef12345678"
            .parse()
            .unwrap();

        let wallet = MockAzureWallet {
            address: expected_address,
        };
        assert_eq!(wallet.address(), expected_address);

        let message_hash = [0u8; 32];
        let signature = wallet.sign(&message_hash).await.unwrap();
        assert_eq!(signature.len(), 65);
    }

    #[tokio::test]
    async fn test_full_signature_flow() {
        // 1 Generate a local signing key (mocking wallet)
        let mut rng = OsRng;
        let signing_key = SigningKey::random(&mut rng);
        let verifying_key = signing_key.verifying_key();

        // 2 Derive the Ethereum address from the public key
        let binding = verifying_key.to_encoded_point(false);
        let public_key_bytes = binding.as_bytes();
        let expected_address =
            AzureWallet::derive_ethereum_address(public_key_bytes).expect("Should derive address");

        // 3 Message to sign (hash)
        let message_hash = [0u8; 32]; // mock 32-byte hash

        // 4 Sign the message
        let signature: K256Signature = signing_key.sign_prehash(&message_hash).unwrap();
        let sig_bytes = signature.to_bytes();
        let r: [u8; 32] = sig_bytes[0..32].try_into().unwrap();
        let s: [u8; 32] = sig_bytes[32..64].try_into().unwrap();

        // 5 Calculate recovery ID (Ethereum expects v = 27 or 28)
        let v = AzureWallet::calculate_recovery_id(&message_hash, &r, &s, &expected_address)
            .expect("Should calculate recovery ID");

        // 6 Recover the address using your helper
        let recovered_address =
            AzureWallet::recover_address(&message_hash, &r, &s, v).expect("Recovery should work");

        // 7 Verify recovered address matches expected
        assert_eq!(recovered_address, expected_address);

        // 8 Optionally, calculate the recovery ID as Ethereum expects (27/28)
        let calculated_v =
            AzureWallet::calculate_recovery_id(&message_hash, &r, &s, &expected_address)
                .expect("Should calculate recovery ID");

        assert!(calculated_v == 27 || calculated_v == 28);
    }
}
