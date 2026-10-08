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

### Optional: tmux session

For remote VMs, running inside `tmux` protects long runs (like proving key generation and the 48h soak) from SSH disconnects:

```bash
# Start a named tmux session on VM-A or VM-B
tmux new -s nf4

# Useful shortcuts:
#   Detach:        Ctrl-b then d
#   Reattach:      tmux attach -t nf4
#   Split window:  Ctrl-b % (vertical) or Ctrl-b " (horizontal)
#   Switch pane:   Ctrl-b <arrow key>
```

A new pane does not inherit `export`s. After section 3, `source temp/sepolia-vm-a.sh` or `source temp/sepolia-vm-b.sh` in each pane before any `curl` or `docker exec`.

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

A new terminal, SSH session, or tmux pane does not inherit exports. `RPC` can still be set, so `cast` works, while `PROP_API`, `C1_API`, `TOKEN20`, `ID20`, `C1_DB`, and `PROP_DB` are empty. `curl` then reports `URL using bad/illegal format`. `docker exec` reports `invalid container name or ID: value is empty`.

Save the exports and `source` that file at the start of every later step, in every pane.

- VM-A: `temp/sepolia-vm-a.sh`
- VM-B: `temp/sepolia-vm-b.sh`

`temp/` is gitignored. From step 4 the file also holds private keys and the RPC URL. Do not commit it. Do not paste it into the report.

The `cat >` below creates the file. Do not run it again after step 4. That wipes keys and token lines. Later steps append. The last `export` of a name wins. Every later command block starts with `source`. If a required name is empty, that block stops before `curl` or `docker exec`.

**Both VMs.** `./scripts/nf4-pick-rpc` reads chainlist.org, probes `eth_chainId` with `cast`, and prints the fastest working `https://` and `wss://` pair. Default `--network sepolia`. A passing chain-id does not prove `eth_subscribe` will hold; if the event listener drops, use a keyed provider.

```bash
eval "$(./scripts/nf4-pick-rpc --network sepolia)"
export RPC="$RPC_HTTP"
export EXPLORER=https://sepolia.etherscan.io
export CONFIG_URL="http://${VM_A_IP}:8080"
export PROPOSER_URL="http://${VM_A_IP}:3001"
printf '%s\n' "$RPC_HTTP" "$RPC_WSS"
: "${VM_A_IP:?}" "${RPC_WSS:?}"
```

**VM-A**

```bash
export PROP_API=http://127.0.0.1:3001
export C1_API=http://127.0.0.1:3000
export C1_CONTAINER=nf4_indie_client
export C1_DB=nf4_db_client
export PROP_CONTAINER=nf4_indie_proposer
export PROP_DB=nf4_db_proposer
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
mkdir -p temp
cat > temp/sepolia-vm-a.sh <<EOF
export VM_A_IP='$VM_A_IP'
export RPC_HTTP='$RPC_HTTP'
export RPC_WSS='$RPC_WSS'
export RPC='$RPC'
export EXPLORER='$EXPLORER'
export CONFIG_URL='$CONFIG_URL'
export PROPOSER_URL='$PROPOSER_URL'
export PROP_API='$PROP_API'
export C1_API='$C1_API'
export C1_CONTAINER='$C1_CONTAINER'
export C1_DB='$C1_DB'
export PROP_CONTAINER='$PROP_CONTAINER'
export PROP_DB='$PROP_DB'
export ID20='$ID20'
export ID721_C1='$ID721_C1'
export ID721_C2='$ID721_C2'
export ID1155='$ID1155'
export ID3525_C1='$ID3525_C1'
export ID3525_C2='$ID3525_C2'
EOF
: "${PROP_API:?}" "${C1_API:?}" "${C1_DB:?}" "${PROP_DB:?}" "${RPC:?}"
```

**VM-B** — `nf4_indie_client` here is Client 2.

```bash
export C2_API=http://127.0.0.1:3000
export C2_CONTAINER=nf4_indie_client
export C2_DB=nf4_db_client
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
mkdir -p temp
cat > temp/sepolia-vm-b.sh <<EOF
export VM_A_IP='$VM_A_IP'
export RPC_HTTP='$RPC_HTTP'
export RPC_WSS='$RPC_WSS'
export RPC='$RPC'
export EXPLORER='$EXPLORER'
export CONFIG_URL='$CONFIG_URL'
export PROPOSER_URL='$PROPOSER_URL'
export C2_API='$C2_API'
export C2_CONTAINER='$C2_CONTAINER'
export C2_DB='$C2_DB'
export ID20='$ID20'
export ID721_C1='$ID721_C1'
export ID721_C2='$ID721_C2'
export ID1155='$ID1155'
export ID3525_C1='$ID3525_C1'
export ID3525_C2='$ID3525_C2'
EOF
: "${C2_API:?}" "${C2_DB:?}" "${CONFIG_URL:?}" "${PROPOSER_URL:?}" "${RPC:?}"
```

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
cast chain-id --rpc-url "$RPC_WSS"
cast block-number --rpc-url "$RPC_WSS"
}

