use configuration::{logging::init_logging, settings::get_settings};
use log::{error, info};
use nightfall_bindings::artifacts::Nightfall;
use nightfall_client::driven::{
    plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
};
use nightfall_proposer::drivers::blockchain::event_listener_manager::ensure_running;
use nightfall_proposer::{
    driven::{db::mongo_db::DB, mock_prover::MockProver, rollup_prover::RollupProver},
    drivers::{blockchain::block_assembly::start_block_assembly, rest::routes},
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

    // drop any existing database
    let db_url = &settings.nightfall_proposer.db_url;
    info!("Dropping database: {DB}");
    let _ = lib::utils::drop_database(db_url, DB).await;

    let task_0 = if settings.mock_prover {
        info!("Using MockProver");
        tokio::spawn(start_block_assembly::<P, MockProver, N>())
    } else {
        info!("Using RollupProver");
        tokio::spawn(start_block_assembly::<P, RollupProver, N>())
    };

    // start the event listener
    // ── start the (owned) event listener once ─────────────────────────────────
    ensure_running::<P, E, N>().await;

    let routes = routes::<P, E>();
    let task_2 = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], 3000)));
    info!("Starting warp server, block assembler and event_handler threads");
    // we'll run the warp server and blockchain listener in parallel in separate threads
    // this maybe overkill so look at combining them into a single thread - depending on speed.
    let (_r0, _r2) = (task_0.await??, task_2.await?);
    error!("Proposer exited unexpectedly. See information above.");
    Ok(())
}
