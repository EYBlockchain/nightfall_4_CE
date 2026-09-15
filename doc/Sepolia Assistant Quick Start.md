# Sepolia Assistant Quick Start

Run a complete Nightfall check on **Ethereum Sepolia** with `nf4`. The sequence matches `doc/sepolia_testing_report_template.md`: one deployer, one proposer, two clients, all four token standards, late Client 2 join, Client 1 stop/recover, proposer restart, then a post-recovery deposit.

Client 1 and Client 2 are separate client processes. Put them on two VMs, or on one host with different ports. Stand-up (deploy, proposer, bootstrap a client VM) is `doc/Testnet Assistant Quick Start.md`. This file is the ordered report using those nodes.

This is not Anvil and not Base Sepolia. Chain ID must be `11155111`. Do not use `--network local`, Anvil’s well-known keys, or `./scripts/nf4-local-e2e.sh`. `--yes` is refused on testnet.

Work from the repository root on `auto/testnet`. Record evidence as you go (command output, logs, `cast` receipts, explorer links). Do not put private keys, mnemonics, or RPC secrets in the report.

For the Anvil dry run of the same sequence, see `doc/Local Assistant Quick Start.md`.

## Report values

| Report field | Sepolia value |
|---|---|
| Environment | Ethereum Sepolia |
| Chain ID | `11155111` |
| RPC | Host and containers: your `wss://` URL. `cast send` / `cast call` / `cast balance`: matching `https://` URL from the same provider |
| Run mode | `sepolia` (`NF4_RUN_MODE=sepolia`) |
| Network | `testnet` (`NF4_NETWORK=testnet`) |
| Prover | `mock` in this walkthrough (wizard default). Official report wants `real`; see **Real prover** under step 2 |
| Explorer | `https://sepolia.etherscan.io` |
| Client 1 API | `$C1_API` (`http://127.0.0.1:3000` on the Client 1 VM) |
| Client 2 API | `$C2_API` (`http://127.0.0.1:3000` on the Client 2 VM; same machine `:3002`) |
| Proposer API | `$PROP_API` (`http://127.0.0.1:3001` on the Operator VM) |
| Configuration | `http://<operator-ip>:8080` from every VM. On Operator, host checks can use `http://127.0.0.1:8080`. Do not type `host.docker.internal`, `http://configuration:80`, or `http://indie-proposer:3000` |
| 48-hour soak | Required for the official report. Note the actual duration if you stop early |

Out of scope, same as the Sepolia template: second proposer, rotation, proposer failover.

There is no unattended copy of this guide. `--yes` is local-only. On testnet, key prompts accept a paste **or** Enter to generate a new account.

## Where to run each command

Copy the matching block from `doc/Testnet Assistant Quick Start.md`.

Split VMs:

```bash
export PROP_API=http://127.0.0.1:3001    # Operator VM shells
export C1_API=http://127.0.0.1:3000      # Client 1 VM shells
export C2_API=http://127.0.0.1:3000      # Client 2 VM shells
export C1_CONTAINER=nf4_indie_client
export C2_CONTAINER=nf4_indie_client
export C1_DB=nf4_db_client
export C2_DB=nf4_db_client
export PROP_CONTAINER=nf4_indie_proposer
export PROP_DB=nf4_db_proposer
```

Same machine:

```bash
export PROP_API=http://127.0.0.1:3001
export C1_API=http://127.0.0.1:3000
export C2_API=http://127.0.0.1:3002
export C1_CONTAINER=nf4_indie_client
export C2_CONTAINER=nf4_indie_client2
export C1_DB=nf4_db_client
export C2_DB=nf4_db_client2
export PROP_CONTAINER=nf4_indie_proposer
export PROP_DB=nf4_db_proposer
```

Run Operator steps on the Operator VM, Client 1 curls on the Client 1 VM, Client 2 curls on the Client 2 VM. Share token addresses, `C2` (L1 address), and ZKP public keys across VMs. Do not copy private keys.

## Accounts

Do **not** use Foundry Anvil keys. Those accounts have no Sepolia ETH and must never be used on a public chain.

You need two Ethereum Sepolia accounts (same split as the local guide):

