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

- Upgrade tests: deploy V1 -> initialize -> write state -> upgrade to V2 -> verify state unchanged & behavior changed.

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

4. We use openzeppelin-foundry-upgrades’ `Upgrades.upgradeProxy(...)`, which runs storage-layout safety checks at build time.

5. Proxies for the local dev environment are read from `addresses.toml` (via the configuration service). Always verify the proxy really exists on your RPC_URL before running an upgrade.

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

- Add contract unit test.

- Emit clear events when changing behavior; update tests/docs accordingly.

- Run storage checks and unit/integration tests before broadcasting.

###  Don’t

-  Don’t delete or rename existing storage vars, even if “unused.”

-  Don’t change types (e.g., uint256 → uint128) or visibility in ways that re-layout storage.

-  Don’t introduce self-destruct / delegatecall to unknown addresses.

-  Don’t depend on constructors for state; they won’t run on the proxy.

-  Don’t break invariants around balances, roots, or auth without explicit migration logic.

## The new changes added 
- Make original contracts upgradable.
  1. `blockchain_assets/contracts/proof_verification/RollupProofVerificationKey.sol`

  2. `blockchain_assets/contracts/proof_verification/RollupProofVerifier.sol`

  3. `blockchain_assets/contracts/X509/Allowlist.sol`

  4. `blockchain_assets/contracts/X509/Certified.sol`

  5. `blockchain_assets/contracts/X509/X509.sol`

  6. `blockchain_assets/contracts/Nightfall.sol`

  7. `blockchain_assets/contracts/RoundRobin.sol`

- Add V2 updates, and their related contract unit test. 

This is to make sure contract changes in last step are able to be updated.
And also give the contract unit test, so every update should follow this pattern, do write contract unit test.
  1. `blockchain_assets/contracts/proof_verification/RollupProofVerifierV2.sol`
     `blockchain_assets/test_contracts/RollupProofVerifier_V2.t.sol`

  2. `blockchain_assets/contracts/Nightfall_V2.sol`
     `blockchain_assets/test_contracts/Nightfall_V2_t.sol`

  3. `blockchain_assets/contracts/RoundRobinV2.sol`
     `blockchain_assets/test_contracts/RoundRobin_V2.t.sol`

  4. `blockchain_assets/test_contracts/RollupProofVerificationKey_V2.t.sol`

  5. `blockchain_assets/contracts/X509/X509V2.sol`
     `blockchain_assets/test_contracts/X509/X509_V2.t.sol`


- Add V3 updates, and their related depoyer scripts.

These are used in the tests, which shows updates are correctly done with its related script to deploy new updates onchain. Scripts can be resused in future if we need to update any contract here, only need to change the name of new contracts. Note that every new version of contract need to claim which contract it's updating, for example:

```
/// @custom:oz-upgrades-from blockchain_assets/contracts/proof_verification/RollupProofVerifier.sol:RollupProofVerifier
```

  1. `blockchain_assets/contracts/X509/X509V3.sol`
     `blockchain_assets/script/upgrade_X509_V3.s.sol`

  2. `blockchain_assets/contracts/Nightfall_V3.sol`
     `blockchain_assets/script/upgrade_Nightfall_V3.s.sol`

  3. `blockchain_assets/contracts/RoundRobinV3.sol`
     `blockchain_assets/script/upgrade_RoundRobin_V3.s.sol`

  4. `blockchain_assets/script/replace_vk_from_toml.s.sol`
     `blockchain_assets/nightfall_replace_vk.toml`
      

  5. `blockchain_assets/contracts/proof_verification/RollupProofVerifierV2.sol`
     `blockchain_assets/script/upgrade_RollupProofVerifier_V2.s.sol`

## Upgrade instruction

Upgrades via UUPS replace the entire implementation behind the proxy. That means you can change function bodies in the new implementation even if the old ones weren’t virtual. The virtual/override pair only matters if your V2 contract inherits from V1 and overrides methods via Solidity inheritance. In V3 examples, you will find we have three types of new updation:

1. replace the whole contract `blockchain_assets/contracts/X509/X509V3.sol`

2. overide one function in original contract: `blockchain_assets/contracts/Nightfall_V3.sol`

3. add a new function/change parameter: `blockchain_assets/contracts/RoundRobinV3.sol`    

### Two valid upgrade patterns

#### Replace implementation (no inheritance)

