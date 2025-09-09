# Nightfall Upgrade Guide

This document explains why we’re upgrading each on-chain component, how our contracts are made upgradeable, what to watch out for when authoring new versions, and a step-by-step test plan (with commands) for each contract.

## How Upgradble Contracts Work

### What “upgradable” really means
- Address stays the same, logic changes. Users interact with a proxy contract whose address never changes. The proxy delegates every call to a separate implementation (logic) contract.
- State lives in the proxy. Because calls use delegatecall, all storage (balances, mappings, configs) is read/written in the proxy’s storage, so upgrading the implementation doesn’t erase state.

### Core pieces
#### Proxy
1. Minimal contract that forwards calls via delegatecall.
2. Stores two special EIP-1967 slots:
implementation (logic address)
(sometimes) admin (who can change the implementation)
#### Implementation (logic)
1. The actual contract code (functions).
2. Use initializer functions instead of constructors (constructors don’t run through a proxy).
#### Upgrade mechanism
1. A function that changes the implementation slot, guarded by access control (e.g., onlyOwner, multisig, timelock).
2. Patterns:
- Transparent Proxy (Admin can’t call logic functions through the proxy).
- UUPS (logic contract exposes upgradeTo, proxy is dumb; recommended by OZ).
Beacon (many proxies read implementation from a shared beacon).

###  The UUPS pattern (most common today)
- Implementation inherits `UUPSUpgradeable` and defines:
1. `initialize(...)` (or `reinitializer(x)`) for setting initial state.
2. `_authorizeUpgrade(address)` to restrict upgrades (e.g., onlyOwner).
- Upgrade flow:
1. Deploy new implementation.
2. Call `upgradeTo(newImpl)` via proxy (authorized caller only).
3. (Optional) `upgradeToAndCall(newImpl, data)` to run a post-upgrade init/migration.

### Storage layout rules (most important!)
- Do not reorder, remove, or change types of existing storage variables.
- Only append new variables at the end of the storage layout.
- Reserve a storage gap: `uint256[50] private __gap;` to allow future vars without colliding inherited layouts.
- If you must migrate data, do it in a one-time reinitializer and keep it simple and auditable.

###  Initializers, not constructors
Constructors run only at the logic contract’s own address, not through the proxy.
Use:
- `function initialize(args...) public initializer { ... }`
- `function _authorizeUpgrade(address) internal override onlyOwner {}`
For new versions: `reinitializer(2)` to set up new state safely once.

###  Security & governance best practices
- Restrict upgrades. Use a multisig and ideally a timelock for production.
- Run storage layout checks (OpenZeppelin/Foundry plugins) during CI.
- No self destruct, no untrusted delegatecall, and avoid `msg.sender` assumptions (it’s the external caller, not the proxy).
- Immutable variables are fixed at the logic contract address; prefer storage variables if you expect changes.
- Consider Pause (Pausable) for emergency response.

###  Testing & operations checklist
- Unit tests: old state remains valid, new behavior works, access control enforced, events unchanged unless intentional.
- Upgrade tests: deploy V1 → initialize → write state → upgrade to V2 → verify state unchanged & behavior changed.
- On-chain verification:
1. Read EIP-1967 implementation slot to confirm update.
2. cast code `<proxy>` must be non-zero.
3. Rollbacks: keep previous implementation artifact/hash handy; upgrade back if needed.

###  Common pitfalls
1. Using constructors for state (won’t run through proxy).
2. Storage collisions from reordering variables.
3. Forgetting `_authorizeUpgrade`.
4. external API (function selector changes) that clients depend on.
5. Event schema changes without updating indexers/consumers.

### When not to use upgradability

- If immutability is a hard requirement (governance, compliance, or trust concerns).

- Extremely simple contracts where a migration is cheaper/safer.

- When upgrade risk outweighs benefits—then deploy immutable and versioned contracts instead.

In short: Upgradable contracts keep a stable address + state via a proxy and rely on strict storage layout discipline, initializer patterns, and governed upgrade rights. Do that well, and you can evolve features safely without breaking users or losing data.

## What Contracts in NF_4 need to be updated and Why

### Why these contracts are being updated

#### X509-Allowlist.sol

- Tighten certificate allowlisting semantics, remove brittle behavior, and make the allowlist admin-driven.

- Enables safer rollouts of EKU/policy/OID changes without redeploying the whole system.

#### X509/X509.sol

- Validate X.509 chains on-chain with clearer failure modes.

- Add upgrade hooks so APIs that call validateCertificate can change behavior (e.g., stricter parsing) without breaking storage.

#### X509/Certified.sol

