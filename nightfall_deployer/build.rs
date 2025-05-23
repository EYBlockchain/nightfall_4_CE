use configuration::{logging::init_logging, settings::Settings};
use log::info;
use std::{os::unix::process::ExitStatusExt, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=blockchain_assets/contracts/Nightfall.sol");
    let settings = Settings::new().unwrap(); // Load configuration
    init_logging(
        settings.nightfall_deployer.log_level.as_str(),
        settings.log_app_only,
    );
    info!("Started running nightfall_deployer/build.rs");
    // Check and forge is installed
    if !is_foundry_installed() {
        info!("Foundry not installed, needed to continue please install via the guide found at https://book.getfoundry.sh/getting-started/installation");
        panic!("Foundry not installed, needed to continue please install via the guide found at https://book.getfoundry.sh/getting-started/installation");
    }

    forge_command(&["install"]);
    forge_command(&[
        "bind",
        "-b",
        "../nightfall_bindings",
        "--crate-name",
        "nightfall-bindings",
        "--skip-cargo-toml",
        "--overwrite",
        "--ethers",
    ]);
}

fn is_foundry_installed() -> bool {
    // Check if foundry is installed
    let cmd_output = Command::new("which").arg("forge").output();

    match cmd_output {
        Ok(output) => {
            if output.status.success() {
                info!(
                    "'which forge' successful got: {:?}",
                    std::str::from_utf8(&output.stdout)
                );
                true
            } else {
                info!("'which forge' not found, got: {:?}", output.stdout);
                false
            }
        }
        Err(e) => {
            info!("Got an error from running 'which forge': {}", e);
            false
        }
    }
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
            panic!(
                "Command 'forge {:?}' ran into an error without executing: {}",
                command, e
            );
        }
    }
}
