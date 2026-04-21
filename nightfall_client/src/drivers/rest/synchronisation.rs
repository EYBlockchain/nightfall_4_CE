use warp::{
    path,
    reply::{self, Reply},
    Filter,
};

use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    ports::contracts::NightfallContract,
};
use lib::error::EventHandlerError;
use std::future::Future;

pub fn synchronisation<N: NightfallContract>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "synchronisation")
        .and(warp::get())
        .and_then(handle_synchronisation::<N>)
}

pub(super) async fn handle_synchronisation_with<F, Fut>(
    fetch_status: F,
) -> Result<impl Reply, warp::Rejection>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<lib::shared_entities::SynchronisationStatus, EventHandlerError>>,
{
    match fetch_status().await {
        Ok(status) => Ok(reply::json(&status)),
        Err(_) => Err(warp::reject::custom(
            crate::domain::error::ClientRejection::SynchronisationUnavailable,
        )),
    }
}

pub async fn handle_synchronisation<N: NightfallContract>() -> Result<impl Reply, warp::Rejection> {
    handle_synchronisation_with(|| get_synchronisation_status::<N>()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::shared_entities::{SynchronisationPhase, SynchronisationStatus};

    #[tokio::test]
    async fn test_handle_synchronisation_returns_status_payload() {
        let filter = warp::path!("v1" / "synchronisation")
            .and(warp::get())
            .and_then(|| async {
                handle_synchronisation_with(|| async {
                    Ok(SynchronisationStatus::new(
                        SynchronisationPhase::Synchronized,
                    ))
                })
                .await
            });
        let response = warp::test::request()
            .method("GET")
            .path("/v1/synchronisation")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), warp::http::StatusCode::OK);
        let body = serde_json::from_slice::<serde_json::Value>(response.body()).unwrap();
        assert_eq!(body["phase"], "Synchronized");
    }
}