- Centralized “certified caller” gate for sensitive entrypoints.

- We retain auth state across upgrades while allowing new checks (e.g., sanctions updates).

#### Nightfall.sol

- Business-logic iteration (e.g., changing fee accounting or events) while preserving user balances, roots, and historical state.

#### RoundRobin.sol

- Adjust proposer economics (e.g., stake from 5 -> 50 wei) without migrations.

#### proof_verification/RollupProofVerificationKey.sol

- Swap verification keys (VK) as circuits evolve.

#### proof_verification/RollupProofVerifier.sol

- Hot-patch the verifier logic (e.g., change sub-checks like verify_acc) without touching L2 data structures.

### How upgrades work (and who can upgrade)

- We use UUPS proxies (OpenZeppelin’s UUPSUpgradeable + Initializable):

- Proxy address is permanent; the implementation (logic) address changes on upgrade.

- State lives in the proxy, so user funds, Merkle roots, allowlists, etc., survive upgrades.

- Each implementation defines:

1. `initialize(...)` (runs once via proxy; replaces constructors),

2. `_authorizeUpgrade(address newImpl)` (we gate this with onlyOwner).

3. Only the proxy owner (the address behind NF4_SIGNING_KEY) can upgrade.

4. We use openzeppelin-foundry-upgrades’ Upgrades.upgradeProxy(...), which runs storage-layout safety checks at build time.

5. Proxies for the local dev environment are read from addresses.toml (via the configuration service). Always verify the proxy really exists on your RPC_URL before running an upgrade.

## Authoring a new version: do’s and don’ts
### Do

- Preserve the storage layout.

- Never reorder existing state variables.

- Only append new variables at the end.

- Keep `uint256[50] private __gap;` (or adjust by reducing the gap, at the end only).

- Keep initializer patterns.

- No constructors with logic; use initialize.

- If adding new init, use a new `reinitializer(version)` gate.

- Keep `_authorizeUpgrade` restricted (e.g., `onlyOwner`).

- Maintain external/public function selectors used by off-chain code (APIs/clients).

- Emit clear events when changing behavior; update tests/docs accordingly.

- Run storage checks and unit/integration tests before broadcasting.

###  Don’t

-  Don’t Delete or rename existing storage vars, even if “unused.”

-  Don’t Change types (e.g., uint256 → uint128) or visibility in ways that re-layout storage.

-  Don’t Introduce self-destruct / delegatecall to unknown addresses.

-  Don’t Depend on constructors for state; they won’t run on the proxy.

-  Don’t Break invariants around balances, roots, or auth without explicit migration logic.

## Common upgrade workflow (local dev)
In this section, we will give example about how to upgrade each contract and how to test.

### X509 upgrade (V3): forces revert for validateCertificate)
In X509V2.sol, we make `validateCertificate` revert no matter it's a valid or invalid certificate.

We set `x509_owner = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720"  # Anvil account (9)` (same for other examples.)

We first clean the test in `nightfall_test/src/run_tests.rs` (comment the code from line 66 to line 992), so we will start with clean certificate status.

Run 
`docker compose --profile development build`
and 
`docker compose --profile development up`

when you see `nf4_test exited with code 0`, use Postman to check client1 health -> expect healthy.

Now you can use Postman to start a X509 api call with valid certificate, you should get 
```
{
    "status", "ok",
    "certified": true
}
```
,

and then try an invalid certificate (for example, you can use same certificate but with the private key from another certificate), you should get

```
{
    "status", "ok",
    "certified": false
}
```

So far, we verified that current `blockchain_assets/contracts/X509/X509.sol` works in the way we expect.

Don't stop your docker.
But add these contracts in:
`blockchain_assets/contracts/X509/X509V3.sol`,
`blockchain_assets/script/UpgradeX509V3.s.sol`.

The first contract defines the new changes we want for X509 contract, and the second contract defines how we deploy and update the contract from `blockchain_assets/contracts/X509/X509.sol` to `blockchain_assets/contracts/X509/X509V3.sol`

After copy paste the contracts in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifactss
docker compose --profile development build deployer

// One-off shell in the fresh image
docker compose --profile development run --no-deps --rm deployer bash

