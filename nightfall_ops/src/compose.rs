use std::process::Command;

use crate::config;

pub fn up_configuration() -> Result<(), String> {
    config::refresh_local_lan_urls()?;
    let args = compose_args("configuration", config::local_env_exists(), &["up", "-d"]);
    run_docker_compose("Starting configuration service", &args)
}

pub fn logs_configuration() -> Result<(), String> {
    let args = compose_args(
        "configuration",
        config::local_env_exists(),
        &["logs", "configuration"],
    );
    run_docker_compose("Showing configuration service logs", &args)
}

pub fn up_proposer() -> Result<(), String> {
    config::refresh_local_lan_urls()?;
    let args = compose_args("indie-proposer", true, &["up", "-d"]);
    run_docker_compose("Starting indie proposer service", &args)
}

pub fn logs_proposer() -> Result<(), String> {
    let args = compose_args(
        "indie-proposer",
        config::local_env_exists(),
        &["logs", "-f", "indie-proposer"],
    );
    run_docker_compose("Showing proposer service logs", &args)
}

pub fn up_client() -> Result<(), String> {
    config::refresh_local_lan_urls()?;
    let args = compose_args("indie-client", true, &["up", "-d"]);
    run_docker_compose("Starting indie client service", &args)
}

pub fn logs_client() -> Result<(), String> {
    let args = compose_args(
        "indie-client",
        config::local_env_exists(),
        &["logs", "-f", "indie-client"],
    );
    run_docker_compose("Showing client service logs", &args)
}

const ANVIL_ACCOUNT1_KEY: &str =
    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const ANVIL_ACCOUNT1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

pub fn build_indie_client2() -> Result<(), String> {
    let args = compose_args("indie-client2", true, &["build"]);
    run_docker_compose("Building indie client 2 image", &args)
}

pub fn up_client2() -> Result<(), String> {
    if !config::local_env_exists() {
        return Err("local.env was not found. Run nf4 wizard client first.".to_string());
    }
    config::refresh_local_lan_urls()?;
    ensure_client2_signing_key()?;
    // Client 2 is a separate image; without a rebuild it keeps a stale proving_key
    // from an earlier keygen and the proposer rejects transfers (zk proof is wrong).
    build_indie_client2()?;
    let args = compose_args(
        "indie-client2",
        true,
        &["up", "-d", "--force-recreate", "--no-deps"],
    );
    run_docker_compose("Starting indie client 2 service", &args)
}

pub fn logs_client2() -> Result<(), String> {
    let args = compose_args(
        "indie-client2",
        config::local_env_exists(),
        &["logs", "-f", "indie-client2"],
    );
    run_docker_compose("Showing client 2 service logs", &args)
}

fn ensure_client2_signing_key() -> Result<(), String> {
    let env = config::read_local_env();
    let has_key = env
        .get("CLIENT2_SIGNING_KEY")
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    if has_key {
        return Ok(());
    }
    if env.get("NF4_NETWORK").map(String::as_str) != Some("local") {
        return Err(
            "CLIENT2_SIGNING_KEY is missing in local.env. Set it before starting client 2."
                .to_string(),
        );
    }

    println!("CLIENT2_SIGNING_KEY was empty; using Anvil account 1 for local client 2");
    config::merge_local_env(&[
        ("CLIENT2_SIGNING_KEY", ANVIL_ACCOUNT1_KEY.to_string()),
        ("CLIENT2_ADDRESS", ANVIL_ACCOUNT1_ADDRESS.to_string()),
    ])
}

pub fn build_indie_deployer() -> Result<(), String> {
    let args = compose_args("indie-deployer", false, &["build"]);
    run_docker_compose("Building indie deployer image", &args)
}

pub fn build_indie_proposer() -> Result<(), String> {
    let args = compose_args("indie-proposer", true, &["build"]);
    run_docker_compose("Building indie proposer image", &args)
}

