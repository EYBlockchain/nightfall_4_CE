# Base Sepolia deployment

This document describes the detailed steps for deploying Nightfall_4 to the Base Sepolia testnet, with real rollup proving keys, a Proposer running on a large Azure server and one or more Clients running on EY laptops.

It doesn't provide very much explanation of why each of these steps is done. For a deeper understanding consult [nf4_4.md](./nf_4.md) in particular the testnet beta section.

This is _not_ a production deployment, or even a permanent testnet deployment: there isn't the right infrastructure and there is no attempt to use a trusted SRS; but it allows for some test running over a longer term.

## Preliminaries

You need access to the RCP port of a blockchain client that supports websockets, from all of your environments. They don't have to be the same blockchain client. Alchemy works for Base Sepolia.

Any environment that you run in should have Rust 1.84.1 installed (1.85.0 currently doesn't work)

If you are using an EY laptop, install the Azure CLI app so you can use the command line to carry out Azure actions

## Generate accounts

- [ ] You will need three Base Sepolia accounts, funded with ~0.5 ETH. You can use Metamask to generate these.
- [ ] Make a copy of `.env` and call it `local.env`. Do not push this to the repo as it will contain secrets.
- [ ] Extract the three private keys from Metamask and replace the `CLIENT_SIGNING_KEY`, `PROPOSER_SIGNING_KEY` and `DEPLOYER_SIGNING_KEY` in `local.env` with these keys, also update the `CLIENT_ADDRESS` with the address corresponding to the `CLIENT_SIGNING_KEY`.
- [ ] While you are editing, add a variable `NF4_ETHEREUM_CLIENT_URL` to your `local.env` that has the websocket url of your blockchain client.

There is no need to change any of the other variables in `local.env`; they won't be used. If you are using different blockchain client urls for different containers then you'll need to make individual `local.env` files for each container.

## Deploy smart contracts

- [ ] Clone the nightfall_4 repo into the environment where you will build and run the Deployer container. This could be your laptop.
- [ ] Run:

  ```sh
  cargo build
  docker compose --profile indie-deployer build
  docker compose --profile indie-deployer --env-file local.env up
  ```

- [ ] Check that the deployer logs say that it is deploying contracts and that it exits with `code 0`
- [ ] Read the `configuration/toml/addresses.toml` file and copy the addresses of the deployed contracts somewhere for later.

## Deploy the Proposer

- [ ] Run screen, unless you have an alternative way to keep the containers running when you log out.
- [ ] Clone the nightfall_4 repo into the environment where you will build and run the Proposer container. This _cannot_ be your laptop.
- [ ] Copy the addresses from the smart contract deployment into `[base_sepolia.contracts.contract_addresses]` in `nightfall.toml`. The Proposer and Client will use these addresses.
- [ ] Run:

  ```sh
  cargo build
  cargo run --release --bin key_generation
  docker compose --profile indie-proposer build
  docker compose --profile indie-proposer --env-file local.env up
  ```

## Deploy the Client

We'll assume that this is running on your EY laptop. The Client needs to access the Proposer's `transaction` API, which is exposed on port 3001. We don't want to access it over the internet because we don't have appropriate infrastructure in place to secure the server in such an environment. Instead, we'll use Azure Bastion to connect and we'll make an SSH tunnel. This has the effect of making one of the server's ports appear as though it is a port on the laptop.

Unfortunately, that's not all there is to it. Azure Bastion will only allow SSH or RDP protocols to use its tunnels, not HTTP, which is what we want to pass to the server. To work around that, we'll make a tunnel to the server, as above, to enable an SSH connection, then we'll use _that_ SSH connection to create a tunnel within the bastion tunnel, over which we'll tunnel the HTTP. If we do that then all Bastion sees is SSH traffic and it's happy.

- [ ] Log in to Azure with your privileged account (the one you use for the Azure web portal):

  ```sh
  az login --use-device-code
  ```

- [ ] Create an SSH tunnel connecting the server's SSH port (22) to a port on your local machine (2222 in the example):

  ```sh
  az network bastion tunnel --name "RNDHPCVM01-vnet-bastion" --resource-group "UE2DBLKCLDRND01" --target-resource-id "/subscriptions/ee201e68-47b6-4321-ac1a-8607739530a8/resourceGroups/UE2DBLKCLDRND01/providers/Microsoft.Compute/virtualMachines/RNDHPCVM03" --resource-port "22" --port "2222"
  ```

  Note that the above assumes you are using VM RNDHPCVM03, if that's not the case, change the target resource id (if it's just another big server, you can just change the VM's name at the end of the long string; the rest will be the same).
- [ ] Make an SSH connection through the tunnel and tunnel port 3001 back:

  ```sh
  ssh -p 2222 -i <path to your server-login private key> -L 3001:localhost:3001 <username>@localhost
  ```

  At this point, your terminal should be connected to the server CLI.

- [ ] _Open another terminal_ so that you are issuing commands to your laptop, and check you get a `200 OK` response from the Proposer. This means your tunnel is working:

  ```sh
  curl -i 'http://localhost:3001/v1/health'
  ```

- [ ] Run up the Client:

  ```sh
  cargo build
  cargo run --release --bin key_generation
  docker compose --profile indie-client build
  docker compose --profile indie-client --env-file local.env up
  ```

## Initialise using Curl

- [ ] The Client and Proposer need to be authorised to interact with Nightfall. Give them test certificates (open a fresh terminal so you can issue the curl commands on your laptop):

  ```sh
  curl -i --request POST 'http://localhost:3000/v1/certification' \
    --header 'Content-Type: multipart/form-data' \
    --form 'file=@blockchain_assets/test_contracts/X509/_certificates/user/user-1.der' \
    --form 'file=@blockchain_assets/test_contracts/X509/_certificates/user/user-1.priv_key'
  curl -i --request POST 'http://localhost:3001/v1/certification' \
    --header 'Content-Type: multipart/form-data' \
    --form 'file=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.der' \
    --form 'file=@blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key'
  ```

- [ ] The Client needs some ZKP Keys:

  ```sh
  curl -i --request POST 'http://localhost:3000/v1/deriveKey' \
    --json '{ "mnemonic": "spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work", "child_path": "m/44'/60'/0'/0/0" }'
  ```

## Usage with curl

After the initialisation is done, you should be able to transact with the deposit, transfer and withdraw endpoints. There is a MockERC20 contract already deployed at `0x6fcb6af7f7947f8480c36e8ffca0c66f6f2be32b`, if you need one. For example, you can deposit with:

```sh
curl -i --request POST 'http://localhost:3000/v1/deposit' \
    --json '{ "ercAddress": "0x6fcb6af7f7947f8480c36e8ffca0c66f6f2be32b", "tokenId": "0x00", "tokenType": "0", "value": "0x04", "rootKey": "0e1e8e51526db38a04508426ec357de828af2ca4cba9d35133d22481523bb7dc", "fee": "0x02",  "deposit_fee": "0x05", "providedCommitmentsFee": ["0x00"] }' 
```
