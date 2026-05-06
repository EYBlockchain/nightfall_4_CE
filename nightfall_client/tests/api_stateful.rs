use alloy::primitives::{Address, I256};
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use lib::{
    error::NightfallContractError,
    hex_conversion::HexConvertible,
    nf_token_id::to_nf_token_id_from_str,
    plonk_prover::plonk_proof::PlonkProof,
    shared_entities::{Preimage, TokenType, WithdrawData},
    tests_utils::{get_db_connection_uri, get_mongo},
};
use mongodb::bson::doc;
use nightfall_bindings::artifacts::Nightfall;
use nightfall_client::{
    domain::entities::{CommitmentStatus, RequestStatus, SlotData, TokenData},
    driven::db::mongo::{CommitmentEntry, DB},
    drivers::rest::routes,
    initialisation::get_db_connection,
    ports::{
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestDB},
    },
};
use testcontainers::{ContainerAsync, GenericImage};
use uuid::Uuid;

struct MockNightfall;

impl NightfallContract for MockNightfall {
    async fn escrow_funds(
        _token_erc_address: Fr254,
        _value: Fr254,
        _token_id: BigInteger256,
        _fee: Fr254,
        _deposit_fee: Fr254,
        _secret_preimage: lib::shared_entities::DepositSecret,
        _token_type: TokenType,
    ) -> Result<[Fr254; 2], NightfallContractError> {
        panic!("escrow_funds should not be called in api_stateful test")
    }

    fn get_address() -> Fr254 {
        Fr254::from(1u64)
    }

    async fn de_escrow_funds(
        _withdraw_data: WithdrawData,
        _token_type: TokenType,
    ) -> Result<(), NightfallContractError> {
        panic!("de_escrow_funds should not be called in api_stateful test")
    }

    async fn withdraw_available(
        _withdraw_data: WithdrawData,
    ) -> Result<bool, NightfallContractError> {
        panic!("withdraw_available should not be called in api_stateful test")
    }

    async fn get_current_layer2_blocknumber() -> Result<I256, NightfallContractError> {
        Ok(I256::ZERO)
    }

    async fn get_token_info(_nf_token_id: Fr254) -> Result<TokenData, NightfallContractError> {
        panic!("get_token_info should not be called in api_stateful test")
    }

    async fn get_slot_info(_nf_slot_id: Fr254) -> Result<SlotData, NightfallContractError> {
        panic!("get_slot_info should not be called in api_stateful test")
    }

    async fn get_layer2_block_by_number(
        _block_number: I256,
    ) -> Result<(Address, Nightfall::Block), NightfallContractError> {
        panic!("get_layer2_block_by_number should not be called in api_stateful test")
    }
}

async fn setup_test_db() -> ContainerAsync<GenericImage> {
    let container = get_mongo().await;
    let host = container.get_host().await.expect("mongo host");
    let port = container
        .get_host_port_ipv4(27017)
        .await
        .expect("mongo port");
    let db_url = get_db_connection_uri(host, port);

    unsafe {
        std::env::set_var("NF4_RUN_MODE", "development");
        std::env::set_var("NF4_NIGHTFALL_CLIENT__DB_URL", &db_url);
    }

    let db = get_db_connection().await;
    db.database(DB)
        .collection::<mongodb::bson::Document>("requests")
        .delete_many(doc! {})
        .await
        .expect("clear requests");
    db.database(DB)
        .collection::<mongodb::bson::Document>("commitments")
        .delete_many(doc! {})
        .await
        .expect("clear commitments");

    container
}

