use configuration::{logging::init_logging, settings::Settings};
use log::{info, warn};
use nightfall_deployer::deployment::deploy_contracts;

#[tokio::main]
async fn main() {
    // Print banner immediately on startup
    println!(
        r#"
███╗   ██╗██╗ ██████╗ ██╗  ██╗████████╗███████╗ █████╗ ██╗     ██╗        ██╗  ██╗
████╗  ██║██║██╔════╝ ██║  ██║╚══██╔══╝██╔════╝██╔══██╗██║     ██║        ██║  ██║
██╔██╗ ██║██║██║  ███╗███████║   ██║   █████╗  ███████║██║     ██║        ███████║
██║╚██╗██║██║██║   ██║██╔══██║   ██║   ██╔══╝  ██╔══██║██║     ██║        ╚════██║
██║ ╚████║██║╚██████╔╝██║  ██║   ██║   ██║     ██║  ██║███████╗███████╗███████╗██║
╚═╝  ╚═══╝╚═╝╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝╚═╝

Nightfall_4,
https://github.com/EYBlockchain/nightfall_4_CE
"#
    );
    let settings: Settings = Settings::new().unwrap();
    init_logging(
        settings.nightfall_deployer.log_level.as_str(),
        settings.log_app_only,
    );
    info!("Deployer has started");

    if &settings.signing_key == "set by environment" {
        panic!("Set the deployer signing key in env variable NF4_DEPLOYER_SIGNING_KEY")
    };
    if settings.contracts.deploy_contracts {
        info!("Deploying contracts");
        deploy_contracts(&settings).await.unwrap();
    } else {
        warn!("Skipping contract deployment");
    }
}
