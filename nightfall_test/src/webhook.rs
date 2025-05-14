use configuration::settings::get_settings;
use log::{debug, warn};
/// Set up a warp server to listen for webhooks from the Nightfall client.
use warp::Filter;
use tokio::sync::Mutex;
use std::sync::Arc;

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
    debug!("Received webhook payload: {:#?}", payload);
    let mut responses = responses.lock().await;
    responses.push(payload);
    // Respond with a 200 OK status
    Ok(warp::http::StatusCode::OK)
}

fn with_responses(
    responses: Arc<Mutex<Vec<serde_json::Value>>>,
) -> impl Filter<Extract = (Arc<Mutex<Vec<serde_json::Value>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || responses.clone())
}

/// Poll the Nightfall client queue every 5 seconds and print the length of the queue.
pub async fn poll_queue() {
    let url = &get_settings().nightfall_client.url;
    let url2 = "http://client2:3000";
    let client = reqwest::Client::new();
    loop {
        // poll the queue
        let response = client
            .get(format!("{}/v1/queue", url))
            .send()
            .await
            .unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            debug!("Client 1 Queue length is : {}", body);
        } else {
            warn!("Failed to poll the queue");
        }
        // poll the queue for client 2
        let response2 = client
            .get(format!("{}/v1/queue", url2))
            .send()
            .await
            .unwrap();
        if response2.status().is_success() {
            let body2 = response2.text().await.unwrap();
            debug!("Client 2 Queue length is : {}", body2);
        } else {
            warn!("Failed to poll the queue");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
