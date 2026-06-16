#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    WizardDeploy,
    WizardProposer,
    CheckDeployer,
    CheckProver,
    UpConfiguration,
    UpProposer,
    Status,
    LogsConfiguration,
    LogsProposer,
}

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Command, String> {
    let args = args.into_iter().collect::<Vec<_>>();
    let parts = args.iter().map(String::as_str).collect::<Vec<_>>();

    match parts.as_slice() {
        [] | ["help"] | ["--help"] | ["-h"] => Ok(Command::Help),
        ["wizard", "deploy"] => Ok(Command::WizardDeploy),
        ["wizard", "proposer"] => Ok(Command::WizardProposer),
        ["check", "deployer"] => Ok(Command::CheckDeployer),
        ["check", "prover"] => Ok(Command::CheckProver),
        ["up", "configuration"] => Ok(Command::UpConfiguration),
        ["up", "proposer"] => Ok(Command::UpProposer),
        ["status"] => Ok(Command::Status),
        ["logs", "configuration"] => Ok(Command::LogsConfiguration),
        ["logs", "proposer"] => Ok(Command::LogsProposer),
        _ => Err(format!(
            "Unknown command: {}\n\n{}",
            args.join(" "),
            usage()
        )),
    }
}

pub fn usage() -> &'static str {
    r#"Nightfall deployment assistant

Usage:
  nf4 wizard deploy
  nf4 wizard proposer
  nf4 check deployer
  nf4 check prover
  nf4 up configuration
  nf4 up proposer
  nf4 status
  nf4 logs configuration
  nf4 logs proposer

Commands:
  nf4 wizard deploy       Collects required inputs, writes config, deploys contracts, starts configuration, and validates hosted metadata.
  nf4 wizard proposer     Reuses deployment metadata, configures proposer values, starts indie-proposer, and validates health.
  nf4 check deployer      Checks local tools required before deployment.
  nf4 check prover        Runs the pinned Nightfish recursive prover capability test.
  nf4 up configuration    Starts only the configuration service.
  nf4 up proposer         Starts only the proposer service.
  nf4 status              Prints deployment, service, metadata, RPC, and contract checks.
  nf4 logs configuration  Shows configuration service logs.
  nf4 logs proposer       Shows proposer service logs.

The main operator command is:
  nf4 wizard deploy
"#
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parses_known_commands() {
        assert_eq!(
            parse(args(&["wizard", "deploy"])),
            Ok(Command::WizardDeploy)
        );
        assert_eq!(
            parse(args(&["wizard", "proposer"])),
            Ok(Command::WizardProposer)
        );
        assert_eq!(
            parse(args(&["check", "deployer"])),
            Ok(Command::CheckDeployer)
        );
        assert_eq!(parse(args(&["check", "prover"])), Ok(Command::CheckProver));
        assert_eq!(
            parse(args(&["up", "configuration"])),
            Ok(Command::UpConfiguration)
        );
        assert_eq!(parse(args(&["up", "proposer"])), Ok(Command::UpProposer));
        assert_eq!(parse(args(&["status"])), Ok(Command::Status));
        assert_eq!(
            parse(args(&["logs", "configuration"])),
            Ok(Command::LogsConfiguration)
        );
        assert_eq!(
            parse(args(&["logs", "proposer"])),
            Ok(Command::LogsProposer)
        );
    }

    #[test]
    fn parses_help() {
        assert_eq!(parse(args(&[])), Ok(Command::Help));
        assert_eq!(parse(args(&["--help"])), Ok(Command::Help));
    }

    #[test]
    fn rejects_unknown_commands() {
        assert!(parse(args(&["wizard"])).is_err());
        assert!(parse(args(&["up", "client"])).is_err());
    }
}
