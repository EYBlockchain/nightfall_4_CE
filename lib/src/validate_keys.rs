//! # Verification Key Validation Module
//!
//! This module provides functionality to validate the integrity and consistency of
//! zero-knowledge proof verification keys used in the Nightfall protocol by the Client and Proposer. It ensures that:
//!
//! 1. Keys stored on the key server have been generated honestly by the deployer.
//! 2. On-chain verification keys have been generated honestly by the deployer.
//!
//! The validation process is critical for the client and proposer to trust the soundness, given we do not assume
//!  that the deployer is honest. If the deployer was malicious they could have generated incorrect keys which could
//! cause security vulnerabilities.

use crate::{
    blockchain_client::BlockchainClientConnection,
    circuit_key_generation::{generate_rollup_keys_for_production, universal_setup_for_production},
    constants::MAX_KZG_DEGREE,
    deposit_circuit::deposit_circuit_builder,
    entities::DepositData,
    error::KeyVerificationError,
    initialisation::get_blockchain_client_connection,
    nf_client_proof::PublicInputs,
    utils::get_block_size,
};
use alloy::primitives::{B256, U256};
use anyhow::{Context, Result};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Write};
use configuration::{
    addresses::get_addresses,
    settings::{self},
};
use futures::{stream, StreamExt, TryStreamExt};
use jf_plonk::{
    nightfall::{ipa_structs::VerificationKeyId, FFTPlonk},
    proof_system::{
        structs::{VerifyingKey, VK},
        UniversalSNARK,
    },
};
use jf_primitives::{pcs::prelude::UnivariateKzgPCS, rescue::sponge::RescueCRHF};
use log::{debug, error, info};
use nightfall_bindings::artifacts::{RollupProofVerifier, VKHashProvider};
use reqwest::{Client, StatusCode};
use sha3::{Digest, Keccak256};
use std::{
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};
use warp::hyper::body::Bytes;
use warp::{path, reply::Reply, Filter};

// User-supplied parameters parsed from JSON body.
#[derive(Debug, serde::Deserialize)]
struct FormParams {
    configuration_url: String,
    #[serde(default = "default_concurrency")]
    concurrency: usize, // concurrency level for downloads
}

fn default_concurrency() -> usize {
    2
}

#[derive(Clone, Debug)]
/// Specification for each key to be validated
struct KeySpec {
    name: String,      // e.g., "decider_pk"
    url: String,       // e.g., "{configuration_url}/decider_pk"
    out_path: PathBuf, // e.g., "configuration/bin/decider_pk"
}

#[derive(Debug, Clone, serde::Serialize)]
/// Report for each key after validation
struct KeyReport {
    name: String,
    path: String,
    keccak256: String,
    bytes: u64,
    fresh_download: bool, // false if resumed
}

#[derive(Debug, serde::Serialize)]
/// Overall validation response
struct ValidationResponse {
    status: bool,
    configuration_url: String,
    keys: Vec<KeyReport>,
    download_comparisons: Vec<DownloadComparison>,
    onchain_comparison: Option<OnchainComparison>,
}
/// Side information about a key (downloaded or generated)
#[derive(Debug, serde::Serialize)]
struct SideInfo {
    path: String,
    keccak256: String,
    bytes: u64,
}
/// Comparison result between downloaded and locally generated key
#[derive(Debug, serde::Serialize)]
struct DownloadComparison {
    name: String,
    downloaded: Option<SideInfo>,
    generated: Option<SideInfo>,
    equal: bool,
}

/// Comparison result between contract and locally generated decider verification key
#[derive(Debug, serde::Serialize)]
struct OnchainComparison {
    onchain: Option<String>,
    generated: Option<String>,
    equal: bool,
}

///curl -sS -X POST http://localhost:3000/v1/keys_validation \
///  -H "Content-Type: application/json" \
///  -d '{"configuration_url":"http://configuration:80","concurrency":2}'
pub fn keys_validation_request(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    debug!("Creating keys_validation_request filter");
    path!("v1" / "keys_validation")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_keys_validation)
}

// Middleware to validate the verification keys
async fn handle_keys_validation(params: FormParams) -> Result<impl Reply, warp::Rejection> {
    debug!("Handling keys validation");
    // 1) Parse inputs

    let configuration_url: String = params.configuration_url.clone();

    // 2) Load settings
    let settings = match settings::Settings::new() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load settings: {e}");
            return Err(warp::reject::custom(KeyVerificationError::new(
                "Error loading settings for verification key validation.",
            )));
        }
    };
    if settings.mock_prover {
        error!("Mock prover is enabled");
        return Err(warp::reject::custom(KeyVerificationError::new(
            "Mock prover is enabled for verification key validation",
        )));
    }

    // 3) Fetch + hash all keys from configuration server (streaming, resumable, bounded concurrency)
    // Where to store downloads
    let out_dir = PathBuf::from("configuration").join("bin");
    let (spec, keys) = fetch_and_hash_keys(&configuration_url, &out_dir, params.concurrency)
        .await
        .map_err(|e| {
            let msg = format!("download/verify failed: {e:#}");
            error!("{msg}");
            warp::reject::custom(KeyVerificationError::new(&msg))
        })?;

    // 4) Delete and regenerate all proving keys from scratch to ensure they are correct
    // Note: This is computationally expensive and can be skipped in test environments
    let current_dir = std::env::current_dir().map_err(|e| {
        error!("Failed to get current directory: {e}");
        warp::reject::custom(KeyVerificationError::new("Error getting current directory"))
    })?;
    if !settings.skip_key_regeneration.unwrap_or(false) {
        delete_existing_key_files(&current_dir, spec.clone())?;
        regenerate_keys_for_production()?;
    } else {
        info!("Skipping key regeneration due to configuration setting");
    }

    // 5) Verify that those freshly generated locally stored keys match those on the key server
    let mut resp = verify_server_vs_stored_keys(keys.clone(), configuration_url).await?;
    let download_status = resp.status;

    // 6) Validate that the on-chain decider verification key hash matches the regenerated decider verification key hash
    let onchain_comparison = validate_on_chain_decider_vk(keys.clone()).await?;
    let onchain_status = onchain_comparison.equal;
    resp.onchain_comparison = Some(onchain_comparison);
    resp.status = download_status && onchain_status;

    if download_status && onchain_status {
        debug!("Keys validation successful");
        Ok(warp::reply::json(&resp))
    } else if !download_status && onchain_status {
        error!(
            "Verification failed - keys from the config server are incorrect. Response: {resp:?}"
        );
        return Err(warp::reject::custom(KeyVerificationError::new(
            "Verification failed - keys from the config server are incorrect",
        )));
    } else if download_status && !onchain_status {
        error!("Verification failed - on-chain decider VK does not match regenerated local VK. Response: {resp:?}");
        return Err(warp::reject::custom(KeyVerificationError::new(
            "Verification failed - on-chain decider VK does not match regenerated local VK",
        )));
    } else {
        error!("Verification failed - keys from the config server and onchain decider vk are incorrect. Response: {resp:?}.");
        return Err(warp::reject::custom(KeyVerificationError::new("Verification failed - keys from the config server and onchain decider vk are incorrect")));
    }
}