| Role | How to get it |
|---|---|
| Deployer / default proposer / Client 1 L1 | Paste a funded key, or press Enter in `wizard deploy` / Client 1 `wizard client` |
| Client 2 L1 | On the Client 2 VM: paste a funded key, or press Enter in **that** VM’s `wizard client`. Or `cast wallet new` there early, fund it, and share only the address with Client 1 for minting |

A generated account starts at **0 ETH**. The wizard prints the address and private key once, writes the key to **that VM’s** `local.env` (gitignored), and waits until `cast balance` is non-zero (or you skip the recheck). Fund about **0.5 Sepolia ETH per account** before continuing. Never paste keys into the report or into chat.

If you already have keys, export them in this shell only (they stay in history; do not screenshot):

```bash
export C1=0xYOUR_CLIENT1_ADDRESS
export C2=0xYOUR_CLIENT2_ADDRESS
export KEY1=0xYOUR_CLIENT1_PRIVATE_KEY
export KEY2=0xYOUR_CLIENT2_PRIVATE_KEY
export RPC_WSS=wss://YOUR_SEPOLIA_WEBSOCKET_URL
export RPC_HTTP=https://YOUR_SEPOLIA_HTTPS_URL
export EXPLORER=https://sepolia.etherscan.io
```

If the wizard generated the accounts, copy `C1` / `C2` / `KEY1` / `KEY2` from its output (or from `local.env`) for the `cast send` mint steps. Still set `RPC_WSS`, `RPC_HTTP`, and `EXPLORER`.

Wizard key prompts hide input when you paste. Include `0x` if the key has it.

ZKP `deriveKey` still uses the 24-word test mnemonics from `nightfall_test.toml`. Those are not L1 keys and do not replace `KEY1` / `KEY2`.

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

Confirm you are on Sepolia and both accounts have ETH:

```bash
cast chain-id --rpc-url "$RPC_WSS"
cast balance "$C1" --rpc-url "$RPC_HTTP"
cast balance "$C2" --rpc-url "$RPC_HTTP"
```

Expect `11155111` and non-zero balances. An `http://` RPC will fail later in the wizard; Nightfall subscribes to logs and needs `ws://` or `wss://`. Public Sepolia should be `wss://`.

If a previous **local Anvil** stack is still up, stop it first. Do not mix `NF4_NETWORK=local` with this run.

```bash
docker compose --profile anvil --profile configuration --profile indie-deployer \
  --profile indie-proposer --profile indie-client --profile indie-client2 --profile no_test \
  down --remove-orphans
```

That only stops local containers. It does **not** remove anything already deployed on Sepolia.

## Do not “reset the chain”

Sepolia is persistent. `docker compose down` does not undeploy contracts. Old Nightfall addresses stay on chain and keep costing nothing until you abandon them.

| Intent | What to do |
|---|---|
| Reuse contracts from a deploy that already exited 0 | Do **not** run `wizard deploy` again. Continue at proposer/client. `NF4_CONTRACTS__DEPLOY_CONTRACTS` must be `false` after the first successful deploy |
| Wizard failed before deployer exited 0 | Fix the prompt/RPC/key, run `wizard deploy --network testnet` again |
| Proposer/client wizard failed | Re-run that wizard only |
| Client 2 container bad | Split VM: `docker rm -f "$C2_CONTAINER"` then `wizard client` on that VM. Same machine: then `./scripts/nf4 up client2` |
| Want **new** Sepolia contracts | `wizard deploy --network testnet` again. You pay gas. Previous addresses are left on chain. Update the report with the new ones |

Leave `nightfall.toml` and `local.env` in place when reusing. Confirm `local.env` is gitignored.

`nf4` does **not** rewrite LAN URLs when `NF4_NETWORK=testnet`. If Wi-Fi/VPN changes the laptop IP mid-run, the on-chain proposer URL (`http://<old-lan-ip>:3001`) stops working. Stay on one network for the run.

Partial retries (do **not** treat this like Anvil full reset):

| Stuck at | Retry |
|---|---|
| RPC / chain ID | Step 0. Do not deploy until `cast chain-id` is `11155111` |
| Deploy wizard | `./scripts/nf4 wizard deploy --network testnet` |
| Proposer wizard failed | `./scripts/nf4 wizard proposer` again |
| Client 1 wizard failed | `./scripts/nf4 wizard client` again |
| Client 2 missing key | On the Client 2 VM run `wizard client` (or same-machine: Yes to local Client 2, then `nf4 up client2`) |
| Deploy, proposer, and client already healthy | Skip to cert/keys/mock tokens. Do not run `wizard deploy` again |

