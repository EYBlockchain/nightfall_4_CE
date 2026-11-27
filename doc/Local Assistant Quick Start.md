# Local Assistant Quick Start

Run a complete Nightfall check on **local Anvil** with `nf4`. The sequence matches `doc/sepolia_testing_report_template.md`: one deployer, one proposer, two clients, all four token standards, late Client 2 join, Client 1 stop/recover, proposer restart, then a post-recovery deposit.

This is not Sepolia. Do not use a public RPC or `--network testnet`. For Ethereum Sepolia from this machine, see `doc/Sepolia Assistant Quick Start.md`.

Work from the repository root on `auto/testnet`. Record evidence as you go (command output, logs, `cast` receipts). Do not put private keys, mnemonics, or RPC secrets in the report.

## Local substitutions for the Sepolia report

| Report field | Local value |
|---|---|
| Environment | Local Anvil |
| Chain ID | `31337` |
| RPC | `ws://127.0.0.1:8545` (host); containers use `ws://anvil:8545` |
| Run mode | `local` (`NF4_RUN_MODE=local`) |
| Prover | `mock` |
| Explorer | none; use `cast receipt`, `cast tx`, `cast block-number` |
| Client 1 API | `http://127.0.0.1:3000` |
| Client 2 API | `http://127.0.0.1:3002` |
| Proposer API | `http://127.0.0.1:3001` |
| Configuration | Host checks: `http://127.0.0.1:8080`. Wizards/compose: current LAN IP, filled by `nf4` (`ipconfig getifaddr en0`). Do not type `host.docker.internal` or `http://configuration:80` into the wizard. |
| 48-hour soak | same checks; duration is optional on Anvil. Note the actual duration in the report |

Out of scope, same as the Sepolia template: second proposer, rotation, proposer failover.

## Accounts (Anvil, public)

| Role | Address | Private key |
|---|---|---|
| Deployer / default proposer / Client 1 L1 | `0xf39Fd6e51aad88F6F4ce6ab8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Client 2 L1 | `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` |

These are Foundry Anvil’s well-known keys. Paste them when a wizard asks for a private key (input is hidden; nothing will echo). Never use them on testnet or mainnet.

Deployer / proposer / Client 1 — paste this:

```text
0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

Client 2 — paste this:

```text
0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

## Fixed test data (report table)

Use these values. Hex is what the client API wants. Token IDs must be even-length hex (`0x03e9` not `0x3e9`, `0x03ea` not `0x3ea`). Odd length returns `Invalid tokenId: Invalid hex format`. Export the padded IDs in step 5 and interpolate `${ID…}` in every curl; do not type a shortened form.

| Standard | `tokenType` | Token ID | Client 1 deposit | Client 2 deposit | Transfer C1→C2 | Withdraw C1 | Withdraw C2 |
|---|---|---|---|---|---|---|---|
| ERC20 | `0` | `0x00` (`ID20`) | `0x64` (100) | none | `0x28` (40) | `0x3c` (60) | `0x28` (40) |
| ERC721 | `2` | C1 `0x03e9` (`ID721_C1`, 1001); C2 `0x03ea` (`ID721_C2`, 1002) | `1001` | `1002` | `1001` to C2, then back | `1001` | `1002` |
| ERC1155 | `1` | `0x07d1` (`ID1155`, 2001) | `0x14` (20) | `0x0a` (10) | `0x08` (8) | `0x0c` (12) | `0x12` (18) |
| ERC3525 | `3` | C1 `0x0bb9` (`ID3525_C1`, 3001); C2 `0x0bba` (`ID3525_C2`, 3002); same slot `0x01` | value `0x64` | value `0x64` | value `0x28` from `3001` | remaining `0x3c` | received `0x28` plus original `3002` |

Fees: send `"fee": "0x00"` and `"deposit_fee": "0x00"`. If the API requires a fee, keep using one value for every call and record post-fee balances from the API and Mongo. Expected balances in the table are pre-fee.

ERC721 deposit, transfer, and withdraw `value` is `0x00`. The NFT is identified by `tokenId`.

## 0. Tools

```bash
git switch auto/testnet
git pull
cargo test -p nightfall_ops
./scripts/nf4 check deployer
```

Need Docker running, `forge`, `cast`, `cargo`, `curl`. All `check deployer` lines must be `OK`.

Unattended copy of this guide (full reset, mock prover, markdown report, no soak):

```bash
./scripts/nf4-local-e2e.sh
```

Requires `nf4 wizard … --yes` (local only). Writes `reports/local-e2e-<utc>/report.md`. `--reuse` skips deploy only if the on-chain proposer URL is `http://indie-proposer:3000`.

