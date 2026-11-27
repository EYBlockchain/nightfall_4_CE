# Automated Sepolia deployment

`scripts/nf4-sepolia-e2e` runs `./scripts/nf4-deploy` and `./scripts/nf4-test` on the machine where you start it. `NF4_ROLE` in that machine's `deploy.env` selects the stages. It does not log in, open a tunnel, `git pull`, fund a wallet, or copy a file. You do those.

Run every command from the repository root. Do not follow `doc/Sepolia Deploy Scripts.md` on a machine while this script is running there. That page is the same work, one command at a time.

```text
operator                              client
configuration :8080                   Client 2 on :3000, after READY
proposer :3001                        own key, own Mongo
Client 1 :3000
NF4_ROLE=operator                     NF4_ROLE=client
```

Chain ID is `11155111`. The script refuses an Anvil key and chain `84532`.

## Before the first command

Both checkouts on the same commit. The script does not compare them.

Operator needs `docker`, `forge`, `cast`, `cargo`, `curl`, `python3`. Client needs `docker`, `cast`, `curl`, `python3`. Start Docker first.

Two funded Sepolia wallets. On the operator, one wallet is deployer, proposer, and Client 1. On the client, the other wallet is Client 2. A zero balance stops the run. The script does not create a key.

```bash
cast wallet new
cast wallet address --private-key 0xYOUR_KEY
```

A key is `0x` plus 64 hex. An address is `0x` plus 40 hex. Paste each key only into the `deploy.env` on the machine that owns it. Mode `600`. Do not copy `deploy.env`, `local.env`, or a private key. Do not commit either file.

## 1. Write `deploy.env`

`--init` writes the file and fills a matched Sepolia RPC pair when those lines are still placeholders. A URL you already set is left alone. `deploy.env already exists` means stop. Do not delete the file. Do not change the ZKP phrase after the first `deriveKey`.

On the client:

```bash
./scripts/nf4-deploy --init client --file deploy.env
```

Paste `NF4_CLIENT_KEY`. Leave `NF4_LATE_JOIN=false`.

On the operator:

```bash
./scripts/nf4-deploy --init operator --file deploy.env
./scripts/nf4-deploy --file deploy.env --check
```

Paste `NF4_DEPLOYER_KEY`. Leave the extra key lines commented. Paste `NF4_CLIENT2_ADDRESS` from the client. Do not put `NF4_JOIN_URL` in this file.

Copy the printed `NF4_JOIN_URL=` line into the client file. `10.1.2.3` in the template is refused. Do not rewrite the host to `127.0.0.1`.

If the client cannot route to that host, open your own route. If you use local forwards, set `NF4_TUNNEL_HTTP` and `NF4_TUNNEL_PROPOSER` in the client file. The script does not open those forwards.

## 2. Run

On the operator, stop after the deposit poll prints `READY`:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --through deposit
```

The first deploy generates proving keys. That takes a while. Done means `Certified user-2`, `Certified user-3`, `Published C1_ZKP`, configuration HTTP 200, both health bodies `Healthy`, and `public.env` mode `644`. `Contracts already have code` is a skipped Forge run, not a failure. Deposit is finished when `temp/sepolia-step10/record.txt` contains `READY`.

Copy that file to the same path on the client. Client 2 stays down until it is there. Then:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --through late-deposit
```

`late-join` writes `temp/zkp.public`. Copy that file to the operator. Do not copy the client key.

On the operator, run again with no `--through`. `transfer` waits for `temp/zkp.public`, publishes `C2_ZKP`, then continues through `soak`.

On the client, run again with no `--through`. That continues through `soak-deposit`. Amounts are fixed in `nf4-test`. This script has no amount flags.

Operator stages, in order: `preflight check deploy tokens baseline deposit transfer stop-client1 restart-client1 withdraw restart-proposer deposit-again soak`.

Client stages, in order: `preflight check late-join late-deposit balances return-721 withdraw soak soak-deposit`.

Between `stop-client1` and `restart-client1`, do not run `nf4-deploy` on the operator. That starts Client 1.

If your session can drop, start `tmux` yourself. The script does not.

## 3. Soak

`soak` records a snapshot and returns. It does not sleep for 48 hours unless you passed `--wait-soak`. Run the same command again after 24 hours and after 48 hours. Until `temp/sepolia-e2e/soak-end-at.txt` exists on that machine, the report says the soak is incomplete. `soak-deposit` refuses to run early unless you passed `--allow-short-soak`. The report will not call that a pass.

## If it stops

Read the last line in `temp/sepolia-e2e/report.md`. Rerun the same command unless the table says otherwise.

| What you see | What to do |
|---|---|
| `NF4_CLIENT2_ADDRESS` missing | Paste the address from the client. Do not paste the key. |
| Wallet check failed, or balance is zero | Fund the wallet. Do not deploy. |
| Deployer, proposer, and Client 1 are not the same wallet | Comment out the extra key lines. Do not deploy. |
| `Contracts already have code` | Not a failure. Do not set `NF4_REDEPLOY`. |
| Join URL is not reachable | Fix your own route. The script will not open a tunnel. |
| `Refusing NF4_JOIN_URL` | Use the host from `--check`. Not `127.0.0.1`, not `10.1.2.3`. |
| `Not synchronised` while a deposit is `Submitted` | Stop. Do not restart that proposer. A restart replays `DepositEscrowed`. |
| READY file missing on the client | Copy `temp/sepolia-step10/record.txt` from the operator. |
| `temp/zkp.public` missing on the operator | Copy it from the client. Do not copy the key. |
| A step directory exists but a `.request-id` is missing | Torn. `--fresh-start` on that machine only. Do not delete the directory. |

`Not synchronised` right after boot is normal. A `public.env` mode of `600` is HTTP 403. It must be `644`.

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --status
```

Progress is also one line per stage in `temp/sepolia-e2e/report.md`. That file may contain contract addresses and L2 block numbers. It must not contain keys or RPC URLs.

## Again, or wipe

Rerun the same command on the same machine. Finished stages are skipped. A request id that is already present is not posted again. Do not delete `temp/sepolia-step10`, `step11`, `step12`, `step17`, `step18-c1`, `step18-c2`, `step20`, or `step21` to force a new POST. Do not set `NF4_REDEPLOY` to clear an error. Do not `docker compose down` or `docker rm` a client to clear a deposit error. Mongo is not a named volume. Removing the container deletes it and does not undeploy Sepolia contracts.

A wipe is a separate command, on each machine:

```bash
./scripts/nf4-sepolia-e2e --file deploy.env --fresh-start
```

That wipes only the machine you are on, keeps that machine's `deploy.env`, and does not start Client 2. The next run begins at `check`. The old `READY` does not count. Copy a new one before the next client `late-join`. After a successful deploy, delete `NF4_REDEPLOY=true` if it is still in the operator file.

`--resume` is accepted and ignored. A second run already continues from `temp/sepolia-e2e/state`. `--remote-dir` is rejected.
