use std::process::Command;

use crate::config;

pub fn up_configuration() -> Result<(), String> {
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

pub fn build_indie_deployer() -> Result<(), String> {
    let args = compose_args("indie-deployer", false, &["build"]);
    run_docker_compose("Building indie deployer image", &args)
}

pub fn build_indie_proposer() -> Result<(), String> {
    let args = compose_args("indie-proposer", true, &["build"]);
    run_docker_compose("Building indie proposer image", &args)
}

pub fn up_indie_deployer() -> Result<(), String> {
    let args = compose_args("indie-deployer", true, &["up"]);
    run_docker_compose("Running indie deployer", &args)
}

pub fn build_configuration() -> Result<(), String> {
    let args = compose_args("configuration", false, &["build"]);
    run_docker_compose("Building configuration service image", &args)
}

fn run_docker_compose(title: &str, args: &[String]) -> Result<(), String> {
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
}
