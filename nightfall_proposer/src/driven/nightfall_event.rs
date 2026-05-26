use crate::{
    domain::entities::DepositDatawithFee,
    domain::entities::{L1Ref, SyncState},
    driven::{
        db::mongo_db::StoredBlock, nightfall_client_transaction::process_deposit_transaction,
    },
    drivers::blockchain::nightfall_event_listener::{
        get_synchronisation_status, is_listener_replay_catch_up_pending,
    },
    initialisation::{
        begin_startup_replay_reset, get_blockchain_client_connection, get_db_connection,
    },
    ports::{
        contracts::NightfallContract,
        db::{BlockStorageDB, PendingBlockDB, SyncStateDB},
        events::EventHandler,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
    services::assemble_block::{cleanup_selected_transactions, release_selected_transactions},
    services::selected_transactions::reconcile_active_client_transaction_lifecycle,
    services::snapshot_scheduler::maybe_schedule_snapshot_for_applied_block,
};
use alloy::primitives::{TxHash, I256};
use alloy::rpc::types::Log;
use alloy::{consensus::Transaction, sol_types::SolInterface};
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger;
use lib::{
    blockchain_client::BlockchainClientConnection,
    contract_conversions::FrBn254,
    error::EventHandlerError,
    get_fee_token_id,
    hex_conversion::HexConvertible,
    nf_client_proof::{Proof, ProvingEngine},
    nf_token_id::to_nf_token_id_from_solidity,
    shared_entities::DepositData,
    shared_entities::OnChainTransaction,
};
use log::{debug, error, info, warn};
use mongodb::Client;
use nightfall_bindings::artifacts::Nightfall;
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use tokio::sync::{OnceCell, RwLock};

#[cfg(test)]
use std::{collections::HashSet, sync::Mutex};

fn merkle_tree_error_to_mongo(
    error: lib::merkle_trees::trees::MerkleTreeError<mongodb::error::Error>,
) -> mongodb::error::Error {
    match error {
        lib::merkle_trees::trees::MerkleTreeError::DatabaseError(db_error) => db_error,
        other => mongodb::error::Error::custom(other.to_string()),
    }
}
// Define a mutable lazy static to hold the layer 2 blocknumber. We need this to
// check if we're still in sync, but putting it in the context would mean passing it around too much
pub async fn get_expected_layer2_blocknumber() -> &'static RwLock<I256> {
    static LAYER2_BLOCKNUMBER: OnceCell<RwLock<I256>> = OnceCell::const_new();
    LAYER2_BLOCKNUMBER
        .get_or_init(|| async { RwLock::new(I256::ZERO) })
        .await
}

#[cfg(test)]
static ENABLED_BLOCK_APPLY_FAILPOINTS: std::sync::OnceLock<Mutex<HashSet<String>>> =
    std::sync::OnceLock::new();

