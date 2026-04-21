use crate::{domain::error::ProposerRejection, initialisation::get_blockchain_client_connection};
use alloy::primitives::U256;
use configuration::{addresses::get_addresses, settings::get_settings};
use lib::{
    blockchain_client::BlockchainClientConnection, error::ProposerError,
    verify_contract::VerifiedContracts,
};
use log::{info, warn};
use std::future::Future;
use url::Url;
/// APIs for managing proposers
use warp::{hyper::StatusCode, path, reply, reply::Reply, Filter};

/// Get request for proposer rotation
pub fn rotate_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "rotate")
        .and(warp::get())
        .and_then(handle_rotate_proposer)
}

async fn handle_rotate_proposer() -> Result<impl Reply, warp::Rejection> {
    handle_rotate_proposer_with(|| async {
        // get a ManageProposers instance
        let blockchain_client = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client();
        let verified =
            VerifiedContracts::resolve_and_verify_contract(blockchain_client.root(), get_addresses())
                .await
                .map_err(|e| {
                    warn!("Contract verification failed: {e}");
                    warp::reject::custom(ProposerRejection::FailedToRotateProposer)
                })?;
        let proposer_manager = verified.round_robin;
        match proposer_manager.proposer_count().call().await {
            Ok(count) => {
                if count <= U256::ONE {
                    warn!("Rotation requested, but only one active proposer; rotation will have no effect.");
                }
            }
            Err(_e) => {
                warn!("Failed to fetch proposer count before rotation");
            }
        }
        // rotate the proposer
        let signer = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_signer();

        let nonce = blockchain_client
            .get_transaction_count(signer.address())
            .await
            .map_err(|e| {
                warn!("Failed to generate nonce during proposer rotation: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;
        let gas_price = blockchain_client
            .get_gas_price()
            .await
            .map_err(|e| {
                warn!("Failed to generate gas_price during proposer rotation: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;
        let gas_limit = 5000000u64;

        let call = proposer_manager
            .rotate_proposer()
            .nonce(nonce)
            .gas(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
            .build_raw_transaction((*signer).clone())
            .await
            .map_err(|e| {
                warn!("Failed to build rotate_proposer transaction: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;

        let tx_receipt = blockchain_client
            .send_raw_transaction(&call)
            .await
            .map_err(|e| {
                warn!("Error sending raw transaction in rotate_proposer: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?
            .get_receipt()
            .await
            .map_err(|e| {
                warn!("Failed to get receipt of rotation proposer transaction: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;
        if tx_receipt.status() {
            info!("Rotated proposer successfully");
            Ok(())
        } else {
            warn!("Failed to rotate proposer");
            Err(warp::reject::custom(
                ProposerRejection::FailedToRotateProposer,
            ))
        }
    })
    .await
}

async fn handle_rotate_proposer_with<F, Fut>(rotate: F) -> Result<impl Reply, warp::Rejection>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), warp::Rejection>>,
{
    rotate().await?;
    Ok(StatusCode::OK)
}

// Add a proposer
pub fn add_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "register")
        .and(warp::body::json())
        .and_then(handle_add_proposer)
}

async fn handle_add_proposer(url: String) -> Result<impl Reply, warp::Rejection> {
    if let Err(message) = validate_proposer_url(&url) {
        return Ok(reply::with_status(message, StatusCode::BAD_REQUEST));
    }
    // get a ManageProposers instance
    let read_connection = get_blockchain_client_connection().await.read().await;
    let blockchain_client = read_connection.get_client();
    let caller = read_connection.get_address();
    let signer = read_connection.get_signer();
    let client = blockchain_client.root();
    let verified = VerifiedContracts::resolve_and_verify_contract(client, get_addresses())
        .await
        .map_err(|e| {
            warn!("Contract verification failed: {e}");
            warp::reject::custom(ProposerRejection::FailedToRotateProposer)
        })?;
    let proposer_manager = verified.round_robin;

    let nonce = blockchain_client
        .get_transaction_count(caller)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?;
    let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
        warn!("{e}");
        ProposerRejection::FailedToAddProposer
    })?;
    let max_fee_per_gas = gas_price * 2;
    let max_priority_fee_per_gas = gas_price;
    let gas_limit = 5000000u64;

    let raw_tx = proposer_manager
        .add_proposer(url)
        .value(U256::from(get_settings().nightfall_deployer.proposer_stake))
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        .build_raw_transaction((*signer).clone())
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?;
    // add the proposer
    let tx = blockchain_client
        .send_raw_transaction(&raw_tx)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToAddProposer
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            warn!("Failed to get transaction receipt: {e}");
            ProposerError::ProviderError(e.to_string())
        })?;
    if tx.status() {
        info!("Registered proposer with address: {:?}", tx.from);
        Ok(reply::with_status("OK", StatusCode::OK))
    } else {
        warn!("Failed to add proposer with address: {:?}", tx.from);
        Err(warp::reject::custom(ProposerRejection::FailedToAddProposer))
    }
}

// remove a proposer
pub fn remove_proposer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    path!("v1" / "deregister")
        .and(warp::get())
        .and_then(handle_remove_proposer)
}

async fn handle_remove_proposer() -> Result<impl Reply, warp::Rejection> {
    handle_remove_proposer_with(|| async {
        // get a ManageProposers instance
        let read_connection = get_blockchain_client_connection().await.read().await;
        let blockchain_client = read_connection.get_client();
        let signer_address = read_connection.get_address();
        let signer = read_connection.get_signer();
        let client = blockchain_client.root();
        let verified = VerifiedContracts::resolve_and_verify_contract(client, get_addresses())
            .await
            .map_err(|e| {
                warn!("Contract verification failed: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;
        let proposer_manager = verified.round_robin;

        // Read penalty + cooling config from settings
        let settings = get_settings();
        let penalty = settings.nightfall_deployer.proposer_exit_penalty;
        let cooling_blocks = settings.nightfall_deployer.proposer_cooling_blocks;

        // Fetch the current proposer address on-chain
        match proposer_manager.get_current_proposer_address().call().await {
            Ok(current_proposer) => {
                if current_proposer == signer_address {
                    warn!(
                        "You are removing yourself as the active proposer — this will deduct an exit penalty of {penalty} units and start a cooldown period of {cooling_blocks} L1 blocks before you can re-register."
                    );
                } else {
                    info!("You are removing yourself, but you are not the active proposer — no penalty will be applied.");
                }
            }
            Err(e) => {
                warn!("Could not check current proposer before removal: {e:?}");
            }
        }

        let nonce = blockchain_client
            .get_transaction_count(signer_address)
            .await
            .map_err(|e| {
                warn!("{e}");
                ProposerRejection::FailedToRemoveProposer
            })?;
        let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToRemoveProposer
        })?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;
        let gas_limit = 5000000u64;

        let raw_tx = proposer_manager
            .remove_proposer()
            .nonce(nonce)
            .gas(gas_limit)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
            .build_raw_transaction((*signer).clone())
            .await
            .map_err(|e| {
                warn!("{e}");
                ProposerRejection::FailedToRemoveProposer
            })?;
        // add the proposer
        let tx = blockchain_client
            .send_raw_transaction(&raw_tx)
            .await
            .map_err(|_e| {
                warn!("Failed to remove proposer");
                ProposerRejection::FailedToRemoveProposer
            })?
            .get_receipt()
            .await
            .map_err(|e| {
                warn!("Failed to get transaction receipt: {e}");
                ProposerError::ProviderError(e.to_string())
            })?;
        if tx.status() {
            info!("Removed proposer with address: {:?}", tx.from);
            Ok(())
        } else {
            warn!("Failed to remove proposer");
            Err(warp::reject::custom(
                ProposerRejection::FailedToRemoveProposer,
            ))
        }
    })
    .await
}

async fn handle_remove_proposer_with<F, Fut>(remove: F) -> Result<impl Reply, warp::Rejection>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), warp::Rejection>>,
{
    remove().await?;
    Ok(StatusCode::OK)
}

// Withdraw a proposer's stake after a successful deregistration
pub fn withdraw() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("v1" / "withdraw")
        .and(warp::body::json())
        .and_then(handle_withdraw)
}

async fn handle_withdraw(amount: u64) -> Result<impl Reply, warp::Rejection> {
    if amount == 0 {
        return Ok(reply::with_status(
            "Withdraw amount must be greater than zero",
            StatusCode::BAD_REQUEST,
        ));
    }
    // get a ManageProposers instance
    let read_connection = get_blockchain_client_connection().await.read().await;
    let blockchain_client = read_connection.get_client();
    let caller = read_connection.get_address();
    let signer = read_connection.get_signer();
    let verified =
        VerifiedContracts::resolve_and_verify_contract(blockchain_client.root(), get_addresses())
            .await
            .map_err(|e| {
                warn!("Contract verification failed: {e}");
                warp::reject::custom(ProposerRejection::FailedToRotateProposer)
            })?;
    let proposer_manager = verified.round_robin;
    // attemp to withdraw the stake
    let nonce = blockchain_client
        .get_transaction_count(caller)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?;
    let gas_price = blockchain_client.get_gas_price().await.map_err(|e| {
        warn!("{e}");
        ProposerRejection::FailedToWithdrawStake
    })?;
    let max_fee_per_gas = gas_price * 2;
    let max_priority_fee_per_gas = gas_price;
    let gas_limit = 5000000u64;

    let raw_tx = proposer_manager
        .withdraw(U256::from(amount))
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .chain_id(get_settings().network.chain_id) // Linea testnet chain ID
        .build_raw_transaction((*signer).clone())
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?;
    // add the proposer
    let tx = blockchain_client
        .send_raw_transaction(&raw_tx)
        .await
        .map_err(|e| {
            warn!("{e}");
            ProposerRejection::FailedToWithdrawStake
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            warn!("Failed to get transaction receipt: {e}");
            ProposerError::ProviderError(e.to_string())
        })?;
    if tx.status() {
        info!("Withdrew {} to address: {:?}", amount, tx.from);
        Ok(reply::with_status("OK", StatusCode::OK))
    } else {
        warn!("Failed to withdraw funds");
        Err(warp::reject::custom(
            ProposerRejection::FailedToWithdrawStake,
        ))
    }
}

fn validate_proposer_url(url: &str) -> Result<(), &'static str> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err("Proposer URL must not be empty");
    }

    let parsed = Url::parse(trimmed).map_err(|_| "Proposer URL must be a valid absolute URL")?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("Proposer URL must use http or https"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_proposer_url_rejects_empty_string() {
        let err = validate_proposer_url("   ").expect_err("empty URL should be rejected");
        assert_eq!(err, "Proposer URL must not be empty");
    }

    #[test]
    fn test_validate_proposer_url_rejects_invalid_url() {
        let err = validate_proposer_url("not-a-url").expect_err("invalid URL should be rejected");
        assert_eq!(err, "Proposer URL must be a valid absolute URL");
    }

    #[test]
    fn test_validate_proposer_url_rejects_unsupported_scheme() {
        let err =
            validate_proposer_url("ftp://example.com").expect_err("FTP URL should be rejected");
        assert_eq!(err, "Proposer URL must use http or https");
    }

    #[test]
    fn test_validate_proposer_url_accepts_http_url() {
        validate_proposer_url("http://example.com").expect("HTTP URL should be accepted");
    }

    #[tokio::test]
    async fn test_add_proposer_route_rejects_empty_url() {
        let filter = add_proposer();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/register")
            .header("content-type", "application/json")
            .body(r#""""#)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Proposer URL must not be empty"
        );
    }

    #[tokio::test]
    async fn test_add_proposer_route_rejects_invalid_url() {
        let filter = add_proposer();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/register")
            .header("content-type", "application/json")
            .body(r#""not-a-url""#)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Proposer URL must be a valid absolute URL"
        );
    }

    #[tokio::test]
    async fn test_add_proposer_route_rejects_malformed_json() {
        let filter = add_proposer();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/register")
            .header("content-type", "application/json")
            .body("{")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_add_proposer_route_rejects_non_string_json() {
        let filter = add_proposer();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/register")
            .header("content-type", "application/json")
            .body("{}")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_zero_amount() {
        let filter = withdraw();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .header("content-type", "application/json")
            .body("0")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Withdraw amount must be greater than zero"
        );
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_malformed_json() {
        let filter = withdraw();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .header("content-type", "application/json")
            .body("{")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_negative_amount() {
        let filter = withdraw();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .header("content-type", "application/json")
            .body("-1")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_non_numeric_json() {
        let filter = withdraw();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .header("content-type", "application/json")
            .body(r#""10""#)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_withdraw_route_rejects_u64_overflow() {
        let filter = withdraw();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/withdraw")
            .header("content-type", "application/json")
            .body("18446744073709551616")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_rotate_route_returns_ok_on_success() {
        let filter = path!("v1" / "rotate")
            .and(warp::get())
            .and_then(|| async { handle_rotate_proposer_with(|| async { Ok(()) }).await });

        let res = warp::test::request()
            .method("GET")
            .path("/v1/rotate")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_rotate_route_maps_failure_to_locked() {
        let filter = path!("v1" / "rotate")
            .and(warp::get())
            .and_then(|| async {
                handle_rotate_proposer_with(|| async {
                    Err(warp::reject::custom(
                        ProposerRejection::FailedToRotateProposer,
                    ))
                })
                .await
            })
            .recover(super::super::handle_rejection);

        let res = warp::test::request()
            .method("GET")
            .path("/v1/rotate")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::LOCKED);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Failed to rotate proposer"
        );
    }

    #[tokio::test]
    async fn test_deregister_route_returns_ok_on_success() {
        let filter = path!("v1" / "deregister")
            .and(warp::get())
            .and_then(|| async { handle_remove_proposer_with(|| async { Ok(()) }).await });

        let res = warp::test::request()
            .method("GET")
            .path("/v1/deregister")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_deregister_route_maps_failure_to_bad_request() {
        let filter = path!("v1" / "deregister")
            .and(warp::get())
            .and_then(|| async {
                handle_remove_proposer_with(|| async {
                    Err(warp::reject::custom(
                        ProposerRejection::FailedToRemoveProposer,
                    ))
                })
                .await
            })
            .recover(super::super::handle_rejection);

        let res = warp::test::request()
            .method("GET")
            .path("/v1/deregister")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Failed to remove proposer"
        );
    }
}
