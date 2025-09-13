use crate::vk_contract::write_vk_to_nightfall_toml;
use alloy::primitives::Address;
use configuration::{
    addresses::{Addresses, Sources},
    settings::Settings,
};
use log::{info, warn};
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
    forge_command(&["clean"]);
    forge_command(&["build"]);

    if !settings.mock_prover && settings.contracts.deploy_contracts {
        forge_command(&["build", "--force"]);
        let vk = RollupProver::get_decider_vk();
        let _ = write_vk_to_nightfall_toml(&vk);
    }

    forge_command(&[
        "script",
        "Deployer",
        "--fork-url",
        &settings.ethereum_client_url,
        "--broadcast",
        "--force",
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
        if let Some(a) = proxy_map.get("nightfall") {
            addresses.nightfall = *a;
        }
        if let Some(a) = proxy_map.get("round_robin") {
            addresses.round_robin = *a;
        }
        if let Some(a) = proxy_map.get("x509") {
            addresses.x509 = *a;
        }
    }

    // -------- save to config server + local fallback --------
    let search = |path: &Path| -> Option<PathBuf> {
        if path.is_absolute() && path.is_file() {
            return Some(path.to_path_buf());
        }
        let mut cwd = std::env::current_dir().ok()?;
        loop {
            let candidate = cwd.join(path);
            if candidate.is_file() {
                return Some(candidate);
            }
            cwd = cwd.parent()?.to_path_buf();
        }
    };

    let url =
        url::Url::parse(&settings.configuration_url)?.join(&settings.contracts.addresses_file)?;
    if addresses.save(Sources::Http(url)).await.is_err() {
        warn!("Failed to save to configuration server. Saving locally instead.");
    }
    let file_path = search(Path::new(&settings.contracts.addresses_file))
        .unwrap_or_else(|| PathBuf::from(&settings.contracts.addresses_file));
    addresses
        .save(Sources::File(file_path))
        .await
        .expect("Failed to save addresses locally");

    Ok(())
}

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
    use alloy::providers::{Provider, ProviderBuilder};
    use alloy_node_bindings::Anvil;
    use configuration::addresses::get_addresses;
    use nightfall_bindings::artifacts::Nightfall;
    use std::{fs, path::Path};
    use tokio::task::spawn_blocking;
    use url::Url;

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
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_http(anvil.endpoint_url());

        let code = provider
            // use spawn blocking because the blocking reqwest client is not async and it complains (but we need loading the addresses to be sync elsewhere)
            .get_code_at(spawn_blocking(get_addresses).await.unwrap().nightfall())
            .await
            .unwrap();
        assert_eq!(code, Nightfall::DEPLOYED_BYTECODE);
        // clean up by remvoing the addresses file and directory that this test created
        fs::remove_dir_all(Path::new("configuration/toml")).unwrap();
    }
}
