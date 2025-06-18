use crate::{test_settings::TestSettings, TestError};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::{BigInteger, PrimeField};
use ark_std::{rand::Rng, test_rng, UniformRand};
use configuration::{
    addresses::{get_addresses, Addresses, AddressesError, Sources},
    settings::Settings,
};
use ethers::{
    signers::{LocalWallet, Signer},
    types::{BigEndianHash, Filter, H160, H256, I256},
    utils::keccak256,
};
use ethers_middleware::Middleware;
use hex::ToHex;
use jf_primitives::{
    poseidon::Poseidon,
    trees::{Directions, MembershipProof, PathElement, TreeHasher},
};
use lib::initialisation::get_blockchain_client_connection;
use lib::{blockchain_client::BlockchainClientConnection, models::CertificateReq};
use log::{debug, info};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};
use nightfall_client::{
    domain::{
        entities::{CommitmentStatus, DepositSecret, HexConvertible, Preimage, Salt},
        error::NightfallContractError,
        notifications::NotificationPayload,
    },
    driven::{
        db::mongo::CommitmentEntry,
        plonk_prover::plonk_proof::{PlonkProof, PlonkProvingEngine},
    },
    drivers::{
        derive_key::ZKPKeys,
        rest::models::{
            DeEscrowDataReq, KeyRequest, NF3DepositRequest, NF3RecipientData, NF3TransferRequest,
            NF3WithdrawRequest, WithdrawDataReq,
        },
    },
    ports::{
        commitments::Commitment,
        proof::{PrivateInputs, ProvingEngine, PublicInputs},
        secret_hash::SecretHash,
    },
};
use num_bigint::BigUint;
use reqwest::{
    multipart::{Form, Part},
    StatusCode,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::Command,
    sync::Arc,
};
use tokio::{sync::Mutex, time};
use url::Url;
use uuid::Uuid;

const REQUEST_ID: &str = "X-Request-ID";

/// This function sets the mining interval to be MINING_INTERVAL and ensures that automining is off
/// We don't want to rely on the initial configuration of Anvil being correct.
pub async fn set_anvil_mining_interval(
    client: &reqwest::Client,
    url: &Url,
    interval: u32,
) -> Result<(), TestError> {
    // Disable automining
    let disable_automine = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "evm_setAutomine",
        "params": [false],
        "id": 1
    });
    client
        .post(url.clone())
        .json(&disable_automine)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;

    // Set interval mining
    let set_interval = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "evm_setIntervalMining",
        "params": [interval],
        "id": 1
    });
    client
        .post(url.clone())
        .json(&set_interval)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;

    Ok(())
}

/// This function will return the transactions in the last n blocks
pub async fn get_transactions_in_last_n_blocks(
    client: &reqwest::Client,
    url: &Url,
    n: u64,
) -> Result<Option<Value>, TestError> {
    // Get the latest block number
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });
    let res = client
        .post(url.clone())
        .json(&payload)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let res_json: Value = res
        .json()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let latest_block_hex = res_json["result"]
        .as_str()
        .ok_or_else(|| TestError::new("Failed to get latest block number".to_string()))?;
    let latest_block = u64::from_str_radix(latest_block_hex.trim_start_matches("0x"), 16)
        .map_err(|e| TestError::new(e.to_string()))?;

    let mut txs_per_block = Vec::new();

    for i in 0..n {
        let block_number = latest_block.saturating_sub(i);
        let block_number_hex = format!("0x{:x}", block_number);
        let block_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [block_number_hex, true],
            "id": 1
        });
        let block_res = client
            .post(url.clone())
            .json(&block_payload)
            .send()
            .await
            .map_err(|e| TestError::new(e.to_string()))?;
        let block_json: Value = block_res
            .json()
            .await
            .map_err(|e| TestError::new(e.to_string()))?;
        let block_obj = &block_json["result"];
        if block_obj.is_null() {
            continue;
        }
        let txs = block_obj["transactions"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        // Only include non-empty transaction arrays
        if !txs.is_empty() {
            txs_per_block.push(serde_json::json!(txs));
        }
    }
    if txs_per_block.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Array(txs_per_block)))
    }
}

/// Function to create a chain reorg on the Anvil test rpc., optionally replaying new transactions to be included in the reorg.
/// The replay boolean indicates whether to replay the transactions from the last 'depth' blocks or to leave them empty.
// pub async fn anvil_reorg(
//     client: &reqwest::Client,
//     url: &Url,
//     depth: u64,
//     replay: bool,
//     interval: u32,
// ) -> Result<(), TestError> {
//     debug!(
//         "Reorging the chain with depth {}, replaying blocks is set to {}",
//         depth, replay
//     );
//     // before we do anything, we should turn mining off because if a block can be created while this function, the depth is ill-defined
//     set_anvil_mining_interval(client, url, 0).await?;
//     // next we'll get all the transactions that are in the last 'depth' worth of blocks, so that we can replay them
//     let new_transactions = if replay {
//         get_transactions_in_last_n_blocks(client, url, depth).await?
//     } else {
//         None
//     };

//     let new_transactions = if let Some(nt) = new_transactions {
//         nt
//     } else {
//         // If no new transactions are provided, we'll provide an empty array.
//         serde_json::json!([
//                 // transactions go here
//             ])
//     };
//     let payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "anvil_reorg",
//         "params": [depth, new_transactions ],
//         "id": 1
//     });

//     let res = client
//         .post(url.clone())
//         .json(&payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(e.to_string()))?;

//     res.error_for_status_ref()
//         .map_err(|e| TestError::new(e.to_string()))?;
//     // reset the mining interval
//     set_anvil_mining_interval(client, url, interval).await?;
//     Ok(())
// }
use reqwest::Client;
// use serde_json::Value;
// use url::Url;
// use tracing::debug;

// use crate::utils::{set_anvil_mining_interval, get_transactions_in_last_n_blocks, TestError};

/// Function to create a chain reorg on the Anvil test rpc,
/// optionally replaying new transactions to be included in the reorg.
/// Injects a dummy tx to ensure different block hashes after reorg.
// pub async fn anvil_reorg(
//     client: &Client,
//     url: &Url,
//     depth: u64,
//     replay: bool,
//     interval: u32,
// ) -> Result<(), TestError> {
//     debug!(
//         "Reorging the chain with depth {}, replaying blocks is set to {}",
//         depth, replay
//     );

//     // Turn off mining to control block production during reorg
//     set_anvil_mining_interval(client, url, 0).await?;

//     // Collect transactions from the last `depth` blocks
//     let original_transactions = if replay {
//         get_transactions_in_last_n_blocks(client, url, depth).await?
//     } else {
//         None
//     };
//      ark_std::println!("original_transactions: {:?}", original_transactions);

//     // Prepare dummy tx to inject and ensure block hash differs
//     use ark_std::rand;
//     let dummy_tx = serde_json::json!({
//     "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266", // default Anvil funded account
//     "to": "0x000000000000000000000000000000000000beef",
//     "value": "0x1",
//     "gas": "0x5208",
//     "gasPrice": "0x3b9aca00",
//     "data": format!("0x{}", hex::encode(rand::random::<[u8; 8]>())),
// });
//   ark_std::println!("inset dummy tx");
//     // Inject dummy tx into replay set or create one if none
//     let new_transactions = if let Some(mut nt) = original_transactions {
//         if let Some(tx_array) = nt.as_array_mut() {
//             // tx_array.push(dummy_tx);
//             tx_array.insert(0, dummy_tx);
//         }
//         nt
//     } else {
//         // Provide a single dummy transaction if no replay
//         serde_json::json!([ dummy_tx ])
//     };

//     // Issue the actual reorg request to Anvil
//     let payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "anvil_reorg",
//         "params": [depth, new_transactions],
//         "id": 1
//     });

//     let res = client
//         .post(url.clone())
//         .json(&payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to send reorg payload: {}", e)))?;

//     res.error_for_status_ref()
//         .map_err(|e| TestError::new(format!("Reorg failed: {}", e)))?;

//     // Restore mining interval to expected value
//     set_anvil_mining_interval(client, url, interval).await?;

//     Ok(())
// }
// pub async fn anvil_reorg2(
//     client: &reqwest::Client,
//     url: &Url,
//     depth: u64,
//     replay: bool,
//     interval: u32,
// ) -> Result<(), TestError> {
//     tracing::warn!("Entered anvil_reorg2() function");

//     use ark_std::println;
//     use ark_std::rand;

//     debug!(
//         "Reorging the chain with depth {}, replaying blocks is set to {}",
//         depth, replay
//     );

//     // Turn off mining to avoid accidental block production during the reorg process
//     set_anvil_mining_interval(client, url, 0).await?;

//     // Get transactions in last N blocks, if replay is requested
//     let original_transactions = if replay {
//         get_transactions_in_last_n_blocks(client, url, depth).await?
//     } else {
//         None
//     };
//     println!("original_transactions: {:?}", original_transactions);

