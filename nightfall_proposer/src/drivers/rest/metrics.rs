use lazy_static::lazy_static;
use prometheus::{
    default_registry, register_histogram, register_int_counter, register_int_counter_vec,
    register_int_gauge, Encoder, Histogram, IntCounter, IntCounterVec, IntGauge, TextEncoder,
};
use warp::Filter;

lazy_static! {
    pub static ref PROPOSER_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "proposer_requests_total",
        "Total number of HTTP requests handled by the proposer",
        &["endpoint", "method", "status"]
    )
    .expect("failed to register proposer_requests_total");
    pub static ref PROPOSER_BLOCKS_PROPOSED_TOTAL: IntCounter = register_int_counter!(
        "proposer_blocks_proposed_total",
        "Total number of blocks proposed"
    )
    .expect("failed to register proposer_blocks_proposed_total");
    pub static ref PROPOSER_BLOCK_PROVING_DURATION_SECONDS: Histogram = register_histogram!(
        "proposer_block_proving_duration_seconds",
        "Duration of block proving in seconds"
    )
    .expect("failed to register proposer_block_proving_duration_seconds");
    pub static ref PROPOSER_MEMPOOL_SIZE: IntGauge = register_int_gauge!(
        "proposer_mempool_size",
        "Current number of transactions in the proposer mempool"
    )
    .expect("failed to register proposer_mempool_size");
}

pub fn metrics() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::get())
        .map(|| {
            // Ensure lazy_static metrics are initialised by touching them.
            let _ = PROPOSER_REQUESTS_TOTAL.desc();
            let _ = PROPOSER_BLOCKS_PROPOSED_TOTAL.desc();
            let _ = PROPOSER_BLOCK_PROVING_DURATION_SECONDS.desc();
            let _ = PROPOSER_MEMPOOL_SIZE.desc();

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
