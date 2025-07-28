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
                "Command 'forge {command:?}' ran into an error without executing: {e}"
            );
        }
    }
}

// use configuration::{logging::init_logging, settings::Settings};
// use log::info;
// use std::{path::Path, process::Command};
// use alloy::primitives::Bytes;
// use alloy::provider::{Provider, ProviderBuilder};
// use alloy::SolType::SolConstructor;
// use std::fs;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("cargo:rerun-if-changed=blockchain_assets/contracts/Nightfall.sol");
//     let settings = Settings::new().unwrap();
//     init_logging(
//         settings.nightfall_deployer.log_level.as_str(),
//         settings.log_app_only,
//     );
//     info!("Started running nightfall_deployer/build.rs");

//     // 1. Compile contracts (no bindings)
//     if !is_foundry_installed() {
//         panic!("Foundry required: https://book.getfoundry.sh/getting-started/installation");
//     }

//     forge_command(&["build", "--skip", "test"]);

//     // 2. Deploy using Alloy (dynamic ABI)
//     let compiled_artifact = Path::new("out/Nightfall.sol/Nightfall.json");
//     deploy_with_alloy(compiled_artifact, &settings);

//     Ok(())
// }

// async fn deploy_with_alloy(
//     artifact_path: &Path,
//     settings: &Settings,
// ) -> Result<(), Box<dyn std::error::Error>> {
    
//     // 1. Parse compiled contract
//     let artifact: serde_json::Value = serde_json::from_str(&fs::read_to_string(artifact_path)?);
//     let bytecode = artifact["bytecode"]["object"].as_str().unwrap();
//     let abi = &artifact["abi"];

//     // 2. Set up Alloy provider
//     let provider = ProviderBuilder::new()
//         .with_recommended_settings()
//         .on_http(settings.ethereum_client_url.parse()?);

//     // 3. Dynamic deployment
//     let constructor = SolConstructor::new(abi)?;
//     let deploy_tx = constructor.deploy(Bytes::from(hex::decode(bytecode)?))?;

//     // 4. Send transaction
//     let receipt = provider
//         .send_transaction(deploy_tx)
//         .await?
//         .get_receipt()
//         .await?;

//     info!("Contract deployed at: {:?}", receipt.contract_address);
//     Ok(())
// }

// // Keep your existing helper functions
// fn is_foundry_installed() -> bool {
//     Command::new("forge").arg("--version").output().is_ok()
// }

// fn forge_command(args: &[&str]) {
//     let output = Command::new("forge").args(args).output().unwrap();
//     assert!(
//         output.status.success(),
//         "Forge command failed: {}",
//         String::from_utf8_lossy(&output.stderr)
//     );
// }