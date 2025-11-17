use alloy::{
    primitives::{keccak256, Address, B256},
    providers::Provider,
};

use configuration::addresses::Addresses;
use nightfall_bindings::artifacts::{
    Nightfall::NightfallInstance, RoundRobin::RoundRobinInstance, X509::X509Instance,
};
use nightfall_bindings::{
    artifacts::{Nightfall, RoundRobin, X509},
    NIGHTFALL_IMPL_HASH_BYTES, ROUND_ROBIN_IMPL_HASH_BYTES, X509_IMPL_HASH_BYTES,
};

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
    print!("Fetching implementation address from proxy at {:?}...\n", proxy);
    let slot = B256::from_slice(&EIP1967_IMPLEMENTATION_SLOT_BYTES);

    let raw: B256 = provider.get_storage_at(proxy, slot.into()).await?.into();

    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw[12..]); // last 20 bytes
    print!("get the address  {:?}...\n", Address::from(addr));
    Ok(Address::from(addr))
}

// use alloy::{
//     primitives::{keccak256, Address, B256},
//     providers::Provider,
// };

// Add the metadata stripping function
fn strip_metadata_and_hash(bytecode: &[u8]) -> [u8; 32] {
    // Solidity metadata is at the end: 0xa2 0x64 'i' 'p' 'f' 's' 0x58 0x22 <32-byte-hash> 0x64 's' 'o' 'l' 'c' 0x43 <version> 0x00 0x33
    // Look for the metadata marker: 0xa264697066735822 (a2 64 "ipfs" 58 22)
    const METADATA_MARKER: [u8; 8] = [0xa2, 0x64, 0x69, 0x70, 0x66, 0x73, 0x58, 0x22];
    
    // Find the last occurrence of the metadata marker
    if let Some(pos) = bytecode.windows(METADATA_MARKER.len())
        .rposition(|window| window == METADATA_MARKER) {
        // Strip everything from the metadata marker onwards
        let stripped = &bytecode[..pos];
        println!("Stripped {} bytes of metadata (original: {}, stripped: {})", 
                 bytecode.len() - stripped.len(), bytecode.len(), stripped.len());
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
    println!("On-chain code length: {} bytes", code.0.len());

    // Use the same metadata stripping logic as build-time
    let hash = strip_metadata_and_hash(&code.0);
    println!("On-chain hash (metadata stripped): {:?}", hash);

    Ok(hash)
}

/// Compute the keccak256 hash of the runtime bytecode currently deployed
/// at a given implementation address.
///
/// # Why?
/// This code hash is compared with a build-time hash generated from local
/// artifacts to ensure critical L1 contracts have not been tampered with.
///
/// # Returns
/// * `[u8; 32]` — keccak256(code)
// pub async fn get_onchain_code_hash<P: Provider>(
//     provider: &P,
//     implementation: Address,
// ) -> eyre::Result<[u8; 32]> {
//     let code = provider.get_code_at(implementation).await?;
//     println!("inside get_onchain_code_hash, On-chain code at {:?}: {:?}", implementation, code);
//     println!("on chain Code length: {} bytes", code.0.len());

//     let hash = keccak256(code.0);

//     println!("hash onchain: {:?}", hash.0);

//     Ok(hash.0)
// }

/// Verify that the proxy → implementation mapping is correct,
/// and that the on-chain implementation bytecode matches the expected build-time hash.
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
        ark_std::println!("Verifying deployed contract implementations at these addresses: {:?}", addresses);
        // Verify each contract's deployed implementation
        verify_impl_hash(
            &provider,
            addresses.nightfall,
            &NIGHTFALL_IMPL_HASH_BYTES,
            "Nightfall",
        )
        .await?;
        verify_impl_hash(
            &provider,
            addresses.round_robin,
            &ROUND_ROBIN_IMPL_HASH_BYTES,
            "RoundRobin",
        )
        .await?;
        verify_impl_hash(&provider, addresses.x509, &X509_IMPL_HASH_BYTES, "X509").await?;

        // Only construct contract bindings if verification passed
        Ok(Self {
            nightfall: Nightfall::new(addresses.nightfall, provider.clone()),
            round_robin: RoundRobin::new(addresses.round_robin, provider.clone()),
            x509: X509::new(addresses.x509, provider.clone()),
        })
    }
}
