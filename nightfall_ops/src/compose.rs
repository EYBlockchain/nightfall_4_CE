use std::process::Command;

use crate::config;

pub fn up_configuration() -> Result<(), String> {
    let args = compose_args(config::local_env_exists(), &["up", "-d", "configuration"]);
    run_docker_compose("Starting configuration service", &args)
}

pub fn logs_configuration() -> Result<(), String> {
    let args = compose_args(config::local_env_exists(), &["logs", "configuration"]);
    run_docker_compose("Showing configuration service logs", &args)
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

fn compose_args(include_env_file: bool, command_args: &[&str]) -> Vec<String> {
    let mut args = vec![
        "compose".to_string(),
        "--profile".to_string(),
        "configuration".to_string(),
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
            compose_args(false, &["up", "-d", "configuration"]),
            vec![
                "compose",
                "--profile",
                "configuration",
                "up",
                "-d",
                "configuration"
            ]
        );
    }

    #[test]
    fn builds_configuration_compose_args_with_env_file() {
        assert_eq!(
            compose_args(true, &["logs", "configuration"]),
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
}
