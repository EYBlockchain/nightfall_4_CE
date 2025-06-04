use crate::{
    domain::entities::{ClientTransactionWithMetaData, DepositDatawithFee, OnChainTransaction},
    initialisation::get_db_connection,
    ports::db::TransactionsDB,
};

use crate::ports::trees::HistoricRootTree;
use crate::ports::trees::NullifierTree;
use ark_std::Zero;
use log::{error, info};
use nightfall_client::{
    domain::entities::ClientTransaction,
    ports::proof::{Proof, ProvingEngine, PublicInputs},
};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use warp::reject;

// The event processor (located in the Repository) and the REST API both need to call the following function. Thus, it can't really be located in services,
//otherwise we'd be doing a reentrant call from repository to services, which is a bit of an odd pattern.

/// If we receive a client transaction, either via a blockchain event, because it was
/// posted to the blockchain directly, or because it was submitted via the REST API,
/// we need to process it. This means:
/// 1. Checking that the transaction is valid
/// 2. Storing the transaction in the database
/// 3. Triggering block assembly if necessary
/// 4. Returning the transaction to the client
///

#[derive(Debug)]
pub enum ClientTransactionError<E: ProvingEngine<P>, P: Proof> {
    InvalidTransaction,
    TransactionAlreadyExists,
    TransactionNotOnChain,
    ProofDidNotVerify(E::Error),
    HashGenerationFailed,
    CouldNotStoreTransaction,
    CouldNotGetNullifierTree,
    DuplicateNullifier,
    CommitmentRootUnknown,
    NullifiersAllZero,
}
impl<E: ProvingEngine<P>, P: Proof> Error for ClientTransactionError<E, P> {}
impl<E: ProvingEngine<P>, P: Proof> Display for ClientTransactionError<E, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientTransactionError::InvalidTransaction => write!(f, "Invalid transaction"),
            ClientTransactionError::TransactionAlreadyExists => {
                write!(f, "Transaction already exists")
            }
            ClientTransactionError::TransactionNotOnChain => write!(f, "Transaction not on chain"),
            ClientTransactionError::ProofDidNotVerify(e) => write!(f, "Proof did not verify: {e}"),
            ClientTransactionError::HashGenerationFailed => write!(f, "Hash generation failed"),
            ClientTransactionError::CouldNotStoreTransaction => {
                write!(f, "Could not store transaction")
            }
            ClientTransactionError::CouldNotGetNullifierTree => {
                write!(f, "Could not get nullifier tree")
            }
            ClientTransactionError::DuplicateNullifier => {
                write!(f, "Duplicate nullifier")
            }
            ClientTransactionError::CommitmentRootUnknown => {
                write!(f, "Commitment root is not known")
            }
            ClientTransactionError::NullifiersAllZero => {
                write!(f, "All nullifiers are zero")
            }
        }
    }
}
impl<E: ProvingEngine<P>, P: Proof> reject::Reject for ClientTransactionError<E, P> {}

/// This function checks a client transaction that has been received from a client, either directly or via a blockchain event.
pub async fn process_nightfall_client_transaction<P, E>(
    client_transaction: ClientTransaction<P>,
) -> Result<(), ClientTransactionError<E, P>>
where
    E: ProvingEngine<P>,
    P: Proof,
{
    //
    // Do a series of checks that make sure this Transaction<P> is valid, and won't cause the rollup to fail
    //
    let db = get_db_connection().await; // `db` is now &'static mongodb::Client
                                        // `db` is directly usable for all database operations, including writes.
    let public_inputs = PublicInputs::from(&client_transaction);
    if let Err(error) = E::verify(&client_transaction.proof, &public_inputs) {
        return Err(ClientTransactionError::ProofDidNotVerify(error));
    }
    // 2) then we should check that the transaction is not already in the database i.e. this isn't a replay
    let hash = &client_transaction
        .hash()
        .map_err(|_| ClientTransactionError::HashGenerationFailed)?;
    let ctx = db.find_transaction(&client_transaction).await;
    // if it is, we should return an error
    if ctx.is_some() {
        return Err(ClientTransactionError::TransactionAlreadyExists);
    }

    // 3) check that the historic commitment roots used for the transaction are actually valid historic roots.
    // If they are, they should be in our database of historic roots because that gets populated from
    // blockproposed events, which are proven correct.
    // We won't do this if we aren't nullifying anything, i.e. it's a deposit transaction
    for (i, historic_commitment_root) in client_transaction
        .historic_commitment_roots
        .iter()
        .enumerate()
    {
        if !client_transaction.nullifiers[i].is_zero()
            && !db
                .is_historic_root(historic_commitment_root)
                .await
                .expect("Database error looking up historic root")
        {
            error!(
                "Historic commitment root not found in database: {}",
                historic_commitment_root
            );
            return Err(ClientTransactionError::CommitmentRootUnknown);
        }
    }

    // 4) check that the nullifiers are not used. Zero nulifiers are ignored.
    for nullifier in client_transaction.nullifiers.iter() {
        if !nullifier.is_zero() && db.get_non_membership_proof(nullifier).await.is_err() {
            return Err(ClientTransactionError::DuplicateNullifier);
        }
    }

    // 5) Validate that we can convert the transaction into a form suitable for the nightfall contract bindings
    let _: OnChainTransaction = (&client_transaction).into();
    let client_transaction_with_metadata = ClientTransactionWithMetaData::<P> {
        client_transaction: client_transaction.clone(),
        block_l2: None,
        in_mempool: true,
        hash: hash.to_vec(),
        historic_roots: client_transaction.historic_commitment_roots.to_vec(),
    };

    // 6) Validate that the first nullifier is not zero (we must nullify the first spent commitment)
    if client_transaction.nullifiers[0].is_zero() {
        return Err(ClientTransactionError::NullifiersAllZero);
    }

    // if all checks pass, we should store it in the TransactionsDB
    info!("Client Transaction is valid, storing in database");
    let key = db.store_transaction(client_transaction_with_metadata).await;
    match key {
        Some(_key) => Ok(()),
        None => Err(ClientTransactionError::CouldNotStoreTransaction),
    }
}
/// This function checks a client transaction that has been received from a client, either directly or via a blockchain event.
pub async fn process_deposit_transaction<P, E>(
    deposit_data: DepositDatawithFee,
) -> Result<(), ClientTransactionError<E, P>>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    let db = get_db_connection().await; // `db` is now &'static mongodb::Client

    // 1) then we should check that the transaction is not already in the database i.e. this isn't a replay

    let ctx = <mongodb::Client as TransactionsDB<P>>::find_deposit(db, &deposit_data).await;
    // if it is, we should return an error
    if ctx.is_some() {
        return Err(ClientTransactionError::TransactionAlreadyExists);
    }
    info!("Deposit Transaction is valid, storing in database");
    let key =
        <mongodb::Client as TransactionsDB<P>>::set_mempool_deposits(db, vec![deposit_data]).await;
    match key {
        Some(_key) => Ok(()),
        None => Err(ClientTransactionError::CouldNotStoreTransaction),
    }
}
