use ark_std::rand;
use bip32::{DerivationPath, Mnemonic};
use inquire::Select;
use inquire::Text;
use lib::hex_conversion::HexConvertible;
use nightfall_client::drivers::derive_key::ZKPKeys;
use nightfall_client::drivers::rest::models::{
    NF3DepositRequest, NF3RecipientData, NF3TransferRequest, NF3WithdrawRequest,
};
use nightfall_test::validate_certs::validate_all_certificates;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use url::Url;
use uuid::Uuid;

const CONFIG_PATH: &str = "nightfall_test/bin/config.toml";

/// This module provides a simple UI for interacting with a Nightfall client.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Nightfall Client UI...");

    // Read and parse config.toml into url and mnemonic variables
    let (url, mnemonic) = load_config(CONFIG_PATH);

    // check for client connectivity
    if !check_client_connection(&url).await {
        return Err(format!("Error: Client is not reachable at {}", url).into());
    } else {
        println!("Client is healthy and reachable at {}", url);
    }

    // Derive ZKP keys from the mnemonic and a standard derivation path
    let derivation_path = "m/44'/60'/0'/0/0"
        .parse::<DerivationPath>()
        .expect("Invalid derivation path");
    let zkp_keys = ZKPKeys::derive_from_mnemonic(&mnemonic, &derivation_path)
        .expect("Failed to derive ZKP keys from mnemonic");
    // Print the ZkP compressed public key
    let layer_2_address = zkp_keys
        .compressed_public_key()
        .expect("Failed to get compressed public key")
        .to_hex_string();
    println!("Your layer 2 address is: 0x{}", layer_2_address);

    // Extract ERC20Mock contract address from deployment log file
    let log_path = "blockchain_assets/logs/mock_deployment.s.sol/31337/run-latest.json";
    let log_content =
        std::fs::read_to_string(log_path).expect("Failed to read deployment log file");
    let log_json: serde_json::Value =
        serde_json::from_str(&log_content).expect("Failed to parse deployment log JSON");
    let erc20_address = log_json["transactions"]
        .as_array()
        .and_then(|txs| txs.iter().find(|tx| tx["contractName"] == "ERC20Mock"))
        .and_then(|tx| tx["contractAddress"].as_str())
        .expect("ERC20Mock contract address not found in log");
    let default_erc_address = erc20_address.to_string();

    println!("ERC20Mock contract address: {}", default_erc_address);

    println!("Presenting certificates for validation...");
    // present certificates for validation
    let http_client = Client::new();
    // Validate all certificates (clients and proposer)
    // (name, cert_path, key_path, url)
    let certs = [
        (
            "Client 1",
            "blockchain_assets/test_contracts/X509/_certificates/user/user-1.der",
            "blockchain_assets/test_contracts/X509/_certificates/user/user-1.priv_key",
            url.join("/v1/certification").unwrap(),
        ),
        (
            "Proposer",
            "blockchain_assets/test_contracts/X509/_certificates/user/user-3.der",
            "blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key",
            Url::parse("http://localhost:3001")
                .unwrap()
                .join("/v1/certification")
                .unwrap(),
        ),
    ];
    validate_all_certificates(certs, &http_client).await;

    println!("Ready");
    // start the inquirer to get user input
    loop {
        let action = get_actions()?;
        match action.as_str() {
            "Get L2 balance" => {
                let balance = get_l2_balance(&url, &default_erc_address).await;
                println!("Balance: {}", balance);
            }
            "Get L1 balance" => get_l1_balance(),
            "Deposit" => deposit(&url, &default_erc_address).await,
            "Transfer" => transfer(&url, &default_erc_address, &layer_2_address).await,
            "Withdraw" => withdraw(&url, &default_erc_address).await,
            "Exit" => {
                println!("Exiting the Nightfall Client UI.");
                break;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn load_config<P: AsRef<Path>>(path: P) -> (Url, Mnemonic) {
    #[derive(Deserialize)]
    struct ConfigSection {
        url: String,
        mnemonic: String,
    }
    #[derive(Deserialize)]
    struct ConfigFile {
        config: ConfigSection,
    }

    let config_content = fs::read_to_string(path).expect("Failed to read config.toml");
    let config: ConfigFile = toml::from_str(&config_content).expect("Failed to parse config.toml");
    let url = Url::parse(&config.config.url).expect("Invalid URL format in config.toml");
    let mnemonic = match Mnemonic::new(&config.config.mnemonic, Default::default()) {
        Ok(m) => m,
        Err(_) => {
            let mut rng = rand::thread_rng();
            let new_mnemonic = Mnemonic::random(&mut rng, Default::default());
            println!("Mnemonic not found in config.toml. Generated new mnemonic: \n{}\nPlease add it to your config.toml", new_mnemonic.phrase());
            new_mnemonic
        }
    };
    (url, mnemonic)
}

fn get_actions() -> Result<String, inquire::InquireError> {
    let options = vec![
        "Get L2 balance",
        "Get L1 balance",
        "Deposit",
        "Transfer",
        "Withdraw",
        "Exit",
    ];
    let ans = Select::new("Choose an action:", options).prompt()?;
    Ok(ans.to_string())
}

async fn get_l2_balance(url: &url::Url, default_erc_address: &str) -> i64 {
    let (erc_address, token_id) = {
        let erc_address = inquire::Text::new("Enter ERC address:")
            .with_initial_value(default_erc_address)
            .prompt()
            .expect("Failed to get ERC address");
        let token_id = inquire::Text::new("Enter Token ID:")
            .with_initial_value("0x00")
            .prompt()
            .expect("Failed to get Token ID");
        (erc_address, token_id)
    };
    let mut balance_url = url.clone();
    // Set the path correctly, preserving the base URL and adding the correct endpoint
    let path = format!("/v1/balance/{}/{}", erc_address, token_id);
    balance_url.set_path(&path); // Clear any existing path
    let client = reqwest::Client::new();
    let resp = client
        .get(balance_url)
        .send()
        .await
        .expect("Failed to send request");
    if resp.status().is_success() {
        i64::from_hex_string(
            resp.text()
                .await
                .expect("Failed to read response body")
                .trim_start_matches("00"),
        )
        .expect("Failed to parse balance as i64")
    } else {
        0 // Return 0 if the request fails
    }
}

fn get_l1_balance() {
    // Placeholder for L1 balance retrieval logic
    unimplemented!("L1 balance retrieval is not implemented yet");
}

async fn deposit(url: &url::Url, default_erc_address: &str) {
    println!("Depositing...");
    let nf3_deposit_request = prompt_nf3_deposit_request(default_erc_address);
    let client = Client::new();
    let uuid = Uuid::new_v4().to_string();

    // Construct the deposit endpoint URL
    let mut deposit_url = url.clone();
    deposit_url.set_path("/v1/deposit");

    let resp = client
        .post(deposit_url)
        .json(&nf3_deposit_request)
        .header("X-Request-ID", &uuid)
        .send()
        .await
        .expect("Failed to send deposit request");
    let status = resp.status();
    let text = resp.text().await.expect("Failed to read response body");
    if status.is_success() {
        println!("{}", text);
    } else {
        panic!("Deposit request failed: {}", text);
    }
}

async fn transfer(url: &url::Url, default_erc_address: &str, default_recipient_key: &str) {
    let req = prompt_nf3_transfer_request(default_erc_address, default_recipient_key);
    let mut endpoint = url.clone();
    endpoint.set_path("/v1/transfer");
    let client = reqwest::Client::new();
    let uuid = uuid::Uuid::new_v4().to_string();
    let resp = client
        .post(endpoint.as_str())
        .json(&req)
        .header("X-Request-ID", &uuid)
        .send()
        .await
        .expect("Failed to send transfer request");
    let status = resp.status();
    let text = resp.text().await.expect("Failed to read response body");
    if status.is_success() {
        println!("{}", text);
    } else {
        panic!("Transfer request failed: {}", text);
    }
}

async fn withdraw(url: &url::Url, default_erc_address: &str) {
    let req = prompt_nf3_withdraw_request(default_erc_address);
    let mut endpoint = url.clone();
    endpoint.set_path("/v1/withdraw");
    let client = reqwest::Client::new();
    let uuid = uuid::Uuid::new_v4().to_string();
    let resp = client
        .post(endpoint.as_str())
        .json(&req)
        .header("X-Request-ID", &uuid)
        .send()
        .await
        .expect("Failed to send withdraw request");
    let status = resp.status();
    let text = resp.text().await.expect("Failed to read response body");
    if status.is_success() {
        println!("{}", text);
    } else {
        panic!("Withdraw request failed: {}", text);
    }
}

fn prompt_nf3_deposit_request(default_erc_address: &str) -> NF3DepositRequest {
    let erc_address = Text::new("Enter ERC address:")
        .with_initial_value(default_erc_address)
        .prompt()
        .expect("Failed to get ERC address");
    let token_id = Text::new("Enter Token ID:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Token ID");
    let token_type = Text::new("Enter Token Type:")
        .with_initial_value("0")
        .prompt()
        .expect("Failed to get Token Type");
    let value = Text::new("Enter Value:")
        .with_initial_value("0x01")
        .prompt()
        .expect("Failed to get Value");
    let fee = Text::new("Enter Fee:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Fee");
    let deposit_fee = Text::new("Enter Deposit Fee:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Deposit Fee");
    NF3DepositRequest {
        erc_address,
        token_id,
        token_type,
        value,
        fee,
        deposit_fee,
        secret_preimage_one: None,
        secret_preimage_two: None,
        secret_preimage_three: None,
    }
}

fn prompt_nf3_transfer_request(
    default_erc_address: &str,
    default_recipient_key: &str,
) -> NF3TransferRequest {
    let erc_address = Text::new("Enter ERC address:")
        .with_initial_value(default_erc_address)
        .prompt()
        .expect("Failed to get ERC address");
    let token_id = Text::new("Enter Token ID:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Token ID");
    let value = Text::new("Enter Value:")
        .with_initial_value("0x01")
        .prompt()
        .expect("Failed to get Value");
    let recipient_key = Text::new("Enter recipient compressed ZKP public key:")
        .with_initial_value(default_recipient_key)
        .prompt()
        .expect("Failed to get recipient key")
        .trim_start_matches("0x")
        .to_string();
    let fee = Text::new("Enter Fee:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Fee");
    NF3TransferRequest {
        erc_address,
        token_id,
        recipient_data: NF3RecipientData {
            values: vec![value],
            recipient_compressed_zkp_public_keys: vec![recipient_key],
        },
        fee,
    }
}

fn prompt_nf3_withdraw_request(default_erc_address: &str) -> NF3WithdrawRequest {
    let erc_address = Text::new("Enter ERC address:")
        .with_initial_value(default_erc_address)
        .prompt()
        .expect("Failed to get ERC address");
    let token_id = Text::new("Enter Token ID:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Token ID");
    let token_type = Text::new("Enter Token Type:")
        .with_initial_value("ERC20")
        .prompt()
        .expect("Failed to get Token Type");
    let value = Text::new("Enter Value:")
        .with_initial_value("0x01")
        .prompt()
        .expect("Failed to get Value");
    let recipient_address = Text::new("Enter Recipient Address:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Recipient Address");
    let fee = Text::new("Enter Fee:")
        .with_initial_value("0x00")
        .prompt()
        .expect("Failed to get Fee");
    NF3WithdrawRequest {
        erc_address,
        token_id,
        token_type,
        value,
        recipient_address,
        fee,
    }
}

async fn check_client_connection(base_url: &Url) -> bool {
    let mut health_url = base_url.clone();
    health_url.set_path("/v1/health");
    match reqwest::get(health_url.as_str()).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}
