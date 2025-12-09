use log::info;
use std::{
    env, os::unix::process::ExitStatusExt, path::Path, path::PathBuf, process::Command,
};

fn main() {
    // Find repo root: nightfall_bindings/.. is repo root
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap().to_path_buf();

    // Check if artifacts exist - they should be pre-compiled by deployer or exist already
    let nightfall_artifact =
        repo_root.join("blockchain_assets/artifacts/Nightfall.sol/Nightfall.json");
    
    if !nightfall_artifact.exists() {
        // If artifacts don't exist, compile them
        info!("Artifacts not found, building with forge");
        forge_command(&["build"]);
    } else {
        info!("Using existing artifacts from blockchain_assets/artifacts/");
    }
    
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
    
    println!("cargo:warning=Contract verification will use runtime hashes from contract_hashes.toml");
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
fn forge_command(command: &[&str]) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();
    let output = Command::new("forge")
        .args(command)
        .current_dir(repo_root)
        .output();

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