- Write new version that doesn’t inherit from original contract.

- Keep the exact same storage layout (append-only changes; don’t reorder/rename existing vars; use the gap).

- Keep function selectors (same signatures) you still want to support.

- You can freely change implementations.
- In this pattern, virtual is irrelevant.

#### Extend via inheritance (V2 is V1)

- For example, `contract NightfallV2 is Nightfall { ... }`

- Override behavior with override.
- Any function you plan to override in this way must be virtual in V1.

- What to mark virtual?

1. Don’t blanket virtual everything. Mark the likely “extension points” so V2 can override them while keeping the ABI stable:

2. External entry points whose internal logic might evolve should be virtual

3. Keep simple setters non-virtual unless you really expect to gate/alter them.

### Other upgrade must-knows

- Storage: append-only; preserve ordering/types; keep uint256[50] __gap; for future vars.

- Initializers: don’t re-run initialize. If V2 needs setup, add reinitializer(2) function and call it during/after upgrade.

- Signatures: overriding requires the exact same signature. If you need a new signature, add a new function and keep the old one as a wrapper/stub.

- UUPS guard: _authorizeUpgrade can stay as is; override only if you’re changing who can upgrade.

### Will upgrade clear the old status remembered onchain?

Replacing the implementation in a UUPS (or Transparent) proxy does not clear on-chain state. All our variables—mappings like `feeBinding`, `withdrawalIncluded`, `tokenIdMapping`, `roots`, `owner`, etc.—live in the proxy’s storage, and upgrades only swap the logic contract address the proxy delegates to.

Where people get “state reset” surprises is in these cases:

1. Incompatible storage layout.

- Reordered/removed variables, changed types, or changed inheritance order -> variables read/write the wrong slots and look “zeroed” or corrupted.

- Rule: append-only to storage; keep ordering; use the __gap.

2. Accidentally deploying a new proxy instead of upgrading the existing one.

You’ll be looking at a fresh address with empty storage.

3. Re-running an initializer (or calling a bad “setup” function) that overwrites fields.

Don’t call initialize again (OpenZeppelin guards this). If V2 needs setup, add reinitializer(2) and only set new vars there.

4. Changing which slot a getter reads (e.g., refactoring to a different struct/parent) so values appear different even though the slot still holds the old data.

So: a replace-implementation upgrade preserves all “remembered status” unless you intentionally (or accidentally) change it via the pitfalls above.


## Common upgrade workflow (local dev)
In this section, we will give examples about how to upgrade each contract and how to test. If you run this on server, only change is that you need to give the proxy address manully, the docker internal https doesnt work on server.

### X509 upgrade (V3): forces revert for `validateCertificate`
- Intend: Upgrade X509 to revert for any certificate validation
- Before: True validation result for valid certificate, false validation result for invalid certificate.
- After: False validation result for valid certificate, false validation result for invalid certificate.
- Change: Make `validateCertificate` revert no matter it's a valid or invalid certificate.
We set `x509_owner = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720"  # Anvil account (9)` (same for other examples.)

We first clean the test in `nightfall_test/src/run_tests.rs` (comment the code from line 66 to line 992), so we will start with clean certificate status.

Run 
`docker compose --profile development build`
and 
`docker compose --profile development up`

when you see `nf4_test exited with code 0`, use Postman to check client1 health -> expect healthy, or you can also use curl, see `doc/nf_4.md` for all api definition.

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
`blockchain_assets/script/UpgradeX509V3.s.sol`. This step is to mimic the real case, in real life these files wont stay in your first deployment. But we have already put the files there.

The first contract defines the new changes we want for X509 contract, and the second contract defines how we deploy and update the contract from `blockchain_assets/contracts/X509/X509.sol` to `blockchain_assets/contracts/X509/X509V3.sol`

After copy paste the contracts in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifacts
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

export RPC_URL=http://anvil:8545

// Sanity: proxies have code at this RPC
cast code "$X509_PROXY"      --rpc-url "$RPC_URL"  # must not be 0x

// Just double check if we put the new things 
ls -l blockchain_assets/script/upgrade_X509_V3.s.sol
ls -l blockchain_assets/contracts/X509/X509V3.sol

// Build the new changes
forge build --force

// deploy new updated contract

