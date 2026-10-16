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

Leave `NF4_OPERATOR_HOST` unset. On VM-A, `./scripts/nf4-deploy --file deploy.env --check` reads the local interfaces, skips loopback, link-local, and Docker bridge addresses, and probes TCP port 8080. Connection refused counts as reachable, because nothing is listening yet. A timeout does not. It prefers a private address (`10.`, `192.168.`, or `172.16–31` other than `172.17` and `172.28`) over a public one when both answer.

The check prints:

```text
Operator host: 10.0.0.8
Join URL: http://10.0.0.8:8080/configuration/toml/join.toml
```

On VM-B, set that exact URL in `deploy.env`. That file is created in step 4 by `./scripts/nf4-deploy --init client --file deploy.env`. Uncomment the join line and replace it with the printed URL:

```bash
NF4_JOIN_URL=http://10.0.0.8:8080/configuration/toml/join.toml
```

Do not invent the host. Do not put this line in VM-A's `deploy.env`. If VM-B cannot route to the printed host, set `NF4_OPERATOR_HOST` on VM-A to the address VM-B uses and run `--check` again. The script probes that value too, and it refuses to deploy if VM-A cannot open TCP to it.

That address is written on chain. Do not change it after deploy. If it changes, transfers break until you deploy again with `NF4_REDEPLOY=true`.

## 3. RPC is filled automatically

Leave `NF4_RPC_WSS` and `NF4_RPC_HTTP` as the example placeholders, or delete them. `--init`, `--check`, and deploy call `./scripts/nf4-pick-rpc --network sepolia` and write the chosen pair into `deploy.env`. Nightfall needs `wss://`. `cast` uses the `https://` pair. A chain-id check does not prove `eth_subscribe` will stay up. If the listener later drops, put a keyed provider in those two lines. The script will not overwrite a URL you already set. Do not redeploy just to change the RPC, and do not put the RPC URL in `join.toml` or the report.

## 4. Create deploy.env, then set the wallets you already have

Do not copy VM-A's file to VM-B. `--init` copies the example, sets mode 600, and fills the RPC if those lines are still placeholders.

```bash
# VM-A
./scripts/nf4-deploy --init operator --file deploy.env

# VM-B
./scripts/nf4-deploy --init client --file deploy.env
```

Do not commit `deploy.env`. The role, ports, `NF4_CONFIRM=apply`, certificates, and default ZKP phrases are already in the file. You edit the wallet lines, and only the overrides you actually need.

A key is `0x` plus 64 hex characters. An address is `0x` plus 40 hex characters. Anvil keys are rejected by the balance and chain checks in practice because they are not funded on Sepolia. Each account that will send a transaction needs about 0.5 Sepolia ETH.

### VM-A: one existing wallet for deployer, proposer, and Client 1

This is the normal Sepolia layout. Paste that wallet's private key. Leave the proposer and Client 1 key lines commented out. They then reuse `NF4_DEPLOYER_KEY`.

```bash
NF4_DEPLOYER_KEY=0xYOUR_EXISTING_PRIVATE_KEY
NF4_DEPLOYER_ADDRESS=0xYOUR_EXISTING_ADDRESS
NF4_CLIENT2_ADDRESS=0xADDRESS_PRINTED_BY_VM_B
```

`NF4_DEPLOYER_ADDRESS` is optional. Set it when you already know the account address, from MetaMask or an explorer. `--check` derives the address from the key and stops if they differ, before any deploy transaction. Omit the pin only if you do not yet know the address.

`NF4_CLIENT2_ADDRESS` is not a key. It is the address VM-B prints. The Client 2 private key never goes in this file.

### VM-A: existing proposer or Client 1 wallet is a different account

Uncomment and set only the role that differs. A different proposer key is registered on chain as the designated proposer. A later run will not rotate it.

```bash
NF4_DEPLOYER_KEY=0xEXISTING_DEPLOYER_KEY
NF4_DEPLOYER_ADDRESS=0xEXISTING_DEPLOYER_ADDRESS
NF4_PROPOSER_KEY=0xEXISTING_PROPOSER_KEY
NF4_PROPOSER_ADDRESS=0xEXISTING_PROPOSER_ADDRESS
NF4_CLIENT_KEY=0xEXISTING_CLIENT1_KEY
NF4_CLIENT_ADDRESS=0xEXISTING_CLIENT1_ADDRESS
NF4_CLIENT2_ADDRESS=0xADDRESS_PRINTED_BY_VM_B
```

