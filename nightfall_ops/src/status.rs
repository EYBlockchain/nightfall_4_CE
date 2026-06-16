use crate::config;
use std::path::Path;

pub fn print() -> Result<(), String> {
    println!("Nightfall testnet status");
    println!();

    print_file_check("nightfall.toml", config::NIGHTFALL_TOML);
    print_file_check("docker-compose.yml", config::DOCKER_COMPOSE_YML);
    print_file_check("local.env", config::LOCAL_ENV);

    println!();
    println!("Deployment metadata:");
    print_file_check("addresses.toml", "configuration/toml/addresses.toml");
    print_file_check(
        "contract_hashes.toml",
        "configuration/toml/contract_hashes.toml",
    );
    print_file_check("proving_key", "configuration/bin/keys/proving_key");

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
