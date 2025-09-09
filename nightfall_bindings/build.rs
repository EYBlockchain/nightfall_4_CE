use log::info;
use std::{os::unix::process::ExitStatusExt, path::Path, process::Command};

fn main() {
    info!("Building the artifacts");
    // Run forge build
    forge_command(&["build"]);
    // read the artifacts.rs and replace the dummy_artifact with artifacts
    let artifacts_path = Path::new("../nightfall_bindings/src/artifacts.rs");
    if artifacts_path.exists() {
        info!("Artifacts file found at {artifacts_path:?}");
        let content =
            std::fs::read_to_string(artifacts_path).expect("Failed to read artifacts.rs file");
        let updated_content = content.replace("dummy_artifacts", "artifacts");
        std::fs::write(artifacts_path, updated_content)
            .expect("Failed to write updated artifacts.rs file");
    } else {
        panic!("Artifacts file not found at {artifacts_path:?}");
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
