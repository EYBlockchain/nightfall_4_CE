use std::{collections::BTreeMap, fs, path::Path, process::Command};

use toml_edit::{DocumentMut, Item};

use crate::{config, validation};

const ADDRESSES_TOML: &str = "configuration/toml/addresses.toml";
const CONTRACT_HASHES_TOML: &str = "configuration/toml/contract_hashes.toml";
const PROVING_KEY: &str = "configuration/bin/keys/proving_key";

pub fn print() -> Result<(), String> {
    let env = read_local_env().unwrap_or_default();
    let profile = env_value(&env, "NF4_RUN_MODE").unwrap_or_else(|| "development".to_string());
    let profile_config = read_profile_config(&profile);
    let rpc_url = env_value(&env, "NF4_ETHEREUM_CLIENT_URL").or_else(|| {
        profile_config
            .as_ref()
            .and_then(|config| config.rpc_url.clone())
    });
    let configuration_url = profile_config
        .as_ref()
        .and_then(|config| config.configuration_url.clone());
    let expected_chain_id = profile_config.as_ref().and_then(|config| config.chain_id);
    let mock_prover = env_value(&env, "NF4_MOCK_PROVER")
        .and_then(|value| parse_bool(&value))
        .or_else(|| {
            profile_config
                .as_ref()
                .and_then(|config| config.mock_prover)
        });

    println!("Nightfall testnet status");
    println!();
    println!("Profile: {profile}");
    println!(
        "Prover: {}",
        match mock_prover {
            Some(true) => "mock",
            Some(false) => "real",
            None => "unknown",
        }
    );

    match &rpc_url {
        Some(url) => print_rpc_status(url, expected_chain_id),
        None => println!("RPC: UNCHECKED, NF4_ETHEREUM_CLIENT_URL was not found"),
    }

    match &configuration_url {
        Some(url) => println!("Configuration URL: {url}"),
        None => println!("Configuration URL: UNCONFIGURED"),
    }

    println!();
    println!("Repository files:");
    print_file_check("nightfall.toml", config::NIGHTFALL_TOML);
    print_file_check("docker-compose.yml", config::DOCKER_COMPOSE_YML);
    print_file_check("local.env", config::LOCAL_ENV);

    println!();
    println!("Services:");
    print_container_status("Deployer", "nf4_indie_deployer");
    print_container_status("Configuration", "nf4_configuration");
    print_container_status("Proposer", "nf4_indie_proposer");
    print_container_status("Client", "nf4_indie_client");

    if let Some(url) = env_value(&env, "NF4_NIGHTFALL_PROPOSER__URL") {
        print_http_health("Proposer health", &endpoint_url(&url, "v1/health"));
    }
    if let Some(url) = env_value(&env, "CLIENT_API_URL") {
        print_http_health("Client health", &endpoint_url(&url, "v1/health"));
    }

    println!();
    println!("Deployment metadata:");
    print_file_check("addresses.toml", ADDRESSES_TOML);
    print_file_check("contract_hashes.toml", CONTRACT_HASHES_TOML);
    print_file_check("proving_key", PROVING_KEY);

    println!();
    println!("Contracts:");
    print_contracts(&rpc_url, mock_prover == Some(true));

    if let Some(url) = configuration_url {
        println!();
        println!("Configuration endpoint checks:");
        let checks = validation::configuration_endpoint_checks(&url);
        for check in checks {
            if check.ok {
                println!("  {}: OK ({})", check.label, check.url);
            } else {
                println!("  {}: FAILED - {}", check.label, check.detail);
            }
        }
    }

    Ok(())
}

fn print_file_check(label: &str, path: &str) {
    let status = if Path::new(path).is_file() {
        "OK"
    } else {
        "MISSING"
    };
    println!("  {label}: {status} ({path})");
}

fn print_rpc_status(rpc_url: &str, expected_chain_id: Option<u64>) {
    let block = cast_value("block-number", rpc_url);
    let chain = cast_value("chain-id", rpc_url);

    match (block, chain) {
        (Ok(block), Ok(chain)) => {
            let chain_label = match expected_chain_id {
                Some(expected) if expected == chain => format!("chain ID OK, {chain}"),
                Some(expected) => format!("chain ID FAILED, expected {expected}, got {chain}"),
                None => format!("chain ID {chain}"),
            };
            println!("RPC: OK, latest block {block}, {chain_label}");
        }
        (Err(err), _) | (_, Err(err)) => println!("RPC: FAILED, {err}"),
    }
}