# VM-B
source temp/sepolia-vm-b.sh && {
cast chain-id --rpc-url "$RPC_WSS"
cast block-number --rpc-url "$RPC_WSS"
}
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
cat >> temp/sepolia-vm-b.sh <<EOF
export C2='$C2'
export KEY2='$KEY2'
EOF
: "${C2:?}" "${KEY2:?}"
```

Send `$C2` (address only) to VM-A. Do not send `KEY2`.

**VM-A** (deployer = proposer = Client 1):

```bash
cast wallet new
export C1=0x<address from cast>
export KEY1=0x<private key from cast>
export C2=0x<address received from VM-B>
cat >> temp/sepolia-vm-a.sh <<EOF
export C1='$C1'
export KEY1='$KEY1'
export C2='$C2'
EOF
: "${C1:?}" "${KEY1:?}" "${C2:?}"
```

Fund each address with about 0.5 Sepolia ETH, then:

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
: "${C1:?}" "${C2:?}" "${RPC_HTTP:?}"
cast balance "$C1" --rpc-url "$RPC_HTTP"
cast balance "$C2" --rpc-url "$RPC_HTTP"
}

# VM-B
source temp/sepolia-vm-b.sh && {
: "${C2:?}" "${RPC_HTTP:?}"
cast balance "$C2" --rpc-url "$RPC_HTTP"
}
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
source temp/sepolia-vm-b.sh && {
: "${CONFIG_URL:?}"
curl -sS -o /dev/null -w '%{http_code}\n' "$CONFIG_URL/configuration/toml/addresses.toml"
}
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
source temp/sepolia-vm-a.sh && {
: "${PROP_API:?}"
curl -sS -i "$PROP_API/v1/health"
curl -i --request POST "$PROP_API/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
curl -sS "$PROP_API/v1/proposers"
./scripts/nf4 logs proposer
}
```

On VM-B:

```bash
source temp/sepolia-vm-b.sh && {
: "${PROPOSER_URL:?}"
curl -sS -o /dev/null -w '%{http_code}\n' "$PROPOSER_URL/v1/health"
}
```

Expect HTTP 200 `Healthy`, exactly one proposer, VM-B `200`. Record cert L1 tx.

---

## 7. Client 1 — VM-A (report step 1: Client 2 down)

Do not run `wizard client` on VM-B yet. Write Client 2’s L1 address into VM-A `local.env` from the sourced file. Do not put `KEY2` there.

```bash
source temp/sepolia-vm-a.sh && {
: "${C2:?}"
grep -v '^CLIENT2_ADDRESS=' local.env > local.env.tmp || true
printf 'CLIENT2_ADDRESS="%s"\n' "$C2" >> local.env.tmp
mv local.env.tmp local.env
./scripts/nf4 wizard client
}
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
source temp/sepolia-vm-a.sh && {
: "${C1_CONTAINER:?}" "${CONFIG_URL:?}" "${PROPOSER_URL:?}" "${C1_API:?}"
docker exec "$C1_CONTAINER" wget -q -S -O /dev/null "$CONFIG_URL/configuration/toml/addresses.toml"
docker exec "$C1_CONTAINER" wget -q -S -O /dev/null "$PROPOSER_URL/v1/health"
curl -sS -i "$C1_API/v1/health"
curl -i --request POST "$C1_API/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
curl -sS -X POST "$C1_API/v1/deriveKey" \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
export C1_ZKP=$(curl -sS -X POST "$C1_API/v1/deriveKey" \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C1_ZKP"
: "${C1_ZKP:?}"
cat >> temp/sepolia-vm-a.sh <<EOF
export C1_ZKP='$C1_ZKP'
EOF
}
```

Expect wget/health 200, 64 hex chars in `C1_ZKP` (no `0x`). Copy `C1_ZKP` to VM-B before step 17.

On **VM-B**: `docker ps --filter name=nf4_indie_client` must be empty. Do not run that check on VM-A.

---

## 8. Mock tokens — VM-A

```bash
./scripts/nf4 client deploy-mock-tokens
```

```bash
source temp/sepolia-vm-a.sh
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
ok=1
for name in TOKEN20 TOKEN721 TOKEN1155 TOKEN3525 NF; do
  eval "val=\${$name-}"
  case "$val" in
    0x[0-9a-fA-F]*) ;;
    *) printf 'bad %s=%s\n' "$name" "$val" >&2; ok=0 ;;
  esac
done
[ "$ok" = 1 ] && cat >> temp/sepolia-vm-a.sh <<EOF
export TOKEN20='$TOKEN20'
export TOKEN721='$TOKEN721'
export TOKEN1155='$TOKEN1155'
export TOKEN3525='$TOKEN3525'
export NF='$NF'
export ID20='$ID20'
export ID721_C1='$ID721_C1'
export ID721_C2='$ID721_C2'
export ID1155='$ID1155'
export ID3525_C1='$ID3525_C1'
export ID3525_C2='$ID3525_C2'
EOF
```

Run that `awk` on VM-A only, from the repo root. The deployer writes `configuration/toml/addresses.toml` there. It is not in git, so VM-B does not have it. Do not run the `awk` on VM-B. Replace `<ERC20Mock>` and the other placeholders with the addresses just printed. If any name prints `bad`, the file is not updated. Do not mint until that check is quiet.

Copy `TOKEN20`, `TOKEN721`, `TOKEN1155`, `TOKEN3525`, and `NF` to VM-B. Do not copy `KEY1`. Do not deploy mocks on VM-B, and do not run the `awk` there.

**VM-B** — paste the five addresses from VM-A. No `awk`.

```bash
source temp/sepolia-vm-b.sh
export TOKEN20=<ERC20Mock from VM-A>
export TOKEN721=<ERC721Mock from VM-A>
export TOKEN1155=<ERC1155Mock from VM-A>
export TOKEN3525=<ERC3525Mock from VM-A>
export NF=<nightfall address from VM-A>
export ID20=0x00
export ID721_C1=0x03e9
export ID721_C2=0x03ea
export ID1155=0x07d1
export ID3525_C1=0x0bb9
export ID3525_C2=0x0bba
ok=1
for name in TOKEN20 TOKEN721 TOKEN1155 TOKEN3525 NF; do
  eval "val=\${$name-}"
  case "$val" in
    0x[0-9a-fA-F][0-9a-fA-F]*) ;;
    *) printf 'bad %s=%s\n' "$name" "$val" >&2; ok=0 ;;
  esac
done
[ "$ok" = 1 ] && cat >> temp/sepolia-vm-b.sh <<EOF
export TOKEN20='$TOKEN20'
export TOKEN721='$TOKEN721'
export TOKEN1155='$TOKEN1155'
export TOKEN3525='$TOKEN3525'
export NF='$NF'
export ID20='$ID20'
export ID721_C1='$ID721_C1'
export ID721_C2='$ID721_C2'
export ID1155='$ID1155'
export ID3525_C1='$ID3525_C1'
export ID3525_C2='$ID3525_C2'
EOF
```

