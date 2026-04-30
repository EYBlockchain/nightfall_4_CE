use crate::{
    domain::entities::{Block, ClientTransactionWithMetaData, DepositDatawithFee},
    driven::db::mongo_db::{StoredBlock, DB, PROPOSED_BLOCKS_COLLECTION},
    drivers::blockchain::block_assembly::BlockAssemblyError,
    initialisation::{get_blockchain_client_connection, get_db_connection},
    ports::{
        db::{BlockStorageDB, TransactionsDB},
        proving::RecursiveProvingEngine,
    },
};
use ark_bn254::Fr as Fr254;
use ark_std::{collections::HashSet, Zero};
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::{
    blockchain_client::BlockchainClientConnection,
    hex_conversion::HexConvertible,
    nf_client_proof::{Proof, PublicInputs},
    shared_entities::DepositData,
    utils::get_block_size,
};
use log::{info, warn};
use std::cmp::Reverse;
use tokio::time::Instant;

// Define a type alias for better readability
type ALLTransactionData<P> = (
    Option<Vec<DepositDatawithFee>>,
    Option<Vec<ClientTransactionWithMetaData<P>>>,
    Fr254,
    usize,
);

type SwapPair<P> = (
    ClientTransactionWithMetaData<P>,
    ClientTransactionWithMetaData<P>,
);

struct GroupedSwapTransactions<P> {
    normal_transactions: Vec<ClientTransactionWithMetaData<P>>,
    matched_swap_pairs: Vec<SwapPair<P>>,
    expired_swaps: Vec<ClientTransactionWithMetaData<P>>,
    unmatched_swap_count: usize,
}

fn group_valid_swap_pairs<P>(
    pending_client_transactions: Vec<ClientTransactionWithMetaData<P>>,
    current_block_number: u64,
) -> GroupedSwapTransactions<P>
where
    P: Proof,
{
    let (swap_transactions, normal_transactions): (Vec<_>, Vec<_>) = pending_client_transactions
        .into_iter()
        .partition(|tx| !tx.client_transaction.swap_link.is_zero());

    let mut swap_groups: std::collections::HashMap<String, Vec<ClientTransactionWithMetaData<P>>> =
        std::collections::HashMap::new();

    for tx in swap_transactions {
        let swap_link_hex = tx.client_transaction.swap_link.to_hex_string();
        swap_groups.entry(swap_link_hex).or_default().push(tx);
    }

    let mut matched_swap_pairs: Vec<SwapPair<P>> = Vec::new();
    let mut expired_swaps: Vec<ClientTransactionWithMetaData<P>> = Vec::new();
    let mut unmatched_swap_count = 0;

    for (_swap_link, txs) in swap_groups {
        let mut by_deadline: std::collections::HashMap<
            String,
            Vec<ClientTransactionWithMetaData<P>>,
        > = std::collections::HashMap::new();
        for tx in txs {
            let deadline_hex = tx.client_transaction.deadline.to_hex_string();
            by_deadline.entry(deadline_hex).or_default().push(tx);
        }

        for (_deadline_key, mut same_deadline_txs) in by_deadline {
            let deadline = same_deadline_txs
                .first()
                .map(|tx| tx.client_transaction.deadline)
                .unwrap_or_else(Fr254::zero);

            if deadline.is_zero() || deadline < Fr254::from(current_block_number) {
                warn!("Swap deadline expired or invalid, skipping group");
                expired_swaps.append(&mut same_deadline_txs);
                continue;
            }

            let mut party_a_legs: Vec<ClientTransactionWithMetaData<P>> = Vec::new();
            let mut party_b_legs: Vec<ClientTransactionWithMetaData<P>> = Vec::new();
            for tx in same_deadline_txs.drain(..) {
                if tx.client_transaction.swap_side == Fr254::from(1u64) {
                    party_a_legs.push(tx);
                } else if tx.client_transaction.swap_side.is_zero() {
                    party_b_legs.push(tx);
                } else {
                    warn!("Invalid swap_side (expected 0 or 1), keeping transaction unmatched");
                    unmatched_swap_count += 1;
                }
            }

            while !party_a_legs.is_empty() && !party_b_legs.is_empty() {
                let tx_a = party_a_legs.pop().expect("len checked");
                let tx_b = party_b_legs.pop().expect("len checked");
                matched_swap_pairs.push((tx_a, tx_b));
            }

            unmatched_swap_count += party_a_legs.len() + party_b_legs.len();
        }
    }

    GroupedSwapTransactions {
        normal_transactions,
        matched_swap_pairs,
        expired_swaps,
        unmatched_swap_count,
    }
}