fn print_container_status(label: &str, container_name: &str) {
    match docker_container_status(container_name) {
        Ok(Some(status)) => println!("  {label}: {status}"),
        Ok(None) => println!("  {label}: not found"),
        Err(err) => println!("  {label}: UNCHECKED - {err}"),
    }
}

fn print_http_health(label: &str, url: &str) {
    match curl_health(url) {
        Ok(true) => println!("  {label}: OK ({url})"),
        Ok(false) => println!("  {label}: FAILED ({url})"),
        Err(err) => println!("  {label}: UNCHECKED - {err}"),
    }
}

fn print_contracts(rpc_url: &Option<String>, mock_prover: bool) {
    match read_addresses() {
        Ok(addresses) => {
            for (label, address) in addresses {
                if mock_prover && label == "verifier" && !is_non_zero_address(&address) {
                    println!("  verifier: SKIPPED, not deployed in mock prover mode");
                    continue;
                }

                let local_status = if is_non_zero_address(&address) {
                    "OK"
                } else {
                    "FAILED"
                };
                print!("  {label}: {local_status}, {address}");

                if let Some(rpc_url) = rpc_url {
                    match contract_has_code(&address, rpc_url) {
                        Ok(true) => println!(", code OK"),
                        Ok(false) => println!(", code MISSING"),
                        Err(err) => println!(", code UNCHECKED - {err}"),
                    }
                } else {
                    println!();
                }
            }
        }
        Err(err) => println!("  UNCHECKED - {err}"),
    }
}

fn cast_value(command: &str, rpc_url: &str) -> Result<u64, String> {
    let output = Command::new("cast")
        .args([command, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("failed to run cast {command}: {err}"))?;

    if !output.status.success() {
        return Err(command_detail(&output).unwrap_or_else(|| format!("cast {command} failed")));
    }

    numeric_line(&output.stdout)
        .ok_or_else(|| format!("cast {command} returned no numeric output"))?
        .parse::<u64>()
        .map_err(|err| format!("failed to parse cast {command} output: {err}"))
}

fn contract_has_code(address: &str, rpc_url: &str) -> Result<bool, String> {
    if !is_non_zero_address(address) {
        return Ok(false);
    }

    let output = Command::new("cast")
        .args(["code", address, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("failed to run cast code: {err}"))?;

    if !output.status.success() {
        return Err(command_detail(&output).unwrap_or_else(|| "cast code failed".to_string()));
    }

    let code = first_stdout_line(&output.stdout).unwrap_or_default();
    Ok(!code.is_empty() && code != "0x")
}

fn docker_container_status(container_name: &str) -> Result<Option<String>, String> {
    let output = Command::new("docker")
        .args([
            "ps",
            "-a",
            "--filter",
            &format!("name={container_name}"),
            "--format",
            "{{.Status}}",
        ])
        .output()
        .map_err(|err| format!("failed to run docker ps: {err}"))?;

    if !output.status.success() {
        return Err(command_detail(&output).unwrap_or_else(|| "docker ps failed".to_string()));
    }

    Ok(command_detail(&output))
}

fn curl_health(url: &str) -> Result<bool, String> {
    let output = Command::new("curl")
        .args(["-fsS", "--max-time", "5", "-o", "/dev/null", url])
        .output()
        .map_err(|err| format!("failed to run curl: {err}"))?;

    Ok(output.status.success())
}

fn endpoint_url(base_url: &str, path: &str) -> String {
    format!("{}/{}", base_url.trim_end_matches('/'), path)
}

fn read_addresses() -> Result<Vec<(&'static str, String)>, String> {
    let source = fs::read_to_string(ADDRESSES_TOML)
        .map_err(|err| format!("missing {ADDRESSES_TOML}: {err}"))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("failed to parse {ADDRESSES_TOML}: {err}"))?;

    let mut addresses = Vec::new();
    for key in ["nightfall", "round_robin", "x509", "verifier"] {
        let address = doc
            .get(key)
            .and_then(Item::as_str)
            .ok_or_else(|| format!("{ADDRESSES_TOML} is missing {key}"))?
            .to_string();
        addresses.push((key, address));
    }
    Ok(addresses)
}

fn read_profile_config(profile: &str) -> Option<ProfileConfig> {
    let source = fs::read_to_string(config::NIGHTFALL_TOML).ok()?;
    let doc = source.parse::<DocumentMut>().ok()?;
    let item = doc.get(profile)?;

    Some(ProfileConfig {
        rpc_url: get_str(item, &["ethereum_client_url"]),
        configuration_url: get_str(item, &["configuration_url"]),
        chain_id: get_i64(item, &["network", "chain_id"]).map(|value| value as u64),
        mock_prover: get_bool(item, &["mock_prover"]),
    })
}

fn read_local_env() -> Result<BTreeMap<String, String>, String> {
    let source = fs::read_to_string(config::LOCAL_ENV)
        .map_err(|err| format!("failed to read {}: {err}", config::LOCAL_ENV))?;
    Ok(parse_env_text(&source))
}

fn parse_env_text(source: &str) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        values.insert(
            key.trim().to_string(),
            strip_quotes(value.trim()).to_string(),
        );
    }
    values
}