## Start over (full reset)

Use this only when you want a **new chain and new contracts**. Do not do this if deploy already succeeded (deployer exited 0 and `http://127.0.0.1:8080/configuration/toml/addresses.toml` returns 200). In that case continue at proposer/client.

Anvil state lives in the Anvil container. Old contract addresses are invalid on a new Anvil. Reset must recreate Anvil, then run `wizard deploy` again.

```bash
docker compose --profile anvil --profile configuration --profile indie-deployer \
  --profile indie-proposer --profile indie-client --profile indie-client2 --profile no_test \
  down --remove-orphans

docker network rm nightfall_4_ce_nightfall_network 2>/dev/null || true

# generated deploy metadata (safe to delete; wizard/deployer recreate it)
rm -f configuration/toml/addresses.toml configuration/toml/contract_hashes.toml

# optional: drop webhook leftovers
rm -f .nightfall/webhook/events.jsonl
```

Leave `nightfall.toml` and `local.env` in place. The next `wizard deploy --network local` overwrites the `local` profile and merges env. Then go to **1. Anvil** and continue through deploy.

Do not `git checkout` those files unless you also want to throw away wizard port/URL edits.

Partial retries (do **not** full-reset):

| Stuck at | Retry |
|---|---|
| Anvil not up | Step 1 only |
| Deploy wizard URL/`host.docker.internal` but deployer exited 0 | Do not reset. Continue at proposer. Accept `nf4` LAN defaults |
| Proposer wizard failed | `./scripts/nf4 wizard proposer` again |
| Client 1 wizard failed | `./scripts/nf4 wizard client` again |
| Client 2 container bad | `docker rm -f nf4_indie_client2`, then `./scripts/nf4 up client2` |
| LAN IP changed (status config timeouts) | `./scripts/nf4 status` (rewrites `local.env`), then recreate proposer/client. Do not redeploy |
| Deploy, proposer, and client already healthy | Skip to cert/keys/mock tokens. Do not run `wizard deploy` again |

## 1. Anvil

```bash
docker compose --profile anvil down
docker network rm nightfall_4_ce_nightfall_network 2>/dev/null || true
docker compose --profile anvil up -d
cast chain-id --rpc-url ws://127.0.0.1:8545
cast balance 0xf39Fd6e51aad88F6F4ce6ab8827279cffFb92266 --rpc-url http://127.0.0.1:8545
cast balance 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
```

Expect chain ID `31337` and non-zero balances. Ignore `NF4_* is not set` warnings.

If `cast chain-id` fails: `docker compose --profile anvil ps` and `docker compose --profile anvil logs anvil`.

## 2. Deploy Nightfall (report: deployer, contracts, config)

```bash
./scripts/nf4 wizard deploy --network local
```

| Prompt | Enter |
|---|---|
| Profile | `local` |
| RPC | `ws://127.0.0.1:8545` |
| Configuration port | `8080` |
| Configuration URL | Accept the default (`http://<detected-lan-ip>:8080`). Do not use `host.docker.internal` |
| Deployer key | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Default proposer address | Enter (deployer) |
| Default proposer URL | Accept `http://indie-proposer:3000` (docker DNS, written on-chain). Do not use a LAN IP. Host health is still `http://127.0.0.1:3001` |
| Real prover? | `no` |
| Block size | `64` |
| Apply changes? | `yes` after the review shows `network: local` and `chain_id: 31337` |

