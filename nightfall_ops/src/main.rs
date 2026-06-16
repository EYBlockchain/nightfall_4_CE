mod checks;
mod cli;
mod compose;
mod config;
mod status;
mod wizard;

use cli::Command;
use std::io::Write;

fn main() {
    if let Err(err) = run(std::env::args().skip(1)) {
        let _ = std::io::stdout().flush();
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    match cli::parse(args)? {
        Command::Help => {
            println!("{}", cli::usage());
            Ok(())
        }
        Command::WizardDeploy => wizard::deploy(),
        Command::CheckDeployer => checks::deployer(),
        Command::CheckProver => checks::prover(),
        Command::UpConfiguration => compose::up_configuration(),
        Command::Status => status::print(),
        Command::LogsConfiguration => compose::logs_configuration(),
    }
}
