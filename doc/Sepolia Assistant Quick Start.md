# Sepolia Assistant Quick Start

Two-VM Sepolia run with `nf4`. Fill `temp/sepolia_testing_report_template.md` as you go. Real prover. Chain ID `11155111`. Branch `auto/testnet`. No Anvil, no `--yes`, no `--network local`.

```text
VM-A                              VM-B
deploy once                       Client 2 :3000
configuration :8080               db_client
proposer :3001                    own key, own Mongo
Client 1 :3000
```

VM-B uses `indie-client` on `:3000`, not `indie-client2`. VM-B never runs `wizard deploy`. Do not copy `local.env` or private keys. Request id is the `x-request-id` header (`curl -D -`). Token IDs must be even-length hex (`0x03ea` not `0x3ea`). Fees `"0x00"`. ERC721 `value` is `"0x00"`. `deriveKey` uses the 24-word phrases below, not L1 keys. Do not put keys, mnemonics, or RPC URLs in the report.

| Token | type | id | C1 deposit | C2 deposit | C1→C2 | C1 withdraw | C2 withdraw |
|---|---|---|---|---|---|---|---|
| ERC20 | `0` | `ID20=0x00` | `0x64` (100) | — | `0x28` (40) | `0x3c` (60) | `0x28` (40) |
| ERC721 | `2` | C1 `ID721_C1=0x03e9` (1001); C2 `ID721_C2=0x03ea` (1002) | 1001 | 1002 | 1001, then back | 1001 | 1002 |
| ERC1155 | `1` | `ID1155=0x07d1` (2001) | `0x14` (20) | `0x0a` (10) | `0x08` (8) | `0x0c` (12) | `0x12` (18) |
| ERC3525 | `3` | C1 `ID3525_C1=0x0bb9` (3001); C2 `ID3525_C2=0x0bba` (3002); slot `1` | `0x64` | `0x64` | `0x28` from 3001 | `0x3c` | `0x28` + original 3002 |

Balances in that table are pre-fee. Record post-fee values from the API and Mongo.

Client 1 `deriveKey` mnemonic (`key_request`):

`spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work`

Client 2 `deriveKey` mnemonic (`key_request2`):

`wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law`

---

## 1. Prep — both VMs

Need Docker, `forge`, `cast`, `cargo`, `curl`. ~0.5 Sepolia ETH per L1 account. VM-A is the proving box.

```bash
git switch auto/testnet
git pull
cargo test -p nightfall_ops
./scripts/nf4 check deployer
```

All `check deployer` lines `OK`. On VM-A also:

```bash
./scripts/nf4 check prover
```

Stop a leftover Anvil stack if present (`docker compose down` does **not** undeploy Sepolia contracts):

```bash
docker compose --profile anvil --profile configuration --profile indie-deployer \
  --profile indie-proposer --profile indie-client --profile indie-client2 --profile no_test \
  down --remove-orphans
```

---

## 2. VM-A IP — VM-A, then VM-B

`$VM_A_IP` is the IPv4 VM-B uses to reach VM-A `:8080` and `:3001`. Not `127.0.0.1`, not `localhost`, not a Docker bridge (`172.17.` / `172.28.`).

Linux: `hostname -I` — first address that is not `127.*` or `172.*`. If several remain, pick the one VM-B can reach (LAN/VPC, or the cloud public IPv4).

macOS: `ipconfig getifaddr en0 || ipconfig getifaddr en1`

Cloud across the internet: provider public IPv4. Open TCP `8080` and `3001` to VM-B.

```bash
export VM_A_IP=<that address>
printf '%s\n' "$VM_A_IP"
```

On VM-B, export the **same** value and `ping -c 1 "$VM_A_IP"`. If ping is blocked, step 5’s `curl` from VM-B is the check. Do not change this IP mid-run; it is written on chain.

---

## 3. Shell env — RPC and URLs

You do not have Nightfall accounts yet. Export only what you already know. `127.0.0.1` is this VM.

**Both VMs.** `./scripts/nf4-pick-rpc` reads chainlist.org, probes `eth_chainId` with `cast`, and prints the fastest working `https://` and `wss://` pair. Default `--network sepolia`. A passing chain-id does not prove `eth_subscribe` will hold; if the event listener drops, use a keyed provider.