`nf4` detects the current LAN IP (`ipconfig getifaddr en0` / `hostname -I`) on deploy, proposer, client, `status`, and compose up. It rewrites `host.docker.internal`, `127.0.0.1`, and stale private IPs in `local.env` (`NF4_CONFIGURATION_URL`, host proposer URL, webhook URL). That does **not** change the on-chain proposer URL. Local deploy registers `http://indie-proposer:3000` so transfers keep working after Wi-Fi/VPN changes. Accept the docker-service default. Do not use `http://configuration:80` in the wizard.

After `Deployment OK`:

```bash
./scripts/nf4 status
curl -sS http://127.0.0.1:8080/configuration/toml/addresses.toml
curl -sS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/configuration/toml/contract_hashes.toml
curl -sS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/configuration/bin/keys/proving_key
docker inspect -f '{{.State.Status}} {{.State.ExitCode}}' nf4_indie_deployer
```

Record:

- `NF4_RUN_MODE=local`, `NF4_MOCK_PROVER=true`, `NF4_CONTRACTS__DEPLOY_CONTRACTS` was `true` for this run
- Nightfall, Round Robin, X509 addresses and `code OK`
- Verifier skipped in mock mode (report: Verifier `not deployed / mock`)
- VK provider if present in `addresses.toml`
- Deployer container exited `0`

```bash
grep -E 'NF4_RUN_MODE|NF4_MOCK_PROVER|NF4_CONTRACTS__DEPLOY_CONTRACTS|NF4_NETWORK' local.env
```

Confirm `local.env` is gitignored. Do not attach it to the report.

If the wizard printed `Could not resolve host: host.docker.internal` but `curl http://127.0.0.1:8080/configuration/toml/addresses.toml` returns `200` and `nf4_indie_deployer` exited `0`, **do not redeploy**. Contracts are already on Anvil. Check:

```bash
curl -sS http://127.0.0.1:8080/configuration/toml/addresses.toml
docker inspect -f '{{.State.Status}} {{.State.ExitCode}}' nf4_indie_deployer
```

Then continue at step 3. `nf4 wizard proposer` / `client` / `status` rewrite stale LAN IPs in `local.env`. Accept those defaults. That is not a new deploy.

If deployer, proposer, and client are already `healthy` and `curl http://127.0.0.1:8080/configuration/toml/addresses.toml` is 200, skip to **Client 1 cert + keys** (after step 4). First refresh env:

```bash
./scripts/nf4 status
docker compose --profile indie-proposer --profile indie-client --env-file local.env up -d
curl -sS http://127.0.0.1:3001/v1/health
curl -sS http://127.0.0.1:3000/v1/health
```

## 3. Proposer (report: one proposer, health, cert)

```bash
./scripts/nf4 wizard proposer
```

| Prompt | Enter |
|---|---|
| Proposer key | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Public proposer URL | Accept the default (current LAN IP, port 3001). Health on the Mac is `http://127.0.0.1:3001` |
| Configuration URL | Accept the default (current LAN IP, port 8080) |

```bash
curl -sS -i http://127.0.0.1:3001/v1/health
curl -i --request POST 'http://localhost:3001/v1/certification' \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
curl -sS http://127.0.0.1:3001/v1/proposers
./scripts/nf4 logs proposer
```

Health must be HTTP 200 and body `Healthy`. Exactly one proposer registered. Save a log excerpt that it connected to chain `31337`.

## 4. Client 1 only (report step 1: Client 2 still stopped)

```bash
./scripts/nf4 wizard client
```

| Prompt | Enter |
|---|---|
| Client key | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Client address | Enter |
| Proposer URL | Accept the default (current LAN IP, port 3001). If proposer is not up, stop and run `wizard proposer` first |
| Configuration URL | Accept the default (current LAN IP, port 8080) |
| Webhook | Accept the default (current LAN IP, port 8081) |
| Client API port | `3000` |

