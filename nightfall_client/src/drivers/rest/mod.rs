use crate::ports::contracts::NightfallContract;
use balance::{get_balance, get_fee_balance, get_l1_balance};
use lib::{
    health_check::health_route, nf_client_proof::Proof,
    validate_certificate::certification_validation_request, validate_keys::keys_validation_request,
};
use log::error;
use proposers::get_proposers;
use reqwest::StatusCode;
use std::fmt::Debug;
use warp::{
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

use self::{
    client_nf_3::{deposit_request, transfer_request, withdraw_request},
    commitment::{
        get_all_commitments, get_commitment, get_commitments_by_token_type,
        get_max_transferable_amount_by_token_type,
    },
    keys::derive_key_mnemonic,
    request_status::{get_queue_length, get_request_status},
    synchronisation::synchronisation,
    token_info::get_token_info,
};

pub mod balance;
pub mod client_nf_3;
pub mod client_operation;
mod commitment;
mod keys;
pub mod proposers;
mod request_status;
mod synchronisation;
mod token_info;
pub mod withdraw;

pub fn routes<P, N>() -> impl Filter<Extract = (impl warp::Reply,)> + Clone
where
    P: Proof + Debug + Send + serde::Serialize + Clone + Sync,
    N: NightfallContract,
{
    health_route()
        .or(deposit_request::<P>())
        .or(transfer_request::<P>())
        .or(withdraw_request::<P>())
        .or(get_commitment())
        .or(get_all_commitments())
        .or(get_commitments_by_token_type())
        .or(get_max_transferable_amount_by_token_type())
        .or(derive_key_mnemonic())
        .or(get_proposers())
        .or(certification_validation_request())
        .or(keys_validation_request())
        .or(get_balance())
        .or(get_fee_balance())
        .or(synchronisation::<N>())
        .or(get_request_status())
        .or(get_queue_length())
        .or(get_token_info::<N>())
        .or(get_l1_balance())
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else if let Some(e) = err.find::<crate::domain::error::ClientRejection>() {
        use crate::domain::error::ClientRejection::*;
        match e {
            NoSuchToken => Ok(reply::with_status("No such token", StatusCode::NOT_FOUND)),
            InvalidTokenId => Ok(reply::with_status(
                "Invalid token id",
                StatusCode::BAD_REQUEST,
            )),
            InvalidRequestId => Ok(reply::with_status(
                "Invalid request id",
                StatusCode::BAD_REQUEST,
            )),
            QueueFull => Ok(reply::with_status(
                "Queue is full",
                StatusCode::SERVICE_UNAVAILABLE,
            )),
            DatabaseError => Ok(reply::with_status(
                "Database error or duplicate transaction",
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
            InvalidCommitmentKey => Ok(reply::with_status(
                "Invalid commitment key",
                StatusCode::BAD_REQUEST,
            )),
            CommitmentNotFound => Ok(reply::with_status(
                "Commitment not found",
                StatusCode::NOT_FOUND,
            )),
            ProposerError => Ok(reply::with_status(
                "Failed to get list of Proposers",
                StatusCode::SERVICE_UNAVAILABLE,
            )),
            RequestNotFound => Ok(reply::with_status("No such request", StatusCode::NOT_FOUND)),
            FailedDeEscrow => Ok(reply::with_status(
                "Failed to de-escrow funds",
                StatusCode::BAD_REQUEST,
            )),
            SynchronisationUnavailable => Ok(reply::with_status(
                "Synchronisation service unavailable",
                StatusCode::SERVICE_UNAVAILABLE,
            )),
            InvalidTokenType => Ok(reply::with_status(
                "Invalid Token Type",
                StatusCode::BAD_REQUEST,
            )),
        }
    } else {
        error!("unhandled rejection: {err:?}");
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::TokenData,
        driven::queue::get_queue,
        ports::contracts::NightfallContract,
    };
    use alloy::primitives::{Address, I256};
    use ark_bn254::Fr as Fr254;
    use ark_ff::BigInteger256;
    use lib::{
        error::NightfallContractError,
        plonk_prover::plonk_proof::PlonkProof,
        shared_entities::{DepositSecret, TokenType, WithdrawData},
    };
    use nightfall_bindings::artifacts::Nightfall;
    use serde_json::Value;

    struct MockNightfall;
    struct MockNightfallSyncError;

    impl NightfallContract for MockNightfall {
        async fn escrow_funds(
            _token_erc_address: Fr254,
            _value: Fr254,
            _token_id: BigInteger256,
            _fee: Fr254,
            _deposit_fee: Fr254,
            _secret_preimage: DepositSecret,
            _token_type: TokenType,
        ) -> Result<[Fr254; 2], NightfallContractError> {
            panic!("escrow_funds should not be called in these route tests")
        }

        fn get_address() -> Fr254 {
            Fr254::from(1u64)
        }

        async fn de_escrow_funds(
            _withdraw_data: WithdrawData,
            _token_type: TokenType,
        ) -> Result<(), NightfallContractError> {
            panic!("de_escrow_funds should not be called in these route tests")
        }

        async fn withdraw_available(
            _withdraw_data: WithdrawData,
        ) -> Result<bool, NightfallContractError> {
            panic!("withdraw_available should not be called in these route tests")
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            panic!("get_current_layer2_blocknumber should not be called in these route tests")
        }

        async fn get_token_info(nf_token_id: Fr254) -> Result<TokenData, NightfallContractError> {
            if nf_token_id == Fr254::from(1u64) {
                Ok(TokenData {
                    erc_address: Fr254::from(2u64),
                    token_id: BigInteger256::from(3u64),
                    token_type: TokenType::ERC20,
                })
            } else {
                Err(NightfallContractError::ProviderError(
                    "token not found".to_string(),
                ))
            }
        }

        async fn get_layer2_block_by_number(
            _block_number: I256,
        ) -> Result<(Address, Nightfall::Block), NightfallContractError> {
            panic!("get_layer2_block_by_number should not be called in these route tests")
        }
    }

    impl NightfallContract for MockNightfallSyncError {
        async fn escrow_funds(
            _token_erc_address: Fr254,
            _value: Fr254,
            _token_id: BigInteger256,
            _fee: Fr254,
            _deposit_fee: Fr254,
            _secret_preimage: DepositSecret,
            _token_type: TokenType,
        ) -> Result<[Fr254; 2], NightfallContractError> {
            panic!("escrow_funds should not be called in these route tests")
        }

        fn get_address() -> Fr254 {
            Fr254::from(1u64)
        }

        async fn de_escrow_funds(
            _withdraw_data: WithdrawData,
            _token_type: TokenType,
        ) -> Result<(), NightfallContractError> {
            panic!("de_escrow_funds should not be called in these route tests")
        }

        async fn withdraw_available(
            _withdraw_data: WithdrawData,
        ) -> Result<bool, NightfallContractError> {
            panic!("withdraw_available should not be called in these route tests")
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Err(NightfallContractError::ProviderError(
                "sync unavailable".to_string(),
            ))
        }

        async fn get_token_info(_nf_token_id: Fr254) -> Result<TokenData, NightfallContractError> {
            panic!("get_token_info should not be called in these route tests")
        }

        async fn get_layer2_block_by_number(
            _block_number: I256,
        ) -> Result<(Address, Nightfall::Block), NightfallContractError> {
            panic!("get_layer2_block_by_number should not be called in these route tests")
        }
    }

    #[tokio::test]
    async fn test_request_status_invalid_uuid_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/request/not-a-uuid")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid request id");
    }

    #[tokio::test]
    async fn test_queue_length_returns_zero_for_empty_queue() {
        get_queue().await.write().await.clear();

        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/queue")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        let body = serde_json::from_slice::<usize>(res.body()).expect("body should be JSON");
        assert_eq!(body, 0);
    }

    #[tokio::test]
    async fn test_commitment_lookup_invalid_key_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/commitment/not-hex")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Invalid commitment key"
        );
    }

    #[tokio::test]
    async fn test_balance_invalid_token_id_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/balance/not-an-address/not-a-token")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid token id");
    }

    #[tokio::test]
    async fn test_token_info_invalid_query_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/token/not-hex")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid token id");
    }

    #[tokio::test]
    async fn test_token_info_unknown_token_returns_not_found() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/token/02")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "No such token");
    }

    #[tokio::test]
    async fn test_token_info_known_token_returns_ok() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/token/01")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        let body = serde_json::from_slice::<Value>(res.body()).expect("body should be JSON");
        assert_eq!(body["token_type"], "ERC20");
        assert_eq!(
            body["erc_address"],
            "0000000000000000000000000000000000000000000000000000000000000002"
        );
        assert_eq!(
            body["token_id"],
            "0000000000000000000000000000000000000000000000000000000000000003"
        );
    }

    #[tokio::test]
    async fn test_commitments_by_token_type_invalid_token_type_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/commitments/token_type/not-a-type")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid Token Type");
    }

    #[tokio::test]
    async fn test_max_transferable_invalid_token_id_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/commitments/max_transferable_amount/ERC20/not-hex")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid token id");
    }

    #[tokio::test]
    async fn test_max_transferable_invalid_token_type_returns_bad_request() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/commitments/max_transferable_amount/not-a-type/01")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Invalid Token Type");
    }

    #[tokio::test]
    async fn test_health_route_is_wired_in_client_router() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/health")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Healthy");
    }

    #[tokio::test]
    async fn test_certification_route_is_wired_in_client_router() {
        let boundary = "x-boundary";
        let body = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"certificate\"; filename=\"cert.der\"\r\nContent-Type: application/octet-stream\r\n\r\nabc\r\n--{boundary}--\r\n"
        );
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/certification")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(body)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).unwrap();
        assert_eq!(body["message"], "Missing 'priv_key' field or empty file");
    }

    #[tokio::test]
    async fn test_keys_validation_route_is_wired_in_client_router() {
        let filter = routes::<PlonkProof, MockNightfall>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/keys_validation")
            .header("content-type", "application/json")
            .body(r#"{"concurrency":2}"#)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "BAD_REQUEST");
    }

    #[tokio::test]
    async fn test_synchronisation_route_is_wired_in_client_router() {
        let filter = routes::<PlonkProof, MockNightfallSyncError>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/synchronisation")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Synchronisation service unavailable"
        );
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_request_not_found_to_not_found() {
        let response = handle_rejection(warp::reject::custom(
            crate::domain::error::ClientRejection::RequestNotFound,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_proposer_error_to_service_unavailable() {
        let response = handle_rejection(warp::reject::custom(
            crate::domain::error::ClientRejection::ProposerError,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
