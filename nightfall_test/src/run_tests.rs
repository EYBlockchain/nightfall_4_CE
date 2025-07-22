use crate::{
    test::{
        self, create_nf3_deposit_transaction, create_nf3_transfer_transaction,
        create_nf3_withdraw_transaction, de_escrow_request, get_key, get_recipient_address,
        load_addresses, set_anvil_mining_interval, verify_deposit_commitments_nf_token_id,
        wait_for_all_responses, wait_on_chain, TokenType,
    },
    test_settings::TestSettings,
    validate_certs::validate_all_certificates,
};
use ark_bn254::Fr as Fr254;
use ark_std::{collections::HashMap, Zero};
use configuration::settings::{get_settings, Settings};
use ethers::{
    providers::Middleware,
    types::{TransactionReceipt, U256},
    utils::{format_units, parse_units},
};
use futures::future::try_join_all;
use lib::{
    blockchain_client::BlockchainClientConnection, hex_conversion::HexConvertible,
    initialisation::get_blockchain_client_connection,
};
use log::{debug, info, warn};
use nightfall_client::drivers::rest::{client_nf_3::WithdrawResponse, models::DeEscrowDataReq};
use serde_json::Value;
use test::{
    anvil_reorg, count_spent_commitments, get_erc20_balance, get_erc721_balance, get_fee_balance,
};
use url::Url;
use uuid::Uuid;

