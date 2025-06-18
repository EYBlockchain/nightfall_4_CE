use warp::{
    path,
    reply::{self, Reply},
    Filter,
};

use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    ports::contracts::NightfallContract,
};

pub fn synchronisation<N: NightfallContract>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "synchronisation")
        .and(warp::get())
        .and_then(handle_synchronisation::<N>)
}

pub async fn handle_synchronisation<N: NightfallContract>() -> Result<impl Reply, warp::Rejection> {
    match get_synchronisation_status::<N>().await {
        Ok(status) => Ok(reply::json(&status)),
        Err(_) => Err(warp::reject::custom(
            crate::domain::error::ClientRejection::SynchronisationUnavailable,
        )),
    }
}
