# Deploy Nightfall on Sepolia

Two machines. The operator VM owns configuration (`:8080`), the proposer (`:3001`), and Client 1 (`:3000`). The client machine owns Client 2 (`:3000`), and only after the operator deposit poll prints `READY`.

Work from the repository root. Pass `--file deploy.env` every time. Chain ID is `11155111`. Do not use Anvil, `--yes`, `--network local`, or chain `84532`.

`scripts/nf4-deploy` and `scripts/nf4-test` do not SSH, and they do not copy `deploy.env`, `local.env`, or a private key. For the automated script, use `doc/Automated Sepolia Run.md`. That page stands alone. This page is the manual procedure, one command at a time. Do not run both, and do not run `doc/Sepolia Assistant Quick Start.md` at the same time.

```text
operator VM                         client machine
configuration :8080                 waits until READY
proposer :3001                      then Client 2 on :3000
Client 1 :3000                      own key, own Mongo
~/nightfall_4_CE                    this checkout
```

## Reach the operator

The client machine cannot route to the operator LAN. Use a bastion shell for commands, and a separate tunnel for ports.

```bash
az network bastion ssh \
  --name <bastion-name> \
  --resource-group <resource-group> \
  --auth-type ssh-key \
  --ssh-key id_ecdsa \
  --username <ssh-user> \
  --target-resource-id <operator-vm-resource-id>
```

```bash
az network bastion tunnel \
  --name <bastion-name> \
  --resource-group <resource-group> \
  --target-resource-id <operator-vm-resource-id> \
  --resource-port 22 \
  --port 50023
```

Leave the tunnel running. Forward the operator's localhost ports, not its LAN address:

```bash
ssh -i ~/.ssh/id_ecdsa -p 50023 -N \
  -o ExitOnForwardFailure=yes \
  -L 127.0.0.1:18080:127.0.0.1:8080 \
  -L 127.0.0.1:13001:127.0.0.1:3001 \
  <ssh-user>@127.0.0.1
```

`./scripts/nf4-laptop-tunnel` is the same forward, once port `50023` is listening. Put these lines in the client `deploy.env` only, using the host `--check` prints:

```bash
NF4_JOIN_URL=http://10.1.2.3:8080/configuration/toml/join.toml
NF4_TUNNEL_HTTP=http://127.0.0.1:18080
NF4_TUNNEL_PROPOSER=http://127.0.0.1:13001
```

`10.1.2.3` is the template example. Do not put `NF4_JOIN_URL` in the operator file. Do not publish ports `8080` or `3001`.

Long commands belong in `tmux` session `nf4`. An SSH drop is not a reboot. A reboot kills `tmux` and the in-memory ZKP keys. Do not delete `temp/` or remove containers until you have used the restart section.

## Rules

- Do not copy `deploy.env`, `local.env`, or a private key between machines.
- Copy `NF4_CLIENT2_ADDRESS` to the operator, and later `temp/zkp.public`. Never copy `NF4_CLIENT_KEY`.
- The on-chain proposer URL is the private host `--check` prints. Never write `127.0.0.1` there. Changing that host after deploy is a new contract set, not an edit.
- Do not set `NF4_REDEPLOY` to clear an error. The only setter is `--fresh-start`, and only when you mean a new contract set.
- `configuration/toml/public.env` must be mode `644`. A `600` file is HTTP 403.
- Do not `chmod 777`. Do not `docker compose down` as a wipe. Do not `docker rm` on a resume. Mongo is not a named volume. Removing the container deletes it and does not undeploy Sepolia contracts.
- Between stopping Client 1 and restarting it, do not run `nf4-deploy` on the operator. That starts Client 1.
- Do not print keys, mnemonics, or RPC URLs in the report.
- Host `cast` uses `NF4_RPC_HTTP`. The container still needs `wss://`. Do not point `NF4_RPC_WSS` at `ws://` to silence a host websocket error.
- ZKP phrases live in the env examples. Do not change a phrase after the first `deriveKey`.

## Health

On the operator, after deploy:

```bash
./scripts/nf4 status
./scripts/nf4-deploy --file deploy.env --check
curl -fsS -o /dev/null -w 'configuration HTTP %{http_code}\n' http://127.0.0.1:8080/configuration/toml/join.toml
curl -fsS http://127.0.0.1:3001/v1/health
curl -fsS http://127.0.0.1:3001/v1/synchronisation
curl -fsS http://127.0.0.1:3000/v1/health
stat -c '%a' configuration/toml/public.env
```