forge script blockchain_assets/script/upgrade_X509_V3.s.sol:UpgradeX509WithLogging \
  --sig "run(address)" "$X509_PROXY" \
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
and in terminal you should see "X509V3: forced invalid certificate", this is what we expected about new behaviours of X509V3.


###  Nightfall upgrade (V3): emit only one deposit event
- Intend: Change deposit event semantics without touching balances/roots.
- Before: When an user sends a `deposit` tx with both `deposit_fee` and `value`, we will see two escrow events and two commitments will be included onchain.
- After: When an user sends a `deposit` tx with both `deposit_fee` and `value`, we will see 1 value escrow events and 1 value commitment will be included onchain.
- Change: `escrow_funds(...)` emits a single DepositEscrowed (no extra “fee” deposit event).

We first clean the test in `nightfall_test/src/run_tests.rs` (comment the code from line 99 to line 992), so we will start with clean status just after validating everyone's certificate.

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

The key part you need to check is that you have two commitments with value 9 and value 2, status is `PendingCreation`.

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

The key part you need to check is that you have two commitments with value 9 and value 2, status is `Unspent`.
So far, we verified that current `blockchain_assets/contracts/Nightfall.sol` works in the way we expect.

Notice that, at the time of writing this document, we restart the test over and over so information such as salt might not be consistant in the example, but it should not affect how we conduct the tests.

Don't stop your docker. But add these contracts in: `blockchain_assets/contracts/Nightfall_V3.sol`, `blockchain_assets/script/upgrade_Nightfall_V3.s.sol`.

The first contract defines the new changes we want for Nightfall contract, and the second contract defines how we deploy and update the contract from `blockchain_assets/contracts/Nightfall.sol` to `blockchain_assets/contracts/Nightfall_V3.sol`

After copy paste the contracts in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifacts
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
ls -l blockchain_assets/contracts/Nightfall_V3.sol
ls -l blockchain_assets/script/upgrade_Nightfall_V3.s.sol

// Build the new changes
forge build --force

// deploy new updated contract
forge script blockchain_assets/script/upgrade_Nightfall_V3.s.sol:UpgradeNightfallWithLogging \
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


Now you can use Postman to start a deposit api call with value 7, deposit_fee 8, in the terminal you should only see one event:
```
nf4_client                                | [2025-09-10T11:16:17Z INFO  nightfall_client::driven::event_handlers::nightfall_event] Client: Decoded DepositEscrowed event from transaction 0xfa52…0b33, Deposit Transaction with nf_slot_id 3904706575986504511978814088454415652555375037070224197537435603550245942968, value 7, is now on-chain

nf4_client2                               | [2025-09-10T11:16:17Z INFO  nightfall_client::driven::event_handlers::nightfall_event] Client: Decoded DepositEscrowed event from transaction 0xfa52…0b33, Deposit Transaction with nf_slot_id 3904706575986504511978814088454415652555375037070224197537435603550245942968, value 7, is now on-chain
```

