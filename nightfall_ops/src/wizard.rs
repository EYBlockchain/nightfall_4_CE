use std::process::Command;

use inquire::{Confirm, Select, Text};

use crate::{
    checks, compose, config,
    model::DeploymentConfig,
    network::{
        ChainIdMatch, LOCAL_ANVIL_ACCOUNT0_ADDRESS, LOCAL_ANVIL_ACCOUNT0_KEY, NetworkKind,
        chain_id_description, command_failure_detail, compose_rpc_url, explain_rpc_error,
        parse_u64_output, validate_profile_name, validate_published_configuration_url,
        validate_rpc_scheme,
    },
    validation,
};

pub fn deploy(selected_network: Option<NetworkKind>, yes: bool) -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }
    if yes {
        let network = selected_network.unwrap_or(NetworkKind::Local);
        if network != NetworkKind::Local {
            return Err(
                "--yes is only supported with --network local. Testnet/mainnet need explicit review."
                    .to_string(),
            );
        }
    }

    println!("Nightfall deployment uses a configuration service.");
    println!();
    println!("The configuration service hosts:");
    println!("- deployed contract addresses");
    println!("- deployed contract hashes");
    println!("- proving keys");
    println!();
    checks::deployer()?;
    println!();

    let deployment = if yes {
        collect_local_yes_inputs()?
    } else {
        collect_inputs(selected_network)?
    };
    print_review(&deployment);

    if !yes {
        if deployment.network.requires_typed_confirm() {
            let phrase = deployment
                .network
                .typed_confirm_phrase()
                .unwrap_or("deploy to mainnet");
            let typed = Text::new(&format!("Type `{phrase}` to continue"))
                .prompt()
                .map_err(|err| err.to_string())?;
            if typed.trim() != phrase {
                return Err(
                    "Deployment cancelled: mainnet confirmation phrase did not match.".to_string(),
                );
            }
        }

        if !Confirm::new("Apply these changes and generate deployment config?")
            .with_default(false)
            .prompt()
            .map_err(|err| err.to_string())?
        {
            return Err("Deployment cancelled before writing files.".to_string());
        }
    } else {
        println!("Applying local --yes defaults without prompts.");
    }

    let backup_dir = config::backup_config_files()?;
    println!("Backed up config files to {}", backup_dir.display());
    config::update_nightfall_toml(&deployment)?;
    config::update_docker_compose(&deployment)?;
    config::write_local_env(&deployment)?;

    println!();
    println!("Deployment configuration generated.");
    run_deployment(&deployment)?;
    Ok(())
}

fn collect_local_yes_inputs() -> Result<DeploymentConfig, String> {
    let network = NetworkKind::Local;
    let profile = network.default_profile().to_string();
    validate_profile_name(&profile)?;
    let rpc_url = network
        .default_rpc_url()
        .ok_or_else(|| "Local RPC default is missing.".to_string())?
        .to_string();
    validate_rpc_scheme(network, &rpc_url)?;

    println!("Using local --yes defaults (Anvil account 0, mock prover, docker proposer URL).");
    println!("Reading chain ID and current block from RPC...");
    let chain_id = cast_rpc_u64("chain-id", &rpc_url)
        .map_err(|err| explain_rpc_error(Some(network), &rpc_url, &err))?;
    let genesis_block = cast_rpc_u64("block-number", &rpc_url)
        .map_err(|err| explain_rpc_error(Some(network), &rpc_url, &err))?;
    if chain_id != 31337 {
        return Err(format!(
            "--yes requires Anvil chain ID 31337, got {}.",
            chain_id_description(chain_id)
        ));
    }
    ensure_chain_matches(network, Some(31337), chain_id)?;

    let configuration_port = 8080;
    let configuration_url = default_configuration_url(network, configuration_port);
    validate_published_configuration_url(network, &configuration_url)?;
    let default_proposer_url = default_proposer_url(network, &configuration_url);
    let deployer_signing_key = LOCAL_ANVIL_ACCOUNT0_KEY.to_string();
    let deployer_address = cast_wallet_address(&deployer_signing_key)?;
    if !deployer_address.eq_ignore_ascii_case(LOCAL_ANVIL_ACCOUNT0_ADDRESS) {
        return Err(format!(
            "--yes expected Anvil account 0 ({LOCAL_ANVIL_ACCOUNT0_ADDRESS}), got {deployer_address}"
        ));
    }
    println!("Derived deployer address: {deployer_address}");

    Ok(DeploymentConfig {
        network,
        profile,
        rpc_url,
        chain_id,
        genesis_block,
        configuration_url,
        configuration_port,
        deployer_signing_key,
        default_proposer_address: deployer_address.clone(),
        deployer_address,
        default_proposer_url,
        mock_prover: true,
        block_size: 64,
    })
}