If any name prints `bad`, the file is not updated. Do not mint on VM-B. The `cast send` lines stay on VM-A.

```bash
source temp/sepolia-vm-a.sh && {
: "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}" "${C1:?}" "${C2:?}" "${NF:?}" "${KEY1:?}" "${RPC:?}"
cast send "$TOKEN721" "mint(address,address,uint256)" "$C1" "$NF" 1001 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN721" "mint(address,address,uint256)" "$C2" "$NF" 1002 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C1" "$NF" 2001 20 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$C2" "$NF" 2001 10 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C1" 3001 1 100 --private-key "$KEY1" --rpc-url "$RPC"
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$C2" 3002 1 100 --private-key "$KEY1" --rpc-url "$RPC"
}
```

Wait for receipts. Record explorer txs.

---

## 9. Baseline — VM-A (report step 1)

Client 2 still down on VM-B. Source the VM-A file first. If `TOKEN20` or a container name is empty, this block stops. Do not rerun the curls until `source` prints nothing and the `:?` line is silent.

```bash
source temp/sepolia-vm-a.sh && {
: "${PROP_API:?}" "${C1_API:?}" "${TOKEN20:?}" "${ID20:?}" "${C1_DB:?}" "${PROP_DB:?}" "${RPC:?}"
curl -sS "$PROP_API/v1/health"
curl -sS "$C1_API/v1/health"
curl -sS "$C1_API/v1/synchronisation"
curl -sS "$C1_API/v1/commitments"
curl -sS "$C1_API/v1/balance/${TOKEN20}/${ID20}"
cast block-number --rpc-url "$RPC"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
}
```

Record Sepolia block and L2 sync payload.

---

## 10. Client 1 deposits — VM-A (report steps 2–4)

Use **Bash**, from the repository root on VM-A. Keep Client 2 stopped and do not submit transfers yet. This section uses the fixed test data and zero `fee`/`deposit_fee` from the table above. It expects no earlier unspent holdings of these assets; if the baseline is not empty, reconcile it before using the balance assertions below.

There are two different L1 transactions to record: **deposit escrow** (report step 2), then **block submission** (report step 3). A commitment's `layer_1_transaction_hash` is the latter. `PendingCreation` with `layer_2_block_number: null` is not an L2 confirmation. Assembly is configuration-dependent (the template uses a 120s maximum wait); real proving can take tens of minutes. Neither timing is a completion guarantee.

### 10a. Submit once — skip this if you already submitted deposits

If the old commands returned HTTP `202`, **do not run this block**. Go to 10b to recover their request IDs. `202` means queued, not escrowed or confirmed.

For a new run, this block saves each request, response body, headers, and `x-request-id` under `temp/sepolia-step10/`. It refuses to run if that directory already exists. A failed or interrupted POST can still have reached the client: inspect the saved evidence and client logs before any retry; do not delete the directory just to resubmit.

```bash
(
set -euo pipefail
source temp/sepolia-vm-a.sh
: "${C1_API:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}"
: "${ID20:?}" "${ID721_C1:?}" "${ID1155:?}" "${ID3525_C1:?}"
for token in "$TOKEN20" "$TOKEN721" "$TOKEN1155" "$TOKEN3525"; do
  [[ "$token" =~ ^0x[0-9a-fA-F]{40}$ ]] || { echo 'Invalid token address' >&2; exit 1; }
done
[[ "$ID20" == 0x00 && "$ID721_C1" == 0x03e9 && "$ID1155" == 0x07d1 && "$ID3525_C1" == 0x0bb9 ]] || {
  echo 'Token IDs do not match the fixed test data in step 3.' >&2; exit 1;
}
d=temp/sepolia-step10
mkdir "$d" || { echo 'STOP: evidence directory exists. Do not redeposit; use 10b/10c.' >&2; exit 1; }
date -u +%Y-%m-%dT%H:%M:%SZ > "$d/started-at.txt"
for spec in "ERC20:$TOKEN20:$ID20:0:0x64" "ERC721:$TOKEN721:$ID721_C1:2:0x00" "ERC1155:$TOKEN1155:$ID1155:1:0x14" "ERC3525:$TOKEN3525:$ID3525_C1:3:0x64"; do
  IFS=: read -r name token id type value <<< "$spec"
  printf '{"ercAddress":"%s","tokenId":"%s","tokenType":"%s","value":"%s","fee":"0x00","deposit_fee":"0x00"}\n' \
    "$token" "$id" "$type" "$value" > "$d/$name.request.json"
  if ! http=$(curl -sS --connect-timeout 10 --max-time 60 \
    -D "$d/$name.headers" -o "$d/$name.body" -w '%{http_code}' \
    -H 'Content-Type: application/json' -X POST "$C1_API/v1/deposit" \
    --data-binary "@$d/$name.request.json"); then
    echo "$name POST outcome unknown. Check saved headers/logs; do not resubmit blindly." >&2
    exit 1
  fi
  printf '%s HTTP %s\n' "$name" "$http"
  cat "$d/$name.body"; printf '\n'
  [[ "$http" == 202 ]] || { echo 'STOP: no HTTP 202 confirmation. Inspect response/logs before any retry.' >&2; exit 1; }
  request=$(awk 'tolower($1) == "x-request-id:" {gsub(/\r/, "", $2); print $2}' "$d/$name.headers")
  [[ "$request" =~ ^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$ ]] || {
    echo 'STOP: accepted response has no unambiguous UUID header. Recover it from client logs.' >&2; exit 1;
  }
  printf '%s\n' "$request" > "$d/$name.request-id"
  printf '%s request_id=%s\n' "$name" "$request"
done
)
```