## 1. RPC check (no Anvil)

There is no Anvil step. The host chain is public Sepolia.

```bash
cast chain-id --rpc-url "$RPC_WSS"
cast block-number --rpc-url "$RPC_WSS"
```

Record the block number. The deploy wizard uses the current block as `genesis_block` so clients do not scan from genesis.

Optional, if the provider rate-limits (Alchemy free tier is tight): after deploy, add `NF4_RPC_RATE_LIMIT=8` to `local.env` and recreate proposer/client. Do not put the RPC URL in the report.

## 2. Deploy Nightfall (report: deployer, contracts, config)

```bash
./scripts/nf4 wizard deploy --network testnet
```

| Prompt | Enter |
|---|---|
| Testnet chain | `sepolia` (not `base_sepolia`) |
| Profile | `sepolia` |
| RPC | `$RPC_WSS` (`wss://…`). No `http://`, no Anvil `ws://127.0.0.1:8545` |
| Configuration port | `8080` |
| Configuration URL | Accept the default (`http://<detected-lan-ip>:8080`). Do not use `host.docker.internal` |
| Deployer key | Paste `KEY1` (hidden), or Enter to generate a new account, fund the printed address, then recheck balance |
| Default proposer address | Enter (deployer / Client 1) |
| Default proposer URL | Accept `http://<lan-ip>:3001`. This is written **on chain**. Do not use `http://indie-proposer:3000` (that default is local-only). Host health is still `${PROP_API}` |
| Real prover? | `no` for this walkthrough |
| Block size | `64` |
| Apply changes? | `yes` after the review shows `network: testnet`, `profile: sepolia`, `chain_id: 11155111` |

`--yes` is not accepted here. There is no typed “deploy to sepolia” phrase; that exists only for mainnet.

`nf4` clones the `[base_sepolia]` TOML template into `[sepolia]` and sets `chain_id` from the RPC. Containers get `NF4_ETHEREUM_CLIENT_URL` as the same `wss://` URL (not rewritten to `anvil`).

After `Deployment OK`:

```bash
./scripts/nf4 status
curl -sS http://127.0.0.1:8080/configuration/toml/addresses.toml
curl -sS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/configuration/toml/contract_hashes.toml
curl -sS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/configuration/bin/keys/proving_key
docker inspect -f '{{.State.Status}} {{.State.ExitCode}}' nf4_indie_deployer
```

Record:

- `NF4_RUN_MODE=sepolia`, `NF4_NETWORK=testnet`, `NF4_MOCK_PROVER=true`, `NF4_CONTRACTS__DEPLOY_CONTRACTS` was `true` for this run
- Nightfall, Round Robin, X509 addresses, `code OK`, and explorer links (`$EXPLORER/address/<addr>`)
- Verifier skipped in mock mode (report: Verifier `not deployed / mock`)
- VK provider if present in `addresses.toml`
- Deployer container exited `0`
- Deploy transactions on the explorer

```bash
grep -E 'NF4_RUN_MODE|NF4_NETWORK|NF4_MOCK_PROVER|NF4_CONTRACTS__DEPLOY_CONTRACTS' local.env
```

Do not attach `local.env` to the report. After a successful first deploy, later proposer/client restarts must keep `NF4_CONTRACTS__DEPLOY_CONTRACTS=false` (the proposer wizard already writes that).

If deployer, proposer, and client are already `healthy` and `curl http://127.0.0.1:8080/configuration/toml/addresses.toml` is 200, skip to **Client 1 cert + keys** (after step 4). First:

```bash
./scripts/nf4 status
docker compose --profile indie-proposer --profile indie-client --env-file local.env up -d
curl -sS ${PROP_API}/v1/health
curl -sS ${C1_API}/v1/health
```

### Real prover

`doc/sepolia_testing_report_template.md` asks for prover mode `real`. This walkthrough uses mock so a laptop can finish the sequence.

To match the official report instead:

1. `./scripts/nf4 check prover` must pass before you treat the machine as capable
2. At “Use real prover mode?” answer `yes`, then confirm key generation
3. Key generation needs large RAM/disk and can take a long time. Success generating keys does not prove this machine can prove a block
4. Record a non-zero Verifier address

Do not switch prover mode on an already-deployed Sepolia profile mid-run.

## 3. Proposer (report: one proposer, health, cert)

```bash
./scripts/nf4 wizard proposer
```

| Prompt | Enter |
|---|---|
| Proposer key | Paste `KEY1` (hidden), or Enter to reuse `local.env`. If `local.env` has no proposer key, Enter generates a new account |
| Public proposer URL | Accept the default (current LAN IP, port 3001). Health on the Mac is `${PROP_API}` |
| Configuration URL | Accept the default (current LAN IP, port 8080) |

```bash
curl -sS -i ${PROP_API}/v1/health
curl -i --request POST "${PROP_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
curl -sS ${PROP_API}/v1/proposers
./scripts/nf4 logs proposer
```

Health must be HTTP 200 and body `Healthy`. Exactly one proposer registered. Save a log excerpt that it connected to chain `11155111`, plus the certification L1 tx on the explorer.

## 4. Client 1 only (report step 1: Client 2 still stopped)

On the **Client 1 VM** (see `doc/Testnet Assistant Quick Start.md` to bootstrap a second machine). Client 2 stays down.

If Client 2 already has an L1 address, set `CLIENT2_ADDRESS` in Client 1’s `local.env` before mock tokens. Do not copy Client 2’s private key onto this VM.

```bash
./scripts/nf4 wizard client
```

| Prompt | Enter |
|---|---|
| Client key | Paste `KEY1` (hidden), or Enter to reuse `local.env` / generate if empty |
| Client address | Enter (derived Client 1) |
| Will Client 2 also run on this machine? | **No** on a Client 1 VM. **Yes** only if this host will also run `nf4 up client2` |
| Proposer URL | Operator URL `http://<operator-ip>:3001`. If proposer is not up, stop and run `wizard proposer` on Operator |
| Configuration URL | `http://<operator-ip>:8080` |
| Webhook | Accept the default (this VM’s LAN IP, port 8081) |
| Client API port | `3000` |

Host checks always use `127.0.0.1` even when `local.env` has the LAN IP.

### Client 1 cert + keys

```bash
curl -sS -i ${C1_API}/v1/health
curl -i --request POST "${C1_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
curl -sS -X POST ${C1_API}/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS ${C1_API}/v1/synchronisation
curl -sS ${C1_API}/v1/commitments
curl -sS ${C1_API}/v1/l1_balance
```

This is the 24-word `key_request` mnemonic from `nightfall_test.toml`, not `KEY1`. Export Client 1’s compressed ZKP public key from the stored result (64 hex chars, no `0x`). Do not paste the name `C1_ZKP` into later JSON.

```bash
export C1_ZKP=$(curl -sS -X POST ${C1_API}/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C1_ZKP"
```

Save L2 sync payload as the baseline block. Record the certification tx on the explorer.

Confirm Client 2 is not running:

```bash
docker ps --filter name="$C2_CONTAINER" --format '{{.Names}} {{.Status}}'
```

Empty is correct.

## 5. Mock tokens and mint the report IDs

Point ERC20’s second allocation at Client 2, then deploy mocks **on Sepolia** (this costs gas):

```bash
./scripts/nf4 client deploy-mock-tokens
```

Copy the four printed addresses and the broadcast txs. Export them:

```bash
export TOKEN20=<ERC20Mock>
export TOKEN721=<ERC721Mock>
export TOKEN1155=<ERC1155Mock>
export TOKEN3525=<ERC3525Mock>
export NF=$(awk -F'"' '/^nightfall *=/{print $2}' configuration/toml/addresses.toml)
export RPC="$RPC_HTTP"
# even-length hex for the client API (0x03ea not 0x3ea)
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
```

Mocks mint leftover IDs (ERC721 `426`, ERC1155 `2`/`73`, ERC3525 `7`/`8`). Mint the report IDs with `KEY1` (Client 1 pays):

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

Wait for each receipt. Record `$EXPLORER/tx/<hash>` and `$EXPLORER/address/$TOKENxx`.

## 6. Report step 1 — baseline