fn collect_inputs(selected_network: Option<NetworkKind>) -> Result<DeploymentConfig, String> {
    let network = prompt_network(selected_network)?;
    let expected_chain_id = prompt_expected_chain_id(network)?;
    let profile = prompt_profile(network.default_profile())?;
    let rpc_url = prompt_rpc_url(network)?;

    println!("Reading chain ID and current block from RPC...");
    let chain_id = cast_rpc_u64("chain-id", &rpc_url)
        .map_err(|err| explain_rpc_error(Some(network), &rpc_url, &err))?;
    let genesis_block = cast_rpc_u64("block-number", &rpc_url)
        .map_err(|err| explain_rpc_error(Some(network), &rpc_url, &err))?;
    ensure_chain_matches(network, expected_chain_id, chain_id)?;

    let configuration_port = prompt_port("Configuration service port", 8080)?;
    let configuration_url = prompt_configuration_url(network, configuration_port)?;
    let default_proposer_url = default_proposer_url(network, &configuration_url);

    let deployer_signing_key =
        crate::network::prompt_l1_signing_key("Deployer", network, None, &rpc_url)?;
    let deployer_address = cast_wallet_address(&deployer_signing_key)?;
    println!("Derived deployer address: {deployer_address}");

    if network == NetworkKind::Mainnet {
        println!();
        println!("Mainnet: the default proposer address owns the registered proposer slot.");
        println!("Press Enter only if you intend the deployer to be that proposer.");
    }
    let default_proposer_address =
        Text::new("Default proposer address [press Enter to use deployer address]")
            .with_default(&deployer_address)
            .prompt()
            .map_err(|err| err.to_string())?;
    if network == NetworkKind::Local {
        println!();
        println!("Local transfers use the on-chain proposer URL, not NF4_NIGHTFALL_PROPOSER__URL.");
        println!(
            "Register the docker service name so a laptop LAN/VPN change cannot break transfers."
        );
        println!("Host health checks stay http://127.0.0.1:3001.");
    }
    let proposer_url_prompt = if network == NetworkKind::Local {
        "Default proposer URL [press Enter to use docker service http://indie-proposer:3000]"
    } else {
        "Default proposer public URL [press Enter to use detected host with port 3001]"
    };
    let default_proposer_url = Text::new(proposer_url_prompt)
        .with_default(&default_proposer_url)
        .prompt()
        .map_err(|err| err.to_string())?;

    let mock_prover = prompt_mock_prover(network)?;

    let block_size = Select::new("Block size", vec![64_u64, 256_u64])
        .with_starting_cursor(0)
        .prompt()
        .map_err(|err| err.to_string())?;

    Ok(DeploymentConfig {
        network,
        profile,
        rpc_url,
        chain_id,
        genesis_block,
        configuration_url,
        configuration_port,
        deployer_signing_key,
        deployer_address,
        default_proposer_address,
        default_proposer_url,
        mock_prover,
        block_size,
    })
}

fn prompt_network(selected: Option<NetworkKind>) -> Result<NetworkKind, String> {
    if let Some(network) = selected {
        println!("Network: {} (--network)", network.as_str());
        return Ok(network);
    }

    let options = vec!["local", "testnet", "mainnet"];
    let choice = Select::new("Network", options)
        .with_starting_cursor(1)
        .prompt()
        .map_err(|err| err.to_string())?;
    NetworkKind::parse(choice)
}