### 10b. Already submitted? Save the existing request IDs, without another POST

Skip this block if 10a saved all four `.request-id` files. Otherwise, recover each **original** `x-request-id` from its response headers or the client logs. If some POSTs were never sent, resolve that partial submission first; do not substitute unrelated request IDs. The request body `"Request queued"` is not the ID.

This block prompts only for missing IDs and does not overwrite saved ones. If an ID cannot be recovered, stop and inspect the client logs/database; do not create another deposit to obtain one.

```bash
(
set -euo pipefail
mkdir -p temp/sepolia-step10
for name in ERC20 ERC721 ERC1155 ERC3525; do
  file="temp/sepolia-step10/$name.request-id"
  [ ! -e "$file" ] || { printf '%s already saved\n' "$name"; continue; }
  read -r -p "Original $name x-request-id: " request
  [[ "$request" =~ ^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$ ]] || {
    echo 'Invalid UUID; nothing saved for this token.' >&2; exit 1;
  }
  (set -o noclobber; printf '%s\n' "$request" > "$file")
done
)
```

### 10c. Poll the four requests and verify Client 1's balances

This is read-only with respect to Nightfall. It matches commitments through `request_commitment_mappings`, then verifies the contract/native ID via `nf_token_id`, token type, deposit salt type, and value. Unrelated holdings or another contract with the same native ID cannot satisfy it. Optional `native_token_id` metadata is not required. Client startup clears `requests` but retains commitments/mappings, so an absent request row is reported and the retained mapping is still checked.

The JavaScript runs in **file mode**, not the interactive mongosh prompt. Exit `0` means confirmed rows, `2` means pending, and other exits stop the poll as an error. Each poll prints the current state, including missing mappings. There are at most 360 polls, 30s apart: about three hours **plus query time**, not a hard deadline. Ctrl-C stops only the monitor. Rerunning 10c does not deposit again.

```bash
(
set -euo pipefail
source temp/sepolia-vm-a.sh
: "${C1_API:?}" "${C1_DB:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}"
: "${ID20:?}" "${ID721_C1:?}" "${ID1155:?}" "${ID3525_C1:?}"
d=temp/sepolia-step10
REQ20=$(< "$d/ERC20.request-id")
REQ721=$(< "$d/ERC721.request-id")
REQ1155=$(< "$d/ERC1155.request-id")
REQ3525=$(< "$d/ERC3525.request-id")
export REQ20 REQ721 REQ1155 REQ3525
export TOKEN20 TOKEN721 TOKEN1155 TOKEN3525 ID20 ID721_C1 ID1155 ID3525_C1
cat > "$d/check.js" <<'JS'
(async () => {
try {
  const e = process.env;
  const want = [
    ["ERC20", e.REQ20, e.TOKEN20, e.ID20, 0n, 100n],
    ["ERC721", e.REQ721, e.TOKEN721, e.ID721_C1, 1001n, 0n],
    ["ERC1155", e.REQ1155, e.TOKEN1155, e.ID1155, 2001n, 20n],
    ["ERC3525", e.REQ3525, e.TOKEN3525, e.ID3525_C1, 3001n, 100n]
  ];
  const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  function hex(v) {
    const s = String(v ?? "").replace(/^0x/i, "");
    if (!/^[0-9a-f]{1,64}$/i.test(s)) throw Error("Invalid stored hex field");
    return s.toLowerCase().padStart(64, "0");
  }
  for (const [name, request, address, id, nativeId] of want) {
    if (!uuid.test(request || "") || !/^0x[0-9a-f]{40}$/i.test(address || "") ||
        !/^0x(?:[0-9a-f]{2}){1,32}$/i.test(id || "") || BigInt(id) !== nativeId) {
      throw Error(name + ": invalid request ID/address or wrong fixed token ID");
    }
  }
  if (new Set(want.map(w => w[1].toLowerCase())).size !== 4) throw Error("Request IDs must be distinct");
  const nf = db.getSiblingDB("nightfall");
  const blocks = new Set();
  let ready = true;
  for (const [name, rawRequest, address, id, nativeId, expected] of want) {
    const request = rawRequest.toLowerCase();
    const req = await nf.requests.findOne({uuid: request});
    print(name + " request_id=" + request + " request_status=" + (req ? req.status : "absent (possibly restarted)"));
    if (req && !["Queued", "Processing", "Submitted", "Confirmed"].includes(req.status)) {
      throw Error(name + ": request " + req.status + "; inspect client logs before any retry");
    }
    const mappings = await nf.request_commitment_mappings.find({request_id: request}).toArray();
    if (!mappings.length) {
      if (!req || !["Queued", "Processing"].includes(req.status)) throw Error(name + ": no commitment mapping; check request ID/client logs");
      print(name + " waiting for commitment mapping");
      ready = false;
      continue;
    }
    const keys = [...new Set(mappings.map(m => m.commitment_hash))];
    const rows = await nf.commitments.find({_id: {$in: keys}}).toArray();
    if (rows.length !== keys.length) throw Error(name + ": mapped commitment missing from database");
    // Same SHA-256 and right shift as lib/src/nf_token_id.rs; includes the contract address.
    const digest = require("crypto").createHash("sha256")
      .update(Buffer.from(hex(address) + hex(id), "hex")).digest("hex");
    const nfTokenId = hex((BigInt("0x" + digest) >> 4n).toString(16));
    if (rows.length !== 1) throw Error(name + ": expected one mapped commitment with deposit_fee=0");
    const r = rows[0];
    if (r.token_type !== name || hex(r.preimage.nf_token_id) !== nfTokenId ||
        r.preimage.salt.type !== "Deposit" || BigInt("0x" + hex(r.preimage.value)) !== expected) {
      throw Error(name + ": mapped commitment does not match this deposit's asset/value");
    }
    let l2 = null;
    if (r.layer_2_block_number !== null && r.layer_2_block_number !== undefined) {
      const raw = String(r.layer_2_block_number);
      l2 = Number(raw);
      if (!/^(0|[1-9][0-9]*)$/.test(raw) || !Number.isSafeInteger(l2)) throw Error(name + ": invalid L2 block number");
    }
    let blockTx = null;
    if (r.layer_1_transaction_hash !== null && r.layer_1_transaction_hash !== undefined) {
      const h = r.layer_1_transaction_hash;
      const s = typeof h === "string" ? h.replace(/^0x/i, "") : h.toString("hex");
      if (!/^[0-9a-f]{64}$/i.test(s)) throw Error(name + ": invalid block transaction hash");
      blockTx = "0x" + s.toLowerCase();
    }
    print(name + " token_id=" + nativeId + " commitment=" + r._id + " status=" + r.status +
      " value=" + expected + " l2=" + l2 + " block_tx=" + blockTx);
    if (r.status === "PendingCreation") ready = false;
    else if (r.status !== "Unspent" || l2 === null || blockTx === null) throw Error(name + ": inconsistent or already spent commitment; do not advance");
    if (r.status === "Unspent") blocks.add(l2);
  }
  print("L2 blocks for these deposits: " + ([...blocks].join(" ") || "none"));
  print(ready ? "READY" : "WAITING");
  quit(ready ? 0 : 2);
} catch (err) {
  console.error("CHECK FAILED: " + err.message);
  quit(1);
}
})();
JS
confirmed=0
for ((i=1; i<=360; i++)); do
  printf '\n%s poll %s/360\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$i"
  rc=0
  out=$(docker exec -i \
    -e REQ20 -e REQ721 -e REQ1155 -e REQ3525 \
    -e TOKEN20 -e TOKEN721 -e TOKEN1155 -e TOKEN3525 \
    -e ID20 -e ID721_C1 -e ID1155 -e ID3525_C1 \
    "$C1_DB" mongosh --quiet --norc --file /dev/stdin < "$d/check.js" 2>&1) || rc=$?
  printf '%s\n' "$out"
  if [ "$rc" -eq 0 ] && grep -qx READY <<< "$out"; then
    confirmed=1; break
  fi
  if [ "$rc" -ne 2 ] || ! grep -qx WAITING <<< "$out"; then
    echo "Poll/query failed (exit $rc). Check Docker/Mongo and the message above; do not redeposit." >&2
    exit 1
  fi
  [ "$i" -eq 360 ] || sleep 30
done
[ "$confirmed" -eq 1 ] || { echo 'Timed out with pending deposits. Inspect 10d; do not redeposit.' >&2; exit 1; }
record="$out"
sync=$(curl -fsS --connect-timeout 10 --max-time 30 "$C1_API/v1/synchronisation")
record+=$'\n'"sync=$sync"
for spec in "ERC20:$TOKEN20:$ID20:100" "ERC721:$TOKEN721:$ID721_C1:0" "ERC1155:$TOKEN1155:$ID1155:20" "ERC3525:$TOKEN3525:$ID3525_C1:100"; do
  IFS=: read -r name token id expected <<< "$spec"
  body=$(curl -fsS --connect-timeout 10 --max-time 30 "$C1_API/v1/balance/$token/$id")
  [[ "$body" =~ ^[0-9a-fA-F]{1,64}$ ]] || { echo "$name: invalid balance response: $body" >&2; exit 1; }
  balance=$(cast to-dec "0x$body")
  printf '%s balance=%s expected=%s\n' "$name" "$balance" "$expected"
  [ "$balance" = "$expected" ] || { echo 'Balance mismatch: reconcile baseline/requests; do not redeposit.' >&2; exit 1; }
  record+=$'\n'"$name contract=$token token_id=$id balance_hex=$body balance=$balance"
done
printf '\n--- record ---\n%s\n%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$record" | tee "$d/record.txt"
)
```

