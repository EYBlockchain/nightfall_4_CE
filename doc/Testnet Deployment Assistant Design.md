# Nightfall Testnet Deployment Assistant Design

This document describes a proposed deployment assistant for Nightfall testnet deployments. It is intended for review and iteration before implementation.

## Goal

Create a guided deployment assistant for Nightfall testnet deployments. The assistant should ask only for essential inputs that cannot be safely inferred, update the required local files, run deployment steps, and validate the result.

Users should not need to manually edit `nightfall.toml` or `docker-compose.yml`.

## User Model

The assistant focuses on the operator/deployer workflow. The user normally runs one command:

```bash
nf4 wizard deploy
```

This command guides the user through one fresh Nightfall contract deployment. It should feel like a short checklist with sensible defaults, not a long interview. Behind the scenes, it performs preflight checks, edits local configuration files, runs the deployer, starts the configuration service, and runs post-deployment validation.

After deployment, the wizard prints the final result. The user can optionally run:

```bash
nf4 status
```

`nf4 status` is a post-deployment health summary. It is not required for a successful deploy, but it helps users understand what is configured, what is running, what is reachable, and what to do next if something is unhealthy.

If the user wants to inspect the configuration service logs, they can run:

```bash
nf4 logs configuration
```

This is mainly for troubleshooting after the wizard has started the configuration service.

The deployer is temporary. It deploys contracts, writes addresses, hashes, and keys, then exits.

The configuration service stays running.

## Implementation Shape

Use a small Rust CLI with a thin shell wrapper.

User-facing commands:

- `nf4 wizard deploy`: Main operator command. It asks only for required deployment decisions, uses defaults or detected values where possible, applies file changes, runs checks, starts required services, and prints the final summary.
- `nf4 status`: Checks the current deployment state, including RPC, chain ID, configuration service, addresses, hashes, keys, and prover check result.
- `nf4 logs configuration`: Shows configuration service logs.

Supporting commands used by `nf4 wizard deploy`, and also available for troubleshooting:

- `nf4 check deployer`: Runs deployer preflight checks without deploying contracts.
- `nf4 check prover`: Runs the prover capability test.
- `nf4 up configuration`: Starts only the configuration service. The main wizard calls this automatically after deployment succeeds.

Rust is preferred because the assistant needs structured TOML/YAML handling, private-key validation, URL validation, JSON parsing, and reliable status output.

Possible repo structure:

```text
nightfall_ops/
  Cargo.toml
  src/main.rs
  src/wizard.rs
  src/checks.rs
  src/config.rs
  src/compose.rs
  src/status.rs
scripts/nf4
```

The shell wrapper can call:

```bash
cargo run -p nightfall_ops -- "$@"
```

This means the `nf4` wrapper forwards the user's command-line arguments into the Rust CLI.

For example, if the user runs:

```bash
nf4 wizard deploy
```

the wrapper runs:

```bash
cargo run -p nightfall_ops -- wizard deploy
```

The `--` separates Cargo arguments from application arguments, and `"$@"` preserves all user-provided arguments.

Proposed file responsibilities:

- `nightfall_ops/Cargo.toml`: Defines the deployment assistant package, binary name, and dependencies.
- `nightfall_ops/src/main.rs`: CLI entrypoint. Parses top-level commands such as `wizard`, `check`, `up`, `status`, and `logs`, then dispatches to the relevant module.
- `nightfall_ops/src/wizard.rs`: Implements the guided deployment wizard. Collects only required user inputs, applies defaults, coordinates preflight checks, config edits, deployment execution, configuration startup, and final output.
- `nightfall_ops/src/checks.rs`: Contains validation logic for tools, RPC connectivity, chain ID, account balance, ports, proving keys, and prover capability checks.
- `nightfall_ops/src/config.rs`: Reads and updates `nightfall.toml` and `local.env`. Owns profile creation, deployer/proposer settings, and safe secret writing.
- `nightfall_ops/src/compose.rs`: Reads and updates `docker-compose.yml`, runs Docker Compose commands, starts the configuration service, and fetches service logs.
- `nightfall_ops/src/status.rs`: Builds the `nf4 status` health summary by checking RPC, metadata files, contract code, configuration service endpoints, keys, and recorded prover-check results.
- `scripts/nf4`: Thin shell wrapper that lets users run `nf4 ...` from the repo without typing the full `cargo run -p nightfall_ops -- ...` command.