Host checks always use `127.0.0.1` even when `local.env` has the LAN IP.

### Client 1 cert + keys

```bash
curl -sS -i http://127.0.0.1:3000/v1/health
curl -i --request POST 'http://localhost:3000/v1/certification' \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
curl -sS -X POST http://127.0.0.1:3000/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS http://127.0.0.1:3000/v1/commitments
curl -sS http://127.0.0.1:3000/v1/l1_balance
```

This is the 24-word `key_request` mnemonic from `nightfall_test.toml`, not the Anvil L1 key. Export Client 1’s compressed ZKP public key from the stored result (64 hex chars, no `0x`). Do not paste the name `C1_ZKP` into later JSON.

```bash
export C1_ZKP=$(curl -sS -X POST http://127.0.0.1:3000/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C1_ZKP"
```

Save L2 sync payload as the baseline block.

Confirm Client 2 is not running:

```bash
docker ps --filter name=nf4_indie_client2 --format '{{.Names}} {{.Status}}'
```

Empty is correct.

## 5. Mock tokens and mint the report IDs

Point ERC20’s second allocation at Client 2, then deploy mocks:

```bash
# merge into local.env (keep existing keys)
# CLIENT2_ADDRESS=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
# CLIENT2_SIGNING_KEY=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
./scripts/nf4 client deploy-mock-tokens
```

Copy the four printed addresses. Export them:

```bash
export TOKEN20=<ERC20Mock>
export TOKEN721=<ERC721Mock>
export TOKEN1155=<ERC1155Mock>
export TOKEN3525=<ERC3525Mock>
export NF=$(awk -F'"' '/^nightfall *=/{print $2}' configuration/toml/addresses.toml)
export RPC=http://127.0.0.1:8545
export C1=0xf39Fd6e51aad88F6F4ce6ab8827279cffFb92266
export C2=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
export KEY1=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
# even-length hex for the client API (0x03ea not 0x3ea)
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
```

Mocks mint leftover IDs (ERC721 `426`, ERC1155 `2`/`73`, ERC3525 `7`/`8`). Mint the report IDs:

```bash
# ERC721 1001 -> C1, 1002 -> C2
cast send "$TOKEN721" "mint(address,address,uint256)" "$C1" "$NF" 1001 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN721" "mint(address,address,uint256)" "$C2" "$NF" 1002 --private-key "$KEY1" --rpc-url "$RPC"
# ERC1155 2001: 20 to C1, 10 to C2
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C1" "$NF" 2001 20 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C2" "$NF" 2001 10 --private-key "$KEY1" --rpc-url "$RPC"
# ERC3525 same slot 1: 3001 value 100 to C1, 3002 value 100 to C2
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C1" 3001 1 100 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C2" 3002 1 100 --private-key "$KEY1" --rpc-url "$RPC"
```

## 6. Report step 1 — baseline

Keep Client 2 stopped.

```bash
curl -sS http://127.0.0.1:3001/v1/health
curl -sS http://127.0.0.1:3000/v1/health
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS http://127.0.0.1:3000/v1/commitments
curl -sS "http://127.0.0.1:3000/v1/balance/${TOKEN20}/${ID20}"
cast block-number --rpc-url "$RPC"
docker exec nf4_db_client mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker exec nf4_db_proposer mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
```

## 7. Report steps 2–4 — Client 1 deposits, wait for a block

