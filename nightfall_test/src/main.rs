use ark_bn254::Fr as Fr254;
use configuration::{
    logging::init_logging,
    settings::{get_settings, Settings},
};
use lib::models::CertificateReq;
use log::{debug, info, warn};
use nightfall_client::domain::entities::HexConvertible;
use std::fs;
use test::{count_spent_commitments, get_erc20_balance, get_erc721_balance, get_fee_balance};

use lib::{
    blockchain_client::BlockchainClientConnection, initialisation::get_blockchain_client_connection,
};

use crate::{
    test::{
        create_nf3_deposit_transaction, create_nf3_transfer_transaction,
        create_nf3_withdraw_transaction, de_escrow_request, forge_command, get_key,
        get_recipient_address, load_addresses, validate_certificate_with_server, wait_on_chain,
        TokenType,
    },
    test_settings::TestSettings,
};
use ark_std::Zero;
use url::Url;

/// This module contains the main function that runs the tests
pub mod test;
/// This module contains the test settings
pub mod test_settings;

use ethers::{
    providers::Middleware,
    types::U256,
    utils::{format_units, parse_units},
};
use futures::future::try_join_all;

#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().unwrap();
    let test_settings: TestSettings = TestSettings::new().unwrap();
    init_logging(
        settings.nightfall_test.log_level.as_str(),
        settings.log_app_only,
    );
    info!("Running tests on nightall_client http:// interface");

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

    forge_command(&["install"]);
    forge_command(&[
        "script",
        "MockDeployer",
        "--fork-url",
        &settings.ethereum_client_url,
        "--broadcast",
        "--force",
    ]);

    let _ = load_addresses(&settings).unwrap();
    info!("* contract addresses obtained");

    // Validate the certificate with the server before proceeding
    info!("Validating Client's certificate");
    let http_client = reqwest::Client::new();
    let cert_url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("/v1/certification")
        .unwrap();

    let client_cert =
        fs::read("../blockchain_assets/test_contracts/X509/_certificates/user/user-1.der")
            .expect("Failed to read user-1 certificate");
    let client_cert_private_key =
        fs::read("../blockchain_assets/test_contracts/X509/_certificates/user/user-1.priv_key")
            .expect("Failed to read user priv_key");

    let client_certificate_req = CertificateReq {
        certificate: client_cert,
        certificate_private_key: client_cert_private_key,
    };
    validate_certificate_with_server(&http_client, cert_url, client_certificate_req.clone())
        .await
        .expect("Certificate validation failed");

    info!("Validating Client Two's certificate");
    let cert_url = Url::parse("http://client2:3000")
        .unwrap()
        .join("/v1/certification")
        .unwrap();
    validate_certificate_with_server(&http_client, cert_url, client_certificate_req)
        .await
        .expect("Certificate validation failed");

    info!("Validating Proposer's certificate");
    //let http_client = reqwest::Client::new();
    let cert_url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("/v1/certification")
        .unwrap();
    let proposer_cert =
        fs::read("../blockchain_assets/test_contracts/X509/_certificates/user/user-2.der")
            .expect("Failed to read proposer's certificate");
    let proposer_cert_private_key =
        fs::read("../blockchain_assets/test_contracts/X509/_certificates/user/user-2.priv_key")
            .expect("Failed to read proposer's certificate priv_key");
    let proposer_certificate_req = CertificateReq {
        certificate: proposer_cert,
        certificate_private_key: proposer_cert_private_key,
    };
    validate_certificate_with_server(&http_client, cert_url, proposer_certificate_req)
        .await
        .expect("Certificate validation failed");

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
        const N_LARGE_BLOCK: usize = 1;
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
        // // first we need to pause block assembly so that we can make all the deposits in the same block
        let pause_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/pause")
            .unwrap();
        let res = http_client.get(pause_url).send().await.unwrap();
        assert!(res.status().is_success());

        // create deposit commitments first
        let url = Url::parse(&settings.nightfall_client.url)
            .unwrap()
            .join("v1/deposit")
            .unwrap();
        let mut large_block_deposits = vec![];
        for _ in 0..N_LARGE_BLOCK * 2 {
            let large_block_deposit = create_nf3_deposit_transaction(
                &http_client,
                url.clone(),
                TokenType::ERC20,
                test_settings.erc20_deposit_large_block.clone(),
                DEPOSIT_FEE.to_string(), //deposit_fee
            )
            .await
            .unwrap();

            large_block_deposits.push(Fr254::from_hex_string(&large_block_deposit.0).unwrap());
            large_block_deposits
                .push(Fr254::from_hex_string(&large_block_deposit.1.clone().unwrap()).unwrap());
        }
        //now we can resume block assembly
        let resume_url = Url::parse(&settings.nightfall_proposer.url)
            .unwrap()
            .join("v1/resume")
            .unwrap();
        let res = http_client.get(resume_url).send().await.unwrap();
        assert!(res.status().is_success());

        wait_on_chain(&large_block_deposits, &get_settings().nightfall_client.url)
            .await
            .unwrap();

        info!("A large block full of ERC20 Deposits is now on-chain");

        // next, we'll do transfers
        // // but first we need to pause block assembly so that we can make all the transfers in the same block
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
        let mut large_block_transfers = vec![];
        for _ in 0..N_LARGE_BLOCK {
            let large_block_transfer = create_nf3_transfer_transaction(
                zkp_key2,
                &http_client,
                url.clone(),
                TokenType::ERC20,
                test_settings.erc20_transfer_large_block.clone(),
            )
            .await
            .unwrap();
            large_block_transfers.push(large_block_transfer);
        }
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

    // /***********************************************************************************************
    //  * Tests using the client_nf_3 API
    //  **********************************************************************************************/
    // // Test values are carefully chosen so we can test the full range of token types and values, please don't change them. Instead, please add new tests if you need to test new values.
    // // To make the tests more readable and easier to debug, we submit commitments to blockchain everytime when we make requests for a specific token.
    info!("Commencing tests using the client_nf_3 API");
    // // create deposit requests
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("v1/deposit")
        .unwrap();

    let transaction_erc20_deposit_0 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_deposit_0.clone(),
        "0x00".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc20_deposit_0 has been created");

    let transaction_erc20_deposit_1 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_deposit_1,
        "0x27".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc20_deposit_1 has been created");

    let transaction_erc20_deposit_2 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_deposit_2.clone(),
        "0x06".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc20_deposit_2 has been created");

    let transaction_erc20_deposit_3 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_deposit_3,
        "0x00".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc20_deposit_3 has been created");

    // check that we have no 'balance' of the ERC721 token
    // get the balance of the ERC721 token we just deposited
    let balance = get_erc721_balance(
        &http_client,
        Url::parse(&settings.nightfall_client.url).unwrap(),
        test_settings.erc721_deposit.token_id.clone(),
    )
    .await;
    assert_eq!(None, balance);

    let transaction_erc721_deposit = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC721,
        test_settings.erc721_deposit.clone(),
        "0x08".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc721_deposit has been created");

    let transaction_erc3525_deposit_1 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC3525,
        test_settings.erc3525_deposit_1,
        "0x0b".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc3525_deposit_1 has been created");

    let transaction_erc3525_deposit_2 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC3525,
        test_settings.erc3525_deposit_2,
        "0x0e".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc3525_deposit_2 has been created");

    let transaction_erc1155_deposit_1 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_deposit_1,
        "0x11".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_deposit_1 has been created");

    let transaction_erc1155_deposit_2 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_deposit_2,
        "0x14".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_deposit_2 has been created");

    let transaction_erc1155_deposit_3 = create_nf3_deposit_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_deposit_3_nft,
        "0x16".to_string(), //deposit_fee
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_deposit_3 has been created");

    // for each deposit request, we have value commitment and fee commitment (if fee is non-zero)
    // wait for the commitments to appear on-chain - we can't transfer until they are there
    wait_on_chain(
        &[
            Fr254::from_hex_string(&transaction_erc20_deposit_0.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc20_deposit_1.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc20_deposit_1.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc20_deposit_2.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc20_deposit_2.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc20_deposit_3.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc721_deposit.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc721_deposit.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc3525_deposit_1.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc3525_deposit_1.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc3525_deposit_2.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc3525_deposit_2.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_1.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_1.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_2.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_2.1.unwrap()).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_3.0).unwrap(),
            Fr254::from_hex_string(&transaction_erc1155_deposit_3.1.unwrap()).unwrap(),
        ],
        &get_settings().nightfall_client.url,
    )
    .await
    .unwrap();
    info!("Deposit commitments for client 1 are now on-chain");

    // get the balance of the ERC721 token we just deposited
    let balance = get_erc721_balance(
        &http_client,
        Url::parse(&settings.nightfall_client.url).unwrap(),
        test_settings.erc721_deposit.token_id,
    )
    .await;
    assert!(balance.is_some_and(|balance| balance.is_zero()));

    // get the fee balance
    let fee_balance = get_fee_balance(
        &http_client,
        Url::parse(&settings.nightfall_client.url).unwrap(),
    )
    .await;
    info!(
        "Fee Commitment Balance  held as layer 2 commitments by client1: {}",
        fee_balance
    );
    assert_eq!(fee_balance, 137 + client1_starting_fee_balance);

    info!("Making client2 fee commitments so that it can withdraw");
    // give client 2 some deposit fee commitments so that it can transact
    // we need up to seven commitments because we'll want to do up to seven withdraws in
    // the same block (we don't control when a block is computed), so we can't use a single commitment
    // even if it has enough value because the change won't be available until the next block.

    let url2 = Url::parse("http://client2:3000")
        .unwrap()
        .join("v1/deposit")
        .unwrap();
    let mut transactions_erc20_deposit_4: Vec<(String, Option<String>)> = vec![];
    for _ in 0..7 {
        transactions_erc20_deposit_4.push(
            create_nf3_deposit_transaction(
                &http_client,
                url2.clone(),
                TokenType::ERC20,
                test_settings.erc20_deposit_4.clone(),
                "0x20".to_string(), //deposit_fee
            )
            .await
            .unwrap(),
        );
        debug!("transaction_erc20_deposit_4 has been created");
    }

    for transaction in transactions_erc20_deposit_4.iter_mut().take(7) {
        // wait for the client2 fee commitments to appear on-chain
        wait_on_chain(
            &[
                Fr254::from_hex_string(&transaction.0).unwrap(),
                Fr254::from_hex_string(transaction.1.as_mut().unwrap()).unwrap(),
            ],
            "http://client2:3000",
        )
        .await
        .unwrap();
    }
    info!("Client2 ERC20 fee commitments are now on-chain");

    // get the balance of the ERC20 tokens we just deposited
    let balance = get_erc20_balance(
        &http_client,
        Url::parse(&settings.nightfall_client.url).unwrap(),
    )
    .await;
    info!(
        "Balance of ERC20 tokens held as layer 2 commitments by client 1: {}",
        balance
    );
    assert_eq!(balance, 14 + client1_starting_balance);

    let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    info!(
        "Balance of ERC20 tokens held as layer 2 commitments by client 2: {}",
        balance
    );
    assert_eq!(balance, 7 + client2_starting_balance);

    info!("Sending transfer transactions");

    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("v1/transfer")
        .unwrap();
    let transaction_erc20_transfer_0 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_transfer_0,
    )
    .await
    .unwrap();
    let transaction_erc20_transfer_1 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_transfer_1,
    )
    .await
    .unwrap();
    debug!("transaction_erc20_transfer_1 has been created");
    let transaction_erc20_transfer_2 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_transfer_2,
    )
    .await
    .unwrap();
    debug!("transaction_erc20_transfer_2 has been created");

    wait_on_chain(
        &[
            Fr254::from_hex_string(
                transaction_erc20_transfer_0["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc20_transfer_1["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc20_transfer_2["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
        ],
        "http://client2:3000",
    )
    .await
    .unwrap();
    info!("ERC20 Transfer commitments are now on-chain");

    // check that we have nullified the correct number of commitments
    let nullifier_count = transaction_erc20_transfer_0["nullifiers"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| !((Fr254::from_hex_string(n.as_str().unwrap()).unwrap()).is_zero()))
        .count()
        + transaction_erc20_transfer_1["nullifiers"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|n| !((Fr254::from_hex_string(n.as_str().unwrap()).unwrap()).is_zero()))
            .count()
        + transaction_erc20_transfer_2["nullifiers"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|n| !((Fr254::from_hex_string(n.as_str().unwrap()).unwrap()).is_zero()))
            .count()
        + nullified_count;
    info!("Expected spent commitment count: {}", nullifier_count);
    let spent_commitments = count_spent_commitments(&http_client, url.clone())
        .await
        .unwrap();
    assert_eq!(spent_commitments, nullifier_count);

    let transaction_erc721_transfer = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC721,
        test_settings.erc721_transfer,
    )
    .await
    .unwrap();
    debug!("transaction_erc721_transfer has been created");

    let transaction_erc3525_transfer_1 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC3525,
        test_settings.erc3525_transfer_1,
    )
    .await
    .unwrap();
    debug!("transaction_erc3525_transfer_1 has been created");

    let transaction_erc3525_transfer_2 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC3525,
        test_settings.erc3525_transfer_2,
    )
    .await
    .unwrap();
    debug!("transaction_erc3525_transfer_2 has been created");

    let transaction_erc1155_transfer_1 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_transfer_1,
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_transfer_1 has been created");

    let transaction_erc1155_transfer_2 = create_nf3_transfer_transaction(
        zkp_key2,
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_transfer_2_nft,
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_transfer_2 has been created");

    wait_on_chain(
        &[
            Fr254::from_hex_string(
                transaction_erc721_transfer["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc3525_transfer_1["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc3525_transfer_2["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc1155_transfer_1["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
            Fr254::from_hex_string(
                transaction_erc1155_transfer_2["commitments"][0]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
        ],
        "http://client2:3000",
    )
    .await
    .unwrap();
    info!("Transfer commitments are now on-chain");

    // check that the new balances are as expected
    let balance = get_erc20_balance(
        &http_client,
        Url::parse(&settings.nightfall_client.url).unwrap(),
    )
    .await;
    info!(
        "Balance of ERC20 tokens held as layer 2 commitments by client 1: {}",
        balance
    );
    assert_eq!(balance, 1 + client1_starting_balance);

    let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    info!(
        "Balance of ERC20 tokens held as layer 2 commitments by client2: {}",
        balance
    );
    assert_eq!(balance, 20 + client2_starting_balance);

    info!("Sending withdraw transactions");
    let url = Url::parse("http://client2:3000")
        .unwrap()
        .join("v1/withdraw")
        .unwrap();
    // compute the recipient address from the signing key (we will reuse the deployer key here to withdraw it to ourselves)
    let recipient_address = get_recipient_address(&settings).unwrap();
    let transaction_erc20_withdraw_0 = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_withdraw_0,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc20_withdraw_0 has been created");

    let transaction_erc20_withdraw_1 = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_withdraw_1,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc20_withdraw_1 has been created");

    let transaction_erc20_withdraw_2 = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC20,
        test_settings.erc20_withdraw_2,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc20_withdraw_2 has been created");

    // get commitment hashes to act as database keys
    // query the DB for these hashes and wait until we have found them all

    let mut proceed = 0;
    while proceed == 0 {
        let result =
            de_escrow_request(transaction_erc20_withdraw_0.clone(), "http://client2:3000").await;
        match result {
            Ok(b) => {
                if b == 1 {
                    info!("Withdrawing funds");
                } else {
                    info!("Not yet able to withdraw funds");
                }
                proceed = b;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    debug!("erc20_withdraw_0 has been withdrawn");

    proceed = 0;

    while proceed == 0 {
        let result =
            de_escrow_request(transaction_erc20_withdraw_1.clone(), "http://client2:3000").await;
        match result {
            Ok(b) => {
                if b == 1 {
                    info!("Withdrawing funds");
                } else {
                    info!("Not yet able to withdraw funds");
                }
                proceed = b;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    debug!("erc20_withdraw_1 has been withdrawn");

    proceed = 0;

    while proceed == 0 {
        let result =
            de_escrow_request(transaction_erc20_withdraw_2.clone(), "http://client2:3000").await;
        match result {
            Ok(_) => {
                info!("Withdrawing funds");
                proceed = 1;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    proceed = 0;

    info!("Successfully withdrew ERC20 tokens");
    let balance = get_erc20_balance(&http_client, Url::parse("http://client2:3000").unwrap()).await;
    info!(
        "Balance of ERC20 tokens held as layer 2 commitments by client2: {}",
        balance
    );
    assert_eq!(balance, 17 + client2_starting_balance);

    let transaction_erc721_withdraw = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC721,
        test_settings.erc721_withdraw,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc721_withdraw has been created");

    while proceed == 0 {
        let result =
            de_escrow_request(transaction_erc721_withdraw.clone(), "http://client2:3000").await;
        match result {
            Ok(_) => {
                info!("Withdrawing funds");
                proceed = 1;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    proceed = 0;

    info!("Successfully withdrew ERC721 tokens");

    let transaction_erc3525_withdraw = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC3525,
        test_settings.erc3525_withdraw,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc3525_withdraw has been created");

    while proceed == 0 {
        let result =
            de_escrow_request(transaction_erc3525_withdraw.clone(), "http://client2:3000").await;
        match result {
            Ok(_) => {
                info!("Withdrawing funds");
                proceed = 1;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    proceed = 0;

    info!("Successfully withdrew ERC3525 token");

    let transaction_erc1155_withdraw_1 = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_withdraw_1,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_withdraw_1 has been created");

    let transaction_erc1155_withdraw_2 = create_nf3_withdraw_transaction(
        &http_client,
        url.clone(),
        TokenType::ERC1155,
        test_settings.erc1155_withdraw_2_nft,
        recipient_address.clone(),
    )
    .await
    .unwrap();
    debug!("transaction_erc1155_withdraw_2 has been created");

    while proceed == 0 {
        let result = de_escrow_request(
            transaction_erc1155_withdraw_1.clone(),
            "http://client2:3000",
        )
        .await;
        match result {
            Ok(_) => {
                info!("Withdrawing funds");
                proceed = 1;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    proceed = 0;
    while proceed == 0 {
        let result = de_escrow_request(
            transaction_erc1155_withdraw_2.clone(),
            "http://client2:3000",
        )
        .await;
        match result {
            Ok(_) => {
                info!("Withdrawing funds");
                proceed = 1;
            }
            Err(_) => {
                info!("Could not yet withdraw funds");
                proceed = 0;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    info!("Successfully withdrew ERC1155 tokens");

    // get the final balance of all the addresses used. As these are all addresses funded by Anvil,
    // we can simple print those balances
    let client = get_blockchain_client_connection()
        .await
        .read()
        .await
        .get_client();
    let accounts = client.get_accounts().await.unwrap();
    let initial_balance = U256::from(parse_units("10000.0", "ether").unwrap());
    let final_balances = try_join_all(
        accounts
            .iter()
            .map(|a| client.get_balance(*a, None))
            .collect::<Vec<_>>(),
    )
    .await
    .unwrap()
    .iter()
    .map(|b| initial_balance - b)
    .collect::<Vec<_>>();
    let final_balances_str = final_balances
        .iter()
        .map(|b| format_units(*b, "ether").unwrap())
        .collect::<Vec<_>>();
    let total = final_balances.iter().fold(U256::zero(), |acc, b| acc + b);
    info!("Eth spent was {:#?}", final_balances_str);
    info!(
        "Total spent was {:#?}",
        format_units(total, "ether").unwrap()
    );
}
