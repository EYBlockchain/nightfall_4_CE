use crate::test_settings::TestSettings;
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
    types::TransactionReceipt,
};
use hex::ToHex;
use jf_primitives::{
    poseidon::Poseidon,
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use lib::models::CertificateReq;
use log::{debug, info};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};
use nightfall_client::{
    domain::entities::{CommitmentStatus, DepositSecret, HexConvertible, Preimage, Salt},
    driven::{
        db::mongo::CommitmentEntry,
        plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
    },
    drivers::{
        derive_key::ZKPKeys,
        rest::models::{
            KeyRequest, NF3DepositRequest, NF3RecipientData, NF3TransferRequest,
            NF3WithdrawRequest, WithdrawDataReq,
        },
    },
    get_fee_token_id,
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
    collections::HashSet, error::Error, fmt, fmt::Display, os::unix::process::ExitStatusExt,
    path::PathBuf, process::Command,
};
use tokio::time;
use url::Url;

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
) -> Result<(), Box<dyn Error>> {
    let part_cert = Part::bytes(certificate_req.certificate.clone())
        .file_name("certificate.der")
        .mime_str("application/octet-stream")?;
    let part_priv_key = Part::bytes(certificate_req.certificate_private_key)
        .file_name("certificate.priv_key")
        .mime_str("application/octet-stream")?;
    let form = Form::new()
        .part("certificate", part_cert)
        .part("certificate_private_key", part_priv_key);
    let response = client.post(url).multipart(form).send().await?;
    if response.status() != StatusCode::ACCEPTED {
        return Err(Box::new(CertificateValidationError {
            status: response.status(),
        }));
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
pub async fn get_key(url: Url, key_request: &KeyRequest) -> Result<ZKPKeys, Box<dyn Error>> {
    // getting a key is easy; we just pass in a mnemonic and a key object is returned from the nightfall client
    let client = reqwest::Client::new();
    let res = client.post(url).json(key_request).send().await?;
    let key = res.json::<ZKPKeys>().await?;
    Ok(key)
}

pub fn load_addresses(settings: &Settings) -> Result<Addresses, AddressesError> {
    let s = Sources::File(PathBuf::from(&settings.contracts.addresses_file));
    Addresses::load(s)
}

pub fn get_recipient_address(settings: &Settings) -> Result<String, Box<dyn Error>> {
    let recipient_address = format!(
        "0x{}",
        settings
            .signing_key
            .parse::<LocalWallet>()?
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

/// this function returns a Future that resolves when all commitments in the slice are available on-chain
/// We need to know who the recipient is so we can query the correct endpoint.
pub async fn wait_on_chain(
    commitment_hashes: &[Fr254],
    recipient_url: &str,
) -> Result<(), Box<dyn Error>> {
    info!("Waiting for commitments to appear on-chain");
    // get commitment hashes to act as database keys
    // query the DB for these hashes and wait until we have found them all
    let client = reqwest::Client::new();
    let mut count = 0;
    let mut onchain_set = HashSet::<Fr254>::new();
    while count < commitment_hashes.len() {
        for hash in commitment_hashes.iter() {
            let url = Url::parse(recipient_url)?
                .join(&format!("v1/commitment/{}", hash.to_hex_string()))?;
            let res = client.get(url).send().await?;
            match res.error_for_status() {
                Ok(res) => {
                    let commit = res.json::<CommitmentEntry>().await?;
                    // we only care about commitments that are unspent
                    if commit.status != CommitmentStatus::Unspent {
                        continue;
                    }
                    let onchain_hash = commit.hash()?;
                    if onchain_set.contains(&onchain_hash) {
                        continue;
                    } else {
                        let onchain_hash = commit.hash()?;
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
pub async fn de_escrow_request(
    req: WithdrawDataReq,
    client_url: &str,
) -> Result<u8, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = Url::parse(client_url)?.join("v1/de-escrow")?;
    let res = client
        .post(url)
        .json(&serde_json::json!(req))
        .send()
        .await?;
    res.json::<u8>().await.map_err(|e| e.into())
}

/// Function to generate proof and inputs for a random transaction
pub fn create_random_proof_and_inputs(
    rng: &mut impl Rng,
) -> Result<(PlonkProof, PublicInputs, PrivateInputs), Box<dyn Error>> {
    let (mut public_inputs, mut private_inputs) = build_valid_transfer_inputs(rng);
    debug!("Generating proof");
    let proof = PlonkProvingEngine::prove(&mut private_inputs, &mut public_inputs)?;

    Ok((proof, public_inputs, private_inputs))
}

/// Function to create a deposit transaction
pub async fn create_nf3_deposit_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    deposit_fee: String,
) -> Result<(String, Option<String>), Box<dyn Error>> {
    info!("Creating deposit transaction onchain");
    let deposit_request = create_nf3_deposit_request(
        tx_details.value,
        tx_details.fee,
        deposit_fee.clone(),
        token_type,
        tx_details.token_id,
    );
    let res = client
        .post(url)
        .json(&serde_json::json!(deposit_request))
        .send()
        .await?;
    res.error_for_status_ref()?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let res_body = res.text().await?;

    // Parse the response for preimage hashes
    let preimage_hashes: Vec<String> = serde_json::from_str(&res_body)?;

    match preimage_hashes.len() {
        1 => Ok((preimage_hashes[0].clone(), None)), // One preimage hash, return value hash only
        2 => Ok((preimage_hashes[0].clone(), Some(preimage_hashes[1].clone()))), // Two preimage hashes, return both
        _ => Err(Box::<dyn std::error::Error>::from(
            "Unexpected number of preimage hashes",
        )),
    }
}

pub async fn create_nf3_transfer_transaction(
    recipient_zkp_key: ZKPKeys,
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
) -> Result<Value, Box<dyn Error>> {
    info!("Creating transfer transaction offchain");
    let transfer_request = create_nf3_transfer_request(
        recipient_zkp_key,
        tx_details.value,
        tx_details.fee,
        token_type,
        tx_details.token_id,
    )?;
    let res = client
        .post(url)
        .json(&serde_json::json!(transfer_request))
        .send()
        .await?;
    res.error_for_status_ref()?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    // extract the transaction from the response and serialize it to a ClientTransaction
    let (transaction, _) = res.json::<(Value, Option<TransactionReceipt>)>().await?;

    Ok(transaction)
}

pub async fn create_nf3_withdraw_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    recipient_address: String,
) -> Result<WithdrawDataReq, Box<dyn Error>> {
    info!("Creating withdraw transaction offchain");
    let withdraw_request = create_nf3_withdraw_request(
        recipient_address.clone(),
        tx_details.value.clone(),
        tx_details.fee.clone(),
        token_type,
        tx_details.token_id.clone(),
    );
    // Send the POST request to the API
    let res = client
        .post(url)
        .json(&serde_json::json!(withdraw_request))
        .send()
        .await?;

    // Ensure the response status is 200 OK or 202 ACCEPTED
    res.error_for_status_ref()?;
    let status = res.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::ACCEPTED,
        "Unexpected status code: {}",
        status
    );

    // Deserialize the response to get withdraw_fund_salt
    let response_body = res.json::<serde_json::Value>().await?; // Deserialize into a generic JSON structure

    // Extract the `withdraw_fund_salt` from the response
    let withdraw_fund_salt = response_body["withdraw_fund_salt"]
        .as_str()
        .ok_or("Missing or invalid withdraw_fund_salt in response")?
        .to_string();

    // Return the updated WithdrawDataReq with withdraw_fund_salt
    Ok(WithdrawDataReq {
        token_id: tx_details.token_id,
        erc_address: token_type.address(),
        recipient_address,
        value: tx_details.value,
        fee: tx_details.fee,
        token_type: token_type.to_string(),
        withdraw_fund_salt,
    })
}

pub async fn get_balance(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    token_id: String,
) -> Result<String, Box<dyn Error>> {
    let erc_address = token_type.address();
    let url = url.join(&format!("{}/{}", erc_address, token_id))?;
    let res = client.get(url).send().await?;
    res.error_for_status_ref()?;
    let balance = res.text().await?;
    Ok(balance)
}

pub async fn handle_fee_balance(
    client: &reqwest::Client,
    url: Url,
) -> Result<String, Box<dyn Error>> {
    let res = client.get(url).send().await?;
    res.error_for_status_ref()?;
    let balance = res.text().await?;
    Ok(balance)
}

pub async fn count_spent_commitments(
    client: &reqwest::Client,
    url: Url,
) -> Result<usize, Box<dyn Error>> {
    let url = url.join("commitments")?;

    let res = client.get(url).send().await?;
    res.error_for_status_ref()?;
    let count = res
        .json::<Vec<CommitmentEntry>>()
        .await?
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
) -> Result<NF3TransferRequest, Box<dyn Error>> {
    Ok(NF3TransferRequest {
        erc_address: token_type.address(),
        token_id,
        recipient_data: NF3RecipientData {
            values: vec![value],
            recipient_compressed_zkp_public_keys: vec![hex::encode(
                recipient_zkp_key.compressed_public_key()?,
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
    let nf_address = get_addresses().nightfall();
    let fee_token_id = get_fee_token_id();

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