```bash
# ERC20 100
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC721 1001
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC1155 2001 x 20
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x14\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC3525 3001 value 100
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

The id is the `x-request-id` header, not the `"Request queued"` body. `-D -` prints headers. Poll until confirmed:

```bash
curl -sS http://127.0.0.1:3000/v1/request/<REQUEST_ID>
./scripts/nf4 webhook events
./scripts/nf4 logs proposer
```

Wait for the proposer to assemble, prove (mock), and submit. Block wait can be up to `block_assembly_max_wait_secs` (120s in the cloned template).

Then:

```bash
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS "http://127.0.0.1:3000/v1/balance/${TOKEN20}/${ID20}"
curl -sS "http://127.0.0.1:3000/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "http://127.0.0.1:3000/v1/balance/${TOKEN1155}/${ID1155}"
curl -sS "http://127.0.0.1:3000/v1/balance/${TOKEN3525}/${ID3525_C1}"
curl -sS http://127.0.0.1:3000/v1/commitments
docker exec nf4_db_client mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Expect Client 1: ERC20 `100`, ERC721 `1001`, ERC1155 `2001:20`, ERC3525 `3001:100` (hex `0x64`, `01`, `0x14`, `0x64` if no fees). ERC721 `/v1/balance` is ownership count (`01` if held), not the on-chain value field. Record L2 block number.

## 8. Report steps 5–7 — Client 2 joins late, then deposits

Start Client 2 after the deposit block is confirmed. LAN URLs and mock/prover/RPC values come from `local.env`.

```bash
./scripts/nf4 up client2
```

If `CLIENT2_SIGNING_KEY` is empty on a local Anvil run, this uses Anvil account 1 (`0x7099…`). If an old `docker run` client 2 exists, `docker rm -f nf4_indie_client2` first.

```bash
curl -sS -i http://127.0.0.1:3002/v1/health
curl -i --request POST 'http://localhost:3002/v1/certification' \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.priv_key;type=application/octet-stream'
curl -sS -X POST http://127.0.0.1:3002/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS http://127.0.0.1:3002/v1/synchronisation
curl -sS http://127.0.0.1:3002/v1/commitments
```

`deriveKey` uses the 24-word `key_request2` mnemonic from `nightfall_test.toml`. Do not use the Foundry 12-word L1 phrase (`test ... junk`); that returns `500`.

Client 2 must reach the same confirmed L2 block as Client 1 without error. Export Client 2’s compressed ZKP public key from the stored result (64 hex chars, no `0x`). Do not paste the name `C2_ZKP` into later JSON.

```bash
export C2_ZKP=$(curl -sS -X POST http://127.0.0.1:3002/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C2_ZKP"
```

Client 2 deposits:

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

Wait for the next proposed block, then verify Client 2 Mongo/API: `1002`, `2001:10`, `3002:100`.

## 9. Report step 8 — transfers from Client 1, Client 2 still up

`C2_ZKP` and the `$ID*` values must already be exported. The curls interpolate `${C2_ZKP}` and `${ID…}`; do not leave those names as JSON strings.

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"recipientData\":{\"values\":[\"0x08\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
```

Use `-D -` so you get `x-request-id`. Poll `/v1/request/<id>` until accepted for **all four** transfers. Do **not** wait for L2 confirmation yet. Do not stop Client 1 if a transfer returned `Invalid recipient public key` or never queued. Proposer logs should show client transactions in the mempool (`Found N client transactions`, N > 0) before you continue.

## 10. Report step 9 — stop Client 1 before the transfer block confirms

```bash
curl -sS http://127.0.0.1:3000/v1/synchronisation
docker exec nf4_db_client mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker stop nf4_indie_client
date -u
```

Record timestamp, last Client 1 L2 block, and DB dump.

## 11. Report step 10 — proposer confirms transfers; Client 2 stays up

`/v1/balance` returns `No such token` (404) when there are no unspent commitments for that token, not `00`. Client 2 never deposits ERC20, so `${TOKEN20}/${ID20}` 404s until the C1→C2 transfer is in an L2 block. That is not a bad token address.

If proposer logs show `Found 0 client transactions` / `No transactions pending`, the transfers never left Client 1. Start Client 1, re-run `deriveKey` (keys are in-memory), resubmit any still-`Unspent` transfers, poll until accepted, then `docker stop nf4_indie_client` again if you still need that report beat.

```bash
./scripts/nf4 logs proposer
curl -sS http://127.0.0.1:3002/v1/synchronisation
curl -sS http://127.0.0.1:3002/v1/commitments
curl -sS "http://127.0.0.1:3002/v1/balance/${TOKEN20}/${ID20}"
curl -sS "http://127.0.0.1:3002/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "http://127.0.0.1:3002/v1/balance/${TOKEN1155}/${ID1155}"
docker exec nf4_db_client2 mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Client 2 must show received ERC20 `40`, ERC721 `1001`, ERC1155 `18` (`10+8`), ERC3525 received value `40`.