#[cfg(test)]
fn enabled_block_apply_failpoints() -> &'static Mutex<HashSet<String>> {
    ENABLED_BLOCK_APPLY_FAILPOINTS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn maybe_fail_block_apply(_name: &str) -> Result<(), mongodb::error::Error> {
    #[cfg(test)]
    {
        if enabled_block_apply_failpoints()
            .lock()
            .expect("block apply failpoint lock poisoned")
            .contains(_name)
        {
            return Err(mongodb::error::Error::custom(format!(
                "Simulated block apply failure at {_name}"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
struct TestBlockApplyFailpointGuard {
    name: String,
}

#[cfg(test)]
impl TestBlockApplyFailpointGuard {
    fn enable(name: &str) -> Self {
        enabled_block_apply_failpoints()
            .lock()
            .expect("block apply failpoint lock poisoned")
            .insert(name.to_string());
        Self {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
impl Drop for TestBlockApplyFailpointGuard {
    fn drop(&mut self) {
        enabled_block_apply_failpoints()
            .lock()
            .expect("block apply failpoint lock poisoned")
            .remove(&self.name);
    }
}

#[derive(Debug)]
pub enum ProcessBlockError {
    CouldNotStoreHistoricRoot,
}

impl Error for ProcessBlockError {}

impl Display for ProcessBlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessBlockError::CouldNotStoreHistoricRoot => {
                write!(f, "Could not store historic root")
            }
        }
    }
}

// This is similar to Client's event handler but we don't simply import that version because
// eventually this implementation will diverge from the Client's implementation.
#[async_trait::async_trait]
impl<P, E, N> EventHandler<P, E, N> for Nightfall::NightfallEvents
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    async fn handle_event(&self, log: Log) -> Result<(), EventHandlerError> {
        // we'll split out individual events here in case that's useful later
        let tx_hash = log.transaction_hash.ok_or_else(|| {
            EventHandlerError::IOError("Event log missing transaction_hash".to_string())
        })?;
        debug!("Handling event {self:?} for transaction {tx_hash:?}");
        match &self {
            Nightfall::NightfallEvents::BlockProposed(filter) => {
                process_nightfall_calldata::<P, E, N>(tx_hash, filter.layer2_block_number, &log)
                    .await?
            }
            Nightfall::NightfallEvents::DepositEscrowed(filter) => {
                info!("Received DepositEscrowed event");
                process_deposit_escrowed_event::<P, E>(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{e}");
                        EventHandlerError::InvalidCalldata
                    })?;
            }
            Nightfall::NightfallEvents::Initialized(_filter) => {
                info!("Received Initialized event");
            }
            Nightfall::NightfallEvents::Upgraded(_filter) => {
                info!("Received Upgraded event");
            }
            Nightfall::NightfallEvents::AuthoritiesUpdated(_filter) => {
                info!("Received AuthoritiesUpdated event");
            }
            Nightfall::NightfallEvents::OwnershipTransferred(_filter) => {
                info!("Received OwnershipTransferred event");
            }
        }
        Ok(())
        // all events, however, can be processed by the same function because you just need the tx hash to get the calldata
    }
}

pub async fn process_nightfall_calldata<P, E, N>(
    transaction_hash: TxHash,
    block_number: I256,
    log: &Log,
) -> Result<(), EventHandlerError>
where
    P: Proof + Send + Serialize + Clone + Debug + Sync,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    // get the transaction
    let tx = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction_by_hash(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?;

    // if there is one, decode it. If not, throw.
    if let Some(tx) = tx {
        let decoded = Nightfall::NightfallCalls::abi_decode(tx.input())
            .map_err(|_| EventHandlerError::InvalidCalldata)?;
        if let Nightfall::NightfallCalls::propose_block(decode) = decoded {
            // OK to use unwrap because the smart contract has to provide a block number
            process_propose_block_event::<P, N>(decode, transaction_hash, block_number, log)
                .await?;
        }
    } else {
        panic!("Transaction not found when looking up calldata");
    }
    Ok(())
}

fn stored_block_from_pending_block(
    pending_block: &crate::domain::entities::PendingBlock,
    proposer_address: alloy::primitives::Address,
) -> Option<StoredBlock> {
    let block = pending_block.block.as_ref()?;

    Some(StoredBlock {
        layer2_block_number: pending_block.layer2_block_number,
        commitments: block
            .transactions
            .iter()
            .flat_map(|ntx| {
                ntx.commitments
                    .iter()
                    .map(|c| c.to_hex_string())
                    .collect::<Vec<_>>()
            })
            .collect(),
        proposer_address,
    })
}

async fn finalize_pending_block_after_applied_block<P>(
    db: &Client,
    pending_block: crate::domain::entities::PendingBlock,
    our_address: alloy::primitives::Address,
    applied_block: &StoredBlock,
) where
    P: Proof,
{
    let cleanup_result = if let Some(pending_block_hash) =
        stored_block_from_pending_block(&pending_block, our_address).map(|block| block.hash())
    {
        if pending_block_hash == applied_block.hash() {
            cleanup_selected_transactions::<P>(
                db,
                &pending_block.selected_deposits,
                &pending_block.selected_client_transaction_hashes,
            )
            .await
        } else {
            release_selected_transactions::<P>(
                db,
                &pending_block.selected_deposits,
                &pending_block.selected_client_transaction_hashes,
            )
            .await
        }
    } else {
        release_selected_transactions::<P>(
            db,
            &pending_block.selected_deposits,
            &pending_block.selected_client_transaction_hashes,
        )
        .await
    };

    if let Err(error) = cleanup_result {
        warn!(
            "Keeping PendingBlock {} after canonical L2 block application because lifecycle \
             cleanup did not complete safely: {error}",
            pending_block.layer2_block_number
        );
        return;
    }

    if db
        .delete_pending_block(pending_block.layer2_block_number)
        .await
        .is_none()
    {
        warn!(
            "PendingBlock {} matched canonical application cleanup but could not be deleted; \
             keeping persisted recovery state for later cleanup",
            pending_block.layer2_block_number
        );
    }
}

async fn cleanup_mismatched_proposer_block_state(
    db: &Client,
    block_number: u64,
) -> Result<(), EventHandlerError> {
    let db_for_cleanup = db.clone();
    let mut session = db.start_session().await.map_err(|_| {
        EventHandlerError::IOError(
            "Could not start MongoDB session for mismatch cleanup".to_string(),
        )
    })?;

    session
        .start_transaction()
        .and_run2(async move |session| {
            db_for_cleanup
                .delete_block_by_number_with_session(block_number, session)
                .await?;
            Ok::<(), mongodb::error::Error>(())
        })
        .await
        .map_err(|e| {
            EventHandlerError::IOError(format!(
                "Could not clean up mismatched proposer block state: {e}"
            ))
        })
}

async fn process_propose_block_event<P, N>(
    decode: Nightfall::propose_blockCall,
    transaction_hash: TxHash,
    layer_2_block_number_in_event: I256,
    log: &Log,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    N: NightfallContract,
{
    let our_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address();

    let sender_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction_by_hash(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?
        .unwrap()
        .inner
        .signer();

    // get a lock on the db, we don't want anything else updating or reading the DB until
    // we're done here
    let db = get_db_connection().await;
    info!("Decoded Proposed block call from transaction {transaction_hash:?}");
    let blk = decode.blk;

    let layer_2_block_number_in_event_u64: u64 = layer_2_block_number_in_event
        .try_into()
        .expect("I256 to u64 conversion failed");
    let store_block_pending = StoredBlock {
        layer2_block_number: layer_2_block_number_in_event_u64,
        commitments: blk
            .transactions
            .iter()
            .flat_map(|ntx| {
                let tx: OnChainTransaction = (*ntx).clone().into();
                tx.commitments
                    .iter()
                    .map(|c| c.to_hex_string())
                    .collect::<Vec<_>>()
            })
            .collect(),
        proposer_address: sender_address,
    };

    // check and update the sychronisation status
    let mut sync_status = get_synchronisation_status().await.write().await;
    let was_synchronised = sync_status.is_synchronised();
    // The first thing to do is to make sure that we've not missed any blocks.
    // If we have, then we'll need to resynchronise with the blockchain.
    let mut expected_onchain_block_number = get_expected_layer2_blocknumber().await.write().await;

    if *expected_onchain_block_number < layer_2_block_number_in_event {
        // we've missed at least one block
        warn!(
            "Out of sync with blockchain. Blocknumber of event was {layer_2_block_number_in_event}, expected {expected_onchain_block_number}"
        );
        sync_status.clear_synchronised();
        //The event listener infrastructure (via start_event_listener and restart_event_listener) is responsible for replaying historical events to fill in the gap.
        return Err(EventHandlerError::MissingBlocks(
            expected_onchain_block_number.as_usize(),
        ));
    }

    // check if we're ahead of the event, this means we've already seen it and we shouldn't process it again
    // This could happen if we've missed some blocks and we're re-synchronising
    if *expected_onchain_block_number > layer_2_block_number_in_event {
        warn!("Already processed layer 2 block {layer_2_block_number_in_event} - skipping");
        return Ok(());
    }

    // if expected_onchain_block_number == layer_2_block_number, we need to check if the block hash is the same
    // if it's not, then we need to re-synchronise.
    // what can cause this situation?
    // 1) If proposer_1 failed to propose a block, and proposer_2
    // proposed the same block, proposer_1 need to re-synchronise otherwise it will assemble next block with a wrong status.
    // 2) If chain reorganisation happened, proposers need to re-synchronise.

    // get the block from the db and compute the block hash
    let expected_block_number_u64: u64 = (*expected_onchain_block_number)
        .try_into()
        .expect("I256 to u64 conversion failed");
    // if proposer is out of sync, it won't have this block in db
    let current_block_stored = db.get_block_by_number(expected_block_number_u64).await;
    let pending_block = db.get_pending_block(expected_block_number_u64).await;

    match current_block_stored {
        Some(current_block) => {
            let current_block_stored_hash = current_block.hash();
            let block_store_pending_hash = store_block_pending.hash();

            if expected_block_number_u64 == layer_2_block_number_in_event_u64
                && current_block_stored_hash != block_store_pending_hash
            {
                warn!(
                    "Block hash mismatch. Expected {current_block_stored_hash}, got {block_store_pending_hash} in layer 2 block {layer_2_block_number_in_event}"
                );

                begin_startup_replay_reset(db).await.map_err(|error| {
                    EventHandlerError::IOError(format!(
                        "Could not persist startup replay reset marker before mismatch recovery: {error}"
                    ))
                })?;

                if let Some(pending_block) = pending_block.as_ref() {
                    let _ = release_selected_transactions::<P>(
                        db,
                        &pending_block.selected_deposits,
                        &pending_block.selected_client_transaction_hashes,
                    )
                    .await;
                    let _ = db.delete_pending_block(expected_block_number_u64).await;
                }

                cleanup_mismatched_proposer_block_state(db, expected_block_number_u64).await?;

                sync_status.clear_synchronised();

                return Err(EventHandlerError::BlockHashError(
                    current_block_stored_hash,
                    block_store_pending_hash,
                ));
            }
        }

        None => {
            warn!(
                "No block found in DB at expected height {expected_block_number_u64}. Assuming fresh state or first sync."
            );
        }
    }

    // warn that we're not synced with the blockchain if we're behind
    // before we used the event filter layer 2 block number
    // now we get the current_block_number from the blockchain
    // what's the difference?
    let current_block_number_in_contract =
        N::get_current_layer2_blocknumber().await.map_err(|_| {
            EventHandlerError::IOError("Could not retrieve current block number".to_string())
        })?;

    let replay_catch_up_pending = is_listener_replay_catch_up_pending();

    // next, we'll unpack the commitments and add them to the proposer's commitment tree
    // normally, we don't update the trees if we're the proposer, because we'll have done it when we proposed the block
    // but if we're not in sync then we need to get this information from the blockchain.
    // There's one more case, where this is the first block, so we must be synchronised in the sense that our block count is the
    // same as the blockchain's block count, but we've lost the commitment data. In this case, we need to update the trees too.
    // If we don't have the data from the first block, out commitment root will be zero.
    let first_block_is_already_caught_up =
        current_block_number_in_contract == I256::ONE && !replay_catch_up_pending;
    let commitment_root = <Client as CommitmentTree<Fr254>>::get_root(db)
        .await
        .map_err(|_| {
            EventHandlerError::IOError("Could not retrieve commitment root".to_string())
        })?;
    let should_refresh_local_trees = our_address != sender_address
        || (!sync_status.is_synchronised() && !first_block_is_already_caught_up)
        || commitment_root.0.is_zero();
    let commitments = if should_refresh_local_trees {
        blk.transactions
            .iter()
            .flat_map(|transaction| &transaction.commitments)
            .map(|u| FrBn254::try_from(*u).map(|f| f.into()))
            .collect::<Result<Vec<Fr254>, _>>()
            .expect("Could not convert commitments to U256")
    } else {
        Vec::new()
    };
    let nullifiers = if should_refresh_local_trees {
        blk.transactions
            .iter()
            .flat_map(|transaction| &transaction.nullifiers)
            .map(|u| FrBn254::try_from(*u).map(|f| f.into()))
            .collect::<Result<Vec<Fr254>, _>>()
            .expect("Could not convert nullifiers to U256")
    } else {
        Vec::new()
    };

    let historic_root: Fr254 = FrBn254::try_from(blk.commitments_root)
        .map_err(|_| EventHandlerError::IOError("Could not convert to Fr254".to_string()))?
        .into();
    let sync_state_l1_ref = L1Ref {
        block_number: log.block_number.ok_or_else(|| {
            EventHandlerError::IOError("BlockProposed log missing block_number".to_string())
        })?,
        tx_hash: log.transaction_hash.ok_or_else(|| {
            EventHandlerError::IOError("BlockProposed log missing transaction_hash".to_string())
        })?,
        log_index: log.log_index.ok_or_else(|| {
            EventHandlerError::IOError("BlockProposed log missing log_index".to_string())
        })?,
    };
    let db_for_transaction = db.clone();
    let commitments_for_transaction = commitments.clone();
    let nullifiers_for_transaction = nullifiers.clone();
    let block_for_transaction = store_block_pending.clone();
    let historic_root_for_transaction = historic_root;
    let sync_state_l1_ref_for_transaction = sync_state_l1_ref.clone();
    let should_refresh_local_trees_for_transaction = should_refresh_local_trees;
    let commitment_root = apply_proposer_block_transaction(
        db_for_transaction,
        should_refresh_local_trees_for_transaction,
        commitments_for_transaction,
        nullifiers_for_transaction,
        historic_root_for_transaction,
        block_for_transaction,
        sync_state_l1_ref_for_transaction,
    )
    .await?;

    *expected_onchain_block_number += I256::ONE; // move on to the next block after durable commit

    if commitment_root != historic_root {
        error!(
            "Historic root does not match commitment tree root. Historic root: {historic_root}, Commitment tree root: {commitment_root}"
        );
    } else {
        debug!("Historic root matches commitment tree root");
    }

    // see if we need to update the synchronisation status
    //This is a final safety check. Earlier we used event-level info to decide whether to sync. Now we consult the contract’s real-time state.

    let latest_current_block_number_in_contract =
        N::get_current_layer2_blocknumber().await.map_err(|_| {
            EventHandlerError::IOError("Could not retrieve current block number".to_string())
        })?;
    let delta = latest_current_block_number_in_contract - layer_2_block_number_in_event - I256::ONE;
    if delta != I256::ZERO {
        warn!("Synchronising - behind blockchain by {delta} layer 2 blocks ");
        sync_status.clear_synchronised();
    } else if replay_catch_up_pending {
        debug!(
            "Listener replay catch-up is still in progress after applying L2 block \
             {layer_2_block_number_in_event_u64}; deferring synchronised status until the \
             listener finishes draining historical events"
        );
        sync_status.clear_synchronised();
    } else {
        debug!("Synchronised with blockchain");
        sync_status.set_synchronised();
    }

    if let Some(pending_block) = pending_block {
        finalize_pending_block_after_applied_block::<P>(
            db,
            pending_block,
            our_address,
            &store_block_pending,
        )
        .await;
    }

    let reconciliation_block_number = current_block_number_in_contract
        .try_into()
        .unwrap_or(layer_2_block_number_in_event_u64);
    let became_synchronised = !was_synchronised && sync_status.is_synchronised();
    drop(sync_status);
    drop(expected_onchain_block_number);

    if became_synchronised {
        let _ = reconcile_active_client_transaction_lifecycle::<P>(db, reconciliation_block_number)
            .await;
    }

    match get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_block_number()
        .await
    {
        Ok(current_l1_block) => {
            maybe_schedule_snapshot_for_applied_block(db, current_l1_block).await;
        }
        Err(error) => {
            warn!(
                "Skipping automatic proposer snapshot scheduling after L2 block {layer_2_block_number_in_event_u64} because current L1 head could not be fetched: {error}"
            );
        }
    }

    Ok(())
}

async fn apply_proposer_block_transaction(
    db: Client,
    should_refresh_local_trees: bool,
    commitments: Vec<Fr254>,
    nullifiers: Vec<Fr254>,
    historic_root: Fr254,
    block_for_transaction: StoredBlock,
    sync_state_l1_ref: L1Ref,
) -> Result<Fr254, EventHandlerError> {
    let mut session = db
        .start_session()
        .await
        .map_err(|_| EventHandlerError::IOError("Could not start MongoDB session".to_string()))?;
    session
        .start_transaction()
        .and_run2(async move |session| {
            if should_refresh_local_trees {
                debug!(
                    "Adding {} commitments to commitment tree",
                    commitments.len()
                );
                <Client as CommitmentTree<Fr254>>::append_sub_trees_with_session(
                    &db,
                    &commitments,
                    true,
                    session,
                )
                .await
                .map_err(merkle_tree_error_to_mongo)?;
                debug!(
                    "Adding {} nullifiers to indexed Timber tree",
                    nullifiers.len()
                );
                <Client as NullifierTree<Fr254>>::insert_nullifiers_with_session(
                    &db,
                    &nullifiers,
                    session,
                )
                .await
                .map_err(merkle_tree_error_to_mongo)?;
            }

            db.append_historic_commitment_root_with_session(&historic_root, true, session)
                .await
                .map_err(merkle_tree_error_to_mongo)?;
            debug!(
                "Stored new commitments tree root in historic root timber tree: {historic_root}"
            );

            let commitment_root =
                <Client as CommitmentTree<Fr254>>::get_root_with_session(&db, session)
                    .await
                    .map_err(merkle_tree_error_to_mongo)?;

            db.store_block_with_session(&block_for_transaction, session)
                .await?;

            let sync_state = SyncState::new(
                block_for_transaction.layer2_block_number,
                block_for_transaction.hash().to_hex_string(),
                sync_state_l1_ref.clone(),
                mongodb::bson::DateTime::now(),
            );

            db.update_sync_state_with_session(&sync_state, session)
                .await?;
            maybe_fail_block_apply("after_sync_state_update")?;

            Ok::<Fr254, mongodb::error::Error>(commitment_root)
        })
        .await
        .map_err(|e| {
            EventHandlerError::IOError(format!("Could not apply proposer block transaction: {e}"))
        })
}

pub async fn process_deposit_escrowed_event<P, E>(
    transaction_hash: TxHash,
    filter: &Nightfall::DepositEscrowed,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    info!(
        "Proposer: Decoded DepositEscrowed event from transaction {}, Deposit Transaction with nf_slot_id {}, value {}, is now on-chain",
        transaction_hash, filter.nfSlotId, filter.value,
    );
    // get the transaction
    let tx = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction_by_hash(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?;

    // If there is one, decode it. If not, throw.
    if let Some(tx) = tx {
        let decoded = Nightfall::NightfallCalls::abi_decode(tx.input())
            .map_err(|_| EventHandlerError::InvalidCalldata)?;

        if let Nightfall::NightfallCalls::escrow_funds(decode) = decoded {
            // Get the information from the calldata
            let fee = Fr254::from(FrBn254::try_from(decode.fee).map_err(|_| {
                EventHandlerError::IOError("Could not convert to Fr254".to_string())
            })?);

            let erc_address = decode.ercAddress;
            let secret_hash = Fr254::from(FrBn254::try_from(decode.secretHash).map_err(|_| {
                EventHandlerError::IOError("Could not convert to Fr254".to_string())
            })?);

            let token_id = decode.tokenId;

            // Get the information from the event
            let nf_slot_id_from_event =
                Fr254::from(FrBn254::try_from(filter.nfSlotId).map_err(|_| {
                    EventHandlerError::IOError("Could not convert to Fr254".to_string())
                })?);
            // Note: value_from_calldata is the value that was escrowed for value escrow event.
            // But if it's a deposit escrow event, deposit_fee is new calculated value = msg.value - 2*fee, which is in filter.value.
            // So we use filter.value for both value escrow and fee escrow events instead of value_from_calldata.
            let value_from_event = Fr254::from(FrBn254::try_from(filter.value).map_err(|_| {
                EventHandlerError::IOError("Could not convert to Fr254".to_string())
            })?);

            // Get the fee token ID
            let fee_token_id = get_fee_token_id();

            let nf_token_id_tmp = to_nf_token_id_from_solidity(erc_address, token_id);

            // If this is a value escrow event, value_from_event gives us value
            // Then we should have DepositDatawithFee { fee, nf_token_id, nf_slot_id, value, secret_hash }
            // If this is a fee escrow event, value_from_event gives us deposit_fee
            // Then we should have DepositDatawithFee { fee, fee_token_id, fee_slot_id, deposit_fee, secret_hash }
            let (nf_slot_id, nf_token_id) = if nf_slot_id_from_event == fee_token_id {
                (fee_token_id, fee_token_id)
            } else {
                (nf_slot_id_from_event, nf_token_id_tmp)
            };

            let deposit_data = DepositData {
                nf_token_id,
                nf_slot_id,
                value: value_from_event,
                secret_hash,
            };
            let deposit_data = DepositDatawithFee {
                fee,
                deposit_data,
                reserved: false,
            };
            process_deposit_transaction::<P, E>(deposit_data)
                .await
                .map_err(|_| {
                    EventHandlerError::IOError("Could not process client transaction".to_string())
                })?;
        }
    } else {
        panic!("Transaction not found when looking up calldata");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entities::{Block, DepositDatawithFee, PendingBlock, PendingBlockState, SyncState},
        driven::db::{
            mongo_db::{ensure_deposit_indexes, StoredBlock, DB, DEPOSIT_COLLECTION},
            snapshot::create_proposer_snapshot,
        },
        ports::{
            db::{BlockStorageDB, PendingBlockDB, SyncStateDB, TransactionsDB},
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
    };
    use alloy::primitives::{Address, Bytes, TxHash};
    use ark_ff::Zero;
    use ark_serialize::SerializationError;
    use lib::{
        hex_conversion::HexConvertible,
        merkle_trees::trees::{MutableTree, TreeMetadata},
        nf_client_proof::Proof,
        shared_entities::{DepositData, OnChainTransaction},
        tests_utils::{get_db_connection as get_test_db_connection, get_mongo},
    };
    use serde::{Deserialize, Serialize};
    use tokio::sync::{Mutex, OnceCell};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct MockProof;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::new())
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self)
        }
    }

    async fn nightfall_event_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) })
            .await
            .lock()
            .await
    }

    async fn persist_sync_state(client: &mongodb::Client, sync_state: &SyncState) {
        let mut session = client.start_session().await.expect("start session");
        let client_for_write = client.clone();
        let sync_state_for_write = sync_state.clone();
        session
            .start_transaction()
            .and_run2(async move |session| {
                client_for_write
                    .update_sync_state_with_session(&sync_state_for_write, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write sync_state");

        let target_historic_root_sub_tree_count = sync_state
            .last_applied_l2_block
            .checked_add(2)
            .expect("historic root count should not overflow in event tests");
        let mut historic_root_sub_tree_count = client
            .database(DB)
            .collection::<TreeMetadata<Fr254>>(&format!(
                "{}_metadata",
                <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
            ))
            .find_one(mongodb::bson::doc! { "_id": 0 })
            .await
            .expect("read historic root metadata")
            .expect("historic root metadata should exist for event tests")
            .sub_tree_count;

        while historic_root_sub_tree_count < target_historic_root_sub_tree_count {
            let commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
                .await
                .expect("read commitment root");
            <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
                client,
                &commitment_root,
                true,
            )
            .await
            .expect("materialize historic root state for event test");
            historic_root_sub_tree_count += 1;
        }
    }

    async fn initialize_test_trees(client: &mongodb::Client) {
        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(client)
            .await
            .expect("create deposit indexes");
    }

    async fn materialize_tree_state_for_block(client: &mongodb::Client, leaf: Fr254) {
        <mongodb::Client as MutableTree<Fr254>>::insert_leaf(
            client,
            leaf,
            true,
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("append test commitment leaf");

        let commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
            .await
            .expect("read commitment root");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client,
            &commitment_root,
            true,
        )
        .await
        .expect("append historic root");
    }

    #[tokio::test]
    async fn snapshots_created_after_pending_block_cleanup_exclude_consumed_deposits() {
        let _lock = nightfall_event_test_lock().await;
        let container = get_mongo().await;
        let client = get_test_db_connection(&container).await;

        initialize_test_trees(&client).await;

        let selected_deposit = DepositDatawithFee {
            fee: Fr254::from(3_u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(11_u64),
                nf_slot_id: Fr254::from(12_u64),
                value: Fr254::from(13_u64),
                secret_hash: Fr254::from(14_u64),
            },
            reserved: true,
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![selected_deposit],
        )
        .await
        .expect("store selected deposit");

        let commitment = Fr254::from(21_u64);
        let on_chain_transaction = OnChainTransaction {
            commitments: [commitment, Fr254::zero(), Fr254::zero(), Fr254::zero()],
            ..Default::default()
        };
        let applied_block = StoredBlock {
            layer2_block_number: 7,
            commitments: on_chain_transaction
                .commitments
                .iter()
                .map(|commitment| commitment.to_hex_string())
                .collect(),
            proposer_address: Address::from([7_u8; 20]),
        };
        client
            .store_block(&applied_block)
            .await
            .expect("store applied block");
        materialize_tree_state_for_block(&client, commitment).await;

        persist_sync_state(
            &client,
            &SyncState::new(
                applied_block.layer2_block_number,
                applied_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 700,
                    tx_hash: TxHash::from([7_u8; 32]),
                    log_index: 7,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let pending_block = PendingBlock {
            layer2_block_number: applied_block.layer2_block_number,
            state: PendingBlockState::ReadyToPropose,
            broadcast_tx_hash: None,
            broadcast_receipt_checks: 0,
            block: Some(Block {
                transactions: vec![on_chain_transaction],
                ..Default::default()
            }),
            selected_deposits: vec![vec![selected_deposit]],
            selected_client_transaction_hashes: Vec::new(),
        };
        client
            .store_pending_block(&pending_block)
            .await
            .expect("store pending block");

        finalize_pending_block_after_applied_block::<MockProof>(
            &client,
            pending_block,
            applied_block.proposer_address,
            &applied_block,
        )
        .await;

        assert_eq!(
            client
                .get_pending_block(applied_block.layer2_block_number)
                .await,
            None
        );
        assert_eq!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_mempool_deposits(&client).await,
            None
        );

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-event-snapshot-ordering-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create proposer snapshot");
        let deposits_manifest = manifest
            .collections
            .iter()
            .find(|collection| collection.collection_name == DEPOSIT_COLLECTION)
            .expect("snapshot should include deposits collection");
        assert_eq!(
            deposits_manifest.document_count, 0,
            "snapshot should not retain deposits consumed by the applied block"
        );

        tokio::fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot root");
    }

    #[tokio::test]
    async fn failed_block_apply_does_not_advance_expected_layer2_blocknumber() {
        let _lock = nightfall_event_test_lock().await;
        let container = get_mongo().await;
        let client = get_test_db_connection(&container).await;

        initialize_test_trees(&client).await;
        *get_expected_layer2_blocknumber().await.write().await =
            I256::try_from(9_u64).expect("test L2 block fits into I256");

        let _failpoint = TestBlockApplyFailpointGuard::enable("after_sync_state_update");
        let error = apply_proposer_block_transaction(
            client.clone(),
            false,
            Vec::new(),
            Vec::new(),
            Fr254::zero(),
            StoredBlock {
                layer2_block_number: 9,
                commitments: Vec::new(),
                proposer_address: Address::from([9_u8; 20]),
            },
            L1Ref {
                block_number: 900,
                tx_hash: TxHash::from([9_u8; 32]),
                log_index: 9,
            },
        )
        .await
        .expect_err("block apply should fail at failpoint");

        let error_debug = format!("{error:?}");
        assert!(
            matches!(&error, EventHandlerError::IOError(message) if message.contains("Could not apply proposer block transaction")),
            "unexpected error from failed block apply: {error_debug}"
        );
        assert_eq!(
            *get_expected_layer2_blocknumber().await.read().await,
            I256::try_from(9_u64).expect("test L2 block fits into I256"),
            "failed block apply must not advance the in-memory expected L2 block number"
        );
        assert_eq!(client.get_sync_state().await, None);
        assert!(client.get_block_by_number(9).await.is_none());
    }

    #[tokio::test]
    async fn mismatch_cleanup_preserves_sync_state_and_tree_coherence() {
        let _lock = nightfall_event_test_lock().await;
        let container = get_mongo().await;
        let client = get_test_db_connection(&container).await;

        initialize_test_trees(&client).await;
        let applied_commitment = Fr254::from(71_u64);
        let applied_block = StoredBlock {
            layer2_block_number: 7,
            commitments: vec![applied_commitment.to_hex_string()],
            proposer_address: Address::from([7_u8; 20]),
        };
        client
            .store_block(&applied_block)
            .await
            .expect("store applied block");
        materialize_tree_state_for_block(&client, applied_commitment).await;

        let sync_state = SyncState::new(
            applied_block.layer2_block_number,
            applied_block.hash().to_hex_string(),
            L1Ref {
                block_number: 700,
                tx_hash: TxHash::from([7_u8; 32]),
                log_index: 7,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &sync_state).await;

        let commitment_root_before = <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
            .await
            .expect("read commitment root before mismatch cleanup");
        let historic_root_before = <mongodb::Client as MutableTree<Fr254>>::get_root(
            &client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        .expect("read historic root before mismatch cleanup");

        let speculative_block = StoredBlock {
            layer2_block_number: 8,
            commitments: vec!["0xspeculative-mismatch".to_string()],
            proposer_address: Address::from([8_u8; 20]),
        };
        client
            .store_block(&speculative_block)
            .await
            .expect("store speculative mismatched block");

        cleanup_mismatched_proposer_block_state(&client, 8)
            .await
            .expect("cleanup should remove only the speculative mismatched block");

        assert_eq!(client.get_sync_state().await, Some(sync_state));
        let retained_block = client
            .get_block_by_number(7)
            .await
            .expect("applied block should still exist after mismatch cleanup");
        assert_eq!(retained_block.hash(), applied_block.hash());
        assert!(client.get_block_by_number(8).await.is_none());
        assert_eq!(
            <mongodb::Client as CommitmentTree<Fr254>>::get_root(&client)
                .await
                .expect("read commitment root after mismatch cleanup"),
            commitment_root_before
        );
        assert_eq!(
            <mongodb::Client as MutableTree<Fr254>>::get_root(
                &client,
                <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
            )
            .await
            .expect("read historic root after mismatch cleanup"),
            historic_root_before
        );
    }
}
