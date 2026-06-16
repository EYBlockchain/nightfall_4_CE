#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    WizardDeploy,
    CheckDeployer,
    CheckProver,
    UpConfiguration,
    Status,
    LogsConfiguration,
}

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Command, String> {
    let args = args.into_iter().collect::<Vec<_>>();
    let parts = args.iter().map(String::as_str).collect::<Vec<_>>();

    match parts.as_slice() {
        [] | ["help"] | ["--help"] | ["-h"] => Ok(Command::Help),
        ["wizard", "deploy"] => Ok(Command::WizardDeploy),
        ["check", "deployer"] => Ok(Command::CheckDeployer),
        ["check", "prover"] => Ok(Command::CheckProver),
        ["up", "configuration"] => Ok(Command::UpConfiguration),
        ["status"] => Ok(Command::Status),
        ["logs", "configuration"] => Ok(Command::LogsConfiguration),
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
  nf4 check deployer
  nf4 check prover
  nf4 up configuration
  nf4 status
  nf4 logs configuration

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
            parse(args(&["check", "deployer"])),
            Ok(Command::CheckDeployer)
        );
        assert_eq!(parse(args(&["check", "prover"])), Ok(Command::CheckProver));
        assert_eq!(
            parse(args(&["up", "configuration"])),
            Ok(Command::UpConfiguration)
        );
        assert_eq!(parse(args(&["status"])), Ok(Command::Status));
        assert_eq!(
            parse(args(&["logs", "configuration"])),
            Ok(Command::LogsConfiguration)
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
