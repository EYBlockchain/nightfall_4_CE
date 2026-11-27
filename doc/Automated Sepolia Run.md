# Automated Sepolia deployment

`scripts/nf4-sepolia-e2e` runs `./scripts/nf4-deploy` and `./scripts/nf4-test` on the machine where you start it. `NF4_ROLE` in that machine's `deploy.env` selects the stages. It does not log in, open a tunnel, `git pull`, fund a wallet, or copy a file. You do those.

Run every command from the repository root on that machine. Do not follow `doc/Sepolia Deploy Scripts.md` on a machine while this script is running there. That page is the same work, one command at a time.

```text
operator                              client
configuration :8080                   Client 2 on :3000, after READY
proposer :3001                        own key, own Mongo
Client 1 :3000
NF4_ROLE=operator                     NF4_ROLE=client
```

Chain ID is `11155111`. The script refuses an Anvil key and chain `84532`.

The machine you start it on is the one it deploys. A client machine does not deploy the operator.

## On the operator

Get onto the operator however that server is set up. The script does not log in, and this page does not say how. Confirm the machine before any wipe.

```bash
grep '^NF4_ROLE=' deploy.env
```

`NF4_ROLE` must be `operator`. If it is not, you are on the wrong machine. Stop.

Work from the repository root on that machine. Start `tmux` before any long command. The script does not.

```bash
tmux new-session -s nf4
```

If your session can drop, that is not a reboot. A reboot kills `tmux` and the in-memory ZKP keys.

## Same commit

The script does not compare the two checkouts and does not `git pull`. From the repository root on each machine:

```bash
git rev-parse --short HEAD
git fetch origin
git status --porcelain
git pull --ff-only
```

The two short hashes must match before you deploy. `deploy.env` and `local.env` are gitignored. Leave them.

If `git pull` stops because `nightfall.toml` or `docker-compose.yml` is modified, those are leftovers from the last deploy. The deploy tool rewrites them. Discard only those two, then pull again:

```bash
git checkout -- nightfall.toml docker-compose.yml
git pull --ff-only
```

If any other tracked file is dirty, stop and look. Do not `git checkout -- deploy.env`. Do not delete `configuration/bin/keys` to make the tree clean.

## Tools

The script adds `~/.foundry/bin` and `~/.cargo/bin` to `PATH`. A shell that does not load your profile can report `forge`, `cast`, and `cargo` missing. That is not a missing install. Check with the same path the script uses:

```bash
export PATH="$HOME/.cargo/bin:$HOME/.foundry/bin:$PATH"
command -v docker forge cast cargo curl python3
```

Operator needs all six. Client needs `docker`, `cast`, `curl`, `python3`. Start Docker before you run. `docker info` must succeed.

## 1. Write `deploy.env`

Each machine has its own file, mode `600`. Do not copy `deploy.env`, `local.env`, or a private key. Do not commit either file. Do not change the ZKP phrase after the first `deriveKey`.

`--init` writes the file and fills a matched Sepolia RPC pair when those lines are still placeholders. A URL you already set is left alone. `deploy.env already exists` means `--init` must stop. Do not delete the file. Skip `--init` and go to `--check`. A fresh contract set keeps this file.

On the client, only if the file is absent:

```bash
./scripts/nf4-deploy --init client --file deploy.env
```

Paste `NF4_CLIENT_KEY`. Leave `NF4_LATE_JOIN=false`. Then print the address. This does not start Client 2.

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Copy the printed `NF4_CLIENT2_ADDRESS=` line. Do not copy the key. A join fetch failure on the client before the operator is up is expected. The address line above that error is still the one to copy. Do not deploy on the client to clear it.

On the operator, only if the file is absent:

```bash
./scripts/nf4-deploy --init operator --file deploy.env
```

Paste `NF4_DEPLOYER_KEY`. Leave the extra key lines commented. Paste `NF4_CLIENT2_ADDRESS`. Do not put `NF4_JOIN_URL` in this file. Then:

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Require a non-zero deployer balance, a non-zero client balance, and the line that deployer, proposer, and Client 1 are the same wallet. Copy the printed `NF4_JOIN_URL=` line into the client file. `10.1.2.3` in the template is refused. Do not rewrite the host to `127.0.0.1`.