and then check commitment before L2 block onchain, you will see two commitments with status pending added, this is correct because client 1 will change its commitment db when it's doing deposit.

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
                    "preimage_one": "2acf9679f6e01a879cccce600e814a59914cc9b273dac3463388403c3910b7d7",
                    "preimage_two": "1faca2f62815104fca3e44b311c652b39cd8cfc190258e4930bf844b8582a85b",
                    "preimage_three": "0c50514a6cf86ab4f786e6851a19e230f45a04e1889c22d1e81163442d5fcce0"
                }
            }
        },
        "status": "Unspent",
        "_id": "11d7b11fcc310fd2dd85adb3c1dde9a489dc542b24c4fae79133b2df60fca787",
        "nullifier": "2505fc3ea383cb35a50635eaa54a1daf2559203a151bc332863825ca9d31e7f3",
        "layer_1_transaction_hash": "0xf66ac23f6db82f78d01be32261f4b60bc1f19cbc24cd98d996aa69a9b2b94594",
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
                    "preimage_one": "2acf9679f6e01a879cccce600e814a59914cc9b273dac3463388403c3910b7d7",
                    "preimage_two": "1faca2f62815104fca3e44b311c652b39cd8cfc190258e4930bf844b8582a85b",
                    "preimage_three": "0c50514a6cf86ab4f786e6851a19e230f45a04e1889c22d1e81163442d5fcce0"
                }
            }
        },
        "status": "Unspent",
        "_id": "302a90db36dfbce7b42fb146beb7af12ec17ccfde1046597e6099dc240fab34e",
        "nullifier": "2d565f98f8742a8918c8fa1954823f4ba5fe720abe4c5be463f4d72e3414660c",
        "layer_1_transaction_hash": "0xf66ac23f6db82f78d01be32261f4b60bc1f19cbc24cd98d996aa69a9b2b94594",
        "layer_2_block_number": "0x0"
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000007",
            "nf_token_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "nf_slot_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "1c4fda09ded62a1f8f992a381ffd4a57fd12e61470e505bb1e196a84746c0593",
                    "preimage_two": "20a69a73140f10d24d9b5e7f568c13c78e4ab9e256304cd60492a05bac7f3c56",
                    "preimage_three": "0f4b4a0f04face3460592fbe9b6ab324052b5554c4c225537249ce8de8a8a108"
                }
            }
        },
        "status": "PendingCreation",
        "_id": "0c7e794554915dc7f1d3311a32524b718343587a7a6b721cde17af46ee830e86",
        "nullifier": "2ef29334fe80e43b1391d78f6af2c732f3c874c48f78fc6feb34da0e65d6dce4",
        "layer_1_transaction_hash": null,
        "layer_2_block_number": null
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000008",
            "nf_token_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "nf_slot_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "1c4fda09ded62a1f8f992a381ffd4a57fd12e61470e505bb1e196a84746c0593",
                    "preimage_two": "20a69a73140f10d24d9b5e7f568c13c78e4ab9e256304cd60492a05bac7f3c56",
                    "preimage_three": "0f4b4a0f04face3460592fbe9b6ab324052b5554c4c225537249ce8de8a8a108"
                }
            }
        },
        "status": "PendingCreation",
        "_id": "1619fb9f40e6e0086670d5d760c49a4cdedad3db7b24ca97cfd58ba94ea0fe95",
        "nullifier": "2919c48d40545d097dbdfb642a7cea403c4b1be6320cf309b26437b64a18232c",
        "layer_1_transaction_hash": null,
        "layer_2_block_number": null
    }
]

// when proposer is assembling block, it only have 1 deposit
nf4_proposer                              | [2025-09-10T11:17:34Z INFO  nightfall_proposer::services::assemble_block] This block has 1 deposit(s), 0 transfer(s), and 0 withdrawal(s)

