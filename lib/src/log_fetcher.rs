//! Paginated log fetching utilities for robust multi-chain event retrieval.
//!
//! This module provides utilities to fetch blockchain logs with automatic pagination
//! to handle RPC provider block range limits gracefully.

use alloy::primitives::Log;
use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use configuration::settings::get_settings;
use log::{debug, warn};
use std::error::Error;
use std::fmt;

/// Error type for log fetching operations.
#[derive(Debug)]
pub enum LogFetchError {
    /// RPC provider returned an error
    ProviderError(String),
    /// Block range exceeds provider limits even after reduction
    BlockRangeTooLarge,
    /// Query returned too many results
    TooManyResults,
}

impl fmt::Display for LogFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogFetchError::ProviderError(msg) => write!(f, "Provider error: {msg}"),
            LogFetchError::BlockRangeTooLarge => {
                write!(f, "Block range exceeds provider limits")
            }
            LogFetchError::TooManyResults => {
                write!(f, "Query returned too many results")
            }
        }
    }
}

impl Error for LogFetchError {}

/// Returns the optimal chunk size for log queries based on the current chain configuration.
///
/// The chunk size is determined in the following order:
/// 1. Explicit `log_chunk_size` from config (if set)
/// 2. Known defaults for common chain IDs
/// 3. A safe fallback of 5,000 blocks
///
/// # Examples
///
/// ```ignore
/// let chunk_size = get_log_chunk_size();
/// // For Anvil (chain_id 31337): returns u64::MAX (unlimited)
/// // For Base Sepolia (chain_id 84532): returns 10,000
/// // For Ethereum mainnet (chain_id 1): returns 5,000
/// ```
pub fn get_log_chunk_size() -> u64 {
    let settings = get_settings();

    // 1. Try config-specified chunk size first
    if let Some(chunk) = settings.network.log_chunk_size {
        return chunk;
    }

    // 2. Fall back to known defaults by chain ID
    match settings.network.chain_id {
        // Local development - no limits
        31337 => u64::MAX,

        // Ethereum mainnet & testnets
        1 | 5 | 11155111 => 5_000,

        // Polygon & Mumbai
        137 | 80001 | 80002 => 3_000,

        // BSC & BSC Testnet
        56 | 97 => 5_000,

        // Arbitrum One & Sepolia
        42161 | 421614 => 10_000,

        // Optimism & Sepolia
        10 | 11155420 => 10_000,

        // Base & Base Sepolia
        8453 | 84532 => 10_000,

        // Avalanche C-Chain & Fuji
        43114 | 43113 => 100_000,

        // Gnosis & Cronos
        100 | 25 => 10_000,

        // Plume mainnet
        98866 => 100,

        // Plume testnet
        98867 => 1_000,

        // Safe default for unknown chains
        _ => 5_000,
    }
}

/// Returns the genesis block number from configuration.
///
/// This should be used instead of hardcoding `0` for event queries.
#[inline]
pub fn get_genesis_block() -> u64 {
    get_settings().genesis_block as u64
}

/// Fetches logs with automatic pagination for large block ranges.
///
/// This function handles RPC provider block range limits by splitting large queries
/// into smaller chunks. It uses adaptive retry with reduced chunk sizes when
/// encountering "block range too large" or "too many results" errors.
///
/// # Arguments
///
/// * `provider` - The blockchain provider to query
/// * `base_filter` - Base filter with address and event signatures (block range will be overwritten)
/// * `from_block` - Starting block number (inclusive)
/// * `to_block` - Ending block number (inclusive)
///
/// # Returns
///
/// A vector of all logs matching the filter across the specified block range.
pub async fn get_logs_paginated<P: Provider>(
    provider: &P,
    base_filter: Filter,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<Log>, LogFetchError> {
    // Short-circuit if range is empty or invalid
    if from_block > to_block {
        return Ok(Vec::new());
    }

    let initial_chunk_size = get_log_chunk_size();

    // If chunk size is unlimited (local dev), try single query first
    if initial_chunk_size == u64::MAX {
        let filter = base_filter
            .clone()
            .from_block(from_block)
            .to_block(to_block);

        return provider
            .get_logs(&filter)
            .await
            .map(|logs| logs.into_iter().map(|l| l.inner).collect())
            .map_err(|e| LogFetchError::ProviderError(e.to_string()));
    }

    let mut all_logs = Vec::new();
    let mut current_from = from_block;
    let mut chunk_size = initial_chunk_size;
    let min_chunk_size = 100u64;

    while current_from <= to_block {
        let current_to = current_from.saturating_add(chunk_size - 1).min(to_block);

        let filter = base_filter
            .clone()
            .from_block(current_from)
            .to_block(current_to);

        debug!(
            "Fetching logs from block {current_from} to {current_to} (chunk_size: {chunk_size})"
        );

        match provider.get_logs(&filter).await {
            Ok(logs) => {
                all_logs.extend(logs.into_iter().map(|l| l.inner));
                current_from = current_to.saturating_add(1);
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();

                if is_range_error(&error_msg) || is_result_limit_error(&error_msg) {
                    // Reduce chunk size and retry
                    let new_chunk_size = chunk_size / 2;

                    if new_chunk_size < min_chunk_size {
                        warn!(
                            "Chunk size {new_chunk_size} is below minimum {min_chunk_size}, cannot reduce further"
                        );
                        return Err(if is_result_limit_error(&error_msg) {
                            LogFetchError::TooManyResults
                        } else {
                            LogFetchError::BlockRangeTooLarge
                        });
                    }

                    warn!(
                        "Reducing chunk size from {chunk_size} to {new_chunk_size} due to RPC limit"
                    );
                    chunk_size = new_chunk_size;
                    // Don't advance current_from, retry with smaller chunk
                } else {
                    // Unknown error, propagate it
                    return Err(LogFetchError::ProviderError(e.to_string()));
                }
            }
        }
    }

    debug!("Fetched {} total logs", all_logs.len());
    Ok(all_logs)
}

/// Checks if an error message indicates a block range limit was exceeded.
fn is_range_error(msg: &str) -> bool {
    msg.contains("block range")
        || msg.contains("range too large")
        || msg.contains("exceed")
        || msg.contains("too wide")
        || msg.contains("query timeout")
}

/// Checks if an error message indicates too many results were returned.
fn is_result_limit_error(msg: &str) -> bool {
    msg.contains("10000 results")
        || msg.contains("too many")
        || msg.contains("response size")
        || msg.contains("limit exceeded")
}
