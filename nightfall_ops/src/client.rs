use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

use inquire::{Confirm, Password, Text};
use toml_edit::{DocumentMut, Item};

use crate::{compose, config, validation, webhook};

const ADDRESSES_TOML: &str = "configuration/toml/addresses.toml";
const CONTRACT_HASHES_TOML: &str = "configuration/toml/contract_hashes.toml";
const PROVING_KEY: &str = "configuration/bin/keys/proving_key";
const MOCK_DEPLOYER_SCRIPT: &str = "blockchain_assets/script/mock_deployment.s.sol:MockDeployer";

pub fn wizard() -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }
    if !config::local_env_exists() {
        return Err("local.env was not found. Run nf4 wizard deploy first.".to_string());
    }

    println!("This command starts an indie client node for the existing local deployment.");
    println!();
    println!("It reuses deployment metadata, proposer URL, and the proving key.");
    println!("You only need to confirm client-specific values.");
    println!();

    check_prerequisites()?;

    let env = config::read_local_env();
    let profile = env_value(&env, "NF4_RUN_MODE").unwrap_or_else(|| "development".to_string());
    let profile_config = read_profile_config(&profile)?;
    let rpc_url = env_value(&env, "NF4_ETHEREUM_CLIENT_URL")
        .or_else(|| profile_config.ethereum_client_url.clone())
        .ok_or_else(|| {
            "NF4_ETHEREUM_CLIENT_URL was not found in local.env or nightfall.toml.".to_string()
        })?;
    let mock_prover = env_value(&env, "NF4_MOCK_PROVER")
        .and_then(|value| parse_bool(&value))
        .or(profile_config.mock_prover)
        .unwrap_or(false);

    println!("Checking client deployment metadata...");
    validate_metadata(mock_prover)?;
    let nightfall_address = read_addresses()?
        .get("nightfall")
        .cloned()
        .ok_or_else(|| format!("{ADDRESSES_TOML} is missing nightfall"))?;

    let client_key = prompt_client_key(env.get("CLIENT_SIGNING_KEY"))?;
    let derived_client_address = cast_wallet_address(&client_key)?;
    let client_address = prompt_client_address(env.get("CLIENT_ADDRESS"), &derived_client_address)?;
    let client2_address = env_value(&env, "CLIENT2_ADDRESS")
        .filter(|address| is_non_zero_address(address))
        .unwrap_or_else(|| client_address.clone());

    let proposer_url_default = default_proposer_url(&env, &profile_config);
    let proposer_url = prompt_url(
        "Proposer URL [press Enter to use default]",
        &proposer_url_default,
    )?;
    validate_proposer_health(&proposer_url)?;

    let configuration_url_default = env_value(&env, "NF4_CONFIGURATION_URL")
        .or_else(|| profile_config.configuration_url.clone())
        .unwrap_or_else(|| format!("http://{}:8080", detected_host()));
    let configuration_url = prompt_url(
        "Configuration URL for client runtime [press Enter to use default]",
        &configuration_url_default,
    )?;
    validate_configuration_runtime_url(&configuration_url)?;

    let webhook_setup = configure_webhook(&env)?;

    let client_port = prompt_port("Client API port", client_api_port(&env).unwrap_or(3000))?;
    let client_api_url = format!("http://127.0.0.1:{client_port}");

    print_review(&ClientReview {
        profile: &profile,
        rpc_url: &rpc_url,
        mock_prover,
        client_address: &client_address,
        client_api_url: &client_api_url,
        proposer_url: &proposer_url,
        configuration_url: &configuration_url,
        webhook_url: &webhook_setup.url,
        client_port,
    });

    if !Confirm::new("Apply these changes and start indie-client?")
        .with_default(true)
        .prompt()
        .map_err(|err| err.to_string())?
    {
        return Err("Client setup cancelled before writing files.".to_string());
    }

    let backup_dir = config::backup_config_files()?;
    println!("Backed up config files to {}", backup_dir.display());

    config::merge_local_env(&[
        ("CLIENT_SIGNING_KEY", client_key.clone()),
        ("NF4_SIGNING_KEY", client_key),
        ("CLIENT_ADDRESS", client_address.clone()),
        ("CLIENT2_ADDRESS", client2_address.clone()),
        ("NIGHTFALL_ADDRESS", nightfall_address.clone()),
        ("CLIENT_API_URL", client_api_url.clone()),
        ("NF4_CONFIGURATION_URL", configuration_url.clone()),
        ("NF4_NIGHTFALL_PROPOSER__URL", proposer_url.clone()),
        (
            "NF4_NIGHTFALL_CLIENT__WEBHOOK_URL",
            webhook_setup.url.clone(),
        ),
        ("WEBHOOK_URL", webhook_setup.url.clone()),
    ])?;
    config::update_client_docker_compose(&profile, client_port)?;

    run_command("Cleaning contract build artifacts", "forge", &["clean"])?;
    run_command("Building contracts", "forge", &["build"])?;

    compose::build_indie_client()?;
    compose::up_client()?;
    poll_client_health(client_port)?;

    println!();
    println!("Client OK");
    println!("  profile: {profile}");
    println!("  mode: {}", if mock_prover { "mock" } else { "real" });
    println!("  client_address: {client_address}");
    println!("  client2_address: {client2_address}");
    println!("  nightfall_address: {nightfall_address}");
    println!("  client_api_url: {client_api_url}");
    println!("  proposer_url: {proposer_url}");
    println!("  configuration_runtime_url: {configuration_url}");
    println!("  webhook_url: {}", webhook_setup.url);
    if let Some(events_path) = webhook_setup.events_path {
        println!("  webhook_events: {}", events_path.display());
    }
    println!();
    println!("Next:");
    println!("  ./scripts/nf4 status");
    println!("  ./scripts/nf4 logs client");
    println!("  ./scripts/nf4 webhook events");
    println!("  ./scripts/nf4 webhook salts");
    println!();
    println!(
        "If you want to do mock ERC deployments so you will have some tokens to play with, run:"
    );
    println!("  ./scripts/nf4 client deploy-mock-tokens");
    Ok(())
}

