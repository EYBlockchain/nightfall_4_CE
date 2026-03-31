use std::env;
use tracing_subscriber::EnvFilter;

pub fn init_logging(log_level: &str, app_only: bool) {
    // Check if RUST_LOG is set - if so, use it to allow fine-grained control
    let use_rust_log = env::var("RUST_LOG").is_ok();

    let base_filter = if use_rust_log {
        // Use RUST_LOG environment variable for fine-grained control
        EnvFilter::from_default_env()
    } else if app_only {
        match log_level {
            "debug" => EnvFilter::new("debug"),
            "info" => EnvFilter::new("info"),
            "warn" => EnvFilter::new("warn"),
            "error" => EnvFilter::new("error"),
            _ => EnvFilter::new("info"),
        }
    } else {
        match log_level {
            "debug" => EnvFilter::new("debug"),
            "info" => EnvFilter::new("info"),
            "warn" => EnvFilter::new("warn"),
            "error" => EnvFilter::new("error"),
            _ => EnvFilter::new("info"),
        }
    };

    // Apply module-level overrides when using RUST_LOG or app_only mode
    let filter = if use_rust_log || app_only {
        base_filter
            .add_directive("alloy_provider=error".parse().unwrap())
            .add_directive("warp=warn".parse().unwrap())
            .add_directive("hyper=warn".parse().unwrap())
            .add_directive("tungstenite=warn".parse().unwrap())
    } else {
        base_filter
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
