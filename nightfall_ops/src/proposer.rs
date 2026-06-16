use std::{collections::BTreeMap, fs, path::Path, process::Command, thread, time::Duration};

use inquire::{Confirm, Password, Text};
use toml_edit::{DocumentMut, Item};

use crate::{compose, config, validation};

const ADDRESSES_TOML: &str = "configuration/toml/addresses.toml";
const CONTRACT_HASHES_TOML: &str = "configuration/toml/contract_hashes.toml";
const KEY_DIR: &str = "configuration/bin/keys";
const MOCK_KEYS: &[&str] = &["proving_key"];
const REAL_KEYS: &[&str] = &[
    "base_bn254_pk",
    "base_grumpkin_pk",
    "decider_pk",
    "decider_vk",
    "merge_bn254_pk_0",
    "merge_grumpkin_pk_0",
    "merge_grumpkin_pk_1",
    "proving_key",
];

pub fn wizard() -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }
    if !config::local_env_exists() {
        return Err("local.env was not found. Run nf4 wizard deploy first.".to_string());
    }

    println!("This command starts an indie proposer node for the existing local deployment.");
    println!();
    println!(
        "It reuses nightfall.toml, docker-compose.yml, local.env, addresses, hashes, and keys."
    );
    println!("You only need to confirm proposer-specific values.");
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

    println!("Checking local deployment metadata...");
    validate_metadata(mock_prover)?;

    let proposer_key = prompt_proposer_key(env.get("PROPOSER_SIGNING_KEY"))?;
    let proposer_address = cast_wallet_address(&proposer_key)?;
    println!("Derived proposer address: {proposer_address}");

    let proposer_role = proposer_role(
        &proposer_address,
        profile_config.default_proposer_address.as_deref(),
    );

    let proposer_url_default = default_proposer_url(&env, &profile_config);
    let proposer_url = prompt_url(
        "Public proposer URL [press Enter to use default]",
        &proposer_url_default,
    )?;
    let proposer_port = url_port(&proposer_url).unwrap_or(3001);

    let configuration_url_default = env_value(&env, "NF4_CONFIGURATION_URL")
        .or_else(|| profile_config.configuration_url.clone())
        .unwrap_or_else(|| format!("http://{}:8080", detected_host()));
    let configuration_url = prompt_url(
        "Configuration URL for proposer runtime [press Enter to use default]",
        &configuration_url_default,
    )?;

    validate_configuration_runtime_url(&configuration_url)?;

    print_review(&ProposerReview {
        profile: &profile,
        rpc_url: &rpc_url,
        mock_prover,
        configuration_url: &configuration_url,
        proposer_url: &proposer_url,
        proposer_address: &proposer_address,
        proposer_role,
        proposer_port,
    });

    if !Confirm::new("Apply these changes and start indie-proposer?")
        .with_default(true)
        .prompt()
        .map_err(|err| err.to_string())?
    {
        return Err("Proposer setup cancelled before writing files.".to_string());
    }

    let backup_dir = config::backup_config_files()?;
    println!("Backed up config files to {}", backup_dir.display());

    config::merge_local_env(&[
        ("PROPOSER_SIGNING_KEY", proposer_key),
        ("NF4_CONFIGURATION_URL", configuration_url.clone()),
        ("NF4_NIGHTFALL_PROPOSER__URL", proposer_url.clone()),
        ("NF4_CONTRACTS__DEPLOY_CONTRACTS", "false".to_string()),
    ])?;
    config::update_proposer_docker_compose(&profile, proposer_port)?;

    compose::build_indie_proposer()?;
    compose::up_proposer()?;
    poll_proposer_health(proposer_port)?;

    println!();
    println!("Proposer OK");
    println!("  profile: {profile}");
    println!("  mode: {}", if mock_prover { "mock" } else { "real" });
    println!("  proposer_address: {proposer_address}");
    println!("  proposer_role: {}", proposer_role.label());
    println!("  proposer_url: {proposer_url}");
    println!("  configuration_runtime_url: {configuration_url}");

    if proposer_role == ProposerRole::Additional {
        println!();
        println!("This proposer is not the designated proposer registered during deployment.");
        println!("It must register and rotate before it can propose blocks.");
    }

    Ok(())
}

