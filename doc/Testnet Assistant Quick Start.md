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
- local testing webhook setup
- client API port

Use the same LAN configuration URL and proposer URL that were used above.

For the local testing webhook, press Enter to use the default:

```text
http://<server-lan-ip>:8081/webhook
```

The assistant starts this webhook for you and stores received webhook events under `.nightfall/webhook/events.jsonl`.

The client wizard also prepares the local mock token deployment environment. It writes `NF4_SIGNING_KEY`, `CLIENT2_ADDRESS`, and `NIGHTFALL_ADDRESS` to `local.env`, then runs:

```bash
forge clean
forge build
```

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

Show stored webhook events:

```bash
./scripts/nf4 webhook events
```

Show withdraw fund salts found in webhook events:

```bash
./scripts/nf4 webhook salts
```

Deploy local mock ERC contracts for deposit testing. This helper reads `local.env`, fills the required Forge environment values, and uses the configured host-chain RPC URL:

```bash
./scripts/nf4 client deploy-mock-tokens
```

## 4. Try Client APIs

After mock ERC deployment, copy the printed mock ERC addresses. The examples below use one ERC20 mock address:

```text
0x99Ed986BB66CC72365b712b0277b1C019146CE6d
```

Replace this address with your deployed `ERC20Mock` address if yours is different.

Token type values:

- `0`: ERC20. Use `tokenId: "0x00"`.
- `1`: ERC1155.
- `2`: ERC721.
- `3`: ERC3525.
- `4`: Fee token, used internally.

Useful function names and Method values to look for on the block explorer:

- X509 certification calls `X509.validateCertificate(...)`; Method: `0x4e5805d3`.
- Deposit first calls `Nightfall.escrow_funds(...)`; Method: `0xe6d5abe5`.
- When a deposit is included in an L2 block, the proposer calls `Nightfall.propose_block(...)`; Method: `0x55420851`.
- Transfer is first submitted to the proposer; when included in an L2 block, the proposer calls `Nightfall.propose_block(...)`; Method: `0x55420851`.
- Withdraw is first submitted to the proposer; when included in an L2 block, the proposer calls `Nightfall.propose_block(...)`; Method: `0x55420851`.
- After withdraw inclusion, de-escrow calls `Nightfall.descrow_funds(...)`; Method: `0xf3b85fc2`.
- During deployment, `Nightfall.set_proposer_manager(address)` may also appear; Method: `0xe3178c86`.

If X509 allowlisting is enabled, certify the proposer:

```bash
curl -i --request POST 'http://localhost:3001/v1/certification' \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
```

Certify the client:

```bash
curl -i --request POST 'http://localhost:3000/v1/certification' \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
```

Deposit ERC20 tokens:

```bash
curl -i \
  -H 'Content-Type: application/json' \
  -X POST 'http://localhost:3000/v1/deposit' \
  --data-raw '{
    "ercAddress": "0x99Ed986BB66CC72365b712b0277b1C019146CE6d",
    "tokenId": "0x00",
    "tokenType": "0",
    "value": "0x05",
    "fee": "0x00",
    "deposit_fee": "0x09"
  }'
```

Transfer ERC20 tokens:

```bash
curl -i \
  -H 'Content-Type: application/json' \
  -X POST 'http://localhost:3000/v1/transfer' \
  --data-raw '{
    "ercAddress": "0x99Ed986BB66CC72365b712b0277b1C019146CE6d",
    "tokenId": "0x00",
    "tokenType": "0",
    "recipientData": {
      "values": ["0x01"],
      "recipientCompressedZkpPublicKeys": ["0572aa70f4e62bcb8f53a28a1c259bd6d3538818afcccc0d8598486973ec2f2a"]
    },
    "fee": "0x00"
  }'
```

Withdraw ERC20 tokens:

```bash
curl -i -X POST 'http://localhost:3000/v1/withdraw' \
  -H 'Content-Type: application/json' \
  -d '{"ercAddress":"0x99Ed986BB66CC72365b712b0277b1C019146CE6d","tokenId":"0x00","tokenType":"0","value":"0x02","recipientAddress":"0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266","fee":"0x00"}'
```

After submitting requests, check webhook events:

```bash
./scripts/nf4 webhook events
./scripts/nf4 webhook salts
```

## Expected Result

After all three wizards succeed:

- deployer has exited successfully
- configuration service is running
- proposer service is healthy
- client service is healthy
- local testing webhook is running if you accepted the default client wizard option
- `./scripts/nf4 status` reports deployed contracts and reachable configuration files

At that point, the local testnet is ready for client API testing.
