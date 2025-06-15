use crate::{test_settings::TestSettings, TestError};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::{BigInteger, PrimeField};
use ark_std::{rand::Rng, test_rng, UniformRand};
use configuration::{
    addresses::{get_addresses, Addresses, AddressesError, Sources},
    settings::Settings,
};
use ethers::{
    signers::{LocalWallet, Signer},
    types::{BigEndianHash, Filter, H160, H256, I256},
    utils::keccak256,
};
use ethers_middleware::Middleware;
use hex::ToHex;
use jf_primitives::{
    poseidon::Poseidon,
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use lib::initialisation::get_blockchain_client_connection;
use lib::{blockchain_client::BlockchainClientConnection, models::CertificateReq};
use log::{debug, info};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};
use nightfall_client::{
    domain::{
        entities::{CommitmentStatus, DepositSecret, HexConvertible, Preimage, Salt},
        error::NightfallContractError,
        notifications::NotificationPayload,
    },
    driven::{
        db::mongo::CommitmentEntry,
        plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
    },
    drivers::{
        derive_key::ZKPKeys,
        rest::models::{
            DeEscrowDataReq, KeyRequest, NF3DepositRequest, NF3RecipientData, NF3TransferRequest,
            NF3WithdrawRequest, WithdrawDataReq,
        },
    },
    ports::{
        commitments::Commitment,
        proof::{PrivateInputs, ProvingEngine, PublicInputs},
        secret_hash::SecretHash,
    },
};
use num_bigint::BigUint;
use reqwest::{
    multipart::{Form, Part},
    StatusCode,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::Command,
    sync::Arc,
};
use tokio::{sync::Mutex, time};
use url::Url;
use uuid::Uuid;

const REQUEST_ID: &str = "X-Request-ID";

/// This function sets the mining interval to be MINING_INTERVAL and ensures that automining is off
/// We don't want to rely on the initial configuration of Anvil being correct.
pub async fn set_anvil_mining_interval(
    client: &reqwest::Client,
    url: &Url,
    interval: u32,
) -> Result<(), TestError> {
    // Disable automining
    let disable_automine = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "evm_setAutomine",
        "params": [false],
        "id": 1
    });
    client
        .post(url.clone())
        .json(&disable_automine)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;

    // Set interval mining
    let set_interval = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "evm_setIntervalMining",
        "params": [interval],
        "id": 1
    });
    client
        .post(url.clone())
        .json(&set_interval)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;

    Ok(())
}

/// This function will return the transactions in the last n blocks
pub async fn get_transactions_in_last_n_blocks(
    client: &reqwest::Client,
    url: &Url,
    n: u64,
) -> Result<Option<Value>, TestError> {
    // Get the latest block number
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });
    let res = client
        .post(url.clone())
        .json(&payload)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let res_json: Value = res
        .json()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let latest_block_hex = res_json["result"]
        .as_str()
        .ok_or_else(|| TestError::new("Failed to get latest block number".to_string()))?;
    let latest_block = u64::from_str_radix(latest_block_hex.trim_start_matches("0x"), 16)
        .map_err(|e| TestError::new(e.to_string()))?;

    let mut txs_per_block = Vec::new();

    for i in 0..n {
        let block_number = latest_block.saturating_sub(i);
        let block_number_hex = format!("0x{:x}", block_number);
        let block_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [block_number_hex, true],
            "id": 1
        });
        let block_res = client
            .post(url.clone())
            .json(&block_payload)
            .send()
            .await
            .map_err(|e| TestError::new(e.to_string()))?;
        let block_json: Value = block_res
            .json()
            .await
            .map_err(|e| TestError::new(e.to_string()))?;
        let block_obj = &block_json["result"];
        if block_obj.is_null() {
            continue;
        }
        let txs = block_obj["transactions"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        // Only include non-empty transaction arrays
        if !txs.is_empty() {
            txs_per_block.push(serde_json::json!(txs));
        }
    }
    if txs_per_block.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Array(txs_per_block)))
    }
}