## Config Strategy

The assistant edits the current repo files directly:

```text
nightfall.toml
docker-compose.yml
local.env
```

Rules:

- Create timestamped backups before editing.
- Show planned changes before writing.
- Never store private keys in `nightfall.toml` or `docker-compose.yml`.
- Store secrets only in `local.env`.
- Update only known fields.
- Run validation after edits.

Example backups:

```text
nightfall.toml.bak.20260612-143000
docker-compose.yml.bak.20260612-143000
```

This design uses the current `nightfall.toml` because the current settings loader reads that file directly.

## Interaction Model

The wizard should be minimally interactive. It should not ask users about implementation details or steps that the assistant can decide safely.

Principles:

- Ask only for values that affect deployment behavior and cannot be safely inferred.
- Group prompts by topic instead of asking a long sequence of isolated questions.
- Use detected values as defaults whenever possible.
- Let users press enter to accept recommended defaults.
- Explain only the risky or easy-to-misunderstand fields.
- Show one review summary before writing files or running expensive commands.
- Do not ask whether to run required internal commands such as Docker Compose build or configuration endpoint checks.

Prompt groups:

```text
1. Network
   - RPC URL
   - Chain ID, default fetched from RPC
   - Profile name

2. Configuration service
   - LAN or public URL
   - Configuration port, default 8080

3. Accounts
   - Deployer private key
   - Default proposer address and URL

4. Prover
   - Mock or real prover
   - Block size, default 64
   - Prover capability check confirmation when real prover is selected

5. Review
   - Show generated settings
   - Show files that will be changed
   - Ask for one final confirmation
```

Most users should only need to provide the RPC URL, deployer key, and configuration access choice.

## Startup Explanation

At wizard start, show a short explanation:

```text
Nightfall deployment uses a configuration service.

The configuration service hosts:
- deployed contract addresses
- deployed contract hashes
- proving keys

Every proposer and client must be able to reach this URL.
For testnet deployments, proposer/client nodes may run on different machines.
Use a LAN or public URL.
```

The startup explanation should be short. More detailed explanations should appear only when the user is choosing a setting that can break remote proposer/client access or cause expensive work.

## Configuration URL Design

The configuration service is the bridge between deployer, proposer, and client. It hosts the contract addresses, contract hashes, and proving keys that other actors need.


Ask:

```text
Configuration service access:
1. LAN/private network
2. Public URL/domain
```

For LAN, detect local IPs and ask:

```text
Select LAN IP reachable by proposer/client nodes:
Configuration service port [8080]:
```

Reaction:

```toml
configuration_url = "http://<lan-ip>:8080"
```

The generated `configuration_url` must include the exposed configuration service port. For example:

```toml
configuration_url = "http://35.225.105.10:8080"
```

and update `docker-compose.yml`:

```yaml
configuration:
  ports:
    - "8080:80"
```

For public URL, ask:

```text
Enter public configuration URL, e.g. https://config.example.com:
```

Reaction:

```toml
configuration_url = "https://config.example.com:8080"
```

The generated `configuration_url` should include the exposed configuration service port unless the user is intentionally using a reverse proxy or public ingress that terminates on `80` or `443`.

After the configuration service starts, the assistant should verify the configuration endpoints itself:

```text
GET <config-url>/configuration/toml/addresses.toml
GET <config-url>/configuration/toml/contract_hashes.toml
HEAD or GET <config-url>/configuration/bin/keys/proving_key
```

The assistant should tell the user what it is checking and report the result:

```text
Checking configuration service...
  addresses.toml: OK
  contract_hashes.toml: OK
  proving_key: OK
```

## Deployment Scope

The assistant supports one fresh Nightfall contract deployment.

It deploys new contracts, writes generated metadata, and starts the configuration service.

Using already deployed contracts is intentionally out of scope for this design.

## Important Settings And Reactions

This section describes the values the assistant manages. It does not mean each value must be a separate prompt. The wizard should auto-detect or default these values where possible, then show the resulting configuration in the final review.

