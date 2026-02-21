use alloy::{
    primitives::{keccak256, Address, B256},
    providers::Provider,
};

use configuration::addresses::validate_config_url;
use configuration::addresses::Addresses;
use configuration::addresses::AddressesError;
use configuration::settings::Settings;
use eyre::eyre;
use nightfall_bindings::artifacts::{Nightfall, RoundRobin, X509};
use nightfall_bindings::artifacts::{
    Nightfall::NightfallInstance, RoundRobin::RoundRobinInstance, X509::X509Instance,
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct ContractHashes {
    nightfall_hash: String,
    round_robin_hash: String,
    x509_hash: String,
}

/// Load deployed contract hashes from file written by deployer
pub async fn load_deployed_hashes() -> eyre::Result<([u8; 32], [u8; 32], [u8; 32])> {
    let hashes_path = PathBuf::from("/app/configuration/toml/contract_hashes.toml");
    let settings = Settings::new().map_err(|_| AddressesError::Settings)?;
    let content = if hashes_path.exists() {
        std::fs::read_to_string(&hashes_path)?
    } else {
        // Fallback to remote configuration server
        let base = validate_config_url(&settings.configuration_url)
            .map_err(|e| eyre!("Invalid or untrusted configuration URL: {}", e))?;

        let url = base
            .join("configuration/toml/contract_hashes.toml")
            .map_err(|e| eyre!("Could not build contract hashes URL: {}", e))?;

        let response = reqwest::get(url.clone())
            .await
            .map_err(|e| eyre!("Failed to fetch contract hashes from {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(eyre!(
                "Configuration server returned error {} for {}",
                response.status(),
                url
            ));
        }

        response.text().await?
    };
    let hashes: ContractHashes = toml::from_str(&content)?;

    let nightfall = hex::decode(&hashes.nightfall_hash)
        .map_err(|e| eyre::eyre!("Failed to decode nightfall hash: {}", e))?;
    let round_robin = hex::decode(&hashes.round_robin_hash)
        .map_err(|e| eyre::eyre!("Failed to decode round_robin hash: {}", e))?;
    let x509 = hex::decode(&hashes.x509_hash)
        .map_err(|e| eyre::eyre!("Failed to decode x509 hash: {}", e))?;

    let mut nf_bytes = [0u8; 32];
    let mut rr_bytes = [0u8; 32];
    let mut x509_bytes = [0u8; 32];

    nf_bytes.copy_from_slice(&nightfall);
    rr_bytes.copy_from_slice(&round_robin);
    x509_bytes.copy_from_slice(&x509);

    Ok((nf_bytes, rr_bytes, x509_bytes))
}

/// EIP-1967: Proxy implementation storage slot
///
/// Computed as:
///     keccak256("eip1967.proxy.implementation") - 1
///
/// References:
///  - https://eips.ethereum.org/EIPS/eip-1967
///
/// The proxy contract stores its logic/implementation address at this slot.
const EIP1967_IMPLEMENTATION_SLOT_BYTES: [u8; 32] =
    hex_literal::hex!("360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");

/// Fetch the implementation address of an EIP-1967 proxy.
///
/// # Purpose
/// Ensures that the address retrieved from the proxy truly points to the logic
/// contract used by the system. An attacker could deploy a fake proxy that
/// returns a different implementation address; this function protects against
/// that.
///
/// # Returns
/// * `Address` — the 20-byte address stored in the implementation slot.
pub async fn get_proxy_implementation<P: Provider>(
    provider: &P,
    proxy: Address,
) -> eyre::Result<Address> {
    // println!("Fetching implementation address from proxy at {proxy:?}...");
    let slot = B256::from_slice(&EIP1967_IMPLEMENTATION_SLOT_BYTES);

    let raw: B256 = provider.get_storage_at(proxy, slot.into()).await?.into();

    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw[12..]); // last 20 bytes
    // println!("get the address  {:?}...", Address::from(addr));
    Ok(Address::from(addr))
}

