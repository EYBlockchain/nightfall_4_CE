use warp::{
    path,
    reply::{self, Reply},
    Filter,
};

use crate::drivers::blockchain::nightfall_event_listener::get_synchronisation_status;

pub fn synchronisation(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "synchronisation")
        .and(warp::get())
        .and_then(handle_synchronisation)
}

pub async fn handle_synchronisation() -> Result<impl Reply, warp::Rejection> {
    let synchronised = get_synchronisation_status()
        .await
        .read()
        .await
        .is_synchronised();
    if synchronised {
        Ok(reply::with_status(
            "Synchronised",
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(reply::with_status(
            "Not synchronised",
            warp::http::StatusCode::OK,
        ))
    }
}
