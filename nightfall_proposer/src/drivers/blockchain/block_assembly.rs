use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_block_assembly_trigger, get_blockchain_client_connection},
    ports::{contracts::NightfallContract, proving::RecursiveProvingEngine},
    services::assemble_block::assemble_block,
};
use alloy::{
    primitives::{Address, TxHash, U64},
    providers::{Provider, RootProvider},
    rpc::types::{BlockId, BlockNumberOrTag},
    sol_types::SolEvent,
};
use ark_serialize::SerializationError;
use configuration::{addresses::get_addresses, settings::get_settings};
use jf_plonk::errors::PlonkError;
use lib::{
    blockchain_client::BlockchainClientConnection,
    error::{ConversionError, EventHandlerError, NightfallContractError},
    log_fetcher::{get_genesis_block, get_logs_paginated},
    nf_client_proof::Proof,
    verify_contract::VerifiedContracts,
};
use log::{debug, error, info, warn};
use nightfall_bindings::artifacts::RoundRobin;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum BlockAssemblyError {
    FailedToAssembleBlock(String),
    FailedToProposeBlock,
    FailedToGetReceipt,
    FailedToGetCalldata,
    FailedToGetDepositData(NightfallContractError),
    InsufficientTransactions,
    SerializationError(SerializationError),
    ConversionError(ConversionError),
    ProvingError(String),
    ContractError(String),
    ProviderError(String),
    EventHandlerError(EventHandlerError),
    FinalityTimeout,
    QueueError(String),
    Other(String),
}

impl From<EventHandlerError> for BlockAssemblyError {
    fn from(e: EventHandlerError) -> Self {
        BlockAssemblyError::EventHandlerError(e)
    }
}
use lib::error::ConfigError;

impl From<ConfigError> for BlockAssemblyError {
    fn from(e: ConfigError) -> Self {
        BlockAssemblyError::Other(format!("Configuration error: {e}"))
    }
}

impl Error for BlockAssemblyError {}
impl Display for BlockAssemblyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToAssembleBlock(s) => write!(f, "Failed to assemble block: {s}"),
            Self::FailedToProposeBlock => write!(f, "Failed to propose block"),
            Self::FailedToGetReceipt => write!(f, "Failed to get receipt for block"),
            Self::FailedToGetCalldata => write!(f, "Failed to get calldata for block"),
            Self::InsufficientTransactions => {
                write!(f, "At least two transactions are required per block")
            }
            Self::SerializationError(e) => {
                write!(f, "{e}")
            }
            Self::ConversionError(e) => {
                write!(f, "{e}")
            }
            Self::FailedToGetDepositData(e) => write!(f, "Failed to acquire deposit data: {e}"),
            Self::ProvingError(s) => write!(f, "Error occurred while proving: {s} "),
            Self::ContractError(s) => write!(f, "Contract error: {s}"),
            Self::ProviderError(s) => write!(f, "Provider error: {s}"),
            Self::EventHandlerError(e) => write!(f, "Event handling error: {e}"),
            Self::QueueError(s) => write!(f, "Queued error: {s}"),
            Self::Other(s) => write!(f, "Other error: {s}"),
            Self::FinalityTimeout => write!(f, "Finality timeout occurred."),
        }
    }
}

impl From<SerializationError> for BlockAssemblyError {
    fn from(e: SerializationError) -> Self {
        BlockAssemblyError::SerializationError(e)
    }
}

impl From<ConversionError> for BlockAssemblyError {
    fn from(e: ConversionError) -> Self {
        BlockAssemblyError::ConversionError(e)
    }
}

impl From<NightfallContractError> for BlockAssemblyError {
    fn from(e: NightfallContractError) -> Self {
        BlockAssemblyError::FailedToGetDepositData(e)
    }
}

impl From<PlonkError> for BlockAssemblyError {
    fn from(e: PlonkError) -> Self {
        BlockAssemblyError::ProvingError(format!("PlonkError: {e}"))
    }
}

