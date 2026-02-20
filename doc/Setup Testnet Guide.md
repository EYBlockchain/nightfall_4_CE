# Setup Testnet Guide

## Overview

Setting up Nightfall_4 on a host chain (testnet) involves deploying smart contracts, generating zero-knowledge proving keys, configuring network metadata, and starting the required services.

At a high level, the process consists of:

1. Running the **deployment service** to deploy contracts and generate proving keys.
2. Running the **configuration service** to host contract addresses, contract hashes, and keys.
3. Starting the **designated proposer** to assemble and submit L2 blocks.
4. Starting the **client service** to submit transactions.
5. Optionally registering and rotating additional proposers.

During deployment, a **designated proposer** is registered by deployer. This proposer is responsible for assembling L2 blocks and proposing them to the host chain. Additional proposers can later be registered and rotated into the active role when it's ready to rotate proposer.

------
******
______

## Step 1: Start the deployer

> ⚠️ **Important**
>
> The host chain RPC endpoint **must support WebSocket connections** (`ws://` or `wss://`).
> Nightfall subscribes to blockchain events, so an `http://` / `https://` endpoint will **not** work.

For example, for the Plume host chain, a WebSocket endpoint such as: `wss://host chain-rpc.plume.org`. 

### Step 1.1: Setup the Configuration file

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout -b host-chain/deployer
```

#### Step 1.1.1: Chang `nightfall.toml`
Add a stanza to the `nightfall.toml` file for your host chain configuration, if it doesn't already exist. Use the [development] section as a reference, and rename it to your host chain name (e.g. [host-chain]). 

Host-chain dependent items:

- `[host-chain]-genesis_block`: If you are deploying contracts for the first time, set this to roughly the current Layer 1 block number on the host chain. This speeds up syncing because Nightfall will not scan for events before this block.

- `[host-chain]-ethereum_client_url`: The host chain RPC URL. Must be WebSocket (`ws://` / `wss://`).

- `[host-chain]-configuration_url`: URL where the configuration service is exposed. If you change the port (default `8080`), update it here and expose the same port in `docker-compose.yml`.

- `[host-chain.network]-chain_id`: Chain ID of the host chain. Deployment logs are written under a folder named after this chain ID.

- `[host-chain.owners]`: Contract owners. These are required for upgrades.

- `[host-chain.nightfall_deployer]`: `default_proposer_address` and `default_proposer_url` bootstrap L2 block proposing.
When running the default proposer node, you must expose port `3001` on the host.
Set `default_proposer_url` to something like `http://<server-ip>:3001`.
Other values such as `proposer_stake` and `proposer_ding` are liveness parameters.
If you need to modify these after deployment, you must upgrade the RoundRobin contract.
See `doc/Upgradable Contracts Guide.md`.

- `[host-chain.contracts]-deploy_contracts`: If true, the deployer will deploy new contracts.
If false, it will generate/setup keys but will not deploy contracts before exiting.
It can be convenient to override this via an environment variable to avoid rebuilding containers.

- `[host-chain.contracts.contract_addresses]`: If contracts are already deployed, place their addresses here so Nightfall can locate them reliably.
Otherwise, Nightfall will attempt to read the latest deployment log file, which is less reliable.
If you want to avoid that behavior, leave these values as empty strings.
These values are ignored if `deploy_contracts=true`.

- `[host-chain.certificates]`: Root CA information for X.509 certificates.
If you are using the testing certificate setup, you can copy the values from [development.certificates].

#### Step 1.1.2: Update `docker-compose.yml`
In `docker-compose.yml`, locate the `indie-deployer` service and update the run mode:

Set:

`NF4_RUN_MODE=${NF4_RUN_MODE:host-chain}`

Go to `Configuration`, uncomment 
```bash
restart: unless-stopped
    ports:
      - "8080:80"   
```

### Step 1.2:  Create `local.env`

Create a file named `local.env` in the repo root with the following content.
Replace placeholders (`0x...`) where required.

The deployer will use the account corresponding to DEPLOYER_SIGNING_KEY to deploy contracts on the host chain.

```bash
DEPLOYER_SIGNING_KEY="0x......." # your private key
```
Make sure the deployer account is funded (0.1 ETH):
```

export NF4_ETHEREUM_CLIENT_URL="your-rpc-url"
cast balance 0x...(your host chain account) --rpc-url "$NF4_ETHEREUM_CLIENT_URL"

```