fn check_prerequisites() -> Result<(), String> {
    let checks = [
        ("docker", command_ok("docker", &["--version"])),
        (
            "docker compose",
            command_ok("docker", &["compose", "version"]),
        ),
        ("cast", command_ok("cast", &["--version"])),
        ("curl", command_ok("curl", &["--version"])),
    ];

    println!("Checking proposer prerequisites...");
    for (name, ok) in checks {
        println!("  {:<14} {}", name, if ok { "OK" } else { "FAILED" });
    }

    if checks.iter().all(|(_, ok)| *ok) {
        Ok(())
    } else {
        Err("Proposer prerequisite checks failed.".to_string())
    }
}

fn command_ok(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .output()
        .is_ok_and(|output| output.status.success())
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

    let keys = if mock_prover { MOCK_KEYS } else { REAL_KEYS };
    for key in keys {
        let path = Path::new(KEY_DIR).join(key);
        if !path.is_file() {
            return Err(format!("Missing required key file: {}", path.display()));
        }
    }
    println!("  keys: OK");
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

fn validate_configuration_runtime_url(configuration_url: &str) -> Result<(), String> {
    println!("Checking proposer configuration URL...");

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

fn docker_service_url(url: &str) -> bool {
    url_host(url).is_some_and(|host| matches!(host, "configuration" | "nf4_configuration"))
}

fn prompt_proposer_key(existing: Option<&String>) -> Result<String, String> {
    println!();
    println!("Paste the funded L1 testnet proposer private key.");
    println!("Input is hidden for safety, so nothing will appear while typing.");
    println!("Include 0x if your key has it, then press Enter.");
    println!("Never share this key in chat or commit local.env.");

    let prompt = if existing.is_some() {
        "Proposer private key [hidden input, press Enter to reuse local.env]"
    } else {
        "Proposer private key [hidden input]"
    };

    let value = Password::new(prompt)
        .without_confirmation()
        .prompt()
        .map_err(|err| err.to_string())?;

    if value.trim().is_empty() {
        existing
            .cloned()
            .ok_or_else(|| "Proposer private key is required.".to_string())
    } else {
        Ok(value)
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

fn valid_http_url(url: &str) -> bool {
    (url.starts_with("http://") || url.starts_with("https://"))
        && url_host(url).is_some_and(|host| !host.is_empty())
}

fn cast_wallet_address(private_key: &str) -> Result<String, String> {
    let output = Command::new("cast")
        .args(["wallet", "address", "--private-key", private_key])
        .output()
        .map_err(|err| format!("Failed to derive proposer address with cast: {err}"))?;

    if !output.status.success() {
        return Err(
            command_detail(&output).unwrap_or_else(|| "cast wallet address failed".to_string())
        );
    }

    first_line(&output.stdout).ok_or_else(|| "cast wallet address returned no output.".to_string())
}

fn poll_proposer_health(port: u16) -> Result<(), String> {
    let url = format!("http://127.0.0.1:{port}/v1/health");
    println!("Checking proposer health at {url}...");

    for attempt in 1..=20 {
        if curl_ok(&url) {
            println!("  proposer health: OK");
            return Ok(());
        }
        if attempt == 1 {
            println!("  proposer is starting; waiting for health endpoint...");
        }
        thread::sleep(Duration::from_secs(3));
    }

    Err(format!(
        "Proposer health check failed. Try: nf4 logs proposer"
    ))
}

fn curl_ok(url: &str) -> bool {
    Command::new("curl")
        .args(["-fsS", "--max-time", "5", "-o", "/dev/null", url])
        .output()
        .is_ok_and(|output| output.status.success())
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
        default_proposer_address: get_str(
            item,
            &["nightfall_deployer", "default_proposer_address"],
        ),
        default_proposer_url: get_str(item, &["nightfall_deployer", "default_proposer_url"]),
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
        .or_else(|| {
            profile
                .default_proposer_url
                .clone()
                .filter(|url| valid_http_url(url) && url_host(url) != Some("proposer"))
        })
        .unwrap_or_else(|| format!("http://{}:3001", detected_host()))
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

fn is_non_zero_address(address: &str) -> bool {
    let Some(hex) = address.strip_prefix("0x") else {
        return false;
    };
    hex.len() == 40 && hex.chars().any(|ch| ch != '0')
}

fn proposer_role(proposer_address: &str, default_proposer_address: Option<&str>) -> ProposerRole {
    match default_proposer_address {
        Some(default) if proposer_address.eq_ignore_ascii_case(default) => ProposerRole::Designated,
        _ => ProposerRole::Additional,
    }
}

fn print_review(review: &ProposerReview<'_>) {
    println!();
    println!("Review proposer settings");
    println!("  profile: {}", review.profile);
    println!("  rpc_url: {}", review.rpc_url);
    println!(
        "  prover_mode: {}",
        if review.mock_prover { "mock" } else { "real" }
    );
    println!("  configuration_runtime_url: {}", review.configuration_url);
    println!("  proposer_public_url: {}", review.proposer_url);
    println!("  proposer_address: {}", review.proposer_address);
    println!("  proposer_role: {}", review.proposer_role.label());
    println!("  proposer_port: {}", review.proposer_port);
}

#[derive(Debug, Default)]
struct ProfileConfig {
    ethereum_client_url: Option<String>,
    configuration_url: Option<String>,
    mock_prover: Option<bool>,
    default_proposer_address: Option<String>,
    default_proposer_url: Option<String>,
}

struct ProposerReview<'a> {
    profile: &'a str,
    rpc_url: &'a str,
    mock_prover: bool,
    configuration_url: &'a str,
    proposer_url: &'a str,
    proposer_address: &'a str,
    proposer_role: ProposerRole,
    proposer_port: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProposerRole {
    Designated,
    Additional,
}

impl ProposerRole {
    fn label(self) -> &'static str {
        match self {
            ProposerRole::Designated => "designated proposer",
            ProposerRole::Additional => "additional proposer",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProposerRole, docker_service_url, is_non_zero_address, parse_bool, proposer_role, url_host,
        url_port, valid_http_url,
    };

    #[test]
    fn parses_bool_values() {
        assert_eq!(parse_bool("true"), Some(true));
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
    fn parses_url_host_and_port() {
        assert!(valid_http_url("http://10.0.0.8:3001"));
        assert_eq!(url_host("http://10.0.0.8:3001/path"), Some("10.0.0.8"));
        assert_eq!(url_port("http://10.0.0.8:3001/path"), Some(3001));
        assert_eq!(url_port("https://config.example.com"), None);
        assert!(!valid_http_url("localhost:3001"));
    }

    #[test]
    fn detects_docker_configuration_service_urls() {
        assert!(docker_service_url("http://configuration:80"));
        assert!(docker_service_url("http://nf4_configuration"));
        assert!(!docker_service_url("http://10.0.0.8:8080"));
    }

    #[test]
    fn detects_proposer_role() {
        assert_eq!(
            proposer_role(
                "0x0000000000000000000000000000000000000001",
                Some("0x0000000000000000000000000000000000000001")
            ),
            ProposerRole::Designated
        );
        assert_eq!(
            proposer_role(
                "0x0000000000000000000000000000000000000002",
                Some("0x0000000000000000000000000000000000000001")
            ),
            ProposerRole::Additional
        );
    }
}