/// Function to create a chain reorg on the Anvil test rpc., optionally replaying new transactions to be included in the reorg.
/// The replay boolean indicates whether to replay the transactions from the last 'depth' blocks or to leave them empty.
pub async fn anvil_reorg(
    client: &reqwest::Client,
    url: &Url,
    depth: u64,
    replay: bool,
    interval: u32,
) -> Result<(), TestError> {
    debug!(
        "Reorging the chain with depth {}, replaying blocks is set to {}",
        depth, replay
    );
    // before we do anything, we should turn mining off because if a block can be created while this function, the depth is ill-defined
    set_anvil_mining_interval(client, url, 0).await?;
    // next we'll get all the transactions that are in the last 'depth' worth of blocks, so that we can replay them
    let new_transactions = if replay {
        get_transactions_in_last_n_blocks(client, url, depth).await?
    } else {
        None
    };

    let new_transactions = if let Some(nt) = new_transactions {
        nt
    } else {
        // If no new transactions are provided, we'll provide an empty array.
        serde_json::json!([
                // transactions go here
            ])
    };
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "anvil_reorg",
        "params": [depth, new_transactions ],
        "id": 1
    });

    let res = client
        .post(url.clone())
        .json(&payload)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;

    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    // reset the mining interval
    set_anvil_mining_interval(client, url, interval).await?;
    Ok(())
}

pub async fn get_erc20_balance(http_client: &reqwest::Client, url: Url) -> i64 {
    let url = url.join("v1/balance/").unwrap();
    i64::from_str_radix(
        get_balance(http_client, url, TokenType::ERC20, "0x00".to_string())
            .await
            .unwrap()
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0)
}
pub async fn get_erc721_balance(
    http_client: &reqwest::Client,
    url: Url,
    token_id: String,
) -> Option<i64> {
    let url = url.join("v1/balance/").unwrap();
    let balance = get_balance(http_client, url, TokenType::ERC721, token_id)
        .await
        .ok()?;
    i64::from_str_radix(balance.trim_start_matches("0x"), 16).ok()
}

pub async fn get_fee_balance(http_client: &reqwest::Client, url: Url) -> i64 {
    let url = url.join("v1/fee_balance/").unwrap();
    i64::from_str_radix(
        handle_fee_balance(http_client, url)
            .await
            .unwrap()
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0)
}
#[derive(Debug)]
struct CertificateValidationError {
    status: StatusCode,
}

impl fmt::Display for CertificateValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Certificate validation failed with status: {}",
            self.status
        )
    }
}

impl Error for CertificateValidationError {}

pub async fn validate_certificate_with_server(
    client: &reqwest::Client,
    url: Url,
    certificate_req: CertificateReq,
) -> Result<(), TestError> {
    let part_cert = Part::bytes(certificate_req.certificate.clone())
        .file_name("certificate.der")
        .mime_str("application/octet-stream")
        .map_err(|e| TestError::new(e.to_string()))?;
    let part_priv_key = Part::bytes(certificate_req.certificate_private_key)
        .file_name("certificate.priv_key")
        .mime_str("application/octet-stream")
        .map_err(|e| TestError::new(e.to_string()))?;
    let form = Form::new()
        .part("certificate", part_cert)
        .part("certificate_private_key", part_priv_key);
    let response = client
        .post(url)
        .multipart(form)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    if response.status() != StatusCode::ACCEPTED {
        return Err(TestError::new(
            CertificateValidationError {
                status: response.status(),
            }
            .to_string(),
        ));
    }
    Ok(())
}
#[derive(serde::Deserialize, Clone)]

/// Define a struct to group related parameters for a deposit/transfer/withdraw transaction to avoid too many arguments clippy error
pub struct TransactionDetails {
    /// Value to be transferred/deposited/withdrawn
    pub value: String,
    /// The transaction fee to be paid by the sender. This fee is awarded to the proposer for processing the transaction. For Deposit, sender will pay fee twice, one to deposit token, and one to deposit fee which can be used for other txs.
    pub fee: String,
    /// Token ID of the transaction
    pub token_id: String,
}

/// Struct used for the four token types supported by Nightfall 4
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    ERC20,
    ERC1155,
    ERC721,
    ERC3525,
}

impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            1 => TokenType::ERC1155,
            2 => TokenType::ERC721,
            3 => TokenType::ERC3525,
            _ => TokenType::ERC20,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::ERC20 => write!(f, "0"),
            TokenType::ERC1155 => write!(f, "1"),
            TokenType::ERC721 => write!(f, "2"),
            TokenType::ERC3525 => write!(f, "3"),
        }
    }
}

impl TokenType {
    /// Gets the relevant mock contract address for the token type
    fn address(&self) -> String {
        match self {
            TokenType::ERC20 => hex::encode(TestSettings::retrieve_mock_addresses().erc20.0),
            TokenType::ERC721 => hex::encode(TestSettings::retrieve_mock_addresses().erc721.0),
            TokenType::ERC1155 => hex::encode(TestSettings::retrieve_mock_addresses().erc1155.0),
            TokenType::ERC3525 => hex::encode(TestSettings::retrieve_mock_addresses().erc3525.0),
        }
    }
}

