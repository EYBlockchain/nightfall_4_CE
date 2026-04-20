use crate::initialisation::get_db_connection;
use mongodb::bson::doc;
use serde_json::json;
use warp::{hyper::StatusCode, path, reply, Filter};

pub fn readiness_check(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "ready")
        .and(warp::get())
        .and_then(handle_readiness)
}

async fn handle_readiness() -> Result<impl warp::Reply, warp::Rejection> {
    let db = get_db_connection().await;
    match db.database("nightfall").run_command(doc! { "ping": 1 }).await {
        Ok(_) => {
            let body = json!({
                "status": "ready",
                "checks": { "database": "ok" }
            });
            Ok(reply::with_status(reply::json(&body), StatusCode::OK))
        }
        Err(e) => {
            let body = json!({
                "status": "not_ready",
                "checks": { "database": format!("error: {e}") }
            });
            Ok(reply::with_status(
                reply::json(&body),
                StatusCode::SERVICE_UNAVAILABLE,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: integration tests requiring a real MongoDB connection are not included here.
    // The readiness endpoint should be tested via integration tests with a running database.
}