Keep Client 2 stopped.

```bash
curl -sS ${PROP_API}/v1/health
curl -sS ${C1_API}/v1/health
curl -sS ${C1_API}/v1/synchronisation
curl -sS ${C1_API}/v1/commitments
curl -sS "${C1_API}/v1/balance/${TOKEN20}/${ID20}"
cast block-number --rpc-url "$RPC"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
```

Record the Sepolia block as the explorer reference.

## 7. Report steps 2–4 — Client 1 deposits, wait for a block

Sepolia L1 is ~12s per block. The proposer may wait up to `block_assembly_max_wait_secs` (120s in the cloned template), then prove, then wait for L1 confirmation. Poll; do not assume Anvil timing.

```bash
# ERC20 100
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC721 1001
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC1155 2001 x 20
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x14\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
# ERC3525 3001 value 100
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

The id is the `x-request-id` header, not the `"Request queued"` body. `-D -` prints headers. Poll until confirmed:

```bash
curl -sS ${C1_API}/v1/request/<REQUEST_ID>
./scripts/nf4 webhook events
./scripts/nf4 logs proposer
```

For each deposit, record request ID, L1 escrow tx, and `$EXPLORER/tx/<hash>`. X509 validate is `0x4e5805d3`. Escrow is `Nightfall.escrow_funds` / `0xe6d5abe5`. A proposed L2 block is `Nightfall.propose_block` / `0x55420851`.

Then:

```bash
curl -sS ${C1_API}/v1/synchronisation
curl -sS "${C1_API}/v1/balance/${TOKEN20}/${ID20}"
curl -sS "${C1_API}/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "${C1_API}/v1/balance/${TOKEN1155}/${ID1155}"
curl -sS "${C1_API}/v1/balance/${TOKEN3525}/${ID3525_C1}"
curl -sS ${C1_API}/v1/commitments
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Expect Client 1: ERC20 `100`, ERC721 `1001`, ERC1155 `2001:20`, ERC3525 `3001:100` (hex `0x64`, `01`, `0x14`, `0x64` if no fees). ERC721 `/v1/balance` is ownership count (`01` if held), not the on-chain value field. Record L2 block number.

## 8. Report steps 5–7 — Client 2 joins late, then deposits

Start Client 2 after the deposit block is confirmed.

- **Client 2 VM:** bootstrap as in the Testnet guide (if not already), then `./scripts/nf4 wizard client` with Client 2’s key. Answer **No** to “Will Client 2 also run on this machine?”. API port `3000`.
- **Same machine:** `CLIENT2_SIGNING_KEY` must be in `local.env`, then `./scripts/nf4 up client2`. If an old container exists, `docker rm -f "$C2_CONTAINER"` first.

```bash
curl -sS -i ${C2_API}/v1/health
curl -i --request POST "${C2_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.priv_key;type=application/octet-stream'
curl -sS -X POST ${C2_API}/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS ${C2_API}/v1/synchronisation
curl -sS ${C2_API}/v1/commitments
```

`deriveKey` uses the 24-word `key_request2` mnemonic from `nightfall_test.toml`. Do not use a Foundry 12-word L1 phrase; that returns `500`.

Client 2 must reach the same confirmed L2 block as Client 1 without error. Export Client 2’s compressed ZKP public key from the stored result (64 hex chars, no `0x`). Do not paste the name `C2_ZKP` into later JSON.

```bash
export C2_ZKP=$(curl -sS -X POST ${C2_API}/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C2_ZKP"
```

Client 2 deposits:

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

Wait for the next proposed block and explorer confirmation, then verify Client 2 Mongo/API: `1002`, `2001:10`, `3002:100`.

## 9. Report step 8 — transfers from Client 1, Client 2 still up

