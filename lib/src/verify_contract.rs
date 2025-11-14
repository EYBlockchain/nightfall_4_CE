use alloy::{
    primitives::{keccak256, Address, B256},
    providers::Provider,
};

// EIP-1967 implementation slot
const EIP1967_IMPLEMENTATION_SLOT_BYTES: [u8; 32] =
    hex_literal::hex!("360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");

/// Load proxy implementation via EIP-1967
pub async fn get_proxy_implementation<P: Provider>(
    provider: &P,
    proxy: Address,
) -> eyre::Result<Address> {
    let slot = B256::from_slice(&EIP1967_IMPLEMENTATION_SLOT_BYTES);

    let raw: B256 = provider.get_storage_at(proxy, slot.into()).await?.into();

    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw[12..]); // last 20 bytes

    Ok(Address::from(addr))
}

/// Compute the on-chain implementation code hash
pub async fn get_onchain_code_hash<P: Provider>(
    provider: &P,
    implementation: Address,
) -> eyre::Result<[u8; 32]> {
    let code = provider.get_code_at(implementation).await?;

    let hash = keccak256(code.0);

    Ok(hash.0)
}

/// Verify that on-chain implementation hash matches expected local hash
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