// set up environment for restarted deployer
export NF4_RUN_MODE=local
// 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6 is the private key of anvil account 9, which is the owner of x509, aka, only account 9 can update x509 contract.
export NF4_SIGNING_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
export X509_PROXY=$(curl -s http://configuration/configuration/toml/addresses.toml | awk -F\" '/^x509/{print $2}')
export FOUNDRY_OUT=blockchain_assets/artifacts

// verify if you set X509_PROXY correctly, this should be in your terminal log during the first time deployment
echo "X509_PROXY=$X509_PROXY"

// Sanity: proxies have code at this RPC
cast code "$X509_PROXY"      --rpc-url "$RPC_URL"  # must not be 0x

// Just double check if we put the new things 
ls -l blockchain_assets/script/UpgradeX509V3.s.sol
ls -l blockchain_assets/contracts/X509/X509V3.sol

// Build the new changes
forge build --force

// deploy new updated contract
forge script blockchain_assets/script/UpgradeX509.s.sol:UpgradeX509WithLogging
 \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

// confirm impl actually changed, expect
cast storage $X509_PROXY \
  0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC \
  --rpc-url $RPC_URL

// confirm owner is the key you're using
cast call $X509_PROXY "owner()(address)" --rpc-url $RPC_URL
```
Now you can use Postman to start a X509 api call with valid certificate, you should get 
```
{
    "status", "ok",
    "certified": false
}
```
,

and then try an invalid certificate (for example, you can use same certificate but with the private key from another certificate), you should get

```
{
    "status", "ok",
    "certified": false
}
```
and in terminal you should see "X509V2: forced invalid certificate", this is what we expected about new behaviours of X509V2.

JJ: I want someone to test this case, because we only allow one address to have one x509 certificate. Before upgrading, test if you can let client 1 and client 2 use same certificate and same private key, you should get error. Then let client 1 do a successful certificate validation. After upgrading (this should go without saying, you have to do another upgrading contract, otherwise you will fail any validation if you use my example), after verifying your upgrading behavour, let client 2 validate same certificate client 1 used, you should get error, so we ensure upgrading contract wont clean old states.

###  Nightfall upgrade (V3): emit only one deposit event

Intent: Change deposit event semantics without touching balances/roots.

Change: In V3, escrow_funds(...) emits a single DepositEscrowed (no extra “fee” deposit event).

We first clean the test in nightfall_test/src/run_tests.rs (comment the code from line 99 to line 992), so we will start with clean status just after validating everyone's certificate.

Run `docker compose --profile development build` and `docker compose --profile development up`

when you see `nf4_test exited with code 0`, use Postman to check client_1 health -> expect healthy.

Now you can use Postman to start a deposit api call with value 9, deposit_fee 2.
in the terminal, you will find we have 2 events emitted. 
```
nf4_client         | [2025-09-09T18:24:29Z INFO  nightfall_client::driven::event_handlers::nightfall_event] Client: Decoded DepositEscrowed event from transaction 0x371b…0151, Deposit Transaction with nf_slot_id 3904706575986504511978814088454415652555375037070224197537435603550245942968, value 9, is now on-chain

nf4_client         | [2025-09-09T18:24:29Z INFO  nightfall_client::driven::event_handlers::nightfall_event] Client: Decoded DepositEscrowed event from transaction 0x371b…0151, Deposit Transaction with nf_slot_id 2306013320856568135054112791043024517655732189858420414288332704154946685855, value 2, is now on-chain
```

Use check commitment api now before the L2 block is onchain, you should get
```
[
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000009",
            "nf_token_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "nf_slot_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "2c576968f7fd8121b6eb85e05572cca2c52978a8105d0e4157349db6dc0ed839",
                    "preimage_two": "16b2f4b42cc9c348040cc86a89ac125fc5f521473a918d047eac3a80f2021c50",
                    "preimage_three": "086c8c1b3afd5f9cc724dbe1a8c3724f1131f4291430ee2de0e3c1d7f5c448d1"
                }
            }
        },
        "status": "PendingCreation",
        "_id": "13e554611058c5dbb17953ba508e7e8972fdd802b42bc723655d76e3091710c4",
        "nullifier": "193ca07b29fff2e0cb1532204fd6959956fcb912398882ea9f55c4937bddf867",
        "layer_1_transaction_hash": null,
        "layer_2_block_number": null
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000002",
            "nf_token_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "nf_slot_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "2c576968f7fd8121b6eb85e05572cca2c52978a8105d0e4157349db6dc0ed839",
                    "preimage_two": "16b2f4b42cc9c348040cc86a89ac125fc5f521473a918d047eac3a80f2021c50",
                    "preimage_three": "086c8c1b3afd5f9cc724dbe1a8c3724f1131f4291430ee2de0e3c1d7f5c448d1"
                }
            }
        },
        "status": "PendingCreation",
        "_id": "2baeb66c40a812636f9f8db5bcb64a7fb8c09d19e92621e8706865e3b06af63d",
        "nullifier": "2f204f7062c9599bae8a7e190f9ec045fba145fc863e6ee427ea17e607afbe07",
        "layer_1_transaction_hash": null,
        "layer_2_block_number": null
    }
]
```

Wait until this deposit is onchain, you check commitment again and you can get:
```
[
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000009",
            "nf_token_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "nf_slot_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "2c576968f7fd8121b6eb85e05572cca2c52978a8105d0e4157349db6dc0ed839",
                    "preimage_two": "16b2f4b42cc9c348040cc86a89ac125fc5f521473a918d047eac3a80f2021c50",
                    "preimage_three": "086c8c1b3afd5f9cc724dbe1a8c3724f1131f4291430ee2de0e3c1d7f5c448d1"
                }
            }
        },
        "status": "Unspent",
        "_id": "13e554611058c5dbb17953ba508e7e8972fdd802b42bc723655d76e3091710c4",
        "nullifier": "193ca07b29fff2e0cb1532204fd6959956fcb912398882ea9f55c4937bddf867",
        "layer_1_transaction_hash": "0xd2582761e1880015d6a01b1f140ea76222cc5b7c53e41a25a95e46b5bc7c3dda",
        "layer_2_block_number": "0x0"
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000002",
            "nf_token_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "nf_slot_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "2c576968f7fd8121b6eb85e05572cca2c52978a8105d0e4157349db6dc0ed839",
                    "preimage_two": "16b2f4b42cc9c348040cc86a89ac125fc5f521473a918d047eac3a80f2021c50",
                    "preimage_three": "086c8c1b3afd5f9cc724dbe1a8c3724f1131f4291430ee2de0e3c1d7f5c448d1"
                }
            }
        },
        "status": "Unspent",
        "_id": "2baeb66c40a812636f9f8db5bcb64a7fb8c09d19e92621e8706865e3b06af63d",
        "nullifier": "2f204f7062c9599bae8a7e190f9ec045fba145fc863e6ee427ea17e607afbe07",
        "layer_1_transaction_hash": "0xd2582761e1880015d6a01b1f140ea76222cc5b7c53e41a25a95e46b5bc7c3dda",
        "layer_2_block_number": "0x0"
    }
]

