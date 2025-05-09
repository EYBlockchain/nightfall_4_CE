use crate::{
    domain::entities::{Block as BlockEntity, DepositData, DepositDatawithFee, OnChainTransaction},
    driven::{
        db::mongo_db::StoredBlock,
        nightfall_client_transaction::{
            process_deposit_transaction, process_nightfall_client_transaction,
        },
    },
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        contracts::NightfallContract,
        db::BlockStorageDB,
        events::EventHandler,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::Fr as Fr254;
use ark_std::Zero;
use ethers::{
    core::abi::AbiDecode,
    providers::Middleware,
    types::{TxHash, H160, H256, I256},
};
use lib::{blockchain_client::BlockchainClientConnection, merkle_trees::trees::IndexedTree};
use log::{debug, error, info, warn};
use mongodb::Client;
use nightfall_bindings::nightfall::{
    DepositEscrowedFilter, NightfallCalls, NightfallEvents, ProposeBlockCall,
    SubmitClientTransactionCall,
};
use nightfall_client::{
    domain::{
        entities::{ClientTransaction, HexConvertible},
        error::EventHandlerError,
    },
    driven::contract_functions::contract_type_conversions::FrBn254,
    drivers::rest::utils::to_nf_token_id_from_solidity,
    get_fee_token_id,
    ports::proof::{Proof, ProvingEngine},
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use tokio::sync::{OnceCell, RwLock};
// Define a mutable lazy static to hold the layer 2 blocknumber. We need this to
// check if we're still in sync, but putting it in the context would mean passing it around too much
pub async fn get_expected_layer2_blocknumber() -> &'static RwLock<I256> {
    static LAYER2_BLOCKNUMBER: OnceCell<RwLock<I256>> = OnceCell::const_new();
    LAYER2_BLOCKNUMBER
        .get_or_init(|| async { RwLock::new(I256::zero()) })
        .await
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
impl<P, E, N> EventHandler<P, E, N> for NightfallEvents
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    async fn handle_event(&self, tx_hash: TxHash) -> Result<(), EventHandlerError> {
        // we'll split out individual events here in case that's useful later
        debug!("Handling event {:?} for transaction {:?}", self, tx_hash);
        match &self {
            NightfallEvents::BlockProposedFilter(filter) => {
                process_nightfall_calldata::<P, E, N>(
                    tx_hash,
                    Some(filter.proposer_address),
                    Some(filter.layer_2_block_number),
                )
                .await?
            }
            NightfallEvents::ClientTransactionSubmittedFilter(_f) => {
                info!("Received TransactionSubmitted event");
                process_nightfall_calldata::<P, E, N>(tx_hash, None, None).await?
            }
            NightfallEvents::DepositEscrowedFilter(filter) => {
                info!("Received DepositEscrowed event");
                process_deposit_escrowed_event::<P, E>(tx_hash, filter)
                    .await
                    .map_err(|e| {
                        debug!("{}", e);
                        EventHandlerError::InvalidCalldata
                    })?;
            }
        }
        Ok(())
        // all events, however, can be processed by the same function because you just need the tx hash to get the calldata
    }
}

pub async fn process_nightfall_calldata<P, E, N>(
    transaction_hash: H256,
    block_proposer: Option<H160>,
    block_number: Option<I256>, // process_nightfall_calldata is called for ClientTransactionSubmitted and BlockProposed events, the first one doesn't have block_number so we make it optional
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
        .get_transaction(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?;
    // if there is one, decode it. If not, throw.
    match tx {
        Some(tx) => {
            let decoded =
                NightfallCalls::decode(tx.input).map_err(|_| EventHandlerError::InvalidCalldata)?;
            match decoded {
                NightfallCalls::ProposeBlock(decode) => {
                    // OK to use unwrap because the smart contract has to provide a block number
                    process_propose_block_event::<N>(
                        decode,
                        transaction_hash,
                        block_proposer.unwrap(),
                        block_number.unwrap(),
                    )
                    .await?
                }

                NightfallCalls::SubmitClientTransaction(decode) => {
                    process_client_transaction_submitted_event::<P, E>(decode, transaction_hash)
                        .await?;
                }
                _ => (),
            }
        }
        None => panic!("Transaction not found when looking up calldata"),
    }
    Ok(())
}

async fn process_propose_block_event<N: NightfallContract>(
    decode: ProposeBlockCall,
    transaction_hash: H256,
    block_proposer: H160,
    layer_2_block_number_in_event: I256,
) -> Result<(), EventHandlerError> {
    ark_std::println!("Inside process_propose_block_event");
    let our_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address()
        .ok_or(EventHandlerError::IOError(
            "Could not retrieve our own address".to_string(),
        ))?;
    let sender_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?
        .ok_or(EventHandlerError::IOError(
            "Could not retrieve transaction".to_string(),
        ))?
        .from;
    ark_std::println!("Inside process_propose_block_event");
    ark_std::println!("our_address: {}", our_address);
    ark_std::println!("sender_address: {}", sender_address);
    ark_std::println!("sender_address from block event: {}", block_proposer);

    // get a lock on the db, we don't want anything else updating or reading the DB until
    // we're done here
    let db = &mut get_db_connection().await.write().await;
    info!(
        "Decoded Proposed block call from transaction {:?}",
        transaction_hash
    );
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
    ark_std::println!("store_block_pending: {:?}", store_block_pending);

    // check and update the sychronisation status
    let mut sync_status = get_synchronisation_status().await.write().await;
    // The first thing to do is to make sure that we've not missed any blocks.
    // If we have, then we'll need to resynchronise with the blockchain.
    let mut expected_onchain_block_number = get_expected_layer2_blocknumber().await.write().await;
    ark_std::println!(
        "expected_onchain_block_number: {}",
        *expected_onchain_block_number
    );
    ark_std::println!("layer_2_block_number: {}", layer_2_block_number_in_event);
    if *expected_onchain_block_number < layer_2_block_number_in_event {
        // we've missed at least one block
        ark_std::println!("Missed at least one block");
        warn!(
            "Out of sync with blockchain. Blocknumber of event was {}, expected {}",
            layer_2_block_number_in_event, expected_onchain_block_number
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
        //JJ: shouldn't we adjust the expected block number here?
        ark_std::println!("Already processed block");
        warn!(
            "Already processed layer 2 block {} - skipping",
            layer_2_block_number_in_event
        );
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
    ark_std::println!("expected_block_number_u64: {:?}", expected_block_number_u64);
    ark_std::println!("current_block_stored: {:?}", current_block_stored);

    // let current_block_stored_hash = current_block_stored.hash();

    // ark_std::println!("current_block_stored_hash: {:?}", current_block_stored_hash);
    // ark_std::println!("block_store_pending_hash: {:?}", block_store_pending_hash);

    //     if (expected_block_number_u64 == layer_2_block_number_in_event_u64) && (current_block_stored_hash != block_store_pending_hash) {
    //         warn!(
    //             "Block hash mismatch. Expected {}, got {} in layer 2 block {}",
    //             current_block_stored_hash, block_store_pending_hash, layer_2_block_number_in_event
    //         );
    //         // delete this block from the db, so later when restarting event listener, it will be reprocessed
    //         db.delete_block_by_number(expected_block_number_u64)
    //             .await;
    //         sync_status.clear_synchronised();
    //         return Err(EventHandlerError::BlockHashError(current_block_stored_hash, block_store_pending_hash));
    //     }

    match current_block_stored {
        Some(current_block) => {
            let current_block_stored_hash = current_block.hash();
            let block_store_pending_hash = store_block_pending.hash();
            ark_std::println!("current_block_stored_hash: {:?}", current_block_stored_hash);
            ark_std::println!("block_store_pending_hash: {:?}", block_store_pending_hash);

            if expected_block_number_u64 == layer_2_block_number_in_event_u64
                && current_block_stored_hash != block_store_pending_hash
            {
                warn!(
                    "Block hash mismatch. Expected {}, got {} in layer 2 block {}",
                    current_block_stored_hash,
                    block_store_pending_hash,
                    layer_2_block_number_in_event
                );

                // Delete the invalid block and clear sync status
                db.delete_block_by_number(expected_block_number_u64).await;
                sync_status.clear_synchronised();

                return Err(EventHandlerError::BlockHashError(
                    current_block_stored_hash,
                    block_store_pending_hash,
                ));
            }
        }

        None => {
            ark_std::println!(
                "No block found in DB at expected height {}. Assuming fresh state or first sync.",
                expected_block_number_u64
            );
            // You might choose to return here, or continue if syncing should proceed regardless.
        }
    }

    *expected_onchain_block_number += 1; // move on to the next block

    // warn that we're not synced with the blockchain if we're behind
    // before we used the event filter layer 2 block number
    // now we get the current_block_number from the blockchain
    // what's the difference?
    let current_block_number_in_contract =
        N::get_current_layer2_blocknumber().await.map_err(|_| {
            EventHandlerError::IOError("Could not retrieve current block number".to_string())
        })?;
    ark_std::println!(
        "process_propose_block_event: expected_onchain_block_number in proposer: {}",
        expected_onchain_block_number
    );

    ark_std::println!(
        "process_propose_block_event: Current L2 block number in proposer: {}",
        current_block_number_in_contract
    );

    // if the current block number is exactly one, then we're automatically synchronised because we've seen one
    // blockproposed event (or we wouldn't be here) and that must also be the only one
    if current_block_number_in_contract == I256::one() {
        debug!("Synchronised with blockchain");
        sync_status.set_synchronised();
    }

    ark_std::println!(
        "sync_status.is_synchronised(): {}",
        sync_status.is_synchronised()
    );

    // next, we'll unpack the commitments and add them to the proposer's commitment tree
    // normally, we don't update the trees if we're the proposer, because we'll have done it when we proposed the block
    // but if we're not in sync then we need to get this information from the blockchain.
    // There's one more case, where this is the first block, so we must be synchronised in the sense that our block count is the
    // same as the blockchain's block count, but we've lost the commitment data. In this case, we need to update the trees too.
    // If we don't have the data from the first block, out commitment root will be zero.
    let commitment_root = <Client as CommitmentTree<Fr254>>::get_root(db)
        .await
        .map_err(|_| {
            EventHandlerError::IOError("Could not retrieve commitment root".to_string())
        })?;
    ark_std::println!("commitment_root: {:?}", commitment_root);
    if our_address != sender_address || !sync_status.is_synchronised() || commitment_root.is_zero()
    {
        let commitments = &blk
            .transactions
            .iter()
            .flat_map(|transaction| &transaction.commitments)
            .map(|u| FrBn254::try_from(*u).map(|f| f.into()))
            .collect::<Result<Vec<Fr254>, _>>()
            .expect("Could not convert commitments to U256");
        debug!(
            "Adding {} commitments to commitment tree",
            commitments.len()
        );
        ark_std::println!("commitments: {:?}", commitments);
        <Client as CommitmentTree<Fr254>>::append_sub_trees(db, commitments, true)
            .await
            .map_err(|_| EventHandlerError::IOError("Could not store commitments".to_string()))?;
        // and do the same with the nullifier tree
        let nullifiers = blk
            .transactions
            .iter()
            .flat_map(|transaction| &transaction.nullifiers)
            .map(|u| FrBn254::try_from(*u).map(|f| f.into()))
            .collect::<Result<Vec<Fr254>, _>>()
            .expect("Could not convert nullifiers to U256");
        debug!(
            "Adding {} nullifiers to indexed Timber tree",
            nullifiers.len()
        );
        <Client as IndexedTree<Fr254>>::insert_leaves(
            db,
            &nullifiers,
            <Client as NullifierTree<Fr254>>::TREE_NAME,
        )
        .await
        .map_err(|_| EventHandlerError::IOError("Could not store nullifiers".to_string()))?;
    }
    // and next,the commitments root (historic_root) is stored in the historic root tree
    let historic_root: Fr254 = FrBn254::try_from(blk.commitments_root)
        .map_err(|_| EventHandlerError::IOError("Could not convert to Fr254".to_string()))?
        .into();
    ark_std::println!("historic_root: {:?}", historic_root);

    db.append_historic_commitment_root(&historic_root, true)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not store historic root".to_string()))?;
    debug!(
        "Stored new commitments tree root in historic root timber tree: {}",
        historic_root
    );

    // it's worth checking that the historic root agrees with what's in the commitment tree
    let commitment_root = <Client as CommitmentTree<Fr254>>::get_root(db)
        .await
        .map_err(|_| {
            EventHandlerError::IOError("Could not retrieve commitment root".to_string())
        })?;
    if commitment_root != historic_root {
        error!(
            "Historic root does not match commitment tree root. Historic root: {}, Commitment tree root: {}",
            historic_root, commitment_root
        );
    } else {
        debug!("Historic root matches commitment tree root");
    }

    // see if we need to update the synchronisation status
    //This is a final safety check. Earlier we used event-level info to decide whether to sync. Now we consult the contract’s real-time state.

    let delta = current_block_number_in_contract - layer_2_block_number_in_event - I256::one();
    if delta != I256::zero() {
        warn!(
            "Synchronising - behind blockchain by {} layer 2 blocks ",
            delta
        );
        sync_status.clear_synchronised();
    } else {
        debug!("Synchronised with blockchain");
        sync_status.set_synchronised();
    }

    // store the block in the db
    // if db doesn't have the block, it will be stored
    db.store_block(&store_block_pending).await;

    Ok(())
}

async fn process_client_transaction_submitted_event<P, E>(
    decode: SubmitClientTransactionCall,
    transaction_hash: H256,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    info!(
        "Decoded SubmitL2Transaction call from transaction {:?}",
        transaction_hash
    );
    // now we do some type conversion Transaction -> Transaction<P> -> ClientTransactionWithMetaData<P>
    let tx = decode.txn;
    let client_transaction: ClientTransaction<P> = tx
        .try_into()
        .map_err(|_| EventHandlerError::InvalidCalldata)?;
    process_nightfall_client_transaction::<P, E>(client_transaction)
        .await
        .map_err(|_| {
            EventHandlerError::IOError("Could not process client transaction".to_string())
        })?;
    Ok(())
}
async fn process_deposit_escrowed_event<P, E>(
    transaction_hash: H256,
    filter: &DepositEscrowedFilter,
) -> Result<(), EventHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    info!(
        "Proposer: Decoded DepositEscrowed event from transaction {}, Deposit Transaction with nf_slot_id {}, value {}, is now on-chain",
        transaction_hash, filter.nf_slot_id, filter.value,
    );
    // get the transaction
    let tx = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client()
        .get_transaction(transaction_hash)
        .await
        .map_err(|_| EventHandlerError::IOError("Could not retrieve transaction".to_string()))?;

    // If there is one, decode it. If not, throw.
    if let Some(tx) = tx {
        let decoded =
            NightfallCalls::decode(tx.input).map_err(|_| EventHandlerError::InvalidCalldata)?;

        if let NightfallCalls::EscrowFunds(decode) = decoded {
            // Get the information from the calldata
            let fee = Fr254::from(FrBn254::try_from(decode.fee).map_err(|_| {
                EventHandlerError::IOError("Could not convert to Fr254".to_string())
            })?);

            let erc_address = decode.erc_address;
            let secret_hash = Fr254::from(FrBn254::try_from(decode.secret_hash).map_err(|_| {
                EventHandlerError::IOError("Could not convert to Fr254".to_string())
            })?);

            let token_id = decode.token_id;

            // Get the information from the event
            let nf_slot_id_from_event =
                Fr254::from(FrBn254::try_from(filter.nf_slot_id).map_err(|_| {
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
            let deposit_data = DepositDatawithFee { fee, deposit_data };

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
