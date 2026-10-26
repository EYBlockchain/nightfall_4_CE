# Sepolia deploy and test scripts

Follow this page from step 1 through step 24. Do not start Client 2 before step 12. Do not run the wizard deploy path in `doc/Sepolia Assistant Quick Start.md` at the same time. That file is the manual fallback if a script step fails and you need the original curl. This page is the procedure.

Two VMs. Two scripts. No SSH. No copied `deploy.env`, `local.env`, or private key.

```text
VM-A                                    VM-B
operator, configuration :8080           waits until step 12
proposer :3001                          then Client 2 on :3000
Client 1 :3000                          own key, own Mongo
```

VM-B uses `nf4_indie_client` on port 3000, not `indie-client2`. Client API ports stay on localhost. Clients do not talk to each other. Both must reach Sepolia `wss://`, `http://<VM-A>:8080`, and `http://<VM-A>:3001`.

Work from the repository root on every command. A new shell does not inherit exports. Pass `--file deploy.env` every time. Do not put keys, mnemonics, or RPC URLs in the report.

These stay manual, because the scripts do not SSH and cannot spend your faucet funds or open a cloud firewall:

- `git switch` and `git pull`
- funding an L1 account
- opening TCP `8080` and `3001` from VM-B to VM-A
- copying `NF4_CLIENT2_ADDRESS` to VM-A, the join URL to VM-B, and `temp/zkp.public` to VM-A
- waiting out the soak and running `soak-check` again at 24 hours and at the end
- explorer screenshots, and any escrow hash the logs only show abbreviated

Everything else in the numbered steps is a script.

Long runs belong in `tmux`, or an SSH drop kills key generation, the deposit poll, or the soak:

```bash
tmux new -s nf4
# detach: Ctrl-b then d
# reattach: tmux attach -t nf4
```

A new pane does not have your shell variables. That does not matter for these scripts, because each command reads `--file`.

A VM reboot is not an SSH drop, and it is not step 18 or step 21. `tmux` dies with the machine. In-memory ZKP keys die with it. Do not delete `temp/` or remove containers until you have used the VM restart section. Resume is only the cases listed there. Anything else is a fresh start, and a fresh start is a new report.

## Amounts

Fees are `0x00`. ERC721 `value` is `0x00`. Token IDs are even-length hex. The scripts already send these values. Balances below are pre-fee. Record the post-fee numbers the API and Mongo actually return.

| Token | type | id | Client 1 deposit | Client 2 deposit | Client 1 to Client 2 | Client 1 withdraw | Client 2 withdraw |
|---|---|---|---|---|---|---|---|
| ERC20 | 0 | `0x00` | `0x64` (100) | none, until the soak | `0x28` (40) | `0x3c` (60) | `0x28` (40) |
| ERC721 | 2 | Client 1 `0x03e9` (1001); Client 2 `0x03ea` (1002) | 1001 | 1002 | 1001, then back | 1001 | 1002 |
| ERC1155 | 1 | `0x07d1` (2001) | `0x14` (20) | `0x0a` (10) | `0x08` (8) | `0x0c` (12) | `0x12` (18) |
| ERC3525 | 3 | Client 1 `0x0bb9` (3001); Client 2 `0x0bba` (3002); slot 1 | `0x64` (100) | `0x64` (100) | `0x28` (40) from 3001 | `0x3c` (60) | `0x28` (40), plus original 3002 |

After the transfer block, before withdraws: Client 1 holds ERC20 60, ERC1155 12, ERC3525 60, and ERC721 1001 is spent. Client 2 holds ERC20 40, ERC721 1001 and 1002, ERC1155 18, and ERC3525 40 received plus its own 3002.

ZKP phrases, if you do not already have one. These are not L1 keys.

Client 1: `spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work`

Client 2: `wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law`

## 1. Prep, both VMs

Need Docker, `forge`, `cast`, `cargo`, `curl`, and `python3`. Chain ID `11155111`. `84532` is Base Sepolia. Do not use Anvil, `--yes`, or `--network local`.

```bash
git switch auto/testnet
git pull
```

The branch switch stays manual. The deploy script runs `check deployer`, stops a leftover Anvil container, and on a first real-prover deploy runs `check prover`. `docker compose down` does not undeploy Sepolia contracts.

Record the branch and commit.

## 2. VM-A address is detected

Leave `NF4_OPERATOR_HOST` unset. On VM-A, this does not need `deploy.env` and does not start Cargo:

```bash
./scripts/nf4-deploy --check
```

It reads the local interfaces, skips loopback, link-local, and Docker bridge addresses, and probes TCP port 8080. Connection refused counts as reachable, because nothing is listening yet. A timeout does not. It prefers a private address (`10.`, `192.168.`, or `172.16–31` other than `172.17` and `172.28`) over a public one when both answer.

The command prints three lines. The third is the one to copy:

```text
Operator host: 10.0.0.8
Join URL: http://10.0.0.8:8080/configuration/toml/join.toml
NF4_JOIN_URL=http://10.0.0.8:8080/configuration/toml/join.toml
```