### Profile Name

Default: `base_sepolia` or a user-provided custom name.

Validation:

- Lowercase letters, numbers, and underscores.
- Must map to a top-level profile in `nightfall.toml`.

Reaction:

- Create or update `[profile]` sections in `nightfall.toml`.
- Set `NF4_RUN_MODE` defaults in relevant Docker Compose services.

### RPC URL

Default: none.

Validation:

- Must start with `ws://` or `wss://`.
- Must respond.
- Must expose chain ID and current block number.

Reaction:

- Set `ethereum_client_url`.
- Set or write `NF4_ETHEREUM_CLIENT_URL`.

### Chain ID

Default: fetched from RPC.

Validation:

- Must match RPC chain ID.

Reaction:

- Set `[profile.network].chain_id`.

### Genesis Block

Default: current L1 block.

Validation:

- Must be less than or equal to current L1 block.

Reaction:

- Set `genesis_block`.

### Configuration Host Port

Default: `8080`.

Validation:

- Must be an integer from 1 to 65535.
- Must not already be in use.
- Warn if less than 1024.
- Should not conflict with proposer or client ports.

Reaction:

- Expose `<port>:80` in `docker-compose.yml`.
- Use this port when building the LAN configuration URL.

### Configuration Public URL

Default:

- LAN mode: `http://<selected-lan-ip>:<configuration-port>`.
- Public mode: none, user must provide full URL.

Validation:

- Must be a valid URL.
- Must include scheme.
- Must not include `/configuration/toml/...`; it should be only the base URL.

Reaction:

- Set `configuration_url`.
- Verify configuration endpoints after service startup and print the result.

### Deployer Private Key

Default: none.

Validation:

- Must be a valid private key.
- Must derive a valid deployer address.
- Deployer account must have enough host-chain funds.

Reaction:

- Write `DEPLOYER_SIGNING_KEY` to `local.env` only.
- Do not print the key back to the terminal.

### Default Proposer Address

Default: derived from proposer key if provided, otherwise user-provided.

Validation:

- Must be a valid Ethereum address.

Reaction:

- Set `default_proposer_address`.

### Default Proposer URL

Default: `http://<lan-or-public-host>:3001`.

Validation:

- Must be a valid URL.
- Should be reachable by clients if proposer/client run on different machines.

Reaction:

- Set `default_proposer_url`.

### Proposer API Port

Default: `3001`.

Validation:

- Must be available.

Reaction:

- Expose proposer API port in `docker-compose.yml`.

### Client API Port

Default: `3000`.

Validation:

- Must be available.

Reaction:

- Expose client API port in `docker-compose.yml`.

### Block Size

Default: `64`.

Validation:

- Must be `64` or `256`.

Reaction:

- Set `[profile.nightfall_proposer].block_size`.

### Deploy Contracts

Default: `true`.

Validation:

- Must be `true` for this deployment assistant.

Reaction:

- Set `[profile.contracts].deploy_contracts`.

### X509 Allowlisting

Default: enabled.

Reaction:

- Should be controlled by config, not by editing Solidity source.

Required implementation behavior:

- Deployment must read this setting from config.
- The assistant must not edit Solidity source to enable or disable X509 allowlisting.
- If the current deployer does not support this setting yet, add config support as part of this work.

## Real Prover

If the user selects real prover mode, require explicit confirmation:

```text
Real prover mode is expensive.
Key generation can take a long time and requires large RAM/disk.
Generating keys successfully does not prove this machine can prove a block.
```

The command is:

```bash
nf4 check prover
```

This should run the Nightfish recursion prover test:

```text
test_preprocess_and_prove
nightfish_CE/plonk/src/recursion/mod.rs
```

Purpose:

```text
Verify this machine can run the recursive proving path used by Nightfall.
Successful key generation alone is not enough evidence that the machine can produce/prove blocks.
```

The default behavior should run the test against the Nightfish revision pinned by this Nightfall workspace and `Cargo.lock`.

An optional advanced mode can run against upstream master:

```bash
nf4 check prover --nightfish-ref master
```

Store the result:

```text
.nightfall/deployments/<profile>/prover-check.json
```

