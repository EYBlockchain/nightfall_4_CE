use std::sync::Arc;
use configuration::{logging::init_logging, settings::Settings};
use log::info;
use nightfall_test::{
    run_tests::run_tests,
    webhook::{poll_queue, run_webhook_server},
};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
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
    tasks.spawn(run_tests(responses.clone()));
    info!("Starting queue polling...");
    tasks.spawn(poll_queue());

    tasks.join_next().await; // wait for any task to finish
    info!("Nightfall test client exited.");
}