On VM-B, put that third line in `deploy.env`. Step 4 creates that file with `./scripts/nf4-deploy --init client --file deploy.env`. Open `deploy.env` on VM-B and set:

```bash
NF4_JOIN_URL=http://10.0.0.8:8080/configuration/toml/join.toml
```

Use the printed host, not this example. Do not put `NF4_JOIN_URL` in VM-A's `deploy.env`. A line that says the local probe timed out is still a result: nothing is listening on 8080 yet, and some hosts cannot connect to their own address. Confirm the URL from VM-B in step 7. If VM-B cannot route to it, set `NF4_OPERATOR_HOST` on VM-A and run `./scripts/nf4-deploy --check` again.

That address is written on chain. Do not change it after deploy. If it changes, transfers break until you deploy again with `NF4_REDEPLOY=true`.

## 3. RPC is filled automatically

Leave `NF4_RPC_WSS` and `NF4_RPC_HTTP` as the example placeholders, or delete them. `--init`, `--check`, and deploy call `./scripts/nf4-pick-rpc --network sepolia` and write the chosen pair into `deploy.env`. Nightfall needs `wss://`. `cast` uses the `https://` pair. A chain-id check does not prove `eth_subscribe` will stay up. If the listener later drops, put a keyed provider in those two lines. The script will not overwrite a URL you already set. Do not redeploy just to change the RPC, and do not put the RPC URL in `join.toml` or the report.

## 4. Create deploy.env and paste one key

Do not copy a file between VMs. Do not commit `deploy.env`. `--init` only creates a missing file. It copies the example, sets mode 600, and fills the RPC if those lines are still placeholders.

```bash
# VM-A, only if deploy.env does not exist
./scripts/nf4-deploy --init operator --file deploy.env

# VM-B, only if deploy.env does not exist
./scripts/nf4-deploy --init client --file deploy.env
```

`deploy.env already exists. Not overwriting.` means stop. The file is already there. Do not delete it. Do not run `--init` again. Go to step 5.

VM-A uses one funded wallet for deployer, proposer, and Client 1. Paste that key. Leave the proposer and Client 1 key lines commented. Paste the address VM-B printed. That line is not a key.

```bash
NF4_DEPLOYER_KEY=0xYOUR_EXISTING_PRIVATE_KEY
NF4_CLIENT2_ADDRESS=0xADDRESS_PRINTED_BY_VM_B
```

VM-B pastes Client 2's key. Leave `NF4_JOIN_URL` commented until step 5 prints the host.

```bash
NF4_CLIENT_KEY=0xYOUR_EXISTING_CLIENT2_PRIVATE_KEY
```

A key is `0x` plus 64 hex characters. An address is `0x` plus 40 hex characters. Each account that sends a transaction needs about 0.5 Sepolia ETH. Do not use an Anvil key.

Do not edit any other line unless a later error names it. Then run step 5.

Only if a role uses a different wallet, uncomment that role's key and address. A different proposer is registered on chain. A later run will not rotate it. Only if you have no wallet, run `cast wallet new`, fund the address, then paste that key. Only if this client already has a 24-word ZKP phrase, replace that VM's `NF4_ZKP_MNEMONIC`. Do not switch phrases after the first `deriveKey`.

## 5. Check the wallets, both VMs

This is the command after step 4, including when `--init` printed `deploy.env already exists`. Do not pass `--init` here.

```bash
./scripts/nf4-deploy --file deploy.env --check
```

The command does not start Docker and does not deploy. On VM-A, read the output in this order:

| Printed line | What to do |
|---|---|
| `Operator host:` and `Join URL:` | This is the host VM-B must use. Do not put it in VM-A's `deploy.env`. On VM-B, set `NF4_JOIN_URL=` to that URL, without the `Join URL:` label. This command does not print an `NF4_JOIN_URL=` line. |
| `Set NF4_OPERATOR_HOST only if VM-B cannot reach that address.` | Leave `NF4_OPERATOR_HOST` unset unless VM-B cannot open that host. |
| `deployer, proposer, and Client 1 are the same existing wallet.` | Correct for the normal layout. Leave the proposer and Client 1 key lines commented. Do not paste the deployer key into those lines. |
| `client2:` | The address already in VM-A's file. It is not a key. If it is not the address VM-B printed, replace `NF4_CLIENT2_ADDRESS` and run this check again. |
| `deployer balance:` and `client 2 balance:` | A non-zero number means that account can send. Zero means fund it, then run this check again. Do not deploy. |
| `Wallet check OK. No contracts were deployed.` | The check passed and did not send a transaction. This line does not say whether contracts already exist. If step 6 has not exited 0, open the ports and run step 6. If it has, do not deploy again. |

A pin mismatch looks like `NF4_DEPLOYER_ADDRESS is 0x..., but the key in this file derives 0x...`. Fix the key or the pin. Do not deploy.

On VM-B a clean wallet check looks like `Wallet check OK. Client 2 was not started.` Copy the printed `NF4_CLIENT2_ADDRESS=0x...` line into VM-A's `deploy.env`. Do not copy `NF4_CLIENT_KEY`.