pub async fn run_tests(
    responses: std::sync::Arc<tokio::sync::Mutex<Vec<serde_json::Value>>>,
    mining_interval: u32,
) {
    let settings: Settings = Settings::new().unwrap();
    let test_settings: TestSettings = TestSettings::new().unwrap();
    info!("Running tests on nightall_client http:// interface");

    // override the mining interval that may have been set in Anvil. If Anvil was set to automine, also turn that off
    let http_client = reqwest::Client::new();
    let url = Url::parse("http://anvil:8545").unwrap();
    set_anvil_mining_interval(&http_client, &url, mining_interval)
        .await
        .expect("Failed to set Anvil mining interval");

    // generate the zkp keys (they will be held in-memory in the client)
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("v1/deriveKey")
        .unwrap();
    let key_request = test_settings.key_request;
    let _zkp_key = get_key(url, &key_request).await.unwrap();
    let url = Url::parse("http://client2:3000")
        .unwrap()
        .join("v1/deriveKey")
        .unwrap();
    let key_request2 = test_settings.key_request2;
    let zkp_key2 = get_key(url, &key_request2).await.unwrap();
    info!("* zkp keys created");

    let _ = load_addresses(&settings).unwrap();
    info!("* contract addresses obtained");
    // Validate all certificates (clients and proposer)
    // (name, cert_path, key_path, url)
    let certs: [(&'static str, &'static str, &'static str, Url); 3] = [
        (
            "Client 1",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-1.der",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-1.priv_key",
            Url::parse(&settings.nightfall_client.url)
                .unwrap()
                .join("/v1/certification")
                .unwrap(),
        ),
        (
            "Client 2",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-2.der",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key",
            Url::parse("http://client2:3000")
                .unwrap()
                .join("/v1/certification")
                .unwrap(),
        ),
        (
            "Proposer",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-3.der",
            "../blockchain_assets/test_contracts/X509/_certificates/user/user-3.priv_key",
            Url::parse(&settings.nightfall_proposer.url)
                .unwrap()
                .join("/v1/certification")
                .unwrap(),
        ),
    ];
    validate_all_certificates(certs, &http_client).await;

    //see if the NF4_LARGE_BLOCK_TEST environment variable is set to 'true' and run the large block test only if it is
    let (
        client1_starting_balance,
        client2_starting_balance,
        client1_starting_fee_balance,
        nullified_count,
    ) = if std::env::var("NF4_LARGE_BLOCK_TEST").is_ok()
        && std::env::var("NF4_LARGE_BLOCK_TEST").unwrap() == "true"
    {
        warn!("Running large block test");
        const N_LARGE_BLOCK: usize = 64;
        const DEPOSIT_FEE: &str = "0x06";

        // work out how much we'll change the balance of the two clients by making the large block deposits
        let client2_starting_balance = N_LARGE_BLOCK as i64
            * i64::from_hex_string(&test_settings.erc20_transfer_large_block.value).unwrap();
        let client1_starting_balance = N_LARGE_BLOCK as i64
            * 2
            * i64::from_hex_string(&test_settings.erc20_deposit_large_block.value).unwrap()
            - client2_starting_balance;
        let client2_starting_fee_balance = N_LARGE_BLOCK as i64
            * i64::from_hex_string(&test_settings.erc20_transfer_large_block.fee).unwrap();
        let client1_starting_fee_balance =
            N_LARGE_BLOCK as i64 * 2 * i64::from_hex_string(DEPOSIT_FEE).unwrap()
                - client2_starting_fee_balance;

        // make up to 64 deposits so that we can test a large block (reuse deposit 2 data)
        // first we need to pause block assembly so that we can make all the deposits in the same block
        let pause_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/pause")
            .unwrap();
        let res = http_client.get(pause_url).send().await.unwrap();
        assert!(res.status().is_success());
        // create deposit transactions first
        info!("Making 64 deposit transactions");
        let url = Url::parse(&settings.nightfall_client.url)
            .unwrap()
            .join("v1/deposit")
            .unwrap();
        let mut large_block_deposit_ids = vec![];
        for _ in 0..N_LARGE_BLOCK * 2 {
            let large_block_deposit_id = create_nf3_deposit_transaction(
                &http_client,
                url.clone(),
                TokenType::ERC20,
                test_settings.erc20_deposit_large_block.clone(),
                DEPOSIT_FEE.to_string(), //deposit_fee
            );
            // save the IDs of the deposits so that we can wait for them to be on-chain
            large_block_deposit_ids.push(large_block_deposit_id);
        }

        // throw all the transactions at the client as fast as we can
        let large_block_deposit_ids = try_join_all(large_block_deposit_ids).await.unwrap();
        let large_block_deposit_ids = large_block_deposit_ids
            .iter()
            .map(|(uuid, _)| *uuid)
            .collect::<Vec<_>>();

        // wait for all the responses to come back and convert the json responses to a vector of Fr254 commitments
        info!("Waiting for deposit responses");
        let large_block_deposits =
            wait_for_all_responses(&large_block_deposit_ids, responses.clone())
                .await
                .into_iter()
                .flat_map(|(_, l)| {
                    serde_json::from_str::<Vec<String>>(&l).expect("Failed to parse response")
                })
                .map(|l| Fr254::from_hex_string(&l).unwrap())
                .collect::<Vec<_>>();
        // note that the responses vector is now empty

        //now we can resume block assembly
        let resume_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/resume")
            .unwrap();
        let res = http_client.get(resume_url).send().await.unwrap();
        assert!(res.status().is_success());
        info!("Waiting for deposits to be on-chain");
        wait_on_chain(&large_block_deposits, &get_settings().nightfall_client.url)
            .await
            .unwrap();

        info!("A large block full of ERC20 Deposits is now on-chain");

        // next, we'll do transfers
        // but first we need to pause block assembly so that we can make all the transfers in the same block
        let pause_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/pause")
            .unwrap();
        let res = http_client.get(pause_url).send().await.unwrap();
        assert!(res.status().is_success());
        let url = Url::parse(&settings.nightfall_client.url)
            .unwrap()
            .join("v1/transfer")
            .unwrap();
        // then make n transfers
        info!("Making 64 transfer transactions");
        let mut large_block_transfer_ids = vec![];
        for _ in 0..N_LARGE_BLOCK {
            let large_block_transfer_id = create_nf3_transfer_transaction(
                zkp_key2,
                &http_client,
                url.clone(),
                TokenType::ERC20,
                test_settings.erc20_transfer_large_block.clone(),
            );
            large_block_transfer_ids.push(large_block_transfer_id);
        }

        // throw all the transactions at the client as fast as we can
        let large_block_transfer_ids = try_join_all(large_block_transfer_ids).await.unwrap();

        // wait for responses to the transfer requests
        info!("Waiting for transfer responses");
        let large_block_transfers =
            wait_for_all_responses(&large_block_transfer_ids, responses.clone())
                .await
                .into_iter()
                .map(|(_, l)| {
                    serde_json::from_str::<(Value, Option<TransactionReceipt>)>(&l)
                        .expect("Failed to parse response")
                })
                .map(|l| l.0)
                .collect::<Vec<_>>();

        // work out how many nullifiers we spent
        let nullifier_count: usize = large_block_transfers
            .iter()
            .flat_map(|l| l["nullifiers"].as_array().unwrap())
            .filter(|n| !((Fr254::from_hex_string(n.as_str().unwrap()).unwrap()).is_zero()))
            .count();

        //now we can resume block assembly
        let resume_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/resume")
            .unwrap();
        let res = http_client.get(resume_url).send().await.unwrap();
        assert!(res.status().is_success());
        info!("Waiting for transfers to be on-chain");
        wait_on_chain(
            large_block_transfers
                .iter()
                .map(|l| Fr254::from_hex_string(l["commitments"][0].as_str().unwrap()).unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
            "http://client2:3000",
        )
        .await
        .unwrap();
        info!("A large block full of ERC20 Transfers is now on-chain");
        (
            client1_starting_balance,
            client2_starting_balance,
            client1_starting_fee_balance,
            nullifier_count,
        )
    } else {
        (0, 0, 0, 0)
    };

    /***********************************************************************************************
     * Tests using the client_nf_3 API
     **********************************************************************************************/
    // Test values are carefully chosen so we can test the full range of token types and values, please don't change them. Instead, please add new tests if you need to test new values.
    // To make the tests more readable and easier to debug, we submit commitments to blockchain everytime when we make requests for a specific token.
    info!("Commencing tests using the client_nf_3 API");

    let pause_url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("v1/pause")
        .unwrap();
    let res = http_client.get(pause_url).send().await.unwrap();
    assert!(res.status().is_success());
    // // create deposit requests
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("v1/deposit")
        .unwrap();

    // this vector stores all the deposit ids that we get back from out requests.
    let mut deposit_requests = vec![];

    deposit_requests.push(create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_deposit_0.clone(),
        "0x00".to_string(), //deposit_fee
    ));
    debug!("transaction_erc20_deposit_0 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_deposit_1,
    //     "0x27".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc20_deposit_1 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_deposit_2.clone(),
    //     "0x06".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc20_deposit_2 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_deposit_3,
    //     "0x00".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc20_deposit_3 has been created");

    // // check that we have no 'balance' of the ERC721 token
    // // get the balance of the ERC721 token we just deposited
    // let balance = get_erc721_balance(
    //     &http_client,
    //     Url::parse(&settings.nightfall_client.url).unwrap(),
    //     test_settings.erc721_deposit.token_id.clone(),
    // )
    // .await;
    // assert_eq!(None, balance);

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC721,
    //     test_settings.erc721_deposit.clone(),
    //     "0x08".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc721_deposit has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC3525,
    //     test_settings.erc3525_deposit_1,
    //     "0x0b".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc3525_deposit_1 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC3525,
    //     test_settings.erc3525_deposit_2,
    //     "0x0e".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc3525_deposit_2 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_deposit_1,
    //     "0x11".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc1155_deposit_1 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_deposit_2,
    //     "0x14".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc1155_deposit_2 has been created");

    // deposit_requests.push(create_nf3_deposit_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_deposit_3_nft,
    //     "0x16".to_string(), //deposit_fee
    // ));
    // debug!("transaction_erc1155_deposit_3 has been created");

    // throw all the transactions at the client as fast as we can
    let mut transaction_data = try_join_all(deposit_requests).await.unwrap();
    // sort by Uuid
    transaction_data.sort_by_key(|(uuid, _)| *uuid);

    //  Extract UUIDs and store expected token info for verification
    let transaction_ids: Vec<Uuid> = transaction_data.iter().map(|(uuid, _)| *uuid).collect();
    // Build a lookup for later token validation

    let mut expected_token_data: HashMap<Uuid, Vec<(String, String)>> = HashMap::new();
    for (uuid, deposit_infos) in transaction_data.iter() {
        for info in deposit_infos {
            expected_token_data
                .entry(*uuid)
                .or_default()
                .push((info.erc_address.clone(), info.token_id.clone()));
        }
    }

    // Wait for webhook responses
    let responses_by_uuid = wait_for_all_responses(&transaction_ids, responses.clone()).await;
    // Sanity check: ensure matching UUID order
    for (i, response) in responses_by_uuid.clone().iter().enumerate() {
        assert_eq!(
            response.0, transaction_data[i].0,
            "{}th Deposit response Uuid does not match deposit data Uuid",
            i
        );
    }
    // Extract commitment hashes
    let commitment_hashes = responses_by_uuid
        .clone()
        .into_iter()
        .flat_map(|(_, l)| {
            serde_json::from_str::<Vec<String>>(&l).expect("Failed to parse response")
        })
        .map(|l| Fr254::from_hex_string(&l).unwrap())
        .collect::<Vec<_>>();
    // Wait for commitments to appear
    let resume_url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("v1/resume")
        .unwrap();
    let res = http_client.get(resume_url).send().await.unwrap();
    assert!(res.status().is_success());

    // for each deposit request, we have value commitment and fee commitment (if fee is non-zero)
    // wait for the commitments to appear on-chain - we can't transfer until they are there
    wait_on_chain(&commitment_hashes, &get_settings().nightfall_client.url)
        .await
        .unwrap();
    info!("Deposit commitments for client 1 are now on-chain");

    // // get the balance of the ERC721 token we just deposited
    // let balance = get_erc721_balance(
    //     &http_client,
    //     Url::parse(&settings.nightfall_client.url).unwrap(),
    //     test_settings.erc721_deposit.token_id,
    // )
    // .await;
    // assert!(balance.is_some_and(|balance| balance.is_zero()));

    // // get the fee balance
    // let fee_balance = get_fee_balance(
    //     &http_client,
    //     Url::parse(&settings.nightfall_client.url).unwrap(),
    // )
    // .await;
    // info!(
    //     "Fee Commitment Balance  held as layer 2 commitments by client1: {}",
    //     fee_balance
    // );
    // assert_eq!(fee_balance, 137 + client1_starting_fee_balance);
    // // call verify_deposit_commitments_nf_token_id
    // info!("Verifying deposit commitments");

    // // check that we can find one of our commitments
    // // Query the commitment endpoint to return the CommitmEntry of commitment_hashes[0]
    // info!("Querying commitment endpoint");
    // // Cache for token info lookup
    // let uuid_to_commitments: HashMap<Uuid, Vec<Fr254>> = responses_by_uuid
    //     .clone()
    //     .iter()
    //     .map(|(uuid, l)| {
    //         let commitments: Vec<Fr254> = serde_json::from_str::<Vec<String>>(l)
    //             .expect("Failed to parse commitment response")
    //             .into_iter()
    //             .map(|s| Fr254::from_hex_string(&s).unwrap())
    //             .collect();
    //         (*uuid, commitments)
    //     })
    //     .collect();
    // verify_deposit_commitments_nf_token_id(
    //     &http_client,
    //     &uuid_to_commitments,
    //     &expected_token_data,
    //     &settings,
    // )
    // .await;

    // info!("Making client2 fee commitments so that it can withdraw");
    // // give client 2 some deposit fee commitments so that it can transact
    // // we need up to seven commitments because we'll want to do up to seven withdraws in
    // // the same block (we don't control when a block is computed), so we can't use a single commitment
    // // even if it has enough value because the change won't be available until the next block.
    // let pause_url = Url::parse(&settings.nightfall_proposer.url)
    //     .unwrap()
    //     .join("v1/pause")
    //     .unwrap();
    // let res = http_client.get(pause_url).send().await.unwrap();
    // assert!(res.status().is_success());

    // let url2 = Url::parse("http://client2:3000")
    //     .unwrap()
    //     .join("v1/deposit")
    //     .unwrap();

    // let mut transaction_ids = vec![];

    // for _ in 0..7 {
    //     transaction_ids.push(create_nf3_deposit_transaction(
    //         &http_client,
    //         url2.clone(),
    //         TokenType::ERC20,
    //         test_settings.erc20_deposit_4.clone(),
    //         "0x20".to_string(), //deposit_fee
    //     ));
    //     debug!("transaction_erc20_deposit_4 has been created");
    // }

    // // throw all the transactions at the client as fast as we can
    // let transaction_ids = try_join_all(transaction_ids).await.unwrap();
    // let transaction_ids = transaction_ids
    //     .iter()
    //     .map(|(uuid, _)| *uuid)
    //     .collect::<Vec<_>>();

    // // wait for the responses to the deposit requests to come back to the webhook server
    // let commitment_hashes = wait_for_all_responses(&transaction_ids, responses.clone())
    //     .await
    //     .into_iter()
    //     .flat_map(|(_, l)| {
    //         serde_json::from_str::<Vec<String>>(&l).expect("Failed to parse response")
    //     })
    //     .map(|l| Fr254::from_hex_string(&l).unwrap())
    //     .collect::<Vec<_>>();

    // let resume_url = Url::parse(&settings.nightfall_proposer.url)
    //     .unwrap()
    //     .join("v1/resume")
    //     .unwrap();
    // let res = http_client.get(resume_url).send().await.unwrap();
    // assert!(res.status().is_success());

    // // wait for the client2 fee commitments to appear on-chain
    // wait_on_chain(&commitment_hashes, "http://client2:3000")
    //     .await
    //     .unwrap();
    // info!("Client2 ERC20 fee commitments are now on-chain");

    // // get the balance of the ERC20 tokens we just deposited
    // let balance = get_erc20_balance(
    //     &http_client,
    //     Url::parse(&settings.nightfall_client.url).unwrap(),
    // )
    // .await;
    // info!(
    //     "Balance of ERC20 tokens held as layer 2 commitments by client 1: {}",
    //     balance
    // );
    // assert_eq!(balance, 14 + client1_starting_balance);

    // let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    // info!(
    //     "Balance of ERC20 tokens held as layer 2 commitments by client 2: {}",
    //     balance
    // );
    // assert_eq!(balance, 7 + client2_starting_balance);

    // info!("Sending transfer transactions");
    // let pause_url = Url::parse(&settings.nightfall_proposer.url)
    //     .unwrap()
    //     .join("v1/pause")
    //     .unwrap();
    // let res = http_client.get(pause_url).send().await.unwrap();
    // assert!(res.status().is_success());

    // // create transfer requests
    // let mut transaction_ids = vec![];

    // let url = Url::parse(&settings.nightfall_client.url)
    //     .unwrap()
    //     .join("v1/transfer")
    //     .unwrap();

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_transfer_0,
    // ));

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_transfer_1,
    // ));

    // debug!("transaction_erc20_transfer_1 has been created");
    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_transfer_2,
    // ));

    // // throw all the transactions at the client as fast as we can
    // let transaction_ids = try_join_all(transaction_ids).await.unwrap();

    // // wait for the responses to the transfer requests to come back to the webhook server
    // let transactions = wait_for_all_responses(&transaction_ids, responses.clone())
    //     .await
    //     .into_iter()
    //     .map(|(_, l)| {
    //         serde_json::from_str::<(Value, Option<TransactionReceipt>)>(&l)
    //             .expect("Failed to parse response")
    //     })
    //     .map(|l| l.0)
    //     .collect::<Vec<_>>();

    // info!("Starting chain reorg, {} blocks reorged", 200);

    // anvil_reorg(
    //     &http_client,
    //     &Url::parse("http://anvil:8545").unwrap(),
    //     200,
    //     true,
    //     5,
    // )
    // .await
    // .unwrap();

    // info!("====== Chain reorg completed =========");

    // // compute the commmitments for the transactions
    // let commitment_hashes = transactions
    //     .iter()
    //     .map(|l| Fr254::from_hex_string(l["commitments"][0].as_str().unwrap()).unwrap())
    //     .collect::<Vec<_>>();

    // debug!("transaction_erc20_transfer_2 has been created");
    // let resume_url = Url::parse(&settings.nightfall_proposer.url)
    //     .unwrap()
    //     .join("v1/resume")
    //     .unwrap();
    // let res = http_client.get(resume_url).send().await.unwrap();
    // assert!(res.status().is_success());

    // wait_on_chain(&commitment_hashes, "http://client2:3000")
    //     .await
    //     .unwrap();
    // info!("ERC20 Transfer commitments are now on-chain");

    // // check that we have nullified the correct number of commitments
    // let nullifier_count = transactions
    //     .iter()
    //     .flat_map(|l| l["nullifiers"].as_array().unwrap())
    //     .map(|n| Fr254::from_hex_string(n.as_str().unwrap()).unwrap())
    //     .filter(|&n| !n.is_zero())
    //     .count()
    //     + nullified_count;

    // info!("Expected spent commitment count: {}", nullifier_count);
    // let spent_commitments = count_spent_commitments(&http_client, url.clone())
    //     .await
    //     .unwrap();
    // assert_eq!(spent_commitments, nullifier_count);

    // // create transfer requests for the other token types
    // let mut transaction_ids = vec![];

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC721,
    //     test_settings.erc721_transfer,
    // ));
    // debug!("transaction_erc721_transfer has been created");

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC3525,
    //     test_settings.erc3525_transfer_1,
    // ));
    // debug!("transaction_erc3525_transfer_1 has been created");

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC3525,
    //     test_settings.erc3525_transfer_2,
    // ));
    // debug!("transaction_erc3525_transfer_2 has been created");

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_transfer_1,
    // ));
    // debug!("transaction_erc1155_transfer_1 has been created");

    // transaction_ids.push(create_nf3_transfer_transaction(
    //     zkp_key2,
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_transfer_2_nft,
    // ));
    // debug!("transaction_erc1155_transfer_2 has been created");

    // // throw all the transactions at the client as fast as we can
    // let transaction_ids = try_join_all(transaction_ids).await.unwrap();

    // // wait for the responses to the transfer requests to come back to the webhook server
    // let transactions = wait_for_all_responses(&transaction_ids, responses.clone())
    //     .await
    //     .into_iter()
    //     .map(|(_, l)| {
    //         serde_json::from_str::<(Value, Option<TransactionReceipt>)>(&l)
    //             .expect("Failed to parse response")
    //     })
    //     .map(|l| l.0)
    //     .collect::<Vec<_>>();

    // // compute the commmitments for the transactions
    // let commitment_hashes = transactions
    //     .iter()
    //     .map(|l| Fr254::from_hex_string(l["commitments"][0].as_str().unwrap()).unwrap())
    //     .collect::<Vec<_>>();

    // wait_on_chain(&commitment_hashes, "http://client2:3000")
    //     .await
    //     .unwrap();
    // info!("Transfer commitments are now on-chain");

    // // check that the new balances are as expected
    // let balance = get_erc20_balance(
    //     &http_client,
    //     Url::parse(&settings.nightfall_client.url).unwrap(),
    // )
    // .await;
    // info!(
    //     "Balance of ERC20 tokens held as layer 2 commitments by client 1: {}",
    //     balance
    // );

    // assert_eq!(balance, 1 + client1_starting_balance);

    // let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    // info!(
    //     "Balance of ERC20 tokens held as layer 2 commitments by client2: {}",
    //     balance
    // );
    // assert_eq!(balance, 20 + client2_starting_balance);

    // // create withdraw requests
    // let mut withdraw_data = vec![];

    // info!("Sending withdraw transactions");
    // let url = Url::parse("http://client2:3000")
    //     .unwrap()
    //     .join("v1/withdraw")
    //     .unwrap();
    // // compute the recipient address from the signing key (we will reuse the deployer key here to withdraw it to ourselves)
    // let recipient_address = get_recipient_address(&settings).unwrap();

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_withdraw_0,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc20_withdraw_0 has been created");

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_withdraw_1,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc20_withdraw_1 has been created");

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC20,
    //     test_settings.erc20_withdraw_2,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc20_withdraw_2 has been created");

    // // throw all the transactions at the client as fast as we can
    // let mut withdraw_data = try_join_all(withdraw_data).await.unwrap();
    // // sort by Uuid
    // withdraw_data.sort_by_key(|(uuid, _)| *uuid);

    // // create a vector of withdraw ids to wait for responses
    // let withdraw_ids = withdraw_data
    //     .iter()
    //     .map(|(uuid, _)| *uuid)
    //     .collect::<Vec<_>>();

    // // wait for the responses to the withdraw requests to come back to the webhook server
    // let withdraw_responses = wait_for_all_responses(&withdraw_ids, responses.clone()).await;

    // // convert the withdraw_responses into a vector of (Uuid, WithdrawResponse)
    // let withdraw_responses = withdraw_responses
    //     .into_iter()
    //     .map(|(u, l)| {
    //         (
    //             u,
    //             serde_json::from_str::<WithdrawResponse>(&l).expect("Failed to parse response"),
    //         )
    //     })
    //     .collect::<Vec<_>>();

    // // we should have the same set of Uuids in the withdraw_responses as in the withdraw_data and they should be in the same order
    // for (i, response) in withdraw_responses.iter().enumerate() {
    //     assert_eq!(
    //         response.0, withdraw_data[i].0,
    //         "{}th Withdraw response Uuid does not match withdraw data Uuid",
    //         i
    //     );
    // }

    // //replace the empty withdraw_fund_salts in the withdraw_data with the salts from the withdraw_responses
    // for (i, response) in withdraw_responses.iter().enumerate() {
    //     withdraw_data[i].1.withdraw_fund_salt = response.1.withdraw_fund_salt.clone();
    // }

    // // convert the withdraw_data into DeEscrowDataReq
    // let de_escrow_data_requests = withdraw_data
    //     .into_iter()
    //     .map(|(_, l)| DeEscrowDataReq {
    //         token_id: l.token_id,
    //         erc_address: l.erc_address,
    //         recipient_address: l.recipient_address,
    //         value: l.value,
    //         token_type: l.token_type,
    //         withdraw_fund_salt: l.withdraw_fund_salt,
    //     })
    //     .collect::<Vec<_>>();

    // // Wait until all ERC20 funds are available to withdraw
    // info!("Waiting for ERC20 funds to be available for withdrawal");
    // for request in &de_escrow_data_requests {
    //     while !de_escrow_request(request, "http://client2:3000")
    //         .await
    //         .unwrap()
    //     {
    //         info!("Not yet able to withdraw funds {:?}", request);
    //         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //     }
    // }
    // info!("Successfully withdrew ERC20 tokens");

    // //check the balance of the ERC20 tokens after the withdraws
    // let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    // info!(
    //     "Balance of ERC20 tokens held as layer 2 commitments by client2: {}",
    //     balance
    // );
    // assert_eq!(balance, 17 + client2_starting_balance);

    // // withdraw the other token types
    // let mut withdraw_data = vec![];

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC721,
    //     test_settings.erc721_withdraw,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc721_withdraw has been created");

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC3525,
    //     test_settings.erc3525_withdraw,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc3525_withdraw has been created");

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_withdraw_1,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc1155_withdraw_1 has been created");

    // withdraw_data.push(create_nf3_withdraw_transaction(
    //     &http_client,
    //     url.clone(),
    //     TokenType::ERC1155,
    //     test_settings.erc1155_withdraw_2_nft,
    //     recipient_address.clone(),
    // ));
    // debug!("transaction_erc1155_withdraw_2 has been created");

    // // throw all the transactions at the client as fast as we can
    // let mut withdraw_data = try_join_all(withdraw_data).await.unwrap();
    // // sort by Uuid
    // withdraw_data.sort_by_key(|(uuid, _)| *uuid);

    // // create a vector of withdraw ids to wait for responses
    // let withdraw_ids = withdraw_data
    //     .iter()
    //     .map(|(uuid, _)| *uuid)
    //     .collect::<Vec<_>>();

    // // wait for the responses to the withdraw requests to come back to the webhook server
    // let withdraw_responses = wait_for_all_responses(&withdraw_ids, responses.clone()).await;

    // // convert the withdraw_responses into a vector of (Uuid, WithdrawResponse)
    // let withdraw_responses = withdraw_responses
    //     .into_iter()
    //     .map(|(u, l)| {
    //         (
    //             u,
    //             serde_json::from_str::<WithdrawResponse>(&l).expect("Failed to parse response"),
    //         )
    //     })
    //     .collect::<Vec<_>>();

    // // we should have the same set of Uuids in the withdraw_responses as in the withdraw_data and they should be in the same order
    // for (i, response) in withdraw_responses.iter().enumerate() {
    //     assert_eq!(
    //         response.0, withdraw_data[i].0,
    //         "{}th Withdraw response Uuid does not match withdraw data Uuid",
    //         i
    //     );
    // }

    // //replace the empty withdraw_fund_salts in the withdraw_data with the salts from the withdraw_responses
    // for (i, response) in withdraw_responses.iter().enumerate() {
    //     withdraw_data[i].1.withdraw_fund_salt = response.1.withdraw_fund_salt.clone();
    // }

    // // convert the withdraw_data into DeEscrowDataReq
    // let de_escrow_data_requests = withdraw_data
    //     .into_iter()
    //     .map(|(_, l)| DeEscrowDataReq {
    //         token_id: l.token_id,
    //         erc_address: l.erc_address,
    //         recipient_address: l.recipient_address,
    //         value: l.value,
    //         token_type: l.token_type,
    //         withdraw_fund_salt: l.withdraw_fund_salt,
    //     })
    //     .collect::<Vec<_>>();

    // // Wait until all ERC721, ERC3525 and ERC1155 funds are available to withdraw
    // info!("Waiting for ERC721, ERC3525 and ERC1155 funds to be available for withdrawal");
    // for request in &de_escrow_data_requests {
    //     while !de_escrow_request(request, "http://client2:3000")
    //         .await
    //         .unwrap()
    //     {
    //         info!("Not yet able to withdraw funds {:?}", request);
    //         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //     }
    // }
    // info!("Successfully withdrew other tokens");

    // // get the final balance of all the addresses used. As these are all addresses funded by Anvil,
    // // we can simple print those balances
    // let client = get_blockchain_client_connection()
    //     .await
    //     .read()
    //     .await
    //     .get_client();
    // let accounts = client.get_accounts().await.unwrap();
    // let initial_balance = U256::from(parse_units("10000.0", "ether").unwrap());
    // let final_balances = try_join_all(
    //     accounts
    //         .iter()
    //         .map(|a| client.get_balance(*a, None))
    //         .collect::<Vec<_>>(),
    // )
    // .await
    // .unwrap()
    // .iter()
    // .map(|b| initial_balance - b)
    // .collect::<Vec<_>>();
    // let final_balances_str = final_balances
    //     .iter()
    //     .map(|b| format_units(*b, "ether").unwrap())
    //     .collect::<Vec<_>>();
    // let total = final_balances.iter().fold(U256::zero(), |acc, b| acc + b);
    // info!("Eth spent was {:#?}", final_balances_str);
    // info!(
    //     "Total spent was {:#?}",
    //     format_units(total, "ether").unwrap()
    // );
}