//     // Generate a dummy tx that ensures block hash changes
//     // let dummy_tx = serde_json::json!({
//     //     "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266", // Default Anvil unlocked account
//     //     "to": "0x000000000000000000000000000000000000beef",
//     //     "value": "0x1",
//     //     "gas": "0x5208",
//     //     "gasPrice": "0x3b9aca00",
//     //     "data": format!("0x{}", hex::encode(rand::random::<[u8; 8]>())),
//     // });
//     use rand::Rng;
//     use uuid::Uuid;
//     let random_value = rand::thread_rng().gen_range(1u64..100_000);
//     let random_to: [u8; 20] = rand::random();
//     let random_to_address = format!("0x{}", hex::encode(random_to));
//     let unique_data = Uuid::new_v4(); // 128 bits of randomness

//     let dummy_tx = serde_json::json!({
//         "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
//         "to": random_to_address, // More variation here ensures different state path
//         "value": format!("0x{:x}", random_value), // Random ETH value (non-zero)
//         "gas": "0x5208",
//         "gasPrice": "0x3b9aca00",
//         "data": format!("0x{}", hex::encode(unique_data.as_bytes())), // Unique calldata
//     });
//     println!("injecting dummy tx");

//     // Submit dummy transaction to mempool
//     let send_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "eth_sendTransaction",
//         "params": [dummy_tx.clone()],
//         "id": 100,
//     });
//     let send_resp = client
//         .post(url.clone())
//         .json(&send_payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to send dummy tx: {}", e)))?;

//     if !send_resp.status().is_success() {
//         return Err(TestError::new(format!(
//             "Failed to inject dummy tx: {:?}",
//             send_resp.text().await
//         )));
//     }
//     println!("submit dummy tx");

//     let increase_payload = serde_json::json!({
//     "jsonrpc": "2.0",
//     "method": "evm_increaseTime",
//     "params": [50],
//     "id": 99
// });
// client.post(url.clone()).json(&increase_payload).send().await.unwrap();

//     // Mine a block to include dummy transaction
//     let mine_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "evm_mine",
//         "params": [],
//         "id": 100,
//     });
//     let mine_resp = client
//         .post(url.clone())
//         .json(&mine_payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to mine block: {}", e)))?;

//     if !mine_resp.status().is_success() {
//         return Err(TestError::new(format!(
//             "Failed to mine block after dummy tx: {:?}",
//             mine_resp.text().await
//         )));
//     }
//     println!("Mine a block to include dummy transaction");

//     // Compose new transaction array: dummy first, then previous txs (if any)
//     let new_transactions = if let Some(mut nt) = original_transactions {
//         if let Some(tx_array) = nt.as_array_mut() {
//             tx_array.insert(0, dummy_tx);
//         }
//         nt
//     } else {
//         serde_json::json!([dummy_tx])
//     };

//     // Submit the actual reorg request
//     let payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "anvil_reorg",
//         "params": [depth, new_transactions],
//         "id": 1
//     });

//     let res = client
//         .post(url.clone())
//         .json(&payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to send reorg payload: {}", e)))?;

//     res.error_for_status_ref()
//         .map_err(|e| TestError::new(format!("Reorg failed: {}", e)))?;
//     println!("Submit the actual reorg request");

//     // Resume normal mining interval
//     set_anvil_mining_interval(client, url, interval).await?;
//     ark_std::println!(
//         "Final replay transactions including dummy: {:?}",
//         new_transactions
//     );

//     println!("Finished reorg with depth {}", depth);
//     Ok(())
// }

pub async fn anvil_reorg2(
    client: &reqwest::Client,
    url: &Url,
    depth: u64,
    replay: bool,
    interval: u32,
) -> Result<(), TestError> {
    tracing::warn!("Entered anvil_reorg2() function");

    use ark_std::{println, rand};
    use rand::Rng;
    use uuid::Uuid;

    debug!(
        "Reorging the chain with depth {}, replaying blocks is set to {}",
        depth, replay
    );

    // Step 1: Turn off auto mining
    set_anvil_mining_interval(client, url, 0).await?;

    // Step 2: Get original transactions if replay is true
    let original_transactions = if replay {
        get_transactions_in_last_n_blocks(client, url, depth).await?
    } else {
        None
    };

    println!("original_transactions: {:?}", original_transactions);

    // Step 3: Build new transactions (split them across N blocks)
    let mut new_transactions_per_block = vec![];

    if let Some(txns) = original_transactions {
        let tx_array = txns.as_array().cloned().unwrap_or_default();

        // Group txs into roughly equal slices per block
        let chunk_size = std::cmp::max(1, tx_array.len() as u64 / depth) as usize;
        let mut tx_chunks = tx_array.chunks(chunk_size);

        for _ in 0..depth {
            let mut chunk = tx_chunks.next().unwrap_or(&[]).to_vec();

            // Add a random dummy tx to guarantee hash divergence
            let random_value = rand::thread_rng().gen_range(1u64..100_000);
            let random_to: [u8; 20] = rand::random();
            let unique_data = Uuid::new_v4();

            let dummy_tx = serde_json::json!({
                "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
                "to": format!("0x{}", hex::encode(random_to)),
                "value": format!("0x{:x}", random_value),
                "gas": "0x5208",
                "gasPrice": "0x3b9aca00",
                "data": format!("0x{}", hex::encode(unique_data.as_bytes())),
            });

            chunk.insert(0, dummy_tx);
            new_transactions_per_block.push(chunk);
        }
    } else {
        // Only dummy txs if no original txs
        for _ in 0..depth {
            let random_value = rand::thread_rng().gen_range(1u64..100_000);
            let random_to: [u8; 20] = rand::random();
            let unique_data = Uuid::new_v4();

            let dummy_tx = serde_json::json!({
                "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
                "to": format!("0x{}", hex::encode(random_to)),
                "value": format!("0x{:x}", random_value),
                "gas": "0x5208",
                "gasPrice": "0x3b9aca00",
                "data": format!("0x{}", hex::encode(unique_data.as_bytes())),
            });

            new_transactions_per_block.push(vec![dummy_tx]);
        }
    }

    // Step 4: Inject transactions + mine block for each
    for (i, tx_list) in new_transactions_per_block.iter().enumerate() {
        for tx in tx_list {
            let send_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_sendTransaction",
                "params": [tx.clone()],
                "id": 100 + i,
            });

            let send_resp = client
                .post(url.clone())
                .json(&send_payload)
                .send()
                .await
                .map_err(|e| TestError::new(format!("Failed to send tx: {}", e)))?;

            if !send_resp.status().is_success() {
                return Err(TestError::new(format!(
                    "Failed to inject tx: {:?}",
                    send_resp.text().await
                )));
            }
        }

        // Advance time to make sure timestamp is different
        let increase_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "evm_increaseTime",
            "params": [rand::thread_rng().gen_range(10..60)],
            "id": 99
        });

        client
            .post(url.clone())
            .json(&increase_payload)
            .send()
            .await
            .unwrap();

        // Mine the block
        let mine_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "evm_mine",
            "params": [],
            "id": 101
        });

        let mine_resp = client
            .post(url.clone())
            .json(&mine_payload)
            .send()
            .await
            .map_err(|e| TestError::new(format!("Failed to mine block: {}", e)))?;

        if !mine_resp.status().is_success() {
            return Err(TestError::new(format!(
                "Failed to mine block: {:?}",
                mine_resp.text().await
            )));
        }

        println!("Block {} mined with {} txs", i, tx_list.len());
    }

    // Step 5: Resume mining
    set_anvil_mining_interval(client, url, interval).await?;

    println!("Finished reorg with depth {}", depth);
    Ok(())
}
/// Performs a full reorg by reverting to a snapshot, generating a new chain fork with same length but different blocks.
/// Ensures that the L1 block containing the tx is replayed with a different hash.
// pub async fn anvil_reorg3(
//     client: &reqwest::Client,
//     url: &Url,
//     depth: u64,
//     interval: u32,
// ) -> Result<(), TestError> {
//     use ark_std::{println, rand};
//     use rand::Rng;
//     use serde_json::json;
//     use uuid::Uuid;

//     // 1. Take snapshot
//     let snapshot_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_snapshot",
//         "params": [],
//         "id": 1
//     });
//     let snapshot_resp = client
//         .post(url.clone())
//         .json(&snapshot_payload)
//         .send()
//         .await
//         .unwrap();
//     // let snapshot_id = snapshot_resp.json::<serde_json::Value>().await.unwrap().as_str().unwrap().to_string();
//     let snapshot_json = snapshot_resp.json::<serde_json::Value>().await.unwrap();

//     let snapshot_id = snapshot_json
//         .get("result")
//         .unwrap()
//         .as_str()
//         .unwrap()
//         .to_string();

//     println!("Snapshot taken with ID: {}", snapshot_id);

