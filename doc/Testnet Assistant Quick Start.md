# Testnet Assistant Quick Start

This guide explains how to use the `nf4` helper script to start a local testnet deployment with deployer, proposer, and client nodes on the same machine.

The assistant edits local configuration files, starts Docker services, and runs basic checks for you. You should run these commands from the repository root.

## Before You Start

Make sure the machine has:

- Docker and Docker Compose
- Foundry tools, including `forge` and `cast`
- Rust/Cargo
- A funded testnet private key
- A WebSocket RPC URL for the host chain

Update your branch before testing:

```bash
git switch auto/testnet
git pull
```

Run the assistant tests:

```bash
cargo test -p nightfall_ops
```

## The Helper Script

Use the script from the repository root:

```bash
./scripts/nf4 <command>
```

The script runs the `nightfall_ops` helper from Cargo and passes your command through to it.

## 1. Deploy Contracts And Configuration

Start a fresh deployment:

```bash
./scripts/nf4 wizard deploy
```

The deploy wizard asks for the main deployment settings, including:

- profile name
- host-chain WebSocket RPC URL
- configuration service URL
- deployer private key
- default proposer address and URL
- mock prover or real prover mode
- block size

For same-machine testing, use the LAN IP default for the configuration URL, for example:

```text
http://10.0.0.8:8080
```

For mock prover testing, answer `no` when asked whether to use real prover mode.

When the deploy wizard succeeds, check the deployment:

```bash
./scripts/nf4 status
```

## 2. Start The Proposer

Start the proposer node:

```bash
./scripts/nf4 wizard proposer
```

For the first same-machine test, use the same private key/address that was registered as the default proposer during deployment.

Use the LAN proposer URL, for example:

```text
http://10.0.0.8:3001
```

Check proposer status and logs:

```bash
./scripts/nf4 status
./scripts/nf4 logs proposer
```

## 3. Start The Client

Start the client node:

```bash
./scripts/nf4 wizard client
```

The client wizard asks for:

- client private key
- client address
- proposer URL
- configuration URL
- client API port

Use the same LAN configuration URL and proposer URL that were used above.

Check client status and logs:

```bash
./scripts/nf4 status
./scripts/nf4 logs client
```

## Useful Commands

Check the whole local testnet:

```bash
./scripts/nf4 status
```

Follow configuration service logs:

```bash
./scripts/nf4 logs configuration
```

Follow proposer logs:

```bash
./scripts/nf4 logs proposer
```

Follow client logs:

```bash
./scripts/nf4 logs client
```

## Expected Result

After all three wizards succeed:

- deployer has exited successfully
- configuration service is running
- proposer service is healthy
- client service is healthy
- `./scripts/nf4 status` reports deployed contracts and reachable configuration files

At that point, the local testnet is ready for client API testing.
