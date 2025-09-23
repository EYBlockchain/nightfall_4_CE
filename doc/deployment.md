## Deployment on a testnet for beta testing

This section is useful if you want to deploy to a testnet for longer-term testing in a more 'production like' scenario where you will keep a proposer running and perhaps have serveral clients moving funds between themselves. These instructions may also work for mainnet deployment, but that hasn't been tested.

### Configuration file

Add a stanza to the `nightfall.toml` file for your testnet configuration, if it doesn't already exist. The `[base_sepolia]` section can be used as a guide. The testnet-dependent items are:

- `genesis_block`: If you are deploying contracts for the first time, set this to roughly the current layer 1 blocknumber for the testnet. This will make responses faster because nightfall will not look for events prior to this block when synchronising.
- `chain_id`: The chain id of the testnet. This is useful because deployments will be logged under a folder with the same name as the chain id. This allows one to distinguish from Anvil deploys, for example.
- `proposer_url`: We need to expose the port 3001, so that client can reach it. update the url with expose url, in `nightfall.toml` and in `docker-compose.yml` for `indie-client`.
- `contracts.contract_addresses`: If you have already deployed the contracts then their addresses can be placed here so that nightfall knows where to find the contracts. Otherwise it will attempt to read the latest deployment log file, which is less reliable (e.g. you may not have deployed the contracts so your log file won't contain the correct information). If you don't want this to happen, leave the values blank (empty strings). These values will be ignored if you've told nightfall to deploy contracts via the `deploy_contracts` configuration item.
- `deploy_contracts`: If true, deployer will deploy new contracts, otherwise it will run and set up keys but won't deploy contracts before exiting. It's often convenient to override this value with an environment variable (see later) to save rebuilding the containers.

Other items may of course require configuring but they're not anything to do with which chain nightfall is deployed on. *You must rebuild the containers for these changes to take effect*.

### Environment variables

Set the environment variable `NF4_RUN_MODE=` like for base_sepolia set it as `NF4_RUN_MODE=base_sepolia`. This will cause Nightfall to use the `[base_sepolia]` section of `nightfall.toml` rather than the `[development]` section.

If you need to, override the `deploy_contracts` configuration item, e.g.: `NF4_CONTRACTS__DEPLOY_CONTRACTS=false` (note the double underscore, which replaces the more common 'dot' notation).

Set `ethereum_client_url`: The url of the rpc endpoint of your layer 1 client. Alternatively, you can set this via `NF4_ETHEREUM_CLIENT_URL` in the `local.env` file.

### Beta testing preliminaries

1. Make sure you have tokens in an ERC20|721|1155|3525 contract that your `client` can access. Your `client` will need some L1 funds to move into L2.
2. Have sufficient base currency funds in a separate account for each `client`, `proposer` and `deployer` to use. On Base Sepolia about 0.5 Eth per account is more than enough to get started.
3. Consider your deployment infrastructure:
4. All three applications (`proposer`, `client`, `deployer`) will need access to the blockchain through an RPC interface. In the case of the `client` and `proposer` this *must* support websockets.
5. The `client` must be able to reach the `proposer`'s `transaction` endpoint on `3001` to be able to send it client transactions, so you need to expose this port.
6. You will need to be able to `curl` the endpoints of the `client` and `proposer` to provide these applications with your user credentials.
7. The `client` holds in-memory ZKP keys and it must therefore run in a secure environment, which only allows access by bona-fide users. How you secure it is not within the scope of this document and the choices you make will need to be informed by the value of your transactions.
8. Note that, like most blockchain clients, Nightfall expects you to provide appropriate load balancing and security infrastructure. These things are not part of the application. As an example, you should terminate any `https://`, handle access tokens, and pass the payload to nightfall within your infrastructure; you should ensure any internet facing ports are appropriately firewalled.
9. Create a file named local.env in the repo root with the following content. Replace placeholders (0x....) with your values where indicated.
 
```bash
CLIENT_SIGNING_KEY="0x......." # your private key
CLIENT2_SIGNING_KEY= 
CLIENT_ADDRESS="0x......." # your public address
CLIENT2_ADDRESS="0xf2Fca7419389fB8f5Db220cdEe9039AD2FFb03b5" 
PROPOSER_SIGNING_KEY="0x745e9fb463ee15a748b2245e08e798dc5f6388870f4d38c4a7d33f9def590723"
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY="0x....." # same as your private key
NIGHTFALL_ADDRESS="0xe407c6d6D86178Dd3Bba8596bf554f0C2624A2Ab"
NF4_SIGNING_KEY="0x..." # same as your private key
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

NOTE : If you want to turn off the x509 certificate check for testnet please change this line ` x509Contract.enableAllowlisting(true) ` in `blockchain_assets/script/deployer.s.sol` to ` x509Contract.enableAllowlisting(false) `


The below assumes you use docker compose to orchestrate your containers, but anything else that works is fine and you probably shouldn't be using docker compose in anything approaching a production environment. The docker compose version will create database containers for the `client` and the `proposer`.

### Beta testing configuration

This is the same as for the previous section, so follow the advice in the 'Configuration', 'Ethereum private keys' and 'Environment variables' sections above. In this situation, however, you will only run the `deployer` when you want to deploy a new set of contracts so `deploy_contracts` for the deployer should always be `true`. Additionally, `mock_prover` should be `false` as there is no real purpose in running with a mock prover at this stage in the game.

If you are not using the provided MongoDB containers, you'll also need to point your containers to their databases via the appropriate `db_url` configuration.

### Deploying contracts


Build and run the stand-alone deployer. You need generate the keys for the proposer, so maybe for testnet we may use the same server as we will be using it for `propose`

```sh
cargo run --release --bin key_generation
```

```sh
docker compose --profile indie-deployer build
docker compose --profile indie-deployer --env-file local.env up
```

This should deploy all the Nightfall contracts. These logs will be written to  will also be in the deployment logs `blockchain_assets/logs`.
Please write these addresses to `configuration/toml/addresses.toml`. 

Create the TOML config folder:
` mkdir -p configuration/toml `

Create `configuration/toml/addresses.toml` with the following content:
`nightfall = "0x... " # nightfall address from deployement logs
 round_robin = "0x... " # round_robin address from deployement logs
 x509 = "0x... " ` # x509 address from deployement logs

Provide these addresses to the `client` and `proposer` via either the configuration server, their `addresses.toml` file or by adding them to their `nightfall.toml` file (in the latter case don't forget to set `deploy_contracts=false` or they'll be ignored). Use whichever method is most appropriate for your infrastructure.

As usual, a `Code 0` exit indicates a successful outcome.

### Deploying the Proposer and Client

Do not forget that the `proposer` will need to run on a large server (144 cores, 750GB RAM is a good size). Make sure `deploy_contracts` is set to false in nightfall.toml if you want the contract addresses to be read from nightfall.toml. Other than that, you need to create rollup proving and verifying keys by locally running:

```sh
cargo run --release --bin key_generation
```


Check that you are generating `REAL` rollup keys (the log output will tell you) and not mock ones. Then run the `proposer`

```sh
docker compose --profile indie-proposer build
docker compose --profile indie-proposer --env-file local.env up
```