If Client 1 is the same account as the deployer, omit `NF4_CLIENT_KEY` and `NF4_CLIENT_ADDRESS`. If the proposer is the same account as the deployer, omit `NF4_PROPOSER_KEY` and `NF4_PROPOSER_ADDRESS`.

### VM-B: existing Client 2 wallet

```bash
NF4_CLIENT_KEY=0xYOUR_EXISTING_CLIENT2_PRIVATE_KEY
NF4_CLIENT_ADDRESS=0xYOUR_EXISTING_CLIENT2_ADDRESS
```

Then:

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Copy the printed `NF4_CLIENT2_ADDRESS=0x...` line into VM-A's `deploy.env`. Do not copy `NF4_CLIENT_KEY`, and do not copy VM-B's `deploy.env`.

### No wallet yet

Only if you do not already have a funded account:

```bash
cast wallet new
```

Fund the printed address with about 0.5 Sepolia ETH, then set `NF4_DEPLOYER_KEY` or `NF4_CLIENT_KEY` to that new key and set the matching address pin to the printed address.

### Existing ZKP mnemonic

The L1 key is not the ZKP spending phrase. `--init` writes the fixed report phrases. Leave those lines unchanged for a fresh report.

If this client already has a 24-word phrase, replace only that VM's line. Do not switch phrases after the first `deriveKey` of the run.

```bash
# VM-A, Client 1
NF4_ZKP_MNEMONIC='word1 word2 ... word24'

# VM-B, Client 2
NF4_ZKP_MNEMONIC='word1 word2 ... word24'
```

### Other settings already in the file

Leave these unless you have a reason to change them:

| Line | Leave it as | Change it only when |
|---|---|---|
| `NF4_ROLE` | `operator` on VM-A, `client` on VM-B | Never swap these files between VMs |
| `NF4_NETWORK` | `testnet` | This procedure is not local or mainnet |
| `NF4_PROFILE` | `sepolia` | You are deploying a new profile name |
| `NF4_CHAIN_ID` | `11155111` | You are not on Ethereum Sepolia. `84532` is Base Sepolia and is the wrong chain here |
| `NF4_RPC_WSS`, `NF4_RPC_HTTP` | filled by `--init` or `--check` | You have a keyed provider. Set both lines yourself. A URL you set is not overwritten |
| `NF4_OPERATOR_HOST` | unset | VM-B cannot reach the host printed by `--check`. Set the address VM-B uses |
| `NF4_JOIN_URL` | unset on VM-B until VM-A prints it | Paste the printed `Join URL:` line. Uncomment the example line and replace the host |
| `NF4_CONFIGURATION_PORT` | `8080` | That port is taken |
| `NF4_PROPOSER_PORT` | `3001` | That port is taken |
| `NF4_CLIENT_PORT` | `3000` | That port is taken. On VM-B this is still 3000, not 3002 |
| `NF4_WEBHOOK_PORT` | `8081` | That port is taken |
| `NF4_MOCK_PROVER` | `false` | You are not running this report. Do not flip this after a deploy |
| `NF4_BLOCK_SIZE` | `64` | You intentionally want `256` |
| `NF4_CONFIRM` | `apply` | Do not delete it. Deploy refuses to spend gas without it |
| `NF4_LATE_JOIN` | `false` | Do not edit this. Step 13 passes `--late-join` |
| `NF4_PROPOSER_CERT` | `user-2` | You are not using the test certificate shipped in the repo |
| `NF4_CLIENT_CERT` | `user-3` on VM-A | Same |
| `NF4_CERT` | `user-4` on VM-B | Same |
| `NF4_REDEPLOY` | unset | You mean to deploy a new set of contracts. This costs gas |
| `NF4_SKIP_PROVER_CHECK` | unset | You already ran `./scripts/nf4 check prover` and want the deploy to skip it |

Keyed RPC, if the public one drops subscriptions:

```bash
NF4_RPC_WSS=wss://your-provider.example/sepolia
NF4_RPC_HTTP=https://your-provider.example/sepolia
```

Operator host override, only after `--check` printed a host VM-B cannot reach:

```bash
NF4_OPERATOR_HOST=10.1.2.3
```

VM-B join URL, after VM-A prints it:

```bash
NF4_JOIN_URL=http://10.1.2.3:8080/configuration/toml/join.toml
```