// get a ZKP key object from the client container (client knows how to generate them)
pub async fn get_key(url: Url, key_request: &KeyRequest) -> Result<ZKPKeys, TestError> {
    // getting a key is easy; we just pass in a mnemonic and a key object is returned from the nightfall client
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(key_request)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let key = res
        .json::<ZKPKeys>()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(key)
}

pub fn load_addresses(settings: &Settings) -> Result<Addresses, AddressesError> {
    let s = Sources::File(PathBuf::from(&settings.contracts.addresses_file));
    Addresses::load(s)
}

pub fn get_recipient_address(settings: &Settings) -> Result<String, TestError> {
    let recipient_address = format!(
        "0x{}",
        settings
            .signing_key
            .parse::<LocalWallet>()
            .map_err(|e| TestError::new(e.to_string()))?
            .address()
            .encode_hex::<String>()
    );
    Ok(recipient_address)
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
pub fn forge_command(command: &[&str]) {
    let output = Command::new("forge").args(command).output();

    match output {
        Ok(o) => {
            if o.status.success() {
                info!(
                    "Command 'forge {:?}' executed successfully: {}",
                    command,
                    String::from_utf8_lossy(&o.stdout)
                );
            } else {
                panic!(
                "Command 'forge {:?}' executed with failing error code: {:?}\nStandard Output: {}\nStandard Error: {}",
                command,
                o.status.signal(),
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            }
        }
        Err(e) => {
            panic!(
                "Command 'forge {:?}' ran into an error without executing: {}",
                command, e
            );
        }
    }
}

/// This function gets the latest layer 2 block number from the responses vector that the webhook server populates
pub async fn get_latest_l2_block_number(responses: Arc<Mutex<Vec<Value>>>) -> Option<u64> {
    let responses = responses.lock().await;
    responses
        .iter()
        .map(|r| {
            // Deserialize the JSON value into a NotificationPayload
            serde_json::from_value::<NotificationPayload>(r.clone()).unwrap()
        })
        // Filter the NotificationPayloads to find the ones that are BlockchainEvents and extract the l2_block_number
        .filter_map(|r| match r {
            NotificationPayload::BlockchainEvent {
                l2_block_number, ..
            } => Some(l2_block_number.as_u64()),
            _ => None,
        })
        .max()
}

/// This function waits until a Webhook response is received for every Request ID
pub async fn wait_for_all_responses(
    large_block_deposit_ids: &[String],
    responses: Arc<Mutex<Vec<Value>>>,
) -> Vec<String> {
    // compare the response IDs with the request IDs
    // if we find everything we need, we can break out of the loop and return the json data in the responses (ignoring the ID)
    let response_data = loop {
        // get a lock on the current response vectore
        let mut response_values = responses.lock().await;
        // destructure the vector to get the responses and the request IDs
        let response_payloads = response_values
            .iter()
            .map(|r| serde_json::from_value::<NotificationPayload>(r.clone()).unwrap())
            .collect::<Vec<_>>();
        let (response_data, response_ids): (Vec<_>, Vec<_>) = response_payloads
            .iter()
            .cloned()
            .filter_map(|r| match r {
                NotificationPayload::TransactionEvent { uuid, response } => Some((
                    response,
                    serde_json::from_str::<String>(&uuid).expect("Could not parse uuid"),
                )),
                _ => None,
            })
            .unzip();
        debug!("Response IDs: {:?}", response_ids);
        debug!("Request IDs: {:?}", large_block_deposit_ids);
        info!(
            "Have {} IDs and {} processed client transactions",
            large_block_deposit_ids.len(),
            response_ids.len()
        );
        // check if the response IDs contain all the request IDs
        // we'll do a simple O(n^2) search for the request IDs in the responses as the vector is small
        let same = large_block_deposit_ids
            .iter()
            .all(|id| response_ids.contains(id));
        // if we find everything we need, we can break out of the loop and return the json data in the responses (ignoring the ID)
        if same {
            response_values.clear(); // clear the responses so we can reuse the vector
            break response_data;
        }
        // if we get here, we haven't found everything we need yet
        drop(response_values); // free the mutex lock while we wait for more responses
        time::sleep(time::Duration::from_secs(10)).await;
    };
    info!("All expected responses from the clients' webhooks have been received");
    debug!("{:#?}", response_data);
    response_data
}

/// Function to get the L1 block hash of a given layer 2 block number
pub async fn get_l1_block_hash_of_layer2_block(
    block_number: I256,
) -> Result<H256, NightfallContractError> {
    let client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let nightfall_address = get_addresses().nightfall();
    let block_topic = H256::from_uint(&block_number.into_raw());

    let filter = Filter::new()
        .address(nightfall_address)
        .event("BlockProposed(int256)")
        .topic1(block_topic);

    let logs = client
        .get_logs(&filter)
        .await
        .map_err(|e| NightfallContractError::ProviderError(format!("Provider error: {}", e)))?;

    let log = logs
        .first()
        .ok_or_else(|| NightfallContractError::BlockNotFound(block_number.as_u64()))?;

    let tx_hash = log.transaction_hash.ok_or_else(|| {
        NightfallContractError::MissingTransactionHash("Log has no transaction hash".to_string())
    })?;

    // Fetch the full transaction to get block hash
    let tx = client
        .get_transaction(tx_hash)
        .await
        .map_err(|e| {
            NightfallContractError::ProviderError(format!("get_transaction error: {}", e))
        })?
        .ok_or(NightfallContractError::TransactionNotFound(tx_hash))?;

    let block_hash = tx.block_hash.ok_or_else(|| {
        NightfallContractError::MissingTransactionHash(
            "Transaction does not have a block hash (possibly pending)".to_string(),
        )
    })?;

    ark_std::println!(
        "L2 block {} was submitted in L1 block hash: {}",
        block_number,
        block_hash
    );

    Ok(block_hash)
}

/// this function returns a Future that resolves when all commitments in the slice are available on-chain
/// We need to know who the recipient is so we can query the correct endpoint.
pub async fn wait_on_chain(
    commitment_hashes: &[Fr254],
    recipient_url: &str,
) -> Result<(), TestError> {
    info!("Waiting for commitments to appear on-chain");
    // get commitment hashes to act as database keys
    // query the DB for these hashes and wait until we have found them all
    let client = reqwest::Client::new();
    let mut count = 0;
    let mut onchain_set = HashSet::<Fr254>::new();
    while count < commitment_hashes.len() {
        for hash in commitment_hashes.iter() {
            let url = Url::parse(recipient_url)
                .map_err(|e| TestError::new(e.to_string()))?
                .join(&format!("v1/commitment/{}", hash.to_hex_string()))
                .map_err(|e| TestError::new(e.to_string()))?;
            let res = client
                .get(url)
                .header(REQUEST_ID, Uuid::new_v4().to_string())
                .send()
                .await
                .map_err(|e| TestError::new(e.to_string()))
                .map_err(|e| TestError::new(e.to_string()))?;
            match res.error_for_status() {
                Ok(res) => {
                    let commit = res
                        .json::<CommitmentEntry>()
                        .await
                        .map_err(|e| TestError::new(e.to_string()))?;
                    // we only care about commitments that are unspent
                    if commit.status != CommitmentStatus::Unspent {
                        continue;
                    }
                    let onchain_hash = commit.hash().map_err(|e| TestError::new(e.to_string()))?;
                    if onchain_set.contains(&onchain_hash) {
                        continue;
                    } else {
                        let onchain_hash =
                            commit.hash().map_err(|e| TestError::new(e.to_string()))?;
                        debug!("Commitment {} is on-chain", onchain_hash.to_hex_string());
                        count += 1;
                        onchain_set.insert(onchain_hash);
                    }
                }
                Err(err) => {
                    debug!("Waiting for commitments: {}", err.without_url());
                }
            }
        }
        time::sleep(time::Duration::from_secs(5)).await;
    }
    info!("Commitments are now on-chain");
    Ok(())
}

/// Function to submit a request to de-escrow funds after a withdraw
pub async fn de_escrow_request(req: &DeEscrowDataReq, client_url: &str) -> Result<u8, TestError> {
    let client = reqwest::Client::new();
    let url = Url::parse(client_url)
        .map_err(|e| TestError::new(e.to_string()))?
        .join("v1/de-escrow")
        .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .post(url)
        .json(&serde_json::json!(req))
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.json::<u8>()
        .await
        .map_err(|e| TestError::new(e.to_string()))
}

/// Function to generate proof and inputs for a random transaction
pub fn create_random_proof_and_inputs(
    rng: &mut impl Rng,
) -> Result<(PlonkProof, PublicInputs, PrivateInputs), TestError> {
    let (mut public_inputs, mut private_inputs) = build_valid_transfer_inputs(rng);
    debug!("Generating proof");
    let proof = PlonkProvingEngine::prove(&mut private_inputs, &mut public_inputs)
        .map_err(|e| TestError::new(e.to_string()))?;

    Ok((proof, public_inputs, private_inputs))
}

/// Function to create and send a deposit transaction, returning the ID of the transaction
pub async fn create_nf3_deposit_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    deposit_fee: String,
) -> Result<String, TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating deposit transaction onchain {}", &id);
    let deposit_request = create_nf3_deposit_request(
        tx_details.value,
        tx_details.fee,
        deposit_fee.clone(),
        token_type,
        tx_details.token_id,
    );
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(deposit_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    debug!("Sent deposit request");
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();
    info!(
        "Deposit transaction {} has been accepted by the client",
        returned_id
    );
    Ok(returned_id)
}

pub async fn create_nf3_transfer_transaction(
    recipient_zkp_key: ZKPKeys,
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
) -> Result<String, TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating transfer transaction offchain {}", &id);
    let transfer_request = create_nf3_transfer_request(
        recipient_zkp_key,
        tx_details.value,
        tx_details.fee,
        token_type,
        tx_details.token_id,
    )
    .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(transfer_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();
    info!(
        "Transfer transaction {} has been accepted by the client",
        returned_id
    );
    Ok(returned_id)
}

pub async fn create_nf3_withdraw_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    recipient_address: String,
) -> Result<(String, WithdrawDataReq), TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating withdraw transaction offchain {}", &id);
    let withdraw_request = create_nf3_withdraw_request(
        recipient_address.clone(),
        tx_details.value.clone(),
        tx_details.fee.clone(),
        token_type,
        tx_details.token_id.clone(),
    );
    // Send the POST request to the API
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(withdraw_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;

    // Ensure the response status is 200 OK or 202 ACCEPTED
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let status = res.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::ACCEPTED,
        "Unexpected status code: {}",
        status
    );
    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();

    let withdraw_data_request = WithdrawDataReq {
        token_id: tx_details.token_id,
        erc_address: token_type.address(),
        recipient_address,
        value: tx_details.value,
        fee: tx_details.fee,
        token_type: token_type.to_string(),
        withdraw_fund_salt: String::default(),
    };

    info!(
        "Withdraw transaction {} has been accepted by the client",
        returned_id
    );
    Ok((returned_id, withdraw_data_request))
}

