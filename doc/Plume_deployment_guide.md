## Deployment on a Plume testnet for beta testing

Following documentation can be used to do testnet deployment. 

### Preliminaries

You must be able to access a blockchain client that can accept websocket connections (`ws://` or `wss://`). NF_4 subscribes to blockchain events and therefore an `http://` endpoint *will not work*. For plume testnet, `wss://testnet-rpc.plume.org` url is used.

### Setup the Configuration file

Add a stanza to the `nightfall.toml` file for your testnet configuration, if it doesn't already exist. The `[base_sepolia]` section can be used as a guide. The testnet-dependent items are:

- `genesis_block`: If you are deploying contracts for the first time, set this to roughly the current layer 1 blocknumber for the testnet. This will make responses faster because nightfall will not look for events prior to this block when synchronising.
- `chain_id`: The chain id of the testnet. This is useful because deployments will be logged under a folder with the same name as the chain id. This allows one to distinguish from Anvil deploys, for example.
- `contracts.contract_addresses`: If you have already deployed the contracts then their addresses can be placed here so that nightfall knows where to find the contracts. Otherwise it will attempt to read the latest deployment log file, which is less reliable (e.g. you may not have deployed the contracts so your log file won't contain the correct information). If you don't want this to happen, leave the values blank (empty strings). These values will be ignored if you've told nightfall to deploy contracts via the `deploy_contracts` configuration item.
- `deploy_contracts`: If true, deployer will deploy new contracts, otherwise it will run and set up keys but won't deploy contracts before exiting. It's often convenient to override this value with an environment variable (see later) to save rebuilding the containers.
- `proposer_url` : If you are running a proposer node, you need to expose the port 3001 from the large server. Replace the `proposer_url` with that, e.g `http://<server-ip>:3001`
- `configuration_url`: You need to expose the port 8080 or any other port and replace the `configuration_url` with that, add that port to `docker-compose.yml` as under `configuration` service as well.

##  Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY=
CLIENT2_SIGNING_KEY=
CLIENT_ADDRESS=
CLIENT2_ADDRESS=
PROPOSER_SIGNING_KEY=
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY="0x......." # your private key
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

### Deploying contracts

You should only need to do this once and will probably only redeploy if there is an update to the Smart Contracts themselves. Do not forget that you will need to run on a large server (144 cores, 750GB RAM is a good size), to generate the keys.

---

##  Get the source

All the changes for deployment on plume testnet are made on the branch `plume_testnet_deployer`

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout plume_testnet_deployer
```

---

## Generate proving keys

This will download a large file (approximately 12GB):
```bash
cargo run --release --bin key_generation

```

This will save all the keys in `configuration/bin/`
---

## Generate proving keys

```bash
docker compose --profile indie-deployer build
docker compose --profile indie-deployer --env-file local.env up
```

This should deploy all the Nightfall contracts. Their addresses will have been written to `configuration/toml/addresses.toml` and will also be in the deployment logs `blockchain_assets/logs`. Provide these addresses to the `client` and `proposer` via either the configuration server, their `addresses.toml` file or by adding them to their `nightfall.toml` file (in the latter case don't forget to set `deploy_contracts=false` or they'll be ignored). Use whichever method is most appropriate for your infrastructure.

As usual, a `Code 0` exit indicates a successful outcome.

## Configuration for Proposer and Clients

We need to expose some configurations like the keys generated and the deployed contract addresses. 

## Build and Run the Configuration 

```bash
docker compose --profile configuration build
docker compose --profile configuration --env-file local.env up -d
docker compose --profile configuration --env-file local.env logs -f
```