fn place_swap_pairs_on_siblings<P>(
    deposit_count: usize,
    selected_transactions: &[ALLTransactionData<P>],
) -> Vec<ClientTransactionWithMetaData<P>>
where
    P: Proof,
{
    let mut swap_pairs: Vec<SwapPair<P>> = Vec::new();
    let mut normal_txs: Vec<ClientTransactionWithMetaData<P>> = Vec::new();

    for (_, client_txs, _, slots) in selected_transactions {
        if let Some(txs) = client_txs {
            if *slots == 2 && txs.len() == 2 {
                swap_pairs.push((txs[0].clone(), txs[1].clone()));
            } else {
                normal_txs.extend(txs.clone());
            }
        }
    }

    let mut reordered: Vec<ClientTransactionWithMetaData<P>> = Vec::new();
    let mut normal_iter = normal_txs.into_iter();
    let mut global_idx = deposit_count;

    for pair in swap_pairs {
        while global_idx % 2 != 0 {
            if let Some(normal) = normal_iter.next() {
                reordered.push(normal);
                global_idx += 1;
            } else {
                info!(
                    "Skipping remaining swap pairs because global index {global_idx} is odd and no normal transactions are available for alignment"
                );
                break;
            }
        }

        if global_idx % 2 != 0 {
            break;
        }

        reordered.push(pair.0);
        reordered.push(pair.1);
        global_idx += 2;
    }

    reordered.extend(normal_iter);
    reordered
}

