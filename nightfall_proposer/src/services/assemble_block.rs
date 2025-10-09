use crate::{
    domain::entities::{Block, ClientTransactionWithMetaData, DepositData, DepositDatawithFee},
    driven::db::mongo_db::{StoredBlock, PROPOSED_BLOCKS_COLLECTION},
    drivers::blockchain::block_assembly::BlockAssemblyError,
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        db::{BlockStorageDB, TransactionsDB},
        proving::RecursiveProvingEngine,
    },
};
use ark_bn254::Fr as Fr254;
use ark_std::{collections::HashSet, Zero};
use bson::doc;
use configuration::settings::get_settings;
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::{blockchain_client::BlockchainClientConnection, hex_conversion::HexConvertible,
           nf_client_proof::{Proof, PublicInputs}
};
use log::{info, warn};
use nightfall_client::{
    driven::db::mongo::DB,
};
use std::cmp::Reverse;
use tokio::time::Instant;

// Define a type alias for better readability
type ALLTransactionData<P> = (
    Option<Vec<DepositDatawithFee>>,
    Option<ClientTransactionWithMetaData<P>>,
    Fr254,
);

pub(crate) fn transactions_to_include_in_block<K, V>(
    mempool_transactions: Option<Vec<(K, V)>>,
) -> Vec<(K, V)> {
    // stuff happens here to decide which transactions to include in the block
    // NB: make sure the transaction's input commitments are still unspent: it's possible that they could have been spent since the Transaction<P> was created
    // In a block, we will have at most block_size transactions, this includes at most 32 DepositDatas + client transactions
    // If we have more than block_size transactions, we'll only include the block_size - DepositDatas most valuable transactions
    mempool_transactions.unwrap_or_default()
}

/// Fetch the block size from the nightfall toml and ensure it's an allowed number
pub fn get_block_size() -> Result<usize, BlockAssemblyError> {
    let settings = get_settings();
    // get the block size from the environment, if it's not set, default to 64
    let block_size = settings.nightfall_proposer.block_size;
    // Allowed block sizes: 64, 256, 1024
    match block_size {
        // safe to unwrap as we know it's a usize
        64 | 256 | 1024 => Ok(block_size.try_into().unwrap()),
        _ => Err(BlockAssemblyError::ProvingError(
            "Block size must be one of 64, 256, or 1024".to_string(),
        )),
    }
}
/// assemble_block is the main function that is called by the proposer to create a new block,
/// it fetches the necessary data from the database and the contract, then assembles the block
pub(crate) async fn assemble_block<P, R>() -> Result<Block, BlockAssemblyError>
where
    P: Proof,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
{
    info!("Starting block assembly process");
    // initialise included_depositinfos_group, selected_client_transactions
    let included_depositinfos_group;
    let selected_client_transactions;
    {
        info!("Getting DB connection");
        let db = get_db_connection().await;
        info!("Preparing block data");
        let result = prepare_block_data::<P>(db).await;
        match &result {
            Ok(_) => info!("Block data prepared successfully"),
            Err(e) => warn!("Failed to prepare block data: {e:?}"),
        }
        (included_depositinfos_group, selected_client_transactions) = result?;
    }

    // Convert DepositInfo into DepositData while maintaining nested structure
    // included_depositinfos_group has extra fee than DepositData, so we need to remove the fee
    let included_deposits: Vec<Vec<DepositData>> = included_depositinfos_group
        .iter()
        .map(|group| group.iter().map(|deposit| deposit.deposit_data).collect())
        .collect();
    let real_deposit_number = included_deposits
        .iter()
        .flat_map(|group| group.iter())
        .filter(|deposit| **deposit != DepositData::default())
        .count();
    let (withdraw_count, transfer_count) =
        selected_client_transactions
            .iter()
            .fold((0, 0), |(withdraws, transfers), tx| {
                let commitments_0_is_zero = tx.client_transaction.commitments[0].is_zero();
                let nullifiers_0_is_nonzero = !tx.client_transaction.nullifiers[0].is_zero();

                if commitments_0_is_zero && nullifiers_0_is_nonzero {
                    (withdraws + 1, transfers)
                } else {
                    (withdraws, transfers + 1)
                }
            });

    info!(
        "This block has {real_deposit_number} deposit(s), {transfer_count} transfer(s), and {withdraw_count} withdrawal(s)"
    );

    let block = make_block::<P, R>(included_deposits, selected_client_transactions).await?;
    // save this block to Store block db
    let db = get_db_connection().await;
    let current_block_number = db
        .database(DB)
        .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
        .count_documents(doc! {})
        .await
        .expect("Failed to count documents");
    let our_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address();

    let store_block = StoredBlock {
        layer2_block_number: current_block_number,
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
        proposer_address: our_address,
    };
    db.database(DB)
        .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
        .insert_one(store_block.clone())
        .await
        .expect("Failed to insert block into database");
    Ok(block)
}