Expect configuration HTTP 200, both health bodies `Healthy`, proposer `Synchronised` once catch-up finishes, and mode `644`. `Not synchronised` immediately after boot is not a pass. `./scripts/nf4 logs proposer` and `./scripts/nf4 logs client` follow the process that owns that container.

From the client machine, through the forwards:

```bash
curl -fsS -o /dev/null -w 'join HTTP %{http_code}\n' http://127.0.0.1:18080/configuration/toml/join.toml
curl -fsS http://127.0.0.1:13001/v1/health
curl -fsS http://127.0.0.1:13001/v1/synchronisation
```

Client 2 health is `curl -fsS http://127.0.0.1:3000/v1/health` on the client machine, and only after `READY`.

## Amounts

Fees are `0x00`. The scripts already send these values. Record the post-fee numbers the API returns.

| Token | Client 1 deposit | Client 2 deposit | Client 1 to Client 2 | Client 1 withdraw | Client 2 withdraw |
|---|---|---|---|---|---|
| ERC20 `0x00` | 100 | none, until the soak | 40 | 60 | 40 |
| ERC721 | 1001 | 1002 | 1001, then back | 1001 | 1002 |
| ERC1155 `2001` | 20 | 10 | 8 | 12 | 18 |
| ERC3525 | 100 of 3001 | 100 of 3002 | 40 from 3001 | 60 | 40, plus 3002 |

## 1. Prep

Need Docker, `forge`, `cast`, `cargo`, `curl`, and `python3` on the operator. The client machine needs those before Client 2 starts. Record the branch and commit on both. Stay on the branch that contains these scripts. Do not `git pull` under a live prover.

## 2. Detect the operator host

Leave `NF4_OPERATOR_HOST` unset. On the operator:

```bash
./scripts/nf4-deploy --check
```

Copy the printed `NF4_JOIN_URL=` line into the client `deploy.env`. Use that host, not the `10.1.2.3` example. Do not put the line in the operator file. A refused connection on port 8080 is reachable. A timeout is not.

## 3. RPC

Leave `NF4_RPC_WSS` and `NF4_RPC_HTTP` as placeholders, or delete them. `--init`, `--check`, and deploy call `./scripts/nf4-pick-rpc --network sepolia` and write a matched pair. Nightfall needs `wss://`. Host `cast` uses the `https://` URL. Deploy refuses a split pair unless `NF4_ALLOW_SPLIT_RPC=true`. A URL you already set is not overwritten. A public endpoint that cannot serve logs back to the contract genesis will leave the proposer `Not synchronised`. Put a matched pair with that history in both lines. Do not redeploy only to change the RPC.

## 4. Create `deploy.env`

Only if the file does not exist. Do not copy it between machines. Do not commit it.

```bash
./scripts/nf4-deploy --init operator --file deploy.env
./scripts/nf4-deploy --init client --file deploy.env
```

`deploy.env already exists` means stop. Do not delete the file.

On the operator, one funded wallet is deployer, proposer, and Client 1. Paste that key. Leave the extra key lines commented. Paste the client address, not the client key. On the client machine, paste Client 2's key. A key is `0x` plus 64 hex. An address is `0x` plus 40 hex. Each account that sends needs Sepolia ETH. Do not use an Anvil key.

## 5. Check wallets

```bash
./scripts/nf4-deploy --file deploy.env --check
```

This does not deploy. On the operator, require a non-zero deployer balance, a non-zero client balance, and the line that deployer, proposer, and Client 1 are the same wallet. On the client machine, a join fetch failure before deploy is expected. Do not deploy there to clear it. Copy `NF4_CLIENT2_ADDRESS` to the operator. Do not copy `NF4_CLIENT_KEY`.

## 6. Deploy the operator

Confirm port `50023` is listening, then start the forwards. On the operator, in `tmux`:

```bash
./scripts/nf4-deploy --file deploy.env
```

Success is exit 0, the join URL, `Certified user-2`, `Certified user-3`, `Published C1_ZKP`, `public.env mode 644`, configuration HTTP 200, and both health bodies. `deployer exited 0` is printed when `nf4_indie_deployer` is still present. A skipped Forge run may not have that container. That is not a failure. Leave `NF4_CONTRACTS__DEPLOY_CONTRACTS` false.