### Step 1.3: Disable X509 certifiate check if needed
If you don't need clients/proposers to submit X509 certificate, you can disable X509 by changing `x509Contract.enableAllowlisting(true);` to `x509Contract.enableAllowlisting(false);`  in `blockchain_assets/script/deployer.s.sol`.

### Step 1.4: Deploying contracts

You usually only need to deploy once. Re-deploy only when smart contracts change.
> ⚠️ **Resource requirement:**
>
> Key generation is heavy. A large server is recommended (e.g., 144 cores / 750GB RAM).

---

#### Step 1.4.1: Build Contracts
```bash
forge clean && forge build
```
---

#### Step 1.4.2: Generate proving keys

This will download a large file and generate multiple keys:
```bash
NF4_MOCK_PROVER=false cargo run --release --bin key_generation

```
You should see:

- `Generating keys for REAL rollup prover` when it begins

- `Generating keys for rollup prover finished` when complete

Key generation can take ~1.5 hours the first time, it's because we need to download a huge file and run trusted setup. Subsequent runs are faster (~20 minutes) because
`ppot_26.ptau` and `bn254_setup_26.cache` are reused if they are not broken. Only need to change keys when circuits are changed. 

Keys and intermediate files are stored under configuration/bin/, for example for `block_size == 64`, we have the following keys:
```bash
base_bn254_pk: 1.9G
base_grumpkin_pk: 61M
decider_pk: 30G
decider_vk: 1.3K
deposit_proving_key: 233M
merge_bn254_pk_0: 929M
merge_grumpkin_pk_0: 121M
merge_grumpkin_pk_1: 121M
proving_key: 30M
ppot_26.ptau: 73G
bn254_setup_26.cache: 2.1G
```
We have extra keys for `block_size == 256`. 


---

#### Step 1.4.3: Start deployer service 

```bash
docker compose --profile indie-deployer build
docker compose --profile indie-deployer --env-file local.env up
```
This deploys all Nightfall contracts.

- Contract addresses that proposers/clients need to directly interact with  are written to: `configuration/toml/addresses.toml`

- Contract hashes of the aformentioned contracts are written to: `configuration/toml/contract_hashes.toml`

- Deployment logs are saved under: `blockchain_assets/logs`


`nf4_indie_deployer exited with code 0` indicates a successful outcome.

> ⚠️ **Troubleshooting:**
>
> If you see the following error:
> `
ERROR panic: 'main' panicked at 'Could not create blockchain client connection:
ProviderError("URL error: URL scheme not supported")': /app/lib/src/lib.rs:63
`
it usually means the RPC endpoint is invalid or uses an unsupported scheme. 

### Step 1.5: Run the configuration service

The configuration service hosts:

- deployed contract addresses,

- deployed contract hashes (for on-chain verification),

- generated proving keys.

Build and Run the Configuration service following the next steps:

```bash
docker compose --profile configuration build
docker compose --profile configuration --env-file local.env up -d
docker compose --profile configuration --env-file local.env logs -f
```

To verify if this step finishes successfully, you can check if configuration url (`[host-chain]-configuration_url`) hosts keys, contract addresses and contract hashes correctly:
1. Open another terminal in a new folder to get keys, `mkdir configuration/bin/keys`,`curl -v [host-chain]-configuration_url:8080/<key_name> -o configuration/bin/keys/<key_name>`
where you need to it for following keys `base_bn254_pk`, `base_grumpkin_pk`, `decider_pk`, `deposit_proving_key`, `merge_bn254_pk_0`, `merge_grumpkin_pk_0`, `merge_grumpkin_pk_1`, and `proving_key`. use `ls -lh`to check the key size, it should match the size mentioned before.
2. `curl [host-chain]-configuration_url/configuration/toml/addresses.toml` to get addresses for `nightfall`, `round_robin`, `x509` and `verifier`.
3. `curl [host-chain]-configuration_url/configuration/toml/contract_hashes.toml` to get contract hashes for `nightfall_hash`, `round_robin_hash`, `x509_hash`.


------
******
______

## Step 2: Start the disignated proposer
As mentioned before, disignated proposer is for liveness, but this doesn't mean that Nightfall has centralisation problem for proposer. You can safely skip this step, and run Step 3 to start a proposer node, and rotate proposer to its turn. 
> ⚠️ **Resource requirement:**
>
> Proving a L2 block with full privacy is heavy. A large server is recommended (e.g., 144 cores / 750GB RAM).

The difference between running a designated propser and a new proposer is that designated propser node doesnt need to call the registration api as deployer has registered this proposer during deployment.