// this is where we compute the on chain block it's called by make_block
// which spawns it out as a separate thread
#[allow(dead_code)]
pub(crate) async fn compute_block<P, R>(
    deposit_transactions: Vec<(P, PublicInputs)>,
    client_transactions: Vec<ClientTransactionWithMetaData<P>>,
) -> Result<Block, BlockAssemblyError>
where
    P: Proof,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
{
    info!("Computing block");
    // ****************************************
    // lots of hard maths to go in here
    // ****************************************
    // we'll get a rough idea of how long this takes
    let now = Instant::now();
    let block = R::prove_block(&deposit_transactions, &client_transactions)
        .await
        .map_err(|e| e.into());
    info!("Block computation took: {} s", now.elapsed().as_secs());
    block
}
/// This function is used to make a block from the deposit and client transactions
/// mainly generate deposit proofs and then call compute_block to generate block proof
pub(crate) async fn make_block<P, R>(
    deposit_transactions: Vec<Vec<DepositData>>,
    client_transactions: Vec<ClientTransactionWithMetaData<P>>,
) -> Result<Block, BlockAssemblyError>
where
    P: Proof + Send + Sync + 'static,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
{
    // Generate Proofs for deposit transaction
    let deposit_proofs_result = deposit_transactions
        .into_iter()
        .map(|chunk| {
            let mut public_inputs = PublicInputs::new();

            // Convert Vec<DepositData> (which is guaranteed to be size 4) into [DepositData; 4]
            let deposit_array: [DepositData; 4] = chunk.try_into().map_err(|_| {
                BlockAssemblyError::ProvingError(
                    "Could not convert deposit data chunk to fixed-length array".to_string(),
                )
            })?;

            info!("Creating deposit proof for a group of 4 deposits");
            // Generate proof for this group of 4 deposits
            let proof = R::create_deposit_proof(&deposit_array, &mut public_inputs)
                .map_err(|e| BlockAssemblyError::ProvingError(format!("Proving Error: {e}")))?;

            Result::<(P, PublicInputs), BlockAssemblyError>::Ok((proof, public_inputs))
        })
        .collect::<Result<Vec<(P, PublicInputs)>, BlockAssemblyError>>();

    match &deposit_proofs_result {
        Ok(proofs) => info!("Generated {} deposit proofs", proofs.len()),
        Err(e) => warn!("Failed to generate deposit proofs: {e:?}"),
    }

    let mut deposit_proofs = deposit_proofs_result?;

    let block_size = get_block_size()?;
    let transaction_count = deposit_proofs.len() + client_transactions.len();
    info!("Current transaction count: {transaction_count}, block size: {block_size}");
    // append default deposit proof if the transaction count is less than block size
    if transaction_count < block_size {
        let default_deposits_count = block_size - transaction_count;
        info!(
            "Adding {} default deposit proofs to fill block",
            &default_deposits_count
        );
        let mut public_inputs = PublicInputs::new();
        let deposit_array: [DepositData; 4] = [DepositData::default(); 4];
        let proof = R::create_deposit_proof(&deposit_array, &mut public_inputs)
            .map_err(|e| BlockAssemblyError::ProvingError(format!("Proving Error: {e}")))?;
        (0..default_deposits_count).for_each(|_| {
            deposit_proofs.push((proof.clone(), public_inputs));
        });
    }
    compute_block::<P, R>(deposit_proofs, client_transactions).await
}

pub(crate) async fn prepare_block_data<P>(
    db: &mongodb::Client,
) -> Result<
    (
        Vec<Vec<DepositDatawithFee>>,
        Vec<ClientTransactionWithMetaData<P>>,
    ),
    BlockAssemblyError,
