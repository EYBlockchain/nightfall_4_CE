use crate::{
    domain::entities::{DepositData, DepositDatawithFee},
    driven::nightfall_client_transaction::{
        process_deposit_transaction, process_nightfall_client_transaction,
    },
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        contracts::NightfallContract,
        events::EventHandler,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::Fr as Fr254;
use ark_std::Zero;
use ethers::{
    core::abi::AbiDecode,
    providers::Middleware,
    types::{TxHash, H256, I256},
};
use lib::{blockchain_client::BlockchainClientConnection, merkle_trees::trees::IndexedTree};
use log::{debug, error, info, warn};
use mongodb::Client;
use nightfall_bindings::nightfall::{
    DepositEscrowedFilter, NightfallCalls, NightfallEvents, ProposeBlockCall,
    SubmitClientTransactionCall,
};
use nightfall_client::{
    domain::{entities::ClientTransaction, error::EventHandlerError},
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
                process_nightfall_calldata::<P, E, N>(tx_hash, Some(filter.layer_2_block_number))
                    .await?
            }
            NightfallEvents::ClientTransactionSubmittedFilter(_f) => {
                info!("Received TransactionSubmitted event");
                process_nightfall_calldata::<P, E, N>(tx_hash, None).await?
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
    block_number: Option<I256>,
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
    layer_2_block_number: I256,
) -> Result<(), EventHandlerError> {
    // get a lock on the db, we don't want anything else updating or reading the DB until
    // we're done here
    let db = &mut get_db_connection().await.write().await;
    info!(
        "Decoded Proposed block call from transaction {:?}",
        transaction_hash
    );
    let blk = decode.blk;

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

    // check and update the sychronisation status
    let mut sync_status = get_synchronisation_status().await.write().await;
    // The first thing to do is to make sure that we've not missed any blocks.
    // If we have, then we'll need to resynchronise with the blockchain.
    let mut expected_onchain_block_number = get_expected_layer2_blocknumber().await.write().await;
    if *expected_onchain_block_number < layer_2_block_number {
        // we've missed at least one block
        warn!(
            "Out of sync with blockchain. Blocknumber of event was {}, expected {}",
            layer_2_block_number, expected_onchain_block_number
        );
        sync_status.clear_synchronised();
        return Err(EventHandlerError::MissingBlocks(
            expected_onchain_block_number.as_usize(),
        ));
    }

    // check if we're ahead of the event, this means we've already seen it and we shouldn't process it again
    // This could happen if we've missed some blocks and we're re-synchronising
    if *expected_onchain_block_number > layer_2_block_number {
        warn!(
            "Already processed layer 2 block {} - skipping",
            layer_2_block_number
        );
        return Ok(());
    }

    *expected_onchain_block_number += 1; // move on to the next block

    // warn that we're not synced with the blockchain if we're behind
    let current_block_number = N::get_current_layer2_blocknumber().await.map_err(|_| {
        EventHandlerError::IOError("Could not retrieve current block number".to_string())
    })?;
    ark_std::println!("process_propose_block_event: expected_onchain_block_number in proposer: {}", current_block_number);

    ark_std::println!("process_propose_block_event: Current L2 block number in proposer: {}", current_block_number);

    // if the current block number is exactly one, then we're automatically synchronised because we've seen one
    // blockproposed event (or we wouldn't be here) and that must also be the only one
    if current_block_number == I256::one() {
        debug!("Synchronised with blockchain");
        sync_status.set_synchronised();
    }

    // next, we'll unpack the commitments and add them to the proposer's commitment tree
    let our_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address()
        .ok_or(EventHandlerError::IOError(
            "Could not retrieve our own address".to_string(),
        ))?;

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
    let delta = current_block_number - layer_2_block_number - I256::one();
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