The same VM-B check then fetches `NF4_JOIN_URL`. Before step 6 has exited 0, `curl: (7)` and `join HTTP 000` are expected: nothing is listening on 8080 yet. The wallet lines above that curl are still valid, but the command exits non-zero because the fetch failed. Do not deploy on VM-B to clear it. Do not pass `--late-join`. Copy `NF4_CLIENT2_ADDRESS` to VM-A, finish step 6 there, then repeat this check and expect `join HTTP 200`.

If the curl host is `10.0.0.8`, that is the example left in `deploy.env`. VM-A did not print that host. On VM-B, replace it with the URL from VM-A's `Join URL:` line. Do not paste the label:

```bash
NF4_JOIN_URL=http://HOST_FROM_VM_A:8080/configuration/toml/join.toml
```

Do not deploy on VM-B. After step 6 exits 0, run the check again and expect `join HTTP 200`. Before that, connection refused on the correct host is still expected. Connection refused is not a timeout. A timeout is a drop.

Record the four L1 addresses. Do not record the keys.

## 6. Open ports, then deploy VM-A

Open TCP `8080` and `3001` from VM-B to VM-A. Client ports 3000 stay closed to the other VM.

On VM-A:

```bash
./scripts/nf4-deploy --file deploy.env
```

This stops leftover Anvil, runs the prover check on a first real-prover deploy, deploys contracts, starts configuration, the proposer, and Client 1, certifies the proposer with `user-2` and Client 1 with `user-3`, and publishes `C1_ZKP` when the mnemonic is set. Key generation is slow. Real proving keys need a large machine. VM-A is that machine.

Success is the script exiting 0. It prints the join URL, `Certified user-2`, `Certified user-3`, a 64-hex `C1_ZKP` line, configuration HTTP 200, and both health bodies. `deployer exited 0` is printed when `nf4_indie_deployer` is still present. A second run that skipped Forge may not have that container. That is not a failure. `NF4_CONTRACTS__DEPLOY_CONTRACTS` in `local.env` is `false` after success. Leave it false.

A second run skips Forge when that Nightfall contract already has code. It does not wipe token addresses or ZKP keys in `public.env`. It will not change the on-chain proposer URL or the designated proposer wallet.

`Contracts already have code. Skipping Forge deploy.` is that second run. Do not set `NF4_REDEPLOY` because of it. The configuration container starting is not the failure. If `configuration/toml` was created by Docker as root, the same command moves that directory aside, copies readable files back, recreates `nf4_configuration` when it is already running, and writes `join.toml`. It does not delete `addresses.toml`. Do not `chmod 777`. Run it as your user, not under `sudo`.

If it still prints `Permission denied`, the parent directory is not writable and passwordless `sudo` was not available. Then, on VM-A:

```bash
sudo chown -R "$USER:$USER" configuration/toml
./scripts/nf4-deploy --file deploy.env
```

Success is still exit 0, the join URL, both certifications, `C1_ZKP`, configuration HTTP 200, and `deployer exited 0`.

Record Nightfall, Round Robin, X509, and Verifier from `configuration/toml/addresses.toml`, plus explorer links. Those addresses are also in `configuration/toml/public.env` after this step. `public.env` has no keys and no RPC URL.

## 7. VM-B can see the join file, and Client 2 is still down

On VM-B, after `NF4_JOIN_URL` is the printed URL, do not deploy yet.

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Expect `join HTTP 200` and no error about Client 2 already running. `Wallet check OK. Client 2 was not started.` is still required. Do not deploy on VM-B if the join fetch fails.

`curl: (7)` and `join HTTP 000` mean VM-B did not get `join.toml`. Leave Client 2 down. On VM-A, not on VM-B:

```bash
curl -fsS -o /dev/null -w 'configuration HTTP %{http_code}\n' http://127.0.0.1:8080/configuration/toml/join.toml
```

`configuration HTTP 200` means the file is being served on VM-A. VM-B is then using the wrong host, or TCP 8080 from VM-B to VM-A is refused. Compare `NF4_JOIN_URL` with the `Join URL:` line step 6 printed. Step 6 does not print an `NF4_JOIN_URL=` assignment. `10.0.0.8` is only an example. If the printed host is the one VM-B cannot open, set `NF4_OPERATOR_HOST` on VM-A to an address VM-B can route to and run `./scripts/nf4-deploy --check` there again. If step 6 already wrote the old host on chain, that change is a new deploy with `NF4_REDEPLOY=true`, not an edit. Open TCP 8080 from VM-B to that host. Do not open port 3000.

Connection refused on VM-A's own curl means configuration is not up. Re-run `./scripts/nf4-deploy --file deploy.env` on VM-A. Do not set `NF4_REDEPLOY` unless the VM restart section says this is a fresh start. When VM-A returns 200, run the VM-B check again. Step 8 stays on VM-A. It does not wait on this curl.

On VM-A, `docker ps --filter name=nf4_indie_client` shows Client 1. That is expected. The empty-container check is only for VM-B.

## 8. Mock tokens, VM-A

```bash
./scripts/nf4-test --file deploy.env --step tokens
```

This deploys ERC20, ERC721, ERC1155, and ERC3525 mocks, reads the `Deployed ERC*Mock to:` lines, mints 1001/1002, 2001, and 3001/3002, and appends the addresses to `configuration/toml/public.env`. It does not mint on VM-B. Do not run `client deploy-mock-tokens` on VM-B.

