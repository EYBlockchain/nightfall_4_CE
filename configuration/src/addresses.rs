use crate::settings::ContractAddresses;
use crate::settings::Settings;

use ethers::types::Address;

use figment::providers::Format;
use figment::providers::Toml;
use figment::Figment;
use log::warn;
use reqwest::blocking;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;
use toml;
use url::Url;

// rather than pass around what are effectively constant values, let's use the lazy_static crate to
// create a global variable that can be used to consume contract addresses from anywhere in the code.
pub fn get_addresses() -> &'static Addresses {
    static ADDRESSES: OnceLock<Addresses> = OnceLock::new();
    ADDRESSES.get_or_init(|| {
        let settings = Settings::new().unwrap();
        // if we've specificed the contract addresses in the nightfall.toml file and we aren't trying to deploy new contracts, 
        // we'll use the addresses from the configuration file (nightfall.toml) rather than try read them from the server or the addresses.toml file.
        // This makes things simpler when we already have deployed contracts and we're just connecting to them: we can treat them like static
        // configuration values.
        if settings.contracts.contract_addresses != ContractAddresses::default() && !settings.contracts.deploy_contracts {
            warn!("Using contract addresses from nightfall.toml file");
            return Addresses {
                nightfall: settings.contracts.contract_addresses.nightfall.parse().expect("Could not parse nightfall contract address"),
                round_robin: settings.contracts.contract_addresses.round_robin.parse().expect("Could not parse round robin contract address"),
                x509: settings.contracts.contract_addresses.x509.parse().expect("Could not parse x509 contract address"),
            };
        }
        fn find(path: &Path) -> Option<PathBuf> {
            if path.is_absolute() {
                match path.is_file() {
                    true => return Some(path.to_path_buf()),
                    false => return None,
                }
            }
            let cwd = std::env::current_dir().ok()?;
            let mut cwd = cwd.as_path();
            loop {
                let file_path = cwd.join(path);
                if file_path.is_file() {
                    return Some(file_path);
                }

                cwd = cwd.parent()?;
            }
        }

        // We'll try to load from the configuration server first. If that fails we'll try to load from the local file.
        let url = Url::parse(&settings.configuration_url).expect("Could not parse URL").join(&settings.contracts.addresses_file).expect("Could not parse addresses file location");
        match Addresses::load(Sources::Http(url)) {
            Ok(addresses) => addresses,
            Err(_) => {
                warn!("Failed to load the contract addresses from the configuration server. Loading from local file system instead.");
                let path = Path::new(&settings.contracts.addresses_file);
                let source_file = find(path).expect("Could not find addresses file");
                Addresses::load(Sources::parse(&source_file.to_string_lossy()).expect("Could not parse addresses file")).expect("Could not load data from addresses file")
            }
        }
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
    CouldNotReadFile,
    CouldNotPostUrl,
}

impl Error for AddressesError {}
impl fmt::Display for AddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Settings => write!(f, "Settings"),
            Self::Io => write!(f, "Io"),
            Self::Toml(s) => write!(f, "Toml: {}", s),
            Self::CouldNotGetUrl => write!(f, "CouldNotGetUrl"),
            Self::BadResponse => write!(f, "BadResponse"),
            Self::CouldNotWriteFile(s) => write!(f, "CouldNotWriteFile: {}", s),
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
            Self::PathDoesNotExist(s) => write!(f, "PathDoesNotExist, path: {}", s),
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
                        .map_err(|e| AddressesError::Toml(format!("{}", e)))?;
                    Ok(addresses)
                } else {
                    let mut json_file = fs::File::open(p).map_err(|_| AddressesError::Io)?;
                    let mut json_string = String::new();
                    json_file
                        .read_to_string(&mut json_string)
                        .map_err(|_| AddressesError::CouldNotReadFile)?;
                    let v: serde_json::Value = serde_json::from_str(&json_string)
                        .map_err(|_| AddressesError::CouldNotReadFile)?;
                    let mut nightfall = Address::zero();
                    let mut round_robin = Address::zero();
                    let mut x509 = Address::zero();

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
                                    AddressesError::CouldNotWriteFile(format!("hex: {}", e))
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
                                    AddressesError::CouldNotWriteFile(format!("hex: {}", e))
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
                                    AddressesError::CouldNotWriteFile(format!("hex: {}", e))
                                })?
                                .try_into()
                                .map_err(|_| AddressesError::BadResponse)?;
                                x509 = Address::from(bytes);
                            }

                            _ => continue,
                        }
                    }
                    Ok(Addresses {
                        nightfall,
                        round_robin,
                        x509,
                    })
                }
            }
            Sources::Http(u) => {
                let resp = blocking::get(u).map_err(|_| AddressesError::CouldNotGetUrl)?;
                let data = resp.text().map_err(|_| AddressesError::BadResponse)?;
                let addresses: Self =
                    toml::from_str(&data).map_err(|e| AddressesError::Toml(format!("{}", e)))?;
                Ok(addresses)
            }
        }
    }
    pub async fn save(self, s: Sources) -> Result<StatusCode, AddressesError> {
        match s {
            Sources::File(p) => {
                let data =
                    toml::to_string(&self).map_err(|e| AddressesError::Toml(format!("{}", e)))?;
                fs::write(p, data)
                    .map_err(|e| AddressesError::CouldNotWriteFile(format!("{}", e)))?;
                Ok(StatusCode::OK)
            }
            Sources::Http(u) => {
                let data =
                    toml::to_string(&self).map_err(|e| AddressesError::Toml(format!("{}", e)))?;
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
    #[tokio::test]
    async fn test_save_and_load() {
        const FILE: &str = "test_file.tmp";
        let addresses = Addresses {
            nightfall: Address::random(),
            round_robin: Address::random(),
            x509: Address::random(),
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
