use crate::{
    domain::entities::{PendingBlock, PendingBlockState},
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_block_assembly_trigger, get_blockchain_client_connection},
    ports::{
        contracts::{NightfallContract, ProposeBlockOutcome},
        db::{BlockStorageDB, PendingBlockDB},
        proving::RecursiveProvingEngine,
    },
    services::assemble_block::{
        assemble_block, cleanup_selected_transactions, release_selected_transactions,
    },
};
use alloy::{
    primitives::{Address, TxHash, I256, U64},
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

enum PendingBlockQueueAction {
    Completed,
    Requeue(PendingBlock),
}

const MAX_BROADCAST_PENDING_RECEIPT_CHECKS: u32 = 24;

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

async fn release_failed_pending_block<P>(
    db: &mongodb::Client,
    pending_block: &PendingBlock,
) -> Result<(), BlockAssemblyError>
where
    P: Proof,
{
    release_selected_transactions::<P>(
        db,
        &pending_block.selected_deposits,
        &pending_block.selected_client_transaction_hashes,
    )
    .await?;

    if db
        .delete_pending_block(pending_block.layer2_block_number)
        .await
        .is_none()
    {
        warn!(
            "Pending block {} failed to propose and its persisted queue entry could not be removed",
            pending_block.layer2_block_number
        );
    }

    if db
        .delete_block_by_number(pending_block.layer2_block_number)
        .await
        .is_none()
    {
        warn!(
            "Pending block {} failed to propose and its speculative block record could not be removed",
            pending_block.layer2_block_number
        );
    }

    Ok(())
}

async fn propose_and_cleanup_pending_block_with_db<P, N>(
    db: &mongodb::Client,
    pending_block: PendingBlock,
) -> Result<PendingBlockQueueAction, BlockAssemblyError>
where
    P: Proof,
    N: NightfallContract,
{
    if pending_block.state != PendingBlockState::ReadyToPropose {
        return Err(BlockAssemblyError::QueueError(format!(
            "Pending block {} is not ready to propose",
            pending_block.layer2_block_number
        )));
    }

    let block = pending_block.block.clone().ok_or_else(|| {
        BlockAssemblyError::QueueError(format!(
            "Pending block {} is missing block data",
            pending_block.layer2_block_number
        ))
    })?;

    let proposal_outcome = match N::propose_block(block).await {
        Ok(outcome) => outcome,
        Err(e) => {
            if let Err(cleanup_error) = release_failed_pending_block::<P>(db, &pending_block).await
            {
                error!("Failed to release pending block after proposal error: {cleanup_error}");
            }
            return Err(BlockAssemblyError::ContractError(format!(
                "Block proposal failed: {e}"
            )));
        }
    };

    match proposal_outcome {
        ProposeBlockOutcome::Submitted { tx_hash } => {
            info!(
                "Pending block {} proposed successfully with L1 tx {tx_hash:?}",
                pending_block.layer2_block_number
            );
        }
        ProposeBlockOutcome::NotBroadcast { reason } => {
            if let Err(cleanup_error) = release_failed_pending_block::<P>(db, &pending_block).await
            {
                error!(
                    "Failed to release pending block after pre-broadcast failure: {cleanup_error}"
                );
            }
            return Err(BlockAssemblyError::ContractError(format!(
                "Block proposal did not broadcast: {reason:?}"
            )));
        }
        ProposeBlockOutcome::BroadcastUnknown { tx_hash, reason } => {
            let mut broadcast_pending_block = pending_block;
            broadcast_pending_block.state = PendingBlockState::BroadcastPending;
            broadcast_pending_block.broadcast_tx_hash = Some(tx_hash);
            broadcast_pending_block.broadcast_receipt_checks = 0;

            if db
                .store_pending_block(&broadcast_pending_block)
                .await
                .is_none()
            {
                error!(
                    "Failed to persist broadcast-pending state for block {} after ambiguous submission {tx_hash:?}",
                    broadcast_pending_block.layer2_block_number
                );
            }

            warn!(
                "Pending block {} reached broadcast-unknown state for tx {tx_hash:?}: {reason:?}",
                broadcast_pending_block.layer2_block_number
            );
            return Ok(PendingBlockQueueAction::Requeue(broadcast_pending_block));
        }
        ProposeBlockOutcome::Reverted { tx_hash } => {
            if let Err(cleanup_error) = release_failed_pending_block::<P>(db, &pending_block).await
            {
                error!("Failed to release pending block after known revert: {cleanup_error}");
            }
            return Err(BlockAssemblyError::ContractError(format!(
                "Block proposal transaction reverted for tx {tx_hash:?}"
            )));
        }
    }

    cleanup_selected_transactions::<P>(
        db,
        &pending_block.selected_deposits,
        &pending_block.selected_client_transaction_hashes,
    )
    .await?;

    if db
        .delete_pending_block(pending_block.layer2_block_number)
        .await
        .is_none()
    {
        warn!(
            "Pending block {} was proposed, but its persisted queue entry could not be removed",
            pending_block.layer2_block_number
        );
    }
    Ok(PendingBlockQueueAction::Completed)
}

async fn resolve_broadcast_pending_block_with_db<P, N>(
    db: &mongodb::Client,
    pending_block: PendingBlock,
) -> Result<PendingBlockQueueAction, BlockAssemblyError>
where
    P: Proof,
    N: NightfallContract,
{
    let Some(tx_hash) = pending_block.broadcast_tx_hash else {
        if let Err(cleanup_error) = release_failed_pending_block::<P>(db, &pending_block).await {
            error!("Failed to release broadcast-pending block without tx hash: {cleanup_error}");
        }
        return Err(BlockAssemblyError::QueueError(format!(
            "Broadcast-pending block {} has no transaction hash",
            pending_block.layer2_block_number
        )));
    };

    match N::get_proposal_receipt_status(tx_hash).await {
        Ok(Some(true)) => {
            info!(
                "Broadcast-pending block {} was mined successfully in tx {tx_hash:?}",
                pending_block.layer2_block_number
            );
            cleanup_selected_transactions::<P>(
                db,
                &pending_block.selected_deposits,
                &pending_block.selected_client_transaction_hashes,
            )
            .await?;

            if db
                .delete_pending_block(pending_block.layer2_block_number)
                .await
                .is_none()
            {
                warn!(
                    "Broadcast-pending block {} was mined, but its persisted queue entry could not be removed",
                    pending_block.layer2_block_number
                );
            }
            Ok(PendingBlockQueueAction::Completed)
        }
        Ok(Some(false)) => {
            if let Err(cleanup_error) = release_failed_pending_block::<P>(db, &pending_block).await
            {
                error!("Failed to release broadcast-pending block after revert: {cleanup_error}");
            }
            Err(BlockAssemblyError::ContractError(format!(
                "Broadcast-pending block {} reverted in tx {tx_hash:?}",
                pending_block.layer2_block_number
            )))
        }
        Ok(None) => {
            let mut updated_block = pending_block;
            updated_block.broadcast_receipt_checks =
                updated_block.broadcast_receipt_checks.saturating_add(1);

            if updated_block.broadcast_receipt_checks >= MAX_BROADCAST_PENDING_RECEIPT_CHECKS {
                warn!(
                    "Broadcast-pending block {} has no receipt for tx {tx_hash:?} after {} checks; releasing reserved transactions",
                    updated_block.layer2_block_number,
                    updated_block.broadcast_receipt_checks
                );
                if let Err(cleanup_error) =
                    release_failed_pending_block::<P>(db, &updated_block).await
                {
                    error!("Failed to release expired broadcast-pending block: {cleanup_error}");
                }
                return Err(BlockAssemblyError::ContractError(format!(
                    "Broadcast-pending block {} never produced a receipt for tx {tx_hash:?}",
                    updated_block.layer2_block_number
                )));
            }

            if db.store_pending_block(&updated_block).await.is_none() {
                error!(
                    "Failed to persist receipt-check count for broadcast-pending block {}",
                    updated_block.layer2_block_number
                );
            }

            info!(
                "Broadcast-pending block {} still has no receipt for tx {tx_hash:?} ({}/{})",
                updated_block.layer2_block_number,
                updated_block.broadcast_receipt_checks,
                MAX_BROADCAST_PENDING_RECEIPT_CHECKS
            );
            Ok(PendingBlockQueueAction::Requeue(updated_block))
        }
        Err(e) => {
            warn!(
                "Could not check receipt for broadcast-pending block {} tx {tx_hash:?}: {e}",
                pending_block.layer2_block_number
            );
            Ok(PendingBlockQueueAction::Requeue(pending_block))
        }
    }
}

async fn recover_pending_blocks<P>(
    db: &mongodb::Client,
) -> Result<Vec<PendingBlock>, BlockAssemblyError>
where
    P: Proof,
{
    let mut recovered_pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
    let mut ready_pending_blocks = Vec::new();

    for pending_block in recovered_pending_blocks.drain(..) {
        match pending_block.state {
            PendingBlockState::Reserved => {
                if let Err(e) = release_selected_transactions::<P>(
                    db,
                    &pending_block.selected_deposits,
                    &pending_block.selected_client_transaction_hashes,
                )
                .await
                {
                    warn!(
                        "Failed to release reserved transactions for recovered block {}: {e}",
                        pending_block.layer2_block_number
                    );
                }

                if db
                    .delete_pending_block(pending_block.layer2_block_number)
                    .await
                    .is_none()
                {
                    warn!(
                        "Recovered reserved block {} could not be removed from pending storage",
                        pending_block.layer2_block_number
                    );
                }

                let _ = db
                    .delete_block_by_number(pending_block.layer2_block_number)
                    .await;
            }
            PendingBlockState::ReadyToPropose | PendingBlockState::BroadcastPending => {
                ready_pending_blocks.push(pending_block)
            }
        }
    }

    ready_pending_blocks.sort_by_key(|pending_block| pending_block.layer2_block_number);
    if ready_pending_blocks.len() > 1 {
        return Err(BlockAssemblyError::QueueError(
            "Expected at most one pending block in storage".to_string(),
        ));
    }

    Ok(ready_pending_blocks)
}

async fn process_pending_blocks_for_proposal<P, N>(
    pending_blocks: &Arc<Mutex<Vec<PendingBlock>>>,
    phase: &str,
) where
    P: Proof,
    N: NightfallContract,
{
    let drained: Vec<_> = {
        let mut guard = pending_blocks.lock().await;
        guard.drain(..).collect()
    };

    if drained.is_empty() {
        return;
    }

    info!(
        "Finality checker: {phase}, processing {} pending block(s)",
        drained.len()
    );

    let db = crate::initialisation::get_db_connection().await;
    let mut blocks_to_requeue = Vec::new();

    for pending_block in drained {
        match pending_block.state {
            PendingBlockState::ReadyToPropose => {
                match propose_and_cleanup_pending_block_with_db::<P, N>(db, pending_block.clone())
                    .await
                {
                    Ok(PendingBlockQueueAction::Completed) => {}
                    Ok(PendingBlockQueueAction::Requeue(updated_block)) => {
                        blocks_to_requeue.push(updated_block);
                    }
                    Err(e) => {
                        error!("Finality checker: propose_block failed: {e}");
                    }
                }
            }
            PendingBlockState::BroadcastPending => {
                let Some(stored_pending_block) = db
                    .get_pending_block(pending_block.layer2_block_number)
                    .await
                else {
                    info!(
                        "Finality checker: dropping resolved broadcast-pending block {}",
                        pending_block.layer2_block_number
                    );
                    continue;
                };

                match resolve_broadcast_pending_block_with_db::<P, N>(db, stored_pending_block)
                    .await
                {
                    Ok(PendingBlockQueueAction::Completed) => {}
                    Ok(PendingBlockQueueAction::Requeue(updated_block)) => {
                        blocks_to_requeue.push(updated_block);
                    }
                    Err(e) => {
                        error!("Finality checker: broadcast-pending resolution failed: {e}");
                    }
                }
            }
            PendingBlockState::Reserved => {
                warn!(
                    "Finality checker: reserved pending block {} should not be queued; dropping it",
                    pending_block.layer2_block_number
                );
            }
        }
    }

    if !blocks_to_requeue.is_empty() {
        let mut guard = pending_blocks.lock().await;
        guard.extend(blocks_to_requeue);
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

    let db = crate::initialisation::get_db_connection().await;
    let ready_pending_blocks = recover_pending_blocks::<P>(db).await?;

    // Shared queue for blocks waiting for finality confirmation
    let pending_blocks = Arc::new(Mutex::new(ready_pending_blocks));
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

                let onchain_current_proposer = match rr.get_current_proposer_address().call().await
                {
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
                    process_pending_blocks_for_proposal::<P, N>(
                        &pending_blocks,
                        "current proposer turn already finalized",
                    )
                    .await;

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

                        process_pending_blocks_for_proposal::<P, N>(
                            &pending_blocks,
                            "finalized canonical rotation",
                        )
                        .await;
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
        let has_pending = {
            let blocks = pending_blocks.lock().await;
            !blocks.is_empty()
        };
        if has_pending {
            debug!("Pending block already in flight; waiting before assembling another block");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

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
        if current_block_number < I256::ZERO {
            error!(
                "Block assembly aborted: contract returned negative current block number {current_block_number}"
            );
            continue;
        }
        let block_result = assemble_block::<P, R>(current_block_number.as_u64()).await;
        let pending_block = match block_result {
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
            blocks.push(pending_block);
            info!("Added block to queue ({} pending)", blocks.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::{
            Block, ClientTransactionWithMetaData, DepositDatawithFee, PendingBlockState,
            TxLifecycle,
        },
        driven::db::mongo_db::StoredBlock,
        ports::contracts::{BroadcastUnknownReason, ProposeBlockOutcome},
        ports::db::{PendingBlockDB, TransactionsDB},
        services::assemble_block::reserve_selected_transactions,
    };
    use alloy::primitives::{TxHash, I256};
    use lib::{
        error::NightfallContractError,
        plonk_prover::plonk_proof::PlonkProof,
        shared_entities::DepositData,
        tests_utils::{get_db_connection, get_mongo},
    };

    struct FailingContract;

    #[async_trait::async_trait]
    impl NightfallContract for FailingContract {
        async fn propose_block(
            _block: Block,
        ) -> Result<ProposeBlockOutcome, NightfallContractError> {
            Ok(ProposeBlockOutcome::NotBroadcast {
                reason: crate::ports::contracts::NotBroadcastReason::SendRawTransactionFailed,
            })
        }

        async fn get_proposal_receipt_status(
            _tx_hash: TxHash,
        ) -> Result<Option<bool>, NightfallContractError> {
            Ok(None)
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Ok(I256::ZERO)
        }
    }

    struct BroadcastUnknownContract;

    #[async_trait::async_trait]
    impl NightfallContract for BroadcastUnknownContract {
        async fn propose_block(
            _block: Block,
        ) -> Result<ProposeBlockOutcome, NightfallContractError> {
            Ok(ProposeBlockOutcome::BroadcastUnknown {
                tx_hash: TxHash::from([7u8; 32]),
                reason: BroadcastUnknownReason::ReceiptUnavailable,
            })
        }

        async fn get_proposal_receipt_status(
            _tx_hash: TxHash,
        ) -> Result<Option<bool>, NightfallContractError> {
            Ok(None)
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Ok(I256::ZERO)
        }
    }

    struct MissingReceiptContract;

    #[async_trait::async_trait]
    impl NightfallContract for MissingReceiptContract {
        async fn propose_block(
            _block: Block,
        ) -> Result<ProposeBlockOutcome, NightfallContractError> {
            panic!("MissingReceiptContract should only be used for receipt checks")
        }

        async fn get_proposal_receipt_status(
            _tx_hash: TxHash,
        ) -> Result<Option<bool>, NightfallContractError> {
            Ok(None)
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Ok(I256::ZERO)
        }
    }

    struct SuccessfulReceiptContract;

    #[async_trait::async_trait]
    impl NightfallContract for SuccessfulReceiptContract {
        async fn propose_block(
            _block: Block,
        ) -> Result<ProposeBlockOutcome, NightfallContractError> {
            panic!("SuccessfulReceiptContract should only be used for receipt checks")
        }

        async fn get_proposal_receipt_status(
            _tx_hash: TxHash,
        ) -> Result<Option<bool>, NightfallContractError> {
            Ok(Some(true))
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Ok(I256::ZERO)
        }
    }

    struct RevertedReceiptContract;

    #[async_trait::async_trait]
    impl NightfallContract for RevertedReceiptContract {
        async fn propose_block(
            _block: Block,
        ) -> Result<ProposeBlockOutcome, NightfallContractError> {
            panic!("RevertedReceiptContract should only be used for receipt checks")
        }

        async fn get_proposal_receipt_status(
            _tx_hash: TxHash,
        ) -> Result<Option<bool>, NightfallContractError> {
            Ok(Some(false))
        }

        async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
            Ok(I256::ZERO)
        }
    }

    fn test_client_transaction(fee: u64) -> ClientTransactionWithMetaData<PlonkProof> {
        ClientTransactionWithMetaData {
            client_transaction: lib::shared_entities::ClientTransaction {
                fee: ark_bn254::Fr::from(fee),
                proof: PlonkProof::default(),
                ..Default::default()
            },
            lifecycle: TxLifecycle::Mempool,
            hash: vec![fee as u32],
            historic_roots: vec![ark_bn254::Fr::from(123u64)],
        }
    }

    fn test_deposit(fee: u64) -> DepositDatawithFee {
        DepositDatawithFee {
            fee: ark_bn254::Fr::from(fee),
            deposit_data: DepositData {
                nf_token_id: ark_bn254::Fr::from(fee),
                nf_slot_id: ark_bn254::Fr::from(fee),
                value: ark_bn254::Fr::from(100u64),
                secret_hash: ark_bn254::Fr::from(fee),
            },
            reserved: false,
        }
    }

    async fn stored_reserved_pending_block(
        db: &mongodb::Client,
        block_number: u64,
        state: PendingBlockState,
        broadcast_tx_hash: Option<TxHash>,
        broadcast_receipt_checks: u32,
    ) -> PendingBlock {
        let selected_deposits = vec![(1..=4).map(test_deposit).collect::<Vec<_>>()];
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(
            db,
            selected_deposits[0].clone(),
        )
        .await;

        let selected_client_transactions =
            (401..=404).map(test_client_transaction).collect::<Vec<_>>();
        for tx in &selected_client_transactions {
            db.store_transaction(tx.clone()).await.unwrap();
        }

        let selected_client_transaction_hashes = selected_client_transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect::<Vec<_>>();
        reserve_selected_transactions::<PlonkProof>(
            db,
            &selected_deposits,
            &selected_client_transactions,
            block_number,
        )
        .await
        .unwrap();

        let pending_block = PendingBlock {
            layer2_block_number: block_number,
            state,
            broadcast_tx_hash,
            broadcast_receipt_checks,
            block: Some(Block::default()),
            selected_deposits: selected_deposits.clone(),
            selected_client_transaction_hashes: selected_client_transaction_hashes.clone(),
        };
        db.store_pending_block(&pending_block).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: block_number,
            commitments: vec!["broadcast-pending".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .unwrap();

        pending_block
    }

    #[tokio::test]
    async fn test_proposal_failure_releases_mempool_and_clears_pending_state() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        let selected_deposits = vec![(1..=4).map(test_deposit).collect::<Vec<_>>()];
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(
            &db,
            selected_deposits[0].clone(),
        )
        .await;

        let selected_client_transactions =
            (101..=104).map(test_client_transaction).collect::<Vec<_>>();
        for tx in &selected_client_transactions {
            db.store_transaction(tx.clone()).await.unwrap();
        }

        let selected_client_transaction_hashes = selected_client_transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect::<Vec<_>>();
        reserve_selected_transactions::<PlonkProof>(
            &db,
            &selected_deposits,
            &selected_client_transactions,
            0,
        )
        .await
        .unwrap();

        let pending_block = PendingBlock {
            layer2_block_number: 0,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: selected_deposits.clone(),
            selected_client_transaction_hashes: selected_client_transaction_hashes.clone(),
        };
        db.store_pending_block(&pending_block).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: 0,
            commitments: vec!["test-commitment".to_string()],
            proposer_address: Address::from([9u8; 20]),
        })
        .await
        .unwrap();

        let result = propose_and_cleanup_pending_block_with_db::<PlonkProof, FailingContract>(
            &db,
            pending_block,
        )
        .await;

        assert!(matches!(
            result,
            Err(BlockAssemblyError::ContractError(message))
                if message.contains("did not broadcast")
        ));

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap();
        assert_eq!(available_deposits.len(), 4);

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert_eq!(available_client_transactions.len(), 4);

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert!(pending_blocks.is_empty());

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert!(stored_blocks.is_empty());
    }

    #[tokio::test]
    async fn test_recover_reserved_pending_block_releases_mempool_and_drops_orphaned_state() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        let selected_deposits = vec![(1..=4).map(test_deposit).collect::<Vec<_>>()];
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(
            &db,
            selected_deposits[0].clone(),
        )
        .await;

        let selected_client_transactions =
            (201..=204).map(test_client_transaction).collect::<Vec<_>>();
        for tx in &selected_client_transactions {
            db.store_transaction(tx.clone()).await.unwrap();
        }

        let selected_client_transaction_hashes = selected_client_transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect::<Vec<_>>();
        reserve_selected_transactions::<PlonkProof>(
            &db,
            &selected_deposits,
            &selected_client_transactions,
            0,
        )
        .await
        .unwrap();

        let pending_block = PendingBlock {
            layer2_block_number: 0,
            state: PendingBlockState::Reserved,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: None,
            selected_deposits: selected_deposits.clone(),
            selected_client_transaction_hashes: selected_client_transaction_hashes.clone(),
        };
        db.store_pending_block(&pending_block).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: 0,
            commitments: vec!["orphaned-speculative-block".to_string()],
            proposer_address: Address::from([4u8; 20]),
        })
        .await
        .unwrap();

        let recovered = recover_pending_blocks::<PlonkProof>(&db).await.unwrap();

        assert!(recovered.is_empty());

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap();
        assert_eq!(available_deposits.len(), 4);

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert_eq!(available_client_transactions.len(), 4);

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert!(pending_blocks.is_empty());

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert!(stored_blocks.is_empty());
    }

    #[tokio::test]
    async fn test_broadcast_unknown_keeps_pending_state_and_reserved_inputs() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        let selected_deposits = vec![(1..=4).map(test_deposit).collect::<Vec<_>>()];
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(
            &db,
            selected_deposits[0].clone(),
        )
        .await;

        let selected_client_transactions =
            (301..=304).map(test_client_transaction).collect::<Vec<_>>();
        for tx in &selected_client_transactions {
            db.store_transaction(tx.clone()).await.unwrap();
        }

        let selected_client_transaction_hashes = selected_client_transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect::<Vec<_>>();
        reserve_selected_transactions::<PlonkProof>(
            &db,
            &selected_deposits,
            &selected_client_transactions,
            0,
        )
        .await
        .unwrap();

        let pending_block = PendingBlock {
            layer2_block_number: 0,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block::default()),
            selected_deposits: selected_deposits.clone(),
            selected_client_transaction_hashes: selected_client_transaction_hashes.clone(),
        };
        db.store_pending_block(&pending_block).await.unwrap();
        db.store_block(&StoredBlock {
            layer2_block_number: 0,
            commitments: vec!["broadcast-unknown".to_string()],
            proposer_address: Address::from([6u8; 20]),
        })
        .await
        .unwrap();

        let result = propose_and_cleanup_pending_block_with_db::<
            PlonkProof,
            BroadcastUnknownContract,
        >(&db, pending_block)
        .await
        .unwrap();

        assert!(matches!(
            result,
            PendingBlockQueueAction::Requeue(PendingBlock {
                state: PendingBlockState::BroadcastPending,
                ..
            })
        ));

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap_or_default();
        assert!(available_deposits.is_empty());

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert!(available_client_transactions.is_empty());

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert_eq!(pending_blocks.len(), 1);
        assert_eq!(pending_blocks[0].state, PendingBlockState::BroadcastPending);
        assert_eq!(
            pending_blocks[0].broadcast_tx_hash,
            Some(TxHash::from([7u8; 32]))
        );
        assert_eq!(pending_blocks[0].broadcast_receipt_checks, 0);

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert_eq!(stored_blocks.len(), 1);
    }

    #[tokio::test]
    async fn test_broadcast_pending_missing_receipt_requeues_and_increments_check_count() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let tx_hash = TxHash::from([8u8; 32]);
        let pending_block = stored_reserved_pending_block(
            &db,
            0,
            PendingBlockState::BroadcastPending,
            Some(tx_hash),
            0,
        )
        .await;

        let result = resolve_broadcast_pending_block_with_db::<PlonkProof, MissingReceiptContract>(
            &db,
            pending_block,
        )
        .await
        .unwrap();

        let PendingBlockQueueAction::Requeue(updated_block) = result else {
            panic!("missing receipt should keep the pending block queued");
        };
        assert_eq!(updated_block.broadcast_tx_hash, Some(tx_hash));
        assert_eq!(updated_block.broadcast_receipt_checks, 1);

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert_eq!(pending_blocks.len(), 1);
        assert_eq!(pending_blocks[0].broadcast_receipt_checks, 1);

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap_or_default();
        assert!(available_deposits.is_empty());
    }

    #[tokio::test]
    async fn test_broadcast_pending_missing_receipt_eventually_releases_reserved_inputs() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let pending_block = stored_reserved_pending_block(
            &db,
            0,
            PendingBlockState::BroadcastPending,
            Some(TxHash::from([9u8; 32])),
            MAX_BROADCAST_PENDING_RECEIPT_CHECKS - 1,
        )
        .await;

        let result = resolve_broadcast_pending_block_with_db::<PlonkProof, MissingReceiptContract>(
            &db,
            pending_block,
        )
        .await;

        assert!(matches!(
            result,
            Err(BlockAssemblyError::ContractError(message))
                if message.contains("never produced a receipt")
        ));

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap();
        assert_eq!(available_deposits.len(), 4);

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert_eq!(available_client_transactions.len(), 4);

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert!(pending_blocks.is_empty());

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert!(stored_blocks.is_empty());
    }

    #[tokio::test]
    async fn test_broadcast_pending_successful_receipt_cleans_up_reserved_inputs() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let pending_block = stored_reserved_pending_block(
            &db,
            0,
            PendingBlockState::BroadcastPending,
            Some(TxHash::from([10u8; 32])),
            3,
        )
        .await;

        let result =
            resolve_broadcast_pending_block_with_db::<PlonkProof, SuccessfulReceiptContract>(
                &db,
                pending_block,
            )
            .await
            .unwrap();

        assert!(matches!(result, PendingBlockQueueAction::Completed));

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap_or_default();
        assert!(available_deposits.is_empty());

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert!(available_client_transactions.is_empty());

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert!(pending_blocks.is_empty());

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert_eq!(stored_blocks.len(), 1);
    }

    #[tokio::test]
    async fn test_broadcast_pending_reverted_receipt_releases_reserved_inputs() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let pending_block = stored_reserved_pending_block(
            &db,
            0,
            PendingBlockState::BroadcastPending,
            Some(TxHash::from([11u8; 32])),
            3,
        )
        .await;

        let result =
            resolve_broadcast_pending_block_with_db::<PlonkProof, RevertedReceiptContract>(
                &db,
                pending_block,
            )
            .await;

        assert!(matches!(
            result,
            Err(BlockAssemblyError::ContractError(message))
                if message.contains("reverted")
        ));

        let available_deposits =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap();
        assert_eq!(available_deposits.len(), 4);

        let available_client_transactions =
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_all_mempool_client_transactions(
                &db,
            )
            .await
            .unwrap_or_default();
        assert_eq!(available_client_transactions.len(), 4);

        let pending_blocks = db.get_all_pending_blocks().await.unwrap_or_default();
        assert!(pending_blocks.is_empty());

        let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
        assert!(stored_blocks.is_empty());
    }
}