If a mint fails halfway, do not delete `temp/sepolia-test/minted` if that file exists. If it does not exist, a retry starts again at the first mint and ERC721 1001 will revert because it was already minted. Inspect `temp/sepolia-test/mock-deploy.log` and send only the missing `cast send` from the script's mint list. The constructor already minted the ERC20 supply. The script does not mint ERC20 again.

Record the four token addresses and the mint transaction hashes.

## 9. Baseline, VM-A

Client 2 is still down.

```bash
./scripts/nf4-test --file deploy.env --step baseline
```

Expect both health checks to succeed. Record the Sepolia block number and the Client 1 sync payload in `temp/sepolia-test/baseline-sync.json`. An empty ERC20 balance is fine. Do not deposit yet if you still need to reconcile an old balance of these token IDs. The later poll assumes these deposits are the holdings under test.

## 10. Client 1 deposits, VM-A

```bash
./scripts/nf4-test --file deploy.env --step deposit
./scripts/nf4-test --file deploy.env --step poll
```

`deposit` writes `temp/sepolia-step10/` and refuses to run again if that directory exists. HTTP `202` means queued, not escrowed and not confirmed. Do not delete the directory to POST again. If you already have the four `.request-id` files, run `poll` only.

`poll` is the section 10c Mongo check. It runs up to 360 times, 30 seconds apart: about three hours plus query time. `READY` means the four mapped commitments are `Unspent` with an L2 block and a block-submission hash, and the request did not fail. It does not replace explorer evidence. `PendingCreation` is not confirmation. More than one L2 block is allowed. Record every block and `block_tx` in `temp/sepolia-step10/record.txt`.

While that runs, or after it, snapshot logs:

```bash
./scripts/nf4-test --file deploy.env --step evidence
```

For each deposit, find the escrow transaction in `temp/sepolia-step10/client.log`. Search for `Escrowing funds` and `DepositEscrowed`. Log hashes may be abbreviated. Copy the full hash from the explorer, not a string that contains an ellipsis. The transaction must target the Nightfall address, come from Client 1, and the decoded escrow input must match the token, id, type, value, and fee. An approval is not the escrow. If two deposits cannot be told apart, mark that evidence blocked. Do not guess.

For each L2 block, record assembly, proving, and submission from `temp/sepolia-step10/proposer.log`. `Block computation took` by itself is not success. Then:

```bash
./scripts/nf4-test --file deploy.env --step receipts
```

That saves a receipt only for full hashes on escrow or block-submission lines. Other 32-byte values in the logs are ignored. A hash that is not a successful transaction is skipped. If none are saved, copy each full hash from the explorer and run `--step receipt --tx 0xFULL_HASH` once per hash.

A successful receipt proves inclusion, not finality. Record the explorer confirmation state at the time you capture it. Do not attach commitment preimages. They contain deposit secrets. Redact RPC URLs in any log excerpt.

Do not continue until `poll` has printed `READY` and the receipts you need are saved. A timeout, `Failed`, or a missing mapping is not a reason to deposit again.

## 11. Still do not start Client 2 until READY

If step 10 has not printed `READY`, stop. Step 12 is the first time VM-B runs deploy.

## 12. Start Client 2, VM-B

```bash
./scripts/nf4-deploy --file deploy.env --late-join
```

Do not edit `NF4_LATE_JOIN` by hand. The flag is the confirmation that step 10 printed `READY`.

This fetches `join.toml`, addresses, hashes, and proving keys. It writes this VM's `local.env` with `NF4_CONTRACTS__DEPLOY_CONTRACTS=false`. It starts `nf4_indie_client` on port 3000, certifies with `user-4`, and prints `C2_ZKP` (64 hex characters, no `0x`). It does not run `wizard deploy`.

If it stops because `local.env` contains `DEPLOYER_SIGNING_KEY`, that file was copied from VM-A. Delete `local.env` and run the command again. Keep the key on VM-A.

```bash
curl -fsS http://127.0.0.1:3000/v1/health
```

Expect `Healthy`. Copy `C2_ZKP` to VM-A. It is a public key, not a secret. Do not copy `NF4_CLIENT_KEY`.

Record the `Certified user-4` line. The script also writes `temp/zkp.public`. Copy that file to VM-A. Do not copy the key.

## 13. Client 2 deposits, VM-B

```bash
./scripts/nf4-test --file deploy.env --step late-deposit
```

This queues ERC721 1002, ERC1155 10, and ERC3525 100. It does not deposit ERC20. ERC20 40 arrives later by transfer. The step refuses to run again if `temp/sepolia-step11/` exists.

Wait for the next L2 block. `./scripts/nf4-test --file deploy.env --step sync` on both VMs should show the same L2 block before you transfer. Client 2 should then hold 1002, `2001:10`, and `3002:100`.

## 14. Give VM-A the Client 2 ZKP public key

Copy `temp/zkp.public` from VM-B to VM-A. It holds 64 hex characters and no private key. A placeholder is rejected.

