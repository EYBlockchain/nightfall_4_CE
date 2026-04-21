use crate::domain::error::ProposerRejection;
use crate::drivers::rest::{
    block_data::get_block_data, client_transactions::client_transaction,
    proposers::rotate_proposer, synchronisation::synchronisation,
};
use block_assembly::{get_block_assembly_status_route, pause_block_assembly, resume_block_assembly};
use lib::{
    health_check::health_route,
    nf_client_proof::{Proof, ProvingEngine},
    validate_certificate::certification_validation_request,
    validate_keys::keys_validation_request,
};
use proposers::{add_proposer, remove_proposer, withdraw};
use warp::{
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

pub mod block_assembly;
pub mod block_data;
pub mod client_transactions;
pub mod proposers;
pub mod synchronisation;

pub fn routes<P, E>() -> impl Filter<Extract = (impl warp::Reply,)> + Clone
where
    P: Proof,
    E: ProvingEngine<P> + Sync + Send + 'static,
{
    health_route()
        .or(client_transaction::<P, E>())
        .or(rotate_proposer())
        .or(get_block_data())
        .or(add_proposer())
        .or(remove_proposer())
        .or(withdraw())
        .or(certification_validation_request())
        .or(keys_validation_request())
        .or(synchronisation())
        .or(pause_block_assembly())
        .or(resume_block_assembly())
        .or(get_block_assembly_status_route())
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        Ok(reply::with_status(
            "BAD_REQUEST",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(e) = err.find::<ProposerRejection>() {
        match e {
            ProposerRejection::BlockDataUnavailable => Ok(reply::with_status(
                "Block data unavailable",
                warp::http::StatusCode::SERVICE_UNAVAILABLE,
            )),
            ProposerRejection::ClientTransactionFailed => Ok(reply::with_status(
                "Client transaction failed",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToRotateProposer => Ok(reply::with_status(
                "Failed to rotate proposer",
                warp::http::StatusCode::LOCKED,
            )),
            ProposerRejection::FailedToAddProposer => Ok(reply::with_status(
                "Failed to add proposer",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToRemoveProposer => Ok(reply::with_status(
                "Failed to remove proposer",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::FailedToWithdrawStake => Ok(reply::with_status(
                "Failed to withdraw stake",
                warp::http::StatusCode::BAD_REQUEST,
            )),
            ProposerRejection::ProviderError => Ok(reply::with_status(
                "Provider error",
                warp::http::StatusCode::SERVICE_UNAVAILABLE,
            )),
        }
    } else {
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use ark_serialize::SerializationError;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
    struct MockProof {
        a: Vec<u8>,
    }

    #[derive(Debug)]
    struct MockProvingEngine;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::from_static(b"mock-proof"))
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self { a: vec![1] })
        }
    }

    impl ProvingEngine<MockProof> for MockProvingEngine {
        type Error = std::fmt::Error;

        fn prove(
            _private_inputs: &mut lib::nf_client_proof::PrivateInputs,
            _public_inputs: &mut lib::nf_client_proof::PublicInputs,
        ) -> Result<MockProof, Self::Error> {
            Ok(MockProof { a: vec![1] })
        }

        fn verify(
            _proof: &MockProof,
            _public_inputs: &lib::nf_client_proof::PublicInputs,
        ) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_health_route_is_wired_in_proposer_router() {
        let filter = routes::<MockProof, MockProvingEngine>();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/health")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), warp::http::StatusCode::OK);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "Healthy");
    }

    #[tokio::test]
    async fn test_certification_route_is_wired_in_proposer_router() {
        let boundary = "x-boundary";
        let body = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"certificate\"; filename=\"cert.der\"\r\nContent-Type: application/octet-stream\r\n\r\nabc\r\n--{boundary}--\r\n"
        );
        let filter = routes::<MockProof, MockProvingEngine>();
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

        assert_eq!(res.status(), warp::http::StatusCode::BAD_REQUEST);
        let body = serde_json::from_slice::<Value>(res.body()).unwrap();
        assert_eq!(body["message"], "Missing 'priv_key' field or empty file");
    }

    #[tokio::test]
    async fn test_keys_validation_route_is_wired_in_proposer_router() {
        let filter = routes::<MockProof, MockProvingEngine>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/keys_validation")
            .header("content-type", "application/json")
            .body(r#"{"concurrency":2}"#)
            .reply(&filter)
            .await;

        assert_eq!(res.status(), warp::http::StatusCode::BAD_REQUEST);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "BAD_REQUEST");
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_rotate_failure_to_locked() {
        let response = handle_rejection(warp::reject::custom(
            ProposerRejection::FailedToRotateProposer,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), warp::http::StatusCode::LOCKED);
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_add_failure_to_bad_request() {
        let response = handle_rejection(warp::reject::custom(
            ProposerRejection::FailedToAddProposer,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), warp::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_remove_failure_to_bad_request() {
        let response = handle_rejection(warp::reject::custom(
            ProposerRejection::FailedToRemoveProposer,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), warp::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_handle_rejection_maps_withdraw_failure_to_bad_request() {
        let response = handle_rejection(warp::reject::custom(
            ProposerRejection::FailedToWithdrawStake,
        ))
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), warp::http::StatusCode::BAD_REQUEST);
    }
}