`Contracts already have code` is a skipped Forge run. Do not set `NF4_REDEPLOY` because of it. If the command cannot write `join.toml`, run `sudo chown -R "$USER:$USER" configuration/toml` and rerun the same deploy. Do not delete `addresses.toml`.

Record Nightfall, Round Robin, X509, and Verifier from `configuration/toml/addresses.toml`.

## 7. Client machine sees the join file

Client 2 is still down.

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Expect `join HTTP 200` and `Client 2 was not started`. If the client fetch fails and the operator localhost curl is 200, the join URL or the forward is wrong. If the operator curl is refused, rerun the operator deploy. Do not set `NF4_REDEPLOY`.

## 8. Tokens and baseline

On the operator:

```bash
./scripts/nf4-test --file deploy.env --step tokens
./scripts/nf4-test --file deploy.env --step baseline
```

`tokens` deploys the mocks and mints. It skips tokens that already exist. Do not delete `temp/sepolia-test/minted`. `baseline` records the Sepolia block and the Client 1 sync payload. Both health checks must succeed.

## 9. Client 1 deposits

```bash
./scripts/nf4-test --file deploy.env --step deposit
```

This queues four deposits and polls until `READY`. Keep it in `tmux`. HTTP 202 is queued, not confirmed. Do not delete `temp/sepolia-step10`. If the directory exists, the same command polls. Success is the word `READY` in `temp/sepolia-step10/record.txt`. If the proposer stays `Not synchronised` while a request is `Submitted`, stop. Do not restart that proposer. A restart replays `DepositEscrowed`.

`--step evidence` and `--step receipts` save what the logs can prove. An abbreviated hash is blocked evidence. Do not invent the full hash.

## 10. Start Client 2

Only after `READY`.

```bash
./scripts/nf4-deploy --file deploy.env --late-join
curl -fsS http://127.0.0.1:3000/v1/health
```

Expect `Healthy` and `Certified user-4`. Copy `temp/zkp.public` to the operator. Do not copy the key. Do not edit `NF4_LATE_JOIN` by hand.

## 11. Client 2 deposits and the transfer

On the client machine:

```bash
./scripts/nf4-test --file deploy.env --step late-deposit
```

This queues ERC721 1002, ERC1155 10, and ERC3525 100. It does not deposit ERC20, and it does not wait. Poll `--step sync` on both sides until the L2 block matches and is above the baseline. Do not POST again if `temp/sepolia-step11` exists.

On the operator:

```bash
./scripts/nf4-test --file deploy.env --step accept-zkp --name C2_ZKP --from temp/zkp.public
./scripts/nf4-test --file deploy.env --step transfer
```

`transfer` refuses without `C2_ZKP`, and it refuses unless the proposer is `Synchronised`. It waits until all four are `Confirmed` and writes `temp/sepolia-step12/confirmed`. Do not call `stop-client1` without that file.

## 12. Stop, observe, restart

On the operator, only after `confirmed`:

```bash
./scripts/nf4-test --file deploy.env --step stop-client1
```

On the client machine, `--step balances`. ERC20 can 404 until the transfer block is visible. After the block, expect ERC20 40, ERC721 1001, ERC1155 18, and ERC3525 40 received plus 3002.

On the operator:

```bash
./scripts/nf4-test --file deploy.env --step restart-client1
./scripts/nf4-test --file deploy.env --step sync
```

The restart calls `deriveKey` again. Keys are not still in memory. Both sync payloads must share an L2 block.

## 13. Return and withdraw

On the client machine, `--step return-721`. It queues one transfer of 1001 back to `C1_ZKP` and does not wait. Poll until the L2 block advances. Do not run it again if `temp/sepolia-step17` exists.

Withdraw on the operator, then on the client machine:

```bash
./scripts/nf4-test --file deploy.env --step withdraw
```

Each side writes its own directory and will not withdraw again if that directory exists. Wait for descrow selector `0xf3b85fc2`, or a webhook salt, and an L2 block advance. Then `--step l1-balances`. Pass only when that side's L1 ERC20 balance moved.

## 14. Proposer restart and the later deposits

On the operator:

```bash
./scripts/nf4-test --file deploy.env --step restart-proposer
```

This is not `docker compose down`. `NF4_CONTRACTS__DEPLOY_CONTRACTS` stays false. Client 2 stays up. Expect proposer `Healthy`, every block from `proposer-blocks-before.json` still present in `proposer-blocks-after.json`, and both clients on the same L2 block as before the stop.

