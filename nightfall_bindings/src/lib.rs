pub mod artifacts;

// This will include the auto-generated code_hashes.rs 
// Generated at build time by build.rs into src/code_hashes.rs
include!("code_hashes.rs");

use alloy::primitives::B256;

/// Typed accessors (small runtime wrappers)
pub fn nightfall_impl_hash() -> B256 {
    B256::from_slice(&NIGHTFALL_IMPL_HASH_BYTES)
}

pub fn round_robin_impl_hash() -> B256 {
    B256::from_slice(&ROUND_ROBIN_IMPL_HASH_BYTES)
}

pub fn x509_impl_hash() -> B256 {
    B256::from_slice(&X509_IMPL_HASH_BYTES)
}