pub async fn get_balance(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    token_id: String,
) -> Result<String, TestError> {
    let erc_address = token_type.address();
    let url = url
        .join(&format!("{}/{}", erc_address, token_id))
        .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let balance = res
        .text()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(balance)
}

pub async fn handle_fee_balance(client: &reqwest::Client, url: Url) -> Result<String, TestError> {
    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let balance = res
        .text()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(balance)
}

pub async fn count_spent_commitments(
    client: &reqwest::Client,
    url: Url,
) -> Result<usize, TestError> {
    let url = url
        .join("commitments")
        .map_err(|e| TestError::new(e.to_string()))?;

    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let count = res
        .json::<Vec<CommitmentEntry>>()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .into_iter()
        .filter(|c| c.status == CommitmentStatus::Spent)
        .count();
    Ok(count)
}

fn create_nf3_deposit_request(
    value: String,
    fee: String,
    deposit_fee: String,
    token_type: TokenType,
    token_id: String,
) -> NF3DepositRequest {
    NF3DepositRequest {
        erc_address: token_type.address(),
        token_id,
        token_type: token_type.to_string(),
        value,
        fee,
        deposit_fee,
        secret_preimage_one: None,
        secret_preimage_two: None,
        secret_preimage_three: None,
    }
}

fn create_nf3_transfer_request(
    recipient_zkp_key: ZKPKeys,
    value: String,
    fee: String,
    token_type: TokenType,
    token_id: String,
) -> Result<NF3TransferRequest, TestError> {
    Ok(NF3TransferRequest {
        erc_address: token_type.address(),
        token_id,
        recipient_data: NF3RecipientData {
            values: vec![value],
            recipient_compressed_zkp_public_keys: vec![hex::encode(
                recipient_zkp_key
                    .compressed_public_key()
                    .map_err(|e| TestError::new(e.to_string()))?,
            )],
        },
        fee,
    })
}