### Step 2.1: Get the source

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout -b host-chain/proposer
forge clean && forge build
```
---

### Step 2.2: Stop & clean previous Docker state
```bash
docker compose --profile indie-proposer down -v
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes
```
---

### Step 2.3: Generate/download proving keys
There are two ways to form the keys needed to proving a L2 block, proposer can generate itself or download from the configuration url

1. Method 1: generate keys
```bash
NF4_MOCK_PROVER=false cargo run --release --bin key_generation
```
2. Method 2: get the keys using the configuartion url in the root of `nightfall_4_CE` folder, 
```bash
curl -v [host-chain]-configuration_url:8080/<key_name> -o configuration/bin/<key_name>`
```
where you need to it for these keys: `base_bn254_pk`, `base_grumpkin_pk`, `decider_pk`, `deposit_proving_key`, `merge_bn254_pk_0`, `merge_grumpkin_pk_0`, `merge_grumpkin_pk_1`, `proving_key`. You can verify the key size as mentioned before.

Note that when the deployer starts the deployment with `block_size == 64`, it will generate the aforementioned keys, but proposer can decide to increase the `block_size` to `256`, in this case, proposer need to run key generation itself and change `block_size = 64` to `block_size = 256` in `[host-chain.nightfall_proposer]` of `nightfall.toml`. Only 64 and 256 are surpported.

---
### Step 2.4: Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
PROPOSER_SIGNING_KEY="0x......." 
```
`PROPOSER_SIGNING_KEY` is your private key for L1 address on host chain.
---

### Step 2.5: Change `nightfall.toml` and `docker-compose.yml`
1. Copy the values added in Step 1.1.1 for `nightfall.toml`.
2. Change `docker-compose.yml`:
Go to `indie-proposer-environment`, change `- NF4_RUN_MODE=${NF4_RUN_MODE:-host-chain}`
Go to `volumes:`, uncomment `mongodb_proposer_data:`
---

### Step 2.6: Build and run the Nightfall indie proposer node

From the repo root:

```bash
docker compose --profile indie-proposer build

docker compose --profile indie-proposer --env-file local.env up -d

docker compose --profile indie-proposer --env-file local.env logs -f
```


------
******
______


## Step 3: Start the client node

As a client, you can do deposit, transfer, withdraw in Nightfall. To be able to deposit some tokens into Nightfall, you should have tokens in an ERC20|721|1155|3525 contract that you can access.

### Step 3.1: Create `local.env`
```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout -b host-chain/client
```

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY="0x......." 
CLIENT2_SIGNING_KEY= 
CLIENT_ADDRESS="0x......." 
CLIENT2_ADDRESS=
PROPOSER_SIGNING_KEY=
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY=
NIGHTFALL_ADDRESS=
NF4_SIGNING_KEY=
WEBHOOK_URL=
AZURE_VAULT_URL=
DEPLOYER_SIGNING_KEY_NAME=
PROPOSER_SIGNING_KEY_NAME=
PROPOSER_2_SIGNING_KEY_NAME=
CLIENT_SIGNING_KEY_NAME=
CLIENT2_SIGNING_KEY_NAME=
AZURE_CLIENT_ID=
AZURE_CLIENT_SECRET=
AZURE_TENANT_ID=
```
`CLIENT_SIGNING_KEY` is your L1 address's private key on host chain.
`CLIENT_ADDRESS` is your L1 address on host chain.
---
### Step 3.2: Deploy ERC contracts
You can  deploy your own ERC-20/721/1155/3525 contracts using the following script: `blockchain_assets/script/mock_deployment.s.sol`

If you want to do mock ERC deployments, you can do:
1. Run `curl [host-chain]-configuration_url:8080/configuration/toml/addresses.toml` to get Nightfall contract address
2. Add Nightfall contract address in `local.env-NIGHTFALL_ADDRESS` and your host chain L1 private key to `local.env-NF4_SIGNING_KEY`. You will need to add `CLIENT2_ADDRESS`, which can be a dummy value or the same value as `CLIENT_ADDRESS`, this is just for testing.
3.	`forge clean && forge build` 
4.	`export $(grep -v '^#' local.env | xargs)`  
5.  `forge script blockchain_assets/script/mock_deployment.s.sol:MockDeployer --rpc-url host-chain-rpc-url --broadcast --legacy --slow`	

After this step, you will get the mocked ERC address of `ERC20Mock`, `ERC721Mock`, `ERC1155Mock`, and `ERC3525Mock`. You should store these addresses locally, which will be used later when you are using client APIs.
---

