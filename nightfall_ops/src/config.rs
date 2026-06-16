use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use toml_edit::{DocumentMut, Item, value};

use crate::model::DeploymentConfig;

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
    let contents = format!(
        "DEPLOYER_SIGNING_KEY=\"{}\"\nNF4_ETHEREUM_CLIENT_URL=\"{}\"\nNF4_RUN_MODE=\"{}\"\nNF4_CONTRACTS__DEPLOY_CONTRACTS=\"true\"\nNF4_MOCK_PROVER=\"{}\"\n",
        config.deployer_signing_key, config.rpc_url, config.profile, config.mock_prover
    );

    fs::write(LOCAL_ENV, contents).map_err(|err| format!("Failed to write {LOCAL_ENV}: {err}"))?;
    restrict_local_env_permissions()?;
    Ok(())
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

fn update_nightfall_toml_text(source: &str, config: &DeploymentConfig) -> Result<String, String> {
    let mut doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {NIGHTFALL_TOML}: {err}"))?;

    let template = doc
        .get("base_sepolia")
        .cloned()
        .or_else(|| doc.get("development").cloned())
        .ok_or_else(|| "nightfall.toml must contain [base_sepolia] or [development]".to_string())?;

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
    use super::{update_docker_compose_text, update_nightfall_toml_text};
    use crate::model::DeploymentConfig;

    fn sample_config() -> DeploymentConfig {
        DeploymentConfig {
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
}
