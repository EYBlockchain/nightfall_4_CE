use log::info;
use std::{os::unix::process::ExitStatusExt, process::Command};

fn main() {
    info!("Started running nightfall_deployer/build.rs");
    // Check and forge is installed
    if !is_foundry_installed() {
        info!("Foundry not installed, needed to continue please install via the guide found at https://book.getfoundry.sh/getting-started/installation");
        panic!("Foundry not installed, needed to continue please install via the guide found at https://book.getfoundry.sh/getting-started/installation");
    }
    // Run forge install
    forge_command(&["install"]);
    forge_command(&["build"]);
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
            info!("Got an error from running 'which forge': {e}");
            info!("Got an error from running 'which forge': {e}");
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
            panic!("Command 'forge {command:?}' ran into an error without executing: {e}");
        }
    }
}
