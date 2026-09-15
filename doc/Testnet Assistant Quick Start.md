# Testnet Assistant Quick Start

Stand up Nightfall on Ethereum Sepolia (or another wizard testnet) with `nf4`. The same steps work on one machine or on separate VMs: **Client 1 is one client process, Client 2 is another.** They never share a wallet.

For the ordered Sepolia report (deposits, late join, transfers, recover, soak), use `doc/Sepolia Assistant Quick Start.md` after the nodes below are up. That guide uses the API variables defined here.

Work from the repository root on `auto/testnet`. Do not put private keys, mnemonics, or RPC secrets in reports.

## Roles

```text
                    wss:// Sepolia
                           ^
                           |
        +------------------+------------------+
        |                  |                  |
   Operator VM        Client 1 VM        Client 2 VM
   deploy once        wizard client      wizard client
   configuration      API :3000          API :3000
   proposer :3001     own key, Mongo     own key, Mongo
```

| Role | What runs | Typical host |
|---|---|---|
| Operator | `wizard deploy`, configuration `:8080`, `wizard proposer` `:3001` | VM-A (or your laptop) |
| Client 1 | `wizard client` on `:3000` | VM-B |
| Client 2 | `wizard client` on `:3000` | VM-C |

Same machine: collapse all three roles onto one host. Client 2 then uses port **3002** and `./scripts/nf4 up client2` instead of a second `wizard client`. The curls stay the same if you set the API variables below.

Do **not** use `--network local`, Anvil keys, or `./scripts/nf4-local-e2e.sh` here.

## What must be reachable

Clients do not talk to each other. Both must reach:

1. Sepolia `wss://` RPC
2. Configuration service (`http://<operator>:8080`)
3. The **on-chain** proposer URL (`http://<operator>:3001` unless you registered something else)

Pick those URLs **before** `wizard deploy`. They are written on chain and into config. `127.0.0.1` and `indie-proposer` only work on one host. Use the Operator VM’s LAN or public IP.

Open Operator ports `8080` and `3001` to the client VMs. Client API ports stay local unless you want to curl a client from another box.

`nf4` does **not** rewrite LAN URLs when `NF4_NETWORK=testnet`. If the Operator IP changes, transfers break until you deploy again.

## API variables (same curls everywhere)

Set these in the shell where you run curls. On a VM, `127.0.0.1` is **that** VM.

```bash
# Split VMs (run each export on the VM that owns the process)
export PROP_API=http://127.0.0.1:3001    # Operator
export C1_API=http://127.0.0.1:3000      # Client 1 VM
export C2_API=http://127.0.0.1:3000      # Client 2 VM
export C1_CONTAINER=nf4_indie_client
export C2_CONTAINER=nf4_indie_client
export C1_DB=nf4_db_client
export C2_DB=nf4_db_client
export PROP_CONTAINER=nf4_indie_proposer
export PROP_DB=nf4_db_proposer
```

Same machine, all roles:

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

Cross-VM data you **do** copy (not secrets): configuration URL, on-chain proposer URL, Nightfall/token addresses, Client 2 L1 address (for minting), compressed ZKP public keys for transfers. Never copy `local.env` private keys between VMs.

## Every VM

```bash
git switch auto/testnet
git pull
cargo test -p nightfall_ops
./scripts/nf4 check deployer
```

Need Docker, `forge`, `cast`, `cargo`, `curl`, and a Sepolia `wss://` RPC (`http://` will fail).

```bash
export RPC_WSS=wss://YOUR_SEPOLIA_WEBSOCKET_URL
export RPC_HTTP=https://YOUR_SEPOLIA_HTTPS_URL
cast chain-id --rpc-url "$RPC_WSS"
```

Expect `11155111` for Ethereum Sepolia. `84532` is Base Sepolia; start over and choose `sepolia`.

On testnet, key prompts: paste a funded key, or press Enter to generate an account, fund the printed address, then recheck balance. Generated accounts start at 0 ETH.

## Operator VM — deploy and proposer

```bash
./scripts/nf4 wizard deploy --network testnet
```

| Prompt | Enter |
|---|---|
| Testnet chain | `sepolia` |
| Profile | `sepolia` |
| RPC | `$RPC_WSS` |
| Configuration port | `8080` |
| Configuration URL | `http://<operator-ip>:8080` — must be reachable from **both client VMs** |
| Deployer key | Paste, or Enter to generate and fund |
| Default proposer address | Enter (deployer) |
| Default proposer URL | `http://<operator-ip>:3001` — this is written **on chain**. Not `indie-proposer` |
| Real prover? | `no` for a laptop-sized Operator |
| Block size | `64` |
| Apply? | `yes` if review shows `network: testnet`, `chain_id: 11155111` |

```bash
./scripts/nf4 wizard proposer
```