```bash
eval "$(./scripts/nf4-pick-rpc --network sepolia)"
export RPC="$RPC_HTTP"
export EXPLORER=https://sepolia.etherscan.io
export CONFIG_URL=http://${VM_A_IP}:8080
export PROPOSER_URL=http://${VM_A_IP}:3001
printf '%s\n' "$RPC_HTTP" "$RPC_WSS"
```

**VM-A**

```bash
export PROP_API=http://127.0.0.1:3001
export C1_API=http://127.0.0.1:3000
export C1_CONTAINER=nf4_indie_client
export C1_DB=nf4_db_client
export PROP_CONTAINER=nf4_indie_proposer
export PROP_DB=nf4_db_proposer
```

**VM-B** — `nf4_indie_client` here is Client 2.

```bash
export C2_API=http://127.0.0.1:3000
export C2_CONTAINER=nf4_indie_client
export C2_DB=nf4_db_client
```

```bash
cast chain-id --rpc-url "$RPC_WSS"
cast block-number --rpc-url "$RPC_WSS"
```

Expect `11155111`. Nightfall needs `wss://`. Use `https://` only for `cast`.

---

## 4. L1 accounts

Create and fund wallets **before** deploy. Do not use Anvil keys.

**VM-B** (Client 2 only — keep the key here):

```bash
cast wallet new
export C2=0x<address from cast>
export KEY2=0x<private key from cast>
```

Send `$C2` (address only) to VM-A.

**VM-A** (deployer = proposer = Client 1):

```bash
cast wallet new
export C1=0x<address from cast>
export KEY1=0x<private key from cast>
export C2=0x<address received from VM-B>
```

Fund each address with about 0.5 Sepolia ETH, then:

```bash
cast balance "$C1" --rpc-url "$RPC_HTTP"   # VM-A
cast balance "$C2" --rpc-url "$RPC_HTTP"   # both VMs
```

Expect non-zero. The deploy wizard will ask you to paste `KEY1`.

---

## 5. Deploy — VM-A (report: contracts)

```bash
./scripts/nf4 wizard deploy --network testnet
```

| Prompt | Enter |
|---|---|
| Testnet chain | `sepolia` |
| Profile | `sepolia` |
| RPC | `$RPC_WSS` |
| Configuration port | `8080` |
| Configuration URL | `http://$VM_A_IP:8080` |
| Deployer key | paste `$KEY1` |
| Default proposer address | Enter |
| Default proposer URL | `http://$VM_A_IP:3001` (on chain) |
| Real prover? | `yes`, then confirm key generation |
| Block size | `64` |
| Apply? | `yes` if review is `network: testnet`, `profile: sepolia`, `chain_id: 11155111` |

Key generation is slow. After `Deployment OK`:

```bash
# VM-A
grep -E 'NF4_RUN_MODE|NF4_NETWORK|NF4_MOCK_PROVER|NF4_CONTRACTS__DEPLOY_CONTRACTS' local.env
curl -sS http://127.0.0.1:8080/configuration/toml/addresses.toml
docker inspect -f '{{.State.Status}} {{.State.ExitCode}}' nf4_indie_deployer

# VM-B
curl -sS -o /dev/null -w '%{http_code}\n' "$CONFIG_URL/configuration/toml/addresses.toml"
```

Expect `NF4_MOCK_PROVER=false`, deployer exit `0`, VM-B HTTP `200`, non-zero Verifier address. Record Nightfall / Round Robin / X509 / Verifier and explorer links. After this, keep `NF4_CONTRACTS__DEPLOY_CONTRACTS=false`. If this profile was deployed with mock prover, deploy a new profile; do not flip the flag.

---

## 6. Proposer — VM-A (report: one proposer)

```bash
./scripts/nf4 wizard proposer
```

| Prompt | Enter |
|---|---|
| Proposer key | paste `$KEY1`, or Enter to reuse `local.env` |
| Public proposer URL | `http://$VM_A_IP:3001` |
| Configuration URL | `http://$VM_A_IP:8080` |

```bash
curl -sS -i ${PROP_API}/v1/health
curl -i --request POST "${PROP_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
curl -sS ${PROP_API}/v1/proposers
./scripts/nf4 logs proposer
```