Record:

- Test name.
- Nightfish commit or ref.
- Release/debug mode.
- Start time.
- End time.
- Result.
- Log path.

## Preflight Validation

Before expensive operations, validate:

- `docker` exists.
- `docker compose` works.
- `forge` exists.
- `cargo` exists.
- `curl` exists.
- RPC URL starts with `ws://` or `wss://`.
- RPC responds.
- RPC chain ID matches input.
- Current L1 block can be read.
- Deployer private key derives the expected address.
- Deployer account has enough funds.
- Configuration port is available.
- `proposer_stake > proposer_exit_penalty`.
- Block size is `64` or `256`.
- Required key files exist, or user confirms generation.

## Deployment Flow

Fresh contract deployment:

```text
1. Show a short deployment overview.
2. Collect grouped inputs with defaults.
3. Run preflight checks.
4. Back up nightfall.toml and docker-compose.yml.
5. Apply config changes.
6. Generate local.env.
7. Build contracts.
8. Run prover check if requested or required.
9. Generate keys if needed.
10. Build indie-deployer image.
11. Run indie-deployer.
12. Wait for successful exit.
13. Verify addresses.toml.
14. Verify contract_hashes.toml.
15. Verify on-chain code.
16. Start configuration service.
17. Verify configuration URL locally.
18. Verify configuration endpoints and print the result.
```

## Post-Deployment Validation

`nf4 status` should check:

- RPC is reachable.
- Configured chain ID matches RPC chain ID.
- Deployer completed successfully.
- `addresses.toml` exists.
- `contract_hashes.toml` exists.
- Contract addresses are non-zero.
- On-chain code exists at each contract address.
- Proxy implementations can be resolved where applicable.
- Configuration service is reachable.
- Required proving keys are reachable.
- Prover check result is available, if real prover mode was selected.

Example status output:

```text
Nightfall testnet status

Profile: base_sepolia
Mode: fresh deployment
Chain ID: OK, 84532
RPC: OK, latest block 7349120
Configuration URL: http://192.168.1.20:8080
Configuration service: OK

Deployment metadata:
  addresses.toml: OK
  contract_hashes.toml: OK
  keys: OK

Contracts:
  Nightfall: OK, 0x...
  RoundRobin: OK, 0x...
  X509: OK, 0x...
  Verifier: OK, 0x...

Prover:
  Mode: real
  Capability check: OK
  Last checked: 2026-06-12 14:30:00

Services:
  Deployer: completed successfully
  Configuration: running

Next:
  Configuration metadata is available at the configured URL.
```

Example unhealthy output:

```text
Nightfall testnet status

Configuration service: FAILED
Reason: http://192.168.1.20:8080/configuration/toml/addresses.toml returned 404

Suggested fix:
  Run: nf4 up configuration
  Then check: nf4 logs configuration
```

## Output Summary

After successful deployment, print:

```text
Deployment OK

Profile: <profile>
Chain ID: <chain-id>
Nightfall: <address>
RoundRobin: <address>
X509: <address>
Verifier: <address>
Configuration URL: <url>

Configuration endpoint checks:
  addresses.toml: OK
  contract_hashes.toml: OK
  proving_key: OK
```

## Required Command Set

The assistant should implement these commands:

```text
nf4 wizard deploy
nf4 check deployer
nf4 check prover
nf4 up configuration
nf4 status
nf4 logs configuration
```

`nf4 wizard deploy` is the normal operator entrypoint. It should call the lower-level checks and service commands internally when needed.

The standalone lower-level commands exist so users can troubleshoot or rerun one part of the flow without repeating the whole wizard.

## Fixed Decisions

- The assistant edits `nightfall.toml` and `docker-compose.yml` directly.
- The assistant creates timestamped backups before edits.
- The assistant stores private keys only in `local.env`.
- The deployer balance check uses `0.1 ETH` as the default minimum warning threshold.
- Public configuration URLs should use HTTPS when available. HTTP is allowed for testnet if the user explicitly confirms.
- In real prover mode, `nf4 wizard deploy` should recommend `nf4 check prover`. If the user skips it, require explicit confirmation and record that it was skipped in deployment status.
