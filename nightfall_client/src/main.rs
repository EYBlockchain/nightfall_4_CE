use ark_bn254::Fr as Fr254;
use configuration::{logging::init_logging, settings::get_settings};
use lib::{merkle_trees::trees::TreeMetadata, utils, plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine}};
use log::{error, info};
use nightfall_bindings::artifacts::Nightfall;
use nightfall_client::{
    domain::entities::{Node, Request},
    driven::{
        queue::process_queue,
    },
    drivers::{blockchain::event_listener_manager::ensure_running, rest::routes},
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    // declare the types of wallet that we're using
    type N = Nightfall::NightfallCalls;
    init_logging(
        get_settings().nightfall_client.log_level.as_str(),
        get_settings().log_app_only,
    );
    // ── clear desynchronised tree metadata/requests ───────────────────────────
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
    // drop the request-ID tracking collection
    utils::drop_collection::<Request>(url.as_str(), "nightfall", "requests")
        .await
        .expect("Failed to drop Requests collection");

    // ── start the (owned) event listener once ─────────────────────────────────
    ensure_running::<N>().await;

    // ── start Warp server and the queue worker as independent tasks ───────────
    let routes = routes::<PlonkProof, Nightfall::NightfallCalls>();
    let task_warp = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], 3000)));

    let task_queue = tokio::spawn(process_queue::<
        PlonkProof,
        PlonkProvingEngine,
        Nightfall::NightfallCalls,
    >());

    info!("Starting warp server and request queue (event listener managed separately)");
    // Both tasks are long-lived; if either returns, treat as unexpected
    let (_r2, _r3) = (task_warp.await?, task_queue.await?);
    error!("Client exited unexpectedly.");

    Ok(())
}