fn prompt_expected_chain_id(network: NetworkKind) -> Result<Option<u64>, String> {
    if network != NetworkKind::Testnet {
        return Ok(None);
    }

    let options = vec!["sepolia", "base_sepolia", "other"];
    let choice = Select::new("Testnet chain", options)
        .with_starting_cursor(0)
        .prompt()
        .map_err(|err| err.to_string())?;
    Ok(match choice {
        "sepolia" => Some(11155111),
        "base_sepolia" => Some(84532),
        _ => None,
    })
}

fn prompt_profile(default: &str) -> Result<String, String> {
    let prompt = format!("Profile name [default: {default}, press Enter to use default]");
    let profile = Text::new(&prompt)
        .with_default(default)
        .prompt()
        .map_err(|err| err.to_string())?;
    validate_profile_name(&profile)?;
    Ok(profile)
}

fn prompt_rpc_url(network: NetworkKind) -> Result<String, String> {
    let rpc_url = if let Some(default) = network.default_rpc_url() {
        Text::new("Host-chain WebSocket RPC URL [press Enter to use local Anvil default]")
            .with_default(default)
            .prompt()
            .map_err(|err| err.to_string())?
    } else {
        Text::new("Host-chain WebSocket RPC URL")
            .prompt()
            .map_err(|err| err.to_string())?
    };

    validate_rpc_scheme(network, &rpc_url)?;
    Ok(rpc_url)
}

fn ensure_chain_matches(
    network: NetworkKind,
    expected_chain_id: Option<u64>,
    chain_id: u64,
) -> Result<(), String> {
    if let Some(expected) = expected_chain_id {
        if chain_id != expected {
            return Err(format!(
                "You chose {}, but the RPC chain ID is {}. Aborting.",
                chain_id_description(expected),
                chain_id_description(chain_id)
            ));
        }
    }

    match network.classify_chain_id(chain_id) {
        ChainIdMatch::Known => Ok(()),
        ChainIdMatch::Mismatch => Err(network.mismatch_message(chain_id)),
        ChainIdMatch::Unknown => {
            let confirmed = Confirm::new(&format!(
                "RPC chain ID {} is not a known {} ID. Continue?",
                chain_id_description(chain_id),
                network.as_str()
            ))
            .with_default(false)
            .prompt()
            .map_err(|err| err.to_string())?;
            if confirmed {
                Ok(())
            } else {
                Err("Deployment cancelled because the chain ID was not recognized.".to_string())
            }
        }
    }
}

fn prompt_mock_prover(network: NetworkKind) -> Result<bool, String> {
    if !network.allows_mock_prover() {
        println!();
        println!("Mainnet requires real prover mode.");
        confirm_real_prover()?;
        return Ok(false);
    }

    let real_prover = Confirm::new("Use real prover mode?")
        .with_default(!network.default_mock_prover())
        .prompt()
        .map_err(|err| err.to_string())?;
    if real_prover {
        confirm_real_prover()?;
        Ok(false)
    } else {
        Ok(true)
    }
}

fn confirm_real_prover() -> Result<(), String> {
    println!();
    println!("Real prover mode is expensive.");
    println!("Key generation can take a long time and requires large RAM/disk.");
    println!("Generating keys successfully does not prove this machine can prove a block.");
    println!("Run nf4 check prover to run the pinned Nightfish recursive prover test.");
    Confirm::new("Continue with real prover key generation?")
        .with_default(false)
        .prompt()
        .map_err(|err| err.to_string())?
        .then_some(())
        .ok_or_else(|| "Deployment cancelled before real prover setup.".to_string())
}

