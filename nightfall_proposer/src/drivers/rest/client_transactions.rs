use crate::domain::error::ProposerRejection;
use crate::driven::nightfall_client_transaction::process_nightfall_client_transaction;
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
    // first we should check that the transaction is valid
    // then we should check that the transaction is not already in the database
    // then we should add the transaction to the database
    // Luckily, there is a function that does that.
    info!("Received client transaction");
    let result = process_nightfall_client_transaction::<P, E>(transaction).await;
    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error processing client transaction: {e}");
            Err(warp::reject::custom(
                ProposerRejection::ClientTransactionFailed,
            ))
        }
    }
}
