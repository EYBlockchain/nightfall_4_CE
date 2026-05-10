use configuration::{logging::init_logging, settings::get_settings};
use lib::plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine};
use log::{error, info};
use nightfall_bindings::artifacts::Nightfall;
use nightfall_proposer::drivers::blockchain::event_listener_manager::ensure_running;
use nightfall_proposer::{
    driven::{db::mongo_db::DB, mock_prover::MockProver, rollup_prover::RollupProver},
    drivers::{blockchain::block_assembly::start_block_assembly, rest::routes},
    initialisation::bootstrap_proposer_startup_state,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // at some point we have to be specific about the proof we're using
    let settings = get_settings();
    type P = PlonkProof;
    type E = PlonkProvingEngine;
    type N = Nightfall::NightfallCalls;

    init_logging(
        settings.nightfall_proposer.log_level.as_str(),
        settings.log_app_only,
    );

    info!("Bootstrapping proposer startup state from {DB}");
    bootstrap_proposer_startup_state::<N>()
        .await
        .map_err(|e| format!("Proposer startup bootstrap failed: {e}"))?;

    // start the event listener
    ensure_running::<P, E, N>().await;

    let task_0 = if settings.mock_prover {
        info!("Using MockProver");
        tokio::spawn(start_block_assembly::<P, MockProver, N>())
    } else {
        info!("Using RollupProver");
        tokio::spawn(start_block_assembly::<P, RollupProver, N>())
    };

    let routes = routes::<P, E>();
    let task_2 = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], 3000)));
    info!("Starting warp server, block assembler and event_handler threads");
    // we'll run the warp server and blockchain listener in parallel in separate threads
    // this maybe overkill so look at combining them into a single thread - depending on speed.
    let (_r0, _r2) = (task_0.await??, task_2.await?);
    error!("Proposer exited unexpectedly. See information above.");
    Ok(())
}