pub fn build_indie_client() -> Result<(), String> {
    let args = compose_args("indie-client", true, &["build"]);
    run_docker_compose("Building indie client image", &args)
}

pub fn up_indie_deployer() -> Result<(), String> {
    config::refresh_local_lan_urls()?;
    let args = compose_args("indie-deployer", true, &["up", "--no-recreate"]);
    run_docker_compose("Running indie deployer", &args)
}

pub fn build_configuration() -> Result<(), String> {
    let args = compose_args("configuration", false, &["build"]);
    run_docker_compose("Building configuration service image", &args)
}

pub fn ensure_nightfall_network() -> Result<(), String> {
    let network_name = "nightfall_4_ce_nightfall_network";
    let check = Command::new("docker")
        .args(["network", "inspect", network_name])
        .output()
        .map_err(|err| format!("Failed to inspect docker network {network_name}: {err}"))?;

    if check.status.success() {
        return Ok(());
    }

    println!("Creating external docker network {network_name}...");
    let create = Command::new("docker")
        .args([
            "network",
            "create",
            "--driver",
            "bridge",
            "--subnet=172.28.0.0/24",
            network_name,
        ])
        .status()
        .map_err(|err| format!("Failed to create docker network {network_name}: {err}"))?;

    if create.success() {
        Ok(())
    } else {
        Err(format!("Failed to create docker network {network_name}"))
    }
}

fn run_docker_compose(title: &str, args: &[String]) -> Result<(), String> {
    ensure_nightfall_network()?;
    println!("{title}...");
    println!("  docker {}", args.join(" "));

    let status = Command::new("docker")
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run docker compose: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("docker compose exited with {status}"))
    }
}

fn compose_args(profile: &str, include_env_file: bool, command_args: &[&str]) -> Vec<String> {
    let mut args = vec![
        "compose".to_string(),
        "--profile".to_string(),
        profile.to_string(),
    ];

    if include_env_file {
        args.extend(["--env-file".to_string(), config::LOCAL_ENV.to_string()]);
    }

    args.extend(command_args.iter().map(|arg| arg.to_string()));
    args
}

#[cfg(test)]
mod tests {
    use super::compose_args;

    #[test]
    fn builds_configuration_compose_args_without_env_file() {
        assert_eq!(
            compose_args("configuration", false, &["up", "-d"]),
            vec!["compose", "--profile", "configuration", "up", "-d"]
        );
    }

    #[test]
    fn builds_configuration_compose_args_with_env_file() {
        assert_eq!(
            compose_args("configuration", true, &["logs", "configuration"]),
            vec![
                "compose",
                "--profile",
                "configuration",
                "--env-file",
                "local.env",
                "logs",
                "configuration"
            ]
        );
    }

    #[test]
    fn builds_proposer_compose_args_with_env_file() {
        assert_eq!(
            compose_args("indie-proposer", true, &["up", "-d"]),
            vec![
                "compose",
                "--profile",
                "indie-proposer",
                "--env-file",
                "local.env",
                "up",
                "-d"
            ]
        );
    }

    #[test]
    fn builds_client_compose_args_with_env_file() {
        assert_eq!(
            compose_args("indie-client", true, &["up", "-d"]),
            vec![
                "compose",
                "--profile",
                "indie-client",
                "--env-file",
                "local.env",
                "up",
                "-d"
            ]
        );
    }

    #[test]
    fn builds_client2_compose_args_with_env_file() {
        assert_eq!(
            compose_args("indie-client2", true, &["up", "-d"]),
            vec![
                "compose",
                "--profile",
                "indie-client2",
                "--env-file",
                "local.env",
                "up",
                "-d"
            ]
        );
    }

    #[test]
    fn builds_indie_deployer_compose_args_with_no_recreate() {
        assert_eq!(
            compose_args("indie-deployer", true, &["up", "--no-recreate"]),
            vec![
                "compose",
                "--profile",
                "indie-deployer",
                "--env-file",
                "local.env",
                "up",
                "--no-recreate"
            ]
        );
    }
}
