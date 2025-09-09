use configuration::{
    addresses::{Addresses, Sources},
    settings::Settings,
};

use log::{info, warn};

use nightfall_proposer::driven::rollup_prover::RollupProver;
use std::{error::Error, os::unix::process::ExitStatusExt};
use url::Url;

use crate::vk_contract::write_vk_to_nightfall_toml;

use std::{collections::HashMap, fs::File, path::{Path, PathBuf}};
use ethers::types::Address;
use serde_json::Value;

fn proxies_from_broadcast(path: &Path) -> anyhow::Result<HashMap<&'static str, Address>> {
    let v: Value = serde_json::from_reader(File::open(path)?)?;
    let txs = v.get("transactions").and_then(|t| t.as_array()).ok_or_else(|| anyhow::anyhow!("no transactions in broadcast"))?;

    let mut map = HashMap::new();
    let mut last_impl_name: Option<String> = None;

    for tx in txs {
        let ttype   = tx.get("transactionType").and_then(|x| x.as_str()).unwrap_or("");
        let cname   = tx.get("contractName").and_then(|x| x.as_str()).unwrap_or("");
        let caddr_s = tx.get("contractAddress").and_then(|x| x.as_str());

        // Upgrades.deployUUPSProxy deploys: implementation (CREATE, contractName = Nightfall/RoundRobin/X509), then ERC1967Proxy (CREATE)
        if ttype == "CREATE" && cname != "ERC1967Proxy" && !cname.is_empty() {
            last_impl_name = Some(cname.to_string());
        }

        if ttype == "CREATE" && cname == "ERC1967Proxy" {
            if let (Some(prev), Some(addr_s)) = (last_impl_name.as_deref(), caddr_s) {
                let addr: Address = addr_s.parse()?;
                if prev.contains("Nightfall")      { map.insert("nightfall", addr); }
                else if prev.contains("RoundRobin") { map.insert("round_robin", addr); }
                else if prev.contains("X509")       { map.insert("x509", addr); }
            }
        }
    }

    if map.is_empty() { anyhow::bail!("no proxies found in broadcast"); }
    Ok(map)
}

