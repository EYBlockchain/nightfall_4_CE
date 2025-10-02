### How to run a proposer node for Plume testnet

For Plume documentation and testnet details refer this, https://docs.plume.org/plume/developers/network-information

The purpose of the `proposer` is to make Layer 2 blocks. It makes an endpoint available to `clients`. Clients are the application that normal users will employ to make transactions that are hidden by ZKP.

Do not forget that the `proposer` will need to run on a large server (144 cores, 750GB RAM is a good size). Moreover, expose the port `3001` of that large server, so the clients nodes can reach you.

### Prerequisites for local installation

The following applications are required:

- forge >=0.2.0
- anvil >=0.2.0
- docker
- openssl
- rust >=1.81.0 +nightly (nightly features are required for using certain unstable options (such as ignore in `rustfmt.toml`) when running `cargo +nightly fmt`. The normal `cargo fmt` on the stable toolchain will ignore these unstable features but will still format the rest of the code).
- git

forge and anvil can be installed as part of the [Foundry](https://github.com/foundry-rs/foundry) suite. 

## 1) Get the source

```bash
git clone https://github.com/EYBlockchain/nightfall_4_CE.git
cd nightfall_4_CE
git checkout plume_testnet_proposer
```

---

## 2) Stop & clean previous Docker state

```bash
docker compose --profile indie-proposer down -v
# DANGER: removes images, containers, networks, and volumes
docker system prune -a --volumes

# Clean the previous state
forge clean && forge build
cargo clean && cargo build

```

---

## 3) Generate proving keys

This will download a large file (approximately 12GB):
```bash
cargo run --release --bin key_generation
```

or you can get the keys using the configuration url, `curl -v http://35.225.105.10:8080/<key_name> -o configuration/bin/<key_name>
where you need to it for following keys  
```bash
base_bn254_pk    
decider_pk  
deposit_proving_key  
merge_grumpkin_pk
proving_key
base_grumpkin_pk 
merge_bn254_pk_0
```
You need do this once and save all the keys.
---

## 4) Wallet setup: MetaMask + Plume testnet

1. Install the MetaMask browser extension: [https://metamask.io/en-GB](https://metamask.io/en-GB)
2. Create a **new network** in MetaMask for **Plume testnet** using the parameters published here: [https://thirdweb.com/plume-testnet](https://thirdweb.com/plume-testnet)
3. Import or create an account and ensure it has **≥ 10 PLUME** for fees (test funds).

---

## 5) Create `local.env`

Create a file named `local.env` in the repo root with the following content. Replace placeholders (`0x....`) with your values where indicated.

```bash
CLIENT_SIGNING_KEY=
CLIENT2_SIGNING_KEY=
CLIENT_ADDRESS=
CLIENT2_ADDRESS=
PROPOSER_SIGNING_KEY="0x......." # your private key
PROPOSER_2_SIGNING_KEY=
DEPLOYER_SIGNING_KEY=
NIGHTFALL_ADDRESS="0xf86806F5eb3AE6cb08Fa2e5aD23bf1ba7b2D7CE3"
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

## 6) Build and run the Nightfall client

From the repo root:

```bash
docker compose --profile indie-proposer build

docker compose --profile indie-proposer --env-file local.env up -d

docker compose --profile indie-proposer --env-file local.env logs -f
```

---

## 7) Register as a Proposer

Now since you started as a proposer, you need to register as a proposer with the url `http://<server-ip>:3001` using the following api

POST /v1/register

```sh
curl -i --request POST 'http://localhost:3000/v1/register' \
    --json '{ "url": "http://<server-ip>:3001" }'
```
---

## 8) Rotate Proposer
If you want create blocks as a proposer, you can call rotate proposer api

GET v1/rotate

```sh
curl -i 'http://localhost:3001/v1/rotate'
```

Returns: on success `200 OK` if the active `proposer` was rotated, `423 LOCKED` if proposer rotation was not allowed by the smart contract.
This endpoint will rotate the proposers if the current `proposer` has been active for more than the number of Layer 1 blocks that a `proposer` is allowed to propose for (ROTATION_BlOCKS) (currently set as 4 blocks). This value is set in the construction of RoundRobin.sol.

---