`--- record ---` means the four mapped deposit commitments and API balances passed these checks. It does **not** replace the logs and explorer evidence in 10d. ERC721 ownership is established by the matched `Unspent` commitment for token `1001`; its deposit value and `/v1/balance` are zero. More than one L2 block is possible: record every listed block and its `block_tx`, not an unrelated database-wide block number.

### 10d. Collect escrow, proving, and L1 inclusion evidence

Run this in another Bash terminal on VM-A while waiting, and again after confirmation. It takes log snapshots, not a follow stream. Docker logs are local evidence: redact secrets/RPC credentials before attaching excerpts to the report.

```bash
(
set -euo pipefail
source temp/sepolia-vm-a.sh
: "${C1_CONTAINER:?}" "${PROP_CONTAINER:?}"
mkdir -p temp/sepolia-step10
docker logs --timestamps "$C1_CONTAINER" > temp/sepolia-step10/client.log 2>&1
docker logs --timestamps "$PROP_CONTAINER" > temp/sepolia-step10/proposer.log 2>&1
grep -E 'Escrowing funds|DepositEscrowed|Proposed block call|ERROR|WARN|panic' temp/sepolia-step10/client.log || true
grep -E 'DepositEscrowed|Deposit Transaction is valid|This block has|Computing block|Block computation took|The L2 block was sent to L1|ERROR|WARN|panic' temp/sepolia-step10/proposer.log || true
)
```

For **each deposit**, use the client's `Decoded DepositEscrowed event from transaction ...` log to locate its escrow transaction. Log hashes may be abbreviated: copy the **full** hash from the Sepolia explorer, not an ellipsis-containing log string. Use `$NF`'s transaction history and the run's time window if necessary. Cross-check the transaction: it must target `$NF`, come from `$C1`, and its decoded `escrow_funds` input must match the token contract, native token ID, token type, value, and fee. Record that full escrow hash alongside the request ID. If multiple deposits match and cannot be distinguished, mark the evidence blocked rather than guessing. An approval transaction is not the escrow transaction. Capture the corresponding proposer `DepositEscrowed`/mempool evidence too. Do not infer success merely from a function selector appearing in a log.