pub async fn deploy_contracts(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("NF4_RUN_MODE", &settings.run_mode);

    if !settings.mock_prover && settings.contracts.deploy_contracts {
        forge_command(&["build", "--force"]);
        let vk = RollupProver::get_decider_vk();
        let _ = write_vk_to_nightfall_toml(&vk);
    }

    forge_command(&[
        "script","Deployer","--fork-url",&settings.ethereum_client_url,"--broadcast","--force",
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

    // This loads whatever current parser extracts (likely impl addresses)
    let mut addresses = Addresses::load(Sources::parse(
        path_out.to_str().ok_or("Couldn't convert path to str")?,
    )?)?;

    // -------- replace with *proxy* addresses from broadcast --------
    if let Ok(proxy_map) = proxies_from_broadcast(&path_out) {
        if let Some(a) = proxy_map.get("nightfall")   { addresses.nightfall   = *a; }
        if let Some(a) = proxy_map.get("round_robin") { addresses.round_robin = *a; }
        if let Some(a) = proxy_map.get("x509")        { addresses.x509        = *a; }
    }

    // -------- save to config server + local fallback --------
    let mut search = |path: &Path| -> Option<PathBuf> {
        if path.is_absolute() && path.is_file() { return Some(path.to_path_buf()); }
        let mut cwd = std::env::current_dir().ok()?;
        loop {
            let candidate = cwd.join(path);
            if candidate.is_file() { return Some(candidate); }
            cwd = cwd.parent()?.to_path_buf();
        }
    };

    let url = url::Url::parse(&settings.configuration_url)?
        .join(&settings.contracts.addresses_file)?;
    if addresses.save(Sources::Http(url)).await.is_err() {
        warn!("Failed to save to configuration server. Saving locally instead.");
    }
    let file_path = search(Path::new(&settings.contracts.addresses_file))
        .unwrap_or_else(|| PathBuf::from(&settings.contracts.addresses_file));
    addresses.save(Sources::File(file_path)).await
        .expect("Failed to save addresses locally");

    Ok(())
}


// pub async fn deploy_contracts(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
//     std::env::set_var("NF4_RUN_MODE", &settings.run_mode);

//     if !settings.mock_prover && settings.contracts.deploy_contracts {
//         forge_command(&["build", "--force"]);
//         let vk = RollupProver::get_decider_vk();
//         let _ = write_vk_to_nightfall_toml(&vk);
//     }

//     forge_command(&[
//         "script","Deployer","--fork-url",&settings.ethereum_client_url,"--broadcast","--force",
//     ]);

//     // -------- read Foundry broadcast --------
//     let cwd = std::env::current_dir()?;
//     let path_out = cwd
//         .join(&settings.contracts.deployment_file)
//         .join(settings.network.chain_id.to_string())
//         .join("run-latest.json");

//     if !path_out.is_file() {
//         return Err(format!("Deployment log file not found: {path_out:?}").into());
//     }

//     // This loads whatever your current parser extracts (likely impl addresses)
//     let mut addresses = Addresses::load(Sources::parse(
//         path_out.to_str().ok_or("Couldn't convert path to str")?,
//     )?)?;

//     // -------- replace with *proxy* addresses from broadcast --------
//     if let Ok(proxy_map) = proxies_from_broadcast(&path_out) {
//         if let Some(a) = proxy_map.get("nightfall")   { addresses.nightfall   = *a; }
//         if let Some(a) = proxy_map.get("round_robin") { addresses.round_robin = *a; }
//         if let Some(a) = proxy_map.get("x509")        { addresses.x509        = *a; }
//     }

//     // -------- save to config server + local fallback --------
//     let mut search = |path: &Path| -> Option<PathBuf> {
//         if path.is_absolute() && path.is_file() { return Some(path.to_path_buf()); }
//         let mut cwd = std::env::current_dir().ok()?;
//         loop {
//             let candidate = cwd.join(path);
//             if candidate.is_file() { return Some(candidate); }
//             cwd = cwd.parent()?.to_path_buf();
//         }
//     };

//     let url = url::Url::parse(&settings.configuration_url)?
//         .join(&settings.contracts.addresses_file)?;
//     if addresses.save(Sources::Http(url)).await.is_err() {
//         warn!("Failed to save to configuration server. Saving locally instead.");
//     }
//     let file_path = search(Path::new(&settings.contracts.addresses_file))
//         .unwrap_or_else(|| PathBuf::from(&settings.contracts.addresses_file));
//     addresses.save(Sources::File(file_path)).await
//         .expect("Failed to save addresses locally");

//     Ok(())
// }


// pub async fn deploy_contracts(settings: &Settings) -> Result<(), Box<dyn Error>> {
//     // The deployment script will need to know the run mode we are in so that it can use the correct configuration.
//     // The best way to do this is to read the settings and set an environment variable
//     // which the script can then read.
//     std::env::set_var("NF4_RUN_MODE", &settings.run_mode); // this is possibly already set but if it was, it will be the same as settings.run_mode.
//     if !settings.mock_prover && settings.contracts.deploy_contracts {
//         forge_command(&["build", "--force"]);
//         let vk = RollupProver::get_decider_vk();
//         // create_vk_contract::<false>(&vk, settings);
//         let _ = write_vk_to_nightfall_toml(&vk);
//     }

//     forge_command(&[
//         "script",
//         "Deployer",
//         "--fork-url",
//         &settings.ethereum_client_url,
//         "--broadcast",
//         "--force",
//     ]);

//     // read the deployment log file to extract the contract addresses
//     let cwd = std::env::current_dir()?;

//     let path_out = cwd
//         .join(&settings.contracts.deployment_file)
//         .join(settings.network.chain_id.to_string())
//         .join("run-latest.json");

//     if !path_out.is_file() {
//         return Err(format!(
//             "Deployment log file not found at the expected location: {path_out:?}"
//         )
//         .into());
//     }
//     let addresses = Addresses::load(Sources::parse(
//         path_out.to_str().ok_or("Couldn't convert path to str")?,
//     )?)?;

//     // next, try to find the addresses.toml file. If we can't find it, output the location
//     // in the config file `nightfall.toml`
//     let cwd = std::env::current_dir()?;
//     let mut cwd = cwd.as_path();
//     let mut file_path = cwd.join(&settings.contracts.addresses_file);
//     let original_file_path = file_path.clone();
//     // if we can't find the file, we'll look in the parent directories until we find it
//     while !file_path.is_file() {
//         cwd = if let Some(p) = cwd.parent() {
//             p
//         } else {
//             // if we can't find the file, we'll use the path in the config file and create it later
//             file_path = original_file_path;
//             break;
//         };
//         file_path = cwd.join(&settings.contracts.addresses_file);
//     }

//     // now we have the addresses we'll save them to the configuration server, if available
//     // as a backup, we'll save to the local file system (which will then need to be mounted in the other containers if it's to be used)
//     let url = Url::parse(&settings.configuration_url)?.join(&settings.contracts.addresses_file)?;
//     if addresses.save(Sources::Http(url)).await.is_err() {
//         warn!("Failed to save the contract addresses to the configuration server. Saving to local file system instead.");
//     }
//     addresses
//         .save(Sources::File(file_path))
//         .await
//         .expect("Failed to save the contract addresses to local file system");
//     Ok(())
// }

/// Function should only be called after we have checked forge is installed by running 'which forge'
pub fn forge_command(command: &[&str]) {
    info!("DEBUG: Running forge command: {command:?}"); // Use info! as forge_command already uses info!
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

#[cfg(test)]
mod tests {
    use super::*;
    use configuration::addresses::get_addresses;
    use std::{fs, path::Path};

    use ethers::{
        core::utils::Anvil,
        providers::{Http, Middleware, Provider},
    };
    use nightfall_bindings::nightfall::NIGHTFALL_DEPLOYED_BYTECODE;
    use tokio::task::spawn_blocking;

    // NB: This test requires Anvil to be installed (it will use Anvil to simulate a blockchain).
    // Restart VS Code after installing Anvil so that it's in your PATH otherwise VS Code won't find it!
    #[tokio::test]
    async fn test_deploy_contracts() {
        // fire up a blockchain simulator
        let mut settings = Settings::new().unwrap();
        std::env::set_var(
            "NF4_SIGNING_KEY",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        );
        settings.ethereum_client_url = "http://localhost:8545".to_string(); // we're running bare metal so a docker url won't work
        let url = Url::parse(&settings.ethereum_client_url).unwrap();
        let anvil = Anvil::new()
            .port(
                url.port()
                    .expect("Could not get Anvil instance. Have you installed it?"),
            )
            .spawn();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        // set the current working directory to be the project root
        let root = "../";
        std::env::set_current_dir(root).unwrap();

        // run the deploy function and get the contract addresses

        deploy_contracts(&settings).await.unwrap();
        // get a blockchain provider so we can interrogate the deployed code
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
        let code = provider
            // use spawn blocking because the blocking reqwest client is not async and it complains (but we need loading the addresses to be sync elsewhere)
            .get_code(
                spawn_blocking(get_addresses).await.unwrap().nightfall(),
                None,
            )
            .await
            .unwrap();
        assert_eq!(code, NIGHTFALL_DEPLOYED_BYTECODE);
        // clean up by remvoing the addresses file and directory that this test created
        fs::remove_dir_all(Path::new("configuration/toml")).unwrap();
    }
}
