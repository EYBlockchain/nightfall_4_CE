use lazy_static::lazy_static;
use prometheus::{
    default_registry, register_histogram_vec, register_int_counter_vec, register_int_gauge,
    Encoder, HistogramVec, IntCounterVec, IntGauge, TextEncoder,
};
use warp::Filter;

lazy_static! {
    pub static ref CLIENT_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "client_requests_total",
        "Total number of HTTP requests handled by the client",
        &["endpoint", "method", "status"]
    )
    .expect("failed to register client_requests_total");
    pub static ref CLIENT_REQUEST_DURATION_SECONDS: HistogramVec = register_histogram_vec!(
        "client_request_duration_seconds",
        "Duration of HTTP requests handled by the client in seconds",
        &["endpoint"]
    )
    .expect("failed to register client_request_duration_seconds");
    pub static ref CLIENT_QUEUE_DEPTH: IntGauge = register_int_gauge!(
        "client_queue_depth",
        "Current depth of the client request queue"
    )
    .expect("failed to register client_queue_depth");
}

pub fn metrics() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::get())
        .map(|| {
            // Ensure lazy_static metrics are initialised by touching them.
            let _ = CLIENT_REQUESTS_TOTAL.desc();
            let _ = CLIENT_REQUEST_DURATION_SECONDS.desc();
            let _ = CLIENT_QUEUE_DEPTH.desc();

            let encoder = TextEncoder::new();
            let metric_families = default_registry().gather();
            let mut buffer = Vec::new();
            encoder
                .encode(&metric_families, &mut buffer)
                .expect("failed to encode metrics");

            warp::http::Response::builder()
                .header("Content-Type", encoder.format_type())
                .body(buffer)
                .unwrap()
        })
}