pub fn deploy_mock_tokens() -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }
    if !config::local_env_exists() {
        return Err("local.env was not found. Run nf4 wizard client first.".to_string());
    }

    let mut env = config::read_local_env();
    let profile = env_value(&env, "NF4_RUN_MODE").unwrap_or_else(|| "development".to_string());
    let profile_config = read_profile_config(&profile)?;
    let rpc_url = env_value(&env, "NF4_ETHEREUM_CLIENT_URL")
        .or_else(|| profile_config.ethereum_client_url.clone())
        .ok_or_else(|| {
            "NF4_ETHEREUM_CLIENT_URL was not found in local.env or nightfall.toml.".to_string()
        })?;

    let addresses = read_addresses()?;
    let nightfall_address = addresses
        .get("nightfall")
        .cloned()
        .ok_or_else(|| format!("{ADDRESSES_TOML} is missing nightfall"))?;
    if !is_non_zero_address(&nightfall_address) {
        return Err(format!(
            "{ADDRESSES_TOML} has zero or invalid nightfall address."
        ));
    }

    let signing_key = env_value(&env, "NF4_SIGNING_KEY")
        .or_else(|| env_value(&env, "CLIENT_SIGNING_KEY"))
        .ok_or_else(|| {
            "NF4_SIGNING_KEY or CLIENT_SIGNING_KEY was not found in local.env.".to_string()
        })?;
    let client_address =
        match env_value(&env, "CLIENT_ADDRESS").filter(|address| is_non_zero_address(address)) {
            Some(address) => address,
            None => cast_wallet_address(&signing_key)?,
        };
    if !is_non_zero_address(&client_address) {
        return Err("CLIENT_ADDRESS is missing or invalid, and could not be derived.".to_string());
    }
    let client2_address = env_value(&env, "CLIENT2_ADDRESS")
        .filter(|address| is_non_zero_address(address))
        .unwrap_or_else(|| client_address.clone());

    config::merge_local_env(&[
        ("NF4_SIGNING_KEY", signing_key.clone()),
        ("CLIENT_ADDRESS", client_address.clone()),
        ("CLIENT2_ADDRESS", client2_address.clone()),
        ("NIGHTFALL_ADDRESS", nightfall_address.clone()),
    ])?;

    env.insert("NF4_SIGNING_KEY".to_string(), signing_key);
    env.insert("CLIENT_ADDRESS".to_string(), client_address.clone());
    env.insert("CLIENT2_ADDRESS".to_string(), client2_address.clone());
    env.insert("NIGHTFALL_ADDRESS".to_string(), nightfall_address.clone());

    println!("Deploying mock ERC contracts for local testing...");
    println!("  rpc_url: {rpc_url}");
    println!("  owner: {client_address}");
    println!("  client2: {client2_address}");
    println!("  nightfall: {nightfall_address}");

    run_command("Cleaning contract build artifacts", "forge", &["clean"])?;
    run_command("Building contracts", "forge", &["build"])?;
    run_command_with_env(
        "Deploying mock ERC contracts",
        "forge",
        &[
            "script",
            MOCK_DEPLOYER_SCRIPT,
            "--rpc-url",
            &rpc_url,
            "--broadcast",
            "--legacy",
            "--slow",
        ],
        &env,
    )
}

