#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    WizardDeploy,
    WizardProposer,
    WizardClient,
    CheckDeployer,
    CheckProver,
    UpConfiguration,
    UpProposer,
    UpClient,
    Status,
    LogsConfiguration,
    LogsProposer,
    LogsClient,
    ClientDeployMockTokens,
    WebhookStart { port: Option<u16> },
    WebhookServe { port: u16 },
    WebhookStatus,
    WebhookLogs,
    WebhookEvents,
    WebhookSalts,
}

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Command, String> {
    let args = args.into_iter().collect::<Vec<_>>();
    let parts = args.iter().map(String::as_str).collect::<Vec<_>>();

    match parts.as_slice() {
        [] | ["help"] | ["--help"] | ["-h"] => Ok(Command::Help),
        ["wizard", "deploy"] => Ok(Command::WizardDeploy),
        ["wizard", "proposer"] => Ok(Command::WizardProposer),
        ["wizard", "client"] => Ok(Command::WizardClient),
        ["check", "deployer"] => Ok(Command::CheckDeployer),
        ["check", "prover"] => Ok(Command::CheckProver),
        ["up", "configuration"] => Ok(Command::UpConfiguration),
        ["up", "proposer"] => Ok(Command::UpProposer),
        ["up", "client"] => Ok(Command::UpClient),
        ["status"] => Ok(Command::Status),
        ["logs", "configuration"] => Ok(Command::LogsConfiguration),
        ["logs", "proposer"] => Ok(Command::LogsProposer),
        ["logs", "client"] => Ok(Command::LogsClient),
        ["client", "deploy-mock-tokens"] => Ok(Command::ClientDeployMockTokens),
        ["webhook", "start"] => Ok(Command::WebhookStart { port: None }),
        ["webhook", "start", port] => Ok(Command::WebhookStart {
            port: Some(parse_port(port)?),
        }),
        ["webhook", "serve", port] => Ok(Command::WebhookServe {
            port: parse_port(port)?,
        }),
        ["webhook", "status"] => Ok(Command::WebhookStatus),
        ["webhook", "logs"] => Ok(Command::WebhookLogs),
        ["webhook", "events"] => Ok(Command::WebhookEvents),
        ["webhook", "salts"] => Ok(Command::WebhookSalts),
        _ => Err(format!(
            "Unknown command: {}\n\n{}",
            args.join(" "),
            usage()
        )),
    }
}

fn parse_port(value: &str) -> Result<u16, String> {
    value
        .parse::<u16>()
        .map_err(|_| format!("Invalid port: {value}"))
}

pub fn usage() -> &'static str {
    r#"Nightfall deployment assistant

Usage:
  nf4 wizard deploy
  nf4 wizard proposer
  nf4 wizard client
  nf4 check deployer
  nf4 check prover
  nf4 up configuration
  nf4 up proposer
  nf4 up client
  nf4 status
  nf4 logs configuration
  nf4 logs proposer
  nf4 logs client
  nf4 client deploy-mock-tokens
  nf4 webhook start [port]
  nf4 webhook status
  nf4 webhook logs
  nf4 webhook events
  nf4 webhook salts

Commands:
  nf4 wizard deploy       Collects required inputs, writes config, deploys contracts, starts configuration, and validates hosted metadata.
  nf4 wizard proposer     Reuses deployment metadata, configures proposer values, starts indie-proposer, and validates health.
  nf4 wizard client       Reuses deployment/proposer metadata, configures client values, starts indie-client, and validates health.
  nf4 check deployer      Checks local tools required before deployment.
  nf4 check prover        Runs the pinned Nightfish recursive prover capability test.
  nf4 up configuration    Starts only the configuration service.
  nf4 up proposer         Starts only the proposer service.
  nf4 up client           Starts only the client service.
  nf4 status              Prints deployment, service, metadata, RPC, and contract checks.
  nf4 logs configuration  Shows configuration service logs.
  nf4 logs proposer       Shows proposer service logs.
  nf4 logs client         Shows client service logs.
  nf4 client deploy-mock-tokens
                           Deploys local mock ERC contracts using local.env and the configured RPC URL.
  nf4 webhook start       Starts the local testing webhook on port 8081, or the supplied port.
  nf4 webhook status      Shows local testing webhook status and event storage path.
  nf4 webhook logs        Prints local testing webhook process logs.
  nf4 webhook events      Prints stored webhook events.
  nf4 webhook salts       Prints withdraw_fund_salt values found in stored webhook events.

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
            parse(args(&["wizard", "client"])),
            Ok(Command::WizardClient)
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
        assert_eq!(parse(args(&["up", "client"])), Ok(Command::UpClient));
        assert_eq!(parse(args(&["status"])), Ok(Command::Status));
        assert_eq!(
            parse(args(&["logs", "configuration"])),
            Ok(Command::LogsConfiguration)
        );
        assert_eq!(
            parse(args(&["logs", "proposer"])),
            Ok(Command::LogsProposer)
        );
        assert_eq!(parse(args(&["logs", "client"])), Ok(Command::LogsClient));
        assert_eq!(
            parse(args(&["client", "deploy-mock-tokens"])),
            Ok(Command::ClientDeployMockTokens)
        );
        assert_eq!(
            parse(args(&["webhook", "start"])),
            Ok(Command::WebhookStart { port: None })
        );
        assert_eq!(
            parse(args(&["webhook", "start", "8082"])),
            Ok(Command::WebhookStart { port: Some(8082) })
        );
        assert_eq!(
            parse(args(&["webhook", "serve", "8082"])),
            Ok(Command::WebhookServe { port: 8082 })
        );
        assert_eq!(
            parse(args(&["webhook", "status"])),
            Ok(Command::WebhookStatus)
        );
        assert_eq!(parse(args(&["webhook", "logs"])), Ok(Command::WebhookLogs));
        assert_eq!(
            parse(args(&["webhook", "events"])),
            Ok(Command::WebhookEvents)
        );
        assert_eq!(
            parse(args(&["webhook", "salts"])),
            Ok(Command::WebhookSalts)
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
        assert!(parse(args(&["up", "unknown"])).is_err());
    }
}
