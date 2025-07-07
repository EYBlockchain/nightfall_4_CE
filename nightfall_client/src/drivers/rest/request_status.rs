use crate::{driven::queue::get_queue, initialisation::get_db_connection, ports::db::RequestDB};
use log::debug;
use uuid::Uuid;
use warp::{http::StatusCode, path, reply::Reply, Filter};

/// This module provides an end point for querying the status of a request
pub fn get_request_status(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "request" / String)
        .and(warp::get())
        .and_then(handle_get_request_status)
}

pub async fn handle_get_request_status(id: String) -> Result<impl Reply, warp::Rejection> {
    // check if the id is a valid uuid
    match Uuid::parse_str(&id) {
        Ok(_) => {}
        Err(_) => {
            return Err(warp::reject::custom(
                crate::domain::error::ClientRejection::InvalidRequestId,
            ));
        }
    };
    let db = get_db_connection().await;
    // get the request
    debug! {"Getting request status for {id}"};
    let request = db.get_request(&id).await;
    debug! {"Request status: {request:?}"};
    if let Some(request) = request {
        Ok(warp::reply::with_status(
            serde_json::to_string(&request).unwrap(),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(
            crate::domain::error::ClientRejection::RequestNotFound,
        ))
    }
}

/// This endpoint is used to get the length of thr request queue
pub fn get_queue_length(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "queue")
        .and(warp::get())
        .and_then(handle_get_queue_length)
}
pub async fn handle_get_queue_length() -> Result<impl Reply, warp::Rejection> {
    let length = get_queue().await.read().await.len();
    Ok(warp::reply::with_status(
        serde_json::to_string(&length).unwrap(),
        StatusCode::OK,
    ))
}
