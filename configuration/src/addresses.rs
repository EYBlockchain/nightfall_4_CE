use crate::settings::Settings;

use alloy::primitives::Address;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use log::warn;
use reqwest::{blocking, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt, fs,
    io::Read,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use toml;
use url::Url;
// rather than pass around what are effectively constant values, let's use the lazy_static crate to
// create a global variable that can be used to consume contract addresses from anywhere in the code.
pub fn get_addresses() -> &'static Addresses {
    static ADDRESSES: OnceLock<Addresses> = OnceLock::new();
    ADDRESSES.get_or_init(|| {
        let settings = Settings::new().unwrap();

        fn find(path: &Path) -> Option<PathBuf> {
            if path.is_absolute() {
                return path.is_file().then(|| path.to_path_buf());
            }
            let mut cwd = std::env::current_dir().ok()?;
            loop {
                let file_path = cwd.join(path);
                if file_path.is_file() {
                    return Some(file_path);
                }
                cwd = cwd.parent()?.to_path_buf();
            }
        }

        fn parse_addr(s: &str, what: &str) -> Address {
            let s = s.trim();
            if s.is_empty() || s.eq_ignore_ascii_case("0x0000000000000000000000000000000000000000")
            {
                panic!("Missing or zero {what} address");
            }
            s.parse()
                .unwrap_or_else(|_| panic!("Could not parse {what} address"))
        }

        // 1) Try configuration server first
        let url = Url::parse(&settings.configuration_url)
            .expect("Could not parse URL")
            .join(&settings.contracts.addresses_file)
            .expect("Could not parse addresses file location");

        if let Ok(addresses) = Addresses::load(Sources::Http(url.clone())) {
            warn!("Loaded contract addresses from configuration server: {url}");
            return addresses;
        } else {
            warn!("Failed to load from configuration server; trying other sources…");
        }

        // 2) If not deploying, and TOML has all three, use those
        let ca = &settings.contracts.contract_addresses;
        let have_all_toml_addrs =
            !ca.nightfall.is_empty() && !ca.round_robin.is_empty() && !ca.x509.is_empty() &&  !ca.verifier.is_empty();
        if !settings.contracts.deploy_contracts && have_all_toml_addrs {
            warn!("Using contract addresses from nightfall.toml file");
            return Addresses {
                nightfall: parse_addr(&ca.nightfall, "nightfall"),
                round_robin: parse_addr(&ca.round_robin, "round_robin"),
                x509: parse_addr(&ca.x509, "x509"),
                verifier: parse_addr(&ca.verifier, "verifier"),
            };
        }

        // 3) Final fallback: local file
        let path = Path::new(&settings.contracts.addresses_file);
        let source_file = find(path).expect("Could not find addresses file");
        Addresses::load(
            Sources::parse(&source_file.to_string_lossy()).expect("Could not parse addresses file"),
        )
        .expect("Could not load data from addresses file")
    })
}

#[derive(Debug)]
pub enum AddressesError {
    Settings,
    Io,
    Toml(String),
    CouldNotGetUrl,
    BadResponse,
    CouldNotWriteFile(String),
    CouldNotWriteDirectory(String),
    CouldNotReadFile,
    CouldNotPostUrl,
}

impl Error for AddressesError {}
impl fmt::Display for AddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Settings => write!(f, "Settings"),
            Self::Io => write!(f, "Io"),
            Self::Toml(s) => write!(f, "Toml: {s}"),
            Self::CouldNotGetUrl => write!(f, "CouldNotGetUrl"),
            Self::BadResponse => write!(f, "BadResponse"),
            Self::CouldNotWriteFile(s) => write!(f, "CouldNotWriteFile: {s}"),
            Self::CouldNotWriteDirectory(s) => write!(f, "CouldNotWriteDirectory: {s}"),
            Self::CouldNotReadFile => write!(f, "CouldNotReadFile"),
            Self::CouldNotPostUrl => write!(f, "CouldNotPostUrl"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Addresses {
    pub nightfall: Address,
    pub round_robin: Address,
    pub x509: Address,
    pub verifier: Address,
}

impl Addresses {
    /// Getter for the Nightfall contract address
    pub fn nightfall(&self) -> Address {
        self.nightfall
    }
}

pub enum Sources {
    Http(Url),
    File(PathBuf),
}

#[derive(Debug)]
pub enum SourcesError {
    PathDoesNotExist(String),
}

impl Error for SourcesError {}
impl fmt::Display for SourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PathDoesNotExist(s) => write!(f, "PathDoesNotExist, path: {s}"),
        }
    }
}

