use configuration::{logging::init_logging, settings::Settings};
use log::info;
use std::{ os::unix::process::ExitStatusExt, process::Command,};

fn main() {
    println!("cargo:rerun-if-changed=blockchain_assets/contracts/Nightfall.sol");
    let settings = Settings::new().unwrap(); // Load configuration
    init_logging(
        settings.nightfall_deployer.log_level.as_str(),
        settings.log_app_only,
    );
    info!("Started running nightfall_bindings/build.rs");

    forge_command(&["build"]);
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
fn forge_command(command: &[&str]) {
    let output = Command::new("forge").args(command).output();

    match output {
        Ok(o) => {
            if o.status.success() {
                info!(
                    "Command 'forge {:?}' executed successfully: {}",
                    command,
                    String::from_utf8_lossy(&o.stdout)
                );
            } else {
                panic!(
                "Command 'forge {:?}' executed with failing error code: {:?}\nStandard Output: {}\nStandard Error: {}",
                command,
                o.status.signal(),
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            }
        }
        Err(e) => {
            panic!("Command 'forge {command:?}' ran into an error without executing: {e}");
        }
    }
}
