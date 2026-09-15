use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use toml_edit::{DocumentMut, Item, value};

use crate::model::DeploymentConfig;
use crate::network::{compose_rpc_url, detect_lan_host, with_lan_host, NetworkKind};

pub const NIGHTFALL_TOML: &str = "nightfall.toml";
pub const DOCKER_COMPOSE_YML: &str = "docker-compose.yml";
pub const LOCAL_ENV: &str = "local.env";

pub fn required_repo_files_exist() -> bool {
    [NIGHTFALL_TOML, DOCKER_COMPOSE_YML]
        .iter()
        .all(|path| Path::new(path).is_file())
}

pub fn local_env_exists() -> bool {
    Path::new(LOCAL_ENV).is_file()
}

pub fn read_local_env() -> BTreeMap<String, String> {
    fs::read_to_string(LOCAL_ENV)
        .map(|source| parse_env_text(&source))
        .unwrap_or_default()
}

pub fn merge_local_env(values: &[(&str, String)]) -> Result<(), String> {
    let source = fs::read_to_string(LOCAL_ENV).unwrap_or_default();
    let updated = merge_local_env_text(&source, values);
    fs::write(LOCAL_ENV, updated).map_err(|err| format!("Failed to write {LOCAL_ENV}: {err}"))?;
    restrict_local_env_permissions()?;
    Ok(())
}

pub fn backup_config_files() -> Result<PathBuf, String> {
    let backup_dir = Path::new(".nightfall")
        .join("backups")
        .join(timestamp().to_string());
    fs::create_dir_all(&backup_dir)
        .map_err(|err| format!("Failed to create backup directory {backup_dir:?}: {err}"))?;

    for file in [NIGHTFALL_TOML, DOCKER_COMPOSE_YML] {
        fs::copy(file, backup_dir.join(file))
            .map_err(|err| format!("Failed to back up {file}: {err}"))?;
    }

    Ok(backup_dir)
}

pub fn write_local_env(config: &DeploymentConfig) -> Result<(), String> {
    merge_local_env(&[
        ("DEPLOYER_SIGNING_KEY", config.deployer_signing_key.clone()),
        (
            "NF4_ETHEREUM_CLIENT_URL",
            compose_rpc_url(config.network, &config.rpc_url),
        ),
        ("NF4_RUN_MODE", config.profile.clone()),
        ("NF4_NETWORK", config.network.as_str().to_string()),
        ("NF4_CONTRACTS__DEPLOY_CONTRACTS", "true".to_string()),
        ("NF4_MOCK_PROVER", config.mock_prover.to_string()),
        (
            "NF4_CONFIGURATION_URL",
            local_configuration_url(config),
        ),
    ])
}

fn local_configuration_url(config: &DeploymentConfig) -> String {
    if config.network == NetworkKind::Local {
        with_lan_host(&config.configuration_url, &detect_lan_host())
    } else {
        config.configuration_url.clone()
    }
}

const LOCAL_LAN_URL_KEYS: &[&str] = &[
    "NF4_CONFIGURATION_URL",
    "NF4_NIGHTFALL_PROPOSER__URL",
    "NF4_NIGHTFALL_CLIENT__WEBHOOK_URL",
    "WEBHOOK_URL",
];

/// Rewrite stored local URLs to the current LAN IP. No-op unless `NF4_NETWORK=local`.
pub fn refresh_local_lan_urls() -> Result<(), String> {
    if !local_env_exists() {
        return Ok(());
    }
    let env = read_local_env();
    if env.get("NF4_NETWORK").map(String::as_str) != Some("local") {
        return Ok(());
    }
    let lan = detect_lan_host();
    if lan.starts_with("127.") {
        return Ok(());
    }
    let mut updates = Vec::new();
    for key in LOCAL_LAN_URL_KEYS {
        let Some(value) = env.get(*key) else {
            continue;
        };
        let updated = with_lan_host(value, &lan);
        if updated != *value {
            updates.push((*key, updated));
        }
    }
    if updates.is_empty() {
        return Ok(());
    }
    println!("Updated local LAN URLs to {lan}");
    for (key, value) in &updates {
        println!("  {key}={value}");
    }
    merge_local_env(&updates)
}

