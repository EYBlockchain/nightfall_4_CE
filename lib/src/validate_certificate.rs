use super::models::CertificateReq;
use crate::{
    blockchain_client::BlockchainClientConnection, error::CertificateVerificationError,
    initialisation::get_blockchain_client_connection, models::bad_request,
    verify_contract::VerifiedContracts,
};
use alloy::primitives::{Address, U256};
use configuration::addresses::get_addresses;
use futures::stream::TryStreamExt;
use log::{debug, error, trace, warn};
use nightfall_bindings::artifacts::X509;
use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::{Id as PKeyId, PKey},
    rsa::{Padding, Rsa},
    sign::{RsaPssSaltlen, Signer as opensslSigner, Verifier},
    x509::X509 as OpensslX509,
};
use reqwest::StatusCode;
use std::error::Error;
use std::io::Read;
use warp::{filters::multipart::FormData, path, reply::Reply, Buf, Filter};
use x509_parser::nom::AsBytes;
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

    // 1.5) Client-side prevalidation of certificate + private key
    // We deliberately fail fast here and do NOT call the smart contract
    // for obviously invalid / weak material.
    let prevalidation_address = {
        // We do not yet have the blockchain client, but the requestor address is
        // exactly what will be bound, so we need it here anyway.
        let conn_guard = get_blockchain_client_connection().await;
        let read_conn = conn_guard.read().await;
        read_conn.get_address()
    };

    let x509_addr = get_addresses().x509;

    // Resolve client
    let client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();

    let chain_id: u64 = client.get_chain_id().await.map_err(|e| {
        error!("Failed to get chain ID: {e}");
        warp::reject::custom(CertificateVerificationError::new("Failed to get chain ID"))
    })?;

    if let Err(e) = prevalidate_certificate_and_key(
        &certificate_req.certificate,
        &certificate_req.certificate_private_key,
        &prevalidation_address,
        &x509_addr,
        chain_id,
    ) {
        warn!("Client-side certificate prevalidation failed: {e}");
        return Ok(bad_request(
            "Certificate / private key prevalidation failed",
        ));
    }

    // 2) Resolve address
    let blockchain_client = client.root();
    let requestor_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address();
    trace!("Requestor address: {requestor_address}");

    let verified =
        VerifiedContracts::resolve_and_verify_contract(blockchain_client.clone(), get_addresses())
            .await
            .map_err(|e| {
                error!("Contract verification failed: {e}");
                warp::reject::custom(CertificateVerificationError::new(
                    "Failed to verify contract implementation",
                ))
            })?;
    let x509_instance = verified.x509;

    // 3) Build signature over the requester address
    debug!("Signing ethereum address {requestor_address} with certificate private key");
    let ethereum_address_signature = match sign_ethereum_address(
        &certificate_req.certificate_private_key,
        &requestor_address,
        &x509_addr,
        chain_id,
    ) {
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
        .x509Check(requestor_address)
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
    certificate: Vec<u8>,
    ethereum_address_signature: Vec<u8>,
    is_end_user: bool,
    check_only: bool,
    oid_group: u32,
    sender_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let read_connection = get_blockchain_client_connection().await.read().await;
    let provider = read_connection.get_client();
    let blockchain_client = provider.root();
    let caller = read_connection.get_address();
    let verified =
        VerifiedContracts::resolve_and_verify_contract(blockchain_client.clone(), get_addresses())
            .await
            .map_err(|e| {
                error!("Contract verification failed: {e}");
                Box::new(X509ValidationError) as Box<dyn std::error::Error>
            })?;
    let x509_instance = verified.x509;

    let compute_result = x509_instance
        .computeNumberOfTlvs(certificate.clone().into(), U256::ZERO)
        .call()
        .await?;
    let number_of_tlvs: U256 = compute_result; // Convert computeNumberOfTlvsReturn to U256

    let certificate_args = X509::CertificateArgs {
        certificate: certificate.clone().into(),
        tlvLength: number_of_tlvs,
        addressSignature: ethereum_address_signature.into(),
        isEndUser: is_end_user,
        checkOnly: check_only,
        oidGroup: U256::from(oid_group),
        addr: sender_address,
    };

    let tx_receipt = x509_instance
        .validateCertificate(certificate_args)
        .from(caller)
        .send()
        .await
        .map_err(|e| {
            warn!("{e}");
            X509ValidationError
        })?;
    if tx_receipt.get_receipt().await.is_err() {
        error!("X509Validation transaction failed");
        return Err(Box::new(X509ValidationError));
    }
    Ok(())
}