// after this L2 block is onchain, you will see only commitment with value 7's status is changed to unspent, commitment with value 8 is still pending.
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
                    "preimage_one": "2acf9679f6e01a879cccce600e814a59914cc9b273dac3463388403c3910b7d7",
                    "preimage_two": "1faca2f62815104fca3e44b311c652b39cd8cfc190258e4930bf844b8582a85b",
                    "preimage_three": "0c50514a6cf86ab4f786e6851a19e230f45a04e1889c22d1e81163442d5fcce0"
                }
            }
        },
        "status": "Unspent",
        "_id": "11d7b11fcc310fd2dd85adb3c1dde9a489dc542b24c4fae79133b2df60fca787",
        "nullifier": "2505fc3ea383cb35a50635eaa54a1daf2559203a151bc332863825ca9d31e7f3",
        "layer_1_transaction_hash": "0xf66ac23f6db82f78d01be32261f4b60bc1f19cbc24cd98d996aa69a9b2b94594",
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
                    "preimage_one": "2acf9679f6e01a879cccce600e814a59914cc9b273dac3463388403c3910b7d7",
                    "preimage_two": "1faca2f62815104fca3e44b311c652b39cd8cfc190258e4930bf844b8582a85b",
                    "preimage_three": "0c50514a6cf86ab4f786e6851a19e230f45a04e1889c22d1e81163442d5fcce0"
                }
            }
        },
        "status": "Unspent",
        "_id": "302a90db36dfbce7b42fb146beb7af12ec17ccfde1046597e6099dc240fab34e",
        "nullifier": "2d565f98f8742a8918c8fa1954823f4ba5fe720abe4c5be463f4d72e3414660c",
        "layer_1_transaction_hash": "0xf66ac23f6db82f78d01be32261f4b60bc1f19cbc24cd98d996aa69a9b2b94594",
        "layer_2_block_number": "0x0"
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000007",
            "nf_token_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "nf_slot_id": "08a1fc507b97f8760820d019fb1c2a0cafa0ab4fda9a58583de92c02fd32f6b8",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "1c4fda09ded62a1f8f992a381ffd4a57fd12e61470e505bb1e196a84746c0593",
                    "preimage_two": "20a69a73140f10d24d9b5e7f568c13c78e4ab9e256304cd60492a05bac7f3c56",
                    "preimage_three": "0f4b4a0f04face3460592fbe9b6ab324052b5554c4c225537249ce8de8a8a108"
                }
            }
        },
        "status": "Unspent",
        "_id": "0c7e794554915dc7f1d3311a32524b718343587a7a6b721cde17af46ee830e86",
        "nullifier": "2ef29334fe80e43b1391d78f6af2c732f3c874c48f78fc6feb34da0e65d6dce4",
        "layer_1_transaction_hash": "0x68e244c7d91a9fa673e0aa4bd45d8f65548ed394aa6d72451812fd8d94c9f26b",
        "layer_2_block_number": "0x1"
    },
    {
        "preimage": {
            "value": "0000000000000000000000000000000000000000000000000000000000000008",
            "nf_token_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "nf_slot_id": "05192843eed16d0c3cf70a2568da2b2b2c1509fae678d495f31573bbd624cb9f",
            "public_key": "0000000000000000000000000000000000000000000000000000000000000001",
            "salt": {
                "type": "Deposit",
                "value": {
                    "preimage_one": "1c4fda09ded62a1f8f992a381ffd4a57fd12e61470e505bb1e196a84746c0593",
                    "preimage_two": "20a69a73140f10d24d9b5e7f568c13c78e4ab9e256304cd60492a05bac7f3c56",
                    "preimage_three": "0f4b4a0f04face3460592fbe9b6ab324052b5554c4c225537249ce8de8a8a108"
                }
            }
        },
        "status": "PendingCreation",
        "_id": "1619fb9f40e6e0086670d5d760c49a4cdedad3db7b24ca97cfd58ba94ea0fe95",
        "nullifier": "2919c48d40545d097dbdfb642a7cea403c4b1be6320cf309b26437b64a18232c",
        "layer_1_transaction_hash": null,
        "layer_2_block_number": null
    }
]
```

When this L2 block is onchain, check commitment again, and you will find only one commitment is changed to unspent, this is what we expected about new behaviours of NightfallV3.

###   RoundRobin upgrade (V3): change stake required to register as proposers

- Intend: Raise the global stake in an upgrade from 4 to 40.
- Before: Proposer need to stake 4 to become proposer.
- After: New Proposer gets error if it stakes 4
- Change: Add `setStakeRequirement` function in new version.

Note that our proposer registration api only needs new proposer's url, the stake number is read from toml which is set during first time deployment. For this upgrading, we don't plan to change the api. Failing to stake 4 after updrading should be enough to show our upgrading works.

We first clean the test in `nightfall_test/src/run_tests.rs` (comment the code from line 65 to line 992), so we will start with clean status.

Run `docker compose --profile development build` and `docker compose --profile development up`

when you see `nf4_test exited with code 0`, use Postman to check current proposers, and you should see something like:

```
[
    {
        "stake": "0x4",
        "addr": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
        "url": "http://proposer:3000",
        "next_addr": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
        "previous_addr": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720"
    }
]
```

This shows our initial round robin setting, you need to stake 4 to be a proposer.

Next, we start the upgrading:

```
// Build the deployer image with the new artifacts
docker compose --profile development build deployer

// One-off shell in the fresh image
docker compose --profile development run --no-deps --rm deployer bash