## 12. Report step 11 — restart Client 1

ZKP keys are in-memory. After `docker start`, run Client 1 `deriveKey` again (same 24-word `key_request` mnemonic) before checking commitments.

```bash
docker start nf4_indie_client
curl -sS -i http://127.0.0.1:3000/v1/health
curl -sS -X POST http://127.0.0.1:3000/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS http://127.0.0.1:3000/v1/commitments
docker exec nf4_db_client mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Client 1 must catch up to the transfer block. Spent commitments for the transferred assets; remaining ERC20 `60`, ERC1155 `12`, ERC3525 `60`; ERC721 `1001` spent.

## 13. Report step 12 — compare both clients and proposer

```bash
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS http://127.0.0.1:3002/v1/synchronisation
docker exec nf4_db_client mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec nf4_db_client2 mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec nf4_db_proposer mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
```

L2 block numbers must match.

| | Client 1 | Client 2 |
|---|---|---|
| ERC20 | 60 | 40 |
| ERC721 | `1001` spent | `1001` held, `1002` held |
| ERC1155 `2001` | 12 | 18 |
| ERC3525 | remaining 60 on `3001` | received 40 plus original `3002` |

## 14. Report step 13 — Client 2 returns ERC721 `1001`

`C1_ZKP` and `$ID721_C1` must already be exported. Interpolate `${C1_ZKP}` and `${ID721_C1}`; do not leave those names as JSON strings.

```bash
curl -sS -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C1_ZKP}\"]},\"fee\":\"0x00\"}"
```

Wait for a block. Both DBs: Client 1 holds `1001` again; Client 2 does not.

## 15. Report step 14 — withdrawals

Client 1:

```bash
curl -sS -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
```

Client 2:

```bash
curl -sS -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x12\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -H 'Content-Type: application/json' -X POST http://127.0.0.1:3002/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
```

Wait for blocks. For each request id, save `/v1/request/<id>`, webhook events, and:

```bash
./scripts/nf4 webhook salts
cast balance "$C1" --rpc-url "$RPC"
# plus token balances on L1 if you need them, e.g. ERC20:
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C1" --rpc-url "$RPC"
```

Withdrawals are two-phase (L2 inclusion, then L1 descrow). Do not mark the step pass until L1 balances move.

## 16. Report steps 15–16 — restart proposer

```bash
curl -sS http://127.0.0.1:3000/v1/synchronisation
docker exec nf4_db_proposer mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
date -u
docker stop nf4_indie_proposer
date -u
docker start nf4_indie_proposer
curl -sS -i http://127.0.0.1:3001/v1/health
./scripts/nf4 logs proposer
docker exec nf4_db_proposer mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
curl -sS http://127.0.0.1:3000/v1/synchronisation
curl -sS http://127.0.0.1:3002/v1/synchronisation
```

Record downtime. After restart, health `Healthy`, prior L2 history still in proposer Mongo, both clients agree on the same L2 block.

`NF4_CONTRACTS__DEPLOY_CONTRACTS` must stay `false` on restart (the proposer wizard already wrote that).

## 17. Report step 17 — deposit after recovery

Client 1 ERC20 `10` (`0x0a`):

```bash
curl -sS -H 'Content-Type: application/json' -X POST http://127.0.0.1:3000/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

Wait for a new block. Client 1 commitments must include the new `10`.

## 18. Report steps 18–19 — soak (optional length on Anvil)

Leave the proposer running. At start, +24h (if you run that long), and end:

