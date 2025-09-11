use super::models::CertificateReq;
use crate::{
    blockchain_client::BlockchainClientConnection, error::CertificateVerificationError,
    initialisation::get_blockchain_client_connection, models::bad_request,
};
use configuration::addresses::get_addresses;
use ethers::types::{Address, H160, U256};
use futures::stream::TryStreamExt;
use log::{debug, error, trace, warn};
use nightfall_bindings::x509::{CertificateArgs, X509};
use reqwest::StatusCode;
use std::io::Read;
use warp::{filters::multipart::FormData, path, reply::Reply, Buf, Filter};
#[derive(Debug)]
pub struct X509ValidationError;

impl std::fmt::Display for X509ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "X509 certificate validation failed")
    }
}

impl std::error::Error for X509ValidationError {}

pub fn certification_validation_request(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    debug!("Received certification request");
    path!("v1" / "certification")
        .and(warp::post())
        .and(warp::multipart::form().max_length(16192))
        .and_then(handle_certificate_validation)
}

// Middleware to validate the certificate
pub async fn handle_certificate_validation(
    mut x509_data: FormData,
) -> Result<impl Reply, warp::Rejection> {
    // Parse the certificate validation request (by FIELD NAME, not filename)
    let mut certificate_req = CertificateReq::default();
    while let Some(part_res) = x509_data.try_next().await.transpose() {
        let part = part_res.map_err(|e| {
            error!("multipart read error: {e}");
            warp::reject::custom(CertificateVerificationError::new(
                "Malformed multipart form",
            ))
        })?;

        let field_name = part.name().to_string();
        let filename = part.filename().map(|s| s.to_string());

        let mut data = Vec::new();
        let mut stream = part.stream();
        while let Some(chunk) = stream.try_next().await.map_err(|e| {
            error!("stream chunk error: {e}");
            warp::reject::custom(CertificateVerificationError::new("Malformed upload stream"))
        })? {
            // `chunk` implements `Buf`
            let mut reader = chunk.reader();
            reader.read_to_end(&mut data).map_err(|e| {
                error!("read_to_end error: {e}");
                warp::reject::custom(CertificateVerificationError::new(
                    "I/O error reading upload",
                ))
            })?;
        }

        debug!(
            "Received field '{}' (filename: {:?}), size: {} bytes",
            field_name,
            filename,
            data.len()
        );

        match field_name.as_str() {
            "certificate" => certificate_req.certificate = data,
            "certificate_private_key" | "priv_key" | "private_key" => {
                certificate_req.certificate_private_key = data
            }
            _ => return Ok(bad_request("Unexpected form field")),
        }
    }

    if certificate_req.certificate.is_empty() {
        return Ok(bad_request("Missing 'certificate' field or empty file"));
    }
    if certificate_req.certificate_private_key.is_empty() {
        return Ok(bad_request("Missing 'priv_key' field or empty file"));
    }

    // 2) Resolve client + address
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let requestor_address = blockchain_client.address();
    trace!("Requestor address: {requestor_address}");

    let x509_addr = get_addresses().x509;
    let x509_instance = X509::new(x509_addr, blockchain_client.clone());

    // 3) Build signature over the requester address
    debug!("Signing ethereum address {requestor_address} with certificate private key");
    let ethereum_address_signature =
        match sign_ethereum_address(&certificate_req.certificate_private_key, &requestor_address) {
            Ok(sig) => sig,
            Err(e) => {
                error!("sign_ethereum_address failed: {e}");
                let body = warp::reply::json(&serde_json::json!({
                    "status": "ok",
                    "certified": false
                }));
                return Ok(warp::reply::with_status(body, StatusCode::ACCEPTED));
            }
        };

    // 4) READ-ONLY validation first (no state change). Treat any error as "not certified".
    let is_end_user = true; // end-entity certs coming from clients/proposers
    let check_only = true;

    if let Err(err) = validate_certificate(
        x509_addr,
        certificate_req.certificate.clone(),
        ethereum_address_signature.clone(),
        is_end_user,
        check_only, // read-only
        0,
        requestor_address,
    )
    .await
    {
        error!("Read-only certificate validation failed: {err}");
        let body = warp::reply::json(&serde_json::json!({
            "status": "ok",
            "certified": false
        }));
        return Ok(warp::reply::with_status(body, StatusCode::ACCEPTED));
    }

    // 5) ENROLL (state-changing): write the binding on-chain and await receipt.
    // We want one API that validates AND enrolls, so we do the write:
    let check_only = false;

    if let Err(err) = validate_certificate(
        x509_addr,
        certificate_req.certificate.clone(),
        ethereum_address_signature,
        is_end_user,
        check_only, // write path
        0,
        requestor_address,
    )
    .await
    {
        // If the write failed because it's already linked to this address, you may still be "certified".
        // Fall through to a fresh x_509_check to decide the final boolean.
        warn!("Enroll (write) failed: {err}");
    }

    // 6) Return POST-STATE truth from chain
    let is_certified_now = x509_instance
        .x_509_check(requestor_address)
        .call()
        .await
        .map_err(|e| {
            error!("x_509_check failed: {e}");
            warp::reject::custom(CertificateVerificationError::new(
                "Failed to query on-chain certification state",
            ))
        })?;

    let body = warp::reply::json(&serde_json::json!({
        "status": "ok",
        "certified": is_certified_now
    }));
    Ok(warp::reply::with_status(body, StatusCode::ACCEPTED))
}
// Function to perform certificate validation via smart contract
async fn validate_certificate(
    x509_contract_address: Address,
    certificate: Vec<u8>,
    ethereum_address_signature: Vec<u8>,
    is_end_user: bool,
    check_only: bool,
    oid_group: u32,
    sender_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let x509_instance = X509::new(x509_contract_address, blockchain_client.clone());

    let number_of_tlvs: U256 = x509_instance
        .compute_number_of_tlvs(certificate.clone().into(), U256::zero())
        .call()
        .await?;

    let certificate_args = CertificateArgs {
        certificate: certificate.clone().into(),
        tlv_length: number_of_tlvs,
        address_signature: ethereum_address_signature.into(),
        is_end_user,
        check_only,
        oid_group: oid_group.into(),
        addr: sender_address,
    };

    let tx_receipt = x509_instance
        .validate_certificate(certificate_args)
        .send()
        .await
        .map_err(|e| {
            warn!("{e}");
            X509ValidationError
        })?
        .await?;

    if tx_receipt.is_none() {
        error!("X509Validation transaction failed");
        return Err(Box::new(X509ValidationError));
    }
    Ok(())
}
use openssl::{
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    sign::{Signer as opensslSigner, Verifier},
};
use std::error::Error;
#[allow(dead_code)]
/// Sign an Ethereum address using an RSA private key
pub fn sign_ethereum_address(
    der_private_key: &[u8],
    address: &H160,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create an RSA object from the DER-encoded private key
    let private_key = Rsa::private_key_from_der(der_private_key)?;

    // Create a Signer object for SHA-256
    let pkey = PKey::from_rsa(private_key)?;
    let mut signer = opensslSigner::new(MessageDigest::sha256(), &pkey)?;

    // Convert the Ethereum address to bytes
    let address_bytes = address.as_bytes();

    // Sign the address bytes
    signer.update(address_bytes)?;
    let signature = signer.sign_to_vec()?;

    Ok(signature)
}

