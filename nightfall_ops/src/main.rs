mod checks;
mod cli;
mod client;
mod compose;
mod config;
mod model;
mod proposer;
mod status;
mod validation;
mod webhook;
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
        Command::WizardProposer => proposer::wizard(),
        Command::WizardClient => client::wizard(),
        Command::CheckDeployer => checks::deployer(),
        Command::CheckProver => checks::prover(),
        Command::UpConfiguration => compose::up_configuration(),
        Command::UpProposer => compose::up_proposer(),
        Command::UpClient => compose::up_client(),
        Command::Status => status::print(),
        Command::LogsConfiguration => compose::logs_configuration(),
        Command::LogsProposer => compose::logs_proposer(),
        Command::LogsClient => compose::logs_client(),
        Command::WebhookStart { port } => webhook::start_command(port),
        Command::WebhookServe { port } => webhook::serve(port),
        Command::WebhookStatus => webhook::status(),
        Command::WebhookLogs => webhook::logs(),
        Command::WebhookEvents => webhook::events(),
        Command::WebhookSalts => webhook::salts(),
    }
}