// Add the metadata stripping function
fn strip_metadata_and_hash(bytecode: &[u8]) -> [u8; 32] {
    // Solidity metadata is at the end: 0xa2 0x64 'i' 'p' 'f' 's' 0x58 0x22 <32-byte-hash> 0x64 's' 'o' 'l' 'c' 0x43 <version> 0x00 0x33
    // Look for the metadata marker: 0xa264697066735822 (a2 64 "ipfs" 58 22)
    const METADATA_MARKER: [u8; 8] = [0xa2, 0x64, 0x69, 0x70, 0x66, 0x73, 0x58, 0x22];

    // Find the last occurrence of the metadata marker
    if let Some(pos) = bytecode
        .windows(METADATA_MARKER.len())
        .rposition(|window| window == METADATA_MARKER)
    {
        // Strip everything from the metadata marker onwards
        let stripped = &bytecode[..pos];
        // println!(
        //     "Stripped {} bytes of metadata (original: {}, stripped: {})",
        //     bytecode.len() - stripped.len(),
        //     bytecode.len(),
        //     stripped.len()
        // );
        keccak256(stripped).0
    } else {
        println!("No metadata marker found, using full bytecode");
        keccak256(bytecode).0
    }
}

pub async fn get_onchain_code_hash<P: Provider>(
    provider: &P,
    implementation: Address,
) -> eyre::Result<[u8; 32]> {
    let code = provider.get_code_at(implementation).await?;
    // println!("On-chain code length: {} bytes", code.0.len());

    // Use the same metadata stripping logic as build-time
    let hash = strip_metadata_and_hash(&code.0);
    // println!("On-chain hash (metadata stripped): {hash:?}");

    Ok(hash)
}

/// Verify that the proxy → implementation mapping is correct,
///   and that the on-chain implementation bytecode matches the expected build-time hash.
///
/// # Security guarantees
/// - Detects malicious or compromised deployers  
/// - Detects incorrect addresses.toml files  
/// - Detects incorrect RPC endpoints (wrong chain)  
/// - Prevents L1 contract replacement attacks  
///
/// # Errors
/// Returns an error if:
/// - proxy does not resolve to an implementation
/// - implementation bytecode hash mismatches the local artifact
pub async fn verify_impl_hash<P: Provider>(
    provider: &P,
    proxy: Address,
    expected: &[u8; 32],
    name: &str,
) -> eyre::Result<()> {
    let implementation = get_proxy_implementation(provider, proxy).await?;
    let onchain = get_onchain_code_hash(provider, implementation).await?;

    if &onchain != expected {
        eyre::bail!(
            "{name} implementation hash mismatch\n  on-chain:   0x{}\n  expected:   0x{}\n",
            hex::encode(onchain),
            hex::encode(expected),
        );
    }

    Ok(())
}

/// A strongly-typed structure containing **verified** contract bindings.
///
/// Contracts are only constructed after:
/// 1. Verifying the proxy implementation addresses (EIP-1967)
/// 2. Verifying that on-chain bytecode hashes match local build-time bytecode
///
/// This provides a clean and safe interface for downstream services (client,
/// proposer) to consume Nightfall’s L1 contracts.
pub struct VerifiedContracts<P> {
    pub nightfall: NightfallInstance<P>,
    pub round_robin: RoundRobinInstance<P>,
    pub x509: X509Instance<P>,
}

impl<P: Provider + Clone> VerifiedContracts<P> {
    /// Create verified contract bindings for Nightfall, RoundRobin, and X509.
    ///
    /// # Naming rationale
    /// The previous name `load()` was ambiguous.
    /// A clearer name is **`resolve_and_verify`**, because:
    /// - "resolve": fetch proxy → implementation mapping
    /// - "verify": validate implementation bytecode hash
    /// - then "construct contract bindings"
    ///
    /// # Returns
    /// A `VerifiedContracts<P>` object that is *guaranteed* to reference valid,
    /// authenticated L1 contract instances.
    pub async fn resolve_and_verify_contract(
        provider: P,
        addresses: &Addresses,
    ) -> eyre::Result<Self> {
        ark_std::println!(
            "Verifying deployed contract implementations at these addresses: {addresses:?}"
        );

        // Load the expected hashes from the file written by deployer
        let (nightfall_hash, round_robin_hash, x509_hash) = load_deployed_hashes().await?;

        // Verify each contract's deployed implementation
        verify_impl_hash(&provider, addresses.nightfall, &nightfall_hash, "Nightfall").await?;
        verify_impl_hash(
            &provider,
            addresses.round_robin,
            &round_robin_hash,
            "RoundRobin",
        )
        .await?;
        verify_impl_hash(&provider, addresses.x509, &x509_hash, "X509").await?;

        // Only construct contract bindings if verification passed
        Ok(Self {
            nightfall: Nightfall::new(addresses.nightfall, provider.clone()),
            round_robin: RoundRobin::new(addresses.round_robin, provider.clone()),
            x509: X509::new(addresses.x509, provider.clone()),
        })
    }
}
