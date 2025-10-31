use crate::settings::Settings;
use alloy::primitives::Address;
use log::{info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, ToSocketAddrs};
use std::{error::Error, fmt, sync::OnceLock};
use url::Url;
use rand::Rng;

// Note: Chain validation could be added here if accepting external addresses.
// Currently not needed as addresses come from controlled deployment/config.

/// Validates an Ethereum address with strict EIP-55 checksum enforcement.
/// Use for user configuration addresses.
pub fn validate_address(addr: &str) -> Result<Address, AddressesError> {
    let original_addr = addr;
    let addr = addr.trim_start_matches("0x");
    let addr_bytes =
        hex::decode(addr).map_err(|_| AddressesError::InvalidFormat(original_addr.into()))?;
    if addr_bytes.len() != 20 {
        return Err(AddressesError::InvalidLength(original_addr.into()));
    }
    let address = Address::from_slice(&addr_bytes);
    if address == Address::ZERO {
        return Err(AddressesError::ZeroAddress(original_addr.into()));
    }
    // Verify EIP-55 checksum
    let checksummed = address.to_checksum(None);
    if &checksummed[2..] != addr {
        return Err(AddressesError::InvalidChecksum(original_addr.into()));
    }
    Ok(address)
}

fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
        IpAddr::V6(v6) => v6.is_unique_local() || v6.is_loopback() || v6.is_unspecified(),
    }
}
fn deny_if_dns_private(host: &str) -> Result<(), AddressesError> {
    // try to resolve host:443 -> list of SocketAddr, check each IP
    for socket_addr in (host, 443)
        .to_socket_addrs()
        .map_err(|_| AddressesError::Toml(format!("DNS resolution failed for {host}")))?
    {
        if is_private_ip(socket_addr.ip()) {
            return Err(AddressesError::Toml(format!(
                "Host resolves to private IP: {host}"
            )));
        }
    }
    Ok(())
}

/// Validates configuration URLs with security enforcement.
/// Production: HTTPS. Debug: HTTP allowed for localhost/test containers.
fn validate_config_url(raw: &str) -> Result<Url, AddressesError> {
    let url = Url::parse(raw).map_err(|_| AddressesError::Toml(format!("Invalid URL: {raw}")))?;

    let host = url
        .host_str()
        .ok_or_else(|| AddressesError::Toml(format!("Configuration URL missing host: {raw}")))?;

    // Get run mode - fail if not set
    let run_mode = std::env::var("NF4_RUN_MODE").map_err(|_| {
        warn!("NF4_RUN_MODE environment variable not set");
        AddressesError::Toml("NF4_RUN_MODE environment variable must be set".into())
    })?;

    let is_development = matches!(run_mode.as_str(), "development" | "sync_test");

    if is_development {
        // Debug/dev: allow localhost and docker service "configuration"
        let ok_local = matches!(host, "localhost" | "127.0.0.1" | "::1" | "configuration");
        if !ok_local {
            return Err(AddressesError::Toml(format!(
                "Untrusted host in debug mode: {host}"
            )));
        }
    } else {
        // Production checks
        // Block raw IPs that are private/loopback
        if let Ok(ip) = host.parse::<IpAddr>() {
            // If parsing succeeds, it's an IP address - validate it
            match ip {
                IpAddr::V4(v4) => {
                    if v4.is_loopback() || v4.is_private() || v4.is_link_local() {
                        warn!("Private IPv4 not allowed in production: {host}");
                        return Err(AddressesError::Toml(format!(
                            "Private/internal IP not allowed in production: {host}"
                        )));
                    }
                }
                IpAddr::V6(v6) => {
                    if v6.is_loopback() || v6.is_unique_local() || v6.is_unspecified() {
                        return Err(AddressesError::Toml(format!(
                            "Private/Loopback IPv6 not allowed in production: {host}"
                        )));
                    }
                }
            }
        } else {
            // Production: block DNS names that resolve to private IPs
            deny_if_dns_private(host)?;
        }
    }

    // Scheme enforcement: HTTPS only in production
    if url.scheme() != "https" && !is_development {
        let scheme = url.scheme();
        warn!("HTTP not allowed in production, use HTTPS");
        return Err(AddressesError::Toml(format!(
            "Insecure scheme not allowed in production: {scheme}"
        )));
    }
    log::info!("Validated configuration URL: {url}");
    Ok(url)
}