`C2_ZKP` and the `$ID*` values must already be exported. The curls interpolate `${C2_ZKP}` and `${ID…}`; do not leave those names as JSON strings.

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"recipientData\":{\"values\":[\"0x08\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
```

Use `-D -` so you get `x-request-id`. Poll `/v1/request/<id>` until accepted for **all four** transfers. Do **not** wait for L2 confirmation yet. Do not stop Client 1 if a transfer returned `Invalid recipient public key` or never queued. Proposer logs should show client transactions in the mempool (`Found N client transactions`, N > 0) before you continue.

## 10. Report step 9 — stop Client 1 before the transfer block confirms

```bash
curl -sS ${C1_API}/v1/synchronisation
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker stop "$C1_CONTAINER"
date -u
```

Record timestamp, last Client 1 L2 block, and DB dump.

## 11. Report step 10 — proposer confirms transfers; Client 2 stays up

`/v1/balance` returns `No such token` (404) when there are no unspent commitments for that token, not `00`. Client 2 never deposits ERC20, so `${TOKEN20}/${ID20}` 404s until the C1→C2 transfer is in an L2 block. That is not a bad token address.

If proposer logs show `Found 0 client transactions` / `No transactions pending`, the transfers never left Client 1. Start Client 1, re-run `deriveKey` (keys are in-memory), resubmit any still-`Unspent` transfers, poll until accepted, then `docker stop nf4_indie_client` again if you still need that report beat.

```bash
./scripts/nf4 logs proposer
curl -sS ${C2_API}/v1/synchronisation
curl -sS ${C2_API}/v1/commitments
curl -sS "${C2_API}/v1/balance/${TOKEN20}/${ID20}"
curl -sS "${C2_API}/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "${C2_API}/v1/balance/${TOKEN1155}/${ID1155}"
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Client 2 must show received ERC20 `40`, ERC721 `1001`, ERC1155 `18` (`10+8`), ERC3525 received value `40`. Record the propose tx on the explorer.

## 12. Report step 11 — restart Client 1

ZKP keys are in-memory. After `docker start`, run Client 1 `deriveKey` again (same 24-word `key_request` mnemonic) before checking commitments.

```bash
docker start "$C1_CONTAINER"
curl -sS -i ${C1_API}/v1/health
curl -sS -X POST ${C1_API}/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS ${C1_API}/v1/synchronisation
curl -sS ${C1_API}/v1/commitments
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Client 1 must catch up to the transfer block. Spent commitments for the transferred assets; remaining ERC20 `60`, ERC1155 `12`, ERC3525 `60`; ERC721 `1001` spent.

## 13. Report step 12 — compare both clients and proposer

```bash
curl -sS ${C1_API}/v1/synchronisation
curl -sS ${C2_API}/v1/synchronisation
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
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
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C1_ZKP}\"]},\"fee\":\"0x00\"}"
```

Wait for a block and explorer confirmation. Both DBs: Client 1 holds `1001` again; Client 2 does not.

## 15. Report step 14 — withdrawals

Client 1:

```bash
curl -sS -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
```

Client 2:

```bash
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x12\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
```

Wait for L2 inclusion **and** L1 descrow (`Nightfall.descrow_funds` / `0xf3b85fc2`). For each request id, save `/v1/request/<id>`, webhook events, explorer hashes, and:

```bash
./scripts/nf4 webhook salts
cast balance "$C1" --rpc-url "$RPC"
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C1" --rpc-url "$RPC"
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C2" --rpc-url "$RPC"
```

Do not mark the step pass until L1 balances move.

## 16. Report steps 15–16 — restart proposer

```bash
curl -sS ${C1_API}/v1/synchronisation
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
date -u
docker stop "$PROP_CONTAINER"
date -u
docker start "$PROP_CONTAINER"
curl -sS -i ${PROP_API}/v1/health
./scripts/nf4 logs proposer
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
curl -sS ${C1_API}/v1/synchronisation
curl -sS ${C2_API}/v1/synchronisation
```

Record downtime. After restart, health `Healthy`, prior L2 history still in proposer Mongo, both clients agree on the same L2 block.

`NF4_CONTRACTS__DEPLOY_CONTRACTS` must stay `false` on restart (the proposer wizard already wrote that).

## 17. Report step 17 — deposit after recovery

Client 1 ERC20 `10` (`0x0a`):

```bash
curl -sS -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

Wait for a new block and explorer confirmation. Client 1 commitments must include the new `10`.

## 18. Report steps 18–19 — soak (48 hours for the official report)

Leave the proposer running. At start, +24h, and end:

```bash
date -u
curl -sS -i ${PROP_API}/v1/health
curl -sS ${C1_API}/v1/synchronisation
docker stats --no-stream "$PROP_CONTAINER" "$C1_CONTAINER" "$C2_CONTAINER"
```

