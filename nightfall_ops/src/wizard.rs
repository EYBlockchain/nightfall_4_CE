use std::process::Command;

use inquire::{Confirm, Password, Select, Text};

use crate::{checks, compose, config, model::DeploymentConfig, validation};

pub fn deploy() -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
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

    let deployment = collect_inputs()?;
    print_review(&deployment);

    if !Confirm::new("Apply these changes and generate deployment config?")
        .with_default(false)
        .prompt()
        .map_err(|err| err.to_string())?
    {
        return Err("Deployment cancelled before writing files.".to_string());
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

fn collect_inputs() -> Result<DeploymentConfig, String> {
    let profile = prompt_profile()?;
    let rpc_url = prompt_rpc_url()?;

    println!("Reading chain ID and current block from RPC...");
    let chain_id = cast_rpc_u64("chain-id", &rpc_url)?;
    let genesis_block = cast_rpc_u64("block-number", &rpc_url)?;

    let configuration_port = prompt_port("Configuration service port", 8080)?;
    let configuration_url = prompt_configuration_url(configuration_port)?;
    let default_proposer_url = default_proposer_url(&configuration_url);

    let deployer_signing_key = Password::new("Deployer private key")
        .without_confirmation()
        .prompt()
        .map_err(|err| err.to_string())?;
    let deployer_address = cast_wallet_address(&deployer_signing_key)?;
    println!("Derived deployer address: {deployer_address}");

    let default_proposer_address = Text::new("Default proposer address")
        .with_default(&deployer_address)
        .prompt()
        .map_err(|err| err.to_string())?;
    let default_proposer_url = Text::new("Default proposer public URL")
        .with_default(&default_proposer_url)
        .prompt()
        .map_err(|err| err.to_string())?;

    let real_prover = Confirm::new("Use real prover mode?")
        .with_default(true)
        .prompt()
        .map_err(|err| err.to_string())?;
    if real_prover {
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
            .ok_or_else(|| "Deployment cancelled before real prover setup.".to_string())?;
    }

    let block_size = Select::new("Block size", vec![64_u64, 256_u64])
        .with_starting_cursor(0)
        .prompt()
        .map_err(|err| err.to_string())?;

    Ok(DeploymentConfig {
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
        mock_prover: !real_prover,
        block_size,
    })
}

fn prompt_profile() -> Result<String, String> {
    let profile = Text::new("Profile name")
        .with_default("sepolia")
        .prompt()
        .map_err(|err| err.to_string())?;

    if profile
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(profile)
    } else {
        Err("Profile name must use lowercase letters, digits, or underscores.".to_string())
    }
}

fn prompt_rpc_url() -> Result<String, String> {
    let rpc_url = Text::new("Host-chain WebSocket RPC URL")
        .prompt()
        .map_err(|err| err.to_string())?;

    if rpc_url.starts_with("ws://") || rpc_url.starts_with("wss://") {
        Ok(rpc_url)
    } else {
        Err("RPC URL must start with ws:// or wss://.".to_string())
    }
}

fn prompt_port(label: &str, default: u16) -> Result<u16, String> {
    let value = Text::new(label)
        .with_default(&default.to_string())
        .prompt()
        .map_err(|err| err.to_string())?;

    value
        .parse::<u16>()
        .map_err(|_| format!("{label} must be a port number between 1 and 65535."))
}

fn prompt_configuration_url(port: u16) -> Result<String, String> {
    let default = format!("http://127.0.0.1:{port}");
    let url = Text::new("Configuration service URL, including port")
        .with_default(&default)
        .prompt()
        .map_err(|err| err.to_string())?;

    if (url.starts_with("http://") || url.starts_with("https://")) && url.contains(':') {
        Ok(url)
    } else {
        Err("Configuration URL must include http:// or https:// and the exposed port.".to_string())
    }
}

fn cast_rpc_u64(command: &str, rpc_url: &str) -> Result<u64, String> {
    let output = Command::new("cast")
        .args([command, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("Failed to run cast {command}: {err}"))?;

    if !output.status.success() {
        return Err(command_error(&format!("cast {command}"), &output));
    }

    first_line(&output.stdout)?
        .parse::<u64>()
        .map_err(|err| format!("Failed to parse cast {command} output: {err}"))
}

fn cast_wallet_address(private_key: &str) -> Result<String, String> {
    let output = Command::new("cast")
        .args(["wallet", "address", "--private-key", private_key])
        .output()
        .map_err(|err| format!("Failed to derive deployer address with cast: {err}"))?;

    if !output.status.success() {
        return Err(command_error("cast wallet address", &output));
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

fn command_error(command: &str, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let detail = stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("no output");
    format!("{command} failed: {detail}")
}

fn default_proposer_url(configuration_url: &str) -> String {
    let Some((scheme, rest)) = configuration_url.split_once("://") else {
        return "http://127.0.0.1:3001".to_string();
    };
    let host = rest
        .split('/')
        .next()
        .unwrap_or(rest)
        .split(':')
        .next()
        .unwrap_or("127.0.0.1");
    format!("{scheme}://{host}:3001")
}

fn print_review(config: &DeploymentConfig) {
    println!();
    println!("Review deployment settings");
    println!("  profile: {}", config.profile);
    println!("  rpc_url: {}", config.rpc_url);
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
    run_command(
        "Cleaning contract build artifacts",
        "forge",
        &["clean"],
        &[],
    )?;
    run_command("Building contracts", "forge", &["build"], &[])?;

    if !config.mock_prover {
        run_command(
            "Generating real proving keys",
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
                ("NF4_MOCK_PROVER", "false"),
            ],
        )?;
    }

    compose::build_indie_deployer()?;
    compose::up_indie_deployer()?;
    compose::build_configuration()?;
    compose::up_configuration()?;
    validation::configuration_endpoints(&config.configuration_url)?;

    println!();
    println!("Deployment OK");
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
    use super::default_proposer_url;

    #[test]
    fn derives_default_proposer_url_from_configuration_url() {
        assert_eq!(
            default_proposer_url("http://10.0.0.5:8080"),
            "http://10.0.0.5:3001"
        );
        assert_eq!(
            default_proposer_url("https://config.example.com:8080"),
            "https://config.example.com:3001"
        );
    }
}