```bash
./scripts/nf4-test --file deploy.env --step accept-zkp --name C2_ZKP --from temp/zkp.public
```

`C1_ZKP` was published in step 6. VM-B reads it from `public.env` when it returns the ERC721. You do not copy `C1_ZKP` unless that line is missing.

## 15. Transfers, VM-A

```bash
./scripts/nf4-test --file deploy.env --step transfer
./scripts/nf4-test --file deploy.env --step transfer-status
./scripts/nf4-test --file deploy.env --step transfer-evidence
```

`transfer` queues four transfers to `C2_ZKP` and writes `temp/sepolia-step12/`. Do not delete that directory to send them again.

`transfer-status` polls `/v1/request/<id>` until all four are accepted, up to 360 times, 30 seconds apart. Accepted is not L2 confirmation. If a status is `Failed`, `Error`, or `Rejected`, stop. Do not run `stop-client1`. Do not resubmit. Inspect `./scripts/nf4 logs client` and `./scripts/nf4 logs proposer`.

`transfer-evidence` writes the log snapshot to `temp/sepolia-step12/`. It does not overwrite the deposit logs in `temp/sepolia-step10/`. Proposer logs should show client transactions found, not `Found 0 client transactions`. Record the four transfer request IDs from `temp/sepolia-step12/*.request-id`.

## 16. Stop Client 1, VM-A

Only after `transfer-status` has written `temp/sepolia-step12/accepted`. The stop step refuses to run without that file. Do not delete it to skip the wait.

```bash
./scripts/nf4-test --file deploy.env --step stop-client1
```

This stops `nf4_indie_client` on VM-A only. It does not stop VM-B and it does not stop the proposer. Record the UTC time and the sync payload in `temp/sepolia-test/before-stop-sync.json`.

## 17. Confirm the transfer on VM-B

```bash
./scripts/nf4-test --file deploy.env --step balances
```

Client 2's ERC20 balance can 404 until this block. That is expected before the transfer is included. After the block, expect ERC20 40, ERC721 1001, ERC1155 18. ERC3525 received value is 40, plus its own 3002. Record `temp/sepolia-test/c2-sync.json` and `c2-commitments.json`.

If the proposer log said `Found 0 client transactions`, start Client 1 again with step 18, `deriveKey` is inside that step, resubmit only if `temp/sepolia-step12/` does not exist, then stop Client 1 again. Do not create a second transfer directory to force a retry of a transfer that was already queued.

## 18. Restart Client 1, VM-A

```bash
./scripts/nf4-test --file deploy.env --step restart-client1
./scripts/nf4-test --file deploy.env --step sync
```

Keys are in memory. The step calls `deriveKey` again before reading commitments. Expect Client 1 caught up: ERC20 60, ERC1155 12, ERC3525 60, ERC721 1001 spent.

On VM-B:

```bash
./scripts/nf4-test --file deploy.env --step sync
```

The L2 block in both sync payloads must match. Record both payloads.

## 19. Return ERC721 1001, VM-B

```bash
./scripts/nf4-test --file deploy.env --step return-721
```

This sends 1001 back to `C1_ZKP`. It refuses to run again if `temp/sepolia-step17/` exists. Wait for the next L2 block. Client 1 holds 1001 again. Client 2 does not.

## 20. Withdraw, each VM

On VM-A, then on VM-B:

```bash
./scripts/nf4-test --file deploy.env --step withdraw
```

VM-A withdraws Client 1's remaining amounts to Client 1's L1 address. VM-B withdraws Client 2's remaining amounts to Client 2's L1 address. Each side writes its own directory, `temp/sepolia-step18-c1/` or `temp/sepolia-step18-c2/`, and will not withdraw again if that directory exists. The step also prints webhook salts when the local webhook has any.

Wait for the L2 block and for the L1 descrow. The descrow selector is `0xf3b85fc2`. Then, on each VM:

```bash
./scripts/nf4-test --file deploy.env --step l1-balances
```

Pass only when the L1 ERC20 balances move. Record the withdraw request IDs, descrow hashes, salts, and the balances the step prints.

## 21. Restart the proposer, VM-A

```bash
./scripts/nf4-test --file deploy.env --step restart-proposer
```

This stops and starts `nf4_indie_proposer` only. It is not `docker compose down`. `NF4_CONTRACTS__DEPLOY_CONTRACTS` stays `false`. VM-B stays up. Record the UTC times in `temp/sepolia-test/proposer-stopped-at.txt` and the block list before the stop.

Expect `Healthy` after the restart. Compare `temp/sepolia-test/proposer-blocks-before.json` with `proposer-blocks-after.json`. The previous L2 blocks must still be there.

On both VMs:

```bash
./scripts/nf4-test --file deploy.env --step sync
```

Both clients must be on the same L2 block as before the restart. Record the downtime.

## 22. Deposit after recovery, VM-A

```bash
./scripts/nf4-test --file deploy.env --step deposit-again
```

This queues an ERC20 deposit of 10 (`0x0a`). It writes `temp/sepolia-step20/` and will not run twice. Wait for a new L2 block. Expect a Client 1 commitment of 10. Record the request ID and the block.

## 23. Soak

