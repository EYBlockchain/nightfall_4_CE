#![allow(dead_code)]
#![allow(unused_imports)]
use crate::{
    domain::entities::Preimage,
    driven::{contract_functions::contract_type_conversions::FrBn254, db::mongo::CommitmentEntry},
    get_fee_token_id,
    initialisation::get_db_connection,
    ports::{
        commitments::Commitment,
        db::{CommitmentDB, CommitmentEntryDB},
    },
};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger256, PrimeField, Zero};
use log::{debug, trace};
use mongodb::{Client, Database};
use nf_curves::ed_on_bn254::BJJTEAffine as JubJub;
use serde::{Deserialize, Serialize};
use std::{cmp, cmp::Ordering, collections::VecDeque, fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

const MAX_POSSIBLE_COMMITMENTS: usize = 2;

// Calculate the minimum commitments and minimum fee commitments to fulfil this selection.
// The max number of commitments that can be used is 2.
// If minC is 0, it means that the user doesn't have enough commitments to pay the value.
// If minC >2, it means that the user has too many dust commitments and we require client to deposit bigger commitment to fullfill this tx.
pub async fn find_usable_commitments(
    target_token_id: Fr254,
    target_value: Fr254,
    db: &Client,
) -> Result<[Preimage; MAX_POSSIBLE_COMMITMENTS], &'static str> {
    let (avaliable_sorted_commitments, min_num_c) =
        verify_enough_commitments(target_token_id, target_value, db).await?;

    let mut max_num_c = MAX_POSSIBLE_COMMITMENTS;
    if avaliable_sorted_commitments.len() < MAX_POSSIBLE_COMMITMENTS {
        max_num_c = avaliable_sorted_commitments.len();
    }

    // given the avaliable commitment, return the selected commitments
    // We want to use dusts first
    // Example: target_value = 3, commitments = [1, 2, 3]
    // We should use [1, 2] instead of [3] because the change is smaller/0
    let old_commitments = select_commitment(
        &avaliable_sorted_commitments,
        target_value,
        min_num_c,
        max_num_c,
    )?;

    // mark the selected dusts as pending
    let pending_keys = old_commitments
        .iter()
        .map(|c| c.hash())
        .collect::<Result<Vec<Fr254>, _>>()
        .map_err(|_| "Preimage hashing failed during commitment selection")?;

    // Mark Pending
    debug!("Marking these commitments as pending: {:?}", pending_keys);
    for key in pending_keys.iter() {
        debug!("{}", key);
    }
    db.mark_commitments_pending_spend(pending_keys).await;

    Ok([old_commitments[0], old_commitments[1]])
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
    trace!(
        "On-chain commitments for value: {:?}",
        on_chain_old_value_commitments
    );

    // Calculate the minimum number of commitments required for the value
    let min_c =
        calculate_minimum_commitments(&mut on_chain_old_value_commitments.clone(), target_value)
            .inspect_err(|e| {
                println!("Error calculating minimum commitments for value: {}", e);
            })?;
    trace!("Minimum commitments required for value: {}", min_c);
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
    use testcontainers::{
        core::IntoContainerPort, runners::AsyncRunner, ContainerAsync, GenericImage,
    };
    use tokio::io::AsyncReadExt;
    use url::Host;
    

    fn get_db_connection_uri(host: Host, port: u16) -> String {
        format!("mongodb://{}:{}", host, port)
    }

    /// This function creates a mongo container and returns it
    async fn get_mongo() -> ContainerAsync<GenericImage> {
        GenericImage::new("mongo", "4.4.1-bionic")
            .with_exposed_port(27017.udp())
            .start()
            .await
            .unwrap()
    }

    /// This function is used to provide a database connection to the tests
    async fn get_db_connection(container: &ContainerAsync<GenericImage>) -> mongodb::Client {
        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(27017).await.unwrap();
        mongodb::Client::with_uri_str(&get_db_connection_uri(host, port))
            .await
            .expect("Could not create database connection")
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
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(2u64),
            nf_token_id: Fr254::from(1u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(3u64),
            nf_token_id: Fr254::from(1u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(4u64),
            nf_token_id: Fr254::from(1u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
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
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(2u64),
            nf_token_id: Fr254::from(2u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(5u64),
            nf_token_id: Fr254::from(2u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(3u64),
            nf_token_id: Fr254::from(2u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
    ),
    CommitmentEntry::new(
        Preimage {
            value: Fr254::from(6u64),
            nf_token_id: Fr254::from(2u64),
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
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
            let value_result =
                find_usable_commitments(nf_token_id, target_value, &db).await;
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
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
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
            let value_result =
                find_usable_commitments(nf_token_id, target_value, &db).await;
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
    async fn test_commitment_selection_case_4_5() {
        // Case 4: all commitments values are smaller than target, and they are enough to cover the target
        // Case 5: all commitments values are smaller than target, and they are not enough to cover the target (only two commitments can be used)
        // value: 5,6,7,token_id: 1, target_value: 10, output: 5,6
        // fee: 2,5,6,12,13,token_id: 2, target_fee: 12, output: 12
        // Set up MongoDB test container
        let container = get_mongo().await;
        let db = get_db_connection(&container).await;

        // Insert mock commitments into a single database collection
        {
            let database = db.database("nightfall");
            let commitments_collection = database.collection::<CommitmentEntry>("commitments");

            let commitments = vec![
                CommitmentEntry {
                    preimage: Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(5u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(12u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(2u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(13u64),
                        nf_token_id: Fr254::from(2u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
            ];

            commitments_collection
                .insert_many(commitments)
                .await
                .expect("Failed to insert commitments into the database");
        }

        // Call the function under test
        let target_value = Fr254::from(10u64);
        let target_fee = Fr254::from(12u64);
        let nf_token_id = Fr254::from(1u64);
        let fee_token_id = Fr254::from(2u64);

        {
            let value_result =
                find_usable_commitments(nf_token_id, target_value, &db).await;

            // Validate results
            assert!(value_result.is_ok(), "Commitment selection failed");
            let selected_value_commitments = value_result.unwrap();

            let fee_result = find_usable_commitments(fee_token_id, target_fee, &db).await;

            // Validate results
            assert!(fee_result.is_ok(), "Commitment selection failed");
            let selected_fee_commitments = fee_result.unwrap();

            // Expected commitments: [5, 0] for value, [5, 6] for fee
            // old_commitments[0], old_commitments[1], old_fee_commitments[0], old_fee_commitments[1],
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
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
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
                            .contains("Not enough commitments to cover the value"),
                        "Error does not match expected string: {}",
                        err
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
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(6u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
                ),
                CommitmentEntry::new(
                    Preimage {
                        value: Fr254::from(7u64),
                        nf_token_id: Fr254::from(1u64),
                        ..Default::default()
                    },
                    Fr254::default(),
                    CommitmentStatus::Unspent,
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
                        "Error does not match expected string: {}",
                        err
                    );
                }
            }
        }
    }
}
