use std::process::Command;

use crate::config;

struct CheckResult {
    name: &'static str,
    ok: bool,
    detail: String,
}

pub fn deployer() -> Result<(), String> {
    let results = vec![
        repo_files(),
        command_check("docker", "docker", &["--version"]),
        command_check("docker compose", "docker", &["compose", "version"]),
        command_check("forge", "forge", &["--version"]),
        command_check("cast", "cast", &["--version"]),
        command_check("cargo", "cargo", &["--version"]),
        command_check("curl", "curl", &["--version"]),
    ];

    print_results("Checking deployer prerequisites", &results);

    if results.iter().all(|result| result.ok) {
        Ok(())
    } else {
        Err("Deployer prerequisite checks failed.".to_string())
    }
}

pub fn prover() -> Result<(), String> {
    println!("Checking prover capability...");
    println!("Not implemented yet.");
    Ok(())
}

fn repo_files() -> CheckResult {
    CheckResult {
        name: "repository files",
        ok: config::required_repo_files_exist(),
        detail: format!(
            "{} and {}",
            config::NIGHTFALL_TOML,
            config::DOCKER_COMPOSE_YML
        ),
    }
}

fn command_check(name: &'static str, program: &str, args: &[&str]) -> CheckResult {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => CheckResult {
            name,
            ok: true,
            detail: first_line(&output.stdout).unwrap_or_else(|| "available".to_string()),
        },
        Ok(output) => CheckResult {
            name,
            ok: false,
            detail: first_line(&output.stderr)
                .or_else(|| first_line(&output.stdout))
                .unwrap_or_else(|| format!("exited with {}", output.status)),
        },
        Err(err) => CheckResult {
            name,
            ok: false,
            detail: err.to_string(),
        },
    }
}

fn first_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}

fn print_results(title: &str, results: &[CheckResult]) {
    println!("{title}...");
    for result in results {
        let status = if result.ok { "OK" } else { "FAILED" };
        println!("  {status:<6} {} - {}", result.name, result.detail);
    }
}