Leave the proposer and both clients up. `soak-check` does not sleep.

At the start, at 24 hours, and at the end, on both VMs:

```bash
./scripts/nf4-test --file deploy.env --step soak-check
```

Snapshots append to `temp/sepolia-test/soak.txt`. Proposer host resources are VM-A. If you stop early, record the actual duration as skipped or blocked. Do not call that a 48 hour pass.

You can detach `tmux` while this runs.

## 24. Client 2 deposit after the soak, VM-B

The proposer must still be `Healthy`.

```bash
./scripts/nf4-test --file deploy.env --step soak-deposit
```

This queues an ERC20 deposit of 10 and writes `temp/sepolia-step21/`. It will not run twice. Wait for a new L2 block while VM-A's proposer stays healthy. Record the request ID and the block.

## Where the files are

| Path | What |
|---|---|
| `configuration/toml/join.toml` | public deploy bundle VM-B fetches |
| `configuration/toml/public.env` | addresses and ZKP public keys, no secrets |
| `temp/sepolia-test/report.md` | one line per script step |
| `temp/sepolia-step10/` | Client 1 deposits, poll record, log snapshot, receipts |
| `temp/sepolia-step11/` | Client 2 late deposits |
| `temp/sepolia-step12/` | Client 1 transfers |
| `temp/sepolia-step17/` | ERC721 1001 return |
| `temp/sepolia-step18-c1/` and `c2/` | withdraws |
| `temp/sepolia-step20/` | post-recovery deposit |
| `temp/sepolia-step21/` | post-soak Client 2 deposit |

`temp/` is gitignored. Do not commit `deploy.env` or `local.env`.

## VM restart

Use this when the machine rebooted, Docker restarted, or the disk came back without the processes. `--init` will not overwrite an existing `deploy.env`. Do not delete that file to force it.

`docker ps` and `./scripts/nf4 status` before you start or remove anything.

Docker brings back `nf4_configuration`, `nf4_indie_proposer`, and `nf4_db_proposer` when they were not explicitly stopped (`restart: unless-stopped`). `nf4_indie_client` and `nf4_db_client` stay stopped. Their files are still in the container. Mongo is not on a named volume, so a reboot keeps it and `docker rm` or `docker compose down` deletes it. `docker compose down` still does not undeploy Sepolia contracts.

Pick one path. Do not mix them. A fresh start deletes the markers a resume needs.

### Resume

Resume is supported only when all of these are true:

- On VM-A, `./scripts/nf4-deploy --file deploy.env --check` prints the same host that VM-B already has in `NF4_JOIN_URL`. The no-file `--check` ignores a host pinned in `deploy.env`. If deploy then says `On-chain proposer URL is`, the address changed. Resume cannot rewrite it.
- The Nightfall containers from this run have not been removed.
- The step had not submitted yet, or the submit finished. Finished means every `.request-id` file for that step is present. A directory with one missing, or a curl that died before HTTP 202 was written, is not a resume.
- If step 6 already exited 0, `configuration/toml/addresses.toml` has `nightfall`, `round_robin`, `x509`, and `verifier`. A re-run skips Forge once Nightfall has code, and it will not finish a set that died after that. A reboot during `check prover`, key generation, or Forge before Nightfall has code is a retry of `./scripts/nf4-deploy --file deploy.env`, not a resume of the killed Forge process. Do not set `NF4_REDEPLOY` for that retry.

Do not `git pull`. Do not delete `temp/sepolia-step*`. Do not set `NF4_REDEPLOY`.

Start a new `tmux` session. The old one is gone.

Client 1 must stay down from step 16 until step 18. In that window, do not run `nf4-deploy` on VM-A. Port 3000 is down, so that command starts Client 1. Bring back only what the test still needs:

```bash
./scripts/nf4 up configuration
./scripts/nf4 up proposer
```

Leave `nf4_indie_client` stopped. If `temp/sepolia-step12/accepted` exists and the container is already stopped, do not run `stop-client1` again. That step curls Client 1 first and fails while it is down.

On VM-A, everywhere else:

```bash
./scripts/nf4-deploy --file deploy.env --check
./scripts/nf4-deploy --file deploy.env
```

The second command skips Forge when Nightfall already has code, starts what is not healthy, certifies again, and calls `deriveKey` again. A repeat certification is accepted when that address is already on chain. Keys are not still in memory.

On VM-B, before step 12, `--check` only. After step 12, which is allowed only because step 10 already printed `READY`:

```bash
./scripts/nf4-deploy --file deploy.env --late-join
curl -fsS http://127.0.0.1:3000/v1/health
```

The flag here means Client 2 is allowed to start. It is not a second join. Do not pass it if step 10 never printed `READY`.

Webhook events already saved stay in `.nightfall/webhook/events.jsonl`. The process does not. If you still need new events:

```bash
./scripts/nf4 webhook start
```

Do not delete that file on a resume. Then continue at the row. A poll starts its 360-iteration clock again. That is the resume.