//     // 2. Record current L1 block number and hash
//     let latest_block_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "eth_getBlockByNumber",
//         "params": ["latest", false],
//         "id": 2
//     });
//     let block_resp = client
//         .post(url.clone())
//         .json(&latest_block_payload)
//         .send()
//         .await
//         .unwrap();
//     let block = block_resp.json::<serde_json::Value>().await.unwrap();
//     println!("Full block response: {:?}", block);
//     let provider = get_blockchain_client_connection()
//         .await
//         .read()
//         .await
//         .get_client();
//     let original_block_number = provider.get_block_number().await.unwrap();
//     use ethers::types::BlockId;
//     use ethers::types::BlockNumber;
//     let block = provider
//     .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
//     .await
//     .unwrap()
//     .expect("Block not found");
// let original_block_hash: H256 = block
//     .hash
//     .expect("Block is missing hash (maybe it's pending)");
//     println!(
//         "Original block number: {}, hash: {}",
//         original_block_number, original_block_hash
//     );

//     // 3. Revert to snapshot
//     let revert_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_revert",
//         "params": [snapshot_id],
//         "id": 3
//     });
//     client
//         .post(url.clone())
//         .json(&revert_payload)
//         .send()
//         .await
//         .unwrap();

//     // 4. Disable automine
//     set_anvil_mining_interval(client, url, 0).await.unwrap();

//     // 5. Replay depth blocks with changed txs
//     for i in 0..depth {
//         let dummy_tx = json!({
//             "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
//             "to": format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
//             "value": format!("0x{:x}", rand::thread_rng().gen_range(1u64..100_000)),
//             "gas": "0x5208",
//             "gasPrice": "0x3b9aca00",
//             "data": format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())),
//         });

//         let send_payload = json!({
//             "jsonrpc": "2.0",
//             "method": "eth_sendTransaction",
//             "params": [dummy_tx],
//             "id": 1000 + i,
//         });
//         client
//             .post(url.clone())
//             .json(&send_payload)
//             .send()
//             .await
//             .unwrap();

//         let time_offset = rand::thread_rng().gen_range(10..90);
//         let time_payload = json!({
//             "jsonrpc": "2.0",
//             "method": "evm_increaseTime",
//             "params": [time_offset],
//             "id": 1100 + i,
//         });
//         client
//             .post(url.clone())
//             .json(&time_payload)
//             .send()
//             .await
//             .unwrap();

//         let mine_payload = json!({
//             "jsonrpc": "2.0",
//             "method": "evm_mine",
//             "params": [],
//             "id": 1200 + i
//         });
//         client
//             .post(url.clone())
//             .json(&mine_payload)
//             .send()
//             .await
//             .unwrap();
//         println!("Reorg block {} mined", i);

//         let hash_check_payload = json!({
//             "jsonrpc": "2.0",
//             "method": "eth_getBlockByNumber",
//             "params": [format!("0x{:x}", i), false],
//             "id": 1300 + i
//         });
//         let resp = client
//             .post(url.clone())
//             .json(&hash_check_payload)
//             .send()
//             .await
//             .unwrap();
//         let blk = resp.json::<serde_json::Value>().await.unwrap();
//         println!(
//             "Block {} hash after reorg: {}",
//             i,
//             blk
//         );
//     }

//     // 6. Re-enable mining
//     set_anvil_mining_interval(client, url, interval)
//         .await
//         .unwrap();

//     // 7. Fetch new block at original number
//     let new_block_number_hex = format!("0x{:x}", original_block_number);
//     let block_check_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "eth_getBlockByNumber",
//         "params": [new_block_number_hex, false],
//         "id": 4000
//     });
//     let check_resp = client
//     .post(url.clone())
//     .json(&block_check_payload)
//     .send()
//     .await
//     .expect("Failed to send eth_getBlockByNumber request");
//     let new_block = check_resp
//         .json::<serde_json::Value>()
//         .await
//         .unwrap()
//         .clone();
//      println!("new_block: {}", new_block);
//         println!("new_block: {:?}", new_block);


//     Ok(())
// }
// pub async fn anvil_reorg3(
//     client: &reqwest::Client,
//     url: &Url,
//     interval: u32,
// ) -> Result<(), TestError> {
//     use ark_std::{println, rand};
//     use rand::Rng;
//     use serde_json::json;
//     use uuid::Uuid;
//     use ethers::types::{BlockId, BlockNumber, H256};

//     // Step 1: Take snapshot
//     let snapshot_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_snapshot",
//         "params": [],
//         "id": 1
//     });
//     let snapshot_resp = client.post(url.clone()).json(&snapshot_payload).send().await.unwrap();
//     let snapshot_json = snapshot_resp.json::<serde_json::Value>().await.unwrap();
//     let snapshot_id = snapshot_json
//         .get("result")
//         .expect("Missing result field in snapshot response")
//         .as_str()
//         .expect("Snapshot result not a string")
//         .to_string();
//     println!("Snapshot taken with ID: {}", snapshot_id);

//     // Step 2: Record latest L1 block number and hash
//     let provider = get_blockchain_client_connection()
//         .await
//         .read()
//         .await
//         .get_client();
//     let original_block_number = provider.get_block_number().await.unwrap();
//     let original_block = provider
//         .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
//         .await
//         .unwrap()
//         .expect("Block not found");
//     let original_block_hash = original_block
//         .hash
//         .expect("Original block missing hash");

//     println!(
//         "Original block number: {}, hash: {}",
//         original_block_number, original_block_hash
//     );

//     // Step 3: Revert to snapshot
//     let revert_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_revert",
//         "params": [snapshot_id],
//         "id": 2
//     });
//     client.post(url.clone()).json(&revert_payload).send().await.unwrap();
//     println!("Reverted to snapshot");

//     // Step 4: Disable automine
//     set_anvil_mining_interval(client, url, 0).await.unwrap();

//     // Step 5: Replay N+1 blocks with dummy transactions
//     let blocks_to_mine = original_block_number.as_u64() + 1;
//     // for i in 0..blocks_to_mine {
//     //     let dummy_tx = json!({
//     //         "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
//     //         "to": format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
//     //         "value": format!("0x{:x}", rand::thread_rng().gen_range(1u64..100_000)),
//     //         "gas": "0x5208",
//     //         "gasPrice": "0x3b9aca00",
//     //         "data": format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())),
//     //     });

//     //     let send_payload = json!({
//     //         "jsonrpc": "2.0",
//     //         "method": "eth_sendTransaction",
//     //         "params": [dummy_tx],
//     //         "id": 1000 + i,
//     //     });
//     //     client.post(url.clone()).json(&send_payload).send().await.unwrap();

//     //     let time_offset = rand::thread_rng().gen_range(10..90);
//     //     let time_payload = json!({
//     //         "jsonrpc": "2.0",
//     //         "method": "evm_increaseTime",
//     //         "params": [time_offset],
//     //         "id": 1100 + i,
//     //     });
//     //     client.post(url.clone()).json(&time_payload).send().await.unwrap();

//     //     let mine_payload = json!({
//     //         "jsonrpc": "2.0",
//     //         "method": "evm_mine",
//     //         "params": [],
//     //         "id": 1200 + i
//     //     });
//     //     client.post(url.clone()).json(&mine_payload).send().await.unwrap();

//     //     println!("Reorg block {} mined", i);
//     // }
// for i in 0..blocks_to_mine {
//     for _ in 0..3 {
//         let dummy_tx = json!({
//             "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266", // default unlocked
//             "to": format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
//             "value": format!("0x{:x}", rand::thread_rng().gen_range(1u64..1_000_000)),
//             "gas": "0x5208",
//             "gasPrice": format!("0x{:x}", rand::thread_rng().gen_range(1_000_000_000u64..10_000_000_000)),
//             "data": format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())),
//         });

//         let send_payload = json!({
//             "jsonrpc": "2.0",
//             "method": "eth_sendTransaction",
//             "params": [dummy_tx],
//             "id": 1000 + i,
//         });

//         client.post(url.clone()).json(&send_payload).send().await.unwrap();
//     }

//     // Increase time + mine
//     let time_offset = rand::thread_rng().gen_range(30..180);
//     let time_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_increaseTime",
//         "params": [time_offset],
//         "id": 1100 + i,
//     });
//     client.post(url.clone()).json(&time_payload).send().await.unwrap();

//     let mine_payload = json!({
//         "jsonrpc": "2.0",
//         "method": "evm_mine",
//         "params": [],
//         "id": 1200 + i
//     });
//     client.post(url.clone()).json(&mine_payload).send().await.unwrap();
//     println!("Reorg block {} mined", i);
// }

//     // Step 6: Re-enable mining
//     set_anvil_mining_interval(client, url, interval).await.unwrap();

//     // Step 7: Fetch new block at original number and compare hash
//     let new_block = provider
//         .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
//         .await
//         .unwrap()
//         .expect("New block not found after reorg");
//     let new_block_hash = new_block.hash.expect("New block missing hash");

//     println!(
//         "New block number: {}, hash: {}",
//         original_block_number, new_block_hash
//     );

//     if new_block_hash != original_block_hash {
//         println!("Reorg succeeded: block hash changed");
//     } else {
//         println!("Reorg failed: block hash did not change");
//     }

//     Ok(())
// }

// pub async fn anvil_reorg3(
//     client: &reqwest::Client,
//     url: &Url,
//     depth: u64,
//     interval: u32,
// ) -> Result<(), TestError> {
//     use ark_std::{println, rand};
//     use rand::Rng;
//     use serde_json::json;
//     use uuid::Uuid;
//     use ethers::types::{BlockId, BlockNumber, H256};

    

