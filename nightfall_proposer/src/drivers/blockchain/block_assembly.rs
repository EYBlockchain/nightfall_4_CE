use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_block_assembly_trigger, get_blockchain_client_connection},
    ports::{contracts::NightfallContract, proving::RecursiveProvingEngine},
    services::assemble_block::assemble_block,
};
use alloy::primitives::Address;
use alloy::{
    primitives::{TxHash, U64},
    providers::{Provider, RootProvider},
    rpc::types::{BlockId, BlockNumberOrTag},
};
use ark_serialize::SerializationError;
use configuration::addresses::get_addresses;
use jf_plonk::errors::PlonkError;
use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, error, info, warn};
use nightfall_bindings::artifacts::RoundRobin;
use nightfall_client::{
    domain::error::{ConversionError, EventHandlerError, NightfallContractError},
    ports::proof::Proof,
};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use std::{sync::Arc, time::Duration};
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
    let round_robin_instance =
        Arc::new(RoundRobin::new(get_addresses().round_robin, client.clone()));

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
    // we should start this if we have at least one pending block
    let _finality_checker: tokio::task::JoinHandle<Result<(), BlockAssemblyError>> = {
        let pending_blocks = Arc::clone(&pending_blocks);
        let rr = Arc::clone(&round_robin_instance);
        tokio::spawn(async move {
            loop {
                // Check proposer rotation events
                let start_block = match rr.start_l1_block().call().await {
                    Ok(block) => block,
                    Err(_) => {
                        return Err(BlockAssemblyError::FailedToAssembleBlock(
                            "Failed to get start block for round robin".to_string(),
                        ));
                    }
                };
                let mut blocks = pending_blocks.lock().await;
                // If start_block is zero, then we assume the contract has just been deployed and rotation has not yet started.
                if start_block.is_zero() && !blocks.is_empty() {
                    info!("Proposing {} pending blocks", blocks.len());
                    for block in blocks.drain(..) {
                        if let Err(e) = N::propose_block(block).await {
                            error!("Failed to propose block: {e}");
                        }
                    }
                }

                let round_robin_events = rr
                    .event_filter::<RoundRobin::ProposerRotated>()
                    .from_block(0u64);
                let rotate_proposer_log = match round_robin_events.query().await {
                    Ok(logs) => logs,
                    Err(_) => {
                        return Err(BlockAssemblyError::FailedToAssembleBlock(
                            "Failed to query round robin rotate proposer events".to_string(),
                        ));
                    }
                };
                let client = blockchain_client.root().clone();
                for (_, log_meta) in rotate_proposer_log {
                    let tx_hash = log_meta.transaction_hash.ok_or_else(|| {
                        BlockAssemblyError::Other("Transaction hash is None".to_string())
                    })?;
                    match check_l1_finality(
                        &client,
                        tx_hash,
                        confirmations_required,
                        Some(finality_check_interval),
                    )
                    .await
                    {
                        Ok(true) => {
                            // Process all pending blocks
                            info!("Rotate Proposer Transaction finalized: {tx_hash:?}");
                            info!("Proposing {} pending blocks", blocks.len());
                            for block in blocks.drain(..) {
                                if let Err(e) = N::propose_block(block).await {
                                    error!("Failed to propose block: {e}");
                                }
                            }
                        }
                        Ok(false) => {
                            debug!("Transaction not yet finalized");
                        }
                        Err(e) => {
                            error!("Finality check error: {e}");
                        }
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
