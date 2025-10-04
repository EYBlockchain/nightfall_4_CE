# Nightfall Client Playbook — Plume Testnet

For Plume documentation and testnet details refer this, https://docs.plume.org/plume/developers/network-information

Clients are the application that normal users will employ to make transactions that are hidden by ZKP. the user can intitiate three types of transaction via the `client`:

1. Deposit: This will pass a payment from the user to the Nightfall smart contract (an escrow transaction). It will create a pending deposit which will later be picked up by the proposer and turned into deposit commitment(s) with an associated proof (this enables a deposit to be treated as a special case of a transfer transaction). User can deposit a fee in a deposit request which can be used to pay other transactions in the future, but it's not compulsory. If user deposit a so-called `deposit_fee`, then this deposit request will generate two commitments: one deposit commitment represents the `value` of the token the user intends to deposit, while the other is a `deposit_fee` commitment, which represents the fee the user deposits into Nightfall to pay the proposer for future transactions. If `deposit_fee == 0`, then there is only one value deposit commitment.

2. Transfer: This operation will select one or two existing commitments of suitable value (sum equal to or greater than the transfer value, with the tokenId matching the tokenId to be transferred). In addition to the transfer value, it will also account for the required fee. Two new output commitments will be created: one for the new owner receiving the transferred commitment, and the other for any change returned to the original owner of the input commitments. A separate fee commitment and fee change commitment will also be generated. All of these commitments will be validated by creating an UltraPlonk proof, ensuring that the operation is correct. The resulting commitments and proof are wrapped into a `ClientTransaction<P>` struct, which is then sent to one or more `proposer`s for inclusion in a Layer 2 block.

3. Withdraw: This is a special case of a Transfer but, rather than output commitments being created, suffienct funds are de-escrowed for the recipient to withdraw the amount they are being paid from the Nightfall contract.

*Note that all of these transactions expect a `X-Request-ID` header to be provided in UUID v4 format, and will fail with a bad request if this is not provided. The value of this header will be reported in log messages and returned with the response.*

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

# Clean the previous state
forge clean && forge build
cargo clean && cargo build

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
NIGHTFALL_ADDRESS="0xf86806F5eb3AE6cb08Fa2e5aD23bf1ba7b2D7CE3"
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