pub fn update_nightfall_toml(config: &DeploymentConfig) -> Result<(), String> {
    let source = fs::read_to_string(NIGHTFALL_TOML)
        .map_err(|err| format!("Failed to read {NIGHTFALL_TOML}: {err}"))?;
    let updated = update_nightfall_toml_text(&source, config)?;
    fs::write(NIGHTFALL_TOML, updated)
        .map_err(|err| format!("Failed to write {NIGHTFALL_TOML}: {err}"))
}

pub fn update_docker_compose(config: &DeploymentConfig) -> Result<(), String> {
    let source = fs::read_to_string(DOCKER_COMPOSE_YML)
        .map_err(|err| format!("Failed to read {DOCKER_COMPOSE_YML}: {err}"))?;
    let updated = update_docker_compose_text(&source, &config.profile, config.configuration_port);
    fs::write(DOCKER_COMPOSE_YML, updated)
        .map_err(|err| format!("Failed to write {DOCKER_COMPOSE_YML}: {err}"))
}

pub fn update_proposer_docker_compose(profile: &str, proposer_port: u16) -> Result<(), String> {
    let source = fs::read_to_string(DOCKER_COMPOSE_YML)
        .map_err(|err| format!("Failed to read {DOCKER_COMPOSE_YML}: {err}"))?;
    let updated = update_proposer_docker_compose_text(&source, profile, proposer_port);
    fs::write(DOCKER_COMPOSE_YML, updated)
        .map_err(|err| format!("Failed to write {DOCKER_COMPOSE_YML}: {err}"))
}

pub fn update_client_docker_compose(profile: &str, client_port: u16) -> Result<(), String> {
    let source = fs::read_to_string(DOCKER_COMPOSE_YML)
        .map_err(|err| format!("Failed to read {DOCKER_COMPOSE_YML}: {err}"))?;
    let updated = update_client_docker_compose_text(&source, profile, client_port);
    fs::write(DOCKER_COMPOSE_YML, updated)
        .map_err(|err| format!("Failed to write {DOCKER_COMPOSE_YML}: {err}"))
}

fn update_nightfall_toml_text(source: &str, config: &DeploymentConfig) -> Result<String, String> {
    let mut doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {NIGHTFALL_TOML}: {err}"))?;

    let template_name = config.network.template_profile();
    let template = doc
        .get(template_name)
        .cloned()
        .or_else(|| doc.get("base_sepolia").cloned())
        .or_else(|| doc.get("development").cloned())
        .ok_or_else(|| {
            format!(
                "nightfall.toml must contain [{template_name}], [base_sepolia], or [development]"
            )
        })?;

    doc[&config.profile] = template;
    doc[&config.profile]["signing_key"] = value("Key not set");
    doc[&config.profile]["mock_prover"] = value(config.mock_prover);
    doc[&config.profile]["genesis_block"] = value(config.genesis_block as i64);
    doc[&config.profile]["ethereum_client_url"] = value(config.rpc_url.clone());
    doc[&config.profile]["configuration_url"] = value(config.configuration_url.clone());

    doc[&config.profile]["network"]["chain_id"] = value(config.chain_id as i64);
    doc[&config.profile]["owners"] = owner_table(config);
    doc[&config.profile]["nightfall_deployer"]["default_proposer_address"] =
        value(config.default_proposer_address.clone());
    doc[&config.profile]["nightfall_deployer"]["default_proposer_url"] =
        value(config.default_proposer_url.clone());
    doc[&config.profile]["nightfall_proposer"]["block_size"] = value(config.block_size as i64);
    doc[&config.profile]["contracts"]["deploy_contracts"] = value(true);

    for contract in ["nightfall", "round_robin", "x509", "verifier"] {
        doc[&config.profile]["contracts"]["contract_addresses"][contract] = value("");
    }

    Ok(doc.to_string())
}

fn owner_table(config: &DeploymentConfig) -> Item {
    let mut table = toml_edit::Table::new();
    for key in [
        "vk_provider_owner",
        "x509_owner",
        "verifier_owner",
        "round_robin_owner",
        "nightfall_owner",
    ] {
        table[key] = value(config.deployer_address.clone());
    }
    Item::Table(table)
}

fn update_docker_compose_text(source: &str, profile: &str, configuration_port: u16) -> String {
    let mut output = source.to_string();
    output = replace_service_run_mode(&output, "indie-deployer", profile);
    output = replace_service_run_mode(&output, "configuration", profile);
    output = ensure_configuration_port(&output, configuration_port);
    output
}