#[allow(dead_code)]
/// Sign an Ethereum address using an RSA private key
pub fn sign_ethereum_address(
    der_private_key: &[u8],
    address: &Address,
    verifying_contract: &Address,
    chain_id: u64,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create an RSA object from the DER-encoded private key
    let private_key = Rsa::private_key_from_der(der_private_key)?;

    let pkey = PKey::from_rsa(private_key)?;

    let mut signer = opensslSigner::new(MessageDigest::sha256(), &pkey)?;
    signer.set_rsa_padding(Padding::PKCS1_PSS)?;
    signer.set_rsa_mgf1_md(MessageDigest::sha256())?;
    signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;

    // Minimal domain separation: human-readable, versioned, bound to contract + chain
    // preimage = "ADDR-LINK|v1|contract:" || verifying_contract || "|chainId:" || u64_be || "|addr:" || address
    const PREFIX: &[u8] = b"ADDR-LINK|v1|contract:";
    const SEP_CHAIN: &[u8] = b"|chainId:";
    const SEP_ADDR: &[u8] = b"|addr:";
    let mut preimage =
        Vec::with_capacity(PREFIX.len() + 20 + SEP_CHAIN.len() + 8 + SEP_ADDR.len() + 20);
    preimage.extend_from_slice(PREFIX);
    preimage.extend_from_slice(verifying_contract.as_bytes()); // 20 bytes
    preimage.extend_from_slice(SEP_CHAIN);
    preimage.extend_from_slice(&chain_id.to_be_bytes()); // 8 bytes, big-endian
    preimage.extend_from_slice(SEP_ADDR);
    preimage.extend_from_slice(address.as_bytes()); // 20 bytes

    // Sign the address bytes
    signer.update(&preimage)?;
    let signature = signer.sign_to_vec()?;

    Ok(signature)
}

// Convenience alias so we do not keep constructing Box<dyn Error> in the handler
fn prevalidation_error(msg: &str) -> CertificateVerificationError {
    CertificateVerificationError::new(msg)
}

/// Cheap, client-side sanity checks before going on-chain.
///
/// * parses the certificate
/// * checks not_before / not_after against current time
/// * enforces a minimal key size for RSA keys
/// * checks that the private key corresponds to the certificate public key
fn prevalidate_certificate_and_key(
    cert_der: &[u8],
    priv_der: &[u8],
    address: &Address,
    verifying_contract: &Address,
    chain_id: u64,
) -> Result<(), CertificateVerificationError> {
    // 1) Parse certificate
    let cert = OpensslX509::from_der(cert_der).map_err(|e| {
        error!("X.509 parse error: {e}");
        prevalidation_error("Invalid X.509 certificate (DER parsing failed)")
    })?;

    // 2) Validity window: not_before <= now <= not_after
    let now = Asn1Time::days_from_now(0).map_err(|e| {
        error!("Asn1Time::days_from_now error: {e}");
        prevalidation_error("Internal time error while checking certificate validity")
    })?;

    if cert.not_before() > now {
        return Err(prevalidation_error(
            "Certificate is not yet valid (not_before is in the future)",
        ));
    }
    if cert.not_after() < now {
        return Err(prevalidation_error(
            "Certificate has expired (not_after is in the past)",
        ));
    }

    // 3) Key strength (only RSA for now; extend if needed)
    let pubkey = cert.public_key().map_err(|e| {
        error!("Failed to extract public key from certificate: {e}");
        prevalidation_error("Cannot extract public key from certificate")
    })?;

    if pubkey.id() == PKeyId::RSA {
        let rsa_pub = pubkey.rsa().map_err(|e| {
            error!("Failed to convert public key to RSA: {e}");
            prevalidation_error("Invalid RSA public key inside certificate")
        })?;
        // rsa_pub.size() returns modulus length in bytes
        if rsa_pub.size() < 2048 / 8 {
            return Err(prevalidation_error(
                "RSA key too short (must be at least 2048 bits)",
            ));
        }
    }

    // 4) Check that the private key is usable and matches the certificate

    // 4a) Parse private key *syntax*
    let _rsa_priv = Rsa::private_key_from_der(priv_der).map_err(|e| {
        error!("Private key parse (DER) failed: {e}");
        prevalidation_error("Invalid RSA private key (DER parsing failed)")
    })?;
    // If you support EC keys as well, add analogous parsing here.

    // 4b) Check that private key actually corresponds to the certificate public key.
    //
    // We already have `sign_ethereum_address` and `verify_ethereum_address_signature`,
    // so reuse them on the *same* address as the one we are about to bind on-chain.
    let sig =
        sign_ethereum_address(priv_der, address, verifying_contract, chain_id).map_err(|e| {
            error!("Prevalidation signing failed: {e}");
            prevalidation_error("Failed to sign Ethereum address with supplied private key")
        })?;

    let is_valid =
        verify_ethereum_address_signature(&pubkey, address, &sig, verifying_contract, chain_id)
            .map_err(|e| {
                error!("Prevalidation verification failed: {e}");
                prevalidation_error("Failed to verify signature with certificate public key")
            })?;

    if !is_valid {
        return Err(prevalidation_error(
            "Certificate public key does not match supplied private key",
        ));
    }

    Ok(())
}

