use super::models::CertificateReq;
use crate::{
    blockchain_client::BlockchainClientConnection, error::CertificateVerificationError,
    initialisation::get_blockchain_client_connection,
};
use configuration::addresses::get_addresses;
use ethers::types::{Address, H160, U256};
use futures::stream::TryStreamExt;
use log::{debug, error, info, trace, warn};
use nightfall_bindings::x509::{CertificateArgs, X509};
use reqwest::StatusCode;
use std::{ffi::OsStr, io::Read, path::Path};
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
async fn handle_certificate_validation(
    mut x509_data: FormData,
) -> Result<impl Reply, warp::Rejection> {
    // get a blockchain client from the singleton lazy static
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();

    // Parse the certificate validation request
    let mut certificate_req = CertificateReq::default();
    while let Ok(Some(part)) = x509_data.try_next().await {
        let filename = match part.filename() {
            Some(filename) => filename.to_string(),
            None => return Ok(StatusCode::BAD_REQUEST),
        };
        info!("Receiving certificate validation file: {}", filename);

        let mut data = Vec::new();
        let mut stream = part.stream();
        while let Ok(Some(chunk)) = stream.try_next().await {
            chunk.reader().read_to_end(&mut data).unwrap();
        }
        debug!("File size: {} bytes", data.len());
        match Path::new(&filename).extension().and_then(OsStr::to_str) {
            Some("der") => certificate_req.certificate = data,
            Some("priv_key") => certificate_req.certificate_private_key = data,
            _ => return Ok(StatusCode::BAD_REQUEST),
        }
    }

    let requestor_address = blockchain_client.address();
    trace!("Requestor address: {}", requestor_address);
    let x509_instance = X509::new(get_addresses().x509, blockchain_client.clone());
    let is_certified: bool = x509_instance
        .x_509_check(requestor_address)
        .call()
        .await
        .map_err(|e| {
            error!("x_509_check transaction failed {}", e);
            warp::reject::custom(CertificateVerificationError::new(
                "Invalid certificate provided",
            ))
        })?;
    if !is_certified {
        // compute the signature and validate the certificate
        debug!(
            "Signing ethereum address {} with certificate private key",
            requestor_address
        );
        let ethereum_address_signature =
            sign_ethereum_address(&certificate_req.certificate_private_key, &requestor_address)
                .map_err(|e| {
                    error!(
                        "Could not sign ethereum address with certificate private key: {}",
                        e
                    );
                    warp::reject::custom(CertificateVerificationError::new(
                        "Invalid certificate provided",
                    ))
                })?;
        validate_certificate(
            get_addresses().x509,
            certificate_req.certificate,
            ethereum_address_signature,
            true,
            false,
            0,
            blockchain_client.address(),
        )
        .await
        .map_err(|err| {
            error!("Certificate or signature verification failed: {}", err);
            warp::reject::custom(CertificateVerificationError::new(
                "Invalid certificate provided",
            ))
        })?;
    }
    debug!("Certificate validation successful");
    Ok(StatusCode::ACCEPTED)
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
            warn!("{}", e);
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
