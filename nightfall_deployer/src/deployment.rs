use crate::vk_contract::write_vk_to_nightfall_toml;
use alloy::{hex, primitives::Address};
use configuration::{
    addresses::{Addresses, Sources},
    settings::Settings,
};
use jf_plonk::recursion::RecursiveProver;

use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, error, info};
use nightfall_proposer::driven::rollup_prover::RollupProver;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::File,
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
};

fn proxies_from_broadcast(path: &Path) -> anyhow::Result<HashMap<&'static str, Address>> {
    let v: Value = serde_json::from_reader(File::open(path)?)?;
    let txs = v
        .get("transactions")
        .and_then(|t| t.as_array())
        .ok_or_else(|| anyhow::anyhow!("no transactions in broadcast"))?;

    let mut map = HashMap::new();
    let mut last_impl_name: Option<String> = None;

    for tx in txs {
        let ttype = tx
            .get("transactionType")
            .and_then(|x| x.as_str())
            .unwrap_or("");
        let cname = tx
            .get("contractName")
            .and_then(|x| x.as_str())
            .unwrap_or("");
        let caddr_s = tx.get("contractAddress").and_then(|x| x.as_str());

        // Upgrades.deployUUPSProxy deploys: implementation (CREATE, contractName = Nightfall/RoundRobin/X509), then ERC1967Proxy (CREATE)
        if ttype == "CREATE" && cname != "ERC1967Proxy" && !cname.is_empty() {
            last_impl_name = Some(cname.to_string());
        }

        if ttype == "CREATE" && cname == "ERC1967Proxy" {
            if let (Some(prev), Some(addr_s)) = (last_impl_name.as_deref(), caddr_s) {
                let addr: Address = addr_s.parse()?;
                if prev.contains("Nightfall") {
                    map.insert("nightfall", addr);
                } else if prev.contains("RoundRobin") {
                    map.insert("round_robin", addr);
                } else if prev.contains("X509") {
                    map.insert("x509", addr);
                } else if prev.contains("RollupProofVerifier") {
                    map.insert("verifier", addr);
                }
            }
        }
    }

    if map.is_empty() {
        anyhow::bail!("no proxies found in broadcast");
    }
    Ok(map)
}

pub async fn deploy_contracts(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("NF4_RUN_MODE", &settings.run_mode);
    
    // Clean up potentially corrupted build-info files from Docker build stage
    let build_info_path = PathBuf::from("blockchain_assets/artifacts/build-info");
    if build_info_path.exists() {
        info!("Cleaning build-info directory to ensure fresh compilation");
        std::fs::remove_dir_all(&build_info_path).ok();
    }

    // Also clean cache to ensure deterministic compilation
    let cache_path = PathBuf::from("blockchain_assets/cache");
    if cache_path.exists() {
        info!("Cleaning cache directory");
        std::fs::remove_dir_all(&cache_path).ok();
    }

    if !settings.mock_prover && settings.contracts.deploy_contracts {
        let vk = RollupProver::get_decider_vk();
        let _ = write_vk_to_nightfall_toml(&vk);
    }

    // Force a clean rebuild to generate proper build-info files for OpenZeppelin validation
    info!("Building contracts with forge");
    forge_command(&["build", "--force"]);

    info!("Deploying contracts with forge script");
    forge_command(&[
        "script",
        "Deployer",
        "--fork-url",
        &settings.ethereum_client_url,
        "--broadcast",
    ]);

    // -------- read Foundry broadcast --------
    let cwd = std::env::current_dir()?;
    let path_out = cwd
        .join(&settings.contracts.deployment_file)
        .join(settings.network.chain_id.to_string())
        .join("run-latest.json");

    if !path_out.is_file() {
        return Err(format!("Deployment log file not found: {path_out:?}").into());
    }
    let mut addresses = Addresses {
        chain_id: settings.network.chain_id,
        nightfall: Address::ZERO,
        round_robin: Address::ZERO,
        x509: Address::ZERO,
        verifier: Address::ZERO,
    };
    // -------- replace with *proxy* addresses from broadcast --------
    match proxies_from_broadcast(&path_out) {
        Ok(proxy_map) => {
            if let Some(a) = proxy_map.get("nightfall") {
                addresses.nightfall = *a;
            }
            if let Some(a) = proxy_map.get("round_robin") {
                addresses.round_robin = *a;
            }
            if let Some(a) = proxy_map.get("x509") {
                addresses.x509 = *a;
            }
            if let Some(a) = proxy_map.get("verifier") {
                addresses.verifier = *a;
            }
            if settings.mock_prover {
                if addresses.nightfall == Address::ZERO
                    || addresses.round_robin == Address::ZERO
                    || addresses.x509 == Address::ZERO
                {
                    error!("Missing proxy addresses after extraction");
                    return Err("Failed to extract all proxy addresses from deployment".into());
                }
                info!(
                    "Extracted proxy addresses: nightfall={:?}, round_robin={:?}, x509={:?}",
                    addresses.nightfall, addresses.round_robin, addresses.x509
                );
            } else {
                if addresses.nightfall == Address::ZERO
                    || addresses.round_robin == Address::ZERO
                    || addresses.x509 == Address::ZERO
                    || addresses.verifier == Address::ZERO
                {
                    error!("Missing proxy addresses after extraction");
                    return Err("Failed to extract all proxy addresses from deployment".into());
                }
                info!(
                    "Extracted proxy addresses: nightfall={:?}, round_robin={:?}, x509={:?}, verifier={:?}",
                    addresses.nightfall, addresses.round_robin, addresses.x509, addresses.verifier
                );
            }
        }
        Err(e) => {
            error!("Failed to parse deployment broadcast file: {e}");
            return Err(
                format!("Deployment failed: could not extract proxy addresses: {e}").into(),
            );
        }
    }
    // -------- Save addresses to file --------
    let file_path = PathBuf::from("/app/configuration/toml/addresses.toml");
    info!("Saving addresses for chain_id: {}", addresses.chain_id);
    addresses.save(Sources::File(file_path)).await?;
    info!("Addresses saved successfully");
    
    save_deployed_hashes(&addresses).await?;
    
    Ok(())
}

