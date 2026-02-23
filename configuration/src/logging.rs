use env_logger::Builder;
use log::LevelFilter;
use log_panics;
use std::env;

pub fn init_logging(log_level: &str, app_only: bool) {
    log_panics::init(); // this ensures that panics are logged
    
    // Check if RUST_LOG is set - if so, use it to allow fine-grained control
    let use_rust_log = env::var("RUST_LOG").is_ok();
    
    if use_rust_log {
        // Use RUST_LOG environment variable for fine-grained control
        Builder::from_env(env_logger::Env::default())
            .filter_module("alloy_provider", LevelFilter::Error)
            .filter_module("warp", LevelFilter::Warn)
            .filter_module("hyper", LevelFilter::Warn)
            .filter_module("tungstenite", LevelFilter::Warn)
            .init();
    } else if app_only {
        match log_level {
            "debug" => Builder::new()
                .filter_level(LevelFilter::Debug)
                .filter_module("alloy_provider", LevelFilter::Error)
                .filter_module("warp", LevelFilter::Warn)
                .filter_module("hyper", LevelFilter::Warn)
                .filter_module("tungstenite", LevelFilter::Warn)
                .init(),
            "info" => Builder::new()
                .filter_level(LevelFilter::Info)
                .filter_module("alloy_provider", LevelFilter::Error)
                .filter_module("warp", LevelFilter::Warn)
                .filter_module("hyper", LevelFilter::Warn)
                .filter_module("tungstenite", LevelFilter::Warn)
                .init(),
            "warn" => Builder::new().filter_level(LevelFilter::Warn).init(),
            "error" => Builder::new().filter_level(LevelFilter::Error).init(),
            _ => Builder::new().filter_level(LevelFilter::Info).init(),
        };
    } else {
        match log_level {
            "debug" => Builder::new().filter_level(LevelFilter::Debug).init(),
            "info" => Builder::new().filter_level(LevelFilter::Info).init(),
            "warn" => Builder::new().filter_level(LevelFilter::Warn).init(),
            "error" => Builder::new().filter_level(LevelFilter::Error).init(),
            _ => Builder::new().filter_level(LevelFilter::Info).init(),
        };
    };
}