fn update_proposer_docker_compose_text(source: &str, profile: &str, proposer_port: u16) -> String {
    let mut output = source.to_string();
    output = replace_service_run_mode(&output, "indie-proposer", profile);
    output = ensure_service_port(&output, "indie-proposer", proposer_port, 3000);
    output = ensure_service_envs(
        &output,
        "indie-proposer",
        &[
            "NF4_ETHEREUM_CLIENT_URL",
            "NF4_CONFIGURATION_URL=${NF4_CONFIGURATION_URL}",
            "NF4_CONTRACTS__DEPLOY_CONTRACTS=${NF4_CONTRACTS__DEPLOY_CONTRACTS:-false}",
        ],
    );
    output
}

fn update_client_docker_compose_text(source: &str, profile: &str, client_port: u16) -> String {
    let mut output = source.to_string();
    output = replace_service_run_mode(&output, "indie-client", profile);
    output = ensure_service_port(&output, "indie-client", client_port, 3000);
    output = ensure_service_envs(
        &output,
        "indie-client",
        &[
            "NF4_SIGNING_KEY=${CLIENT_SIGNING_KEY}",
            "NF4_ETHEREUM_CLIENT_URL",
            "NF4_CONFIGURATION_URL=${NF4_CONFIGURATION_URL}",
            "NF4_NIGHTFALL_PROPOSER__URL=${NF4_NIGHTFALL_PROPOSER__URL}",
            "NF4_NIGHTFALL_CLIENT__WEBHOOK_URL=${NF4_NIGHTFALL_CLIENT__WEBHOOK_URL}",
            "NF4_MOCK_PROVER=${NF4_MOCK_PROVER:-false}",
        ],
    );
    output
}

fn replace_service_run_mode(source: &str, service: &str, profile: &str) -> String {
    let mut output = Vec::new();
    let mut in_service = false;
    let service_header = format!("  {service}:");

    for line in source.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") {
            in_service = line == service_header;
        }

        if in_service && line.trim_start().starts_with("- NF4_RUN_MODE=") {
            let indent = line
                .chars()
                .take_while(|ch| ch.is_whitespace())
                .collect::<String>();
            output.push(format!(
                "{indent}- NF4_RUN_MODE=${{NF4_RUN_MODE:-{profile}}}"
            ));
        } else {
            output.push(line.to_string());
        }
    }

    preserve_trailing_newline(source, output.join("\n"))
}

fn ensure_service_port(source: &str, service: &str, host_port: u16, container_port: u16) -> String {
    let mut output = Vec::new();
    let mut in_service = false;
    let mut in_ports = false;
    let mut inserted = false;
    let service_header = format!("  {service}:");
    let port_line = format!("      - \"{host_port}:{container_port}\"");

    for line in source.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") {
            if in_service && !inserted {
                output.push("    ports:".to_string());
                output.push(port_line.clone());
                inserted = true;
            }
            in_service = line == service_header;
            in_ports = false;
        }

        if in_service {
            let trimmed = line.trim_start();
            if trimmed.starts_with("ports:") {
                in_ports = true;
                output.push(line.to_string());
                continue;
            }

            if in_ports {
                if trimmed.starts_with("- \"") || trimmed.starts_with("- '") {
                    if trimmed.contains(&format!(":{container_port}\""))
                        || trimmed.contains(&format!(":{container_port}'"))
                    {
                        if !inserted {
                            output.push(port_line.clone());
                            inserted = true;
                        }
                        continue;
                    }
                } else if line.starts_with("    ") && !line.starts_with("      ") {
                    if !inserted {
                        output.push(port_line.clone());
                        inserted = true;
                    }
                    in_ports = false;
                }
            }
        }

        output.push(line.to_string());
    }

    if in_service && !inserted {
        output.push("    ports:".to_string());
        output.push(port_line);
    }

    preserve_trailing_newline(source, output.join("\n"))
}