```
So far, we verified that current `blockchain_assets/contracts/Nightfall.sol` works in the way we expect.

Don't stop your docker. But add these contracts in: `blockchain_assets/contracts/NightfallV3.sol`, `blockchain_assets/script/UpgradeNightfallV3.s.sol`.

The first contract defines the new changes we want for Nightfall contract, and the second contract defines how we deploy and update the contract from `blockchain_assets/contracts/Nightfall.sol` to `blockchain_assets/contracts/NightfallV3.sol`

After copy paste the contracts in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifactss
docker compose --profile development build deployer

// One-off shell in the fresh image
docker compose --profile development run --no-deps --rm deployer bash

// set up environment for restarted deployer
export NF4_RUN_MODE=local
// 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6 is the private key of anvil account 9, which is the owner of x509, aka, only account 9 can update x509 contract.
export NF4_SIGNING_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
export NIGHTFALL_PROXY=$(curl -s http://configuration/configuration/toml/addresses.toml | awk -F\" '/^nightfall/{print $2}')
export FOUNDRY_OUT=blockchain_assets/artifacts

// verify if you set NIGHTFALL_PROXY correctly, this should be in your terminal log during the first time deployment
echo "NIGHTFALL_PROXY=$NIGHTFALL_PROXY"

export RPC_URL=http://anvil:8545

// Just double check if we put the new things 
ls -l blockchain_assets/contracts/NightfallV3.sol
ls -l blockchain_assets/script/UpgradeNightfallV3.s.sol

// Build the new changes
forge build --force

// deploy new updated contract
forge script blockchain_assets/script/UpgradeNightfallV3.s.sol:UpgradeNightfallWithLogging \
  --sig "run(address)" "$NIGHTFALL_PROXY" \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

// confirm impl actually changed, expect
cast storage $NIGHTFALL_PROXY \
  0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC \
  --rpc-url $RPC_URL

// confirm owner is the key you're using
cast call $NIGHTFALL_PROXY "owner()(address)" --rpc-url $RPC_URL
```

Now you can use Postman to start a deposit api call with value 3, deposit_fee 4, in the terminal you should only see one event:
``

{
    "status", "ok",
    "certified": false
}
,

and then try an invalid certificate (for example, you can use same certificate but with the private key from another certificate), you should get

{
    "status", "ok",
    "certified": false
}
and in terminal you should see "X509V2: forced invalid certificate", this is what we expected about new behaviours of X509V2.
Steps
 (inside deployer container)