/// Save the hashes of the deployed contract implementations
/// This allows proposer/client to verify they are using the correct contracts
async fn save_deployed_hashes(addresses: &Addresses) -> Result<(), Box<dyn std::error::Error>> {
    use lib::{
        initialisation::get_blockchain_client_connection,
        verify_contract::{get_onchain_code_hash, get_proxy_implementation},
    };

    info!("Calculating deployed contract hashes for verification");

    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let provider = blockchain_client.root();
    
    // Get implementation addresses
    let nf_impl = get_proxy_implementation(&provider, addresses.nightfall).await?;
    let rr_impl = get_proxy_implementation(&provider, addresses.round_robin).await?;
    let x509_impl = get_proxy_implementation(&provider, addresses.x509).await?;
    
    // Get on-chain bytecode hashes (with metadata stripped)
    let nf_hash = get_onchain_code_hash(&provider, nf_impl).await?;
    let rr_hash = get_onchain_code_hash(&provider, rr_impl).await?;
    let x509_hash = get_onchain_code_hash(&provider, x509_impl).await?;
    
    info!("Nightfall implementation hash: 0x{}", hex::encode(nf_hash));
    info!("RoundRobin implementation hash: 0x{}", hex::encode(rr_hash));
    info!("X509 implementation hash: 0x{}", hex::encode(x509_hash));
    
    // Save to TOML file that will be read by proposer/client
    let hashes_path = PathBuf::from("/app/configuration/toml/contract_hashes.toml");
    let hashes_toml = format!(
        "nightfall_hash = \"{}\"\nround_robin_hash = \"{}\"\nx509_hash = \"{}\"\n",
        hex::encode(nf_hash),
        hex::encode(rr_hash),
        hex::encode(x509_hash)
    );
    std::fs::write(&hashes_path, hashes_toml)?;
    info!("Contract hashes saved to {hashes_path:?}");
    
    Ok(())
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
pub fn forge_command(command: &[&str]) {
    debug!("DEBUG: Running forge command: {command:?}"); // Use info! as forge_command already uses info!
    let output = std::process::Command::new("forge").args(command).output();

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

// Todo: fix unwrap panic in test and re-enable test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use alloy::providers::{Provider, ProviderBuilder};
//     use alloy_node_bindings::Anvil;
//     use configuration::addresses::get_addresses;
//     use nightfall_bindings::artifacts::Nightfall;
//     use std::{fs, path::Path};
//     use tokio::task::spawn_blocking;
//     use url::Url;
//     use std::{fs, path::Path};

//     use nightfall_bindings::artifacts::Nightfall;
//     use tokio::task::spawn_blocking;

//     // NB: This test requires Anvil to be installed (it will use Anvil to simulate a blockchain).
//     // Restart VS Code after installing Anvil so that it's in your PATH otherwise VS Code won't find it!
//     #[tokio::test]
//     async fn test_deploy_contracts() {
//         // fire up a blockchain simulator
//         let mut settings = Settings::new().unwrap();
//         std::env::set_var(
//             "NF4_SIGNING_KEY",
//             "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
//         );
//         settings.ethereum_client_url = "http://localhost:8545".to_string(); // we're running bare metal so a docker url won't work
//         let url = Url::parse(&settings.ethereum_client_url).unwrap();
//         let anvil = Anvil::new()
//             .port(
//                 url.port()
//                     .expect("Could not get Anvil instance. Have you installed it?"),
//             )
//             .spawn();
//         tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
//         // set the current working directory to be the project root
//         let root = "../";
//         std::env::set_current_dir(root).unwrap();

//         // run the deploy function and get the contract addresses

//         deploy_contracts(&settings).await.unwrap();
//         // get a blockchain provider so we can interrogate the deployed code
//         let provider = ProviderBuilder::new()
//             .disable_recommended_fillers()
//             .connect_http(anvil.endpoint_url());

//         let code = provider
//             // use spawn blocking because the blocking reqwest client is not async and it complains (but we need loading the addresses to be sync elsewhere)
//             .get_code_at(spawn_blocking(get_addresses).await.unwrap().nightfall())
//             .await
//             .unwrap();
//         assert_eq!(code, Nightfall::DEPLOYED_BYTECODE);
//         // clean up by remvoing the addresses file and directory that this test created
//         fs::remove_dir_all(Path::new("configuration/toml")).unwrap();
//     }
// }