// rather than pass around what are effectively constant values, let's use the lazy_static crate to
// create a global variable that can be used to consume contract addresses from anywhere in the code.
pub fn get_addresses() -> &'static Addresses {
    static ADDRESSES: OnceLock<Addresses> = OnceLock::new();
    ADDRESSES.get_or_init(|| {
        let settings = Settings::new().expect("Could not load settings");
        let base = validate_config_url(&settings.configuration_url).expect("Invalid or untrusted configuration URL");
        let url = base.join("addresses").expect("Could not parse addresses server endpoint");
        // Retry logic: wait for deployer to finish and save addresses
        let max_attempts = 32;
        let mut wait_time = 2;

        for attempt in 1..=max_attempts {
            match Addresses::load(Sources::Http(url.clone())) {
                Ok(addresses) => {
                    if addresses.chain_id != settings.network.chain_id {
                        panic!(
                            "Addresses chain_id {} != configured network.chain_id {}",
                            addresses.chain_id, settings.network.chain_id
                        );
                    }
                    info!("Loaded contract addresses from configuration server");
                    return addresses;
                }
                Err(e) => {
                    if attempt < max_attempts {
                        let rng = rand::thread_rng().gen_range(0..1000);
                        warn!(
                            "Attempt {attempt}/{max_attempts}: Waiting for addresses on configuration server (retry in {wait_time}s)"
                        );
                        std::thread::sleep(std::time::Duration::from_secs(wait_time) + std::time::Duration::from_millis(rng));
                        wait_time = (wait_time * 2).min(max_attempts);
                    } else {
                        panic!(
                            "Could not load contract addresses from configuration server after {max_attempts} attempts. Last error: {e:?}"
                        );
                    }
                }
            }
        }
         unreachable!()
    })
}

#[derive(Debug)]
pub enum AddressesError {
    Settings,
    Toml(String),
    CouldNotGetUrl,
    BadResponse,
    ZeroAddress(String),
    InvalidAddress { field: String, value: String },
    CouldNotPostUrl,
    InvalidFormat(String),
    InvalidLength(String),
    InvalidChecksum(String),
    InvalidDeploymentData(String),
}

impl Error for AddressesError {}
impl fmt::Display for AddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Settings => write!(f, "Settings"),
            Self::Toml(s) => write!(f, "Toml: {s}"),
            Self::CouldNotGetUrl => write!(f, "CouldNotGetUrl"),
            Self::BadResponse => write!(f, "BadResponse"),
            Self::ZeroAddress(s) => write!(f, "ZeroAddress: {s}"),
            Self::InvalidAddress { field, value } => {
                write!(f, "InvalidAddress in {field}: {value}")
            }
            Self::CouldNotPostUrl => write!(f, "CouldNotPostUrl"),
            Self::InvalidFormat(s) => write!(f, "InvalidFormat:{s}"),
            Self::InvalidLength(s) => write!(f, "InvalidLength:{s}"),
            Self::InvalidChecksum(s) => write!(f, "InvalidChecksum:{s}"),
            Self::InvalidDeploymentData(s) => write!(f, "InvalidDeploymentData:{s}"),
        }
    }
}

mod address_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes an `Address` as an EIP-55 checksummed hexadecimal string.
    /// This ensures that when the address is saved (e.g., to TOML or JSON),
    /// it preserves the checksum formatting.
    pub fn serialize<S>(addr: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let checksummed = addr.to_checksum(None);
        serializer.serialize_str(&checksummed)
    }
    /// Deserializes a string into an `Address`, validating its EIP-55 checksum.
    /// Returns an error if the string is not a valid checksummed address
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        validate_address(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Addresses {
    pub chain_id: u64,
    #[serde(with = "address_serde")]
    pub nightfall: Address,
    #[serde(with = "address_serde")]
    pub round_robin: Address,
    #[serde(with = "address_serde")]
    pub x509: Address,
}

impl Addresses {
    /// Getter for the Nightfall contract address
    pub fn nightfall(&self) -> Address {
        self.nightfall
    }
}
pub enum Sources {
    Http(Url),
}

#[derive(Debug)]
pub enum SourcesError {
    InvalidUrl(String),
}

impl Error for SourcesError {}
impl fmt::Display for SourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUrl(s) => write!(f, "InvalidUrl: {s}"),
        }
    }
}