// Fetch and hash all keys specified by configuration_url, saving them to out_dir.
async fn fetch_and_hash_keys(
    configuration_url: &str,
    out_dir: &Path,
    concurrency: usize,
) -> Result<(Vec<KeySpec>, Vec<KeyReport>)> {
    let client = Client::builder()
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .pool_idle_timeout(Duration::from_secs(90))
        .http2_adaptive_window(true)
        .build()?;

    let specs = build_key_specs(configuration_url, out_dir)?;

    let results = stream::iter(specs.clone().into_iter().map(|spec| {
        let client = client.clone();
        async move {
            let (h, n, fresh) =
                download_with_resume_and_hash(&client, &spec.url, &spec.out_path).await?;
            Ok::<_, anyhow::Error>(KeyReport {
                name: spec.name,
                path: spec.out_path.display().to_string(),
                keccak256: h,
                bytes: n,
                fresh_download: fresh,
            })
        }
    }))
    .buffer_unordered(concurrency)
    .try_collect::<Vec<_>>()
    .await?;

    Ok((specs, results))
}

/// Download `url` to `dest` with resume when possible, hashing Keccak-256 over the entire file.
/// Returns (hex_keccak256, total_bytes_after_download, fresh_download_flag).
pub async fn download_with_resume_and_hash(
    client: &Client,
    url: &str,
    dest: &Path,
) -> Result<(String, u64, bool)> {
    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .await
            .with_context(|| format!("creating parent dir for {}", dest.display()))?;
    }

    // Open/create file (async)
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open(dest)
        .await
        .with_context(|| format!("open {dest:?}"))?;

    // Determine current size for resume
    let mut existing_len = file.metadata().await?.len();
    file.seek(std::io::SeekFrom::Start(existing_len)).await?;

    // HEAD (best effort) to detect length + range support
    let head = client.head(url).send().await.ok();
    let mut accept_ranges = false;
    let mut content_length: Option<u64> = None;
    if let Some(h) = head {
        if h.status().is_success() {
            if let Some(v) = h.headers().get(reqwest::header::ACCEPT_RANGES) {
                if let Ok(s) = v.to_str() {
                    accept_ranges = s.eq_ignore_ascii_case("bytes");
                }
            }
            if let Some(v) = h.headers().get(reqwest::header::CONTENT_LENGTH) {
                if let Ok(s) = v.to_str() {
                    content_length = s.parse::<u64>().ok();
                }
            }
        }
    }

    // If already complete by length, hash and return
    if let Some(len) = content_length {
        if existing_len == len {
            let mut hasher = Keccak256::new();
            let mut f = fs::File::open(dest).await?;
            let mut buf = vec![0u8; 1 << 20];
            loop {
                let n = f.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            let hash_hex = hex::encode(hasher.finalize());
            return Ok((hash_hex, existing_len, false));
        }
    }

    // Build GET; add Range if we have partial and server supports resume
    let mut req = client.get(url);
    if accept_ranges && existing_len > 0 {
        req = req.header(reqwest::header::RANGE, format!("bytes={existing_len}-"));
    } else if existing_len > 0 {
        // Cannot resume; restart from scratch
        file.set_len(0).await?;
        file.seek(std::io::SeekFrom::Start(0)).await?;
        existing_len = 0;
    }

    let mut resp = req.send().await.with_context(|| format!("GET {url}"))?;
    match resp.status() {
        StatusCode::OK | StatusCode::PARTIAL_CONTENT => {}
        s => anyhow::bail!("unexpected HTTP status {s} for {url}"),
    }

    // Prepare hasher; if resuming, pre-hash the existing on-disk prefix
    let mut hasher = Keccak256::new();
    if existing_len > 0 {
        let mut prev = fs::File::open(dest).await?;
        prev.seek(std::io::SeekFrom::Start(0)).await?;
        let mut buf = vec![0u8; 1 << 20];
        loop {
            let n = prev.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        // ensure our writable handle is at the end
        file.seek(std::io::SeekFrom::Start(existing_len)).await?;
    }

    // Chunked copy: read each network chunk, write to disk, update hash

    let mut total = existing_len;
    loop {
        let opt = resp.chunk().await?;
        let chunk = match opt {
            Some(c) => c,
            None => break, // EOF
        };

        file.write_all(&chunk).await?;
        hasher.update(&chunk);
        total += chunk.len() as u64;
    }
    file.flush().await?;

    let hash_hex = hex::encode(hasher.finalize());
    let fresh = existing_len == 0;
    Ok((hash_hex, total, fresh))
}

// Decide how many merge keys each curve needs for a given block size.
fn merge_counts(block_size: usize) -> anyhow::Result<(usize, usize)> {
    // If block size == 64, we will have:
    // merge_bn254_pk_0,
    // merge_grumpkin_pk_0, merge_grumpkin_pk_1

    // If block size == 256, we will have:
    // merge_bn254_pk_0, merge_bn254_pk_1,
    // merge_grumpkin_pk_0, merge_grumpkin_pk_1, merge_grumpkin_pk_2

    match block_size {
        64 => Ok((1, 2)), // (bn254_count, grumpkin_count)
        256 => Ok((2, 3)),
        _ => anyhow::bail!("Unsupported block size: {block_size}"),
    }
}

// Append merge key specs to the provided specs vector based on block size.
fn push_merge_specs(
    specs: &mut Vec<KeySpec>,
    configuration_url: &str,
    out_dir: &Path,
    block_size: usize,
) -> anyhow::Result<()> {
    let (bn_cnt, gr_cnt) = merge_counts(block_size)?;

    // Single loop over (curve, count) pairs; indices are generated uniformly.
    for (curve, count) in [("bn254", bn_cnt), ("grumpkin", gr_cnt)] {
        specs.extend((0..count).map(|i| {
            let name = format!("merge_{curve}_pk_{i}");
            KeySpec {
                url: format!("{configuration_url}/{name}"),
                out_path: out_dir.join(&name),
                name,
            }
        }));
    }
    Ok(())
}

// Build the list of KeySpecs to validate based on configuration_url and max_merge_depth base on block_size
fn build_key_specs(configuration_url: &str, out_dir: &Path) -> anyhow::Result<Vec<KeySpec>> {
    let mut specs = vec![
        KeySpec {
            name: "decider_pk".into(),
            url: format!("{configuration_url}/decider_pk"),
            out_path: out_dir.join("decider_pk"),
        },
        KeySpec {
            name: "decider_vk".into(),
            url: format!("{configuration_url}/decider_vk"),
            out_path: out_dir.join("decider_vk"),
        },
        KeySpec {
            name: "base_bn254_pk".into(),
            url: format!("{configuration_url}/base_bn254_pk"),
            out_path: out_dir.join("base_bn254_pk"),
        },
        KeySpec {
            name: "base_grumpkin_pk".into(),
            url: format!("{configuration_url}/base_grumpkin_pk"),
            out_path: out_dir.join("base_grumpkin_pk"),
        },
    ];

    let block_size = get_block_size().context("Failed to get block size")?;
    push_merge_specs(&mut specs, configuration_url, out_dir, block_size)?;
    Ok(specs)
}

/// Verifies that locally stored keys regenerated above match those retrieved from the key server.
async fn verify_server_vs_stored_keys(
    keys: Vec<KeyReport>,
    configuration_url: String,
) -> Result<ValidationResponse, warp::Rejection> {
    // After generating new rollup keys, re-hash the files IN PLACE at the same paths.
    //      We use the `keys` vector as the authoritative list of (name, path) to check.
    let download_comparisons = compare_overwritten_files(&keys).await.map_err(|e| {
        let msg = format!("post-generation hashing failed: {e:#}");
        error!("{msg}");
        warp::reject::custom(KeyVerificationError::new(&msg))
    })?;

    // Respond with both the downloaded snapshot (`keys`) and the comparison results
    let resp = ValidationResponse {
        status: download_comparisons.iter().all(|c| c.equal),
        configuration_url,
        keys,                 // downloaded snapshot (pre-generation)
        download_comparisons, // regenerated vs downloaded
        onchain_comparison: None,
    };
    Ok(resp)
}

/// Compare overwritten files specified by `keys`, producing a list of DownloadComparison results.
async fn compare_overwritten_files(keys: &[KeyReport]) -> Result<Vec<DownloadComparison>> {
    let mut comps = Vec::with_capacity(keys.len());

    for k in keys {
        let path = PathBuf::from(&k.path);

        // If a key wasn’t regenerated (e.g., generator doesn’t emit that artifact yet),
        // we mark generated=None instead of failing the whole request.
        let generated_side = match fs::try_exists(&path).await {
            Ok(true) => {
                let (h, n) = keccak256_file_async(&path).await?;
                Some(SideInfo {
                    path: path.display().to_string(),
                    keccak256: h,
                    bytes: n,
                })
            }
            _ => None,
        };

        let downloaded_side = SideInfo {
            path: k.path.clone(),
            keccak256: k.keccak256.clone(),
            bytes: k.bytes,
        };

        let equal = match &generated_side {
            Some(gen) => gen.keccak256 == downloaded_side.keccak256, // (optionally also check size)
            None => false,
        };

        comps.push(DownloadComparison {
            name: k.name.clone(),
            downloaded: Some(downloaded_side),
            generated: generated_side,
            equal,
        });
    }

    Ok(comps)
}

/// Compute Keccak-256 hash of a file at `path`.
async fn keccak256_file_async(path: &Path) -> Result<(String, u64)> {
    let mut f = fs::File::open(path)
        .await
        .with_context(|| format!("open {}", path.display()))?;
    let mut hasher = Keccak256::new();
    let mut buf = vec![0u8; 1 << 20]; // 1 MiB
    let mut total = 0u64;
    loop {
        let n = f.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        total += n as u64;
    }
    Ok((hex::encode(hasher.finalize()), total))
}

/// Validates that the decider verification key hash from the blockchain matches the local hash.
async fn validate_on_chain_decider_vk(
    keys: Vec<KeyReport>,
) -> Result<OnchainComparison, warp::Rejection> {
    let client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let blockchain_client = client.root();

    // Get the RollupProofVerifier contract address and create contract instance
    let vk_addr = get_addresses().verifier;
    let rollup_verifier = RollupProofVerifier::new(vk_addr, blockchain_client.clone());

    // Get the vkProvider address (this is the RollupProofVerificationKey contract address)
    let vk_provider_address = rollup_verifier.vkProvider().call().await.map_err(|e| {
        error!("Error calling vkProvider on verifier contract: {e}");
        warp::reject::custom(KeyVerificationError::new(
            "Error calling vkProvider on verifier contract",
        ))
    })?;

    // Create contract instance to call vkHash
    let vk_hash_provider = VKHashProvider::new(vk_provider_address, blockchain_client.clone());

    // Query the on-chain decider verification key hash
    let decider_vk_on_chain_hash: B256 = vk_hash_provider.vkHash().call().await.map_err(|e| {
        error!("Failed to call vkHash: {e}");
        warp::reject::custom(KeyVerificationError::new(
            "Error calling vkHash on vk provider contract",
        ))
    })?;
    let decider_vk_on_chain_hash_bytes: Bytes = Bytes::from(decider_vk_on_chain_hash.to_vec());

    // Get the path of the regenerated local decider vk
    let decider_vk = keys
        .iter()
        .find(|key| key.name == "decider_vk")
        .ok_or_else(|| {
            error!("decider_vk not found in keys vector");
            warp::reject::custom(KeyVerificationError::new(
                "decider_vk not found in keys vector",
            ))
        })?;

    // Read the VK file
    let vk_path = PathBuf::from(&decider_vk.path);
    let vk_file_bytes = std::fs::read(&vk_path).map_err(|e| {
        error!("Failed to read VK file at {}: {}", vk_path.display(), e);
        warp::reject::custom(KeyVerificationError::new(&format!(
            "Failed to read VK file: {e}"
        )))
    })?;

    // Deserialize the VK from the binary file
    let vk: VerifyingKey<Bn254> = CanonicalDeserialize::deserialize_compressed(&vk_file_bytes[..])
        .map_err(|e| {
            error!("Failed to deserialize VK: {e}");
            warp::reject::custom(KeyVerificationError::new(&format!(
                "Failed to deserialize VK: {e}"
            )))
        })?;

    // Convert VK to Solidity ABI format and compute hash
    let local_vk_hash_bytes = convert_vk_to_solidity_abi_hash(&vk).map_err(|e| {
        error!("Failed to convert VK to Solidity ABI hash: {e}");
        warp::reject::custom(KeyVerificationError::new(&format!(
            "Failed to convert VK to Solidity ABI hash: {e}"
        )))
    })?;

    let hashes_match = local_vk_hash_bytes == decider_vk_on_chain_hash_bytes;

    Ok(OnchainComparison {
        onchain: Some(hex::encode(decider_vk_on_chain_hash_bytes.clone())),
        generated: Some(hex::encode(local_vk_hash_bytes.clone())),
        equal: hashes_match,
    })
}

/// Converts a VerifyingKey to Solidity ABI-encoded format and computes its Keccak256 hash.
/// This matches the on-chain vkHash computation: keccak256(abi.encode(vk)).
fn convert_vk_to_solidity_abi_hash(vk: &VerifyingKey<Bn254>) -> Result<Bytes, String> {
    // Convert VK to Vec<Fq254> to access all fields
    let vk_vec_fq = Vec::<Fq254>::from(vk.clone());

    // Convert each Fq254 to U256
    let vk_vec_u256: Vec<U256> = vk_vec_fq
        .into_iter()
        .map(|x| {
            let bytes: [u8; 32] = x
                .into_bigint()
                .to_bytes_le()
                .try_into()
                .map_err(|_| "Failed to convert Vec<u8> to [u8; 32]".to_string())?;
            Ok(U256::from_le_bytes::<32>(bytes))
        })
        .collect::<Result<Vec<_>, String>>()?;

    // The VK fields are laid out in the vec as follows (matching Types.sol order):
    // 0: domain_size
    // 1: num_inputs
    // 2-13: sigma_comms (6 G1Points = 12 U256s)
    // 14-49: selector_comms (18 G1Points = 36 U256s)
    // 50-55: k1-k6
    // 56-57: range_table_comm (G1Point)
    // 58-59: key_table_comm (G1Point)
    // 60-61: table_dom_sep_comm (G1Point)
    // 62-63: q_dom_sep_comm (G1Point)
    // 64-65: open_key_g (G1Point)
    // 66-69: h (G2Point, note: order is x1,x2,y1,y2 but ABI needs x0,x1,y0,y1)
    // 70-73: beta_h (G2Point)

    // Build the complete field array in order of the VerificationKey struct from contracts
    let mut vk_fields = Vec::with_capacity(77);

    let domain_size = vk.domain_size();
    let domain_size_fr = Fr254::from(domain_size as u32);
    let domain_size_inv = U256::from_le_bytes::<32>(
        domain_size_fr
            .inverse()
            .ok_or("Failed to compute inverse of domain size")?
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .map_err(|_| "Failed to convert domain_size_inv to [u8; 32]")?,
    );
    let domain = Radix2EvaluationDomain::<Fr254>::new(domain_size)
        .ok_or("Failed to create Radix2EvaluationDomain - domain size must be a power of 2")?;
    let size_inv = domain_size_inv;
    let group_gen = U256::from_le_bytes::<32>(
        domain
            .group_gen()
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .map_err(|_| "Failed to convert group_gen to [u8; 32]")?,
    );
    let group_gen_inv = U256::from_le_bytes::<32>(
        domain
            .group_gen_inv()
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .map_err(|_| "Failed to convert group_gen_inv to [u8; 32]")?,
    );

    // domain_size, num_inputs
    vk_fields.push(U256::from(domain_size));
    vk_fields.push(U256::from(vk.num_inputs()));

    // sigma_comms 1-6 (each is G1Point with x, y)
    for i in 0..6 {
        let comm = &vk.sigma_comms[i];
        let x = U256::from_be_bytes::<32>(
            comm.x
                .into_bigint()
                .to_bytes_be()
                .try_into()
                .map_err(|_| format!("Failed to convert sigma[{i}] x to bytes"))?,
        );
        let y = U256::from_be_bytes::<32>(
            comm.y
                .into_bigint()
                .to_bytes_be()
                .try_into()
                .map_err(|_| format!("Failed to convert sigma[{i}] y to bytes"))?,
        );
        vk_fields.push(x);
        vk_fields.push(y);
    }

    // selector_comms 1-18
    for i in 0..18 {
        let comm = &vk.selector_comms[i];
        let x = U256::from_be_bytes::<32>(
            comm.x
                .into_bigint()
                .to_bytes_be()
                .try_into()
                .map_err(|_| format!("Failed to convert selector[{i}] x to bytes"))?,
        );
        let y = U256::from_be_bytes::<32>(
            comm.y
                .into_bigint()
                .to_bytes_be()
                .try_into()
                .map_err(|_| format!("Failed to convert selector[{i}] y to bytes"))?,
        );
        vk_fields.push(x);
        vk_fields.push(y);
    }

    // k1-k6
    for i in 0..6 {
        let k = U256::from_be_bytes::<32>(
            vk.k[i]
                .into_bigint()
                .to_bytes_be()
                .try_into()
                .map_err(|_| format!("Failed to convert k[{i}] to bytes"))?,
        );
        vk_fields.push(k);
    }

    // plookup commitments
    vk_fields.push(vk_vec_u256[56]); // range_table_comm.x
    vk_fields.push(vk_vec_u256[57]); // range_table_comm.y
    vk_fields.push(vk_vec_u256[58]); // key_table_comm.x
    vk_fields.push(vk_vec_u256[59]); // key_table_comm.y
    vk_fields.push(vk_vec_u256[60]); // table_dom_sep_comm.x
    vk_fields.push(vk_vec_u256[61]); // table_dom_sep_comm.y
    vk_fields.push(vk_vec_u256[62]); // q_dom_sep_comm.x
    vk_fields.push(vk_vec_u256[63]); // q_dom_sep_comm.y

    vk_fields.push(size_inv);
    vk_fields.push(group_gen);
    vk_fields.push(group_gen_inv);

    // open_key_g
    vk_fields.push(vk_vec_u256[64]); // x
    vk_fields.push(vk_vec_u256[65]); // y

    // h (G2Point) - reorder from (x1,x2,y1,y2) to (x0,x1,y0,y1)
    vk_fields.push(vk_vec_u256[67]); // x0 (was x1)
    vk_fields.push(vk_vec_u256[66]); // x1 (was x2)
    vk_fields.push(vk_vec_u256[69]); // y0 (was y1)
    vk_fields.push(vk_vec_u256[68]); // y1 (was y2)

    // beta_h (G2Point)
    vk_fields.push(vk_vec_u256[71]); // x0
    vk_fields.push(vk_vec_u256[70]); // x1
    vk_fields.push(vk_vec_u256[73]); // y0
    vk_fields.push(vk_vec_u256[72]); // y1

    // ABI-encode the VK struct
    let vk_encoded = abi_encode_verification_key(&vk_fields);

    // Hash the encoded VK
    let mut hasher = Keccak256::new();
    hasher.update(&vk_encoded);
    let local_vk_hash = hasher.finalize();
    Ok(Bytes::from(local_vk_hash.to_vec()))
}

/// Manually ABI-encodes a VerificationKey struct by encoding each field as uint256 (32 bytes each).
/// This follows Solidity's ABI encoding where all struct fields are tightly packed in order.
/// Each G1Point (x, y) becomes 2 x uint256, G2Point (x0, x1, y0, y1) becomes 4 x uint256.
fn abi_encode_verification_key(vk_fields: &[U256]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(vk_fields.len() * 32);

    for field in vk_fields {
        // Each U256 is encoded as 32 bytes (big-endian)
        encoded.extend_from_slice(&field.to_be_bytes::<32>());
    }

    encoded
}

/// This function ensures that all cryptographic keys are generated fresh from the
/// trusted setup parameters, eliminating any possibility of using stale or corrupted keys.
/// This operation is computationally expensive
/// and should only be run in production validation scenarios.
fn regenerate_keys_for_production() -> Result<(), warp::Rejection> {
    // We need to perform trusted setup first.
    let kzg_srs = universal_setup_for_production(MAX_KZG_DEGREE).map_err(|e| {
        error!(
            "Failed to perform universal trusted setup for production: {e}"
        );
        warp::reject::custom(KeyVerificationError::new(
            "Error performing universal trusted setup for production",
        ))
    })?;

    // We prepare some dummy deposit data and later rollup them to build rollup keys.
    let deposit_data = [DepositData::default(); 4];
    let mut deposit_public_inputs = PublicInputs::new();
    let mut deposit_circuit = deposit_circuit_builder(&deposit_data, &mut deposit_public_inputs)
        .map_err(|e| warp::reject::custom(KeyVerificationError::from(e)))?;
    deposit_circuit
        .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
        .map_err(|e| warp::reject::custom(KeyVerificationError::from(e)))?;

    let (deposit_pk, _) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(
        &kzg_srs,
        Some(VerificationKeyId::Deposit),
        &deposit_circuit,
        true,
    )
    .map_err(|e| {
        error!("Failed to preprocess deposit circuit: {e}");
        warp::reject::custom(KeyVerificationError::new(
            "Error preprocessing deposit circuit",
        ))
    })?;

    let path = std::env::current_dir()
        .map_err(|e| {
            error!("Failed to get current directory: {e}");
            warp::reject::custom(KeyVerificationError::new("Error getting current directory"))
        })?
        .as_path()
        .join("configuration");
    let deposit_pk_path = path.join("bin/test/deposit_proving_key");

    let mut file = File::create(deposit_pk_path.clone()).map_err(|e| {
        error!("Failed to create deposit proving key file: {e}");
        warp::reject::custom(KeyVerificationError::new(
            "Error creating deposit proving key file",
        ))
    })?;
    let mut deposit_compressed_bytes = Vec::new();
    deposit_pk
        .serialize_compressed(&mut deposit_compressed_bytes)
        .map_err(|e| {
            error!("Failed to serialize deposit proving key: {e}");
            warp::reject::custom(KeyVerificationError::new(
                "Error serializing deposit proving key",
            ))
        })?;
    file.write_all(&deposit_compressed_bytes).map_err(|e| {
        error!("Failed to write deposit_compressed_bytes to file: {e}");
        warp::reject::custom(KeyVerificationError::new(
            "Error writing deposit_compressed_bytes to file",
        ))
    })?;

    generate_rollup_keys_for_production(deposit_circuit, deposit_pk_path, &kzg_srs).map_err(
        |e| {
            error!("Failed to generate rollup keys for production: {e}");
            warp::reject::custom(KeyVerificationError::new(
                "Error generating rollup keys for production",
            ))
        },
    )?;

    Ok(())
}

/// Deletes all existing zk proof key files to ensure fresh generation.
fn delete_existing_key_files(
    base_path: &std::path::Path,
    specs: Vec<KeySpec>,
) -> Result<(), warp::Rejection> {
    // Check if base path exists - if not, this might indicate a configuration issue
    if !base_path.exists() {
        let error_msg = format!(
            "Base directory '{}' does not exist - cannot clean up key files",
            base_path.display()
        );
        error!("{error_msg}");
        return Err(warp::reject::custom(KeyVerificationError::new(&error_msg)));
    }
    let bin_path = base_path.join("configuration/bin");
    if !bin_path.exists() {
        let error_msg = format!(
            "Bin directory '{}' does not exist - cannot clean up key files",
            bin_path.display()
        );
        error!("{error_msg}");
        return Err(warp::reject::custom(KeyVerificationError::new(&error_msg)));
    }
    for spec in specs {
        let file_path = base_path.join(spec.out_path);
        if file_path.exists() {
            std::fs::remove_file(file_path.clone()).map_err(|e| {
                error!("Failed to delete key file '{}': {}", file_path.display(), e);
                warp::reject::custom(KeyVerificationError::new(&format!(
                    "Failed to delete key file '{}': {}",
                    file_path.display(),
                    e
                )))
            })?;
        }
    }
    let ppot_file_path = base_path.join(format!("configuration/bin/ppot_{MAX_KZG_DEGREE}.ptau"));
    if ppot_file_path.exists() {
        std::fs::remove_file(ppot_file_path.clone()).map_err(|e| {
            error!(
                "Failed to delete key file '{}': {}",
                ppot_file_path.display(),
                e
            );
            warp::reject::custom(KeyVerificationError::new(&format!(
                "Failed to delete key file '{}': {}",
                ppot_file_path.display(),
                e
            )))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_existing_key_files() {
        // Create a temporary directory structure
        let temp_dir = std::env::temp_dir().join("test_key_deletion");
        std::fs::create_dir_all(temp_dir.join("configuration/bin")).unwrap();

        let merge_counts =
            merge_counts(get_block_size().expect("Failed to get block size")).unwrap();
        let (bn254_count, grumpkin_count) = merge_counts;

        // Create all the static test key files that should be deleted
        let mut static_test_files = vec![
            "ppot_26.ptau".to_string(),
            "base_grumpkin_pk".to_string(),
            "base_bn254_pk".to_string(),
            "decider_pk".to_string(),
        ];
        // Add merge_bn254_pk_0 .. merge_bn254_pk_{bn254_count-1}
        for i in 0..bn254_count {
            static_test_files.push(format!("merge_bn254_pk_{i}"));
        }

        // Add merge_grumpkin_pk_0 .. merge_grumpkin_pk_{grumpkin_count-1}
        for i in 0..grumpkin_count {
            static_test_files.push(format!("merge_grumpkin_pk_{i}"));
        }

        // Create the static test files
        for file in &static_test_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            std::fs::write(&file_path, b"test_key_data").unwrap();
            assert!(file_path.exists(), "Test file should be created: {file}");
        }

        // Also create some files that should NOT be deleted (to ensure we're selective)
        let preserve_files = vec!["other_file.txt", "config.toml"];
        for file in &preserve_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            std::fs::write(&file_path, b"preserve_me").unwrap();
        }

        // Get the KeySpecs
        let out_dir = PathBuf::from("configuration").join("bin");
        let specs = build_key_specs("http://example.com/configuration", &out_dir).unwrap();

        // Call the deletion function
        let result = delete_existing_key_files(&temp_dir, specs);
        assert!(result.is_ok(), "Key file deletion should succeed");

        // Verify that all static key files are deleted
        for file in &static_test_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            assert!(
                !file_path.exists(),
                "Static key file should be deleted: {file}"
            );
        }

        // Verify that non-key files are preserved
        for file in &preserve_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            assert!(
                file_path.exists(),
                "Non-key file should be preserved: {file}",
            );
        }
        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_delete_existing_key_files_nonexistent_directory() {
        // Get the KeySpecs
        let out_dir = PathBuf::from("configuration").join("bin");
        let specs = build_key_specs("http://example.com/configuration", &out_dir).unwrap();

        // Test with a directory that doesn't exist
        let nonexistent_dir = std::path::Path::new("/tmp/nonexistent_key_dir_12345");

        // The function should fail if the base directory doesn't exist
        // This indicates a configuration problem or missing workspace setup
        let result = delete_existing_key_files(nonexistent_dir, specs);
        assert!(
            result.is_err(),
            "Should fail if base directory doesn't exist - this indicates a configuration issue"
        );
    }

    #[test]
    fn test_delete_existing_key_files_missing_bin_directory() {
        // Get the KeySpecs
        let out_dir = PathBuf::from("configuration").join("bin");
        let specs = build_key_specs("http://example.com/configuration", &out_dir).unwrap();

        // Create base directory but not the bin subdirectory
        let temp_dir = std::env::temp_dir().join("test_missing_bin_dir");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // The function should fail if the bin directory doesn't exist
        let result = delete_existing_key_files(&temp_dir, specs);
        assert!(
            result.is_err(),
            "Should fail if bin directory doesn't exist"
        );

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_delete_existing_key_files_partial_files() {
        // Get the KeySpecs
        let out_dir = PathBuf::from("configuration").join("bin");
        let specs = build_key_specs("http://example.com/configuration", &out_dir).unwrap();

        // Test with only some files existing
        let temp_dir = std::env::temp_dir().join("test_partial_key_deletion");
        std::fs::create_dir_all(temp_dir.join("configuration/bin")).unwrap();

        // Create only some of the expected files
        let partial_files = vec!["base_grumpkin_pk", "merge_bn254_pk_0"];
        for file in &partial_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            std::fs::write(&file_path, b"test_key_data").unwrap();
        }

        // Call the deletion function
        let result = delete_existing_key_files(&temp_dir, specs);
        assert!(result.is_ok(), "Partial key file deletion should succeed");

        // Verify that existing files are deleted
        for file in &partial_files {
            let file_path = temp_dir.join("configuration/bin").join(file);
            assert!(
                !file_path.exists(),
                "Existing key file should be deleted: {file}"
            );
        }

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_merge_counts() {
        // Test valid block size 64
        let result_64 = merge_counts(64).unwrap();
        assert_eq!(
            result_64,
            (1, 2),
            "Block size 64 should have 1 bn254 and 2 grumpkin"
        );

        // Test valid block size 256
        let result_256 = merge_counts(256).unwrap();
        assert_eq!(
            result_256,
            (2, 3),
            "Block size 256 should have 2 bn254 and 3 grumpkin"
        );

        // Test invalid block sizes
        assert!(
            merge_counts(128).is_err(),
            "Block size 128 should return error"
        );
        assert!(
            merge_counts(32).is_err(),
            "Block size 32 should return error"
        );
        assert!(
            merge_counts(512).is_err(),
            "Block size 512 should return error"
        );
    }

    #[test]
    fn test_build_key_specs() {
        let out_dir = PathBuf::from("test/bin");
        let config_url = "http://example.com";
        let specs = build_key_specs(config_url, &out_dir).unwrap();

        // Should contain base keys with correct URLs and paths
        let decider_pk = specs
            .iter()
            .find(|s| s.name == "decider_pk")
            .expect("Should have decider_pk");
        assert_eq!(decider_pk.url, format!("{config_url}/decider_pk"));
        assert_eq!(decider_pk.out_path, out_dir.join("decider_pk"));

        let decider_vk = specs
            .iter()
            .find(|s| s.name == "decider_vk")
            .expect("Should have decider_vk");
        assert_eq!(decider_vk.url, format!("{config_url}/decider_vk"));
        assert_eq!(decider_vk.out_path, out_dir.join("decider_vk"));

        let base_bn254 = specs
            .iter()
            .find(|s| s.name == "base_bn254_pk")
            .expect("Should have base_bn254_pk");
        assert_eq!(base_bn254.url, format!("{config_url}/base_bn254_pk"));
        assert_eq!(base_bn254.out_path, out_dir.join("base_bn254_pk"));

        let base_grumpkin = specs
            .iter()
            .find(|s| s.name == "base_grumpkin_pk")
            .expect("Should have base_grumpkin_pk");
        assert_eq!(
            base_grumpkin.url,
            format!("{config_url}/base_grumpkin_pk")
        );
        assert_eq!(base_grumpkin.out_path, out_dir.join("base_grumpkin_pk"));

        let merge_counts =
            merge_counts(get_block_size().expect("Failed to get block size")).unwrap();
        let (bn254_count, grumpkin_count) = merge_counts;

        // Check merge_bn254_pk_0 .. merge_bn254_pk_{bn254_count-1}
        for i in 0..bn254_count {
            let name = format!("merge_bn254_pk_{i}");
            let spec = specs
                .iter()
                .find(|s| s.name == name)
                .unwrap_or_else(|| panic!("Should have {name}"));
            assert_eq!(spec.url, format!("{config_url}/{name}"));
            assert_eq!(spec.out_path, out_dir.join(&name));
        }

        // Check merge_grumpkin_pk_0 .. merge_grumpkin_pk_{grumpkin_count-1}
        for i in 0..grumpkin_count {
            let name = format!("merge_grumpkin_pk_{i}");
            let spec = specs
                .iter()
                .find(|s| s.name == name)
                .unwrap_or_else(|| panic!("Should have {name}"));
            assert_eq!(spec.url, format!("{config_url}/{name}"));
            assert_eq!(spec.out_path, out_dir.join(&name));
        }
    }

    #[tokio::test]
    async fn test_keccak256_file_async() {
        // Create a temporary file with known content
        let temp_file = std::env::temp_dir().join("test_hash_file");
        std::fs::write(&temp_file, b"test content").unwrap();

        let result = keccak256_file_async(&temp_file).await;
        assert!(result.is_ok());

        let (hash, size) = result.unwrap();
        assert_eq!(size, 12); // "test content" is 12 bytes
        assert!(!hash.is_empty());

        // Cleanup
        std::fs::remove_file(&temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_compare_overwritten_files_matching_hashes() {
        // Create temporary files with matching content
        let temp_dir = std::env::temp_dir().join("test_compare_matching");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let test_content = b"matching test content";
        let test_file = temp_dir.join("test_key");
        std::fs::write(&test_file, test_content).unwrap();

        // Compute the expected hash
        let mut hasher = Keccak256::new();
        hasher.update(test_content);
        let expected_hash = hex::encode(hasher.finalize());

        let keys = vec![KeyReport {
            name: "test_key".to_string(),
            path: test_file.display().to_string(),
            keccak256: expected_hash.clone(),
            bytes: test_content.len() as u64,
            fresh_download: true,
        }];

        let result = compare_overwritten_files(&keys).await;
        assert!(result.is_ok());

        let comparisons = result.unwrap();
        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].equal, "Hashes should match");
        assert!(comparisons[0].generated.is_some());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_compare_overwritten_files_mismatching_hashes() {
        // Create temporary files with different content
        let temp_dir = std::env::temp_dir().join("test_compare_mismatch");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test_key");
        std::fs::write(&test_file, b"actual content").unwrap();

        // Provide a different hash in the KeyReport
        let keys = vec![KeyReport {
            name: "test_key".to_string(),
            path: test_file.display().to_string(),
            keccak256: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            bytes: 14,
            fresh_download: true,
        }];

        let result = compare_overwritten_files(&keys).await;
        assert!(result.is_ok());

        let comparisons = result.unwrap();
        assert_eq!(comparisons.len(), 1);
        assert!(!comparisons[0].equal, "Hashes should not match");

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_compare_overwritten_files_missing_file() {
        // Test with a file that doesn't exist
        let nonexistent_path = "/tmp/nonexistent_key_file_12345";

        let keys = vec![KeyReport {
            name: "missing_key".to_string(),
            path: nonexistent_path.to_string(),
            keccak256: "abcd1234".to_string(),
            bytes: 100,
            fresh_download: true,
        }];

        let result = compare_overwritten_files(&keys).await;
        assert!(result.is_ok());

        let comparisons = result.unwrap();
        assert_eq!(comparisons.len(), 1);
        assert!(
            !comparisons[0].equal,
            "Should not be equal when file is missing"
        );
        assert!(
            comparisons[0].generated.is_none(),
            "Generated side should be None for missing file"
        );
        assert!(
            comparisons[0].downloaded.is_some(),
            "Downloaded side should be present"
        );
    }

    #[tokio::test]
    async fn test_compare_overwritten_files_multiple_keys() {
        // Test with multiple keys - mix of matching, mismatching, and missing
        let temp_dir = std::env::temp_dir().join("test_compare_multiple");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Key 1: Matching
        let file1 = temp_dir.join("key1");
        let content1 = b"content1";
        std::fs::write(&file1, content1).unwrap();
        let mut hasher1 = Keccak256::new();
        hasher1.update(content1);
        let hash1 = hex::encode(hasher1.finalize());

        // Key 2: Mismatching
        let file2 = temp_dir.join("key2");
        std::fs::write(&file2, b"content2").unwrap();

        // Key 3: Missing
        let file3 = temp_dir.join("key3_missing");

        let keys = vec![
            KeyReport {
                name: "key1".to_string(),
                path: file1.display().to_string(),
                keccak256: hash1,
                bytes: content1.len() as u64,
                fresh_download: true,
            },
            KeyReport {
                name: "key2".to_string(),
                path: file2.display().to_string(),
                keccak256: "wrong_hash".to_string(),
                bytes: 8,
                fresh_download: true,
            },
            KeyReport {
                name: "key3".to_string(),
                path: file3.display().to_string(),
                keccak256: "another_hash".to_string(),
                bytes: 0,
                fresh_download: false,
            },
        ];

        let result = compare_overwritten_files(&keys).await;
        assert!(result.is_ok());

        let comparisons = result.unwrap();
        assert_eq!(comparisons.len(), 3);

        // Verify key1 matches
        assert!(comparisons[0].equal, "Key1 should match");
        assert!(comparisons[0].generated.is_some());

        // Verify key2 doesn't match
        assert!(!comparisons[1].equal, "Key2 should not match");
        assert!(comparisons[1].generated.is_some());

        // Verify key3 is missing
        assert!(!comparisons[2].equal, "Key3 should not match");
        assert!(comparisons[2].generated.is_none());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_compare_overwritten_files_empty_input() {
        let keys: Vec<KeyReport> = vec![];
        let result = compare_overwritten_files(&keys).await;
        assert!(result.is_ok());

        let comparisons = result.unwrap();
        assert_eq!(
            comparisons.len(),
            0,
            "Should return empty vector for empty input"
        );
    }
}