| Interrupted | Resume | Do not |
|---|---|---|
| `check prover` or key generation | `./scripts/nf4-deploy --file deploy.env` | `NF4_REDEPLOY`, unless Nightfall already has code |
| step 6, all four addresses in `addresses.toml`, services down | same deploy command | delete `configuration/toml`. A missing Round Robin, X509, or Verifier line is a fresh start |
| step 8, `temp/sepolia-test/minted` exists | `--step tokens` skips the mint | delete `minted` |
| step 8, mint died and `minted` does not exist | not a resume. Step 8 says which `cast send` is still safe | re-run `tokens` and expect 1001 to mint again |
| step 9 | `--step baseline` | |
| step 10, no `temp/sepolia-step10` | `--step deposit`, then `--step poll` | |
| step 10, all four `.request-id` files exist | `--step poll` | `--step deposit`, or delete the directory |
| step 10 directory exists, a `.request-id` is missing | not supported. Fresh start | delete the directory and POST again |
| evidence or receipts | re-run that step. `docker logs` survive `docker stop`, not `docker rm` | |
| step 12, container down | `--late-join`, as above | copy `local.env` from VM-A |
| step 13, `temp/sepolia-step11` has all three `.request-id` files | `--step sync` on both VMs | `--step late-deposit` |
| step 13 directory torn | not supported. Fresh start | |
| step 14, `C2_ZKP` missing | `--step accept-zkp` from `temp/zkp.public`. A VM-B `--late-join` rewrites that file | |
| step 15, all four transfer `.request-id` files exist | `--step transfer-status` | `--step transfer` |
| step 15 directory torn | not supported. Fresh start | |
| steps 16–17 | configuration and proposer only. Client 1 stays stopped | `nf4-deploy`, `stop-client1` |
| step 18, container still exists | `--step restart-client1`, or `nf4-deploy` if configuration is also down | |
| step 19, `temp/sepolia-step17` has its `.request-id` | wait for the block, then `--step sync` | `--step return-721` |
| step 20, that VM's withdraw directory has all four `.request-id` files | wait for descrow, then `--step l1-balances` | `--step withdraw` |
| step 21, `proposer-blocks-after.json` exists and proposer is `Healthy` | `--step sync` on both VMs | `--step restart-proposer` |
| step 21 died before that file | a re-run is a new stop and start, not the original measurement. Record that | treat the new downtime as the original one |
| step 22 or 24, directory exists | wait for the block | delete the directory and deposit again |
| step 23 | `--step soak-check` once services are up. Record the reboot gap. A gapped soak is not a continuous pass | |

A torn directory is `temp/sepolia-step10`, `step11`, `step12`, `step17`, `step18-c1`, `step18-c2`, `step20`, or `step21` with any expected `.request-id` missing. The POST may already have reached the client. There is no script resume for that.

If the disk did not survive, this is not a resume. Request IDs and Mongo are gone. Fresh start, and only if you still have the same keys to paste into a new `deploy.env`.

### Fresh start

A fresh start retries the procedure from step 5. It is a new report. It is not a way to POST the same deposits, transfers, or withdraws again on the same contracts.

Use it when resume is not supported, or when you mean to abandon the run. Do it on both VMs. A wiped VM-A and a still-running VM-B are not a resume. If a mint, escrow, transfer, or withdraw may have landed, the new run needs new contracts. ERC721 1001 cannot be minted twice on the same token contract, and a second POST of an escrowed token reverts or sends twice.

Keep each VM's `deploy.env`. Do not copy it across. Do not `--init`. Leave the ZKP phrases. Do not `git switch` unless you mean a different branch.

Copy `docker logs` first if you still want the abandoned run. Then on VM-A:

```bash
docker rm -f nf4_indie_client nf4_db_client nf4_indie_proposer nf4_db_proposer nf4_configuration nf4_indie_deployer || true
rm -rf temp/sepolia-step10 temp/sepolia-step11 temp/sepolia-step12 temp/sepolia-step17 \
  temp/sepolia-step18-c1 temp/sepolia-step18-c2 temp/sepolia-step20 temp/sepolia-step21 \
  temp/sepolia-test
rm -f configuration/toml/public.env .nightfall/webhook/events.jsonl
```

Leave `configuration/bin/keys`. Removing `public.env` is what stops the next deploy from keeping the old `TOKEN*` lines and ZKP keys. If those lines stay, `--step tokens` does not deploy new mocks, and the 1001 mint reverts.

On VM-B:

```bash
docker rm -f nf4_indie_client nf4_db_client || true
rm -rf temp/sepolia-step11 temp/sepolia-step17 temp/sepolia-step18-c2 temp/sepolia-step21 \
  temp/sepolia-test temp/zkp.public temp/public.env
rm -f .nightfall/webhook/events.jsonl
```

Do not start Client 2. If VM-B `local.env` contains `DEPLOYER_SIGNING_KEY`, delete that file and keep the key on VM-A.

If Nightfall already has code, set this in VM-A `deploy.env` for the one deploy:

```bash
NF4_REDEPLOY=true
```

If Nightfall has no code, leave `NF4_REDEPLOY` unset. The next deploy runs Forge from the start. Contracts that landed in the killed attempt stay on Sepolia and are not reused. If `addresses.toml` has a Nightfall address with code but Round Robin, X509, or Verifier is missing, the script skips Forge unless `NF4_REDEPLOY=true`. That skip is not a repair.