### Step 3.3: Get the proving key from configuration server
Client node needs to prove its transfers and withdraws using the `proving_key` from configuration url:
```bash
curl -v [host-chain]-configuration_url:8080/proving_key -o configuration/bin/proving_key`
```

You can verify that there is `configuration/bin/proving_key` with size 30M.
---

### Step 3.4: Stop & clean previous Docker state

```bash
docker compose --profile indie-client down
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes
```




---
### Step 3.5: Change `nightfall.toml` and `docker-compose.yml`
1. Copy the values added in Step 1.1.1 for `nightfall.toml`.
2. Change `docker-compose.yml`:
Go to `indie-client-environment`, change `- NF4_RUN_MODE=${NF4_RUN_MODE:-host-chain}` and `- NF4_NIGHTFALL_PROPOSER__URL= ${NF4_NIGHTFALL_PROPOSER__URL:-server-ip:3001}`
Go to `volumes:`, uncomment `mongodb_client_data:`
Go to `db_client`, uncomment `volumes: - mongodb_client_data:/data/db`

### Step 3.6: Build and run the Nightfall client

From the repo root:

```bash
docker compose --profile indie-client build

docker compose --profile indie-client --env-file local.env up
```
### Step 3.7: Call Client APIs
When you see `nightfall_client::drivers::blockchain::nightfall_event_listener Subscribed to events`,  you can then interact with Nightfall using the client APIs: https://github.com/EYBlockchain/nightfall_4_CE/blob/master/doc/nf_4.md#client-apis

------
******
______

## Step 4: Start the proposer node

### Step 4.1: Get the source

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout -b host-chain/proposer
```
---

### Step 4.2:Stop & clean previous Docker state
```bash
docker compose --profile indie-proposer down -v
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes
```
---

### Step 4.3: Generate/download proving keys
```bash
NF4_MOCK_PROVER=false cargo run --release --bin key_generation
```

or you can get the keys using the configuartion url, `curl -v [host-chain]-configuration_url:8080/<key_name> -o configuration/bin/<key_name>`
where you need to it for following keys `base_bn254_pk`, `base_grumpkin_pk`, `decider_pk`, `deposit_proving_key`, `merge_bn254_pk_0`, `merge_grumpkin_pk_0`, `merge_grumpkin_pk_1`, `proving_key`.

Note that when the deployer starts the deployment with `block_size == 64`, it will generate the aforementioned keys, but proposer can decide to increase the `block_size` to `256`, in this way, proposer have to run key generation itself.





---
### Step 4.4: Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY=
CLIENT2_SIGNING_KEY=
CLIENT_ADDRESS=
CLIENT2_ADDRESS=
PROPOSER_SIGNING_KEY="0x......." # your private key
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY=
NIGHTFALL_ADDRESS=
WEBHOOK_URL=
AZURE_VAULT_URL=
DEPLOYER_SIGNING_KEY_NAME=
PROPOSER_SIGNING_KEY_NAME=
PROPOSER_2_SIGNING_KEY_NAME=
CLIENT_SIGNING_KEY_NAME=
CLIENT2_SIGNING_KEY_NAME=
AZURE_CLIENT_ID=
AZURE_CLIENT_SECRET=
AZURE_TENANT_ID=
```
### Step 4.5: Build and run the Nightfall indie proposer node

From the repo root:

```bash
docker compose --profile indie-proposer build

docker compose --profile indie-proposer --env-file local.env up -d

docker compose --profile indie-proposer --env-file local.env logs -f
```

### Step 4.6: Rotate Proposer

Now since you started as a proposer, you need to register as a proposer with the url `http://<server-ip>:3001` using the following api

POST /v1/register

```sh
curl -i --request POST 'http://localhost:3000/v1/register' \
    --json '{ "url": "http://<server-ip>:3001" }'
```
### Step 4.7: Register as a Proposer
If you want create blocks as a proposer, you can call rotate proposer api

GET v1/rotate

```sh
curl -i 'http://localhost:3001/v1/rotate'
```

Returns: on success `200 OK` if the active `proposer` was rotated, `423 LOCKED` if proposer rotation was not allowed by the smart contract.
This endpoint will rotate the proposers if the current `proposer` has been active for more than the number of Layer 1 blocks that a `proposer` is allowed to propose for (ROTATION_BlOCKS) (currently set as 4 blocks). This value is set in the construction of RoundRobin.sol.