## 5. Check the wallets, both VMs

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Expect the derived addresses and a non-zero balance for every wallet that will send a transaction. On VM-A that is the deployer, Client 2's address, and Client 1 or the proposer if those keys differ. On VM-B that is Client 2. The command does not start Docker and does not require `NF4_LATE_JOIN`.

A pin mismatch looks like `NF4_DEPLOYER_ADDRESS is 0x..., but the key in this file derives 0x...`. Fix the key or the pin. Do not deploy.

Record the four L1 addresses. Do not record the keys.

## 6. Open ports, then deploy VM-A

Open TCP `8080` and `3001` from VM-B to VM-A. Client ports 3000 stay closed to the other VM.

On VM-A:

```bash
./scripts/nf4-deploy --file deploy.env
```

This stops leftover Anvil, runs the prover check on a first real-prover deploy, deploys contracts, starts configuration, the proposer, and Client 1, certifies the proposer with `user-2` and Client 1 with `user-3`, and publishes `C1_ZKP` when the mnemonic is set. Key generation is slow. Real proving keys need a large machine. VM-A is that machine.

Success is the script exiting 0. It prints the join URL, `Certified user-2`, `Certified user-3`, a 64-hex `C1_ZKP` line, configuration HTTP 200, both health bodies, and `deployer exited 0`. `NF4_CONTRACTS__DEPLOY_CONTRACTS` in `local.env` is `false` after success. Leave it false.

A second run skips Forge when that Nightfall contract already has code. It does not wipe token addresses or ZKP keys in `public.env`. It will not change the on-chain proposer URL or the designated proposer wallet.

Record Nightfall, Round Robin, X509, and Verifier from `configuration/toml/addresses.toml`, plus explorer links. Those addresses are also in `configuration/toml/public.env` after this step. `public.env` has no keys and no RPC URL.

## 7. VM-B can see the join file, and Client 2 is still down

On VM-B, after `NF4_JOIN_URL` is the printed URL, do not deploy yet.

```bash
./scripts/nf4-deploy --file deploy.env --check
```

Expect `join HTTP 200` and no error about Client 2 already running. A failed curl is the firewall or the operator host. Do not deploy on VM-B to fix it.

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

## When a step fails

| What you see | What to do |
|---|---|
| `wrong wallet` | The pinned address does not match the key. Fix one of them. `--check` catches this before deploy. |
| `Designated proposer is` | The proposer key is not the wallet already registered. A re-run will not rotate it. |
| `NF4_CONFIRM=apply` | The env file has no confirm line. `--check` does not need it. Deploy does. |
| `NF4_LATE_JOIN` or `Client 2 is already running` | Client 2 was started without `--late-join`, or before the deposit poll printed `READY`. `--check` on VM-B fails if that container is already up. |
| `loopback or Docker-only host` | `NF4_OPERATOR_HOST` is not an address VM-B can route to. |
| `cannot open TCP` or `Could not detect an operator host` | The probe ran before gas. Set `NF4_OPERATOR_HOST` to an address this machine can reach, or fix the route. Do not deploy against a host that times out. |
| `On-chain proposer URL is` | The operator IP changed. Do not edit it in place. A new deploy costs gas and is a new report. |
| `DEPLOYER_SIGNING_KEY` on VM-B | `local.env` was copied from VM-A. Delete it. Keep the key on VM-A. |
| `STOP: temp/sepolia-step10 exists` | Deposits were already submitted. Run `--step poll`. Do not delete the directory. The same rule applies to `temp/sepolia-step11`, `step12`, `step17`, `step18-c1`, `step18-c2`, `step20`, and `step21`. |
| chain-id `31337` or `84532` | Wrong network. This procedure is Ethereum Sepolia, `11155111`. |
| poll exit other than 0 or 2 | The request failed or the query failed. Read client and proposer logs. Do not deposit again. |
| `Transfers are not accepted yet` | Do not run `stop-client1`. Read `./scripts/nf4 logs proposer`. |
| `C2_ZKP missing` | Copy `temp/zkp.public` from VM-B in step 12 and pass `--from` on VM-A. |
| `curl: (3)` or an empty container name | You ran a raw curl without the variables. Use the script step, which reads `--file`. |

`./scripts/nf4 status`, `./scripts/nf4 logs proposer`, and `./scripts/nf4 logs client` still work on the VM that owns that process. The laptop wizards are still there. `wizard deploy` also sets `NF4_CONTRACTS__DEPLOY_CONTRACTS=false` after a successful deploy. `--yes` is still local-only, and it is not part of this procedure.