fn ensure_service_envs(source: &str, service: &str, envs: &[&str]) -> String {
    let mut output = Vec::new();
    let mut in_service = false;
    let mut in_environment = false;
    let mut seen = BTreeSet::new();
    let service_header = format!("  {service}:");

    for line in source.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") {
            if in_service && in_environment {
                append_missing_envs(&mut output, envs, &seen);
            }
            in_service = line == service_header;
            in_environment = false;
            seen.clear();
        }

        if in_service {
            let trimmed = line.trim_start();
            if trimmed.starts_with("environment:") {
                in_environment = true;
                output.push(line.to_string());
                continue;
            }

            if in_environment {
                if let Some(env_name) = env_line_name(trimmed) {
                    seen.insert(env_name.to_string());
                    if let Some(replacement) = envs.iter().find(|env| env_name == env_key(env)) {
                        let indent = line
                            .chars()
                            .take_while(|ch| ch.is_whitespace())
                            .collect::<String>();
                        output.push(format!("{indent}- {replacement}"));
                        continue;
                    }
                } else if line.starts_with("    ") && !line.starts_with("      ") {
                    append_missing_envs(&mut output, envs, &seen);
                    in_environment = false;
                }
            }
        }

        output.push(line.to_string());
    }

    if in_service && in_environment {
        append_missing_envs(&mut output, envs, &seen);
    }

    preserve_trailing_newline(source, output.join("\n"))
}

fn append_missing_envs(output: &mut Vec<String>, envs: &[&str], seen: &BTreeSet<String>) {
    for env in envs {
        if !seen.contains(env_key(env)) {
            output.push(format!("      - {env}"));
        }
    }
}

fn env_line_name(line: &str) -> Option<&str> {
    line.strip_prefix("- ")
        .and_then(|env| env.split_once('=').map(|(key, _)| key).or(Some(env)))
}

fn env_key(env: &str) -> &str {
    env.split_once('=').map(|(key, _)| key).unwrap_or(env)
}

fn ensure_configuration_port(source: &str, port: u16) -> String {
    let mut output = Vec::new();
    let mut in_configuration = false;
    let mut inserted_ports = false;

    for line in source.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") {
            if in_configuration && !inserted_ports {
                output.push("    restart: unless-stopped".to_string());
                output.push("    ports:".to_string());
                output.push(format!("      - \"{port}:80\""));
                inserted_ports = true;
            }
            in_configuration = line == "  configuration:";
        }

        if in_configuration {
            let trimmed = line.trim_start();
            if trimmed.starts_with("# restart:")
                || trimmed.starts_with("restart:")
                || trimmed.starts_with("# ports:")
                || trimmed.starts_with("ports:")
                || trimmed.starts_with("#   - \"")
                || trimmed.starts_with("- \"")
            {
                continue;
            }
        }

        output.push(line.to_string());
    }

    if in_configuration && !inserted_ports {
        output.push("    restart: unless-stopped".to_string());
        output.push("    ports:".to_string());
        output.push(format!("      - \"{port}:80\""));
    }

    preserve_trailing_newline(source, output.join("\n"))
}

