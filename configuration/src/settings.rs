use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{env, sync::OnceLock};

// rather than pass around what are effectively constant values, or recreate them locally,
// let's use the lazy_static crate to create a global variable that can be used to consume
// settings from anywhere in the code.
pub fn get_settings() -> &'static Settings {
    static SETTINGS: OnceLock<Settings> = OnceLock::new();
    SETTINGS.get_or_init(|| Settings::new().unwrap())
}

/// Deserializes an optional u64, treating empty strings as None.
/// This handles the case where an env var is set but empty (e.g., `NF4_NETWORK__LOG_CHUNK_SIZE=`).
fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    // First try to deserialize as Option<u64> directly
    // If that fails, try as a string and handle empty case
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU64 {
        U64(u64),
        Str(String),
        None,
    }

    match Option::<StringOrU64>::deserialize(deserializer)? {
        Some(StringOrU64::U64(n)) => Ok(Some(n)),
        Some(StringOrU64::Str(s)) if s.is_empty() => Ok(None),
        Some(StringOrU64::Str(s)) => s
            .parse::<u64>()
            .map(Some)
            .map_err(|_| de::Error::custom(format!("invalid u64 value: {s}"))),
        Some(StringOrU64::None) | None => Ok(None),
    }
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct Network {
    pub chain_id: u64,
    /// Max blocks per eth_getLogs query. if None, uses chain-specific defaults
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    pub log_chunk_size: Option<u64>,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletTypeConfig {
    #[default]
    Local,
    Azure,
    #[serde(rename = "YubiWallet")]
    YubiWallet,
    #[serde(rename = "AwsSigner")]
    AwsSigner,
    #[serde(rename = "EYTransactionManager")]
    EyTransactionManager,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct ClientConfig {
    pub url: String,
    pub log_level: String,
    pub wallet_type: WalletTypeConfig,
    pub db_url: String,
    pub max_event_listener_attempts: Option<u32>,
    pub webhook_url: String,
    pub max_queue_size: Option<u32>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct ProposerConfig {
    pub url: String,
    pub log_level: String,
    pub wallet_type: WalletTypeConfig,
    pub db_url: String,
    pub block_assembly_max_wait_secs: u64,
    pub block_assembly_target_fill_ratio: f64,
    pub block_assembly_initial_interval_secs: u64,
    pub max_event_listener_attempts: Option<u32>,
    pub block_size: u64,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct DeployerConfig {
    pub log_level: String,
    pub default_proposer_address: String,
    pub default_proposer_url: String,
    pub proposer_stake: u64,
    pub proposer_ding: u64,
    pub proposer_exit_penalty: u64,
    pub proposer_cooling_blocks: u64,
    pub proposer_rotation_blocks: u64,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct TestConfig {
    pub log_level: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct EthereumClientConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
pub struct ContractAddresses {
    pub nightfall: String,
    pub round_robin: String,
    pub x509: String,
    pub verifier: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
#[allow(unused)]
pub struct Contracts {
    pub assets: String,
    pub deployment_file: String,
    pub deploy_contracts: bool,
    pub contract_addresses: ContractAddresses,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CertificateConfig {
    pub authority_key_identifier: String,
    pub modulus: String,
    pub exponent: u64,
    pub extended_key_usages: Vec<String>,
    pub certificate_policies: Vec<String>,
}

fn default_max_key_download_bytes() -> u64 {
    30 * 1024 * 1024 * 1024 // 30 gb
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub signing_key: String,
    #[serde(default)]
    pub azure_vault_url: String,
    #[serde(default)]
    pub azure_key_name: String,
    pub log_app_only: bool,
    pub test_x509_certificates: bool,
    pub mock_prover: bool,
    pub skip_key_regeneration: Option<bool>,
    pub nightfall_client: ClientConfig,
    pub contracts: Contracts,
    pub nightfall_deployer: DeployerConfig,
    pub nightfall_proposer: ProposerConfig,
    pub network: Network,
    pub ethereum_client_url: String,
    pub nightfall_test: TestConfig,
    pub genesis_block: usize,
    pub certificates: CertificateConfig,
    pub configuration_url: String,
    pub run_mode: String,
    /// Optional upper bound
    /// If not set, default value of 30 GB is used
    #[serde(default = "default_max_key_download_bytes")]
    pub max_key_download_bytes: u64,
}

impl Settings {
    pub fn new() -> Result<Self, String> {
        let run_mode = env::var("NF4_RUN_MODE").unwrap_or_else(|_| "development".into());

        let figment = Figment::new()
            .join(("run_mode", &run_mode))
            .merge(Toml::file("nightfall.toml").nested())
            .merge(Env::prefixed("NF4_").profile(run_mode.as_str()).split("__"))
            .select(run_mode);

        let mut settings: Settings = figment.extract().map_err(|e| format!("{e}"))?;
        // Check the wallet type and read additional Azure-specific settings
        if settings.nightfall_client.wallet_type == WalletTypeConfig::Azure
            || settings.nightfall_proposer.wallet_type == WalletTypeConfig::Azure
        {
            settings.azure_vault_url = env::var("AZURE_VAULT_URL").unwrap_or_default();
            settings.azure_key_name = env::var("PROPOSER_SIGNING_KEY_NAME")
                .or_else(|_| env::var("CLIENT_SIGNING_KEY_NAME"))
                .or_else(|_| env::var("AZURE_KEY_NAME"))
                .unwrap_or_default();
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    #[test]
    #[serial]
    fn test_config() {
        let tmp_signing_key = env::var("NF4_SIGNING_KEY").ok();
        let tmp_run_mode = env::var("NF4_RUN_MODE").ok();

        env::set_var("NF4_SIGNING_KEY", "0x2a");
        env::set_var("NF4_RUN_MODE", "development");

        // Load settings *after* setting environment variables
        let s = Settings::new().unwrap();

        // Assert that the loaded settings match what we set
        assert_eq!(
            s.signing_key.as_str(),
            "0x2a",
            "The signing key should be overridden by the environment variable."
        );
        assert_eq!(
            s.run_mode, "development",
            "The run mode should be set to development."
        );

        // Clean up environment variables using match for correct restoration
        match tmp_signing_key {
            Some(val) => env::set_var("NF4_SIGNING_KEY", val),
            None => env::remove_var("NF4_SIGNING_KEY"),
        }
        match tmp_run_mode {
            Some(val) => env::set_var("NF4_RUN_MODE", val),
            None => env::remove_var("NF4_RUN_MODE"),
        }
    }

    #[test]
    #[serial]
    fn test_override() {
        let tmp_db_url = env::var("NF4_NIGHTFALL_CLIENT__DB_URL").ok();
        let tmp_run_mode = env::var("NF4_RUN_MODE").ok();

        // Set environment variables for the test
        env::set_var(
            "NF4_NIGHTFALL_CLIENT__DB_URL",
            "mongodb://nf4_db_client2:27017",
        );
        // Crucially, set NF4_RUN_MODE before Settings::new()
        env::set_var("NF4_RUN_MODE", "development");

        // Load settings *after* setting environment variables
        let s = Settings::new().unwrap();

        // Assert that the loaded setting matches what we set
        assert_eq!(
            s.nightfall_client.db_url.as_str(),
            "mongodb://nf4_db_client2:27017",
            "The DB URL should be overridden by the environment variable."
        );

        assert_eq!(
            s.run_mode, "development",
            "The run mode should be set to development."
        );

        // Clean up environment variables using match
        match tmp_db_url {
            Some(val) => env::set_var("NF4_NIGHTFALL_CLIENT__DB_URL", val),
            None => env::remove_var("NF4_NIGHTFALL_CLIENT__DB_URL"),
        }
        match tmp_run_mode {
            Some(val) => env::set_var("NF4_RUN_MODE", val),
            None => env::remove_var("NF4_RUN_MODE"),
        }
    }

    #[test]
    #[serial]
    fn test_override_with_profile() {
        // Backup original env vars
        let tmp_db_url = env::var("NF4_NIGHTFALL_CLIENT__DB_URL").ok();
        let tmp_run_mode = env::var("NF4_RUN_MODE").ok();

        // Set environment variables for the test
        env::set_var(
            "NF4_NIGHTFALL_CLIENT__DB_URL",
            "mongodb://nf4_db_client2:27017",
        );
        env::set_var("NF4_RUN_MODE", "development");

        // Load settings *after* setting environment variables
        let s = Settings::new().unwrap();

        // Assert that the loaded setting matches what we set
        assert_eq!(
            s.nightfall_client.db_url.as_str(),
            "mongodb://nf4_db_client2:27017",
            "The DB URL should be overridden by the environment variable for the 'development' profile."
        );

        assert_eq!(
            s.run_mode, "development",
            "The run mode should be set to development."
        );

        // Clean up environment variables to avoid affecting other tests
        match tmp_db_url {
            Some(val) => env::set_var("NF4_NIGHTFALL_CLIENT__DB_URL", val),
            None => env::remove_var("NF4_NIGHTFALL_CLIENT__DB_URL"),
        }

        match tmp_run_mode {
            Some(val) => env::set_var("NF4_RUN_MODE", val),
            None => env::remove_var("NF4_RUN_MODE"),
        }
    }
}
