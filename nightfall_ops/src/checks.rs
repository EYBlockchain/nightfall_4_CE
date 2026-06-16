use std::process::Command;
use std::{path::PathBuf, process::Output};

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
    let results = vec![
        repo_files(),
        command_check("cargo", "cargo", &["--version"]),
    ];
    print_results("Checking prover prerequisites", &results);
    if !results.iter().all(|result| result.ok) {
        return Err("Prover prerequisite checks failed.".to_string());
    }

    println!();
    println!("Checking recursive prover capability...");
    println!(
        "This runs Nightfish test_preprocess_and_prove from the jf-plonk revision pinned by Cargo.lock."
    );

    let package = jf_plonk_package_from_cargo_metadata()?;
    let plonk_dir = package
        .manifest_path
        .parent()
        .ok_or_else(|| "jf-plonk manifest path has no parent directory.".to_string())?;
    let test_file = plonk_dir.join("src/recursion/mod.rs");
    if !test_file.is_file() {
        return Err(format!(
            "Could not find Nightfish recursion test file at {}.",
            test_file.display()
        ));
    }

    let mut args = vec![
        "test".to_string(),
        "test_preprocess_and_prove".to_string(),
        "--release".to_string(),
    ];
    if package.has_test_srs_feature {
        args.extend(["--features".to_string(), "test-srs".to_string()]);
    }
    args.extend(["--".to_string(), "--nocapture".to_string()]);

    println!("  cd {}", plonk_dir.display());
    println!("  cargo {}", args.join(" "));

    let status = Command::new("cargo")
        .current_dir(plonk_dir)
        .args(&args)
        .status()
        .map_err(|err| format!("Failed to run Nightfish prover test: {err}"))?;

    if !status.success() {
        return Err(format!("Nightfish prover test exited with {status}"));
    }

    println!("Prover capability check OK.");
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

fn jf_plonk_package_from_cargo_metadata() -> Result<NightfishPackage, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--locked"])
        .output()
        .map_err(|err| format!("Failed to run cargo metadata: {err}"))?;

    if !output.status.success() {
        return Err(command_detail(&output).unwrap_or_else(|| "cargo metadata failed".to_string()));
    }

    jf_plonk_package_from_metadata_json(&output.stdout)
}

fn jf_plonk_package_from_metadata_json(bytes: &[u8]) -> Result<NightfishPackage, String> {
    let metadata: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|err| format!("Failed to parse cargo metadata JSON: {err}"))?;
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "cargo metadata JSON did not contain a packages array.".to_string())?;

    let package = packages
        .iter()
        .find(|package| package.get("name").and_then(serde_json::Value::as_str) == Some("jf-plonk"))
        .ok_or_else(|| {
            "Could not find jf-plonk in cargo metadata. Run cargo fetch and try again.".to_string()
        })?;

    let manifest_path = package
        .get("manifest_path")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "jf-plonk package metadata did not contain manifest_path.".to_string())?;
    let has_test_srs_feature = package
        .get("features")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|features| features.contains_key("test-srs"));

    Ok(NightfishPackage {
        manifest_path: PathBuf::from(manifest_path),
        has_test_srs_feature,
    })
}

fn command_detail(output: &Output) -> Option<String> {
    first_line(&output.stderr).or_else(|| first_line(&output.stdout))
}

fn print_results(title: &str, results: &[CheckResult]) {
    println!("{title}...");
    for result in results {
        let status = if result.ok { "OK" } else { "FAILED" };
        println!("  {status:<6} {} - {}", result.name, result.detail);
    }
}

struct NightfishPackage {
    manifest_path: PathBuf,
    has_test_srs_feature: bool,
}

#[cfg(test)]
mod tests {
    use super::jf_plonk_package_from_metadata_json;

    #[test]
    fn finds_jf_plonk_package_in_cargo_metadata() {
        let metadata = br#"
{
  "packages": [
    {
      "name": "other",
      "manifest_path": "/tmp/other/Cargo.toml",
      "features": {}
    },
    {
      "name": "jf-plonk",
      "manifest_path": "/tmp/nightfish_CE/plonk/Cargo.toml",
      "features": {
        "test-srs": []
      }
    }
  ]
}
"#;

        let package = jf_plonk_package_from_metadata_json(metadata).unwrap();
        assert_eq!(
            package.manifest_path,
            std::path::PathBuf::from("/tmp/nightfish_CE/plonk/Cargo.toml")
        );
        assert!(package.has_test_srs_feature);
    }
}