To see the deployer balance without printing the key:

```bash
addr=$(cast wallet address --private-key "$(awk -F= '/^NF4_DEPLOYER_KEY=/{print $2}' deploy.env)")
cast from-wei "$(cast balance "$addr" --rpc-url "$(awk -F= '/^NF4_RPC_HTTP=/{print $2}' deploy.env)")"
```

The script stops at zero. That is the only balance check. A fresh contract set still spends gas after it. One fresh deploy on this operator finished from about `0.13` ETH. A dust balance can pass the check and fail in Forge. Fund the deployer before `--fresh-start`. Do not echo `deploy.env`.

If the client cannot reach the host `--check` printed, open your own route. If you use local forwards, set `NF4_TUNNEL_HTTP` and `NF4_TUNNEL_PROPOSER` in the client file to those forwards. The script does not open them and does not choose the ports.

A key is `0x` plus 64 hex. An address is `0x` plus 40 hex. To create a wallet, `cast wallet new`, then `cast wallet address --private-key` on that machine only.

## 2. Run

Start `tmux` first, on the operator. Then choose one.

A new contract set. This is not a resume. Confirm `NF4_ROLE=operator` again, then:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --fresh-start
./scripts/nf4-sepolia-e2e --file deploy.env --through deploy
```

`--fresh-start` wipes only this machine. It keeps `deploy.env` and `configuration/bin/keys`. It does not start Client 2. Run it on the client too before the next `late-join`. The old `READY` file does not count.

`--through deploy` is the network. Done means `Certified user-2`, `Certified user-3`, `Published C1_ZKP`, configuration HTTP 200, both health bodies `Healthy`, and `public.env` mode `644`. `Contracts already have code` is a skipped Forge run, not a failure. After a fresh deploy, copy the new `NF4_JOIN_URL=` line from `--check` into the client file even if the host did not change. The join file content did.

`--through deposit` is the next stop, and it is more than the network. It mints tokens, records a baseline, and posts Client 1 deposits. The long wait is the deposit proof, not key generation. `--fresh-start` kept the proving keys. `Computing block` can sit for about half an hour with the CPU pegged. `/v1/synchronisation` can time out during that proof. That is not a hang. Do not restart the proposer.

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --through deposit
```

Deposit is finished when `temp/sepolia-step10/record.txt` contains `READY`. Copy that new file to the same path on the client. Client 2 stays down until it is there.

A second run with no `--fresh-start` resumes. Finished stages are skipped. `--through deposit` on a machine whose last stage is already `deposit` does not deploy again.

After `check OK` the log can go quiet while Docker builds the deployer, proposer, and client images. `--status` stays at `check` until `deploy OK`. `temp/sepolia-e2e/deploy.log` can stay empty when stdout is redirected, because that output is buffered. Look at `docker ps`. A quiet log there is not a stuck run. Do not restart it.

On the client, after the new `READY` file is in place:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --through late-deposit
```

`late-join` writes `temp/zkp.public`. Copy that file to the operator. Do not copy the client key.

On the operator, run again with no `--through`. `transfer` waits for `temp/zkp.public`, publishes `C2_ZKP`, then continues through `soak`.

On the client, run again with no `--through`. That continues through `soak-deposit`. Amounts are fixed in `nf4-test`. This script has no amount flags.

Operator stages, in order: `preflight check deploy tokens baseline deposit transfer stop-client1 restart-client1 withdraw restart-proposer deposit-again soak`.

Client stages, in order: `preflight check late-join late-deposit balances return-721 withdraw soak soak-deposit`.

Between `stop-client1` and `restart-client1`, do not run `nf4-deploy` on the operator. That starts Client 1.

## 3. Soak

`soak` records a snapshot and returns. It does not sleep for 48 hours unless you passed `--wait-soak`. Run the same command again after 24 hours and after 48 hours. Until `temp/sepolia-e2e/soak-end-at.txt` exists on that machine, the report says the soak is incomplete. `soak-deposit` refuses to run early unless you passed `--allow-short-soak`. The report will not call that a pass.

## If it stops

Read the last line in `temp/sepolia-e2e/report.md`. `--status` prints the last finished stage, not the one running. During the deposit proof the state file still says `baseline`. Rerun the same command unless the table says otherwise.

To see whether the network is up, on the operator:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --network
```