pub(crate) fn transactions_to_include_in_block<K, V>(
    mempool_transactions: Option<Vec<(K, V)>>,
) -> Vec<(K, V)> {
    // stuff happens here to decide which transactions to include in the block
    // NB: make sure the transaction's input commitments are still unspent: it's possible that they could have been spent since the Transaction<P> was created
    // In a block, we will have at most block_size transactions, this includes at most 32 DepositDatas + client transactions
    // If we have more than block_size transactions, we'll only include the block_size - DepositDatas most valuable transactions
    mempool_transactions.unwrap_or_default()
}
/// assemble_block is the main function that is called by the proposer to create a new block,
/// it fetches the necessary data from the database and the contract, then assembles the block
pub(crate) async fn assemble_block<P, R>(
    current_l2_block_number: u64,
) -> Result<Block, BlockAssemblyError>
where
    P: Proof,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
{
    info!("Starting block assembly process");
    // initialise included_depositinfos_group, selected_client_transactions
    let included_depositinfos_group;
    let selected_client_transactions: Vec<ClientTransactionWithMetaData<P>>;
    {
        info!("Getting DB connection");
        let db = get_db_connection().await;
        info!("Preparing block data");
        let block_size = get_block_size()?;
        let result = prepare_block_data::<P>(db, block_size, current_l2_block_number).await;
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
    let (withdraw_count, transfer_count, swap_count) =
        selected_client_transactions
            .iter()
            .fold((0, 0, 0), |(withdraws, transfers, swaps), tx| {
                let commitments_0_is_zero = tx.client_transaction.commitments[0].is_zero();
                let nullifiers_0_is_nonzero = !tx.client_transaction.nullifiers[0].is_zero();
                let is_swap = !tx.client_transaction.swap_link.is_zero();

                if is_swap {
                    (withdraws, transfers, swaps + 1)
                } else if commitments_0_is_zero && nullifiers_0_is_nonzero {
                    (withdraws + 1, transfers, swaps)
                } else {
                    (withdraws, transfers + 1, swaps)
                }
            });

    info!(
        "This block has {real_deposit_number} deposit(s), {transfer_count} transfer(s), \
        {withdraw_count} withdrawal(s), and {swap_count} swap transaction(s) ({} pair(s))",
        swap_count / 2
    );
    let block = make_block::<P, R>(included_deposits, selected_client_transactions).await?;
    // save this block to Store block db
    let db = get_db_connection().await;
    let our_address = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_address();

    let store_block = StoredBlock {
        layer2_block_number: current_l2_block_number,
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
    block_size: usize,
    current_block_number: u64,
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
    // 1. Fetch unused deposits from mempool
    let stored_deposits_in_mempool: Option<Vec<DepositDatawithFee>> =
        <mongodb::Client as TransactionsDB<P>>::get_mempool_deposits(db).await;
    // if there are no deposits in mempool, the all_deposits will be empty, otherwise will be the deposits in mempool
    let all_deposits = stored_deposits_in_mempool.unwrap_or_default();

    info!("Found {} deposits in mempool", all_deposits.len());

    // 2. Get client transactions from mempool
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

    let GroupedSwapTransactions {
        normal_transactions,
        matched_swap_pairs,
        expired_swaps,
        unmatched_swap_count,
    } = group_valid_swap_pairs(pending_client_transactions, current_block_number);

    if unmatched_swap_count != 0 {
        info!("Keeping {unmatched_swap_count} unmatched non-expired swap leg(s) in mempool");
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
        all_transactions.push((Some(deposit_group.clone()), None, total_fee, 1));
    }

    // 5.3. Push normal client transactions (1 slot each)
    for client_tx in normal_transactions.iter() {
        all_transactions.push((
            None,
            Some(vec![client_tx.clone()]),
            client_tx.client_transaction.fee,
            1,
        ));
    }
    // 5.3b. Push matched swap pairs (2 slots each, combined fee)
    for (tx_a, tx_b) in matched_swap_pairs.iter() {
        let combined_fee = tx_a.client_transaction.fee + tx_b.client_transaction.fee;
        all_transactions.push((
            None,
            Some(vec![tx_a.clone(), tx_b.clone()]),
            combined_fee,
            2,
        ));
    }
    // 5.4. Sort transactions by total fee (descending)
    // 5.4. Sort transactions by fee-per-slot (descending)
    // Compare fee_a/slots_a vs fee_b/slots_b using cross-multiplication
    // to avoid division: fee_a * slots_b > fee_b * slots_a
    all_transactions.sort_by(|a, b| {
        let fee_a = a.2;
        let slots_a = Fr254::from(a.3 as u64);
        let fee_b = b.2;
        let slots_b = Fr254::from(b.3 as u64);

        // fee_b * slots_a vs fee_a * slots_b (reversed for descending)
        let lhs = fee_b * slots_a;
        let rhs = fee_a * slots_b;
        lhs.cmp(&rhs)
    });

    // 6. Select top block_size transactions
    let mut selected_transactions: Vec<ALLTransactionData<P>> = Vec::new();
    let mut slots_used = 0;
    for tx in all_transactions {
        let slots = tx.3;
        if slots_used + slots <= block_size {
            slots_used += slots;
            selected_transactions.push(tx);
        }
    }

    // 7. Separate used deposits and client transactions
    let used_deposits_info: Vec<Vec<DepositDatawithFee>> = selected_transactions
        .iter()
        .filter_map(|(deposit, _, _, _)| deposit.clone())
        .collect();

    let deposit_count = used_deposits_info.len();
    let reordered = place_swap_pairs_on_siblings(deposit_count, &selected_transactions);

    // 9. Delete used deposits in mempool
    <mongodb::Client as TransactionsDB<P>>::remove_mempool_deposits(db, used_deposits_info.clone())
        .await;

    // 10. Clear selected client transactions from mempool
    db.set_in_mempool(&reordered, false).await;

    // 10b. Clear expired swaps from mempool
    if !expired_swaps.is_empty() {
        db.set_in_mempool(&expired_swaps, false).await;
    }

    if used_deposits_info.is_empty() && reordered.is_empty() {
        warn!("No selectable transactions remain after swap filtering");
        return Err(BlockAssemblyError::InsufficientTransactions);
    }

    Ok((used_deposits_info, reordered))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{
        plonk_prover::plonk_proof::PlonkProof,
        tests_utils::{get_db_connection, get_mongo},
    };
    #[tokio::test]
    async fn test_prepare_block_data_simple_case() {
        // Prepare data: 44 deposit data in mempool, fee (1...240), 4 tx data, fee (241...244)
        // block_size = 64, 4 client transactions, 240 deposit data  = 64 transactions
        // Used deposit (1...240), used client: (241...244)
        // left deposit = None
        // left client transactions: None
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 64;

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
                    client_transaction: lib::shared_entities::ClientTransaction {
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

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;

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
        let block_size = 64;

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

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;

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
        let block_size = 64;

        // Insert 74 deposit transactions into mempool**
        {
            let transactions: Vec<ClientTransactionWithMetaData<PlonkProof>> = (1..=74)
                .map(|i| ClientTransactionWithMetaData {
                    client_transaction: lib::shared_entities::ClientTransaction {
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

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;

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
        let block_size = 64;

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
                    client_transaction: lib::shared_entities::ClientTransaction {
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

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;

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

    #[tokio::test]
    async fn test_prepare_block_data_with_swaps() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        let normal_tx = ClientTransactionWithMetaData {
            client_transaction: lib::shared_entities::ClientTransaction {
                fee: Fr254::from(10u64),
                proof: PlonkProof::default(),
                ..Default::default()
            },
            block_l2: None,
            in_mempool: true,
            hash: vec![1],
            historic_roots: vec![],
        };

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = vec![
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(20u64),
                    swap_link: Fr254::from(1u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from(1u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![2],
                historic_roots: vec![],
            },
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(20u64),
                    swap_link: Fr254::from(1u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::zero(),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![3],
                historic_roots: vec![],
            },
        ];

        for tx in std::iter::once(normal_tx).chain(swap_txs.into_iter()) {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(result.is_ok());
        let (_, selected) = result.unwrap();

        assert_eq!(selected.len(), 3); // 2 swap + 1 normal

        // Swap pair should be consecutive
        let swap_idx = selected
            .iter()
            .position(|tx| !tx.client_transaction.swap_link.is_zero())
            .unwrap();
        assert_eq!(
            selected[swap_idx].client_transaction.swap_link,
            selected[swap_idx + 1].client_transaction.swap_link,
        );
        // Swap pair (fee/slot=25) ranked above normal (fee/slot=10)
        assert!(swap_idx < selected.len() - 1);
    }

    #[tokio::test]
    async fn test_prepare_block_data_swap_excluded_when_block_full() {
        // block_size = 2, swap pair needs 2 slots
        // 3 normal txs: fee = 50, 40, 30
        // 1 swap pair: fee = 10 + 10 = 20, fee/slot = 10
        // Ranking: normal(50), normal(40), normal(30), swap(10/slot)
        // Block takes normal(50) + normal(40) = 2 slots, swap stays in mempool
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 2;

        let normal_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = [50u64, 40, 30]
            .iter()
            .enumerate()
            .map(|(i, &fee)| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(fee),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![i as u32 + 1],
                historic_roots: vec![],
            })
            .collect();

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(10u64),
                    swap_link: Fr254::from(42u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![10 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in normal_txs.into_iter().chain(swap_txs.into_iter()) {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(result.is_ok());
        let (_, selected) = result.unwrap();

        assert_eq!(selected.len(), 2);
        let swap_count = selected
            .iter()
            .filter(|tx| !tx.client_transaction.swap_link.is_zero())
            .count();
        assert_eq!(
            swap_count, 0,
            "Swap pair should be excluded when block full"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_swap_expired_deadline() {
        // Swap pair with deadline = 0 (expired), should be ignored
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        // Insert a block so current_block_number = 1
        db.database(DB)
            .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
            .insert_one(StoredBlock {
                layer2_block_number: 0,
                commitments: vec![],
                proposer_address: alloy::primitives::Address::ZERO,
            })
            .await
            .unwrap();

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(100u64),
                    swap_link: Fr254::from(55u64),
                    deadline: Fr254::from(0u64), // expired
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![i as u32 + 1],
                historic_roots: vec![],
            })
            .collect();

        for tx in swap_txs {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 1).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        // Expired swap legs must be removed from mempool.
        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_expired_swaps = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(55u64))
            .count();
        assert_eq!(
            remaining_expired_swaps, 0,
            "Expired swap legs should be removed from mempool"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_swap_zero_deadline_rejected_at_block_zero() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(100u64),
                    swap_link: Fr254::from(77u64),
                    deadline: Fr254::zero(),
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![20 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in swap_txs {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_zero_deadline_swaps = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(77u64))
            .count();
        assert_eq!(
            remaining_zero_deadline_swaps, 0,
            "Zero-deadline swap legs should be removed from mempool at block 0"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_deposits_swaps_normals_mixed() {
        // block_size = 8
        // 8 deposits: fee 100 each → 2 groups of 4 = 2 slots, total_fee/slot = 400
        // 1 swap pair: fee 50 + 50 = 100, fee/slot = 50
        // 4 normal txs: fee = 60, 45, 30, 10
        //
        // Ranking: deposit_group(400), deposit_group(400), normal(60), swap(50/slot), normal(45), normal(30), normal(10)
        // Fill 8 slots: 2 deposits + normal(60) + swap_pair(2) + normal(45) + normal(30) = 8 slots
        // Leftover: normal(10)
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 8;

        // 8 deposits
        let deposits: Vec<DepositDatawithFee> = (1..=8)
            .map(|i| DepositDatawithFee {
                fee: Fr254::from(100u64),
                deposit_data: DepositData {
                    nf_token_id: Fr254::from(i as u64),
                    nf_slot_id: Fr254::from(i as u64),
                    value: Fr254::from(100u64),
                    secret_hash: Fr254::from(i as u64),
                },
            })
            .collect();
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(&db, deposits).await;

        // Swap pair
        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(50u64),
                    swap_link: Fr254::from(99u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![50 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        // Normal txs
        let normal_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = [60u64, 45, 30, 10]
            .iter()
            .enumerate()
            .map(|(i, &fee)| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(fee),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![70 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in swap_txs.into_iter().chain(normal_txs.into_iter()) {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(result.is_ok());
        let (included_deposits, selected) = result.unwrap();

        // 2 deposit groups
        assert_eq!(included_deposits.len(), 2);

        // Swap pair included
        let swap_count = selected
            .iter()
            .filter(|tx| !tx.client_transaction.swap_link.is_zero())
            .count();
        assert_eq!(swap_count, 2, "Swap pair should be included");

        // Swap pair should be consecutive and at sibling positions
        let swap_idx = selected
            .iter()
            .position(|tx| !tx.client_transaction.swap_link.is_zero())
            .unwrap();
        assert_eq!(
            selected[swap_idx].client_transaction.swap_link,
            selected[swap_idx + 1].client_transaction.swap_link,
        );
        // Global index = deposit_count + swap_idx should be even
        let global_idx = included_deposits.len() + swap_idx;
        assert_eq!(
            global_idx % 2,
            0,
            "Swap pair should start at even global index"
        );

        // Total: 2 deposit slots + 6 client slots = 8
        assert_eq!(included_deposits.len() + selected.len(), block_size);
    }

    #[tokio::test]
    async fn test_prepare_block_data_swap_pair_skipped_when_no_alignment_tx_available() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 3;

        let deposits: Vec<DepositDatawithFee> = (1..=4)
            .map(|i| DepositDatawithFee {
                fee: Fr254::from(100u64),
                deposit_data: DepositData {
                    nf_token_id: Fr254::from(i as u64),
                    nf_slot_id: Fr254::from(i as u64),
                    value: Fr254::from(100u64),
                    secret_hash: Fr254::from(i as u64),
                },
            })
            .collect();
        <mongodb::Client as TransactionsDB<PlonkProof>>::set_mempool_deposits(&db, deposits).await;

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(50u64),
                    swap_link: Fr254::from(199u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![600 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in swap_txs {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(result.is_ok());
        let (included_deposits, selected) = result.unwrap();

        assert_eq!(
            included_deposits.len(),
            1,
            "Deposit group should still be selected"
        );
        assert!(
            selected.is_empty(),
            "Swap pair should be skipped when no alignment transaction is available"
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
        let remaining_swap_legs = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(199u64))
            .count();
        assert_eq!(
            remaining_swap_legs, 2,
            "Misaligned swap pair should remain in mempool for a future block"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_same_side_swaps_not_paired() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        // Two A-legs with same swap_link/deadline must stay unmatched.
        let same_side_swaps: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(20u64),
                    swap_link: Fr254::from(777u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from(1u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![200 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in same_side_swaps {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        // Same-side swap legs should remain pending in mempool for future pairing.
        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_same_side = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(777u64))
            .count();
        assert_eq!(
            remaining_same_side, 2,
            "Same-side swap legs should remain pending in mempool"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_single_orphan_swap_leg_stays_in_mempool() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        db.store_transaction(ClientTransactionWithMetaData {
            client_transaction: lib::shared_entities::ClientTransaction {
                fee: Fr254::from(20u64),
                swap_link: Fr254::from(778u64),
                deadline: Fr254::from(1000u64),
                swap_side: Fr254::from(1u64),
                proof: PlonkProof::default(),
                ..Default::default()
            },
            block_l2: None,
            in_mempool: true,
            hash: vec![210],
            historic_roots: vec![],
        })
        .await
        .unwrap();

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_orphan_swap_legs = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(778u64))
            .count();
        assert_eq!(
            remaining_orphan_swap_legs, 1,
            "A single orphan swap leg should remain pending in mempool"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_three_legs_one_pair_one_leftover() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        // A, A, B with same swap_link/deadline => one pair selected, one left in mempool.
        let three_legs: Vec<ClientTransactionWithMetaData<PlonkProof>> = vec![
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(20u64),
                    swap_link: Fr254::from(888u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from(1u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![300],
                historic_roots: vec![],
            },
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(21u64),
                    swap_link: Fr254::from(888u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from(1u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![301],
                historic_roots: vec![],
            },
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(22u64),
                    swap_link: Fr254::from(888u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::zero(),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![302],
                historic_roots: vec![],
            },
        ];

        for tx in three_legs {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(result.is_ok());
        let (_, selected) = result.unwrap();

        let swap_count = selected
            .iter()
            .filter(|tx| !tx.client_transaction.swap_link.is_zero())
            .count();
        assert_eq!(swap_count, 2, "Exactly one swap pair should be selected");

        // One leg must remain in mempool.
        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_same_swap = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(888u64))
            .count();
        assert_eq!(
            remaining_same_swap, 1,
            "One unmatched swap leg should remain in mempool"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_invalid_swap_side_not_paired() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 4;

        // Invalid side (2) must never be paired.
        let invalid_side_swaps: Vec<ClientTransactionWithMetaData<PlonkProof>> = vec![
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(20u64),
                    swap_link: Fr254::from(999u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from(2u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![400],
                historic_roots: vec![],
            },
            ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(21u64),
                    swap_link: Fr254::from(999u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::zero(),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![401],
                historic_roots: vec![],
            },
        ];

        for tx in invalid_side_swaps {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_invalid_side = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(999u64))
            .count();
        assert_eq!(
            remaining_invalid_side, 2,
            "Invalid-side swap legs should remain pending in mempool"
        );
    }

    #[tokio::test]
    async fn test_prepare_block_data_swap_pair_not_selected_when_block_size_is_one() {
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let block_size = 1;

        let swap_txs: Vec<ClientTransactionWithMetaData<PlonkProof>> = (0..2)
            .map(|i| ClientTransactionWithMetaData {
                client_transaction: lib::shared_entities::ClientTransaction {
                    fee: Fr254::from(100u64),
                    swap_link: Fr254::from(123u64),
                    deadline: Fr254::from(1000u64),
                    swap_side: Fr254::from((i % 2) as u64),
                    proof: PlonkProof::default(),
                    ..Default::default()
                },
                block_l2: None,
                in_mempool: true,
                hash: vec![500 + i as u32],
                historic_roots: vec![],
            })
            .collect();

        for tx in swap_txs {
            db.store_transaction(tx).await.unwrap();
        }

        let result = prepare_block_data::<PlonkProof>(&db, block_size, 0).await;
        assert!(matches!(
            result,
            Err(BlockAssemblyError::InsufficientTransactions)
        ));

        let remaining_client = {
            let mempool_client_transactions: Option<
                Vec<(Vec<u32>, ClientTransactionWithMetaData<PlonkProof>)>,
            > = db.get_all_mempool_client_transactions().await;

            transactions_to_include_in_block(mempool_client_transactions)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<ClientTransactionWithMetaData<PlonkProof>>>()
        };
        let remaining_swap_legs = remaining_client
            .iter()
            .filter(|tx| tx.client_transaction.swap_link == Fr254::from(123u64))
            .count();
        assert_eq!(
            remaining_swap_legs, 2,
            "A valid non-expired swap pair should remain in mempool if it does not fit"
        );
    }
}
