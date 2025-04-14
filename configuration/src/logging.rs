use env_logger::Builder;
use log::LevelFilter;
use log_panics;

pub fn init_logging(log_level: &str, app_only: bool) {
    log_panics::init(); // this ensures that panics are logged
    if app_only {
        match log_level {
            "debug" => Builder::new()
                .filter_level(LevelFilter::Debug)
                .filter_module("ethers_providers", LevelFilter::Error)
                .filter_module("warp", LevelFilter::Warn)
                .filter_module("hyper", LevelFilter::Warn)
                .filter_module("tungstenite", LevelFilter::Warn)
                .init(),
            "info" => Builder::new()
                .filter_level(LevelFilter::Info)
                .filter_module("ethers_providers", LevelFilter::Error)
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