#[allow(dead_code)]
fn verify_ethereum_address_signature(
    pkey: &PKey<openssl::pkey::Public>,
    address: &Address,
    signature: &[u8],
    verifying_contract: &Address,
    chain_id: u64,
) -> Result<bool, Box<dyn Error>> {
    // Create a Verifier object for SHA-256
    let mut verifier = Verifier::new(MessageDigest::sha256(), pkey)?;

    verifier.set_rsa_padding(Padding::PKCS1_PSS)?;
    verifier.set_rsa_mgf1_md(MessageDigest::sha256())?;
    verifier.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;

    // Minimal domain separation: human-readable, versioned, bound to contract + chain
    // preimage = "ADDR-LINK|v1|contract:" || verifying_contract || "|chainId:" || u64_be || "|addr:" || address
    const PREFIX: &[u8] = b"ADDR-LINK|v1|contract:";
    const SEP_CHAIN: &[u8] = b"|chainId:";
    const SEP_ADDR: &[u8] = b"|addr:";
    let mut preimage =
        Vec::with_capacity(PREFIX.len() + 20 + SEP_CHAIN.len() + 8 + SEP_ADDR.len() + 20);
    preimage.extend_from_slice(PREFIX);
    preimage.extend_from_slice(verifying_contract.as_bytes()); // 20 bytes
    preimage.extend_from_slice(SEP_CHAIN);
    preimage.extend_from_slice(&chain_id.to_be_bytes()); // 8 bytes, big-endian
    preimage.extend_from_slice(SEP_ADDR);
    preimage.extend_from_slice(address.as_bytes()); // 20 bytes

    // Verify the signature over the structured preimage
    verifier.update(&preimage)?;

    let result = verifier.verify(signature)?; // expects same PKCS#1 v1.5 structure

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use hex::decode;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;
    #[test]
    fn test_sign_and_verify_ethereum_address() {
        let der_private_key = include_bytes!(
            "../../blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key"
        );
        let verifying_contract =
            Address::from_str("0xF62849F9A0B5Bf2913b396098F7c7019b51A820a").unwrap();
        let address_bytes: [u8; 20] = decode("1804c8AB1F12E6bbf3894d4083f33e07309d1f38")
            .unwrap()
            .try_into()
            .unwrap();
        let address = Address::from(address_bytes);

        let chain_id = 31337;

        let signature =
            sign_ethereum_address(der_private_key, &address, &verifying_contract, chain_id)
                .expect("Failed to sign address");
        // print signature in hex format
        ark_std::println!("Signature: {:?}", hex::encode(&signature));
        let private_key =
            Rsa::private_key_from_der(der_private_key).expect("Failed to parse private key");
        let public_key_pem = private_key
            .public_key_to_pem()
            .expect("Failed to derive public key");
        let public_key =
            PKey::public_key_from_pem(&public_key_pem).expect("Failed to create public key");
        let is_valid = verify_ethereum_address_signature(
            &public_key,
            &address,
            &signature.clone(),
            &verifying_contract,
            chain_id,
        )
        .expect("Failed to verify signature");
        assert!(is_valid, "Signature verification failed");
    }
}