async fn check_l1_finality(
    client: &RootProvider,
    tx_hash_l1: TxHash,
    confirmations_required: U64,
    wait_timeout: Option<Duration>,
) -> Result<bool, BlockAssemblyError> {
    let start_time = std::time::Instant::now();
    let poll_interval = Duration::from_secs(2);

    loop {
        // Get finalized block (with fallback to latest)
        let finalized_block = match client
            .get_block(BlockId::Number(BlockNumberOrTag::Finalized))
            .await
        {
            Ok(Some(block)) => block,
            Ok(None) => {
                let current_block = client
                    .get_block_number()
                    .await
                    .map_err(|e| BlockAssemblyError::ProviderError(e.to_string()))?;
                client
                    .get_block(BlockId::Number(BlockNumberOrTag::Number(current_block)))
                    .await
                    .map_err(|e| BlockAssemblyError::ProviderError(e.to_string()))?
                    .ok_or(BlockAssemblyError::Other("Current block not found".into()))?
            }
            Err(e) => return Err(BlockAssemblyError::ProviderError(e.to_string())),
        };
        // Check transaction receipt
        match client.get_transaction_receipt(tx_hash_l1).await {
            Ok(Some(tx_receipt)) => {
                if let (Some(receipt_block_number), finalized_block_number) =
                    (tx_receipt.block_number, finalized_block.header.number)
                {
                    // If we are using anvil, assume finality immediately
                    if get_settings().network.chain_id == 31337 {
                        return Ok(true);
                    }
                    // Already finalized
                    if receipt_block_number <= finalized_block_number {
                        let confirmations =
                            finalized_block_number.saturating_sub(receipt_block_number);
                        if U64::from(confirmations) >= confirmations_required {
                            return Ok(true);
                        }
                    }

                    // Can never be finalized (tx too new)
                    println!(
                        "additional confirmations required: {}",
                        U64::from(receipt_block_number) + confirmations_required
                    );
                    if U64::from(receipt_block_number) + confirmations_required
                        > U64::from(finalized_block_number)
                        && wait_timeout.is_none()
                    {
                        return Ok(false);
                    }
                }
            }
            Ok(None) => {
                // Transaction not found yet
                if let Some(timeout) = wait_timeout {
                    if start_time.elapsed() > timeout {
                        return Err(BlockAssemblyError::FinalityTimeout);
                    }
                } else {
                    return Ok(false);
                }
            }
            Err(e) => return Err(BlockAssemblyError::ProviderError(e.to_string())),
        }

        // Exit if no waiting requested
        if wait_timeout.is_none() {
            return Ok(false);
        }

        tokio::time::sleep(poll_interval).await;
    }
}