export NIGHTFALL_PROXY=$(curl -s http://configuration/configuration/toml/addresses.toml | awk -F\" '/^nightfall/{print $2}')
echo "NIGHTFALL_PROXY=$NIGHTFALL_PROXY"

ls -l blockchain_assets/script/UpgradeNightfallV3.s.sol
ls -l blockchain_assets/contracts/NightfallV3.sol

forge build --force
```

 Option A: default run() reads NIGHTFALL_PROXY from env
forge script blockchain_assets/script/UpgradeNightfallV3.s.sol:UpgradeNightfallWithLogging \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

 Option B: pass the proxy explicitly (avoids env mismatch)
forge script blockchain_assets/script/UpgradeNightfallV3.s.sol:UpgradeNightfallWithLogging \
  --sig "run(address)" "$NIGHTFALL_PROXY" \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

 (Optional) confirm impl slot changed
cast storage "$NIGHTFALL_PROXY" \
  0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC \
  --rpc-url "$RPC_URL"

Expected behavior (after upgrade)

Deposits still succeed and mark escrow as before.

Only one DepositEscrowed event is emitted per deposit (no extra “fee” event).

Roots/balances/managers unchanged.

(If you included a versionMarker() in V3, it should return the new string.)

3) RoundRobin upgrade: stake from 5 wei → 50 wei

Intent: Tighten proposer stake requirement without migration.

Steps

Upgrade with your UpgradeRoundRobin.s.sol (pattern identical to above).

Test with Postman or cast:

 Before: 5 wei succeeds
 After: 5 wei should revert; 50 wei should succeed

 Example (names will vary):
cast send $ROUNDROBIN_PROXY "registerProposer()" \
  --value 5 --rpc-url "$RPC_URL" --private-key "$NF4_SIGNING_KEY"   # expect revert AFTER upgrade

cast send $ROUNDROBIN_PROXY "registerProposer()" \
  --value 50 --rpc-url "$RPC_URL" --private-key "$NF4_SIGNING_KEY"  # expect success AFTER upgrade

Expected behavior

Attempts to register with 5 wei now revert.

50 wei succeeds.

Existing registered proposers remain intact.

4) RollupProofVerificationKey upgrade: install a “bad” VK

Intent: Simulate verification failures at the VK level.

Steps

Point the VK provider to a deliberately incorrect key (as in your TestVKProvider implementation).

Upgrade the VK contract or the provider address reference used by the verifier (depending on your wiring).

Try to propose a block.

Expected behavior

Nightfall.propose_block(...) reverts with “Rollup proof verification failed” when the proposer submits a block.

5) RollupProofVerifier upgrade: make verify_acc revert

Intent: Force verification failure in the verifier logic itself.

Steps

Implement V2 of the verifier (or Vx) that reverts inside verify_acc (or returns false as needed).

Upgrade the RollupProofVerifier proxy.

Proposer attempts to submit a block.

Expected behavior

Block proposal fails on-chain with a deterministic revert (or false return) and Nightfall bubbles “Rollup proof verification failed.”

Quick reference table
Contract	What changed	Expected after upgrade
X509	validateCertificate forced revert in V2	Calls revert; storage intact
Nightfall	Only one deposit event	Single DepositEscrowed per deposit
RoundRobin	Stake 5 → 50 wei	5 wei reverts; 50 wei ok
VK	Bad VK	Block proposal fails verification
Verifier	verify_acc reverts	Block proposal fails verification
Verification & sanity commands
 Impl & admin slots (EIP-1967)
cast storage "$PROXY" 0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC --rpc-url "$RPC_URL"
cast storage "$PROXY" 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103 --rpc-url "$RPC_URL"

 Proxy has code?
cast code "$PROXY" --rpc-url "$RPC_URL"

 Owner check
cast call "$PROXY" "owner()(address)" --rpc-url "$RPC_URL"

Cleanup (optional, when the deployer image needs fresh artifacts)
forge clean
rm -rf blockchain_assets/artifacts/build-info/*
forge build --force

Rollback plan (recommended)

Capture implBefore from the upgrade script logs.

To roll back: deploy (or reuse) the previous implementation and run Upgrades.upgradeProxy(proxy, PREVIOUS_ARTIFACT, "").

Never write EIP-1967 slots manually on production—only as a last resort in local tests.

Notes & tips

If forge script ... with no --sig "run(address)" reverts with “proxy has no code”, your NIGHTFALL_PROXY / X509_PROXY env var is stale for the current RPC_URL. Pass the proxy explicitly or refresh env vars and verify with cast code.

Keep revert messages stable where clients depend on them.

When changing events, update log parsers and API consumers accordingly.