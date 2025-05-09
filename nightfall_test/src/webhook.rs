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
    println!("Received webhook payload: {:?}", payload);
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


