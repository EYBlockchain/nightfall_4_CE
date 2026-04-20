# Nightfall 4 CE

Community Edition of Nightfall\_4

Nightfall\_4 is a Zero-Knowledge Proof (ZKP)-based Layer 2 ZK-ZK rollup for transferring ERC20, ERC721, ERC1155, and ERC3525 tokens privately on Ethereum. Unlike Nightfall\_3's optimistic rollup, Nightfall\_4 uses cryptographic proofs for near-instant finality. A private transfer typically costs around 6000 gas.

_This code is not owned by EY and EY provides no warranty and disclaims any and all liability for use of this code. Users must conduct their own diligence with respect to use for their purposes and any and all usage is on an as-is basis and at your own risk._

**This software is experimental. It should not be used to make significant value transactions.**

## Architecture

The project is a Rust workspace with the following crates:

| Crate | Description |
|---|---|
| `nightfall_client` | Client service for creating private transactions (deposit, transfer, withdraw) |
| `nightfall_proposer` | Block proposer that assembles L2 blocks and generates rollup proofs |
| `nightfall_deployer` | Smart contract deployment and ZK proving key generation |
| `nightfall_bindings` | Auto-generated Rust bindings for Solidity contracts |
| `lib` | Shared cryptographic and blockchain utilities (Merkle trees, Poseidon hashing, PLONK proofs) |
| `configuration` | Configuration management via TOML files and environment variables |

Smart contracts are in `blockchain_assets/contracts/` and use the UUPS upgradeable proxy pattern.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) 1.88.0+ (pinned via `rust-toolchain.toml`)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (forge, anvil)

## Quick Start

Start the full local development stack (Anvil chain, deployer, proposer, two clients, and MongoDB):

```bash
make dev
```

This uses the `development` profile with a local Anvil chain and mock provers, so no heavy ZK key generation is required.

Once running:
- Client 1 API: `http://localhost:3000`
- Client 2 API: `http://localhost:3002`
- Proposer API: `http://localhost:3001`
- Anvil RPC: `http://localhost:8545`

## Development Commands

Run `make help` to see all available targets:

| Command | Description |
|---|---|
| `make dev` | Start the full development stack |
| `make dev-build` | Build and start the development stack |
| `make dev-down` | Stop all development services |
| `make build` | Build all Rust crates |
| `make build-contracts` | Build Solidity contracts with Foundry |
| `make test-unit` | Run Rust unit tests |
| `make test-sync` | Run synchronization tests via Docker |
| `make test-forge` | Run Solidity contract tests |
| `make fmt` | Format Rust code (requires nightly) |
| `make clippy` | Run clippy linter (strict mode) |
| `make key-gen` | Generate ZK proving keys (resource-intensive) |
| `make clean` | Clean build artifacts |

## API Endpoints

### Client (port 3000)

| Method | Path | Description |
|---|---|---|
| POST | `/v1/deposit` | Deposit tokens into Nightfall |
| POST | `/v1/transfer` | Private token transfer |
| POST | `/v1/withdraw` | Withdraw tokens from Nightfall |
| GET | `/v1/commitments` | List commitments (supports `?limit=&offset=`) |
| GET | `/v1/commitment/{key}` | Get a single commitment |
| GET | `/v1/balance/{token}/{owner}` | Get ERC token balance |
| GET | `/v1/fee_balance` | Get fee balance |
| GET | `/v1/l1_balance` | Get Layer 1 balance |
| GET | `/v1/synchronisation` | Check sync status |
| POST | `/v1/certification` | Submit X.509 certificate |
| GET | `/v1/health` | Health check |

### Proposer (port 3001)

| Method | Path | Description |
|---|---|---|
| POST | `/v1/transaction` | Submit client transaction |
| POST | `/v1/register` | Register as a proposer |
| POST | `/v1/deregister` | Deregister a proposer |
| GET | `/v1/rotate` | Rotate active proposer |
| POST | `/v1/pause` | Pause block assembly |
| POST | `/v1/resume` | Resume block assembly |
| GET | `/v1/blockdata` | Get current block data |
| POST | `/v1/certification` | Submit X.509 certificate |
| GET | `/v1/health` | Health check |

## Documentation

- [Architecture and API Reference](doc/nf_4.md) - Full documentation
- [Testnet Setup Guide](doc/Setup%20Testnet%20Guide.md) - Deploy to a host chain
- [Upgradable Contracts Guide](doc/Upgradable%20Contracts%20Guide.md) - Contract upgrade procedures
- [Changelog](doc/CHANGELOG.md) - Version history

## License

See [LICENSE](LICENSE).