impl Addresses {
    pub fn ensure_nonzero(&self) -> Result<(), AddressesError> {
        if self.nightfall == Address::ZERO {
            return Err(AddressesError::ZeroAddress("nightfall".into()));
        }
        if self.round_robin == Address::ZERO {
            return Err(AddressesError::ZeroAddress("round_robin".into()));
        }
        if self.x509 == Address::ZERO {
            return Err(AddressesError::ZeroAddress("x509".into()));
        }
        Ok(())
    }
    pub fn load(s: Sources) -> Result<Self, AddressesError> {
        match s {
            Sources::Http(u) => {
                let client = reqwest::blocking::Client::builder()
                    .redirect(reqwest::redirect::Policy::none())
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|_| AddressesError::CouldNotGetUrl)?;

                let resp = client
                    .get(u)
                    .send()
                    .map_err(|_| AddressesError::CouldNotGetUrl)?;

                let data = resp.text().map_err(|_| AddressesError::BadResponse)?;
                let addresses: Self =
                    toml::from_str(&data).map_err(|e| AddressesError::Toml(format!("{e}")))?;
                addresses.ensure_nonzero()?;
                Ok(addresses)
            }
        }
    }
    pub async fn save(self, s: Sources) -> Result<StatusCode, AddressesError> {
        match s {
            Sources::Http(u) => {
                let data =
                    toml::to_string(&self).map_err(|e| AddressesError::Toml(format!("{e}")))?;
                let client = reqwest::Client::new();
                let resp = client
                    .put(u)
                    .body(data)
                    .send()
                    .await
                    .map_err(|_| AddressesError::CouldNotPostUrl)?;
                Ok(resp.status())
            }
        }
    }
}

impl Sources {
    pub fn parse(s: &str) -> Result<Self, SourcesError> {
        let u = Url::parse(s);
        // If it's a valid base URL, then job done
        if let Ok(x) = u {
            if s.contains("://") && !x.cannot_be_a_base() {
                Ok(Self::Http(x))
            } else {
                Err(SourcesError::InvalidUrl(s.into()))
            }
        } else {
            Err(SourcesError::InvalidUrl(s.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    async fn test_checksum_validation() {
        // checksummed address (valid)
        let valid_address = "0x52908400098527886E0F7030069857D2E4169EE7";
        assert!(validate_address(valid_address).is_ok());

        // wrong checksum (invalid)
        let invalid_address = "0x52908400098527886e0f7030069857d2e4169ee7";
        assert!(validate_address(invalid_address).is_err());

        // Exemple TOML
        let addresses = Addresses {
            chain_id: 31337,
            nightfall: validate_address(valid_address).unwrap(),
            round_robin: validate_address(valid_address).unwrap(),
            x509: validate_address(valid_address).unwrap(),
        };

        addresses.ensure_nonzero().unwrap();
    }
    #[tokio::test]
    #[serial]
    async fn test_config_url_validation() {
        assert!(validate_config_url("http://example.com").is_err());
        assert!(validate_config_url("not-a-url").is_err());

        // Set ONCE and keep it for all development tests
        std::env::set_var("NF4_RUN_MODE", "development");
        assert!(validate_config_url("http://localhost:8080").is_ok());
        assert!(validate_config_url("http://configuration:80").is_ok());

        // Cleanup only at the very end
        std::env::remove_var("NF4_RUN_MODE");
    }

    #[tokio::test]
    #[serial]
    async fn test_path_injection_protection() {
        std::env::set_var("NF4_RUN_MODE", "development");

        // attack.com should fail
        assert!(validate_config_url("https://attack.com/configuration").is_err());
        assert!(validate_config_url("https://configuration/some/path").is_ok());

        std::env::remove_var("NF4_RUN_MODE");
    }
}