fn create_nf3_withdraw_request(
    recipient_address: String,
    value: String,
    fee: String,
    token_type: TokenType,
    token_id: String,
) -> NF3WithdrawRequest {
    NF3WithdrawRequest {
        erc_address: token_type.address(),
        token_id,
        token_type: token_type.to_string(),
        value,
        recipient_address,
        fee,
    }
}

fn generate_random_path(leaf_value: Fr254, rng: &mut impl Rng) -> (MembershipProof<Fr254>, Fr254) {
    let mut root = leaf_value;
    let poseidon = Poseidon::<Fr254>::new();
    let leaf_index = u32::rand(rng);
    let mut path_elements = Vec::<PathElement<Fr254>>::new();
    for i in 0..32 {
        let dir = leaf_index >> i & 1;
        let value = Fr254::rand(rng);
        if dir == 0 {
            root = poseidon.tree_hash(&[root, value]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnRight,
                value,
            })
        } else {
            root = poseidon.tree_hash(&[value, root]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnLeft,
                value,
            })
        }
    }

    (
        MembershipProof {
            node_value: leaf_value,
            sibling_path: path_elements,
            leaf_index: leaf_index as usize,
        },
        root,
    )
}

// Creates a random 96 bit element of Fr254
fn rand_96_bit(rng: &mut impl Rng) -> Fr254 {
    let random_96_bit = u128::rand(rng) % (1u128 << 96);
    Fr254::from(random_96_bit)
}