That does not deploy. `network: up` means chain ID `11155111`, configuration HTTP 200, proposer `Healthy`, `Synchronised`, assembly `Running`, and the on-chain L2 block equals the proposer block. `l1_block` is the Sepolia block. `l2_block` is `layer2_block_number` on the Nightfall contract. A new network can be up at L2 block `0`. `--status` does not report this.

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --status
docker ps --format '{{.Names}} {{.Status}}'
docker logs --tail 20 nf4_indie_proposer
```

| What you see | What to do |
|---|---|
| `NF4_ROLE` is not `operator` | Stop. You are on the wrong machine. |
| `command -v forge` fails | Export `~/.foundry/bin` and `~/.cargo/bin`, or just run the script. It adds that path. |
| `deploy.env already exists` | Do not delete it. Skip `--init`. |
| `git pull` blocked by `nightfall.toml` or `docker-compose.yml` | Discard those two files and pull. Do not touch `deploy.env`. |
| `NF4_CLIENT2_ADDRESS` missing | Paste the address line from the client `--check`. Do not paste the key. |
| Wallet check failed, or balance is zero | Fund the wallet. Do not deploy. |
| Deployer, proposer, and Client 1 are not the same wallet | Comment out the extra key lines. Do not deploy. |
| Quiet log after `check OK`, `docker ps` shows a build | Wait. Do not restart. |
| `Contracts already have code` | Not a failure. Do not set `NF4_REDEPLOY`. |
| Join URL is not reachable | Fix your own route. The script will not open a tunnel. |
| `Refusing NF4_JOIN_URL` | Use the host from `--check`. Not `127.0.0.1`, not `10.1.2.3`. |
| `/v1/synchronisation` times out, proposer log says `Computing block` | Wait. High CPU is the proof. Do not restart the proposer. |
| `Not synchronised` while a deposit is `Submitted` | Stop. Do not restart that proposer. A restart replays `DepositEscrowed`. |
| READY file missing on the client | Copy `temp/sepolia-step10/record.txt` from the operator. |
| `temp/zkp.public` missing on the operator | Copy it from the client. Do not copy the key. |
| A step directory exists but a `.request-id` is missing | Torn. `--fresh-start` on that machine only. Do not delete the directory. |

`Not synchronised` right after boot is normal. A `public.env` mode of `600` is HTTP 403. It must be `644`.

Progress is also one line per stage in `temp/sepolia-e2e/report.md`. That file may contain contract addresses and L2 block numbers. It must not contain keys or RPC URLs.

## Again, or wipe

Rerun the same command on the same machine. Finished stages are skipped. A request id that is already present is not posted again. Do not delete `temp/sepolia-step10`, `step11`, `step12`, `step17`, `step18-c1`, `step18-c2`, `step20`, or `step21` to force a new POST. Do not set `NF4_REDEPLOY` to clear an error. Do not `docker compose down` or `docker rm` a client to clear a deposit error. Mongo is not a named volume. Removing the container deletes it and does not undeploy Sepolia contracts.

A wipe is the `--fresh-start` command in section 2, on each machine. It keeps that machine's `deploy.env` and proving keys. The next run begins at `check`. The old `READY` does not count. Copy a new `READY` file and the new `NF4_JOIN_URL=` line before the next client `late-join`. After a successful deploy, delete `NF4_REDEPLOY=true` if it is still in the operator file.

`--resume` is accepted and ignored. A second run already continues from `temp/sepolia-e2e/state`. `--remote-dir` is rejected.