On VM-B: `curl -sS -o /dev/null -w '%{http_code}\n' "$PROPOSER_URL/v1/health"`

Expect HTTP 200 `Healthy`, exactly one proposer, VM-B `200`. Record cert L1 tx.

---

## 7. Client 1 — VM-A (report step 1: Client 2 down)

Do not run `wizard client` on VM-B yet. Put `CLIENT2_ADDRESS=$C2` in VM-A `local.env`.

```bash
./scripts/nf4 wizard client
```

| Prompt | Enter |
|---|---|
| Client key | paste `$KEY1` |
| Client address | Enter |
| Will Client 2 also run on this machine? | **No** |
| Proposer URL | `$PROPOSER_URL` |
| Configuration URL | `$CONFIG_URL` |
| Webhook | default (VM-A, port 8081) |
| Client API port | `3000` |

```bash
docker exec "$C1_CONTAINER" wget -q -S -O /dev/null "$CONFIG_URL/configuration/toml/addresses.toml"
docker exec "$C1_CONTAINER" wget -q -S -O /dev/null "$PROPOSER_URL/v1/health"
curl -sS -i ${C1_API}/v1/health
curl -i --request POST "${C1_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
curl -sS -X POST ${C1_API}/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
export C1_ZKP=$(curl -sS -X POST ${C1_API}/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C1_ZKP"
```

Expect wget/health 200, 64 hex chars in `C1_ZKP` (no `0x`). Copy `C1_ZKP` to VM-B before step 17.

On **VM-B**: `docker ps --filter name=nf4_indie_client` must be empty. Do not run that check on VM-A.

---

## 8. Mock tokens — VM-A

```bash
./scripts/nf4 client deploy-mock-tokens
```

```bash
export TOKEN20=<ERC20Mock>
export TOKEN721=<ERC721Mock>
export TOKEN1155=<ERC1155Mock>
export TOKEN3525=<ERC3525Mock>
export NF=$(awk -F'"' '/^nightfall *=/{print $2}' configuration/toml/addresses.toml)
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
```

Copy those exports to VM-B before step 11.

```bash
cast send "$TOKEN721" "mint(address,address,uint256)" "$C1" "$NF" 1001 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN721" "mint(address,address,uint256)" "$C2" "$NF" 1002 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C1" "$NF" 2001 20 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C2" "$NF" 2001 10 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C1" 3001 1 100 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C2" 3002 1 100 --private-key "$KEY1" --rpc-url "$RPC"
```

Wait for receipts. Record explorer txs.

---

## 9. Baseline — VM-A (report step 1)

Client 2 still down on VM-B.

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

Record Sepolia block and L2 sync payload.

---

## 10. Client 1 deposits — VM-A (report steps 2–4)

Poll. Assembly can wait ~120s, then real proving (tens of minutes), then L1 (~12s/block).

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x14\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

```bash
curl -sS ${C1_API}/v1/request/<REQUEST_ID>
./scripts/nf4 webhook events
./scripts/nf4 logs proposer
```

Escrow `0xe6d5abe5`. Propose block `0x55420851`. X509 validate `0x4e5805d3`.