pub fn build_valid_transfer_inputs(rng: &mut impl Rng) -> (PublicInputs, PrivateInputs) {
    let mut rng_fr = test_rng();
    let token_id = Fr254::rand(&mut rng_fr);
    let erc_address = Fr254::rand(&mut rng_fr);
    let nf_slot_id = Fr254::rand(&mut rng_fr);

    let token_id_bytes = token_id.into_bigint().to_bytes_be();
    let erc_address_bytes = erc_address.into_bigint().to_bytes_be();

    // Concatenate the bytes
    let mut input_bytes = Vec::new();
    input_bytes.extend_from_slice(&erc_address_bytes);
    input_bytes.extend_from_slice(&token_id_bytes);

    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    let digest = hasher.finalize();

    // Convert digest to BigUint
    let mut nf_token_id = BigUint::from_bytes_be(&digest);

    // Right shift by 4 bits
    nf_token_id >>= 4;
    let nf_token_id = Fr254::from(nf_token_id);

    // Retrieve the fee token ID and nightfall address
    let nf_address = H160::rand(rng);
    // generate a 'random' fee token ID (we just use the keccak hash of 1)
    let fee_token_id = Fr254::from(BigUint::from_bytes_be(&keccak256([1])) >> 4);

    // Random values for fee and value
    let mut nullified_fee_one = rand_96_bit(rng);
    let mut nullified_fee_two = rand_96_bit(rng);
    let mut nullified_value_one = rand_96_bit(rng);
    let mut nullified_value_two = rand_96_bit(rng);

    let mut value = rand_96_bit(rng);
    let mut fee = rand_96_bit(rng);

    // We need to make sure the fee and value are less than the sum of the nullified fee and value.
    // We also need to ensure the change will not exceed 2^96.
    while value > (nullified_value_one + nullified_value_two)
        || (nullified_value_one + nullified_value_two) - value >= Fr254::from(1u128 << 96)
    {
        nullified_value_one = rand_96_bit(rng);
        nullified_value_two = rand_96_bit(rng);
        value = rand_96_bit(rng);
    }

    while fee > (nullified_fee_one + nullified_fee_two)
        || (nullified_fee_one + nullified_fee_two) - fee >= Fr254::from(1u128 << 96)
    {
        nullified_fee_one = rand_96_bit(rng);
        nullified_fee_two = rand_96_bit(rng);
        fee = rand_96_bit(rng);
    }

    // Generate random root key
    let root_key = Fr254::rand(rng);
    let keys = ZKPKeys::new(root_key).unwrap();

    // Generate random recipient public key
    let recipient_public_key = TEAffine::<BabyJubJub>::rand(rng);

    // Generate random ephemeral private key
    let ephemeral_key = BJJScalar::rand(rng);

    // Make commitments for nullified values
    let nullified_one = Preimage::new(
        nullified_value_one,
        nf_token_id,
        nf_slot_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    // The second token commitment nullified will be from a deposit so the public key will be the neutral point
    let deposit_secret = DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_two = Preimage::new(
        nullified_value_two,
        nf_token_id,
        nf_slot_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(deposit_secret),
    );

    // Now nullified fee tokens
    let nullified_three = Preimage::new(
        nullified_fee_one,
        fee_token_id,
        fee_token_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    let fee_deposit_secret =
        DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_four = Preimage::new(
        nullified_fee_two,
        fee_token_id,
        fee_token_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(fee_deposit_secret),
    );

    // Make membership proofs
    let spend_commitments = [
        nullified_one,
        nullified_two,
        nullified_three,
        nullified_four,
    ];
    let mut membership_proofs = vec![];
    let mut roots = vec![];
    for nullifier in spend_commitments.iter() {
        let (membership_proof, root) = generate_random_path(nullifier.hash().unwrap(), rng);
        membership_proofs.push(membership_proof);
        roots.push(root);
    }

    let mem_proofs: [MembershipProof<Fr254>; 4] = membership_proofs.try_into().unwrap();
    let roots: [Fr254; 4] = roots.try_into().unwrap();

    // Work out what the change values will be
    let value_change = nullified_value_one + nullified_value_two - value;
    let fee_change = nullified_fee_one + nullified_fee_two - fee;

    // Salts for new commitments
    let new_salts = [Salt::new_transfer_salt().get_salt(); 3];

    let public_inputs = PublicInputs::new().fee(fee).roots(&roots).build();

    let private_inputs = PrivateInputs::new()
        .nf_address(nf_address)
        .value(value)
        .nf_token_id(nf_token_id)
        .nf_slot_id(nf_slot_id)
        .ephemeral_key(ephemeral_key)
        .recipient_public_key(recipient_public_key)
        .nullifiers_values(&[
            nullified_one.get_value(),
            nullified_two.get_value(),
            nullified_three.get_value(),
            nullified_four.get_value(),
        ])
        .nullifiers_salts(&[
            nullified_one.get_salt(),
            nullified_two.get_salt(),
            nullified_three.get_salt(),
            nullified_four.get_salt(),
        ])
        .commitments_values(&[value_change, fee_change])
        .commitments_salts(&new_salts)
        .membership_proofs(&mem_proofs)
        .nullifier_key(keys.nullifier_key)
        .secret_preimages(&[
            nullified_one.get_secret_preimage().to_array(),
            nullified_two.get_secret_preimage().to_array(),
            nullified_three.get_secret_preimage().to_array(),
            nullified_four.get_secret_preimage().to_array(),
        ])
        .zkp_private_key(keys.zkp_private_key)
        .public_keys(&[
            nullified_one.get_public_key(),
            nullified_two.get_public_key(),
            nullified_three.get_public_key(),
            nullified_four.get_public_key(),
        ])
        .build();

    (public_inputs, private_inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::signers::Wallet;
    use ethers::types::{TransactionRequest, U256};
    use ethers::{
        providers::{Http, Provider},
        utils::Anvil,
    };
    use ethers_middleware::Middleware;
    use ethers_middleware::SignerMiddleware;
    use reqwest::Client;
    use std::str::FromStr;
    #[tokio::test]
    async fn test_get_transactions_in_last_n_blocks() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let url = Url::parse(&endpoint).unwrap();
        let client = Client::new();

        // Set up ethers provider and wallet
        let provider = Provider::<Http>::try_from(endpoint.clone()).unwrap();
        let wallet: Wallet<ethers::core::k256::ecdsa::SigningKey> = anvil.keys()[0].clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());
        let provider = Arc::new(SignerMiddleware::new(provider, wallet));

        // Get two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let from = accounts[0];
        let to = accounts[1];

        // Send three transactions
        let mut tx_hashes = Vec::new();
        for i in 0..3 {
            let tx = TransactionRequest::new()
                .from(from)
                .to(to)
                .value(U256::from(1_000_000_000_000_000u128 + i));
            let pending_tx = provider.send_transaction(tx, None).await.unwrap();
            let receipt = pending_tx.await.unwrap().unwrap();
            tx_hashes.push(receipt.transaction_hash);
        }

        // Wait for the blocks to be mined
        let _latest_block = provider.get_block_number().await.unwrap().as_u64();

        // Retrieve transactions in the last 3 blocks
        let txs_json = get_transactions_in_last_n_blocks(&client, &url, 3)
            .await
            .expect("Failed to get transactions")
            .expect("No transactions found");

        // Flatten the transactions from all blocks
        let mut found_hashes = Vec::new();
        for block_txs in txs_json.as_array().unwrap() {
            for tx in block_txs.as_array().unwrap() {
                let hash_str = tx["hash"].as_str().unwrap();
                let hash = ethers::types::H256::from_str(hash_str).unwrap();
                found_hashes.push(hash);
            }
        }

        // All submitted tx hashes should be present in the found_hashes
        for hash in tx_hashes {
            assert!(
                found_hashes.contains(&hash),
                "Transaction hash {:?} not found in last n blocks",
                hash
            );
        }
    }
    #[tokio::test]
    async fn test_anvil_reorg_no_replay() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        // Mine three blocks to ensure there is some chain history before reorg
        provider
            .request::<_, ()>("anvil_mine", serde_json::json!([3]))
            .await
            .unwrap();
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(3)
        );

        // get the balances of the first two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let balance1 = provider.get_balance(accounts[0], None).await.unwrap();
        let balance2 = provider.get_balance(accounts[1], None).await.unwrap();
        // transfer 0.1 ether from the first account to the second account
        let from = accounts[0];
        let to = accounts[1];
        let tx = provider
            .send_transaction(
                ethers::types::TransactionRequest::new()
                    .from(from)
                    .to(to)
                    .value(ethers::types::U256::from(100_000_000_000_000_000u128)),
                None,
            )
            .await
            .unwrap();
        // wait for the transaction to be mined
        let tx_receipt = tx.await.unwrap().unwrap();
        // check that the block number is 4
        assert_eq!(
            tx_receipt.block_number.unwrap(),
            ethers::types::U64::from(4)
        );
        // check that the node also thinks the block number is 4
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(4)
        );
        // check that the transaction is in the block
        let block = provider
            .get_block_with_txs(tx_receipt.block_number.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert!(block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));

        // Check the balances transferred after the transaction
        let new_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            new_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );

        // simulate a reorg of depth 1
        let url = Url::parse(&endpoint).unwrap();
        let client = Client::new();
        anvil_reorg(&client, &url, 1, false, 5).await.unwrap();
        // Check the block number after the reorg, it should not have changed because the reorged chain has the same depth (a property of Anvil)
        let new_block_number = provider.get_block_number().await.unwrap();
        assert_eq!(new_block_number, ethers::types::U64::from(4));
        // Check the balances after the reorg, they should be the original balances, before the transaction
        let reverted_balance1 = provider.get_balance(from, None).await.unwrap();
        let reverted_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(reverted_balance1, balance1);
        assert_eq!(reverted_balance2, balance2);
        // Check that the transaction is no longer in the block
        let block = provider
            .get_block_with_txs(new_block_number)
            .await
            .unwrap()
            .unwrap();
        assert!(!block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));
    }

    #[tokio::test]
    async fn test_anvil_reorg_with_replay() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        // Mine three blocks to ensure there is some chain history before reorg
        provider
            .request::<_, ()>("anvil_mine", serde_json::json!([3]))
            .await
            .unwrap();
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(3)
        );

        // Get the balances of the first two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let balance1 = provider.get_balance(accounts[0], None).await.unwrap();
        let balance2 = provider.get_balance(accounts[1], None).await.unwrap();

        // Transfer 0.1 ether from the first account to the second account
        let from = accounts[0];
        let to = accounts[1];
        let tx = provider
            .send_transaction(
                ethers::types::TransactionRequest::new()
                    .from(from)
                    .to(to)
                    .value(ethers::types::U256::from(100_000_000_000_000_000u128)),
                None,
            )
            .await
            .unwrap();
        // Wait for the transaction to be mined
        let tx_receipt = tx.await.unwrap().unwrap();
        // Check that the block number is 4
        assert_eq!(
            tx_receipt.block_number.unwrap(),
            ethers::types::U64::from(4)
        );
        // Check that the node also thinks the block number is 4
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(4)
        );
        // Check that the transaction is in the block
        let block = provider
            .get_block_with_txs(tx_receipt.block_number.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert!(block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));

        // Check the balances transferred after the transaction
        let new_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            new_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );

        // Simulate a reorg of depth 1 with replay = true
        let url = Url::parse(&endpoint).unwrap();
        ark_std::println!("url: {:?}", url);
        let client = Client::new();
        anvil_reorg(&client, &url, 1, true, 5).await.unwrap();

        // Check the block number after the reorg, it should not have changed because the reorged chain has the same depth (a property of Anvil)
        let new_block_number = provider.get_block_number().await.unwrap();
        assert_eq!(new_block_number, ethers::types::U64::from(4));

        // Check the balances after the reorg, they should be the same as after the transaction, since the tx was replayed
        let replayed_balance1 = provider.get_balance(from, None).await.unwrap();
        let replayed_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            replayed_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );
        // The sender's balance should be less than or equal to the original balance (due to gas costs)
        assert!(replayed_balance1 < balance1);

        // Check that the transaction is still in the block (replayed)
        let block = provider
            .get_block_with_txs(new_block_number)
            .await
            .unwrap()
            .unwrap();
        let found = block.transactions.iter().any(|t| {
            t.to == Some(to) && t.value == ethers::types::U256::from(100_000_000_000_000_000u128)
        });
        assert!(
            found,
            "Replayed transaction not found in block after reorg with replay"
        );
    }
}
