use crate::vk_contract::write_vk_to_nightfall_toml;
use alloy::{
    hex,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use configuration::{
    addresses::{Addresses, Sources},
    settings::Settings,
};
use jf_plonk::recursion::RecursiveProver;

use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, error, info, warn};
use nightfall_proposer::driven::rollup_prover::RollupProver;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::File,
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

    // Containers run as root and bind-mount this directory. Make it writable by
    // the host user before and after forge creates chain-id folders.
    share_broadcast_logs_with_host(Path::new("blockchain_assets/logs"));

    // Force a clean rebuild to generate proper build-info files for OpenZeppelin validation
    info!("Building contracts with forge");
    forge_command(&["build", "--force"]);

    // If there is an existing broadcast log for this chain, clean it before fresh broadcast
    // so forge script does not attempt to resume or collide with old broadcast transactions.
    // Resume would also be wrong after a nonce drift: it resends the old nonce instead of
    // re-simulating.
    let cwd = std::env::current_dir()?;
    let chain_logs = cwd
        .join(&settings.contracts.deployment_file)
        .join(settings.network.chain_id.to_string());

    // Forge's alloy WS transport can drop/retry the same nonce when a provider sends a
    // malformed JSON-RPC message (`missing field params`), which surfaces as
    // `replacement transaction underpriced`. Broadcast over HTTP instead.
    let broadcast_rpc_url = http_rpc_url_for_broadcast(&settings.ethereum_client_url);
    if broadcast_rpc_url != settings.ethereum_client_url {
        info!("Using HTTP RPC for forge broadcast: {broadcast_rpc_url}");
    }

    // `--slow` makes forge compare each script nonce with eth_getTransactionCount before
    // sending, and abort if the chain is ahead (`Expected 50 got 51`). That happens when a
    // pending tx confirms during simulation, or another process spends the deployer key.
    // Wait until pending == latest, then retry a fresh simulation if it still drifts.
    const BROADCAST_ATTEMPTS: u32 = 3;
    for attempt in 1..=BROADCAST_ATTEMPTS {
        clear_broadcast_logs(&chain_logs);
        wait_for_stable_deployer_nonce(&broadcast_rpc_url, &settings.signing_key).await?;
        info!("Deploying contracts with forge script (attempt {attempt}/{BROADCAST_ATTEMPTS})");
        match try_forge_command(&[
            "script",
            "Deployer",
            "--fork-url",
            &broadcast_rpc_url,
            "--broadcast",
            "--slow",
        ]) {
            Ok(()) => break,
            Err(err) if is_recoverable_broadcast_failure(&err) && attempt < BROADCAST_ATTEMPTS => {
                warn!(
                    "Forge broadcast hit a deployer nonce race ({}); waiting and retrying with a fresh simulation",
                    nonce_drift_summary(&err)
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Err(err) => return Err(err.into()),
        }
    }

    // -------- read Foundry broadcast --------
    let path_out = chain_logs.join("run-latest.json");

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
    share_broadcast_logs_with_host(Path::new("blockchain_assets/logs"));

    Ok(())
}

fn share_broadcast_logs_with_host(logs: &Path) {
    if let Err(err) = relax_broadcast_tree(logs) {
        warn!(
            "Could not make {} writable for the host user: {err}",
            logs.display()
        );
    }
}

fn relax_broadcast_tree(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    let mut stack = vec![path.to_path_buf()];
    while let Some(current) = stack.pop() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = if current.is_dir() { 0o777 } else { 0o666 };
            std::fs::set_permissions(&current, std::fs::Permissions::from_mode(mode))?;
        }
        if current.is_dir() {
            for entry in std::fs::read_dir(&current)? {
                stack.push(entry?.path());
            }
        }
    }
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

/// Convert a WebSocket RPC URL to HTTP for Foundry broadcast.
/// Nightfall itself still uses ws/wss for event subscriptions.
fn http_rpc_url_for_broadcast(url: &str) -> String {
    let scheme_end = url.find("://").map(|i| i + 3).unwrap_or(0);
    let scheme = url[..scheme_end.min(url.len())].to_ascii_lowercase();
    if scheme == "wss://" {
        format!("https://{}", &url[scheme_end..])
    } else if scheme == "ws://" {
        format!("http://{}", &url[scheme_end..])
    } else {
        url.to_string()
    }
}

fn clear_broadcast_logs(chain_logs: &Path) {
    if chain_logs.exists() {
        info!("Removing stale broadcast logs from {chain_logs:?}");
        std::fs::remove_dir_all(chain_logs).ok();
    }
}