// set up environment for restarted deployer
export NF4_RUN_MODE=local
export NF4_SIGNING_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
export ROUNDROBIN_PROXY=$(curl -s http://configuration/configuration/toml/addresses.toml | awk -F\" '/^round_robin/{print $2}')
export FOUNDRY_OUT=blockchain_assets/artifacts

// verify if you set ROUNDROBIN_PROXY correctly, this should be in your terminal log during the first time deployment
echo "ROUNDROBIN_PROXY=$ROUNDROBIN_PROXY"

export RPC_URL=http://anvil:8545

// Just double check if we put the new things 
ls -l blockchain_assets/contracts/RoundRobinV3.sol
ls -l blockchain_assets/script/upgrade_RoundRobin_V3.s.sol

// Build the new changes
forge build --force

// deploy new updated contract
forge script blockchain_assets/script/upgrade_RoundRobin_V3.s.sol:UpgradeRoundRobinWithLogging \
  --sig "run(address)" "$ROUNDROBIN_PROXY" \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

// Raise the on-chain stake requirement to 40

cat > blockchain_assets/script/rr_set_stake.s.sol <<'SOL'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Script.sol";

interface IRR {
    function setStakeRequirement(uint256) external;
    function STAKE() external view returns (uint256);
}

contract SetStake is Script {
    function run(address proxy, uint256 newStakeWei) external {
        uint256 pk = vm.envUint("NF4_SIGNING_KEY");
        vm.startBroadcast(pk);
        IRR(proxy).setStakeRequirement(newStakeWei);
        vm.stopBroadcast();
    }
}
SOL

// Call it with 40:

export NEW_STAKE_WEI="40"

forge script blockchain_assets/script/rr_set_stake.s.sol:SetStake \
  --sig "run(address,uint256)" "$ROUNDROBIN_PROXY" "$NEW_STAKE_WEI" \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

// Use certification api to give Proposer 1 a valid certificate, because we asked everyone should have valid certificate before interacting with our contracts.

// Use register proposer api again, this will read the stake from toml and try to register with stake 4

nf4_proposer                              | [2025-09-10T17:46:29Z ERROR ethers_providers::rpc::transports::ws] error=(code: 3, message: execution reverted: revert: You have not paid the correct staking amount during registration, data: Some(String("0x08c379a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040596f752068617665206e6f7420706169642074686520636f7272656374207374616b696e6720616d6f756e7420647572696e6720726567697374726174696f6e")))
```

So far, we verified that we successfully increased required stakes for proposer registration.


###   RollupProofVerificationKey upgrade (V3): install a “bad” VK

- Intend: Change vk in the updation to mimic a accident update.
- Before: Proposer can propose a block with valid information.
- After: Proposer can't propose a block with valid information.
- Change: point the VK provider to an intentionally incorrect key; valid blocks should fail verification.

You can make a small change so `nightfall_test/src/run_tests.rs` will produce at least two blocks, therefore we are sure everything works as we expect.

Run `docker compose --profile development build` and `docker compose --profile development up`

When you see `nf4_test exited with code 0`, don't stop your docker. But add these files in: `blockchain_assets/nightfall_replace_vk.toml`, `blockchain_assets/script/replace_vk_from_toml.s.sol`.

The first toml file defines the new changes we want for new vk (in this example, we just change one g1 point to generator point.), and the second contract defines how we deploy and update the vk from the one in generated `nightfall.toml` to the one in `nightfall_replace_vk.toml`

After copy paste these files in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifacts
docker compose --profile development build deployer

// One-off shell in the fresh image
docker compose --profile development run --no-deps --rm deployer bash

// Need to put the file in deployer
// mv blockchain_assets/nightfall_replace_vk.toml ./

// Just double check if we put the new things 
ls -l nightfall_replace_vk.toml
ls -l blockchain_assets/script/replace_vk_from_toml.s.sol


// set up environment for restarted deployer
export NF4_RUN_MODE=local
// 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6 is the private key of anvil account 9, which is the owner of x509, aka, only account 9 can update x509 contract.
export NF4_SIGNING_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
export VK_PROXY=$(cat configuration/toml/addresses.toml | \
  awk -F'"' '/^(vk_provider|vk)[[:space:]]*=/{print $2}')

// Change this to your vk proxy contract address.
export VK_PROXY=0x5FC8d32690cc91D4c39d9d3abcBD16989F875707

export FOUNDRY_OUT=blockchain_assets/artifacts
export RPC_URL=http://anvil:8545

// verify if you set NIGHTFALL_PROXY correctly, this should be in your terminal log during the first time deployment
echo "VK_PROXY=$VK_PROXY"

# sanity: the proxy must have code
cast code "$VK_PROXY" --rpc-url "$RPC_URL"  # must not be 0x

# see current version/hash
cast call "$VK_PROXY" 'vkVersion()(uint64)' --rpc-url "$RPC_URL"
cast call "$VK_PROXY" 'vkHash()(bytes32)'   --rpc-url "$RPC_URL"

// Build the new changes
forge build --force

// replace vk
forge script blockchain_assets/script/replace_vk_from_toml.s.sol:ReplaceVKFromToml \
  --sig "run(address)" "$VK_PROXY" \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

# verify it changed
cast call "$VK_PROXY" 'vkVersion()(uint64)' --rpc-url "$RPC_URL"
cast call "$VK_PROXY" 'vkHash()(bytes32)'   --rpc-url "$RPC_URL"

// Start a deposit api call and wait for proposer to make a block
// Trigger another block -> expect verification failure

nf4_proposer | [2025-09-12T19:21:16Z ERROR ethers_providers::rpc::transports
::ws] error=(code: 3, message: execution reverted: revert: Rollup proof verification failed, data: Some(String("0x08c379a0000000000000000000000000000000000000000000
0000000000000000000020000000000000000000000000000000000000000000000000000000000000
0020526f6c6c75702070726f6f6620766572696669636174696f6e206661696c6564")))
nf4_proposer       | [2025-09-12T19:21:16Z ERROR nightfall_proposer::drivers::blockchain::block_assembly] Failed to propose block: Did not receive a transaction receipt
```

###   RollupProofVerifier upgrade (V2): make verification fail

- Intend: Force verification failure in the `verify_OpeningProof` logic itself.
- Before: Proposer can propose a block with valid information.
- After: Proposer get error during proof verification onchain with valid information.  Block proposal fails on-chain with a deterministic revert and Nightfall bubbles “Rollup proof verification failed.”
- Change: `verify_OpeningProof` gives `false` as return.

You can make a small change so `nightfall_test/src/run_tests.rs` will produce at least two blocks, therefore we are sure everything works as we expect.

Run `docker compose --profile development build` and `docker compose --profile development up`

When you see `nf4_test exited with code 0`, don't stop your docker. But add these files in: `blockchain_assets/contracts/proof_verification/RollupProofVerifierV2.sol`, `blockchain_assets/script/upgrade_RollupProofVerifier_V2.s.sol`.

The first file defines the new changes we want for RollupProofVerifier, and the second contract defines how we deploy and update the vk from the one in generated `blockchain_assets/contracts/proof_verification/RollupProofVerifier.sol` to the one in `blockchain_assets/contracts/proof_verification/RollupProofVerifierV2.sol`

After copy paste these files in, do the followings in a new terminal under nightfall_4_pv:

```
// Build the deployer image with the new artifacts
docker compose --profile development build deployer

// One-off shell in the fresh image
docker compose --profile development run --no-deps --rm deployer bash

// Just double check if we put the new things 
ls -l blockchain_assets/contracts/proof_verification/RollupProofVerifierV2.sol
ls -l blockchain_assets/script/upgrade_RollupProofVerifier_V2.s.sol

// set up environment for restarted deployer
export NF4_RUN_MODE=local
export NF4_SIGNING_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

// Change this to your vk proxy contract address.
export VERIFIER_PROXY=0x8A791620dd6260079BF849Dc5567aDC3F2FdC318

export FOUNDRY_OUT=blockchain_assets/artifacts
export RPC_URL=http://anvil:8545

// verify if you set NIGHTFALL_PROXY correctly, this should be in your terminal log during the first time deployment
echo "VERIFIER_PROXY=$VERIFIER_PROXY"

// Build the new changes
forge build --force

// replace vk
forge script blockchain_assets/script/upgrade_RollupProofVerifier_V2.s.sol:UpgradeRollupProofVerifierWithLogging \
  --rpc-url "$RPC_URL" \
  --broadcast -vvvv

// Start a deposit api call and wait for proposer to make a block
// Trigger another block -> expect verification failure

nf4_proposer | [2025-09-12T19:21:16Z ERROR ethers_providers::rpc::transports
::ws] error=(code: 3, message: execution reverted: revert: Rollup proof verification failed, data: Some(String("0x08c379a0000000000000000000000000000000000000000000
0000000000000000000020000000000000000000000000000000000000000000000000000000000000
0020526f6c6c75702070726f6f6620766572696669636174696f6e206661696c6564")))
nf4_proposer       | [2025-09-12T19:21:16Z ERROR nightfall_proposer::drivers::blockchain::block_assembly] Failed to propose block: Did not receive a transaction receipt
```

### Cleanup (optional, when the deployer image needs fresh artifacts)
`forge clean`

`rm -rf blockchain_assets/artifacts/build-info/*`

`forge build --force`