Run step 5, then step 6 on VM-A. If `--check` prints a new host, that is the host this deploy writes on chain. Put the new `NF4_JOIN_URL` in VM-B's `deploy.env` after it is printed. Do not put it in VM-A's file.

After `deployer exited 0`, remove `NF4_REDEPLOY` from `deploy.env`. A later deploy with that line still set spends gas on another set of contracts.

Continue at step 7. Step 12 waits for the new poll to print `READY`. The old `READY` does not count. Do not use `docker compose down` for this wipe, and do not use these `docker rm` lines on a resume.

## When a step fails

| What you see | What to do |
|---|---|
| `deploy.env already exists. Not overwriting.` | `--init` refused to replace the file. On VM-A, do not delete it. Run `./scripts/nf4-deploy --file deploy.env --check` with no `--init`. If that check is clean and step 6 has not exited 0, open the ports and run step 6. If step 6 already exited 0, do not deploy again. |
| `wrong wallet` | The pinned address does not match the key. Fix one of them. `--check` catches this before deploy. |
| `Designated proposer is` | The proposer key is not the wallet already registered. A re-run will not rotate it. |
| `NF4_CONFIRM=apply` | The env file has no confirm line. `--check` does not need it. Deploy does. |
| `NF4_LATE_JOIN` or `Client 2 is already running` | Client 2 was started without `--late-join`, or before the deposit poll printed `READY`. `--check` on VM-B fails if that container is already up. |
| `loopback or Docker-only host` | `NF4_OPERATOR_HOST` is not an address VM-B can route to. |
| `cannot open TCP` or `Could not detect an operator host` | The probe ran before gas. Set `NF4_OPERATOR_HOST` to an address this machine can reach, or fix the route. Do not deploy against a host that times out. |
| `Certification for user-2 returned HTTP` | The deploy itself may already have finished. Do not set `NF4_REDEPLOY`. The response body names the failure. A gas or RPC error is a retry of `./scripts/nf4-deploy --file deploy.env`. Fund the proposer only if the body says the balance cannot cover certification gas. |
| `no such object: nf4_indie_deployer` | Forge was skipped, so the one-shot deployer container is gone. Certifications and both health checks already passed. Do not set `NF4_REDEPLOY`. Continue at step 7. |
| `curl: (7)` or `join HTTP 000` on VM-B | Wallet check can still be OK. Do not deploy on VM-B and do not pass `--late-join`. If step 6 has not exited 0, finish it on VM-A, then re-run the check. If VM-A's own `join.toml` curl is refused, re-run `./scripts/nf4-deploy --file deploy.env` there. If VM-A returns 200, `NF4_JOIN_URL` is the wrong host or TCP 8080 is closed. `10.0.0.8` is the guide example unless VM-A printed it. |
| `Failed to write configuration/toml/join.toml: Permission denied` | Re-run `./scripts/nf4-deploy --file deploy.env` as your user, not under `sudo`. The script reclaims a root-owned `configuration/toml` and does not set `NF4_REDEPLOY`. If it still cannot write, `sudo chown -R "$USER:$USER" configuration/toml` and run the same command again. Do not delete `addresses.toml`. |
| `On-chain proposer URL is` | The operator address changed, including after a reboot. Do not edit it in place. Resume is not supported. Fresh start with `NF4_REDEPLOY=true`. That costs gas and is a new report. |
| `DEPLOYER_SIGNING_KEY` on VM-B | `local.env` was copied from VM-A. Delete it. Keep the key on VM-A. |
| `STOP: temp/sepolia-step10 exists` | Deposits were already submitted, or a reboot tore the directory. If all four `.request-id` files exist, run `--step poll`. If one is missing, resume is not supported. Fresh start. Do not delete the directory to POST again. The same rule applies to `temp/sepolia-step11`, `step12`, `step17`, `step18-c1`, `step18-c2`, `step20`, and `step21`. |
| `tmux` session gone after a reboot | VM restart section. Do not `docker rm` if you are resuming. |
| `NF4_REDEPLOY=true` still set after a successful deploy | Remove it before the next `nf4-deploy`. Left in place, the next run deploys another set and spends gas. |
| chain-id `31337` or `84532` | Wrong network. This procedure is Ethereum Sepolia, `11155111`. |
| poll exit other than 0 or 2 | The request failed or the query failed. Read client and proposer logs. Do not deposit again. |
| `Transfers are not accepted yet` | Do not run `stop-client1`. Read `./scripts/nf4 logs proposer`. |
| `C2_ZKP missing` | Copy `temp/zkp.public` from VM-B in step 12 and pass `--from` on VM-A. |
| `curl: (3)` or an empty container name | You ran a raw curl without the variables. Use the script step, which reads `--file`. |

`./scripts/nf4 status`, `./scripts/nf4 logs proposer`, and `./scripts/nf4 logs client` still work on the VM that owns that process. The laptop wizards are still there. `wizard deploy` also sets `NF4_CONTRACTS__DEPLOY_CONTRACTS=false` after a successful deploy. `--yes` is still local-only, and it is not part of this procedure.