```bash
date -u
curl -sS -i http://127.0.0.1:3001/v1/health
curl -sS http://127.0.0.1:3000/v1/synchronisation
docker stats --no-stream nf4_indie_proposer nf4_indie_client nf4_indie_client2
```

Sepolia report target is 48 hours. On Anvil, write the actual duration. After soak, Client 2 deposits ERC20 `10` and the proposer must produce a block while still `Healthy`.

## Status snapshot

```bash
./scripts/nf4 status
docker ps -a --filter name=nf4_ --format '{{.Names}}\t{{.Status}}'
```

When the run is complete:

- `Nightfall local status`, `Network: local`, `Prover: mock`, chain `31337`
- Deployer exited 0; configuration, proposer, both clients running
- Health 200 `Healthy` on `:3000`, `:3001`, `:3002`
- Configuration files reachable on `http://127.0.0.1:8080`
- Mongo `nightfall.commitments` and `nightfall.ProposedBlocks` populated on client1, client2, and proposer

## Failures

| Symptom | Action |
|---|---|
| Anvil connection refused | Step 1 |
| Pool overlaps `172.18` | Old compose still pins a subnet. Update branch, `compose down`, remove `nightfall_4_ce_nightfall_network` |
| Wizard failed on `host.docker.internal`, but `127.0.0.1:8080` returns 200 and deployer exited 0 | Do not redeploy. Run `./scripts/nf4 status` (rewrites LAN URLs) and continue at proposer |
| Status config checks time out on an old LAN IP | Wi-Fi/VPN changed. `curl http://127.0.0.1:8080/configuration/toml/addresses.toml` should be 200. Run `./scripts/nf4 status` then `./scripts/nf4 up proposer`, `./scripts/nf4 up client`, and `./scripts/nf4 up client2` if client 2 was running. Do not redeploy contracts |
| Transfers fail with connection refused to an old LAN `:3001` | Client posts to the on-chain Round Robin URL, not `NF4_NIGHTFALL_PROPOSER__URL`. New local deploys register `http://indie-proposer:3000`. Recreating containers does not update an existing chain. Full reset + `wizard deploy`, or keep one network for the run |
| Need a clean chain / new contracts | **Start over (full reset)**, then Anvil, then `wizard deploy` |
| `dependency failed to start: container nf4_db_proposer is unhealthy` | Docker Linux kernel is too new for `mongo:8.0`. This repo uses `mongo:8.2`. Recreate the DB container, then rerun `wizard proposer` (do not redeploy contracts) |
| `development` / `production` profile rejected | Use `local` |
| Client 2 cannot reach config/proposer | Do not use `127.0.0.1` inside the container. Use `./scripts/nf4 up client2` so LAN URLs come from `local.env` |
| Deposits sit in mempool | Proposer logs; wait for assembly interval; mock prove should be fast |
| Balances off by fee | Record actuals; do not change the intended asset amounts mid-run |
| `deriveKey` 500 `INTERNAL_SERVER_ERROR` | Use the 24-word mnemonic from `nightfall_test.toml` (`key_request` Client 1, `key_request2` Client 2). The Foundry 12-word `test ... junk` phrase is L1-only |
| `Invalid recipient public key: Invalid string` | The JSON still has the literal `C2_ZKP` / `C1_ZKP`. Export the hex key, then use `\"${C2_ZKP}\"` so the shell interpolates it |
| `Invalid tokenId: Invalid hex format` | Token IDs must be even-length hex. Use the exported `$ID*` values (`0x03ea` / `0x0bba`), not `0x3ea` / `0xbba` |
| `/v1/balance` `No such token` | No unspent commitments for that token. Client 2 ERC20 404s until the transfer block. If proposer has `Found 0 client transactions`, restart Client 1, `deriveKey`, resubmit transfers |
| Withdraw L2 done, L1 unchanged | Wait for descrow; check `webhook salts` and Nightfall `descrow_funds` |

Do not run `--network testnet` in this document.
