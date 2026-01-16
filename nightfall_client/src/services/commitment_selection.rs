#![allow(dead_code)]
#![allow(unused_imports)]
use crate::{
    driven::db::mongo::CommitmentEntry,
    initialisation::get_db_connection,
    ports::{
        db::{CommitmentDB, CommitmentEntryDB},
    },
};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger256, PrimeField, Zero};
use lib::{
    commitments::Commitment,
    contract_conversions::FrBn254, 
    get_fee_token_id, 
    hex_conversion::HexConvertible,
    shared_entities::Preimage,
};
use log::{debug, trace};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::{Client, Database};
use nf_curves::ed_on_bn254::BJJTEAffine as JubJub;
use serde::{Deserialize, Serialize};
use std::{cmp, cmp::Ordering, collections::VecDeque, fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

const MAX_POSSIBLE_COMMITMENTS: usize = 2;

// Calculate the minimum commitments required to fulfill this selection.
// The max number of commitments that can be used is MAX_POSSIBLE_COMMITMENTS.
// If min_num_c is 0, it means the user doesn't have enough commitments to pay the value.
// If min_num_c > MAX_POSSIBLE_COMMITMENTS, it means the user has too many dust commitments
// and we require the client to deposit larger commitments to fulfill this transaction.
//
// The function returns exactly MAX_POSSIBLE_COMMITMENTS preimages, with unused slots
// filled with Preimage::default().
pub async fn find_usable_commitments(
    target_token_id: Fr254,
    target_value: Fr254,
    db: &Client,
) -> Result<[Preimage; MAX_POSSIBLE_COMMITMENTS], &'static str> {
    // Verify enough commitments and get sorted available commitments
    let (avaliable_sorted_commitments, min_num_c) =
        verify_enough_commitments(target_token_id, target_value, db).await?;

    // Determine max number of commitments to use
    let max_num_c = avaliable_sorted_commitments
        .len()
        .min(MAX_POSSIBLE_COMMITMENTS);
    if max_num_c < min_num_c {
        return Err("Not enough commitments available to cover target value");
    }

    // Given the available commitments, select the ones to use for this transfer.
    // We want to use dusts first to minimize change.
    // Example: target_value = 3, commitments = [1, 2, 3]
    // We should use [1, 2] instead of [3]
    let selected_commitments = select_commitment(
        &avaliable_sorted_commitments,
        target_value,
        min_num_c,
        max_num_c,
    )?;

    // Get commitment IDs for atomic reservation
    let commitment_ids = selected_commitments
        .iter()
        .map(|c| c.hash())
        .collect::<Result<Vec<Fr254>, _>>()
        .map_err(|_| "Preimage hashing failed during commitment selection")?;

    // Atomic reservation to avoid TOCTOU
    let reserved_commitments = db.reserve_commitments_atomic(commitment_ids).await?;

    // Debug: show exactly what was successfully reserved
    debug!(
        "Reserved {} commitments atomically: {:?}",
        reserved_commitments.len(),
        reserved_commitments
            .iter()
            .filter_map(|c| c.hash().ok().map(|h| h.to_hex_string()))
            .collect::<Vec<_>>()
    );

    if reserved_commitments.len() < min_num_c {
        return Err("Could not reserve enough commitments - taken by another process");
    }

    // Convert reserved commitments to Preimage and return
    let preimages: Vec<Preimage> = reserved_commitments
        .iter()
        .map(|c| c.get_preimage())
        .collect::<Vec<_>>();

    let mut preimages_fixed = [Preimage::default(); MAX_POSSIBLE_COMMITMENTS];
    for (i, p) in preimages.into_iter().enumerate() {
        preimages_fixed[i] = p;
    }
    Ok(preimages_fixed)
}

