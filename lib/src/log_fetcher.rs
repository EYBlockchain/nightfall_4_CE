//! Paginated log fetching utilities for robust multi-chain event retrieval.
//!
//! This module provides utilities to fetch blockchain logs with automatic pagination
//! to handle RPC provider block range limits gracefully.

use alloy::providers::Provider;
use alloy::rpc::types::{Filter, Log};
use configuration::settings::get_settings;
use log::{debug, info, warn};
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
    if let Some(chunk) = settings.log_chunk_size {
        log::info!(
            "Using configured log_chunk_size: {chunk} (chain_id: {})",
            settings.network.chain_id
        );
        return chunk;
    }

    // 2. Fall back to known defaults by chain ID
    let chunk_size = match settings.network.chain_id {
        // Local development - no limits
        31337 => u64::MAX,

        // Ethereum mainnet & testnets
        1 | 5 | 11155111 => 5_000,

        // Polygon, Mumbai, Amoy
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
    };

    if chunk_size == u64::MAX {
        debug!(
            "Using chain-id default log_chunk_size: unlimited (chain_id: {})",
            settings.network.chain_id
        );
    } else {
        debug!(
            "Using chain-id default log_chunk_size: {} (chain_id: {})",
            chunk_size, settings.network.chain_id
        );
    }
    chunk_size
}

/// Returns the genesis block number from configuration.
///
/// This should be used instead of hardcoding `0` for event queries.
#[inline]
pub fn get_genesis_block() -> u64 {
    get_settings().genesis_block as u64
}

/// Returns the max RPC calls per second (0 = unlimited).
#[inline]
pub fn get_rpc_rate_limit() -> u32 {
    get_settings().rpc_rate_limit
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
        debug!(
            "Chunk size is unlimited - fetching all logs in single query (blocks {from_block} to {to_block})"
        );
        let filter = base_filter
            .clone()
            .from_block(from_block)
            .to_block(to_block);

        return provider
            .get_logs(&filter)
            .await
            .map_err(|e| LogFetchError::ProviderError(e.to_string()));
    }

    let total_blocks = to_block.saturating_sub(from_block) + 1;
    let estimated_chunks_initial = total_blocks.div_ceil(initial_chunk_size);
    log::info!(
        "Fetching logs: blocks {from_block} to {to_block} ({total_blocks} blocks, chunk_size: {initial_chunk_size}, ~{estimated_chunks_initial} chunks)"
    );

    let mut all_logs = Vec::new();
    let mut current_from = from_block;
    let mut chunk_size = initial_chunk_size;
    let min_chunk_size = 1u64;
    let mut chunk_count = 0u64;
    let mut rate_limit_retries = 0u32;
    let max_rate_limit_retries = 5;

    // Rate limiting: calculate delay between calls if rate limit is set
    let rpc_rate_limit = get_rpc_rate_limit();
    let delay_between_calls = if rpc_rate_limit > 0 {
        Some(std::time::Duration::from_secs_f64(1.0 / rpc_rate_limit as f64))
    } else {
        None
    };
    
    if let Some(delay) = delay_between_calls {
        log::info!("RPC rate limit: {rpc_rate_limit} calls/sec (delay: {}ms per call)", delay.as_millis());
    }

    let mut last_call_time = std::time::Instant::now();

    while current_from <= to_block {
        chunk_count += 1;
        let current_to = current_from.saturating_add(chunk_size - 1).min(to_block);

        let remaining_blocks = to_block.saturating_sub(current_from) + 1;
        let estimated_remaining_chunks = remaining_blocks.div_ceil(chunk_size);
        let estimated_total_chunks = chunk_count.saturating_sub(1) + estimated_remaining_chunks;

        let filter = base_filter
            .clone()
            .from_block(current_from)
            .to_block(current_to);

        // Log progress every chunk to track progress
        info!(
            "[Chunk {chunk_count}/~{estimated_total_chunks}] Fetching logs: blocks {current_from} to {current_to} (chunk_size: {chunk_size})"
        );

        // Rate limiting: wait if needed before making the RPC call
        if let Some(delay) = delay_between_calls {
            let elapsed = last_call_time.elapsed();
            if elapsed < delay {
                tokio::time::sleep(delay - elapsed).await;
            }
        }

        match provider.get_logs(&filter).await {
            Ok(logs) => {
                let logs_in_chunk = logs.len();
                let total_so_far = all_logs.len() + logs_in_chunk;
                all_logs.extend(logs);
                info!("[Chunk {chunk_count}] Retrieved {logs_in_chunk} logs (total so far: {total_so_far})");
                current_from = current_to.saturating_add(1);
                last_call_time = std::time::Instant::now();
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                warn!("RPC error (will retry): {error_msg}");

                if is_rate_limit_error(&error_msg) {
                    rate_limit_retries += 1;
                    if rate_limit_retries > max_rate_limit_retries {
                        warn!(
                            "Rate limited {rate_limit_retries} times, giving up after {max_rate_limit_retries} retries"
                        );
                        return Err(LogFetchError::ProviderError(e.to_string()));
                    }
                    
                    // Exponential backoff: 2s, 4s, 8s, 16s, 32s
                    let backoff_secs = 2u64.pow(rate_limit_retries.min(5));
                    warn!(
                        "Rate limited by RPC provider (attempt {rate_limit_retries}/{max_rate_limit_retries}), waiting {backoff_secs}s before retry..."
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
                    // Continue without reducing chunk size - retry same request
                } else if is_range_error(&error_msg) || is_result_limit_error(&error_msg) {
                    // Reduce chunk size by half and retry
                    let new_chunk_size = chunk_size.saturating_div(2).max(min_chunk_size);

                    if new_chunk_size == min_chunk_size && chunk_size == min_chunk_size {
                        // Already at minimum and still failing - this is an RPC limitation
                        warn!(
                            "Chunk size already at minimum {min_chunk_size} and still failing - RPC provider block range limit is too restrictive"
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
                    // Add delay to avoid rate limiting
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    // Don't advance current_from, retry with smaller chunk
                } else {
                    // Unknown error, propagate it
                    return Err(LogFetchError::ProviderError(e.to_string()));
                }
            }
        }
    }

    let total_logs = all_logs.len();
    debug!(
        "Pagination complete: {total_logs} total logs fetched across {chunk_count} chunks (initial chunk_size: {initial_chunk_size}, final chunk_size: {chunk_size})"
    );
    Ok(all_logs)
}

/// Checks if an error message indicates a block range limit was exceeded.
fn is_range_error(msg: &str) -> bool {
    msg.contains("block range")
        || msg.contains("range too large")
        || msg.contains("block range limit")
        || msg.contains("too wide")
        || msg.contains("query timeout")
        || msg.contains("10 block range")
        || msg.contains("10 block")
        || msg.contains("超出")
}

/// Checks if an error message indicates too many results were returned.
fn is_result_limit_error(msg: &str) -> bool {
    msg.contains("10000 results")
        || msg.contains("too many")
        || msg.contains("response size")
        || msg.contains("limit exceeded")
}

/// Checks if an error message indicates a rate limit (429)
fn is_rate_limit_error(msg: &str) -> bool {
    msg.contains("rate limit")
        || msg.contains("429")
        || msg.contains("too many requests")
        || msg.contains("throttl")
}