//     // 2. Get block number & hash before reorg
//     let provider = get_blockchain_client_connection()
//         .await
//         .read()
//         .await
//         .get_client();
//     let original_block_number = provider.get_block_number().await.expect("msg: Failed to get block number");
//     let block = provider
//         .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
//         .await.expect("msg: Block not found")
//         .ok_or_else(|| TestError::new("Block not found".to_string()))?;
//     let original_block_hash = block
//         .hash
//         .ok_or_else(|| TestError::new("Block has no hash".to_string()))?;

//     println!(
//         "Original block number: {}, hash: {}",
//         original_block_number, original_block_hash
//     );
// // 1. Snapshot
//     let snapshot_id = client
//         .post(url.clone())
//         .json(&json!({
//             "jsonrpc": "2.0",
//             "method": "evm_snapshot",
//             "params": [],
//             "id": 1
//         }))
//         .send()
//         .await.expect("Missing result field in snapshot response1")
//         .json::<serde_json::Value>()
//         .await.expect("Missing result field in snapshot response2")
//         .get("result")
//         .and_then(|v| v.as_str())
//         .ok_or_else(|| TestError::new("Missing snapshot ID".to_string()))?
//         .to_string();

//     println!("Snapshot taken with ID: {}", snapshot_id);
//     // 3. Revert to snapshot
//     client
//         .post(url.clone())
//         .json(&json!({
//             "jsonrpc": "2.0",
//             "method": "evm_revert",
//             "params": [snapshot_id],
//             "id": 3
//         }))
//         .send()
//         .await.expect("msg: Failed to revert to snapshot");

//     // 4. Disable automine
//     set_anvil_mining_interval(client, url, 0).await?;

//     // 5. Mine `depth` blocks with dummy txs and random time
//     for i in 0..depth {
//         for _ in 0..2 {
//             let dummy_tx = json!({
//                 "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
//                 "to": format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
//                 "value": format!("0x{:x}", rand::thread_rng().gen_range(1u64..1_000_000)),
//                 "gas": "0x5208",
//                 "gasPrice": format!("0x{:x}", rand::thread_rng().gen_range(1_000_000_000u64..10_000_000_000)),
//                 "data": format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())),
//             });

//             client
//                 .post(url.clone())
//                 .json(&json!({
//                     "jsonrpc": "2.0",
//                     "method": "eth_sendTransaction",
//                     "params": [dummy_tx],
//                     "id": 1000 + i,
//                 }))
//                 .send()
//                 .await.expect("msg: Failed to send transaction");
//         }

//         // Add randomness to timestamp
//         let time_offset = rand::thread_rng().gen_range(30..180);
//         client
//             .post(url.clone())
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "method": "evm_increaseTime",
//                 "params": [time_offset],
//                 "id": 1100 + i,
//             }))
//             .send()
//             .await.expect("msg: Failed to increase time");

//         client
//             .post(url.clone())
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "method": "evm_mine",
//                 "params": [],
//                 "id": 1200 + i
//             }))
//             .send()
//             .await.expect("msg: Failed to mine block");

//         println!("Reorg block {} mined", i);
//     }

//     // 6. Re-enable mining
//     set_anvil_mining_interval(client, url, interval).await?;

//     // 7. Compare block hash at original height
//     let reorged_block = provider
//         .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
//         .await.expect("msg: Failed to get reorged block")
//         .ok_or_else(|| TestError::new("Reorged block not found".to_string()))?;
//     let new_block_hash = reorged_block
//         .hash
//         .ok_or_else(|| TestError::new("Reorged block has no hash".to_string()))?;

//     println!(
//         "New block hash at same height: {}",
//         new_block_hash
//     );

//     assert_ne!(
//         original_block_hash, new_block_hash,
//         "Reorg did not produce a new block"
//     );

//     println!("Reorg succeeded: block hash changed");

//     Ok(())
// }

use ark_std::{println, rand};
use serde_json::{json};
use ethers::types::{BlockId, BlockNumber};

pub async fn anvil_reorg3(
    client: &Client,
    url: &Url,
    depth: u64,
    interval: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Take snapshot
    let snapshot_payload = json!({
        "jsonrpc": "2.0",
        "method": "evm_snapshot",
        "params": [],
        "id": 1
    });
    let snapshot_resp = client.post(url.clone()).json(&snapshot_payload).send().await.expect("msg: Failed to take snapshot");
    let snapshot_json = snapshot_resp.json::<Value>().await.expect("msg: Failed to take snapshot");
    let snapshot_id = snapshot_json.get("result").unwrap().as_str().unwrap();

    // 2. Record current L1 block number and hash
    let provider = get_blockchain_client_connection().await.read().await.get_client();
    let original_block_number = provider.get_block_number().await?;
    let block = provider
        .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
        .await?
        .expect("Block not found");
    let original_block_hash = block.hash.expect("Block is missing hash");
    println!(
        "Original block number: {}, hash: {:?}",
        original_block_number, original_block_hash
    );

    // 3. Revert to snapshot
    let revert_payload = json!({
        "jsonrpc": "2.0",
        "method": "evm_revert",
        "params": [snapshot_id],
        "id": 3
    });
    client.post(url.clone()).json(&revert_payload).send().await.expect("msg: Failed to take snapshot");

    // 4. Disable automine
    set_anvil_mining_interval(client, url, 0).await.expect("msg: Failed to take snapshot");

    // 5. Mine new blocks with dummy txs
    for i in 0..depth {
        let dummy_tx = json!({
            "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "to": format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
            "value": format!("0x{:x}", rand::thread_rng().gen_range(1u64..100_000)),
            "gas": "0x5208",
            "gasPrice": "0x3b9aca00",
            "data": format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())),
        });

        client
            .post(url.clone())
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "eth_sendTransaction",
                "params": [dummy_tx],
                "id": 1000 + i,
            }))
            .send()
            .await.expect("msg: Failed to take snapshot");

        client
            .post(url.clone())
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "evm_increaseTime",
                "params": [rand::thread_rng().gen_range(10..90)],
                "id": 1100 + i,
            }))
            .send()
            .await.expect("msg: Failed to take snapshot");

        client
            .post(url.clone())
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "evm_mine",
                "params": [],
                "id": 1200 + i
            }))
            .send()
            .await.expect("msg: Failed to take snapshot");
        println!("Reorg block {} mined", i);
    }

    // 6. Re-enable mining
    set_anvil_mining_interval(client, url, interval).await.expect("msg: Failed to take snapshot");

    // 7. Compare block hash at original height
    let new_block = provider
        .get_block(BlockId::Number(BlockNumber::Number(original_block_number)))
        .await
        .expect("New block not found");
    let new_block_hash = new_block.expect("New block not found at original height").hash.expect("New block has no hash");

    println!(
        "After reorg, block {} hash: {:?}",
        original_block_number, new_block_hash
    );

    assert_ne!(
        original_block_hash, new_block_hash,
        "Reorg did not produce a new block"
    );

    Ok(())
}




// pub async fn anvil_reorg2(
//     client: &reqwest::Client,
//     url: &Url,
//     depth: u64,
//     replay: bool,
//     interval: u32,
// ) -> Result<(), TestError> {
//     use ark_std::{println, rand};
//     use rand::Rng;
//     use uuid::Uuid;
//     use tracing::{debug, warn};

//     warn!("Entered anvil_reorg2() function");
//     debug!(
//         "Reorging the chain with depth {}, replaying blocks is set to {}",
//         depth, replay
//     );

//     // Turn off mining to avoid accidental block production
//     set_anvil_mining_interval(client, url, 0).await?;

//     // Query current block number
//     let provider = get_blockchain_client_connection()
//         .await
//         .read()
//         .await
//         .get_client();

//     let current_block_number = provider.get_block_number().await.unwrap();
//     let fork_point = current_block_number
//         .checked_sub(depth.into())
//         .ok_or_else(|| TestError::new("Depth too large: cannot rewind past genesis".to_string()))?;

//     // Rewind Anvil to the fork point (this truncates the chain)
//     let rewind_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "anvil_setHead",
//         "params": [format!("0x{:x}", fork_point)],
//         "id": 55,
//     });
//     client.post(url.clone()).json(&rewind_payload).send().await.map_err(|e| {
//         TestError::new(format!("Failed to rewind with anvil_setHead: {}", e))
//     })?;

//     // Get previous transactions if replaying
//     let original_transactions = if replay {
//         get_transactions_in_last_n_blocks(client, url, depth).await?
//     } else {
//         None
//     };
//     println!("original_transactions: {:?}", original_transactions);

//     // Create unique dummy tx
//     let random_value = rand::thread_rng().gen_range(1u64..100_000);
//     let random_to: [u8; 20] = rand::random();
//     let random_to_address = format!("0x{}", hex::encode(random_to));
//     let unique_data = Uuid::new_v4();