`--step deposit-again` queues ERC20 10 into `temp/sepolia-step20`. Wait for a new L2 block and a Client 1 commitment of 10. Do not run it twice.

## 15. Soak

`soak-check` does not sleep. At the start, at 24 hours, and at the end, on both machines:

```bash
./scripts/nf4-test --file deploy.env --step soak-check
```

A gapped soak is not a continuous pass. After the end snapshot, on the client machine, with the proposer still `Healthy`:

```bash
./scripts/nf4-test --file deploy.env --step soak-deposit
```

This queues ERC20 10 into `temp/sepolia-step21` and will not run twice.

## Files

| Path | What |
|---|---|
| `configuration/toml/join.toml` | public bundle the client machine fetches |
| `configuration/toml/public.env` | addresses and ZKP public keys, no secrets |
| `temp/sepolia-test/report.md` | one line per script step |
| `temp/sepolia-step10/` | Client 1 deposits |
| `temp/sepolia-step11/` | Client 2 deposits |
| `temp/sepolia-step12/` | transfers |
| `temp/sepolia-step17/` | ERC721 1001 return |
| `temp/sepolia-step18-c1/` and `c2/` | withdraws |
| `temp/sepolia-step20/` | post-recovery deposit |
| `temp/sepolia-step21/` | post-soak deposit |

`temp/` is gitignored. Do not commit `deploy.env` or `local.env`.

## Restart

`docker ps` and `./scripts/nf4 status` before you start or remove anything. Docker brings back configuration and the proposer when they were not explicitly stopped. Client 1 stays stopped. A reboot is not an SSH drop.

Resume is supported only when the operator `--check` still prints the host in the client `NF4_JOIN_URL`, the containers from this run have not been removed, and every `.request-id` for an in-progress step is present. A missing request ID is not a resume. Do not `git pull`. Do not set `NF4_REDEPLOY`.

Outside the Client 1 stop window:

```bash
./scripts/nf4-deploy --file deploy.env --check
./scripts/nf4-deploy --file deploy.env
```

The second command skips Forge when Nightfall already has code, starts what is not healthy, certifies again, and calls `deriveKey`. In the stop window, bring back configuration and the proposer only. Leave Client 1 stopped.

On the client machine, before `READY`, `--check` only. After `READY`:

```bash
./scripts/nf4-deploy --file deploy.env --late-join
```

## Fresh start

Use this when resume is not supported, or when you mean to abandon the run. Keep each machine's `deploy.env`. Do not copy it. Do not change the ZKP phrases.

```bash
./scripts/nf4-deploy --file deploy.env --fresh-start
```

That removes the current Nightfall containers, deletes the step markers, and on the operator deletes `configuration/toml/public.env`. It keeps `deploy.env` and `configuration/bin/keys`. If `addresses.toml` already names Nightfall, it sets `NF4_REDEPLOY=true` for the one deploy. It does not start Client 2. After `deployer exited 0`, the script removes `NF4_REDEPLOY=true`. If that line remains, delete it before the next deploy.

The old `READY` does not count. Do not use `docker compose down` for this wipe.

## When a step fails

| What you see | What to do |
|---|---|
| `deploy.env already exists` | Continue to `--check`. Do not delete the file. |
| wrong wallet, or pin mismatch | Stop. Do not deploy. |
| `Designated proposer is` | Stop. A re-run will not rotate it. |
| `Client 2 is already running` before `READY` | Stop. Do not pass `--late-join`. |
| `curl: (7)` or `join HTTP 000` after deploy | Check the operator localhost first. If that is 200, the client URL or the forward is wrong. |
| `Contracts already have code` | Not a failure. Do not set `NF4_REDEPLOY`. |
| `On-chain proposer URL is` | The operator address changed. Resume is not supported. Fresh start only. |
| `NF4_REDEPLOY=true` still set after exit 0 | Delete it before the next deploy. |
| `request_status=Failed` on a deposit with no commitment | `--step deposit` archives and submits again. Do not delete the directory by hand. |
| proposer `Not synchronised` during a submitted deposit | Stop. Do not restart the proposer. |
| `Transfers are not accepted yet` | Do not run `stop-client1`. |
| `C2_ZKP missing` | Copy `temp/zkp.public` and run `accept-zkp`. Do not transfer first. |
| `rate limit exceeded` or `block not found` | Put a matched RPC pair in `deploy.env` and rerun the same command. Do not redeploy only to change the RPC. |
