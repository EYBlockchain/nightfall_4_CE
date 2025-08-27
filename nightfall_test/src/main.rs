use configuration::{logging::init_logging, settings::Settings};
use log::{error, info};
use nightfall_test::{
    run_tests::run_tests,
    webhook::{poll_queue, run_webhook_server},
};
use std::sync::Arc;
use tokio::task::{JoinError, JoinSet};

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    const MINING_INTERVAL: u32 = 5; // seconds
    let settings: Settings = Settings::new().unwrap();
    init_logging(
        settings.nightfall_test.log_level.as_str(),
        settings.log_app_only,
    );
    // set up a vec to hold responses from the webhook server
    let responses = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut tasks = JoinSet::new();
    //start all the tasks
    info!("Starting webhook server...");
    tasks.spawn(run_webhook_server(responses.clone()));
    info!("Running tests...");
    tasks.spawn(run_tests(responses.clone(), MINING_INTERVAL));
    info!("Starting queue polling...");
    tasks.spawn(poll_queue());

    let result = tasks.join_next().await; // wait for any task to finish
    match result {
        Some(Ok(_)) => { 
            info!("Nightfall tests completed successfully.");
            return Ok(());
        },
        Some(Err(e)) => {
            error!("Nightfall tests failed with error: {e:?}",);
            return Err(e);
        }
        None => {
            error!("No tasks were completed.");
            panic!("No tasks were completed, this is unexpected.");
        }
    }
}