fn env_value(values: &BTreeMap<String, String>, key: &str) -> Option<String> {
    values
        .get(key)
        .cloned()
        .or_else(|| std::env::var(key).ok())
        .filter(|value| !value.is_empty())
}

fn strip_quotes(value: &str) -> &str {
    if value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\'')))
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn get_item<'a>(item: &'a Item, path: &[&str]) -> Option<&'a Item> {
    let mut current = item;
    for key in path {
        current = current.get(key)?;
    }
    Some(current)
}

fn get_str(item: &Item, path: &[&str]) -> Option<String> {
    get_item(item, path)
        .and_then(Item::as_str)
        .map(ToString::to_string)
}

fn get_i64(item: &Item, path: &[&str]) -> Option<i64> {
    get_item(item, path).and_then(Item::as_integer)
}

fn get_bool(item: &Item, path: &[&str]) -> Option<bool> {
    get_item(item, path).and_then(Item::as_bool)
}

fn command_detail(output: &std::process::Output) -> Option<String> {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}

fn first_stdout_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}

fn numeric_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| line.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToString::to_string)
}

fn is_non_zero_address(address: &str) -> bool {
    let address = address.strip_prefix("0x").unwrap_or(address);
    address.len() == 40 && address.chars().any(|ch| ch != '0')
}

struct ProfileConfig {
    rpc_url: Option<String>,
    configuration_url: Option<String>,
    chain_id: Option<u64>,
    mock_prover: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::{endpoint_url, is_non_zero_address, numeric_line, parse_env_text, strip_quotes};

    #[test]
    fn parses_local_env_values() {
        let env = parse_env_text(
            r#"
DEPLOYER_SIGNING_KEY="0xabc"
NF4_RUN_MODE="sepolia"
export NF4_MOCK_PROVER=false
"#,
        );

        assert_eq!(env.get("NF4_RUN_MODE").unwrap(), "sepolia");
        assert_eq!(env.get("NF4_MOCK_PROVER").unwrap(), "false");
        assert_eq!(env.get("DEPLOYER_SIGNING_KEY").unwrap(), "0xabc");
    }

    #[test]
    fn strips_matching_quotes() {
        assert_eq!(strip_quotes("\"value\""), "value");
        assert_eq!(strip_quotes("'value'"), "value");
        assert_eq!(strip_quotes("value"), "value");
    }

    #[test]
    fn detects_non_zero_addresses() {
        assert!(is_non_zero_address(
            "0x1111111111111111111111111111111111111111"
        ));
        assert!(!is_non_zero_address(
            "0x0000000000000000000000000000000000000000"
        ));
        assert!(!is_non_zero_address(""));
    }

    #[test]
    fn parses_numeric_cast_output_after_warnings() {
        assert_eq!(
            numeric_line(
                b"Warning: Found unknown `debug` config for profile `default` defined in foundry.toml.\n11155111\n"
            )
            .unwrap(),
            "11155111"
        );
    }

    #[test]
    fn joins_endpoint_url_without_double_slash() {
        assert_eq!(
            endpoint_url("http://127.0.0.1:3001/", "v1/health"),
            "http://127.0.0.1:3001/v1/health"
        );
    }
}