For **each L2 block**, capture assembly, proving completion, and submission logs; record its `block_tx` from 10c. A `Block computation took` duration alone does not prove success; check the subsequent result/submission and receipt. Verify successful L1 inclusion for each escrow and block-submission hash. Run this block once per hash (paste the hash at the prompt):

```bash
(
set -euo pipefail
source temp/sepolia-vm-a.sh
: "${RPC:?}" "${NF:?}"
[ "$(cast chain-id --rpc-url "$RPC")" = 11155111 ] || { echo 'Not Ethereum Sepolia' >&2; exit 1; }
read -r -p 'Escrow or block-submission transaction hash: ' TX
[[ "$TX" =~ ^0x[0-9a-fA-F]{64}$ ]] || { echo 'Invalid transaction hash' >&2; exit 1; }
status=$(cast receipt "$TX" status --async --rpc-url "$RPC")
[[ "$status" == 1 || "$status" == 0x1 || "$status" == '1 (success)' ]] || { echo 'Transaction not successfully included' >&2; exit 1; }
cast receipt "$TX" --json --async --rpc-url "$RPC" | tee "temp/sepolia-step10/$TX.receipt.json"
printf 'Check destination %s and decoded input/events at https://sepolia.etherscan.io/tx/%s\n' "$NF" "$TX"
)
```

A successful receipt proves inclusion, not Ethereum finality. Record the explorer's confirmation/finality status at capture time. Fill report steps 2–4 with the request IDs, escrow hashes, proposer lifecycle evidence, L2 numbers/block-submission transaction hashes, `record.txt`, and screenshots of the safe commitment summaries/balances. Do not attach full commitment preimages (they contain deposit secrets). Advance to step 11 only when those checks pass. If waiting persists, inspect the saved request state and client/proposer errors; `Failed`, a query error, or a missing mapping is not a reason to redeposit blindly.

---

## 11. Client 2 late join — VM-B (report steps 5–7)

Only after step 10’s deposit block is confirmed. `$RPC`, `$CONFIG_URL`, and `$PROPOSER_URL` are already in `temp/sepolia-vm-b.sh` from section 3. Copy `C1`, `TOKEN20`, `TOKEN721`, `TOKEN1155`, `TOKEN3525`, and `NF` from VM-A (addresses only). Do not copy `KEY1`. If a name is still a placeholder, this block stops and does not write the file.

```bash
source temp/sepolia-vm-b.sh
export C1=<Client 1 address from VM-A>
export TOKEN20=<ERC20Mock from VM-A>
export TOKEN721=<ERC721Mock from VM-A>
export TOKEN1155=<ERC1155Mock from VM-A>
export TOKEN3525=<ERC3525Mock from VM-A>
export NF=<nightfall address from VM-A>
ok=1
for name in C1 TOKEN20 TOKEN721 TOKEN1155 TOKEN3525 NF; do
  eval "val=\${$name-}"
  case "$val" in
    0x[0-9a-fA-F][0-9a-fA-F]*) ;;
    *) printf 'bad %s=%s\n' "$name" "$val" >&2; ok=0 ;;
  esac
done
[ "$ok" = 1 ] && cat >> temp/sepolia-vm-b.sh <<EOF
export C1='$C1'
export TOKEN20='$TOKEN20'
export TOKEN721='$TOKEN721'
export TOKEN1155='$TOKEN1155'
export TOKEN3525='$TOKEN3525'
export NF='$NF'
EOF
[ "$ok" = 1 ] && source temp/sepolia-vm-b.sh && {
: "${CONFIG_URL:?}" "${PROPOSER_URL:?}" "${RPC_WSS:?}" "${TOKEN20:?}" "${NF:?}" "${C1:?}"
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
}
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
source temp/sepolia-vm-b.sh && {
: "${C2_CONTAINER:?}" "${C2_API:?}" "${CONFIG_URL:?}" "${PROPOSER_URL:?}"
docker exec "$C2_CONTAINER" wget -q -S -O /dev/null "$CONFIG_URL/configuration/toml/addresses.toml"
docker exec "$C2_CONTAINER" wget -q -S -O /dev/null "$PROPOSER_URL/v1/health"
curl -sS -i "$C2_API/v1/health"
curl -i --request POST "$C2_API/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-4.priv_key;type=application/octet-stream'
curl -sS -X POST "$C2_API/v1/deriveKey" \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
export C2_ZKP=$(curl -sS -X POST "$C2_API/v1/deriveKey" \
  -H 'Content-Type: application/json' -d '{}' \
  | sed -n 's/.*"zkp_public_key":"\([^"]*\)".*/\1/p')
printf '%s\n' "$C2_ZKP"
: "${C2_ZKP:?}"
cat >> temp/sepolia-vm-b.sh <<EOF
export C2_ZKP='$C2_ZKP'
EOF
curl -sS "$C2_API/v1/synchronisation"
}
```

Paste `C2_ZKP` onto VM-A and append it before step 12. Client 2 L2 block must match Client 1.

```bash
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}" "${ID721_C2:?}" "${ID1155:?}" "${ID3525_C2:?}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C2_API/v1/deposit" \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C2_API/v1/deposit" \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C2_API/v1/deposit" \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
}
```

After the next L2 block: Client 2 has 1002, `2001:10`, `3002:100`.

---

## 12. Transfers — VM-A (report step 8)

`C2_ZKP` must be the 64 hex chars from VM-B, not the characters `C2_ZKP`. A placeholder does not get written, and the transfers do not run.