//     let dummy_tx = serde_json::json!({
//         "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
//         "to": random_to_address,
//         "value": format!("0x{:x}", random_value),
//         "gas": "0x5208",
//         "gasPrice": "0x3b9aca00",
//         "data": format!("0x{}", hex::encode(unique_data.as_bytes())),
//     });

//     println!("Injecting dummy tx...");

//     let send_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "eth_sendTransaction",
//         "params": [dummy_tx.clone()],
//         "id": 100,
//     });

//     let send_resp = client
//         .post(url.clone())
//         .json(&send_payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to send dummy tx: {}", e)))?;

//     if !send_resp.status().is_success() {
//         return Err(TestError::new(format!(
//             "Failed to inject dummy tx: {:?}",
//             send_resp.text().await
//         )));
//     }

//     println!("Submitted dummy tx");

//     // Advance time to avoid duplicated timestamps
//     let increase_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "evm_increaseTime",
//         "params": [50],
//         "id": 99
//     });
//     client.post(url.clone()).json(&increase_payload).send().await.unwrap();

//     // Mine a block to include dummy tx
//     let mine_payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "evm_mine",
//         "params": [],
//         "id": 101
//     });

//     let mine_resp = client
//         .post(url.clone())
//         .json(&mine_payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to mine block: {}", e)))?;

//     if !mine_resp.status().is_success() {
//         return Err(TestError::new(format!(
//             "Failed to mine block after dummy tx: {:?}",
//             mine_resp.text().await
//         )));
//     }

//     println!("Mined block with dummy tx");

//     // Compose transactions: dummy first + original txs
//     let new_transactions = if let Some(mut nt) = original_transactions {
//         if let Some(tx_array) = nt.as_array_mut() {
//             tx_array.insert(0, dummy_tx);
//         }
//         nt
//     } else {
//         serde_json::json!([dummy_tx])
//     };

//     // Call `anvil_reorg` — NOTE: this is optional now since `setHead` + mining has already forked
//     // You could skip this if you're mining your forked blocks manually
//     let payload = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "anvil_reorg",
//         "params": [depth, new_transactions],
//         "id": 1
//     });

//     let res = client
//         .post(url.clone())
//         .json(&payload)
//         .send()
//         .await
//         .map_err(|e| TestError::new(format!("Failed to send reorg payload: {}", e)))?;

//     res.error_for_status_ref()
//         .map_err(|e| TestError::new(format!("Reorg failed: {}", e)))?;

//     println!("Submitted `anvil_reorg` request");

//     // Resume mining
//     set_anvil_mining_interval(client, url, interval).await?;

//     println!("Finished reorg at fork point block {}", fork_point);
//     Ok(())
// }

pub async fn get_erc20_balance(http_client: &reqwest::Client, url: Url) -> i64 {
    let url = url.join("v1/balance/").unwrap();
    i64::from_str_radix(
        get_balance(http_client, url, TokenType::ERC20, "0x00".to_string())
            .await
            .unwrap()
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0)
}
pub async fn get_erc721_balance(
    http_client: &reqwest::Client,
    url: Url,
    token_id: String,
) -> Option<i64> {
    let url = url.join("v1/balance/").unwrap();
    let balance = get_balance(http_client, url, TokenType::ERC721, token_id)
        .await
        .ok()?;
    i64::from_str_radix(balance.trim_start_matches("0x"), 16).ok()
}

pub async fn get_fee_balance(http_client: &reqwest::Client, url: Url) -> i64 {
    let url = url.join("v1/fee_balance/").unwrap();
    i64::from_str_radix(
        handle_fee_balance(http_client, url)
            .await
            .unwrap()
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0)
}
#[derive(Debug)]
struct CertificateValidationError {
    status: StatusCode,
}

impl fmt::Display for CertificateValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Certificate validation failed with status: {}",
            self.status
        )
    }
}

impl Error for CertificateValidationError {}

pub async fn validate_certificate_with_server(
    client: &reqwest::Client,
    url: Url,
    certificate_req: CertificateReq,
) -> Result<(), TestError> {
    let part_cert = Part::bytes(certificate_req.certificate.clone())
        .file_name("certificate.der")
        .mime_str("application/octet-stream")
        .map_err(|e| TestError::new(e.to_string()))?;
    let part_priv_key = Part::bytes(certificate_req.certificate_private_key)
        .file_name("certificate.priv_key")
        .mime_str("application/octet-stream")
        .map_err(|e| TestError::new(e.to_string()))?;
    let form = Form::new()
        .part("certificate", part_cert)
        .part("certificate_private_key", part_priv_key);
    let response = client
        .post(url)
        .multipart(form)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    if response.status() != StatusCode::ACCEPTED {
        return Err(TestError::new(
            CertificateValidationError {
                status: response.status(),
            }
            .to_string(),
        ));
    }
    Ok(())
}
#[derive(serde::Deserialize, Clone)]

/// Define a struct to group related parameters for a deposit/transfer/withdraw transaction to avoid too many arguments clippy error
pub struct TransactionDetails {
    /// Value to be transferred/deposited/withdrawn
    pub value: String,
    /// The transaction fee to be paid by the sender. This fee is awarded to the proposer for processing the transaction. For Deposit, sender will pay fee twice, one to deposit token, and one to deposit fee which can be used for other txs.
    pub fee: String,
    /// Token ID of the transaction
    pub token_id: String,
}

/// Struct used for the four token types supported by Nightfall 4
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    ERC20,
    ERC1155,
    ERC721,
    ERC3525,
}

impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            1 => TokenType::ERC1155,
            2 => TokenType::ERC721,
            3 => TokenType::ERC3525,
            _ => TokenType::ERC20,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::ERC20 => write!(f, "0"),
            TokenType::ERC1155 => write!(f, "1"),
            TokenType::ERC721 => write!(f, "2"),
            TokenType::ERC3525 => write!(f, "3"),
        }
    }
}

impl TokenType {
    /// Gets the relevant mock contract address for the token type
    fn address(&self) -> String {
        match self {
            TokenType::ERC20 => hex::encode(TestSettings::retrieve_mock_addresses().erc20.0),
            TokenType::ERC721 => hex::encode(TestSettings::retrieve_mock_addresses().erc721.0),
            TokenType::ERC1155 => hex::encode(TestSettings::retrieve_mock_addresses().erc1155.0),
            TokenType::ERC3525 => hex::encode(TestSettings::retrieve_mock_addresses().erc3525.0),
        }
    }
}

// get a ZKP key object from the client container (client knows how to generate them)
pub async fn get_key(url: Url, key_request: &KeyRequest) -> Result<ZKPKeys, TestError> {
    // getting a key is easy; we just pass in a mnemonic and a key object is returned from the nightfall client
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(key_request)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    let key = res
        .json::<ZKPKeys>()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(key)
}

pub fn load_addresses(settings: &Settings) -> Result<Addresses, AddressesError> {
    let s = Sources::File(PathBuf::from(&settings.contracts.addresses_file));
    Addresses::load(s)
}

pub fn get_recipient_address(settings: &Settings) -> Result<String, TestError> {
    let recipient_address = format!(
        "0x{}",
        settings
            .signing_key
            .parse::<LocalWallet>()
            .map_err(|e| TestError::new(e.to_string()))?
            .address()
            .encode_hex::<String>()
    );
    Ok(recipient_address)
}

