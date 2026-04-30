use crate::domain::error::ProposerRejection;
use crate::driven::nightfall_client_transaction::process_nightfall_client_transaction;
use futures::Future;
use lib::{
    nf_client_proof::{Proof, ProvingEngine},
    shared_entities::ClientTransaction,
};
use log::{error, info};

use warp::{hyper::StatusCode, path, Filter};

pub fn client_transaction<P, E>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    P: Proof,
    E: ProvingEngine<P>,
{
    path!("v1" / "transaction")
        .and(warp::body::json())
        .and_then(|transaction| handle_client_transaction::<P, E>(transaction))
}

async fn handle_client_transaction<P, E>(
    transaction: ClientTransaction<P>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    handle_client_transaction_with(transaction, |transaction| async move {
        // first we should check that the transaction is valid
        // then we should check that the transaction is not already in the database
        // then we should add the transaction to the database
        // Luckily, there is a function that does that.
        let result = process_nightfall_client_transaction::<P, E>(transaction).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error processing client transaction: {e}");
                Err(warp::reject::custom(
                    ProposerRejection::ClientTransactionFailed,
                ))
            }
        }
    })
    .await
}

async fn handle_client_transaction_with<P, F, Fut>(
    transaction: ClientTransaction<P>,
    process: F,
) -> Result<impl warp::Reply, warp::Rejection>
where
    P: Proof,
    F: FnOnce(ClientTransaction<P>) -> Fut,
    Fut: Future<Output = Result<(), warp::Rejection>>,
{
    info!("Received client transaction");
    process(transaction).await?;
    Ok(StatusCode::CREATED)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::rest::handle_rejection;
    use alloy::primitives::Bytes;
    use ark_bn254::Fr as Fr254;
    use ark_serialize::SerializationError;
    use lib::{
        nf_client_proof::ProvingEngine,
        shared_entities::{ClientTransaction, CompressedSecrets},
    };
    use serde::{Deserialize, Serialize};
    use warp::{http::StatusCode, Filter};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
    struct MockProof {
        a: Vec<u8>,
        b: Vec<u8>,
        c: Vec<u8>,
    }

    #[derive(Debug)]
    struct MockProvingEngine;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::from_static(b"mock-proof"))
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self {
                a: vec![1],
                b: vec![2],
                c: vec![3],
            })
        }
    }

    impl ProvingEngine<MockProof> for MockProvingEngine {
        type Error = std::fmt::Error;

        fn prove(
            _private_inputs: &mut lib::nf_client_proof::PrivateInputs,
            _public_inputs: &mut lib::nf_client_proof::PublicInputs,
        ) -> Result<MockProof, Self::Error> {
            Ok(Self::default_proof())
        }

        fn verify(
            _proof: &MockProof,
            _public_inputs: &lib::nf_client_proof::PublicInputs,
        ) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    impl MockProvingEngine {
        fn default_proof() -> MockProof {
            MockProof {
                a: vec![1],
                b: vec![2],
                c: vec![3],
            }
        }
    }

    fn sample_transaction() -> ClientTransaction<MockProof> {
        ClientTransaction {
            fee: Fr254::from(2u64),
            historic_commitment_root: Fr254::from(0u64),
            commitments: [
                Fr254::from(10u64),
                Fr254::from(0u64),
                Fr254::from(0u64),
                Fr254::from(0u64),
            ],
            nullifiers: [
                Fr254::from(1u64),
                Fr254::from(0u64),
                Fr254::from(0u64),
                Fr254::from(0u64),
            ],
            compressed_secrets: CompressedSecrets::default(),
            swap_link: Fr254::from(0u64),
            deadline: Fr254::from(0u64),
            swap_side: Fr254::from(0u64),
            proof: MockProvingEngine::default_proof(),
        }
    }

    #[tokio::test]
    async fn test_client_transaction_route_rejects_malformed_json() {
        let filter = client_transaction::<MockProof, MockProvingEngine>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/transaction")
            .header("content-type", "application/json")
            .body("{")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_client_transaction_route_rejects_invalid_payload_shape() {
        let filter = client_transaction::<MockProof, MockProvingEngine>();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/transaction")
            .header("content-type", "application/json")
            .body("{}")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_client_transaction_route_maps_processing_failure_to_bad_request() {
        let filter = path!("v1" / "transaction")
            .and(warp::body::json())
            .and_then(|transaction: ClientTransaction<MockProof>| async move {
                handle_client_transaction_with(transaction, |_tx| async {
                    Err(warp::reject::custom(
                        ProposerRejection::ClientTransactionFailed,
                    ))
                })
                .await
            })
            .recover(handle_rejection);

        let res = warp::test::request()
            .method("POST")
            .path("/v1/transaction")
            .json(&sample_transaction())
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            std::str::from_utf8(res.body()).unwrap(),
            "Client transaction failed"
        );
    }
}
