use configuration::{logging::init_logging, settings::get_settings};

use lib::{merkle_trees::trees::TreeMetadata, utils, wallets::LocalWsClient};
use log::{error, info};

use ark_bn254::Fr as Fr254;
use nightfall_bindings::nightfall::Nightfall;
use nightfall_client::{
    domain::entities::Node,
    driven::plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
    drivers::{blockchain::nightfall_event_listener::start_event_listener, rest::routes},
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    // declare the types of wallet that we're using
    type N = Nightfall<LocalWsClient>;
    init_logging(
        get_settings().nightfall_client.log_level.as_str(),
        get_settings().log_app_only,
    );
    // drop the commitment merkle tree data because it will be out of date and need resynching. The commitments are retained.
    // status reflected in the DB
    let url = &get_settings().nightfall_client.db_url;
    utils::drop_collection::<TreeMetadata<Fr254>>(
        url.as_str(),
        "nightfall",
        "commitment_tree_metadata",
    )
    .await
    .expect("Failed to drop Metadata collection");
    utils::drop_collection::<Node<Fr254>>(url.as_str(), "nightfall", "commitment_tree_nodes")
        .await
        .expect("Failed to drop Node collection");
    utils::drop_collection::<Node<Fr254>>(url.as_str(), "nightfall", "commitment_tree_cache")
        .await
        .expect("Failed to drop Cache collection");
    // start the event listener
    let task_1 = tokio::spawn(start_event_listener::<N>(0, get_settings().genesis_block));
    // set up the warp server
    let routes = routes::<PlonkProof, PlonkProvingEngine, Nightfall<LocalWsClient>>();
    let task_2 = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], 3000)));
    info!("Starting warp server and event_handler threads");
    // we'll run the warp server and blockchain listener in parallel in separate threads
    let (_r1, _r2) = (task_2.await?, task_1.await?);
    error!("Client exited unexpectedly.");
    Ok(())
}