/// Function should only be called after we have checked forge is installed by running 'which forge'
pub fn forge_command(command: &[&str]) {
    let output = Command::new("forge").args(command).output();

    match output {
        Ok(o) => {
            if o.status.success() {
                info!(
                    "Command 'forge {:?}' executed successfully: {}",
                    command,
                    String::from_utf8_lossy(&o.stdout)
                );
            } else {
                panic!(
                "Command 'forge {:?}' executed with failing error code: {:?}\nStandard Output: {}\nStandard Error: {}",
                command,
                o.status.signal(),
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            }
        }
        Err(e) => {
            panic!(
                "Command 'forge {:?}' ran into an error without executing: {}",
                command, e
            );
        }
    }
}

/// This function gets the latest layer 2 block number from the responses vector that the webhook server populates
pub async fn get_latest_l2_block_number(responses: Arc<Mutex<Vec<Value>>>) -> Option<u64> {
    let responses = responses.lock().await;
    responses
        .iter()
        .map(|r| {
            // Deserialize the JSON value into a NotificationPayload
            serde_json::from_value::<NotificationPayload>(r.clone()).unwrap()
        })
        // Filter the NotificationPayloads to find the ones that are BlockchainEvents and extract the l2_block_number
        .filter_map(|r| match r {
            NotificationPayload::BlockchainEvent {
                l2_block_number, ..
            } => Some(l2_block_number.as_u64()),
            _ => None,
        })
        .max()
}

/// This function waits until a Webhook response is received for every Request ID
pub async fn wait_for_all_responses(
    large_block_deposit_ids: &[String],
    responses: Arc<Mutex<Vec<Value>>>,
) -> Vec<String> {
    // compare the response IDs with the request IDs
    // if we find everything we need, we can break out of the loop and return the json data in the responses (ignoring the ID)
    let response_data = loop {
        // get a lock on the current response vectore
        let mut response_values = responses.lock().await;
        // destructure the vector to get the responses and the request IDs
        let response_payloads = response_values
            .iter()
            .map(|r| serde_json::from_value::<NotificationPayload>(r.clone()).unwrap())
            .collect::<Vec<_>>();
        let (response_data, response_ids): (Vec<_>, Vec<_>) = response_payloads
            .iter()
            .cloned()
            .filter_map(|r| match r {
                NotificationPayload::TransactionEvent { uuid, response } => Some((
                    response,
                    serde_json::from_str::<String>(&uuid).expect("Could not parse uuid"),
                )),
                _ => None,
            })
            .unzip();
        debug!("Response IDs: {:?}", response_ids);
        debug!("Request IDs: {:?}", large_block_deposit_ids);
        info!(
            "Have {} IDs and {} processed client transactions",
            large_block_deposit_ids.len(),
            response_ids.len()
        );
        // check if the response IDs contain all the request IDs
        // we'll do a simple O(n^2) search for the request IDs in the responses as the vector is small
        let same = large_block_deposit_ids
            .iter()
            .all(|id| response_ids.contains(id));
        // if we find everything we need, we can break out of the loop and return the json data in the responses (ignoring the ID)
        if same {
            response_values.clear(); // clear the responses so we can reuse the vector
            break response_data;
        }
        // if we get here, we haven't found everything we need yet
        drop(response_values); // free the mutex lock while we wait for more responses
        time::sleep(time::Duration::from_secs(10)).await;
    };
    info!("All expected responses from the clients' webhooks have been received");
    debug!("{:#?}", response_data);
    response_data
}

/// Function to get the L1 block hash of a given layer 2 block number
pub async fn get_l1_block_hash_of_layer2_block(
    block_number: I256,
) -> Result<H256, NightfallContractError> {
    let block_number = block_number - I256::one();
    let client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let nightfall_address = get_addresses().nightfall();
    ark_std::println!("nightfall_address: {}", nightfall_address);
    let block_topic = H256::from_uint(&block_number.into_raw());
    ark_std::println!("block_topic: {}", block_topic);

    // This is keccak256("BlockProposed(int256)")
    let latest_block = client.get_block_number().await.map_err(|e| {
        NightfallContractError::ProviderError(format!("get_block_number error: {}", e))
    })?;

    let event_sig = H256::from(keccak256("BlockProposed(int256)"));
    let filter = Filter::new()
        .address(nightfall_address)
        .from_block(0u64)
        .to_block(latest_block)
        .topic0(event_sig)
        // .event("BlockProposed(int256)");
        .topic1(block_topic);
    ark_std::println!("filter: {:?}", filter);

    let logs = client
        .get_logs(&filter)
        .await
        .map_err(|e| NightfallContractError::ProviderError(format!("Provider error: {}", e)))?;
    ark_std::println!("logs: {:?}", logs);
    let logs2 = client
        .get_logs(
            &Filter::new()
                .from_block(0u64)
                .to_block(latest_block)
                .address(nightfall_address),
        )
        .await
        .unwrap();
    for log in logs2 {
        println!("Log topics2: {:?}", log.topics);
    }

    // get the first log, as we only check first l1 block which contains the block number
    let log = logs
        .first()
        .ok_or_else(|| NightfallContractError::BlockNotFound(block_number.as_u64()))?;
    ark_std::println!("log: {:?}", log);
    let tx_hash = log.transaction_hash.ok_or_else(|| {
        NightfallContractError::MissingTransactionHash("Log has no transaction hash".to_string())
    })?;
    ark_std::println!("tx_hash: {:?}", tx_hash);

    // Fetch the full transaction to get block hash
    let tx = client
        .get_transaction(tx_hash)
        .await
        .map_err(|e| {
            NightfallContractError::ProviderError(format!("get_transaction error: {}", e))
        })?
        .ok_or(NightfallContractError::TransactionNotFound(tx_hash))?;

    let block_hash = tx.block_hash.ok_or_else(|| {
        NightfallContractError::MissingTransactionHash(
            "Transaction does not have a block hash (possibly pending)".to_string(),
        )
    })?;

    ark_std::println!(
        "L2 block {} was submitted in L1 block {} with L1 block hash : {}",
        block_number,
        tx.block_number.unwrap_or_default(),
        block_hash
    );

    Ok(block_hash)
}

/// this function returns a Future that resolves when all commitments in the slice are available on-chain
/// We need to know who the recipient is so we can query the correct endpoint.
pub async fn wait_on_chain(
    commitment_hashes: &[Fr254],
    recipient_url: &str,
) -> Result<(), TestError> {
    info!("Waiting for commitments to appear on-chain");
    // get commitment hashes to act as database keys
    // query the DB for these hashes and wait until we have found them all
    let client = reqwest::Client::new();
    let mut count = 0;
    let mut onchain_set = HashSet::<Fr254>::new();
    while count < commitment_hashes.len() {
        for hash in commitment_hashes.iter() {
            let url = Url::parse(recipient_url)
                .map_err(|e| TestError::new(e.to_string()))?
                .join(&format!("v1/commitment/{}", hash.to_hex_string()))
                .map_err(|e| TestError::new(e.to_string()))?;
            let res = client
                .get(url)
                .header(REQUEST_ID, Uuid::new_v4().to_string())
                .send()
                .await
                .map_err(|e| TestError::new(e.to_string()))
                .map_err(|e| TestError::new(e.to_string()))?;
            match res.error_for_status() {
                Ok(res) => {
                    let commit = res
                        .json::<CommitmentEntry>()
                        .await
                        .map_err(|e| TestError::new(e.to_string()))?;
                    // we only care about commitments that are unspent
                    if commit.status != CommitmentStatus::Unspent {
                        continue;
                    }
                    let onchain_hash = commit.hash().map_err(|e| TestError::new(e.to_string()))?;
                    if onchain_set.contains(&onchain_hash) {
                        continue;
                    } else {
                        let onchain_hash =
                            commit.hash().map_err(|e| TestError::new(e.to_string()))?;
                        debug!("Commitment {} is on-chain", onchain_hash.to_hex_string());
                        count += 1;
                        onchain_set.insert(onchain_hash);
                    }
                }
                Err(err) => {
                    debug!("Waiting for commitments: {}", err.without_url());
                }
            }
        }
        time::sleep(time::Duration::from_secs(5)).await;
    }
    info!("Commitments are now on-chain");
    Ok(())
}

/// Function to submit a request to de-escrow funds after a withdraw
pub async fn de_escrow_request(req: &DeEscrowDataReq, client_url: &str) -> Result<u8, TestError> {
    let client = reqwest::Client::new();
    let url = Url::parse(client_url)
        .map_err(|e| TestError::new(e.to_string()))?
        .join("v1/de-escrow")
        .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .post(url)
        .json(&serde_json::json!(req))
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.json::<u8>()
        .await
        .map_err(|e| TestError::new(e.to_string()))
}

/// Function to generate proof and inputs for a random transaction
pub fn create_random_proof_and_inputs(
    rng: &mut impl Rng,
) -> Result<(PlonkProof, PublicInputs, PrivateInputs), TestError> {
    let (mut public_inputs, mut private_inputs) = build_valid_transfer_inputs(rng);
    debug!("Generating proof");
    let proof = PlonkProvingEngine::prove(&mut private_inputs, &mut public_inputs)
        .map_err(|e| TestError::new(e.to_string()))?;

    Ok((proof, public_inputs, private_inputs))
}

/// Function to create and send a deposit transaction, returning the ID of the transaction
pub async fn create_nf3_deposit_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    deposit_fee: String,
) -> Result<String, TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating deposit transaction onchain {}", &id);
    let deposit_request = create_nf3_deposit_request(
        tx_details.value,
        tx_details.fee,
        deposit_fee.clone(),
        token_type,
        tx_details.token_id,
    );
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(deposit_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    debug!("Sent deposit request");
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();
    info!(
        "Deposit transaction {} has been accepted by the client",
        returned_id
    );
    Ok(returned_id)
}

pub async fn create_nf3_transfer_transaction(
    recipient_zkp_key: ZKPKeys,
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
) -> Result<String, TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating transfer transaction offchain {}", &id);
    let transfer_request = create_nf3_transfer_request(
        recipient_zkp_key,
        tx_details.value,
        tx_details.fee,
        token_type,
        tx_details.token_id,
    )
    .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(transfer_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();
    info!(
        "Transfer transaction {} has been accepted by the client",
        returned_id
    );
    Ok(returned_id)
}