fn preserve_trailing_newline(source: &str, mut output: String) -> String {
    if source.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
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

fn merge_local_env_text(source: &str, values: &[(&str, String)]) -> String {
    let mut output = Vec::new();
    let mut written = BTreeSet::new();

    for line in source.lines() {
        let trimmed = line.trim();
        let comparable = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let key = comparable
            .split_once('=')
            .map(|(key, _)| key.trim())
            .unwrap_or("");

        if let Some((_, value)) = values.iter().find(|(candidate, _)| *candidate == key) {
            output.push(format!("{key}={}", quote_env_value(value)));
            written.insert(key.to_string());
        } else {
            output.push(line.to_string());
        }
    }

    for (key, value) in values {
        if !written.contains(*key) {
            output.push(format!("{key}={}", quote_env_value(value)));
        }
    }

    preserve_trailing_newline(source, output.join("\n"))
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

fn quote_env_value(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(unix)]
fn restrict_local_env_permissions() -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = fs::Permissions::from_mode(0o600);
    fs::set_permissions(LOCAL_ENV, permissions)
        .map_err(|err| format!("Failed to set permissions on {LOCAL_ENV}: {err}"))
}

#[cfg(not(unix))]
fn restrict_local_env_permissions() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        merge_local_env_text, parse_env_text, update_client_docker_compose_text,
        update_docker_compose_text, update_nightfall_toml_text,
        update_proposer_docker_compose_text,
    };
    use crate::model::DeploymentConfig;
    use crate::network::NetworkKind;

    fn sample_config() -> DeploymentConfig {
        DeploymentConfig {
            network: NetworkKind::Testnet,
            profile: "sepolia".to_string(),
            rpc_url: "wss://example.test".to_string(),
            chain_id: 11155111,
            genesis_block: 1024,
            configuration_url: "http://10.0.0.5:8080".to_string(),
            configuration_port: 8080,
            deployer_signing_key: "0xabc".to_string(),
            deployer_address: "0x1111111111111111111111111111111111111111".to_string(),
            default_proposer_address: "0x2222222222222222222222222222222222222222".to_string(),
            default_proposer_url: "http://10.0.0.5:3001".to_string(),
            mock_prover: false,
            block_size: 64,
        }
    }

    #[test]
    fn updates_nightfall_profile_from_template() {
        let source = r#"
[base_sepolia]
signing_key = "Key not set"
mock_prover = true
genesis_block = 1
ethereum_client_url = "ws://old"
configuration_url = "http://old"

[base_sepolia.network]
chain_id = 1

[base_sepolia.owners]
nightfall_owner = "0x0"

[base_sepolia.nightfall_deployer]
default_proposer_address = "0x0"
default_proposer_url = "http://old"

[base_sepolia.nightfall_proposer]
block_size = 64

[base_sepolia.contracts]
deploy_contracts = false

[base_sepolia.contracts.contract_addresses]
nightfall = "0x123"
round_robin = "0x123"
x509 = "0x123"
verifier = "0x123"
"#;

        let updated = update_nightfall_toml_text(source, &sample_config()).unwrap();
        assert!(updated.contains("[sepolia]"));
        assert!(updated.contains("mock_prover = false"));
        assert!(updated.contains("genesis_block = 1024"));
        assert!(updated.contains("ethereum_client_url = \"wss://example.test\""));
        assert!(updated.contains("configuration_url = \"http://10.0.0.5:8080\""));
        assert!(updated.contains("chain_id = 11155111"));
        assert!(updated.contains("deploy_contracts = true"));
        assert!(updated.contains("nightfall = \"\""));
    }

    #[test]
    fn local_network_clones_development_template() {
        let source = r#"
[development]
mock_prover = true
ethereum_client_url = "ws://anvil:8545"
configuration_url = "http://configuration:80"

[development.network]
chain_id = 31337

[development.owners]
nightfall_owner = "0x0"

[development.nightfall_deployer]
default_proposer_address = "0x0"
default_proposer_url = "http://proposer:3000"

[development.nightfall_proposer]
block_size = 64

[development.contracts]
deploy_contracts = true

[development.contracts.contract_addresses]
nightfall = "0x123"
round_robin = "0x123"
x509 = "0x123"
verifier = "0x123"

[base_sepolia]
mock_prover = false
ethereum_client_url = "ws://old"
configuration_url = "http://old"

[base_sepolia.network]
chain_id = 84532

[base_sepolia.owners]
nightfall_owner = "0x0"

[base_sepolia.nightfall_deployer]
default_proposer_address = "0x0"
default_proposer_url = "http://old"

[base_sepolia.nightfall_proposer]
block_size = 256

[base_sepolia.contracts]
deploy_contracts = false

[base_sepolia.contracts.contract_addresses]
nightfall = "0xabc"
round_robin = "0xabc"
x509 = "0xabc"
verifier = "0xabc"
"#;

        let mut config = sample_config();
        config.network = NetworkKind::Local;
        config.profile = "local".to_string();
        config.rpc_url = "ws://127.0.0.1:8545".to_string();
        config.configuration_url = "http://127.0.0.1:8080".to_string();
        config.chain_id = 31337;
        config.mock_prover = true;

        let updated = update_nightfall_toml_text(source, &config).unwrap();
        assert!(updated.contains("[local]"));
        assert!(updated.contains("ethereum_client_url = \"ws://127.0.0.1:8545\""));
        assert!(updated.contains("configuration_url = \"http://127.0.0.1:8080\""));
        assert!(updated.contains("chain_id = 31337"));
        assert!(updated.contains("mock_prover = true"));
    }

    #[test]
    fn updates_compose_run_modes_and_configuration_port() {
        let source = r#"services:
  indie-deployer:
    environment:
      - NF4_RUN_MODE=${NF4_RUN_MODE:-base_sepolia}
  configuration:
    # restart: unless-stopped
    # ports:
    #   - "8080:80"
    environment:
      - NF4_RUN_MODE=${NF4_RUN_MODE:-development}
  anvil:
    image: test
"#;

        let updated = update_docker_compose_text(source, "sepolia", 9090);
        assert!(updated.contains("- NF4_RUN_MODE=${NF4_RUN_MODE:-sepolia}"));
        assert!(updated.contains("restart: unless-stopped"));
        assert!(updated.contains("- \"9090:80\""));
        assert!(!updated.contains("# ports:"));
    }

    #[test]
    fn merges_local_env_without_dropping_existing_values() {
        let source = r#"DEPLOYER_SIGNING_KEY="0xaaa"
NF4_RUN_MODE="sepolia"
PROPOSER_SIGNING_KEY="old"
"#;

        let updated = merge_local_env_text(
            source,
            &[
                ("PROPOSER_SIGNING_KEY", "0xbbb".to_string()),
                ("NF4_CONFIGURATION_URL", "http://10.0.0.8:8080".to_string()),
            ],
        );

        assert!(updated.contains("DEPLOYER_SIGNING_KEY=\"0xaaa\""));
        assert!(updated.contains("NF4_RUN_MODE=\"sepolia\""));
        assert!(updated.contains("PROPOSER_SIGNING_KEY=\"0xbbb\""));
        assert!(updated.contains("NF4_CONFIGURATION_URL=\"http://10.0.0.8:8080\""));
    }

    #[test]
    fn parses_local_env_values() {
        let values = parse_env_text(
            r#"
export NF4_RUN_MODE="sepolia"
NF4_MOCK_PROVER='true'
"#,
        );

        assert_eq!(values.get("NF4_RUN_MODE").unwrap(), "sepolia");
        assert_eq!(values.get("NF4_MOCK_PROVER").unwrap(), "true");
    }

    #[test]
    fn updates_proposer_compose_defaults() {
        let source = r#"services:
  indie-proposer:
    ports:
      - "3001:3000"
    environment:
      - NF4_RUN_MODE=${NF4_RUN_MODE:-base_sepolia}
      - NF4_CONFIGURATION_URL=${NF4_CONFIGURATION_URL:-http://configuration:80}
  db:
    image: mongo
"#;

        let updated = update_proposer_docker_compose_text(source, "sepolia", 4001);

        assert!(updated.contains("- \"4001:3000\""));
        assert!(updated.contains("- NF4_RUN_MODE=${NF4_RUN_MODE:-sepolia}"));
        assert!(updated.contains("- NF4_CONFIGURATION_URL=${NF4_CONFIGURATION_URL}"));
        assert!(updated.contains("- NF4_ETHEREUM_CLIENT_URL"));
        assert!(updated.contains(
            "- NF4_CONTRACTS__DEPLOY_CONTRACTS=${NF4_CONTRACTS__DEPLOY_CONTRACTS:-false}"
        ));
    }

    #[test]
    fn updates_client_compose_defaults() {
        let source = r#"services:
  indie-client:
    ports:
      - "3000:3000"
    environment:
      - NF4_RUN_MODE=${NF4_RUN_MODE:-base_sepolia}
      - NF4_NIGHTFALL_PROPOSER__URL=${NF4_NIGHTFALL_PROPOSER__URL:-localhost:3001}
  db:
    image: mongo
"#;

        let updated = update_client_docker_compose_text(source, "sepolia", 4000);

        assert!(updated.contains("- \"4000:3000\""));
        assert!(updated.contains("- NF4_RUN_MODE=${NF4_RUN_MODE:-sepolia}"));
        assert!(updated.contains("- NF4_SIGNING_KEY=${CLIENT_SIGNING_KEY}"));
        assert!(updated.contains("- NF4_CONFIGURATION_URL=${NF4_CONFIGURATION_URL}"));
        assert!(updated.contains("- NF4_NIGHTFALL_PROPOSER__URL=${NF4_NIGHTFALL_PROPOSER__URL}"));
        assert!(
            updated.contains(
                "- NF4_NIGHTFALL_CLIENT__WEBHOOK_URL=${NF4_NIGHTFALL_CLIENT__WEBHOOK_URL}"
            )
        );
        assert!(updated.contains("- NF4_ETHEREUM_CLIENT_URL"));
    }
}