Use the same key as the default proposer (Enter reuses `local.env`). Certify the proposer on the Operator:

```bash
curl -sS -i "$PROP_API/v1/health"
curl -i --request POST "$PROP_API/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der;type=application/pkix-cert' \
  --form 'priv_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key;type=application/octet-stream'
```

Health must be `200` `Healthy`. Publish these three strings to the client VMs:

```text
CONFIG_URL=http://<operator-ip>:8080
PROPOSER_URL=http://<operator-ip>:3001
RPC_WSS=wss://...
```

`--yes` is local-only.

## Client VM — bootstrap, then wizard

Each client VM clones the repo independently. Do **not** run `wizard deploy` again (that would deploy new contracts).

On the client VM, after Operator `Deployment OK`:

```bash
# reachable config (replace host)
export CONFIG_URL=http://<operator-ip>:8080
export PROPOSER_URL=http://<operator-ip>:3001

# profile + metadata (no private keys)
scp user@operator:path/to/nightfall_4_CE/nightfall.toml ./nightfall.toml
mkdir -p configuration/toml configuration/bin/keys
curl -sS "$CONFIG_URL/configuration/toml/addresses.toml" -o configuration/toml/addresses.toml
curl -sS "$CONFIG_URL/configuration/toml/contract_hashes.toml" -o configuration/toml/contract_hashes.toml
curl -sS "$CONFIG_URL/configuration/bin/keys/proving_key" -o configuration/bin/keys/proving_key
```

Create `local.env` on **this** VM only (`NF4_CONTRACTS__DEPLOY_CONTRACTS=false`):

```bash
cat > local.env <<EOF
NF4_RUN_MODE=sepolia
NF4_NETWORK=testnet
NF4_MOCK_PROVER=true
NF4_CONTRACTS__DEPLOY_CONTRACTS=false
NF4_ETHEREUM_CLIENT_URL=${RPC_WSS}
NF4_CONFIGURATION_URL=${CONFIG_URL}
NF4_NIGHTFALL_PROPOSER__URL=${PROPOSER_URL}
EOF
```

```bash
./scripts/nf4 wizard client
```

| Prompt | Enter |
|---|---|
| Client key | This VM’s key (paste or Enter to generate and fund) |
| Client address | Enter |
| Client 2 on this machine? | **No** (Client 2 is the other VM) |
| Proposer URL | `$PROPOSER_URL` |
| Configuration URL | `$CONFIG_URL` |
| Webhook | Accept default (this VM’s LAN IP, port 8081) |
| Client API port | `3000` |

Same-machine Client 2: on the Operator/Client 1 host, answer **Yes** to “Will Client 2 also run on this machine?”, then later `./scripts/nf4 up client2`.

Certify with a **different** test cert per client (`user-3` on Client 1, `user-4` on Client 2):

```bash
curl -sS -i "$C1_API/v1/health"
curl -i --request POST "$C1_API/v1/certification" \
  --form 'certificate=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.der;type=application/pkix-cert' \
  --form 'certificate_private_key=@blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key;type=application/octet-stream'
```

On Client 2 VM use `$C2_API` and `user-4`.

Late join: keep Client 2’s **process** stopped until Client 1’s deposit block is confirmed. You can still create and fund Client 2’s L1 account earlier (`cast wallet new`) and send that **address** (not the key) to Client 1 for mock-token minting.

## Mock tokens

Run once, from Client 1 or Operator, **before** Client 2 starts. Put Client 2’s L1 address in Client 1’s `local.env`:

```bash
# CLIENT2_ADDRESS=0x...   # from Client 2 VM; no Client 2 private key on this VM
./scripts/nf4 client deploy-mock-tokens
```

Share the four token addresses with Client 2. Mint report IDs with Client 1’s key to both L1 addresses (see the Sepolia guide).

## Next

Run `doc/Sepolia Assistant Quick Start.md` from **Report step 1** using `$C1_API`, `$C2_API`, and `$PROP_API`. Start Client 2 only after Client 1’s deposit block (on a Client 2 VM: `wizard client` or `docker start $C2_CONTAINER`; same machine: `./scripts/nf4 up client2`).

## Useful commands

```bash
./scripts/nf4 status
./scripts/nf4 logs configuration    # Operator
./scripts/nf4 logs proposer         # Operator
./scripts/nf4 logs client           # that VM’s client
./scripts/nf4 webhook events
./scripts/nf4 webhook salts
```

## Expected result

- Operator: deployer exited 0, configuration up, proposer `Healthy`
- Each client VM: `GET $C*_API/v1/health` is `200` `Healthy`
- Both clients reach `$CONFIG_URL/configuration/toml/addresses.toml`
- Transfers use the on-chain proposer URL, not `localhost` from inside Docker