fn select_commitment(
    commitments: &[Preimage],
    target_val: Fr254,
    min: usize,
    max: usize,
) -> Result<[Preimage; MAX_POSSIBLE_COMMITMENTS], &'static str> {
    // Get the commitments with size min..MAX_POSSIBLE_COMMITMENTS, return the best one
    // What's the best set: generate less change first, and then use more dust commitments if the change is the same
    let mut subsets: Vec<Vec<Preimage>> = Vec::new();
    if min == MAX_POSSIBLE_COMMITMENTS || max == 1 {
        match find_subset_commitments(commitments, target_val, min, vec![]) {
            Ok(subset) => subsets.push(subset),
            Err(e) => return Err(e),
        }
    } else {
        for i in min..MAX_POSSIBLE_COMMITMENTS + 1 {
            match find_subset_commitments(commitments, target_val, i, vec![]) {
                Ok(subset) => {
                    if !subset.is_empty() {
                        subsets.push(subset)
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    // We prioritize the subset that minimizes the change.
    // If two subsets have the same change,
    // we priority the subset that uses more commitments.
    subsets.sort_by(|a, b| {
        let change_a = a.iter().fold(Fr254::from(0), |acc, x| acc + x.get_value()) - target_val;
        let change_b = b.iter().fold(Fr254::from(0), |acc, x| acc + x.get_value()) - target_val;

        // First prioritize smaller change, then prioritize subsets with more elements
        match change_a.cmp(&change_b) {
            Ordering::Equal => b.len().cmp(&a.len()), // Favor subsets with more elements
            _ => change_a.cmp(&change_b),
        }
    });

    let mut res = (subsets.first().ok_or("No valid subsets found"))?.to_vec();
    res.resize(MAX_POSSIBLE_COMMITMENTS, Preimage::default());

    let fixed_length_out: [Preimage; MAX_POSSIBLE_COMMITMENTS] = res
        .try_into()
        .map_err(|_| "Could not convert commitment subset to fixed length array")?;

    Ok(fixed_length_out)
}

fn find_subset_commitments(
    commitments: &[Preimage],
    target_val: Fr254,
    n: usize,
    mut acc: Vec<Preimage>,
) -> Result<Vec<Preimage>, &'static str> {
    // n can only be 1 or 2, error has been thrown before if n = 0 or n > 2
    let values_below = commitments
        .iter()
        .filter(|a| a.get_value() < target_val)
        .map(|c| c.get_preimage())
        .collect::<Vec<_>>();

    let value_below_total = values_below
        .iter()
        .fold(Fr254::from(0), |acc, x| x.get_value() + acc);

    // Handle cases where we can use only 1 commitment
    if (values_below.len() <= 1) || (n == 1) {
        // Try to find a commitment with a value greater than or equal to the target value
        let res = commitments
            .iter()
            .find(|a| a.get_value() >= target_val)
            .ok_or("Failed to find a matching commitment")?; // Use `ok_or` to return an error in case no match is found

        acc.push(*res);
    } else if (n == 2) && (values_below.len() > 1) && (value_below_total >= target_val) {
        // Handle cases where 2 commitments are needed
        let result = find_subset_two_commitments(target_val, values_below);
        acc.extend(result);
    }

    Ok(acc)
}

/**
 * This function finds if there is any pair of commitments
 * whose sum value is equal or higher than the target_val
 */
fn find_subset_two_commitments(target_val: Fr254, values_below: Vec<Preimage>) -> Vec<Preimage> {
    let mut lhs = 0; // Left pointer
    let mut rhs = values_below.len() - 1; // Right pointer
    let max_value = <Fr254 as PrimeField>::MODULUS_MINUS_ONE_DIV_TWO;

    let mut change = max_value.into();
    let mut commitments_to_use = vec![];

    while lhs < rhs {
        let two_sum_commitment: Fr254 =
            values_below[lhs].get_value() + values_below[rhs].get_value();
        if two_sum_commitment == target_val {
            return vec![values_below[lhs], values_below[rhs]];
        }
        // Since the array of commitments is sorted by value,
        // depending if the sum is higher or smaller
        // we will move the left pointer (increase) or the right one
        if two_sum_commitment > target_val {
            let temp_change: Fr254 = two_sum_commitment - target_val;
            if temp_change < change {
                change = temp_change;
                commitments_to_use = vec![values_below[lhs], values_below[rhs]];
            }
            rhs -= 1;
        } else {
            lhs += 1;
        }
    }
    commitments_to_use
}

// Calculate the minimum number of commitments required
fn calculate_minimum_commitments(
    commitments: &mut [Preimage],
    target_value: Fr254,
) -> Result<usize, &'static str> {
    commitments.sort_by_key(|a| a.get_value());
    let mut sum_commitments = Fr254::from(0);
    let mut count = 0;

    for commitment in commitments.iter().rev().take(MAX_POSSIBLE_COMMITMENTS) {
        sum_commitments += commitment.get_value();
        count += 1;
        if sum_commitments >= target_value {
            return Ok(count);
        }
    }
    // check if there are enough balance to cover the value, but too many dust commitments
    for commitment in commitments.iter() {
        sum_commitments += commitment.get_value();
        if sum_commitments >= target_value {
            return Err("Sufficient balance to cover the value, but too many dust commitments — only up to two commitments are allowed.");
        }
    }
    Err("Not enough commitments to cover the value")
}

// Fetch and filter on-chain commitments
async fn fetch_on_chain_commitments(
    db: &Client,
    token_id: Fr254,
) -> Result<Vec<Preimage>, &'static str> {
    let commitments = db
        .get_available_commitments(token_id)
        .await
        .ok_or("No commmitments found in the db")?;
    Ok(commitments.into_iter().map(|c| c.get_preimage()).collect())
}

async fn verify_enough_commitments(
    target_token_id: Fr254,
    target_value: Fr254,
    db: &Client,
) -> Result<(std::vec::Vec<Preimage>, usize), &'static str> {
    // Fetch on-chain commitments for the non-fee component
    let mut on_chain_old_value_commitments =
        fetch_on_chain_commitments(db, target_token_id).await?;
    on_chain_old_value_commitments.sort_by_key(|a| a.get_value());
    trace!("On-chain commitments for value: {on_chain_old_value_commitments:?}");

    // Calculate the minimum number of commitments required for the value
    let min_c =
        calculate_minimum_commitments(&mut on_chain_old_value_commitments.clone(), target_value)
            .inspect_err(|e| {
                println!("Error calculating minimum commitments for value: {e}");
            })?;
    trace!("Minimum commitments required for value: {min_c}");
    // Handle case where too many dust commitments are required
    if min_c > MAX_POSSIBLE_COMMITMENTS {
        return Err("Too many dust commitments found; only up to two commitments can be used to cover the value");
    }

    Ok((on_chain_old_value_commitments.clone(), min_c))
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::entities::CommitmentStatus;
    use ark_bn254::Fr as Fr254;
    use lib::tests_utils::{get_db_connection, get_db_connection_uri, get_mongo};
    use mongodb::bson::doc;
    use url::Host;

    #[tokio::test]
    async fn test_find_usable_commitments_success() {
        // 1. Setup: start Mongo test container and get DB connection
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let commitments_collection = db
            .database("nightfall")
            .collection::<CommitmentEntry>("commitments");

        // Ensure the collection is clean before inserting test data
        commitments_collection
            .delete_many(doc! {})
            .await
            .expect("Failed to clear commitments collection");

        // Insert commitments for token_id = 1 (value commitments)
        let value_commitments = vec![
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(5u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(6u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(7u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
        ];
        commitments_collection
            .insert_many(value_commitments)
            .await
            .expect("Failed to insert value commitments");

        // 2. Call function under test (target = 10, token_id = 1)
        let target_value = Fr254::from(10u64);
        let token_id = Fr254::from(1u64);
        let result = find_usable_commitments(token_id, target_value, &db).await;

        // 3. Validate result
        assert!(result.is_ok(), "Commitment selection failed");
        let selected = result.unwrap();

        // The function should select commitments 5 and 6 (sum >= 10)
        assert_eq!(selected[0].value, Fr254::from(5u64));
        assert_eq!(selected[0].nf_token_id, Fr254::from(1u64));
        assert_eq!(selected[1].value, Fr254::from(6u64));
        assert_eq!(selected[1].nf_token_id, Fr254::from(1u64));
    }

    #[tokio::test]
    async fn test_find_usable_commitments_exact_fee_match() {
        // 1. Setup: start Mongo test container and get DB connection
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let commitments_collection = db
            .database("nightfall")
            .collection::<CommitmentEntry>("commitments");

        // Ensure the collection is clean before inserting test data
        commitments_collection
            .delete_many(doc! {})
            .await
            .expect("Failed to clear commitments collection");

        // Insert commitments for token_id = 2 (fee commitments)
        let fee_commitments = vec![
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(2u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(12u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(13u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
        ];
        commitments_collection
            .insert_many(fee_commitments)
            .await
            .expect("Failed to insert fee commitments");

        // 2. Call function under test (target = 12, token_id = 2)
        let target_fee = Fr254::from(12u64);
        let token_id = Fr254::from(2u64);
        let result = find_usable_commitments(token_id, target_fee, &db).await;

        // 3. Validate result
        assert!(result.is_ok(), "Fee commitment selection failed");
        let selected = result.unwrap();

        // Expect an exact match with 12
        assert_eq!(selected[0].value, Fr254::from(12u64));
        assert_eq!(selected[0].nf_token_id, Fr254::from(2u64));

        // The second slot should be a dummy (0,0)
        assert_eq!(selected[1].value, Fr254::from(0u64));
        assert_eq!(selected[1].nf_token_id, Fr254::from(0u64));
    }

    #[tokio::test]
    async fn test_commitment_selection_case_1_2() {
        // Case 1: When subsets have the same changes, prioritize dust commitments.
        // Case 2: prioritize dust commitments with smaller change
        // value: 1, 2, 3, 4,token_id: 1, target_value: 3, output: 1, 2
        // fee: 1, 2, 5, 3, 6,token_id: 2, target_fee: 4, output: 1,3
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert mock commitments into a single database collection
        {
            let database = db.database("nightfall");
            let commitments_collection = database.collection::<CommitmentEntry>("commitments");

            let commitments = vec![
                // Value commitments for nf_token_id: 1
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(1u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(2u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(3u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(4u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                // Fee commitments for nf_token_id: 2
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(1u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(2u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(3u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
            ];
            // Insert the commitments into the database
            commitments_collection
                .insert_many(commitments)
                .await
                .expect("Failed to insert commitments into the database");

            // Immediately fetch and print to ensure data is present
            use mongodb::bson::doc;
            let filter = doc! {};
            let count = commitments_collection
                .count_documents(filter)
                .await
                .unwrap();
            assert_eq!(count, 9);
        }

        // Call the function under test
        let target_value = Fr254::from(3u64);
        let target_fee = Fr254::from(4u64);
        let nf_token_id = Fr254::from(1u64);
        let fee_token_id = Fr254::from(2u64);

        {
            let value_result = find_usable_commitments(nf_token_id, target_value, &db).await;
            let fee_result = find_usable_commitments(fee_token_id, target_fee, &db).await;

            // Validate results
            assert!(value_result.is_ok(), "Value Commitment selection failed");
            let selected_value_commitments = value_result.unwrap();
            assert!(fee_result.is_ok(), "Fee Commitment selection failed");
            let selected_fee_commitments = fee_result.unwrap();

            // Expected commitments: [1, 2] for value, [1, 3] for fee
            // old_commitments[0], old_commitments[1], old_fee_commitments[0], old_fee_commitments[1],
            assert_eq!(
                (
                    selected_value_commitments[0].value,
                    selected_value_commitments[0].nf_token_id
                ),
                (Fr254::from(1u64), Fr254::from(1u64))
            );
            assert_eq!(
                (
                    selected_value_commitments[1].value,
                    selected_value_commitments[1].nf_token_id
                ),
                (Fr254::from(2u64), Fr254::from(1u64))
            );
            assert_eq!(
                (
                    selected_fee_commitments[0].value,
                    selected_fee_commitments[0].nf_token_id
                ),
                (Fr254::from(1u64), Fr254::from(2u64))
            );
            assert_eq!(
                (
                    selected_fee_commitments[1].value,
                    selected_fee_commitments[1].nf_token_id
                ),
                (Fr254::from(3u64), Fr254::from(2u64))
            );
        }
    }
    #[tokio::test]
    async fn test_commitment_selection_case_3() {
        // Case 3: all commitments values are bigger than target
        // value: 5,6,7,token_id: 1, target_value: 3, output: 5
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert mock commitments into a single database collection
        {
            let database = db.database("nightfall");
            let commitments_collection = database.collection::<CommitmentEntry>("commitments");

            let commitments = vec![
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
            ];

            commitments_collection
                .insert_many(commitments)
                .await
                .expect("Failed to insert commitments into the database");
        }

        // Call the function under test
        let target_value = Fr254::from(3u64);
        let nf_token_id = Fr254::from(1u64);

        {
            let value_result = find_usable_commitments(nf_token_id, target_value, &db).await;
            // Validate results
            assert!(value_result.is_ok(), "Commitment selection failed");
            let selected_value_commitments = value_result.unwrap();

            // // Expected commitments: [5, 0] for value
            // // old_commitments[0], old_commitments[1], old_fee_commitments[0], old_fee_commitments[1],
            assert_eq!(
                (
                    selected_value_commitments[0].value,
                    selected_value_commitments[0].nf_token_id
                ),
                (Fr254::from(5u64), Fr254::from(1u64))
            );
            assert_eq!(
                (
                    selected_value_commitments[1].value,
                    selected_value_commitments[1].nf_token_id
                ),
                (Fr254::from(0u64), Fr254::from(0u64))
            );
        }
    }
    #[tokio::test]
    async fn test_commitment_selection_case_4_value() {
        // Case 4: all commitments values are smaller than target, and they are enough to cover the target
        // value: 5, 6, 7, token_id: 1, target_value: 10, output: 5,6
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        let commitments_collection = db
            .database("nightfall")
            .collection::<CommitmentEntry>("commitments");

        // Clear and insert only the value commitments
        commitments_collection
            .delete_many(doc! {})
            .await
            .expect("Failed to clear commitments");
        let value_commitments = vec![
            // Only insert commitments for nf_token_id: 1
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(5u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(6u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(7u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
        ];
        commitments_collection
            .insert_many(value_commitments)
            .await
            .expect("Failed to insert value commitments");

        // Test and validate the value selection
        let value_result =
            find_usable_commitments(Fr254::from(1u64), Fr254::from(10u64), &db).await;
        assert!(
            value_result.is_ok(),
            "Commitment selection for value failed"
        );
        let selected_value_commitments = value_result.unwrap();

        assert_eq!(
            (
                selected_value_commitments[0].value,
                selected_value_commitments[0].nf_token_id
            ),
            (Fr254::from(5u64), Fr254::from(1u64))
        );
        assert_eq!(
            (
                selected_value_commitments[1].value,
                selected_value_commitments[1].nf_token_id
            ),
            (Fr254::from(6u64), Fr254::from(1u64))
        );
    }
    #[tokio::test]
    async fn test_commitment_selection_case_5_fee() {
        // Case 5: a commitment value exactly matches the target fee
        // fee: 2, 5, 6, 12, 13, token_id: 2, target_fee: 12, output: 12
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        let commitments_collection = db
            .database("nightfall")
            .collection::<CommitmentEntry>("commitments");

        // Clear and insert only the fee commitments
        commitments_collection
            .delete_many(doc! {})
            .await
            .expect("Failed to clear commitments");
        let fee_commitments = vec![
            // Only insert commitments for nf_token_id: 2
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(5u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(6u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(12u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(2u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(13u64),
                    nf_token_id: Fr254::from(2u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
        ];
        commitments_collection
            .insert_many(fee_commitments)
            .await
            .expect("Failed to insert fee commitments");

        // Test and validate the fee selection
        let fee_result = find_usable_commitments(Fr254::from(2u64), Fr254::from(12u64), &db).await;
        assert!(fee_result.is_ok(), "Commitment selection for fee failed");
        let selected_fee_commitments = fee_result.unwrap();

        assert_eq!(
            (
                selected_fee_commitments[0].value,
                selected_fee_commitments[0].nf_token_id
            ),
            (Fr254::from(12u64), Fr254::from(2u64))
        );
        assert_eq!(
            (
                selected_fee_commitments[1].value,
                selected_fee_commitments[1].nf_token_id
            ),
            (Fr254::from(0u64), Fr254::from(0u64))
        );
    }

    #[tokio::test]
    async fn test_commitment_selection_case_6() {
        // Case 6: too many dust commitments, the sum of all commitments is enough to cover the target
        // value: 5,6,7,token_id: 1, target_value: 14, catch error
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert mock commitments into a single database collection
        {
            let database = db.database("nightfall");
            let commitments_collection = database.collection::<CommitmentEntry>("commitments");

            let commitments = vec![
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
            ];

            commitments_collection
                .insert_many(commitments)
                .await
                .expect("Failed to insert commitments into the database");
        }

        // Call the function under test
        let target_value = Fr254::from(14u64);
        let nf_token_id = Fr254::from(1u64);

        {
            let result = find_usable_commitments(nf_token_id, target_value, &db).await;
            // catch error
            match result {
                Ok(_) => panic!("Expected an error, but got Ok."),
                Err(err) => {
                    assert!(
                        err.to_string()
                            .contains("Sufficient balance to cover the value, but too many dust commitments — only up to two commitments are allowed."),
                        "Error does not match expected string: {err}"
                    );
                }
            }
        }
    }
    #[tokio::test]
    async fn test_commitment_selection_case_7() {
        // Case 8: not enough commitments, the sum of all commitments is not enough to cover the target
        // value: 5,6,7,token_id: 1, target_value: 100, catch error
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert mock commitments into a single database collection
        {
            let database = db.database("nightfall");
            let commitments_collection = database.collection::<CommitmentEntry>("commitments");

            let commitments = vec![
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                    None,
                    None,
                ),
            ];

            commitments_collection
                .insert_many(commitments)
                .await
                .expect("Failed to insert commitments into the database");
        }

        // Call the function under test
        let target_value = Fr254::from(100u64);
        let nf_token_id = Fr254::from(1u64);
        {
            let result = find_usable_commitments(nf_token_id, target_value, &db).await;
            // catch error
            match result {
                Ok(_) => panic!("Expected an error, but got Ok."),
                Err(err) => {
                    assert!(
                        err.to_string()
                            .contains("Not enough commitments to cover the value"),
                        "Error does not match expected string: {err}"
                    );
                }
            }
        }
    }

    // Test concurrent access to ensure atomic reservation
    #[tokio::test]
    async fn test_find_usable_commitments() {
        // This test verifies the atomic reservation of commitments.
        // It simulates two concurrent processes trying to reserve the same commitments.
        // Only one process should succeed; the other must fail, preventing race conditions.

        let container = get_mongo().await;
        let db = get_db_connection(&container).await;
        let commitments_collection = db
            .database("nightfall")
            .collection::<CommitmentEntry>("commitments");

        commitments_collection.delete_many(doc! {}).await.unwrap();

        let commitments = vec![
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(10u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
            CommitmentEntry::new(
                Preimage {
                    value: Fr254::from(20u64),
                    nf_token_id: Fr254::from(1u64),
                    ..Default::default()
                },
                Fr254::default(),
                CommitmentStatus::Unspent,
                None,
                None,
            ),
        ];
        commitments_collection
            .insert_many(&commitments)
            .await
            .unwrap();

        // Spawn two concurrent tasks
        let db1 = db.clone();
        let db2 = db.clone();

        let handle1 = tokio::spawn(async move {
            find_usable_commitments(Fr254::from(1u64), Fr254::from(30u64), &db1).await
        });

        let handle2 = tokio::spawn(async move {
            find_usable_commitments(Fr254::from(1u64), Fr254::from(30u64), &db2).await
        });

        let res1 = handle1.await.unwrap();
        let res2 = handle2.await.unwrap();

        // Ensure atomicity: only one concurrent task can reserve the commitments, the other must fail
        let success_count = [res1, res2].iter().filter(|r| r.is_ok()).count();
        let failure_count = [res1, res2].iter().filter(|r| r.is_err()).count();

        assert_eq!(
            success_count, 1,
            "Only one process should successfully reserve all commitments"
        );
        assert_eq!(
            failure_count, 1,
            "The other process should fail due to commitments being already reserved"
        );
    }
}
