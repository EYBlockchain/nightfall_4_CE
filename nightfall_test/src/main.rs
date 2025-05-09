use std::sync::Arc;

use configuration::{logging::init_logging, settings::Settings};
use log::info;
use nightfall_test::{run_tests::run_tests, webhook::run_webhook_server};


#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().unwrap();
    init_logging(
        settings.nightfall_test.log_level.as_str(),
        settings.log_app_only,
    );
    // set up a vec to hold responses from the webhook server
    let responses = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    // Start the webhook server
    info!("Starting webhook server...");
    let webhook_handle = tokio::spawn(run_webhook_server(responses.clone()));
    info!("Running tests...");
    let tests_handle = tokio::spawn(run_tests(responses.clone()));
    // run the tests
    webhook_handle.await.unwrap();
    tests_handle.await.unwrap();
}