impl Addresses {
    pub fn load(s: Sources) -> Result<Self, AddressesError> {
        match s {
            Sources::File(p) => {
                if p.extension().unwrap() == "toml" || p.extension().unwrap() == "tmp" {
                    let addresses: Self = Figment::new()
                        .merge(Toml::file(p.as_os_str()))
                        .extract()
                        .map_err(|e| AddressesError::Toml(format!("{e}")))?;
                    Ok(addresses)
                } else {
                    let mut json_file = fs::File::open(p).map_err(|_| AddressesError::Io)?;
                    let mut json_string = String::new();
                    json_file
                        .read_to_string(&mut json_string)
                        .map_err(|_| AddressesError::CouldNotReadFile)?;
                    let v: serde_json::Value = serde_json::from_str(&json_string)
                        .map_err(|_| AddressesError::CouldNotReadFile)?;
                    let mut nightfall = Address::ZERO;
                    let mut round_robin = Address::ZERO;
                    let mut x509 = Address::ZERO;
                    let mut verifier = Address::ZERO;

                    let transaction_array = v["transactions"].as_array().unwrap();

                    for transaction in transaction_array {
                        match transaction["contractName"]
                            .as_str()
                            .ok_or(AddressesError::CouldNotReadFile)?
                        {
                            "Nightfall" => {
                                let bytes: [u8; 20] = hex::decode(
                                    transaction["contractAddress"]
                                        .as_str()
                                        .ok_or(AddressesError::CouldNotReadFile)?
                                        .trim_start_matches("0x"),
                                )
                                .map_err(|e| {
                                    AddressesError::CouldNotWriteFile(format!("hex: {e}"))
                                })?
                                .try_into()
                                .map_err(|_| AddressesError::BadResponse)?;
                                nightfall = Address::from(bytes);
                            }
                            "RoundRobin" => {
                                let bytes: [u8; 20] = hex::decode(
                                    transaction["contractAddress"]
                                        .as_str()
                                        .ok_or(AddressesError::CouldNotReadFile)?
                                        .trim_start_matches("0x"),
                                )
                                .map_err(|e| {
                                    AddressesError::CouldNotWriteFile(format!("hex: {e}"))
                                })?
                                .try_into()
                                .map_err(|_| AddressesError::BadResponse)?;
                                round_robin = Address::from(bytes);
                            }
                            "X509" => {
                                let bytes: [u8; 20] = hex::decode(
                                    transaction["contractAddress"]
                                        .as_str()
                                        .ok_or(AddressesError::CouldNotReadFile)?
                                        .trim_start_matches("0x"),
                                )
                                .map_err(|e| {
                                    AddressesError::CouldNotWriteFile(format!("hex: {e}"))
                                })?
                                .try_into()
                                .map_err(|_| AddressesError::BadResponse)?;
                                x509 = Address::from(bytes);
                            }
                            "RollupProofVerifier" => {
                                let bytes: [u8; 20] = hex::decode(
                                    transaction["contractAddress"]
                                        .as_str()
                                        .ok_or(AddressesError::CouldNotReadFile)?
                                        .trim_start_matches("0x"),
                                )
                                .map_err(|e| {
                                    AddressesError::CouldNotWriteFile(format!("hex: {e}"))
                                })?
                                .try_into()
                                .map_err(|_| AddressesError::BadResponse)?;
                                verifier = Address::from(bytes);
                            }

                            _ => continue,
                        }
                    }
                    Ok(Addresses {
                        nightfall,
                        round_robin,
                        x509,
                        verifier,
                    })
                }
            }
            Sources::Http(u) => {
                let resp = blocking::get(u).map_err(|_| AddressesError::CouldNotGetUrl)?;
                let data = resp.text().map_err(|_| AddressesError::BadResponse)?;
                let addresses: Self =
                    toml::from_str(&data).map_err(|e| AddressesError::Toml(format!("{e}")))?;
                Ok(addresses)
            }
        }
    }
    pub async fn save(self, s: Sources) -> Result<StatusCode, AddressesError> {
        match s {
            Sources::File(p) => {
                let data =
                    toml::to_string(&self).map_err(|e| AddressesError::Toml(format!("{e}")))?;
                // create a path if it doesn't exist
                if let Some(path) = p.parent() {
                    fs::create_dir_all(path)
                        .map_err(|e| AddressesError::CouldNotWriteDirectory(format!("{e}")))?;
                }
                fs::write(p, data)
                    .map_err(|e| AddressesError::CouldNotWriteFile(format!("{e}")))?;
                Ok(StatusCode::OK)
            }
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
        let p = PathBuf::from(s);
        let u = Url::parse(s);
        // if it's a valid base url, then job done
        if let Ok(x) = u {
            if s.contains("://") && !x.cannot_be_a_base() {
                return Ok(Self::Http(x));
            }
        }
        // if not, treat it as a file
        // check if a path exists to where you want to write the file
        if let Some(x) = p.parent() {
            if x != PathBuf::from("") && !x.exists() {
                return Err(SourcesError::PathDoesNotExist(
                    x.to_str().unwrap().to_string(),
                ));
            }
        }
        Ok(Self::File(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[tokio::test]
    async fn test_save_and_load() {
        const FILE: &str = "test_file.tmp";
        let addresses = Addresses {
            nightfall: Address::from(rand::thread_rng().gen::<[u8; 20]>()),
            round_robin: Address::from(rand::thread_rng().gen::<[u8; 20]>()),
            x509: Address::from(rand::thread_rng().gen::<[u8; 20]>()),
            verifier: Address::from(rand::thread_rng().gen::<[u8; 20]>()),
        };
        let address = addresses.nightfall;
        let res = addresses.save(Sources::parse(FILE).unwrap()).await.unwrap();
        assert_eq!(res, StatusCode::OK);
        let test_addresses = Addresses::load(Sources::parse(FILE).unwrap()).unwrap();
        // remove test file
        fs::remove_file(FILE).unwrap();
        assert_eq!(test_addresses.nightfall, address);
    }
}
