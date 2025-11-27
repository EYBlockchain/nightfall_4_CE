use std::{fmt, time::Duration};

use alloy_rlp::BytesMut;
/// A module containing uncategorised functions used by more than one component
use configuration::settings::get_settings;
use futures::StreamExt;
use log::{debug, info, warn};
use url::Url;
use warp::hyper::body::Bytes;
use tokio::{runtime::Handle, task::block_in_place};
use serde::ser::StdError;

use crate::error::ConfigError;

// log progress every 100 MB during key downloads
const DOWNLOAD_PROGRESS_LOG_INTERVAL_BYTES: u64 = 100 * 1024 * 1024;

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

#[derive(Debug)]
pub enum KeyDownloadError {
    Http(reqwest::Error),
    Status(reqwest::StatusCode),
    SizeLimit { actual: u64, limit: u64 },
}

impl fmt::Display for KeyDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyDownloadError::Http(e) => write!(f, "HTTP Error: {e}"),
            KeyDownloadError::Status(status_code) => write!(f, "Server returned status {status_code}"),
            KeyDownloadError::SizeLimit { actual, limit } => write!(f, "Download exceeded size limit: {actual} bytes > {limit} bytes"),
        }
    }
}

impl StdError for KeyDownloadError {}

impl From<reqwest::Error> for KeyDownloadError {
    fn from(e: reqwest::Error) -> Self {
        KeyDownloadError::Http(e)
    }
}

struct KeyDownloader {
    client: reqwest::Client,
    base_url: Url,
    max_bytes: u64,
}

impl KeyDownloader {
    fn new() -> Self {
        let settings = get_settings();
        let base_url = Url::parse(&settings.configuration_url).expect("Invalid configuration_url");

        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client");

        Self { client, base_url, max_bytes: settings.max_key_download_bytes }
    }

    async fn download(&self, key_name: &str) -> Result<Bytes, KeyDownloadError> {
        let url = self.base_url.join(key_name).expect("Failed to combine key path on to configuration_url");
        info!("Downloading key '{key_name}' from {url}");

        let res = self.client.get(url).send().await?;
        let status = res.status();
        if !status.is_success() {
            warn!("Key download failed with HTTP status {status}");
            return Err(KeyDownloadError::Status(status));
        }

        let mut stream = res.bytes_stream();
        let mut buf = BytesMut::new();
        let mut total: u64 = 0;
        let mut last_log: u64 = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            total += chunk.len() as u64;

            if total > self.max_bytes {
                warn!("key download exceeded the configured limit ({} > {}) bytes", total, self.max_bytes);
                return Err(KeyDownloadError::SizeLimit { actual: total, limit: self.max_bytes });
            }

            if total - last_log > DOWNLOAD_PROGRESS_LOG_INTERVAL_BYTES {
                debug!("Downloaded {} MB of key data so far", total / (1024 * 1024));
                last_log = total;
            }
            buf.extend_from_slice(&chunk);
        }
        info!("Downloaded key '{key_name}': {total} bytes");
        Ok(buf.freeze())

    }
}

async fn load_key_from_server_async(key_file: &str) -> Result<Bytes, KeyDownloadError> {
    let downloader = KeyDownloader::new();
    downloader.download(key_file).await
}

/// function to pull the proving key or deposit proving key from the server as a byte array
pub fn load_key_from_server(key_file: &str) -> Option<Bytes> {
    // if we are inside a tokio runtime
    if let Ok(handle) = Handle::try_current() {
        return block_in_place(|| {
            handle
                .block_on(async { load_key_from_server_async(key_file).await.ok() })
        });
    }
    None
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