fn check_prerequisites() -> Result<(), String> {
    let checks = [
        ("docker", command_ok("docker", &["--version"])),
        (
            "docker compose",
            command_ok("docker", &["compose", "version"]),
        ),
        ("cast", command_ok("cast", &["--version"])),
        ("forge", command_ok("forge", &["--version"])),
        ("curl", command_ok("curl", &["--version"])),
    ];

    println!("Checking client prerequisites...");
    for (name, ok) in checks {
        println!("  {:<14} {}", name, if ok { "OK" } else { "FAILED" });
    }

    if checks.iter().all(|(_, ok)| *ok) {
        Ok(())
    } else {
        Err("Client prerequisite checks failed.".to_string())
    }
}

fn command_ok(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .output()
        .is_ok_and(|output| output.status.success())
}

fn run_command(title: &str, program: &str, args: &[&str]) -> Result<(), String> {
    println!("{title}...");
    println!("  {program} {}", args.join(" "));

    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run {program}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
    }
}

fn run_command_with_env(
    title: &str,
    program: &str,
    args: &[&str],
    env: &BTreeMap<String, String>,
) -> Result<(), String> {
    println!("{title}...");
    println!("  {program} {}", args.join(" "));

    let status = Command::new(program)
        .args(args)
        .envs(env)
        .status()
        .map_err(|err| format!("Failed to run {program}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
    }
}

fn validate_metadata(mock_prover: bool) -> Result<(), String> {
    let addresses = read_addresses()?;
    for key in ["nightfall", "round_robin", "x509"] {
        let address = addresses
            .get(key)
            .ok_or_else(|| format!("{ADDRESSES_TOML} is missing {key}"))?;
        if !is_non_zero_address(address) {
            return Err(format!(
                "{ADDRESSES_TOML} has zero or invalid {key} address."
            ));
        }
    }

    let verifier = addresses
        .get("verifier")
        .ok_or_else(|| format!("{ADDRESSES_TOML} is missing verifier"))?;
    if !mock_prover && !is_non_zero_address(verifier) {
        return Err("verifier address is required in real prover mode.".to_string());
    }

    print_file_check("addresses.toml", ADDRESSES_TOML)?;
    print_file_check("contract_hashes.toml", CONTRACT_HASHES_TOML)?;
    print_file_check("proving_key", PROVING_KEY)?;
    Ok(())
}

fn print_file_check(label: &str, path: &str) -> Result<(), String> {
    if Path::new(path).is_file() {
        println!("  {label}: OK");
        Ok(())
    } else {
        Err(format!("Missing {path}"))
    }
}

fn read_addresses() -> Result<BTreeMap<String, String>, String> {
    let source = fs::read_to_string(ADDRESSES_TOML)
        .map_err(|err| format!("Failed to read {ADDRESSES_TOML}: {err}"))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {ADDRESSES_TOML}: {err}"))?;

    let mut addresses = BTreeMap::new();
    for key in ["nightfall", "round_robin", "x509", "verifier"] {
        let address = doc
            .get(key)
            .and_then(Item::as_str)
            .ok_or_else(|| format!("{ADDRESSES_TOML} is missing {key}"))?;
        addresses.insert(key.to_string(), address.to_string());
    }
    Ok(addresses)
}

fn prompt_client_key(existing: Option<&String>) -> Result<String, String> {
    println!();
    println!("Paste the funded L1 testnet client private key.");
    println!("Input is hidden for safety, so nothing will appear while typing.");
    println!("Include 0x if your key has it, then press Enter.");
    println!("Never share this key in chat or commit local.env.");

    let prompt = if existing.is_some() {
        "Client private key [hidden input, press Enter to reuse local.env]"
    } else {
        "Client private key [hidden input]"
    };

    let value = Password::new(prompt)
        .without_confirmation()
        .prompt()
        .map_err(|err| err.to_string())?;

    if value.trim().is_empty() {
        existing
            .cloned()
            .ok_or_else(|| "Client private key is required.".to_string())
    } else {
        Ok(value)
    }
}

fn prompt_client_address(
    existing: Option<&String>,
    derived_client_address: &str,
) -> Result<String, String> {
    let default = existing
        .filter(|address| is_non_zero_address(address))
        .cloned()
        .unwrap_or_else(|| derived_client_address.to_string());
    let address = Text::new("Client address [press Enter to use derived/default address]")
        .with_default(&default)
        .prompt()
        .map_err(|err| err.to_string())?;

    if is_non_zero_address(&address) {
        if !address.eq_ignore_ascii_case(derived_client_address) {
            println!(
                "  warning: CLIENT_ADDRESS does not match the address derived from CLIENT_SIGNING_KEY"
            );
        }
        Ok(address)
    } else {
        Err("Client address must be a non-zero Ethereum address.".to_string())
    }
}

fn prompt_url(label: &str, default: &str) -> Result<String, String> {
    let url = Text::new(label)
        .with_default(default)
        .prompt()
        .map_err(|err| err.to_string())?;

    if valid_http_url(&url) {
        Ok(url)
    } else {
        Err("URL must start with http:// or https:// and include a host.".to_string())
    }
}

fn configure_webhook(env: &BTreeMap<String, String>) -> Result<WebhookSetup, String> {
    let existing_url = env_value(env, "NF4_NIGHTFALL_CLIENT__WEBHOOK_URL")
        .or_else(|| env_value(env, "WEBHOOK_URL"));
    let default_url =
        existing_url.unwrap_or_else(|| format!("http://{}:8081/webhook", detected_host()));

    let start_local = Confirm::new("Start local testing webhook?")
        .with_default(true)
        .prompt()
        .map_err(|err| err.to_string())?;

    if start_local {
        let port = prompt_port("Webhook port", url_port(&default_url).unwrap_or(8081))?;
        let local = webhook::ensure_local(port)?;
        Ok(WebhookSetup {
            url: format!("http://{}:{port}/webhook", detected_host()),
            events_path: Some(local.events_path),
        })
    } else {
        let url = prompt_url("Webhook URL [press Enter to use default]", &default_url)?;
        Ok(WebhookSetup {
            url,
            events_path: None,
        })
    }
}

fn prompt_port(label: &str, default: u16) -> Result<u16, String> {
    let prompt = format!("{label} [default: {default}, press Enter to use default]");
    let value = Text::new(&prompt)
        .with_default(&default.to_string())
        .prompt()
        .map_err(|err| err.to_string())?;

    value
        .parse::<u16>()
        .map_err(|_| format!("{label} must be a port number between 1 and 65535."))
}

fn cast_wallet_address(private_key: &str) -> Result<String, String> {
    let output = Command::new("cast")
        .args(["wallet", "address", "--private-key", private_key])
        .output()
        .map_err(|err| format!("Failed to derive client address with cast: {err}"))?;

    if !output.status.success() {
        return Err(
            command_detail(&output).unwrap_or_else(|| "cast wallet address failed".to_string())
        );
    }

    first_line(&output.stdout).ok_or_else(|| "cast wallet address returned no output.".to_string())
}

fn validate_configuration_runtime_url(configuration_url: &str) -> Result<(), String> {
    println!("Checking client configuration URL...");

    if docker_service_url(configuration_url) {
        println!("  skipped host-side curl for Docker service URL: {configuration_url}");
        return Ok(());
    }

    let checks = validation::configuration_endpoint_checks(configuration_url);
    let mut failures = Vec::new();
    for check in checks {
        if check.ok {
            println!("  {}: OK", check.label);
        } else {
            println!("  {}: FAILED - {}", check.label, check.detail);
            failures.push(format!("{}: {}", check.label, check.detail));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Configuration URL is not reachable: {}",
            failures.join("; ")
        ))
    }
}