pub async fn create_nf3_withdraw_transaction(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    tx_details: TransactionDetails,
    recipient_address: String,
) -> Result<(String, WithdrawDataReq), TestError> {
    let id = Uuid::new_v4().to_string();
    info!("Creating withdraw transaction offchain {}", &id);
    let withdraw_request = create_nf3_withdraw_request(
        recipient_address.clone(),
        tx_details.value.clone(),
        tx_details.fee.clone(),
        token_type,
        tx_details.token_id.clone(),
    );
    // Send the POST request to the API
    let res = client
        .post(url.clone())
        .json(&serde_json::json!(withdraw_request))
        .header(REQUEST_ID, &id)
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;

    // Ensure the response status is 200 OK or 202 ACCEPTED
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let status = res.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::ACCEPTED,
        "Unexpected status code: {}",
        status
    );
    let returned_id = res
        .headers()
        .get(REQUEST_ID)
        .unwrap()
        .to_str()
        .map_err(|e| TestError::new(e.to_string()))?
        .to_string();

    let withdraw_data_request = WithdrawDataReq {
        token_id: tx_details.token_id,
        erc_address: token_type.address(),
        recipient_address,
        value: tx_details.value,
        fee: tx_details.fee,
        token_type: token_type.to_string(),
        withdraw_fund_salt: String::default(),
    };

    info!(
        "Withdraw transaction {} has been accepted by the client",
        returned_id
    );
    Ok((returned_id, withdraw_data_request))
}

pub async fn get_balance(
    client: &reqwest::Client,
    url: Url,
    token_type: TokenType,
    token_id: String,
) -> Result<String, TestError> {
    let erc_address = token_type.address();
    let url = url
        .join(&format!("{}/{}", erc_address, token_id))
        .map_err(|e| TestError::new(e.to_string()))?;
    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let balance = res
        .text()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(balance)
}

pub async fn handle_fee_balance(client: &reqwest::Client, url: Url) -> Result<String, TestError> {
    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let balance = res
        .text()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    Ok(balance)
}

pub async fn count_spent_commitments(
    client: &reqwest::Client,
    url: Url,
) -> Result<usize, TestError> {
    let url = url
        .join("commitments")
        .map_err(|e| TestError::new(e.to_string()))?;

    let res = client
        .get(url)
        .header(REQUEST_ID, Uuid::new_v4().to_string())
        .send()
        .await
        .map_err(|e| TestError::new(e.to_string()))?;
    res.error_for_status_ref()
        .map_err(|e| TestError::new(e.to_string()))?;
    let count = res
        .json::<Vec<CommitmentEntry>>()
        .await
        .map_err(|e| TestError::new(e.to_string()))?
        .into_iter()
        .filter(|c| c.status == CommitmentStatus::Spent)
        .count();
    Ok(count)
}

fn create_nf3_deposit_request(
    value: String,
    fee: String,
    deposit_fee: String,
    token_type: TokenType,
    token_id: String,
) -> NF3DepositRequest {
    NF3DepositRequest {
        erc_address: token_type.address(),
        token_id,
        token_type: token_type.to_string(),
        value,
        fee,
        deposit_fee,
        secret_preimage_one: None,
        secret_preimage_two: None,
        secret_preimage_three: None,
    }
}

fn create_nf3_transfer_request(
    recipient_zkp_key: ZKPKeys,
    value: String,
    fee: String,
    token_type: TokenType,
    token_id: String,
) -> Result<NF3TransferRequest, TestError> {
    Ok(NF3TransferRequest {
        erc_address: token_type.address(),
        token_id,
        recipient_data: NF3RecipientData {
            values: vec![value],
            recipient_compressed_zkp_public_keys: vec![hex::encode(
                recipient_zkp_key
                    .compressed_public_key()
                    .map_err(|e| TestError::new(e.to_string()))?,
            )],
        },
        fee,
    })
}

fn create_nf3_withdraw_request(
    recipient_address: String,
    value: String,
    fee: String,
    token_type: TokenType,
    token_id: String,
) -> NF3WithdrawRequest {
    NF3WithdrawRequest {
        erc_address: token_type.address(),
        token_id,
        token_type: token_type.to_string(),
        value,
        recipient_address,
        fee,
    }
}

fn generate_random_path(leaf_value: Fr254, rng: &mut impl Rng) -> (MembershipProof<Fr254>, Fr254) {
    let mut root = leaf_value;
    let poseidon = Poseidon::<Fr254>::new();
    let leaf_index = u32::rand(rng);
    let mut path_elements = Vec::<PathElement<Fr254>>::new();
    for i in 0..32 {
        let dir = leaf_index >> i & 1;
        let value = Fr254::rand(rng);
        if dir == 0 {
            root = poseidon.tree_hash(&[root, value]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnRight,
                value,
            })
        } else {
            root = poseidon.tree_hash(&[value, root]).unwrap();
            path_elements.push(PathElement {
                direction: Directions::HashWithThisNodeOnLeft,
                value,
            })
        }
    }

    (
        MembershipProof {
            node_value: leaf_value,
            sibling_path: path_elements,
            leaf_index: leaf_index as usize,
        },
        root,
    )
}

// Creates a random 96 bit element of Fr254
fn rand_96_bit(rng: &mut impl Rng) -> Fr254 {
    let random_96_bit = u128::rand(rng) % (1u128 << 96);
    Fr254::from(random_96_bit)
}

