use configuration::settings::get_settings;
use log::{debug, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
/// Set up a warp server to listen for webhooks from the Nightfall client.
use warp::Filter;

pub async fn run_webhook_server(responses: Arc<Mutex<Vec<serde_json::Value>>>) {
    // Define the webhook route
    let webhook = warp::path("webhook")
        .and(warp::post())
        .and(with_responses(responses))
        .and(warp::body::json())
        .and_then(handle_webhook);

    // Start the server on port 8080
    warp::serve(webhook).run(([0, 0, 0, 0], 8080)).await;
}

async fn handle_webhook(
    responses: Arc<Mutex<Vec<serde_json::Value>>>,
    payload: serde_json::Value,
) -> Result<warp::http::StatusCode, warp::Rejection> {
    debug!("Received webhook payload: {payload:#?}");
    let mut responses = responses.lock().await;
    responses.push(payload);
    // Respond with a 200 OK status
    Ok(warp::http::StatusCode::OK)
}

fn with_responses(
    responses: Arc<Mutex<Vec<serde_json::Value>>>,
) -> impl Filter<Extract = (Arc<Mutex<Vec<serde_json::Value>>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || responses.clone())
}

/// Poll the Nightfall client queue every 5 seconds and print the length of the queue.
pub async fn poll_queue() {
    let url = &get_settings().nightfall_client.url;
    let url2 = "http://client2:3000";
    let client = reqwest::Client::new();
    loop {
        // poll the queue
        match client.get(format!("{url}/v1/queue")).send().await {
            Ok(response) if response.status().is_success() => {
                match response.text().await {
                    Ok(body) => debug!("Client 1 Queue length is : {body}"),
                    Err(err) => warn!("Failed to read response body for client1: {err}"),
                }
            }
            Ok(resp) => warn!("Client 1 returned status: {}", resp.status()),
            Err(err) => warn!("Failed to poll client1: {err}"),
        }
        // poll the queue for client 2
        match client.get(format!("{url2}/v1/queue")).send().await {
            Ok(response) if response.status().is_success() => {
                match response.text().await {
                    Ok(body2) => debug!("Client 2 Queue length is : {body2}"),
                    Err(err) => warn!("Failed to read response body for client2: {err}"),
                }
            }
            Ok(resp) => warn!("Client 2 returned status: {}", resp.status()),
            Err(err) => warn!("Failed to poll client2: {err}"),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