fn validate_proposer_health(proposer_url: &str) -> Result<(), String> {
    let url = endpoint_url(proposer_url, "v1/health");
    println!("Checking proposer health at {url}...");
    if curl_ok(&url) {
        println!("  proposer health: OK");
        Ok(())
    } else {
        Err(format!(
            "Proposer health check failed at {url}. Start proposer first with nf4 wizard proposer."
        ))
    }
}

fn poll_client_health(port: u16) -> Result<(), String> {
    let url = format!("http://127.0.0.1:{port}/v1/health");
    println!("Checking client health at {url}...");

    for attempt in 1..=20 {
        if curl_ok(&url) {
            println!("  client health: OK");
            return Ok(());
        }
        if attempt == 1 {
            println!("  client is starting; waiting for health endpoint...");
        }
        thread::sleep(Duration::from_secs(3));
    }

    Err("Client health check failed. Try: nf4 logs client".to_string())
}

fn curl_ok(url: &str) -> bool {
    Command::new("curl")
        .args(["-fsS", "--max-time", "5", "-o", "/dev/null", url])
        .output()
        .is_ok_and(|output| output.status.success())
}

fn read_profile_config(profile: &str) -> Result<ProfileConfig, String> {
    let source = fs::read_to_string(config::NIGHTFALL_TOML)
        .map_err(|err| format!("Failed to read {}: {err}", config::NIGHTFALL_TOML))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {}: {err}", config::NIGHTFALL_TOML))?;
    let item = doc
        .get(profile)
        .ok_or_else(|| format!("Profile [{profile}] was not found in nightfall.toml."))?;

    Ok(ProfileConfig {
        ethereum_client_url: get_str(item, &["ethereum_client_url"]),
        configuration_url: get_str(item, &["configuration_url"]),
        mock_prover: get_bool(item, &["mock_prover"]),
        nightfall_proposer_url: get_str(item, &["nightfall_proposer", "url"]),
    })
}