Official target is 48 hours. If you stop early, write the actual duration and mark soak skipped/blocked in the report. After soak, Client 2 deposits ERC20 `10` and the proposer must produce a block while still `Healthy`.

## Status snapshot

```bash
./scripts/nf4 status
docker ps -a --filter name=nf4_ --format '{{.Names}}\t{{.Status}}'
```

When the run is complete:

- Status shows `Network: testnet`, `NF4_RUN_MODE=sepolia`, chain `11155111`
- Deployer exited 0; configuration, proposer, both clients running
- Health 200 `Healthy` on `$C1_API`, `$PROP_API`, `$C2_API`
- Configuration files reachable on `http://127.0.0.1:8080`
- Mongo `nightfall.commitments` and `nightfall.ProposedBlocks` populated on client1, client2, and proposer
- Fill `doc/sepolia_testing_report_template.md` (redact keys, mnemonics, RPC URLs)

## Failures

| Symptom | Action |
|---|---|
| `cast chain-id` is `31337` | You are on Anvil. Stop. This document is Sepolia only |
| `cast chain-id` is `84532` | That is Base Sepolia. Restart deploy and choose `sepolia` |
| RPC `http://` rejected / no subscriptions | Use `wss://` in the wizard. Keep `https://` only for `cast send` / `cast call` |
| RPC 401/403 | Bad or missing API key. Do not paste the URL into the report |
| RPC 429 / timeouts | Provider limit. Set `NF4_RPC_RATE_LIMIT=8` in `local.env`, recreate proposer/client |
| `--yes is only supported with --network local` | Drop `--yes`. Review the prompts |
| Anvil key / zero Sepolia balance | Fund real accounts. Never use `0xac0974…` / `0x59c6995e…` |
| `CLIENT2_SIGNING_KEY is missing` | Same-machine only. Run `wizard client`, answer Yes to local Client 2, or start Client 2 on its own VM with `wizard client` |
| Wizard failed on `host.docker.internal`, but `127.0.0.1:8080` returns 200 and deployer exited 0 | Do not redeploy (you would pay for new contracts). Continue at proposer |
| Transfers fail with connection refused to an old LAN `:3001` | Client posts to the **on-chain** Round Robin URL. Testnet deploy registers `http://<lan-ip>:3001`. Recreating containers does not update Sepolia. Stay on one network, or you need a new deploy |
| Need new contracts | `wizard deploy --network testnet` again. Docker down does not undeploy Sepolia |
| `dependency failed to start: container nf4_db_proposer is unhealthy` | Docker Linux kernel is too new for `mongo:8.0`. This repo uses `mongo:8.2`. Recreate the DB container, then rerun `wizard proposer` (do not redeploy contracts) |
| `development` / `production` profile rejected | Use `sepolia` |
| Client 2 cannot reach config/proposer | Do not use `127.0.0.1` as the configuration or on-chain proposer URL. Use the Operator IP. On a Client 2 VM run `wizard client` with those URLs |
| Deposits sit in mempool | Proposer logs; wait for assembly interval and L1 confirm. Mock prove is fast; L1 is not |
| Balances off by fee | Record actuals; do not change the intended asset amounts mid-run |
| `deriveKey` 500 `INTERNAL_SERVER_ERROR` | Use the 24-word mnemonic from `nightfall_test.toml` (`key_request` Client 1, `key_request2` Client 2). L1 keys are `KEY1` / `KEY2` only |
| `Invalid recipient public key: Invalid string` | The JSON still has the literal `C2_ZKP` / `C1_ZKP`. Export the hex key, then use `\"${C2_ZKP}\"` so the shell interpolates it |
| `Invalid tokenId: Invalid hex format` | Token IDs must be even-length hex. Use the exported `$ID*` values (`0x03ea` / `0x0bba`), not `0x3ea` / `0xbba` |
| `/v1/balance` `No such token` | No unspent commitments for that token. Client 2 ERC20 404s until the transfer block. If proposer has `Found 0 client transactions`, restart Client 1, `deriveKey`, resubmit transfers |
| Withdraw L2 done, L1 unchanged | Wait for descrow; check `webhook salts`, explorer `descrow_funds`, and Nightfall L1 balances |

Do not run `--network local` in this document.
