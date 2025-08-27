use crate::test::validate_certificate_with_server;
use lib::models::CertificateReq;
use log::info;
use std::fs;
use url::Url;

// Validate all client and proposer certificates
pub async fn validate_all_certificates<const N: usize>(
    certs: [(&'static str, &'static str, &'static str, Url); N],
    http_client: &reqwest::Client,
) {
    for (name, cert_path, key_path, url) in certs.iter() {
        info!("Validating {name}'s certificate");
        let cert =
            fs::read(cert_path).unwrap_or_else(|_| panic!("Failed to read {name} certificate"));
        let key = fs::read(key_path).unwrap_or_else(|_| panic!("Failed to read {name} priv_key"));
        let cert_req = CertificateReq {
            certificate: cert,
            certificate_private_key: key,
        };
        validate_certificate_with_server(http_client, url.clone(), cert_req)
            .await
            .unwrap_or_else(|e| panic!("{name} Certificate validation failed: {e}"));
    }
}
