use std::process::Command;

const CONFIGURATION_ENDPOINTS: &[(&str, &str)] = &[
    ("addresses.toml", "configuration/toml/addresses.toml"),
    (
        "contract_hashes.toml",
        "configuration/toml/contract_hashes.toml",
    ),
    ("proving_key", "configuration/bin/keys/proving_key"),
];

pub struct EndpointCheck {
    pub label: &'static str,
    pub url: String,
    pub ok: bool,
    pub detail: String,
}

pub fn configuration_endpoints(configuration_url: &str) -> Result<(), String> {
    println!("Checking configuration service...");

    let mut failures = Vec::new();
    for check in configuration_endpoint_checks_with_timeout(configuration_url, "20") {
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
            "Configuration endpoint validation failed: {}",
            failures.join("; ")
        ))
    }
}

pub fn configuration_endpoint_checks(configuration_url: &str) -> Vec<EndpointCheck> {
    configuration_endpoint_checks_with_timeout(configuration_url, "5")
}

fn configuration_endpoint_checks_with_timeout(
    configuration_url: &str,
    timeout_secs: &str,
) -> Vec<EndpointCheck> {
    CONFIGURATION_ENDPOINTS
        .iter()
        .map(|(label, path)| {
            let url = format!("{}/{}", configuration_url.trim_end_matches('/'), path);
            match curl_head_or_get(&url, timeout_secs) {
                Ok(()) => EndpointCheck {
                    label,
                    url,
                    ok: true,
                    detail: "reachable".to_string(),
                },
                Err(detail) => EndpointCheck {
                    label,
                    url,
                    ok: false,
                    detail,
                },
            }
        })
        .collect()
}

fn curl_head_or_get(url: &str, timeout_secs: &str) -> Result<(), String> {
    let head = Command::new("curl")
        .args(["-fsSI", "--max-time", timeout_secs, url])
        .output()
        .map_err(|err| format!("failed to run curl: {err}"))?;

    if head.status.success() {
        return Ok(());
    }

    let get = Command::new("curl")
        .args(["-fsS", "--max-time", timeout_secs, "-o", "/dev/null", url])
        .output()
        .map_err(|err| format!("failed to run curl: {err}"))?;

    if get.status.success() {
        Ok(())
    } else {
        Err(command_detail(&get).unwrap_or_else(|| format!("curl could not fetch {url}")))
    }
}

fn command_detail(output: &std::process::Output) -> Option<String> {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .chain(String::from_utf8_lossy(&output.stdout).lines())
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}
