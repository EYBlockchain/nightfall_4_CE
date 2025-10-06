# Nightfall Client Playbook — Plume Testnet

## 1) Get the source

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout plume_testnet_client
```

Make sure you pulled the latest changes.

---

## 2) Stop & clean previous Docker state

```bash
# DANGER: removes commitment db
docker compose --profile indie-client down -v
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes
```

---

## 3) Wallet setup: MetaMask + Plume testnet

1. Install the MetaMask browser extension: [https://metamask.io/en-GB](https://metamask.io/en-GB)
2. Create a **new network** in MetaMask for **Plume testnet** using the parameters published here: [https://thirdweb.com/plume-testnet](https://thirdweb.com/plume-testnet)
3. Import or create an account and ensure it has **≥ 10 PLUME** for fees (test funds).

---

## 4) Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY="0x......." # your private key
CLIENT2_SIGNING_KEY= 
CLIENT_ADDRESS="0x......." # your public address
CLIENT2_ADDRESS="0xf2Fca7419389fB8f5Db220cdEe9039AD2FFb03b5" # keep this as test
PROPOSER_SIGNING_KEY="0x745e9fb463ee15a748b2245e08e798dc5f6388870f4d38c4a7d33f9def590723" # keep this as test
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY="0x....." # same as your private key
NIGHTFALL_ADDRESS="0xf86806f5eb3ae6cb08fa2e5ad23bf1ba7b2d7ce3"
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
If you host a online webhook, please put you url in `WEBHOOK_URL`, if you want to host a local webhook, you can follow nf_4.md about how to host a local webhook and replace the webhook url. For easier start, run `python3 webhook.py` in seperate terminal to start a testing local webhook. Webhook is essential to de-escrow fund back to host chain.
Please remove all comments (beginning with `#`) in your local.env file.

---

## 5) Build and run the Nightfall client

From the repo root:

```bash
docker compose --profile indie-client build

docker compose --profile indie-client --env-file local.env up
```

## 6) Deployment script

You can also deploy your own ERC-20/721/1155/3525 contracts using the following script: `blockchain_assets/script/mock_deployment.s.sol`

Or, you can use mock ERC deployments to play with.

1.	`forge build` 
2.	`export $(grep -v '^#' local.env | xargs)`  
3.  `forge script blockchain_assets/script/mock_deployment.s.sol:MockDeployer --rpc-url https://testnet-rpc.plume.org --broadcast --legacy --slow`	


You can then interact with those contract using the APIs: https://github.com/EYBlockchain/nightfall_4_CE/blob/master/doc/nf_4.md#client-apis# Nightfall Client Playbook — Plume Testnet