/// Block until the deployer has no in-flight transactions, so forge's nonce snapshot is not
/// already stale. RPCs that do not support the `pending` tag are treated as stable.
async fn wait_for_stable_deployer_nonce(
    rpc_url: &str,
    signing_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = signing_key
        .parse()
        .map_err(|e| format!("Invalid deployer signing key: {e}"))?;
    let url = url::Url::parse(rpc_url)?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_http(url);
    let address = signer.address();

    for attempt in 1..=10 {
        let latest = provider
            .get_transaction_count(address)
            .latest()
            .await
            .map_err(|e| format!("Failed to read latest deployer nonce: {e}"))?;
        match provider.get_transaction_count(address).pending().await {
            Ok(pending) if pending <= latest => {
                info!("Deployer {address} nonce is stable at {latest}");
                return Ok(());
            }
            Ok(pending) => {
                warn!(
                    "Deployer {address} has in-flight transactions (latest nonce {latest}, pending nonce {pending}); waiting before broadcast ({attempt}/10)"
                );
            }
            Err(err) => {
                warn!(
                    "Pending nonce unavailable for {address} ({err}); continuing with latest nonce {latest}"
                );
                return Ok(());
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Err(format!(
        "Deployer {address} still has pending transactions. Stop any other process using this key and retry."
    )
    .into())
}

fn is_recoverable_broadcast_failure(output: &str) -> bool {
    output.contains("EOA nonce changed unexpectedly")
        || output.contains("replacement transaction underpriced")
        || output.contains("nonce too low")
}

fn nonce_drift_summary(output: &str) -> String {
    output
        .lines()
        .find(|line| {
            line.contains("EOA nonce changed unexpectedly")
                || line.contains("replacement transaction underpriced")
                || line.contains("nonce too low")
        })
        .unwrap_or("deployer nonce changed during broadcast")
        .trim()
        .to_string()
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
pub fn forge_command(command: &[&str]) {
    if let Err(err) = try_forge_command(command) {
        panic!("{err}");
    }
}

fn try_forge_command(command: &[&str]) -> Result<(), String> {
    debug!("DEBUG: Running forge command: {command:?}");
    let output = match std::process::Command::new("forge").args(command).output() {
        Ok(output) => output,
        Err(e) => {
            return Err(format!(
                "Command 'forge {command:?}' ran into an error without executing: {e}"
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        info!(
            "Command 'forge {:?}' executed successfully: {stdout}",
            command
        );
        Ok(())
    } else {
        Err(format!(
            "Command 'forge {:?}' failed with status {}:\nStandard Output: {stdout}\nStandard Error: {stderr}",
            command, output.status
        ))
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

#[cfg(test)]
mod tests {
    use super::{
        http_rpc_url_for_broadcast, is_recoverable_broadcast_failure, nonce_drift_summary,
        relax_broadcast_tree,
    };
    use std::fs;

    #[test]
    fn converts_websocket_rpc_urls_to_http_for_broadcast() {
        assert_eq!(
            http_rpc_url_for_broadcast("wss://eth-sepolia.example/v2/key"),
            "https://eth-sepolia.example/v2/key"
        );
        assert_eq!(
            http_rpc_url_for_broadcast("ws://anvil:8545"),
            "http://anvil:8545"
        );
        assert_eq!(
            http_rpc_url_for_broadcast("WSS://rpc.example"),
            "https://rpc.example"
        );
        assert_eq!(
            http_rpc_url_for_broadcast("https://rpc.example"),
            "https://rpc.example"
        );
        assert_eq!(
            http_rpc_url_for_broadcast("http://127.0.0.1:8545"),
            "http://127.0.0.1:8545"
        );
    }

    #[test]
    fn retries_only_nonce_races() {
        let drift = "Error: Failed to send transaction\n\nContext:\n- EOA nonce changed unexpectedly while sending transactions. Expected 50 got 51 from provider.\n";
        assert!(is_recoverable_broadcast_failure(drift));
        assert!(nonce_drift_summary(drift).contains("Expected 50 got 51"));
        assert!(is_recoverable_broadcast_failure(
            "server returned an error response: nonce too low"
        ));
        assert!(is_recoverable_broadcast_failure(
            "replacement transaction underpriced"
        ));
        assert!(!is_recoverable_broadcast_failure(
            "Error: script failed: execution reverted"
        ));
    }

    #[cfg(unix)]
    #[test]
    fn makes_broadcast_tree_writable_by_others() {
        use std::os::unix::fs::PermissionsExt;

        let root = std::env::temp_dir().join(format!(
            "nf4-relax-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let logs = root.join("logs");
        let chain = logs.join("mock_deployment.s.sol").join("11155111");
        fs::create_dir_all(&chain).unwrap();
        fs::write(chain.join("run-latest.json"), b"{}").unwrap();
        fs::set_permissions(&chain, fs::Permissions::from_mode(0o755)).unwrap();

        relax_broadcast_tree(&logs).unwrap();

        let mode = fs::metadata(&chain).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o777);
        let file_mode = fs::metadata(chain.join("run-latest.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(file_mode, 0o666);
        let _ = fs::remove_dir_all(&root);
    }
}
