use warp::{hyper::StatusCode, path, reply, Filter};

pub fn health_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    path!("v1" / "health")
        .map(warp::reply)
        .map(|_| reply::with_status("Healthy", StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn rest_healthy() {
        let filter = health_route();
        let res = warp::test::request()
            .method("GET")
            .path("/v1/health")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.body(), "Healthy");
    }
}