fn get_str(item: &Item, path: &[&str]) -> Option<String> {
    let mut current = item;
    for key in path {
        current = current.get(key)?;
    }
    current.as_str().map(ToString::to_string)
}

fn get_bool(item: &Item, path: &[&str]) -> Option<bool> {
    let mut current = item;
    for key in path {
        current = current.get(key)?;
    }
    current.as_bool()
}

fn env_value(values: &BTreeMap<String, String>, key: &str) -> Option<String> {
    values
        .get(key)
        .cloned()
        .or_else(|| std::env::var(key).ok())
        .filter(|value| !value.is_empty())
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Some(true),
        "false" | "0" | "no" | "n" => Some(false),
        _ => None,
    }
}

fn default_proposer_url(env: &BTreeMap<String, String>, profile: &ProfileConfig) -> String {
    env_value(env, "NF4_NIGHTFALL_PROPOSER__URL")
        .or_else(|| profile.nightfall_proposer_url.clone())
        .and_then(|url| normalize_or_reject_url(&url))
        .filter(|url| url_host(url) != Some("proposer"))
        .unwrap_or_else(|| format!("http://{}:3001", detected_host()))
}

fn client_api_port(env: &BTreeMap<String, String>) -> Option<u16> {
    env_value(env, "CLIENT_API_URL").and_then(|url| url_port(&url))
}

fn normalize_or_reject_url(url: &str) -> Option<String> {
    if valid_http_url(url) {
        Some(url.to_string())
    } else if url.contains(':') && !url.contains("://") {
        let normalized = format!("http://{url}");
        valid_http_url(&normalized).then_some(normalized)
    } else {
        None
    }
}

fn valid_http_url(url: &str) -> bool {
    (url.starts_with("http://") || url.starts_with("https://"))
        && url_host(url).is_some_and(|host| !host.is_empty())
}

fn docker_service_url(url: &str) -> bool {
    url_host(url).is_some_and(|host| matches!(host, "configuration" | "nf4_configuration"))
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
    String::from_utf8_lossy(&output.stdout)
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
    first_line(&output.stdout).filter(|ip| !ip.starts_with("127."))
}

