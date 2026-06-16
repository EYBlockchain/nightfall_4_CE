use std::process::Command;

const CONFIGURATION_ENDPOINTS: &[(&str, &str)] = &[
    ("addresses.toml", "configuration/toml/addresses.toml"),
    (
        "contract_hashes.toml",
        "configuration/toml/contract_hashes.toml",
    ),
    ("proving_key", "configuration/bin/keys/proving_key"),
];

pub fn configuration_endpoints(configuration_url: &str) -> Result<(), String> {
    println!("Checking configuration service...");

    let mut failures = Vec::new();
    for (label, path) in CONFIGURATION_ENDPOINTS {
        let url = format!("{}/{}", configuration_url.trim_end_matches('/'), path);
        match curl_head_or_get(&url) {
            Ok(()) => println!("  {label}: OK"),
            Err(err) => {
                println!("  {label}: FAILED - {err}");
                failures.push(format!("{label}: {err}"));
            }
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

fn curl_head_or_get(url: &str) -> Result<(), String> {
    let head = Command::new("curl")
        .args(["-fsSI", "--max-time", "20", url])
        .status()
        .map_err(|err| format!("failed to run curl: {err}"))?;

    if head.success() {
        return Ok(());
    }

    let get = Command::new("curl")
        .args(["-fsS", "--max-time", "20", "-o", "/dev/null", url])
        .status()
        .map_err(|err| format!("failed to run curl: {err}"))?;

    if get.success() {
        Ok(())
    } else {
        Err(format!("curl could not fetch {url}"))
    }
}