#[allow(dead_code)]
fn verify_ethereum_address_signature(
    pkey: &PKey<openssl::pkey::Public>,
    address: &H160,
    signature: &[u8],
) -> Result<bool, Box<dyn Error>> {
    // Create a Verifier object for SHA-256
    let mut verifier = Verifier::new(MessageDigest::sha256(), pkey)?;

    // Convert the Ethereum address to bytes
    let address_bytes = address.as_bytes();

    // Verify the signature
    verifier.update(address_bytes)?;
    let result = verifier.verify(signature)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;
    #[test]
    fn test_sign_and_verify_ethereum_address() {
        let der_private_key = include_bytes!(
            "../../blockchain_assets/test_contracts/X509/_certificates/user/user-1.priv_key"
        );
        let address_bytes: [u8; 20] = decode("1804c8AB1F12E6bbf3894d4083f33e07309d1f38")
            .unwrap()
            .try_into()
            .unwrap();
        let address = H160::from(address_bytes);
        let signature =
            sign_ethereum_address(der_private_key, &address).expect("Failed to sign address");
        // print signature in hex format
        ark_std::println!("Signature: {:?}", hex::encode(&signature));
        let private_key =
            Rsa::private_key_from_der(der_private_key).expect("Failed to parse private key");
        let public_key_pem = private_key
            .public_key_to_pem()
            .expect("Failed to derive public key");
        let public_key =
            PKey::public_key_from_pem(&public_key_pem).expect("Failed to create public key");
        let is_valid = verify_ethereum_address_signature(&public_key, &address, &signature.clone())
            .expect("Failed to verify signature");
        assert!(is_valid, "Signature verification failed");
    }
}