```bash
export C2_ZKP=<64 hex chars from VM-B>
case "$C2_ZKP" in
  *[!0-9a-fA-F]*|'') printf 'C2_ZKP must be 64 hex chars\n' >&2; false ;;
  *) [ "${#C2_ZKP}" = 64 ] ;;
esac && {
cat >> temp/sepolia-vm-a.sh <<EOF
export C2_ZKP='$C2_ZKP'
EOF
source temp/sepolia-vm-a.sh
: "${C1_API:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}" "${ID20:?}" "${ID721_C1:?}" "${ID1155:?}" "${ID3525_C1:?}" "${C2_ZKP:?}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/transfer" \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/transfer" \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/transfer" \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"recipientData\":{\"values\":[\"0x08\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/transfer" \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
}
```

Poll `/v1/request/<id>` until all four are accepted. Do not wait for L2 yet. Proposer log `Found N client transactions` with N > 0. Do not stop Client 1 if a transfer failed.

---

## 13. Stop Client 1 — VM-A (report step 9)

Do not stop VM-B.

```bash
source temp/sepolia-vm-a.sh && {
: "${C1_API:?}" "${C1_DB:?}" "${C1_CONTAINER:?}"
curl -sS "$C1_API/v1/synchronisation"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
docker stop "$C1_CONTAINER"
date -u
}
```

Record timestamp, last L2 block, DB dump.

---

## 14. Confirm transfers — VM-A logs, VM-B balances (report step 10)

Client 2 ERC20 `/v1/balance` is 404 until this block (`No such token`). That is expected.

If proposer says `Found 0 client transactions`, start Client 1, `deriveKey`, resubmit, then `docker stop "$C1_CONTAINER"` again.

```bash
# VM-A
source temp/sepolia-vm-a.sh && ./scripts/nf4 logs proposer

# VM-B
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}" "${C2_DB:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${ID20:?}" "${ID721_C1:?}" "${ID1155:?}"
curl -sS "$C2_API/v1/synchronisation"
curl -sS "$C2_API/v1/commitments"
curl -sS "$C2_API/v1/balance/${TOKEN20}/${ID20}"
curl -sS "$C2_API/v1/balance/${TOKEN721}/${ID721_C1}"
curl -sS "$C2_API/v1/balance/${TOKEN1155}/${ID1155}"
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
}
```

Expect Client 2: ERC20 40, ERC721 1001, ERC1155 18, ERC3525 received 40.

---

## 15. Restart Client 1 — VM-A (report step 11)

Keys are in-memory. `deriveKey` again before reading commitments.

```bash
source temp/sepolia-vm-a.sh && {
: "${C1_CONTAINER:?}" "${C1_API:?}" "${C1_DB:?}"
docker start "$C1_CONTAINER"
curl -sS -i "$C1_API/v1/health"
curl -sS -X POST "$C1_API/v1/deriveKey" \
  -H 'Content-Type: application/json' \
  -d '{"mnemonic":"spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work","child_path":"m/44'\''/60'\''/0'\''/0/0"}'
curl -sS "$C1_API/v1/synchronisation"
curl -sS "$C1_API/v1/commitments"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").commitments.find().toArray()'
}
```

Expect Client 1 caught up: ERC20 60, ERC1155 12, ERC3525 60, ERC721 1001 spent.

---

## 16. Compare DBs (report step 12)

L2 block numbers must match.

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
: "${C1_API:?}" "${C1_DB:?}" "${PROP_DB:?}"
curl -sS "$C1_API/v1/synchronisation"
docker exec "$C1_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
}

# VM-B
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}" "${C2_DB:?}"
curl -sS "$C2_API/v1/synchronisation"
docker exec "$C2_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
}
```

| | Client 1 | Client 2 |
|---|---|---|
| ERC20 | 60 | 40 |
| ERC721 | 1001 spent | 1001 and 1002 held |
| ERC1155 2001 | 12 | 18 |
| ERC3525 | 60 on 3001 | 40 received + 3002 |

---

## 17. Return ERC721 1001 — VM-B (report step 13)

Needs `C1_ZKP` from VM-A (64 hex chars, not the characters `C1_ZKP`). A placeholder does not get written, and the transfer does not run.

```bash
export C1_ZKP=<64 hex chars from VM-A>
case "$C1_ZKP" in
  *[!0-9a-fA-F]*|'') printf 'C1_ZKP must be 64 hex chars\n' >&2; false ;;
  *) [ "${#C1_ZKP}" = 64 ] ;;
esac && {
cat >> temp/sepolia-vm-b.sh <<EOF
export C1_ZKP='$C1_ZKP'
EOF
source temp/sepolia-vm-b.sh
: "${C2_API:?}" "${TOKEN721:?}" "${ID721_C1:?}" "${C1_ZKP:?}"
curl -sS -H 'Content-Type: application/json' -X POST "$C2_API/v1/transfer" \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C1_ZKP}\"]},\"fee\":\"0x00\"}"
}
```

After the block: Client 1 holds 1001; Client 2 does not.

---

## 18. Withdraw — C1 on VM-A, C2 on VM-B (report step 14)

Webhook commands on the VM that owns that client.

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
: "${C1_API:?}" "${C1:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}" "${ID20:?}" "${ID721_C1:?}" "${ID1155:?}" "${ID3525_C1:?}"
curl -sS -H 'Content-Type: application/json' -X POST "$C1_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C1_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x3c\",\"recipientAddress\":\"${C1}\",\"fee\":\"0x00\"}"
}
```

```bash
# VM-B
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}" "${C2:?}" "${TOKEN20:?}" "${TOKEN721:?}" "${TOKEN1155:?}" "${TOKEN3525:?}" "${ID20:?}" "${ID721_C2:?}" "${ID1155:?}" "${ID3525_C2:?}"
curl -sS -H 'Content-Type: application/json' -X POST "$C2_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C2_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -D - -H 'Content-Type: application/json' -X POST "$C2_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x12\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
curl -sS -H 'Content-Type: application/json' -X POST "$C2_API/v1/withdraw" \
  --data-raw "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x28\",\"recipientAddress\":\"${C2}\",\"fee\":\"0x00\"}"
}
```