// once called this function will trigger the block assembly process whenever
// certain conditions are met
// Any errors that propogate back up to here will cause a panic.
pub async fn start_block_assembly<P, R, N>() -> Result<(), BlockAssemblyError>
where
    P: Proof,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
    N: NightfallContract,
{
    let blockchain_client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .clone();
    let client = blockchain_client.root().clone();
    let verified = VerifiedContracts::resolve_and_verify_contract(client.clone(), get_addresses())
        .await
        .map_err(|e| {
            NightfallContractError::ContractVerificationError(format!(
                "Contract verification failed: {e}"
            ))
        })?;
    let round_robin_instance = Arc::new(verified.round_robin.clone());

    let rr_addr = get_addresses().round_robin;
    let code = blockchain_client
        .get_code_at(rr_addr)
        .await
        .unwrap_or_default();
    tracing::warn!(
        "RoundRobin address: {rr_addr:?}, bytecode_len: {}",
        code.0.len()
    );

    // EIP-1967 implementation slot = keccak256("eip1967.proxy.implementation") - 1
    let impl_slot = "0x360894A13BA1A3210667C828492DB98DCA3E2076CC3735A920A3CA505D382BBC"
        .parse()
        .expect("valid slot");
    let impl_raw = blockchain_client
        .get_storage_at(rr_addr, impl_slot)
        .await
        .unwrap_or_default();
    let impl_addr = Address::from_slice(&impl_raw.as_le_bytes()[12..]);
    tracing::warn!("EIP-1967 impl at RR addr: {impl_addr:?}");

    let a = get_addresses();
    tracing::info!(
        "Using addresses — nightfall: {:?}, round_robin: {:?}, x509: {:?}",
        a.nightfall,
        a.round_robin,
        a.x509
    );

    // Shared queue for blocks waiting for finality confirmation
    let pending_blocks = Arc::new(Mutex::new(Vec::new()));
    let confirmations_required = U64::from(12);
    let finality_check_interval = Duration::from_secs(5);

    debug!("Starting block assembly");

    // Spawn the finality checking task
    let _finality_checker: tokio::task::JoinHandle<Result<(), BlockAssemblyError>> = {
        let pending_blocks = Arc::clone(&pending_blocks);
        let rr = Arc::clone(&round_robin_instance);
        let blockchain_client = blockchain_client.clone();
        tokio::spawn(async move {
            let mut last_scanned: u64 = get_genesis_block();
            let mut last_finalized_turn: Option<u64> = None;

            loop {
                // If nothing to propose, don't waste RPC calls
                let has_pending = {
                    let guard = pending_blocks.lock().await;
                    !guard.is_empty()
                };
                if !has_pending {
                    tokio::time::sleep(finality_check_interval).await;
                    continue;
                }

                let latest_block = match blockchain_client.root().get_block_number().await {
                    Ok(n) => n,
                    Err(e) => {
                        error!("Finality checker: failed to get latest block number: {e}");
                        tokio::time::sleep(finality_check_interval).await;
                        continue;
                    }
                };

                if latest_block < last_scanned {
                    // chain reorg / provider weirdness; just clamp
                    last_scanned = latest_block;
                }

                let onchain_start_block: u64 = match rr.start_l1_block().call().await {
                    Ok(b) => match b.try_into() {
                        Ok(block) => block,
                        Err(_) => {
                            error!("Finality checker: start_l1_block does not fit into u64");
                            tokio::time::sleep(finality_check_interval).await;
                            continue;
                        }
                    },
                    Err(e) => {
                        error!("Finality checker: failed rr.start_l1_block(): {e}");
                        tokio::time::sleep(finality_check_interval).await;
                        continue;
                    }
                };

                let onchain_current_proposer = match rr.get_current_proposer_address().call().await {
                    Ok(addr) => addr,
                    Err(e) => {
                        error!("Finality checker: failed rr.get_current_proposer_address(): {e}");
                        tokio::time::sleep(finality_check_interval).await;
                        continue;
                    }
                };

                let our_addr = get_blockchain_client_connection()
                    .await
                    .read()
                    .await
                    .get_address();

                if onchain_current_proposer != our_addr {
                    debug!(
                        "Finality checker: proposer is {onchain_current_proposer:?}, we are {our_addr:?}. Not our turn."
                    );
                    tokio::time::sleep(finality_check_interval).await;
                    continue;
                }

                if last_finalized_turn == Some(onchain_start_block) {
                    let drained_for_same_turn: Vec<_> = {
                        let mut guard = pending_blocks.lock().await;
                        guard.drain(..).collect()
                    };

                    if !drained_for_same_turn.is_empty() {
                        info!(
                            "Finality checker: current proposer turn {onchain_start_block} already finalized, proposing {} pending blocks",
                            drained_for_same_turn.len()
                        );
                        for block in drained_for_same_turn {
                            if let Err(e) = N::propose_block(block).await {
                                error!("Finality checker: propose_block failed: {e}");
                            }
                        }
                    }

                    tokio::time::sleep(finality_check_interval).await;
                    continue;
                }

                let from_block = last_scanned
                    .saturating_sub(5)
                    .min(onchain_start_block.saturating_sub(5));

                let rotation_filter = rr
                    .event_filter::<RoundRobin::ProposerRotated>()
                    .from_block(from_block);

                let rotation_logs = match get_logs_paginated(
                    blockchain_client.root(),
                    rotation_filter.filter.clone(),
                    from_block,
                    latest_block,
                )
                .await
                {
                    Ok(events) => events,
                    Err(e) => {
                        error!(
                            "Finality checker: failed to fetch ProposerRotated logs paginated: {e}"
                        );
                        tokio::time::sleep(finality_check_interval).await;
                        continue;
                    }
                };

                // Advance cursor only after successful fetch.
                last_scanned = latest_block;

                let maybe_current_turn_event = rotation_logs.into_iter().rev().find(|evt| {
                    matches!(evt.block_number, Some(block_number) if block_number == onchain_start_block)
                        && RoundRobin::ProposerRotated::decode_log(&evt.inner).is_ok()
                });

                let Some(evt) = maybe_current_turn_event else {
                    debug!(
                        "Finality checker: no ProposerRotated event found for current turn {onchain_start_block} in range {from_block}..{latest_block}"
                    );
                    tokio::time::sleep(finality_check_interval).await;
                    continue;
                };

                let tx_hash = match evt.transaction_hash {
                    Some(h) => h,
                    None => {
                        error!("Finality checker: rotation event missing transaction_hash");
                        tokio::time::sleep(finality_check_interval).await;
                        continue;
                    }
                };

                let client = blockchain_client.root().clone();

                match check_l1_finality(
                    &client,
                    tx_hash,
                    confirmations_required,
                    Some(finality_check_interval),
                )
                .await
                {
                    Ok(true) => {
                        info!(
                            "ProposerRotated tx finalized: {tx_hash:?} (event block: {onchain_start_block})"
                        );
                        last_finalized_turn = Some(onchain_start_block);

                        let drained_after_finality: Vec<_> = {
                            let mut guard = pending_blocks.lock().await;
                            guard.drain(..).collect()
                        };

                        if !drained_after_finality.is_empty() {
                            info!(
                                "Finality checker: finalized & canonical rotation, proposing {} pending blocks",
                                drained_after_finality.len()
                            );
                            for block in drained_after_finality {
                                if let Err(e) = N::propose_block(block).await {
                                    error!("Finality checker: propose_block failed: {e}");
                                }
                            }
                        }
                    }
                    Ok(false) => {
                        debug!("Finality checker: rotation tx not yet finalized: {tx_hash:?}");
                    }
                    Err(e) => {
                        error!("Finality checker: finality check error: {e}");
                    }
                }

                tokio::time::sleep(finality_check_interval).await;
            }
        })
    };
    // Main block assembly loop
    loop {
        debug!("Checking proposer status...");
        // Step 1: Get current proposer address from smart contract
        let current_proposer = match round_robin_instance
            .get_current_proposer_address()
            .call()
            .await
        {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to get current proposer: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        let our_address = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_address();

        // Step 2: If we are not the proposer, wait and retry
        if current_proposer != our_address {
            info!("We are not the current proposer. Current proposer is: {current_proposer:?}");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

        // Step 3: We are the current proposer. Wait for trigger.
        info!("We are the current proposer. Awaiting trigger...");
        get_block_assembly_trigger::<P>()
            .await
            .read()
            .await
            .await_trigger()
            .await;
        let current_proposer_after_trigger = match round_robin_instance
            .get_current_proposer_address()
            .call()
            .await
        {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to get current proposer after trigger: {e}");
                continue;
            }
        };

        let our_address = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_address();

        if current_proposer_after_trigger != our_address {
            info!(
        "Proposer has changed after trigger. Skipping block assembly. New proposer is: {current_proposer_after_trigger:?}"
    );
            continue;
        }
        // Step 4: check if we're synchronised.
        // Go round again if we're not because we can't make new blocks
        let mut sync_status = get_synchronisation_status().await.write().await;
        let current_block_number = N::get_current_layer2_blocknumber().await.map_err(|_| {
            BlockAssemblyError::FailedToAssembleBlock(
                "Failed to get current block number".to_string(),
            )
        })?;
        if current_block_number.is_zero() {
            // if we're at block 0, we're automatically synchronised because no blocks have been made yet
            sync_status.set_synchronised();
        }
        if !sync_status.is_synchronised() {
            warn!("We are not synchronised. We won't make blocks until we are");
            continue;
        }
        debug!("Triggered block assembly");
        let block_result = assemble_block::<P, R>().await;
        let block = match block_result {
            Ok(block) => block,
            Err(e) => match e {
                BlockAssemblyError::InsufficientTransactions => continue,
                _ => {
                    error!("Block assembly failed with error {e}");
                    continue;
                }
            },
        };
        // Add to pending blocks queue
        {
            let mut blocks = pending_blocks.lock().await;
            blocks.push(block);
            info!("Added block to queue ({} pending)", blocks.len());
        }
    }
}
