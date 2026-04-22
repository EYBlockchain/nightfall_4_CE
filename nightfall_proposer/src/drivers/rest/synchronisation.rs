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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::blockchain::nightfall_event_listener::get_synchronisation_status;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_synchronisation_route_reports_both_states() {
        let filter = synchronisation();

        get_synchronisation_status()
            .await
            .write()
            .await
            .set_synchronised();
        let synchronised = warp::test::request()
            .method("GET")
            .path("/v1/synchronisation")
            .reply(&filter)
            .await;
        assert_eq!(synchronised.status(), StatusCode::OK);
        assert_eq!(std::str::from_utf8(synchronised.body()).unwrap(), "Synchronised");

        get_synchronisation_status()
            .await
            .write()
            .await
            .clear_synchronised();
        let not_synchronised = warp::test::request()
            .method("GET")
            .path("/v1/synchronisation")
            .reply(&filter)
            .await;
        assert_eq!(not_synchronised.status(), StatusCode::OK);
        assert_eq!(
            std::str::from_utf8(not_synchronised.body()).unwrap(),
            "Not synchronised"
        );
    }
}