fn prompt_port(label: &str, default: u16) -> Result<u16, String> {
    let prompt = format!("{label} [default: {default}, press Enter to use default]");
    let value = Text::new(&prompt)
        .with_default(&default.to_string())
        .prompt()
        .map_err(|err| err.to_string())?;

    let port = value
        .parse::<u16>()
        .map_err(|_| format!("{label} must be a port number between 1 and 65535."))?;
    if port == 0 {
        return Err(format!(
            "{label} must be a port number between 1 and 65535."
        ));
    }
    Ok(port)
}

fn prompt_configuration_url(network: NetworkKind, port: u16) -> Result<String, String> {
    let default = default_configuration_url(network, port);
    let url = if default.is_empty() {
        Text::new("Public configuration service URL, including https://")
            .prompt()
            .map_err(|err| err.to_string())?
    } else {
        Text::new("Configuration service URL, including port [press Enter to use detected default]")
            .with_default(&default)
            .prompt()
            .map_err(|err| err.to_string())?
    };

    validate_published_configuration_url(network, &url)?;
    Ok(url)
}

fn default_configuration_url(network: NetworkKind, port: u16) -> String {
    match network {
        NetworkKind::Local | NetworkKind::Testnet => {
            format!("http://{}:{port}", crate::network::detect_lan_host())
        }
        NetworkKind::Mainnet => String::new(),
    }
}