#[tokio::test]
async fn test_request_status_and_balance_routes_cover_stateful_sanity_cases() {
    let _container = setup_test_db().await;
    let db = get_db_connection().await;
    let filter = routes::<PlonkProof, MockNightfall>();

    let unknown_uuid = Uuid::new_v4().to_string();
    let unknown_request = warp::test::request()
        .method("GET")
        .path(&format!("/v1/request/{unknown_uuid}"))
        .reply(&filter)
        .await;
    assert_eq!(unknown_request.status(), reqwest::StatusCode::NOT_FOUND);
    assert_eq!(
        std::str::from_utf8(unknown_request.body()).unwrap(),
        "No such request"
    );

    let request_id = Uuid::new_v4().to_string();
    db.store_request(&request_id, RequestStatus::Queued)
        .await
        .expect("store request");

    for expected_status in [
        RequestStatus::Queued,
        RequestStatus::Submitted,
        RequestStatus::Failed,
    ] {
        db.update_request(&request_id, expected_status)
            .await
            .expect("update request");

        let response = warp::test::request()
            .method("GET")
            .path(&format!("/v1/request/{request_id}"))
            .reply(&filter)
            .await;

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body = serde_json::from_slice::<serde_json::Value>(response.body())
            .expect("request body should be JSON");
        assert_eq!(body["uuid"], request_id);
        assert_eq!(body["status"], expected_status.to_string());
    }

    let empty_commitments = warp::test::request()
        .method("GET")
        .path("/v1/commitments")
        .reply(&filter)
        .await;
    assert_eq!(empty_commitments.status(), reqwest::StatusCode::OK);
    let empty_commitments_body =
        serde_json::from_slice::<Vec<serde_json::Value>>(empty_commitments.body())
            .expect("commitments body should be JSON");
    assert!(empty_commitments_body.is_empty());

    let erc_address = "0x1111111111111111111111111111111111111111";
    let erc721_token_id = "0x01";

    let empty_wallet = warp::test::request()
        .method("GET")
        .path(&format!("/v1/balance/{erc_address}/{erc721_token_id}"))
        .reply(&filter)
        .await;
    assert_eq!(empty_wallet.status(), reqwest::StatusCode::NOT_FOUND);
    assert_eq!(
        std::str::from_utf8(empty_wallet.body()).unwrap(),
        "No such token"
    );

    let nf_token_id =
        to_nf_token_id_from_str(erc_address, erc721_token_id).expect("valid nf token id");
    db.store_commitment(CommitmentEntry::new(
        Preimage {
            value: Fr254::from(0u64),
            nf_token_id,
            ..Default::default()
        },
        Fr254::default(),
        CommitmentStatus::Unspent,
        TokenType::ERC721,
        None,
        None,
    ))
    .await
    .expect("store ERC721 commitment");

    let erc721_balance = warp::test::request()
        .method("GET")
        .path(&format!("/v1/balance/{erc_address}/{erc721_token_id}"))
        .reply(&filter)
        .await;
    assert_eq!(erc721_balance.status(), reqwest::StatusCode::OK);
    assert_eq!(std::str::from_utf8(erc721_balance.body()).unwrap(), "00");

    let erc20_address = "0x2222222222222222222222222222222222222222";
    let erc20_token_id = "0x00";
    let erc20_nf_token_id =
        to_nf_token_id_from_str(erc20_address, erc20_token_id).expect("valid ERC20 nf token id");

    let deposited_commitment = CommitmentEntry::new(
        Preimage {
            value: Fr254::from(10u64),
            nf_token_id: erc20_nf_token_id,
            ..Default::default()
        },
        Fr254::from(101u64),
        CommitmentStatus::Unspent,
        TokenType::ERC20,
        None,
        None,
    );
    let deposited_nullifier = deposited_commitment.nullifier;
    db.store_commitment(deposited_commitment)
        .await
        .expect("store deposited ERC20 commitment");

    let post_deposit_balance = warp::test::request()
        .method("GET")
        .path(&format!("/v1/balance/{erc20_address}/{erc20_token_id}"))
        .reply(&filter)
        .await;
    assert_eq!(post_deposit_balance.status(), reqwest::StatusCode::OK);
    assert_eq!(
        Fr254::from_hex_string(std::str::from_utf8(post_deposit_balance.body()).unwrap())
            .expect("deposit balance should be valid hex"),
        Fr254::from(10u64)
    );

    db.mark_commitments_spent(vec![deposited_nullifier])
        .await
        .expect("mark deposited commitment spent after transfer");
    db.store_commitment(CommitmentEntry::new(
        Preimage {
            value: Fr254::from(6u64),
            nf_token_id: erc20_nf_token_id,
            ..Default::default()
        },
        Fr254::from(202u64),
        CommitmentStatus::Unspent,
        TokenType::ERC20,
        None,
        None,
    ))
    .await
    .expect("store ERC20 change commitment after transfer");

    let post_transfer_balance = warp::test::request()
        .method("GET")
        .path(&format!("/v1/balance/{erc20_address}/{erc20_token_id}"))
        .reply(&filter)
        .await;
    assert_eq!(post_transfer_balance.status(), reqwest::StatusCode::OK);
    assert_eq!(
        Fr254::from_hex_string(std::str::from_utf8(post_transfer_balance.body()).unwrap())
            .expect("post-transfer balance should be valid hex"),
        Fr254::from(6u64)
    );

    db.mark_commitments_spent(vec![Fr254::from(202u64)])
        .await
        .expect("mark change commitment spent after withdraw");

    let post_withdraw_balance = warp::test::request()
        .method("GET")
        .path(&format!("/v1/balance/{erc20_address}/{erc20_token_id}"))
        .reply(&filter)
        .await;
    assert_eq!(
        post_withdraw_balance.status(),
        reqwest::StatusCode::NOT_FOUND
    );
    assert_eq!(
        std::str::from_utf8(post_withdraw_balance.body()).unwrap(),
        "No such token"
    );

    let all_commitments = warp::test::request()
        .method("GET")
        .path("/v1/commitments")
        .reply(&filter)
        .await;
    assert_eq!(all_commitments.status(), reqwest::StatusCode::OK);
    let all_commitments_body =
        serde_json::from_slice::<Vec<serde_json::Value>>(all_commitments.body())
            .expect("all commitments body should be JSON");
    assert_eq!(all_commitments_body.len(), 3);
}
