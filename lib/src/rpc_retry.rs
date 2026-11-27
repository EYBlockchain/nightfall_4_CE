//! Retry transient JSON-RPC failures. A public Sepolia endpoint can reject one
//! `eth_sendRawTransaction` with `-32005 rate limit exceeded` and still accept
//! the same raw transaction a few seconds later. That must not fail a deposit.

use alloy::{network::Network, primitives::TxHash, providers::Provider};
use log::warn;
use std::{future::IntoFuture, time::Duration};

const ATTEMPTS: u32 = 8;

/// True for a provider rejection that is safe to repeat with the same request.
pub fn is_transient_rpc_error(msg: &str) -> bool {
    let msg = msg.to_ascii_lowercase();
    msg.contains("rate limit")
        || msg.contains("-32005")
        || msg.contains("429")
        || msg.contains("too many requests")
        || msg.contains("throttl")
        || msg.contains("temporarily unavailable")
        || msg.contains("timeout")
        || msg.contains("timed out")
        || msg.contains("connection reset")
        || msg.contains("broken pipe")
}

/// Run `op` again when the error is a transient RPC rejection.
///
/// The closure must build a new future on every call. Non-transient errors,
/// including a reverted transaction, are returned immediately.
pub async fn retry_transient<T, E, F, Fut>(what: &str, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: IntoFuture<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = Duration::from_secs(2);
    for attempt in 1..=ATTEMPTS {
        match op().into_future().await {
            Ok(value) => return Ok(value),
            Err(err) if !is_transient_rpc_error(&err.to_string()) || attempt == ATTEMPTS => {
                return Err(err);
            }
            Err(err) => {
                warn!("{what} transient RPC error (attempt {attempt}/{ATTEMPTS}): {err}");
                tokio::time::sleep(delay).await;
                delay = (delay * 2).min(Duration::from_secs(15));
            }
        }
    }
    unreachable!("retry loop returns on every attempt")
}

/// Broadcast one signed transaction, retrying only a rate-limit or transport
/// rejection. The raw bytes stay the same, so a rejected send is not a second
/// escrow. The receipt watch is polled by hash if that call is rate limited.
pub async fn confirm_raw_transaction<P, N>(
    provider: &P,
    raw_tx: &[u8],
    what: &str,
) -> Result<N::ReceiptResponse, String>
where
    P: Provider<N>,
    N: Network,
{
    let pending = retry_transient(&format!("{what} send"), || {
        provider.send_raw_transaction(raw_tx)
    })
    .await
    .map_err(|err| err.to_string())?;
    let hash = *pending.tx_hash();
    match pending.get_receipt().await {
        Ok(receipt) => Ok(receipt),
        Err(err) if is_transient_rpc_error(&err.to_string()) => {
            warn!("{what} receipt watch hit a transient RPC error: {err}");
            poll_receipt(provider, hash, what).await
        }
        Err(err) => Err(err.to_string()),
    }
}

async fn poll_receipt<P, N>(
    provider: &P,
    hash: TxHash,
    what: &str,
) -> Result<N::ReceiptResponse, String>
where
    P: Provider<N>,
    N: Network,
{
    let mut delay = Duration::from_secs(2);
    for attempt in 1..=ATTEMPTS {
        match provider.get_transaction_receipt(hash).await {
            Ok(Some(receipt)) => return Ok(receipt),
            Ok(None) => {
                warn!("{what} receipt for {hash} is not mined yet (attempt {attempt}/{ATTEMPTS})");
            }
            Err(err) if is_transient_rpc_error(&err.to_string()) && attempt < ATTEMPTS => {
                warn!("{what} receipt poll transient RPC error (attempt {attempt}/{ATTEMPTS}): {err}");
            }
            Err(err) => return Err(err.to_string()),
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(15));
    }
    Err(format!(
        "{what} receipt for {hash} was not available after {ATTEMPTS} attempts"
    ))
}

#[cfg(test)]
mod tests {
    use super::is_transient_rpc_error;

    #[test]
    fn tenderly_send_rate_limit_is_transient() {
        let msg = "Contract error when sending raw transaction in ERC20::set_approval: \
server returned an error response: error code -32005: rate limit exceeded";
        assert!(is_transient_rpc_error(msg));
    }

    #[test]
    fn reverted_transaction_is_not_transient() {
        assert!(!is_transient_rpc_error(
            "ERC20 SetApproval Transaction reverted (status=0)"
        ));
        assert!(!is_transient_rpc_error("insufficient funds for gas * price"));
    }
}