pub fn build_valid_transfer_inputs(rng: &mut impl Rng) -> (PublicInputs, PrivateInputs) {
    let mut rng_fr = test_rng();
    let token_id = Fr254::rand(&mut rng_fr);
    let erc_address = Fr254::rand(&mut rng_fr);
    let nf_slot_id = Fr254::rand(&mut rng_fr);

    let token_id_bytes = token_id.into_bigint().to_bytes_be();
    let erc_address_bytes = erc_address.into_bigint().to_bytes_be();

    // Concatenate the bytes
    let mut input_bytes = Vec::new();
    input_bytes.extend_from_slice(&erc_address_bytes);
    input_bytes.extend_from_slice(&token_id_bytes);

    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    let digest = hasher.finalize();

    // Convert digest to BigUint
    let mut nf_token_id = BigUint::from_bytes_be(&digest);

    // Right shift by 4 bits
    nf_token_id >>= 4;
    let nf_token_id = Fr254::from(nf_token_id);

    // Retrieve the fee token ID and nightfall address
    let nf_address = H160::rand(rng);
    // generate a 'random' fee token ID (we just use the keccak hash of 1)
    let fee_token_id = Fr254::from(BigUint::from_bytes_be(&keccak256([1])) >> 4);

    // Random values for fee and value
    let mut nullified_fee_one = rand_96_bit(rng);
    let mut nullified_fee_two = rand_96_bit(rng);
    let mut nullified_value_one = rand_96_bit(rng);
    let mut nullified_value_two = rand_96_bit(rng);

    let mut value = rand_96_bit(rng);
    let mut fee = rand_96_bit(rng);

    // We need to make sure the fee and value are less than the sum of the nullified fee and value.
    // We also need to ensure the change will not exceed 2^96.
    while value > (nullified_value_one + nullified_value_two)
        || (nullified_value_one + nullified_value_two) - value >= Fr254::from(1u128 << 96)
    {
        nullified_value_one = rand_96_bit(rng);
        nullified_value_two = rand_96_bit(rng);
        value = rand_96_bit(rng);
    }

    while fee > (nullified_fee_one + nullified_fee_two)
        || (nullified_fee_one + nullified_fee_two) - fee >= Fr254::from(1u128 << 96)
    {
        nullified_fee_one = rand_96_bit(rng);
        nullified_fee_two = rand_96_bit(rng);
        fee = rand_96_bit(rng);
    }

    // Generate random root key
    let root_key = Fr254::rand(rng);
    let keys = ZKPKeys::new(root_key).unwrap();

    // Generate random recipient public key
    let recipient_public_key = TEAffine::<BabyJubJub>::rand(rng);

    // Generate random ephemeral private key
    let ephemeral_key = BJJScalar::rand(rng);

    // Make commitments for nullified values
    let nullified_one = Preimage::new(
        nullified_value_one,
        nf_token_id,
        nf_slot_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    // The second token commitment nullified will be from a deposit so the public key will be the neutral point
    let deposit_secret = DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_two = Preimage::new(
        nullified_value_two,
        nf_token_id,
        nf_slot_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(deposit_secret),
    );

    // Now nullified fee tokens
    let nullified_three = Preimage::new(
        nullified_fee_one,
        fee_token_id,
        fee_token_id,
        keys.zkp_public_key,
        Salt::new_transfer_salt(),
    );
    let fee_deposit_secret =
        DepositSecret::new(Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng));
    let nullified_four = Preimage::new(
        nullified_fee_two,
        fee_token_id,
        fee_token_id,
        TEAffine::<BabyJubJub>::zero(),
        Salt::Deposit(fee_deposit_secret),
    );

    // Make membership proofs
    let spend_commitments = [
        nullified_one,
        nullified_two,
        nullified_three,
        nullified_four,
    ];
    let mut membership_proofs = vec![];
    let mut roots = vec![];
    for nullifier in spend_commitments.iter() {
        let (membership_proof, root) = generate_random_path(nullifier.hash().unwrap(), rng);
        membership_proofs.push(membership_proof);
        roots.push(root);
    }

    let mem_proofs: [MembershipProof<Fr254>; 4] = membership_proofs.try_into().unwrap();
    let roots: [Fr254; 4] = roots.try_into().unwrap();

    // Work out what the change values will be
    let value_change = nullified_value_one + nullified_value_two - value;
    let fee_change = nullified_fee_one + nullified_fee_two - fee;

    // Salts for new commitments
    let new_salts = [Salt::new_transfer_salt().get_salt(); 3];

    let public_inputs = PublicInputs::new().fee(fee).roots(&roots).build();

    let private_inputs = PrivateInputs::new()
        .nf_address(nf_address)
        .value(value)
        .nf_token_id(nf_token_id)
        .nf_slot_id(nf_slot_id)
        .ephemeral_key(ephemeral_key)
        .recipient_public_key(recipient_public_key)
        .nullifiers_values(&[
            nullified_one.get_value(),
            nullified_two.get_value(),
            nullified_three.get_value(),
            nullified_four.get_value(),
        ])
        .nullifiers_salts(&[
            nullified_one.get_salt(),
            nullified_two.get_salt(),
            nullified_three.get_salt(),
            nullified_four.get_salt(),
        ])
        .commitments_values(&[value_change, fee_change])
        .commitments_salts(&new_salts)
        .membership_proofs(&mem_proofs)
        .nullifier_key(keys.nullifier_key)
        .secret_preimages(&[
            nullified_one.get_secret_preimage().to_array(),
            nullified_two.get_secret_preimage().to_array(),
            nullified_three.get_secret_preimage().to_array(),
            nullified_four.get_secret_preimage().to_array(),
        ])
        .zkp_private_key(keys.zkp_private_key)
        .public_keys(&[
            nullified_one.get_public_key(),
            nullified_two.get_public_key(),
            nullified_three.get_public_key(),
            nullified_four.get_public_key(),
        ])
        .build();

    (public_inputs, private_inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::signers::Wallet;
    use ethers::types::{TransactionRequest, U256};
    use ethers::{
        providers::{Http, Provider},
        utils::Anvil,
    };
    use ethers_middleware::Middleware;
    use ethers_middleware::SignerMiddleware;
    use reqwest::Client;
    use std::str::FromStr;
    #[tokio::test]
    async fn test_get_transactions_in_last_n_blocks() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let url = Url::parse(&endpoint).unwrap();
        let client = Client::new();

        // Set up ethers provider and wallet
        let provider = Provider::<Http>::try_from(endpoint.clone()).unwrap();
        let wallet: Wallet<ethers::core::k256::ecdsa::SigningKey> = anvil.keys()[0].clone().into();
        let wallet = wallet.with_chain_id(anvil.chain_id());
        let provider = Arc::new(SignerMiddleware::new(provider, wallet));

        // Get two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let from = accounts[0];
        let to = accounts[1];

        // Send three transactions
        let mut tx_hashes = Vec::new();
        for i in 0..3 {
            let tx = TransactionRequest::new()
                .from(from)
                .to(to)
                .value(U256::from(1_000_000_000_000_000u128 + i));
            let pending_tx = provider.send_transaction(tx, None).await.unwrap();
            let receipt = pending_tx.await.unwrap().unwrap();
            tx_hashes.push(receipt.transaction_hash);
        }

        // Wait for the blocks to be mined
        let _latest_block = provider.get_block_number().await.unwrap().as_u64();

        // Retrieve transactions in the last 3 blocks
        let txs_json = get_transactions_in_last_n_blocks(&client, &url, 3)
            .await
            .expect("Failed to get transactions")
            .expect("No transactions found");

        // Flatten the transactions from all blocks
        let mut found_hashes = Vec::new();
        for block_txs in txs_json.as_array().unwrap() {
            for tx in block_txs.as_array().unwrap() {
                let hash_str = tx["hash"].as_str().unwrap();
                let hash = ethers::types::H256::from_str(hash_str).unwrap();
                found_hashes.push(hash);
            }
        }

        // All submitted tx hashes should be present in the found_hashes
        for hash in tx_hashes {
            assert!(
                found_hashes.contains(&hash),
                "Transaction hash {:?} not found in last n blocks",
                hash
            );
        }
    }
    #[tokio::test]
    async fn test_anvil_reorg_no_replay() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        // Mine three blocks to ensure there is some chain history before reorg
        provider
            .request::<_, ()>("anvil_mine", serde_json::json!([3]))
            .await
            .unwrap();
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(3)
        );

        // get the balances of the first two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let balance1 = provider.get_balance(accounts[0], None).await.unwrap();
        let balance2 = provider.get_balance(accounts[1], None).await.unwrap();
        // transfer 0.1 ether from the first account to the second account
        let from = accounts[0];
        let to = accounts[1];
        let tx = provider
            .send_transaction(
                ethers::types::TransactionRequest::new()
                    .from(from)
                    .to(to)
                    .value(ethers::types::U256::from(100_000_000_000_000_000u128)),
                None,
            )
            .await
            .unwrap();
        // wait for the transaction to be mined
        let tx_receipt = tx.await.unwrap().unwrap();
        // check that the block number is 4
        assert_eq!(
            tx_receipt.block_number.unwrap(),
            ethers::types::U64::from(4)
        );
        // check that the node also thinks the block number is 4
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(4)
        );
        // check that the transaction is in the block
        let block = provider
            .get_block_with_txs(tx_receipt.block_number.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert!(block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));

        // Check the balances transferred after the transaction
        let new_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            new_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );

        // simulate a reorg of depth 1
        let url = Url::parse(&endpoint).unwrap();
        let client = Client::new();
        anvil_reorg2(&client, &url, 1, false, 5).await.unwrap();
        // Check the block number after the reorg, it should not have changed because the reorged chain has the same depth (a property of Anvil)
        let new_block_number = provider.get_block_number().await.unwrap();
        assert_eq!(new_block_number, ethers::types::U64::from(4));
        // Check the balances after the reorg, they should be the original balances, before the transaction
        let reverted_balance1 = provider.get_balance(from, None).await.unwrap();
        let reverted_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(reverted_balance1, balance1);
        assert_eq!(reverted_balance2, balance2);
        // Check that the transaction is no longer in the block
        let block = provider
            .get_block_with_txs(new_block_number)
            .await
            .unwrap()
            .unwrap();
        assert!(!block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));
    }

    #[tokio::test]
    async fn test_anvil_reorg_with_replay() {
        // Start a new Anvil instance
        let anvil = Anvil::new().spawn();
        let endpoint = anvil.endpoint();
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        // Mine three blocks to ensure there is some chain history before reorg
        provider
            .request::<_, ()>("anvil_mine", serde_json::json!([3]))
            .await
            .unwrap();
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(3)
        );

        // Get the balances of the first two accounts
        let accounts = provider.get_accounts().await.unwrap();
        let balance1 = provider.get_balance(accounts[0], None).await.unwrap();
        let balance2 = provider.get_balance(accounts[1], None).await.unwrap();

        // Transfer 0.1 ether from the first account to the second account
        let from = accounts[0];
        let to = accounts[1];
        let tx = provider
            .send_transaction(
                ethers::types::TransactionRequest::new()
                    .from(from)
                    .to(to)
                    .value(ethers::types::U256::from(100_000_000_000_000_000u128)),
                None,
            )
            .await
            .unwrap();
        // Wait for the transaction to be mined
        let tx_receipt = tx.await.unwrap().unwrap();
        // Check that the block number is 4
        assert_eq!(
            tx_receipt.block_number.unwrap(),
            ethers::types::U64::from(4)
        );
        // Check that the node also thinks the block number is 4
        assert_eq!(
            provider.get_block_number().await.unwrap(),
            ethers::types::U64::from(4)
        );
        // Check that the transaction is in the block
        let block = provider
            .get_block_with_txs(tx_receipt.block_number.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert!(block
            .transactions
            .iter()
            .any(|t| t.hash == tx_receipt.transaction_hash));

        // Check the balances transferred after the transaction
        let new_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            new_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );

        // Simulate a reorg of depth 1 with replay = true
        let url = Url::parse(&endpoint).unwrap();
        ark_std::println!("url: {:?}", url);
        let client = Client::new();
        anvil_reorg2(&client, &url, 1, true, 5).await.unwrap();

        // Check the block number after the reorg, it should not have changed because the reorged chain has the same depth (a property of Anvil)
        let new_block_number = provider.get_block_number().await.unwrap();
        assert_eq!(new_block_number, ethers::types::U64::from(4));

        // Check the balances after the reorg, they should be the same as after the transaction, since the tx was replayed
        let replayed_balance1 = provider.get_balance(from, None).await.unwrap();
        let replayed_balance2 = provider.get_balance(to, None).await.unwrap();
        assert_eq!(
            replayed_balance2,
            balance2 + ethers::types::U256::from(100_000_000_000_000_000u128)
        );
        // The sender's balance should be less than or equal to the original balance (due to gas costs)
        assert!(replayed_balance1 < balance1);

        // Check that the transaction is still in the block (replayed)
        let block = provider
            .get_block_with_txs(new_block_number)
            .await
            .unwrap()
            .unwrap();
        let found = block.transactions.iter().any(|t| {
            t.to == Some(to) && t.value == ethers::types::U256::from(100_000_000_000_000_000u128)
        });
        assert!(
            found,
            "Replayed transaction not found in block after reorg with replay"
        );
    }
}
