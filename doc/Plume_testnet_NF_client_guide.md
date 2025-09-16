# Nightfall Client Playbook — Plume Testnet

## 1) Get the source

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout Plume_testnet_client
```

---

## 2) Stop & clean previous Docker state

```bash
docker compose --profile indie-client down
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes
```

---

## 3) Generate proving keys

This will download a large file (approximately 72 GB):
```bash
cargo run --release --bin key_generation
```

---

## 4) Specify Nightfall contract addresses

Create the TOML config folder:

```bash
mkdir -p configuration/toml
```

Create `configuration/toml/addresses.toml` with the following content:

```toml
nightfall = "0xe407c6d6D86178Dd3Bba8596bf554f0C2624A2Ab"
round_robin = "0x955F325CBd3C664333d12cceaAE714089fdF7a84"
x509 = "0xF12b0578237Ea2479Ec97BB5bbd69a52D828F451"
```

---

## 5) Ensure the Docker image copies your addresses

Open `nightfall_client/Dockerfile` and uncomment the copy line (line \~21):

```diff
- # COPY configuration/toml/addresses.toml configuration/toml/
+ COPY configuration/toml/addresses.toml configuration/toml/
```

---

## 6) Wallet setup: MetaMask + Plume testnet

1. Install the MetaMask browser extension: [https://metamask.io/en-GB](https://metamask.io/en-GB)
2. Create a **new network** in MetaMask for **Plume testnet** using the parameters published here: [https://thirdweb.com/plume-testnet](https://thirdweb.com/plume-testnet)
3. Import or create an account and ensure it has **≥ 10 PLUME** for fees (test funds).

---

## 7) Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY="0x......." # your private key
CLIENT_ADDRESS="0x......." # your public address
PROPOSER_SIGNING_KEY="0x745e9fb463ee15a748b2245e08e798dc5f6388870f4d38c4a7d33f9def590723"
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY="0xf07873bcae1dda8c0b2c47e61c761ce55f0c9f11cd493e892ce2650acc60edd8"
NIGHTFALL_ADDRESS="0x68B1D87F95878fE05B998F19b66F4baba5De1aed"
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

---

## 8) Build and run the Nightfall client

From the repo root:

```bash
docker compose --profile indie-client build

docker compose --profile indie-client --env-file local.env up
```

## 9) Deployment script

You can also deploy your own ERC-20/721/1155/3525 contracts using the following script: `blockchain_assets/script/mock_deployment.s.sol`