```bash
curl -sS ${C1_API}/v1/synchronisation
curl -sS "${C1_API}/v1/balance/${TOKEN20}/${ID20}"
curl -sS "${C1_API}/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "${C1_API}/v1/balance/${TOKEN1155}/${ID1155}"
curl -sS "${C1_API}/v1/balance/${TOKEN3525}/${ID3525_C1}"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Expect ERC20 100, ERC721 1001 (balance `01` = held), ERC1155 20, ERC3525 100. Record L2 block.

---

## 11. Client 2 late join — VM-B (report steps 5–7)

Only after step 10’s deposit block is confirmed. Copy `$TOKEN*`, `$ID*`, `$NF`, `$RPC`, `$CONFIG_URL`, `$PROPOSER_URL` from VM-A.

```bash
scp user@vm-a:path/to/nightfall_4_CE/nightfall.toml ./nightfall.toml
mkdir -p configuration/toml configuration/bin/keys
curl -sS "$CONFIG_URL/configuration/toml/addresses.toml" -o configuration/toml/addresses.toml
curl -sS "$CONFIG_URL/configuration/toml/contract_hashes.toml" -o configuration/toml/contract_hashes.toml
curl -sS "$CONFIG_URL/configuration/bin/keys/proving_key" -o configuration/bin/keys/proving_key
cat > local.env <<EOF
NF4_RUN_MODE=sepolia
NF4_NETWORK=testnet
NF4_MOCK_PROVER=false
NF4_CONTRACTS__DEPLOY_CONTRACTS=false
NF4_ETHEREUM_CLIENT_URL=${RPC_WSS}
NF4_CONFIGURATION_URL=${CONFIG_URL}
NF4_NIGHTFALL_PROPOSER__URL=${PROPOSER_URL}
EOF
./scripts/nf4 wizard client
```

| Prompt | Enter |
|---|---|
| Client key | paste `$KEY2` |
| Client address | Enter |
| Will Client 2 also run on this machine? | **No** |
| Proposer URL | `$PROPOSER_URL` |
| Configuration URL | `$CONFIG_URL` |
| Webhook | default (VM-B, port 8081) |
| Client API port | `3000` |

```bash
docker exec "$C2_CONTAINER" wget -q -S -O /dev/null "$CONFIG_URL/configuration/toml/addresses.toml"
docker exec "$C2_CONTAINER" wget -q -S -O /dev/null "$PROPOSER_URL/v1/health"
curl -sS -i ${C2_API}/v1/health
curl -i --request POST "${C2_API}/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.priv_key;type=application/octet-stream'
curl -sS -X POST ${C2_API}/v1/deriveKey \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
export C2_ZKP=$(curl -sS -X POST ${C2_API}/v1/deriveKey \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C2_ZKP"
curl -sS ${C2_API}/v1/synchronisation
```

Paste `C2_ZKP` onto VM-A. Client 2 L2 block must match Client 1.

```bash
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

After the next L2 block: Client 2 has 1002, `2001:10`, `3002:100`.

---

## 12. Transfers — VM-A (report step 8)

`C2_ZKP` must be the hex string, not the characters `C2_ZKP`.

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

Poll `/v1/request/<id>` until all four are accepted. Do not wait for L2 yet. Proposer log `Found N client transactions` with N > 0. Do not stop Client 1 if a transfer failed.

---

## 13. Stop Client 1 — VM-A (report step 9)

Do not stop VM-B.

```bash
curl -sS ${C1_API}/v1/synchronisation
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker stop "$C1_CONTAINER"
date -u
```

Record timestamp, last L2 block, DB dump.

---

## 14. Confirm transfers — VM-A logs, VM-B balances (report step 10)

Client 2 ERC20 `/v1/balance` is 404 until this block (`No such token`). That is expected.

If proposer says `Found 0 client transactions`, start Client 1, `deriveKey`, resubmit, then `docker stop "$C1_CONTAINER"` again.

```bash
# VM-A
./scripts/nf4 logs proposer

# VM-B
curl -sS ${C2_API}/v1/synchronisation
curl -sS ${C2_API}/v1/commitments
curl -sS "${C2_API}/v1/balance/${TOKEN20}/${ID20}"
curl -sS "${C2_API}/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "${C2_API}/v1/balance/${TOKEN1155}/${ID1155}"
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
```

Expect Client 2: ERC20 40, ERC721 1001, ERC1155 18, ERC3525 received 40.

---

## 15. Restart Client 1 — VM-A (report step 11)

Keys are in-memory. `deriveKey` again before reading commitments.

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

Expect Client 1 caught up: ERC20 60, ERC1155 12, ERC3525 60, ERC721 1001 spent.

---

## 16. Compare DBs (report step 12)

L2 block numbers must match.

```bash
# VM-A
curl -sS ${C1_API}/v1/synchronisation
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'

# VM-B
curl -sS ${C2_API}/v1/synchronisation
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
```

| | Client 1 | Client 2 |
|---|---|---|
| ERC20 | 60 | 40 |
| ERC721 | 1001 spent | 1001 and 1002 held |
| ERC1155 2001 | 12 | 18 |
| ERC3525 | 60 on 3001 | 40 received + 3002 |

---

## 17. Return ERC721 1001 — VM-B (report step 13)

Needs `C1_ZKP` from VM-A.

```bash
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/transfer \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C1_ZKP}\"]},\"fee\":\"0x00\"}"
```

After the block: Client 1 holds 1001; Client 2 does not.

---

## 18. Withdraw — C1 on VM-A, C2 on VM-B (report step 14)

Webhook commands on the VM that owns that client.

```bash
# VM-A
curl -sS -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C1_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
```

```bash
# VM-B
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x12\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -H 'Content-Type: application/json' -X POST ${C2_API}/v1/withdraw \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
```

Wait for L2 **and** L1 descrow (`0xf3b85fc2`). Then on that client’s VM:

```bash
./scripts/nf4 webhook salts
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C1" --rpc-url "$RPC"
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C2" --rpc-url "$RPC"
```

Pass only when L1 balances move.

---

## 19. Restart proposer — VM-A (report steps 15–16)

`docker stop` the proposer only. Not `compose down`. VM-B stays up. `NF4_CONTRACTS__DEPLOY_CONTRACTS` stays `false`.

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
```

VM-B: `curl -sS ${C2_API}/v1/synchronisation`

Expect `Healthy`, prior L2 still in proposer Mongo, both clients on the same L2 block. Record downtime.

---

## 20. Deposit after recovery — VM-A (report step 17)

```bash
curl -sS -H 'Content-Type: application/json' -X POST ${C1_API}/v1/deposit \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
```

Expect a new L2 block and Client 1 commitment `10`.

---

## 21. Soak 48h, then Client 2 deposit (report steps 18–19)

Leave the proposer up. At start, +24h, and end:

```bash
# VM-A
date -u
curl -sS -i ${PROP_API}/v1/health
curl -sS ${C1_API}/v1/synchronisation
docker stats --no-stream "$PROP_CONTAINER" "$C1_CONTAINER"

# VM-B
date -u
curl -sS ${C2_API}/v1/health
curl -sS ${C2_API}/v1/synchronisation
docker stats --no-stream "$C2_CONTAINER"
```

Proposer host resources = VM-A. If you stop early, record the actual duration as skipped/blocked.

Then on **VM-B**, deposit ERC20 `10` and wait for a block while proposer is `Healthy`.

Fill `temp/sepolia_testing_report_template.md`. Hosts: deployer / config / proposer / Client 1 = VM-A; Client 2 = VM-B.

---

## Failures

| Symptom | Action |
|---|---|
| chain-id `31337` / `84532` | Wrong chain. Sepolia is `11155111` |
| RPC `http://` | Use `wss://` for Nightfall; `https://` only for `cast` |
| RPC 429 | `NF4_RPC_RATE_LIMIT=8` in `local.env`, recreate that VM’s containers |
| `--yes is only supported with --network local` | Drop `--yes` |
| Anvil key / 0 balance | Fund real Sepolia accounts |
| `CLIENT2_SIGNING_KEY is missing` | You used `indie-client2`. On VM-B use `wizard client` |
| Wizard failed on `host.docker.internal`, config `:8080` is 200, deployer exit 0 | Do not redeploy. Continue at proposer |
| Connection refused to old `:3001` | On-chain URL is `http://$VM_A_IP:3001`. New containers do not update it |
| Client 1 cannot reach `$VM_A_IP:3001` from Docker | Hairpin failed. Fix routing. Do not use `indie-proposer` |
| Client 2 cannot reach config/proposer | URLs must be `$CONFIG_URL` / `$PROPOSER_URL`, not `127.0.0.1` |
| `docker ps` Client 2 on VM-A | That is Client 1. Check Client 2 on VM-B |
| `nf4_db_proposer is unhealthy` | Need `mongo:8.2`. Recreate DB, rerun `wizard proposer` |
| Deposits sit in mempool | Wait for assembly + real prove + L1 |
| `deriveKey` 500 | 24-word `key_request` / `key_request2`, not the L1 key |
| `Invalid recipient public key` | JSON still has literal `C2_ZKP` / `C1_ZKP`. Paste the hex |
| `Invalid tokenId` | Use `$ID*` even-length hex |
| `/v1/balance` `No such token` | No unspent commitments. C2 ERC20 404s until the transfer block |
| Withdraw L2 done, L1 unchanged | Wait for descrow; `webhook salts` on that client’s VM |
| Need new contracts | `wizard deploy --network testnet` on VM-A again. You pay gas |