>
where
    P: Proof,
{
    let block_size = get_block_size()?;

    // 1. Fetch unused deposits from mempool
    let stored_deposits_in_mempool: Option<Vec<DepositDatawithFee>> =
        <mongodb::Client as TransactionsDB<P>>::get_mempool_deposits(db).await;
    // if there are no deposits in mempool, the all_deposits will be empty, otherwise will be the deposits in mempool
    let all_deposits = stored_deposits_in_mempool.unwrap_or_default();

    info!("Found {} deposits in mempool", all_deposits.len());

    // 4. Get client transactions from mempool
    let current_client_transaction_meta_in_mempool = {
        let mempool_client_transactions: Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> =
            db.get_all_mempool_client_transactions().await;
        let transactions = transactions_to_include_in_block(mempool_client_transactions);
        info!(
            "Found {} client transactions in mempool",
            transactions.len()
        );
        transactions
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<ClientTransactionWithMetaData<P>>>()
    };

    // 3. Get the block stored in the database during processing propose_block
    let stored_blocks = db.get_all_blocks().await.unwrap_or_default();
    // check if commitments in current_client_transaction_meta_in_mempool and all_deposits are in the stored_block's commitments
    // if they are, remove the related transactions from the mempool
    let all_commitments_onchain: HashSet<String> = stored_blocks
        .iter()
        .flat_map(|block| block.commitments.iter().cloned())
        .collect();

    // 4. Partition deposits into pending and stale
    let (mut pending_deposits, stale_deposits): (Vec<_>, Vec<_>) =
        all_deposits.into_iter().partition(|d| {
            let inputs = [
                d.deposit_data.nf_token_id,
                d.deposit_data.nf_slot_id,
                d.deposit_data.value,
                Fr254::from(0u64),
                Fr254::from(1u64),
                d.deposit_data.secret_hash,
            ];
            let poseidon = Poseidon::<Fr254>::new();
            let commitment_hex = poseidon.hash(&inputs).unwrap().to_hex_string();
            !all_commitments_onchain.contains(&commitment_hex)
        });
    // Partition client transactions into pending and stale
    let (pending_client_transactions, stale_client_transactions): (Vec<_>, Vec<_>) =
        current_client_transaction_meta_in_mempool
            .into_iter()
            .partition(|tx| {
                tx.client_transaction
                    .commitments
                    .iter()
                    .filter(|c| c.to_hex_string() != Fr254::zero().to_hex_string())
                    .all(|c| !all_commitments_onchain.contains(&c.to_hex_string()))
            });

    // Clean stale items from mempool
    if !stale_deposits.is_empty() {
        let _ = <mongodb::Client as TransactionsDB<P>>::remove_mempool_deposits(
            db,
            vec![stale_deposits.clone()],
        )
        .await;
    }

    if !stale_client_transactions.is_empty() {
        let _ = db.set_in_mempool(&stale_client_transactions, false).await;
    }

    // 4. Check if there are any pending deposits or client transactions
    if pending_deposits.is_empty() && pending_client_transactions.is_empty() {
        warn!("No transactions pending");
        return Err(BlockAssemblyError::InsufficientTransactions);
    }

    // Step 5. Sort and prioritize transactions
    // 1 client transaction = 1 transaction, 4 DepositInfo = 1 transaction
    // we should group and rank Depositinfos into sets of 4, padding with default deposits if necessary
    // then we rank client transactions and groups of 4 depositinfo based on the fee and give the selected transactions
    let mut all_transactions: Vec<ALLTransactionData<P>> = Vec::new();

    let mut deposit_groups: Vec<Vec<DepositDatawithFee>> = vec![];
    let mut current_group: Vec<DepositDatawithFee> = vec![];

    // 5.1. Group deposits into sets of 4, if there are less than 4, pad with default deposits
    // sort the deposits by fee in descending order
    pending_deposits.sort_by_key(|d| Reverse(d.fee));
    for deposit in pending_deposits.clone().iter() {
        current_group.push(*deposit);
        if current_group.len() == 4 {
            deposit_groups.push(current_group.clone());
            current_group.clear();
        }
    }
    // pad default deposits to group with less than 4 deposits
    if !current_group.is_empty() {
        while current_group.len() < 4 {
            current_group.push(DepositDatawithFee::default());
        }
        deposit_groups.push(current_group.clone());
    }

    // 5.2. Push grouped deposits as full transactions
    for deposit_group in deposit_groups.iter() {
        let total_fee = deposit_group.iter().map(|d| d.fee).sum(); // Sum fees of 4 deposits
        all_transactions.push((Some(deposit_group.clone()), None, total_fee));
    }

    // 5.3. Push client transactions (1:1 mapping)
    for client_tx in pending_client_transactions.iter() {
        all_transactions.push((
            None,
            Some(client_tx.clone()),
            client_tx.client_transaction.fee,
        ));
    }
    // 5.4. Sort transactions by total fee (descending)
    all_transactions.sort_by_key(|(_, _, fee)| Reverse(*fee));

    // 6. Select top block_size transactions
    let selected_transactions: Vec<ALLTransactionData<P>> =
        all_transactions.into_iter().take(block_size).collect();

    // 7. Separate used deposits and client transactions
    let used_deposits_info: Vec<Vec<DepositDatawithFee>> = selected_transactions
        .iter()
        .filter_map(|(deposit, _, _)| deposit.clone()) // Ensure cloning deposit groups properly
        .collect();

    let selected_client_transactions: Vec<ClientTransactionWithMetaData<P>> = selected_transactions
        .iter()
        .filter_map(|(_, client_tx, _)| client_tx.clone()) // Extract client transactions
        .collect();

    // 9. Delete used deposits in mempool
    <mongodb::Client as TransactionsDB<P>>::remove_mempool_deposits(db, used_deposits_info.clone())
        .await;

    // 10. Clear selected client transactions from mempool
    db.set_in_mempool(&selected_client_transactions, false)
        .await;
    Ok((used_deposits_info, selected_client_transactions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::tests_utils::{get_db_connection, get_mongo};
    use nightfall_client::driven::plonk_prover::plonk_proof::PlonkProof;

    #[tokio::test]
    async fn test_prepare_block_data_simple_case() {
        // Prepare data: 44 deposit data in mempool, fee (1...240), 4 tx data, fee (241...244)
        // block_size = 64, 4 client transactions, 240 deposit data  = 64 transactions
        // Used deposit (1...240), used client: (241...244)
        // left deposit = None
        // left client transactions: None
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // **1. Insert 240 deposits into mempool**
        {
            let deposits: Vec<DepositDatawithFee> = (1..=240)
                .map(|i| DepositDatawithFee {
                    fee: Fr254::from(i),
                    deposit_data: DepositData {
                        nf_token_id: Fr254::from(i),
                        nf_slot_id: Fr254::from(i),
                        value: Fr254::from(100u64),
                        secret_hash: Fr254::from(i),
                    },
                })
                .collect();

            <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(&db, deposits)
                .await;
        }
        // **2. Insert 32 client transactions into mempool**
        {
            let transactions: Vec<ClientTransactionWithMetaData<PlonkProof>> = (241..=244)
                .map(|i| ClientTransactionWithMetaData {
                    client_transaction: nightfall_client::domain::entities::ClientTransaction {
                        fee: Fr254::from(i),
                        proof: PlonkProof::default(),
                        ..Default::default()
                    },
                    block_l2: None,
                    in_mempool: true,
                    hash: vec![i as u32],
                    historic_roots: vec![Fr254::from(123)],
                })
                .collect();

            for tx in transactions {
                db.store_transaction(tx).await.unwrap();
            }
        }

        let result = { prepare_block_data::<PlonkProof>(&db).await };

        assert!(result.is_ok(), "prepare_block_data failed");
        let (included_deposits, selected_client_transactions) = result.unwrap();

        let mut actual_fees_deposit: Vec<Fr254> = included_deposits
            .iter()
            .flat_map(|group| group.iter().map(|d| d.fee))
            .collect();
        actual_fees_deposit.sort_by_key(|&fee| Reverse(fee));
        let expected_fees_deposit: Vec<Fr254> = (1..=240).rev().map(Fr254::from).collect();

        assert_eq!(
            expected_fees_deposit, actual_fees_deposit,
            "Deposit fees do not match expected values"
        );
        let mut actual_fees_client: Vec<Fr254> = selected_client_transactions
            .iter()
            .map(|d| d.client_transaction.fee)
            .collect();
        actual_fees_client.sort_by_key(|&fee| Reverse(fee));

        let expected_fees_client: Vec<Fr254> = (241..=244).rev().map(Fr254::from).collect();

        assert_eq!(
            expected_fees_client, actual_fees_client,
            "Client fees do not match expected values"
        );
        assert_eq!(
            included_deposits.len(),
            60,
            "Incorrect number of deposits included"
        );
        assert_eq!(
            selected_client_transactions.len(),
            4,
            "Incorrect number of client transactions included"
        );

        // **3. Check that the remaining 2 deposits are stored back in the mempool**
        let remaining_deposits =
            { <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db).await };
        assert!(
            remaining_deposits
                .as_ref()
                .is_none_or(|deposits| deposits.is_empty()),
            "Remaining deposits are not empty"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_only_mempool_deposits() {
        // Prepare data: 247 deposit data in mempool: fee (1...=257), 0 client tx data,
        // Used deposit  (2...=257)
        // left deposit = (1)
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert 257 deposit transactions into mempool**
        {
            let deposits: Vec<DepositDatawithFee> = (1..=257)
                .map(|i| DepositDatawithFee {
                    fee: Fr254::from(i),
                    deposit_data: DepositData {
                        nf_token_id: Fr254::from(i),
                        nf_slot_id: Fr254::from(i),
                        value: Fr254::from(i),
                        secret_hash: Fr254::from(i),
                    },
                })
                .collect();

            <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(&db, deposits)
                .await;
        }

        let result = { prepare_block_data::<PlonkProof>(&db).await };

        assert!(result.is_ok(), "Should succeed with only on-chain deposits");
        let (included_deposits, selected_client_transactions) = result.unwrap();
        assert!(
            !included_deposits.is_empty(),
            "On-chain deposits should be included"
        );
        assert!(
            selected_client_transactions.is_empty(),
            "No client transactions should be included"
        );
        let mut actual_used_deposit_fees: Vec<Fr254> = included_deposits
            .iter()
            .flat_map(|group| group.iter())
            .filter(|d| !d.fee.is_zero())
            .map(|d| d.fee)
            .collect();
        actual_used_deposit_fees.sort_by_key(|&fee| Reverse(fee));
        let mut expected_fees_deposit: Vec<Fr254> = (2..=257).rev().map(Fr254::from).collect();
        expected_fees_deposit.sort_by_key(|&fee| Reverse(fee));
        assert_eq!(
            expected_fees_deposit, actual_used_deposit_fees,
            "Deposit fees do not match expected values"
        );

        let remaining_deposits =
            { <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db).await };
        // fee in the remaining deposit should be 1
        let remain_deposits_fee: Vec<Fr254> =
            remaining_deposits.unwrap().iter().map(|d| d.fee).collect();
        assert_eq!(
            remain_deposits_fee,
            vec![Fr254::from(1)],
            "Remaining deposit fees do not match expected values"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_only_client_transactions() {
        // prepare data: 0 deposit data in mempool, 74 client tx data, fee (1...=74),
        // Used deposit 0, Tx data:  (11...=74)
        // Left client transactions: 10 transactions (fees 1...10)
        // Left deposits: 0
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert 74 deposit transactions into mempool**
        {
            let transactions: Vec<ClientTransactionWithMetaData<PlonkProof>> = (1..=74)
                .map(|i| ClientTransactionWithMetaData {
                    client_transaction: nightfall_client::domain::entities::ClientTransaction {
                        fee: Fr254::from(i),
                        proof: PlonkProof::default(),
                        ..Default::default()
                    },
                    block_l2: None,
                    in_mempool: true,
                    hash: vec![i as u32],
                    historic_roots: vec![Fr254::from(123)],
                })
                .collect();

            for tx in transactions {
                db.store_transaction(tx).await.unwrap();
            }
        }

        let result = { prepare_block_data::<PlonkProof>(&db).await };

        assert!(result.is_ok(), "Should succeed with only on-chain deposits");
        let (included_deposits, selected_client_transactions) = result.unwrap();
        assert!(
            included_deposits.is_empty(),
            "No deposits should be included"
        );
        let mut actual_fees_client: Vec<Fr254> = selected_client_transactions
            .iter()
            .map(|d| d.client_transaction.fee)
            .collect();
        actual_fees_client.sort_by_key(|&fee| Reverse(fee));

        let expected_fees_client: Vec<Fr254> = (11..=74).rev().map(Fr254::from).collect();
        assert_eq!(
            expected_fees_client, actual_fees_client,
            "Deposit fees do not match expected values"
        );

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let mut actual_remaining_client_fees: Vec<Fr254> = remaining_client
            .iter()
            .map(|d| d.client_transaction.fee)
            .collect();
        actual_remaining_client_fees.sort_by_key(|&fee| Reverse(fee));

        let expected_remaining_client_fees: Vec<Fr254> = (1..=10).rev().map(Fr254::from).collect();
        assert_eq!(
            actual_remaining_client_fees, expected_remaining_client_fees,
            "Remaining client transaction fees do not match expected values"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_1_deposit() {
        // prepare data: 3 deposit data in mempool, fee (200..=203), 64 client tx data, fee (1..=64),
        // Used deposit (200..=203) , Tx data:  (2..=64)
        // Left client transactions: 1
        // Left deposits: none

        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // **1. Insert 3 deposits into mempool**
        {
            let deposits: Vec<DepositDatawithFee> = (200..=203)
                .map(|i| DepositDatawithFee {
                    fee: Fr254::from(i),
                    deposit_data: DepositData {
                        nf_token_id: Fr254::from(i),
                        nf_slot_id: Fr254::from(i),
                        value: Fr254::from(100u64),
                        secret_hash: Fr254::from(i),
                    },
                })
                .collect();

            <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(&db, deposits)
                .await;
        }

        // Insert 64 client transactions into mempool**
        {
            let transactions: Vec<ClientTransactionWithMetaData<PlonkProof>> = (1..=64)
                .map(|i| ClientTransactionWithMetaData {
                    client_transaction: nightfall_client::domain::entities::ClientTransaction {
                        fee: Fr254::from(i),
                        proof: PlonkProof::default(),
                        ..Default::default()
                    },
                    block_l2: None,
                    in_mempool: true,
                    hash: vec![i as u32],
                    historic_roots: vec![Fr254::from(123)],
                })
                .collect();

            for tx in transactions {
                db.store_transaction(tx).await.unwrap();
            }
        }

        let result = { prepare_block_data::<PlonkProof>(&db).await };

        assert!(
            result.is_ok(),
            "Should succeed with 10 deposits + 53 client transactions"
        );
        let (included_deposits, selected_client_transactions) = result.unwrap();

        let expected_used_deposit_fees: Vec<Fr254> = (200..=203).map(Fr254::from).rev().collect();

        let mut actual_fees_deposit: Vec<Fr254> = included_deposits
            .iter()
            .flat_map(|group| group.iter())
            .filter(|d| !d.fee.is_zero())
            .map(|d| d.fee)
            .collect();

        actual_fees_deposit.sort_by_key(|&fee| Reverse(fee));
        assert_eq!(
            expected_used_deposit_fees, actual_fees_deposit,
            "Used deposit fees do not match expected values"
        );

        let expected_used_client_fees: Vec<Fr254> = (2..=64).rev().map(Fr254::from).collect();

        let actual_used_client_fees: Vec<Fr254> = selected_client_transactions
            .iter()
            .map(|d| d.client_transaction.fee)
            .collect();

        assert_eq!(
            expected_used_client_fees, actual_used_client_fees,
            "Used client transaction fees do not match expected values"
        );

        let actual_fees_deposit_remainning: Vec<Fr254> = {
            <mongodb::Client as TransactionsDB<PlonkProof>>::get_mempool_deposits(&db)
                .await
                .unwrap_or_else(Vec::new) // Ensuring it's never None
                .into_iter()
                .map(|deposit| deposit.fee) // Extracting only the fees
                .collect()
        };
        assert!(
            actual_fees_deposit_remainning.is_empty(),
            "Remaining deposit fees should be empty"
        );

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_client_fees: Vec<Fr254> = remaining_client
            .iter()
            .map(|d| d.client_transaction.fee)
            .collect();
        assert_eq!(
            remaining_client_fees,
            vec![Fr254::from(1)],
            "Remaining client transaction fees do not match expected values"
        );
    }
}