Wait for L2 **and** L1 descrow (`0xf3b85fc2`). Then on that client’s VM:

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
: "${TOKEN20:?}" "${C1:?}" "${C2:?}" "${RPC:?}"
./scripts/nf4 webhook salts
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C1" --rpc-url "$RPC"
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C2" --rpc-url "$RPC"
}

# VM-B — salts for Client 2, then the same L1 balances
source temp/sepolia-vm-b.sh && {
: "${TOKEN20:?}" "${C1:?}" "${C2:?}" "${RPC:?}"
./scripts/nf4 webhook salts
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C1" --rpc-url "$RPC"
cast call "$TOKEN20" "balanceOf(address)(uint256)" "$C2" --rpc-url "$RPC"
}
```

`$C1` on VM-B comes from the step 11 append (address only). If the balance block says `C1: parameter null or not set`, append that address and rerun. Do not copy `KEY1`.

Pass only when L1 balances move.

---

## 19. Restart proposer — VM-A (report steps 15–16)

`docker stop` the proposer only. Not `compose down`. VM-B stays up. `NF4_CONTRACTS__DEPLOY_CONTRACTS` stays `false`.

```bash
source temp/sepolia-vm-a.sh && {
: "${C1_API:?}" "${PROP_API:?}" "${PROP_DB:?}" "${PROP_CONTAINER:?}"
curl -sS "$C1_API/v1/synchronisation"
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
date -u
docker stop "$PROP_CONTAINER"
date -u
docker start "$PROP_CONTAINER"
curl -sS -i "$PROP_API/v1/health"
./scripts/nf4 logs proposer
docker exec "$PROP_DB" mongosh --quiet --eval 'db.getSiblingDB("nightfall").ProposedBlocks.find().toArray()'
curl -sS "$C1_API/v1/synchronisation"
}
```

VM-B:

```bash
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}"
curl -sS "$C2_API/v1/synchronisation"
}
```

Expect `Healthy`, prior L2 still in proposer Mongo, both clients on the same L2 block. Record downtime.

---

## 20. Deposit after recovery — VM-A (report step 17)

```bash
source temp/sepolia-vm-a.sh && {
: "${C1_API:?}" "${TOKEN20:?}" "${ID20:?}"
curl -sS -H 'Content-Type: application/json' -X POST "$C1_API/v1/deposit" \
  --data-raw "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
}
```

Expect a new L2 block and Client 1 commitment `10`.

---

## 21. Soak 48h, then Client 2 deposit (report steps 18–19)

Leave the proposer up. If using `tmux`, you can safely detach (`Ctrl-b d`) and leave the session running on the VM:

```bash
# Detach from tmux while the stack and soak run
# Press Ctrl-b, then press d

# When returning to check status:
tmux attach -t nf4
```

At start, +24h, and end:

```bash
# VM-A
source temp/sepolia-vm-a.sh && {
: "${PROP_API:?}" "${C1_API:?}" "${PROP_CONTAINER:?}" "${C1_CONTAINER:?}"
date -u
curl -sS -i "$PROP_API/v1/health"
curl -sS "$C1_API/v1/synchronisation"
docker stats --no-stream "$PROP_CONTAINER" "$C1_CONTAINER"
}

# VM-B
source temp/sepolia-vm-b.sh && {
: "${C2_API:?}" "${C2_CONTAINER:?}"
date -u
curl -sS "$C2_API/v1/health"
curl -sS "$C2_API/v1/synchronisation"
docker stats --no-stream "$C2_CONTAINER"
}
```

Proposer host resources = VM-A. If you stop early, record the actual duration as skipped/blocked.

Then on **VM-B**, deposit ERC20 `10` and wait for a block while proposer is `Healthy`.

Fill `temp/sepolia_testing_report_template.md`. Hosts: deployer / config / proposer / Client 1 = VM-A; Client 2 = VM-B.

---

## Failures

| Symptom | Action |
|---|---|
| `curl: (3) URL using bad/illegal format` or `invalid container name or ID: value is empty` | This shell did not `source temp/sepolia-vm-a.sh` (or `temp/sepolia-vm-b.sh`). `cast` can still work if `RPC` is set. Source the file and rerun the block. If the file is missing, recreate it from section 3 only when it has no keys yet; otherwise append the missing `export` lines. Do not redeploy. Do not rerun section 3 `cat >` after step 4 |
| `TOKEN20: parameter null or not set` (or `C1_DB`, `PROP_API`, `ID20`) | Same shell gap. Step 8 must have appended the mock addresses. A placeholder `<ERC20Mock>` is not an address |
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
| `awk: fatal: cannot open file configuration/toml/addresses.toml` on VM-B | Step 8 `awk` is VM-A only. Paste `$NF` from VM-A. Do not redeploy. Step 11 downloads the file |
| `nf4_db_proposer is unhealthy` | Need `mongo:8.2`. Recreate DB, rerun `wizard proposer` |
| Deposits sit in mempool | Wait for assembly + real prove + L1 |
| Step 10 poll times out or reports an error | Inspect the per-request status and section 10d's client/proposer logs. Fix query/configuration errors before rerunning **10c only**. Recover original request IDs via 10b if needed. Do not redeposit to clear a timeout, failure, or missing mapping |
| `deriveKey` 500 | 24-word `key_request` / `key_request2`, not the L1 key |
| `Invalid recipient public key` | JSON still has literal `C2_ZKP` / `C1_ZKP`. Paste the hex |
| `Invalid tokenId` | Use `$ID*` even-length hex |
| `/v1/balance` `No such token` | No unspent commitments. C2 ERC20 404s until the transfer block |
| Withdraw L2 done, L1 unchanged | Wait for descrow; `webhook salts` on that client’s VM |
| Need new contracts | `wizard deploy --network testnet` on VM-A again. You pay gas |
