use crate::{checks, config};

pub fn deploy() -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }

    println!("Nightfall deployment uses a configuration service.");
    println!();
    println!("The configuration service hosts:");
    println!("- deployed contract addresses");
    println!("- deployed contract hashes");
    println!("- proving keys");
    println!();
    checks::deployer()?;
    println!();
    println!("Fresh deployment configuration collection is not implemented yet.");
    Ok(())
}
