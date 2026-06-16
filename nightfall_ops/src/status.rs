use crate::config;

pub fn print() -> Result<(), String> {
    println!("Nightfall testnet status");
    println!();
    println!(
        "Repository files: {}",
        if config::required_repo_files_exist() {
            "OK"
        } else {
            "FAILED"
        }
    );
    Ok(())
}