fn url_port(url: &str) -> Option<u16> {
    let host_port = url_host_port(url)?;
    host_port.rsplit_once(':')?.1.parse().ok()
}

fn url_host(url: &str) -> Option<&str> {
    let host_port = url_host_port(url)?;
    Some(
        host_port
            .rsplit_once(':')
            .map(|(host, _)| host)
            .unwrap_or(host_port),
    )
}

fn url_host_port(url: &str) -> Option<&str> {
    let (_, rest) = url.split_once("://")?;
    let authority = rest.split('/').next().unwrap_or(rest);
    if authority.is_empty() {
        None
    } else {
        Some(authority)
    }
}

fn endpoint_url(base_url: &str, path: &str) -> String {
    format!("{}/{}", base_url.trim_end_matches('/'), path)
}

fn first_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}

fn command_detail(output: &std::process::Output) -> Option<String> {
    first_line(&output.stderr).or_else(|| first_line(&output.stdout))
}

fn is_non_zero_address(address: &str) -> bool {
    let Some(hex) = address.strip_prefix("0x") else {
        return false;
    };
    hex.len() == 40 && hex.chars().any(|ch| ch != '0')
}

fn print_review(review: &ClientReview<'_>) {
    println!();
    println!("Review client settings");
    println!("  profile: {}", review.profile);
    println!("  rpc_url: {}", review.rpc_url);
    println!(
        "  prover_mode: {}",
        if review.mock_prover { "mock" } else { "real" }
    );
    println!("  client_address: {}", review.client_address);
    println!("  client_api_url: {}", review.client_api_url);
    println!("  proposer_url: {}", review.proposer_url);
    println!("  configuration_runtime_url: {}", review.configuration_url);
    println!("  webhook_url: {}", review.webhook_url);
    println!("  client_port: {}", review.client_port);
}

#[derive(Debug, Default)]
struct ProfileConfig {
    ethereum_client_url: Option<String>,
    configuration_url: Option<String>,
    mock_prover: Option<bool>,
    nightfall_proposer_url: Option<String>,
}

struct ClientReview<'a> {
    profile: &'a str,
    rpc_url: &'a str,
    mock_prover: bool,
    client_address: &'a str,
    client_api_url: &'a str,
    proposer_url: &'a str,
    configuration_url: &'a str,
    webhook_url: &'a str,
    client_port: u16,
}

struct WebhookSetup {
    url: String,
    events_path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::{
        client_api_port, docker_service_url, endpoint_url, is_non_zero_address,
        normalize_or_reject_url, parse_bool, url_host, url_port, valid_http_url,
    };
    use std::collections::BTreeMap;

    #[test]
    fn parses_bool_values() {
        assert_eq!(parse_bool("yes"), Some(true));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn validates_non_zero_addresses() {
        assert!(is_non_zero_address(
            "0x0000000000000000000000000000000000000001"
        ));
        assert!(!is_non_zero_address(
            "0x0000000000000000000000000000000000000000"
        ));
        assert!(!is_non_zero_address("0xabc"));
    }

    #[test]
    fn parses_and_normalizes_urls() {
        assert!(valid_http_url("http://10.0.0.8:3001"));
        assert_eq!(url_host("http://10.0.0.8:3001/path"), Some("10.0.0.8"));
        assert_eq!(url_port("http://10.0.0.8:3001/path"), Some(3001));
        assert_eq!(
            normalize_or_reject_url("localhost:3001").unwrap(),
            "http://localhost:3001"
        );
        assert!(normalize_or_reject_url("not a url").is_none());
    }

    #[test]
    fn detects_docker_configuration_service_urls() {
        assert!(docker_service_url("http://configuration:80"));
        assert!(!docker_service_url("http://10.0.0.8:8080"));
    }

    #[test]
    fn builds_endpoint_urls() {
        assert_eq!(
            endpoint_url("http://127.0.0.1:3000/", "v1/health"),
            "http://127.0.0.1:3000/v1/health"
        );
    }

    #[test]
    fn reads_client_api_port_from_env() {
        let mut env = BTreeMap::new();
        env.insert(
            "CLIENT_API_URL".to_string(),
            "http://127.0.0.1:4000".to_string(),
        );
        assert_eq!(client_api_port(&env), Some(4000));
    }
}