fn cast_rpc_u64(command: &str, rpc_url: &str) -> Result<u64, String> {
    let output = Command::new("cast")
        .args([command, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("Failed to run cast {command}: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "cast {command} failed: {}",
            command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }

    parse_u64_output(&output.stdout).map_err(|err| format!("Failed to parse cast {command}: {err}"))
}

fn cast_wallet_address(private_key: &str) -> Result<String, String> {
    let output = Command::new("cast")
        .args(["wallet", "address", "--private-key", private_key])
        .output()
        .map_err(|err| format!("Failed to derive deployer address with cast: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "cast wallet address failed: {}",
            command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }

    first_line(&output.stdout)
}

fn first_line(bytes: &[u8]) -> Result<String, String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
        .ok_or_else(|| "Command returned no output.".to_string())
}

fn default_proposer_url(network: NetworkKind, configuration_url: &str) -> String {
    if network == NetworkKind::Local {
        return "http://indie-proposer:3000".to_string();
    }
    let Some((scheme, rest)) = configuration_url.split_once("://") else {
        return format!("http://{}:3001", detected_host());
    };
    let host = rest
        .split('/')
        .next()
        .unwrap_or(rest)
        .split(':')
        .next()
        .filter(|host| !host.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(detected_host);
    format!("{scheme}://{host}:3001")
}

fn detected_host() -> String {
    hostname_ip()
        .or_else(|| ipconfig_ip("en0"))
        .or_else(|| ipconfig_ip("en1"))
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn hostname_ip() -> Option<String> {
    let output = Command::new("hostname").arg("-I").output().ok()?;
    if !output.status.success() {
        return None;
    }
    host_from_hostname_output(&String::from_utf8_lossy(&output.stdout))
}

fn host_from_hostname_output(output: &str) -> Option<String> {
    output
        .split_whitespace()
        .find(|ip| !ip.starts_with("127.") && !ip.starts_with("172."))
        .map(ToString::to_string)
}

fn ipconfig_ip(interface: &str) -> Option<String> {
    let output = Command::new("ipconfig")
        .args(["getifaddr", interface])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    first_line(&output.stdout)
        .ok()
        .filter(|ip| !ip.starts_with("127."))
}

fn print_review(config: &DeploymentConfig) {
    println!();
    println!("Review deployment settings");
    println!("  network: {}", config.network.as_str());
    println!("  profile: {}", config.profile);
    println!("  rpc_url: {}", config.rpc_url);
    let compose_rpc = compose_rpc_url(config.network, &config.rpc_url);
    if compose_rpc != config.rpc_url {
        println!("  compose_rpc_url: {compose_rpc}");
    }
    println!("  chain_id: {}", config.chain_id);
    println!("  genesis_block: {}", config.genesis_block);
    println!("  configuration_url: {}", config.configuration_url);
    println!("  configuration_port: {}", config.configuration_port);
    println!("  deployer_address: {}", config.deployer_address);
    println!(
        "  default_proposer_address: {}",
        config.default_proposer_address
    );
    println!("  default_proposer_url: {}", config.default_proposer_url);
    println!("  prover_mode: {}", config.prover_mode());
    println!("  block_size: {}", config.block_size);
}

fn run_deployment(config: &DeploymentConfig) -> Result<(), String> {
    if config.network != NetworkKind::Local {
        println!("Checking deployer account mempool on RPC...");
        crate::network::verify_clean_mempool(&config.rpc_url, &config.deployer_address)?;
    }

    // Clean stale Foundry broadcast and cache files so deployer doesn't resubmit old nonces
    let stale_chain_logs = format!("blockchain_assets/logs/deployer.s.sol/{}", config.chain_id);
    let _ = std::fs::remove_dir_all(&stale_chain_logs);
    let _ = std::fs::remove_dir_all("blockchain_assets/cache");

    run_command(
        "Cleaning contract build artifacts",
        "forge",
        &["clean"],
        &[],
    )?;
    run_command("Building contracts", "forge", &["build"], &[])?;

    let key_generation_title = if config.mock_prover {
        "Generating mock proving key"
    } else {
        "Generating real proving keys"
    };
    let mock_prover = config.mock_prover.to_string();
    run_command(
        key_generation_title,
        "cargo",
        &[
            "run",
            "--release",
            "-p",
            "nightfall_deployer",
            "--bin",
            "key_generation",
        ],
        &[
            ("NF4_RUN_MODE", config.profile.as_str()),
            ("NF4_MOCK_PROVER", mock_prover.as_str()),
        ],
    )?;

    compose::build_indie_deployer()?;
    compose::up_indie_deployer()?;
    compose::build_configuration()?;
    compose::up_configuration()?;
    validation::configuration_endpoints(&config.configuration_url)?;

    println!();
    println!("Deployment OK");
    println!("  network: {}", config.network.as_str());
    println!("  profile: {}", config.profile);
    println!("  chain_id: {}", config.chain_id);
    println!("  configuration_url: {}", config.configuration_url);
    Ok(())
}

fn run_command(
    title: &str,
    program: &str,
    args: &[&str],
    envs: &[(&str, &str)],
) -> Result<(), String> {
    println!("{title}...");
    println!("  {program} {}", args.join(" "));

    let mut command = Command::new(program);
    command.args(args);
    for (key, value) in envs {
        command.env(key, value);
    }

    let status = command
        .status()
        .map_err(|err| format!("Failed to run {program}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{default_configuration_url, default_proposer_url, host_from_hostname_output};
    use crate::network::NetworkKind;

    #[test]
    fn derives_default_proposer_url_from_configuration_url() {
        assert_eq!(
            default_proposer_url(NetworkKind::Local, "http://10.0.0.5:8080"),
            "http://indie-proposer:3000"
        );
        assert_eq!(
            default_proposer_url(NetworkKind::Testnet, "http://10.0.0.5:8080"),
            "http://10.0.0.5:3001"
        );
        assert_eq!(
            default_proposer_url(NetworkKind::Testnet, "https://config.example.com:8080"),
            "https://config.example.com:3001"
        );
    }

    #[test]
    fn selects_lan_ip_from_hostname_output() {
        assert_eq!(
            host_from_hostname_output("10.0.0.8 172.17.0.1 172.18.0.1"),
            Some("10.0.0.8".to_string())
        );
        assert_eq!(host_from_hostname_output("127.0.0.1 172.17.0.1"), None);
    }

    #[test]
    fn configuration_url_default_depends_on_network() {
        let local = default_configuration_url(NetworkKind::Local, 8080);
        assert!(local.starts_with("http://"));
        assert!(local.ends_with(":8080"));
        assert!(!local.contains("host.docker.internal"));
        assert!(!local.contains("configuration"));
        assert!(default_configuration_url(NetworkKind::Mainnet, 8080).is_empty());
    }
}
