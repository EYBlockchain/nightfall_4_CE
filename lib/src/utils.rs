/// A module containing uncategorised functions used by more than one component
use configuration::settings::{get_settings, Settings};
use reqwest::blocking;
use url::Url;
use warp::hyper::body::Bytes;

use crate::error::ConfigError;

/// Fetch the block size from the nightfall toml and ensure it's an allowed number
pub fn get_block_size() -> Result<usize, ConfigError> {
    let settings = get_settings();
    // get the block size from the environment, if it's not set, default to 64
    let block_size = settings.nightfall_proposer.block_size;
    // Allowed block sizes: 64, 256
    match block_size {
        // safe to unwrap as we know it's a usize
        64 | 256 => Ok(block_size.try_into().unwrap()),
        _ => Err(ConfigError::InvalidBlockSize(
            "Block size must be one of 64 or 256".to_string(),
        )),
    }
}

/// function to pull the proving key or deposit proving key from the server as a byte array
pub fn load_key_from_server(key_file: &str) -> Option<Bytes> {
    let settings = Settings::new().expect("Could not get settings");
    let url = Url::parse(&settings.configuration_url)
        .expect("Could not parse URL")
        .join(key_file)
        .unwrap();
    let response = blocking::get(url).ok()?;
    let bytes = response.bytes().ok()?;
    Some(bytes)
}

/// function to drop a database
pub async fn drop_database(db_url: &str, db_name: &str) -> Result<(), mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(db_url).await?;
    client.database(db_name).drop().await
}

/// function to drop a collection
pub async fn drop_collection<C: Send + Sync>(
    db_url: &str,
    db_name: &str,
    collection_name: &str,
) -> Result<(), mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(db_url).await?;
    client
        .database(db_name)
        .collection::<C>(collection_name)
        .drop()
        .await
}
