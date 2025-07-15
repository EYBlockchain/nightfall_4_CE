use configuration::settings::Settings;
use ethers::types::Address;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use nightfall_client::drivers::rest::models::KeyRequest;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use crate::test::TransactionDetails;

// rather than pass around what are effectively constant values, or recreate them locally,
// let's use the lazy_static crate to create a global variable that can be used to consume
// settings from anywhere in the code.
pub fn get_test_settings() -> &'static TestSettings {
    static SETTINGS: OnceLock<TestSettings> = OnceLock::new();
    SETTINGS.get_or_init(|| TestSettings::new().unwrap())
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DepositValues {
    pub path: String,
    pub value: String,
    pub fee: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TransferValues {
    pub path: String,
    pub value: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WithdrawValues {
    pub path: String,
    pub value: String,
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MockAddresses {
    pub erc20: Address,
    pub erc721: Address,
    pub erc1155: Address,
    pub erc3525: Address,
}

#[derive(serde::Deserialize)]
pub struct TestSettings {
    pub key_request: KeyRequest,
    pub key_request2: KeyRequest,
    pub erc20_deposit_0: TransactionDetails,
    pub erc20_deposit_1: TransactionDetails,
    pub erc20_deposit_2: TransactionDetails,
    pub erc20_deposit_3: TransactionDetails,
    pub erc20_deposit_4: TransactionDetails,
    pub erc20_deposit_large_block: TransactionDetails,
    pub erc20_transfer_0: TransactionDetails,
    pub erc20_transfer_1: TransactionDetails,
    pub erc20_transfer_2: TransactionDetails,
    pub erc20_transfer_large_block: TransactionDetails,
    pub erc20_withdraw_0: TransactionDetails,
    pub erc20_withdraw_1: TransactionDetails,
    pub erc20_withdraw_2: TransactionDetails,
    pub erc721_deposit: TransactionDetails,
    pub erc721_transfer: TransactionDetails,
    pub erc721_withdraw: TransactionDetails,
    pub erc3525_deposit_1: TransactionDetails,
    pub erc3525_deposit_2: TransactionDetails,
    pub erc3525_transfer_1: TransactionDetails,
    pub erc3525_transfer_2: TransactionDetails,
    pub erc3525_withdraw: TransactionDetails,
    pub erc1155_deposit_1: TransactionDetails,
    pub erc1155_deposit_2: TransactionDetails,
    pub erc1155_deposit_3_nft: TransactionDetails,
    pub erc1155_transfer_1: TransactionDetails,
    pub erc1155_transfer_2_nft: TransactionDetails,
    pub erc1155_withdraw_1: TransactionDetails,
    pub erc1155_withdraw_2_nft: TransactionDetails,
}
impl TestSettings {
    pub fn new() -> Result<Self, String> {
        let test_settings: TestSettings = Figment::new()
            .merge(Toml::file("nightfall_test.toml").nested())
            .extract()
            .map_err(|e| format!("{}", e))?;

        Ok(test_settings)
    }

    pub fn retrieve_mock_addresses() -> MockAddresses {
        fn find(path: &Path) -> Option<PathBuf> {
            if path.is_absolute() {
                match path.is_file() {
                    true => return Some(path.to_path_buf()),
                    false => return None,
                }
            }

            let cwd = std::env::current_dir().ok()?;
            let mut cwd = cwd.as_path();
            loop {
                let file_path = cwd.join(path);
                if file_path.is_file() {
                    return Some(file_path);
                }

                cwd = cwd.parent()?;
            }
        }

        let json_path = find(
            &Path::new("blockchain_assets/logs/mock_deployment.s.sol")
                .join(Settings::new().unwrap().network.chain_id.to_string())
                .join("run-latest.json"),
        )
        .unwrap();
        let mut json_file = File::open(json_path).unwrap();
        let mut json_string = String::new();
        json_file.read_to_string(&mut json_string).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let mut erc20 = Address::zero();
        let mut erc721 = Address::zero();
        let mut erc1155 = Address::zero();
        let mut erc3525 = Address::zero();
        let transaction_array = v["transactions"].as_array().unwrap();

        for transaction in transaction_array {
            match transaction["contractName"].as_str().unwrap() {
                "ERC20Mock" => {
                    let bytes: [u8; 20] = hex::decode(
                        transaction["contractAddress"]
                            .as_str()
                            .unwrap()
                            .trim_start_matches("0x"),
                    )
                    .unwrap()
                    .try_into()
                    .unwrap();
                    erc20 = Address::from(bytes);
                }
                "ERC721Mock" => {
                    let bytes: [u8; 20] = hex::decode(
                        transaction["contractAddress"]
                            .as_str()
                            .unwrap()
                            .trim_start_matches("0x"),
                    )
                    .unwrap()
                    .try_into()
                    .unwrap();
                    erc721 = Address::from(bytes);
                }
                "ERC1155Mock" => {
                    let bytes: [u8; 20] = hex::decode(
                        transaction["contractAddress"]
                            .as_str()
                            .unwrap()
                            .trim_start_matches("0x"),
                    )
                    .unwrap()
                    .try_into()
                    .unwrap();
                    erc1155 = Address::from(bytes);
                }
                "ERC3525Mock" => {
                    let bytes: [u8; 20] = hex::decode(
                        transaction["contractAddress"]
                            .as_str()
                            .unwrap()
                            .trim_start_matches("0x"),
                    )
                    .unwrap()
                    .try_into()
                    .unwrap();
                    erc3525 = Address::from(bytes);
                }
                _ => continue,
            }
        }
        MockAddresses {
            erc20,
            erc721,
            erc1155,
            erc3525,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::forge_command;
    use ethers::{
        providers::{Http, Middleware, Provider},
        utils::Anvil,
    };
    // use nightfall_bindings::{
    //     erc1155_mock::ERC1155MOCK_DEPLOYED_BYTECODE, erc20_mock::ERC20MOCK_DEPLOYED_BYTECODE,
    //     erc3525_mock::ERC3525MOCK_DEPLOYED_BYTECODE, erc721_mock::ERC721MOCK_DEPLOYED_BYTECODE,
    // };
    #[tokio::test]
    async fn test_mock_addresses() {
        // fire up a blockchain simulator
        let mut settings = configuration::settings::Settings::new().unwrap();
        settings.ethereum_client_url = "ws://localhost:8545".to_string(); // we're running bare metal so a docker url won't work
        std::env::set_var(
            "NF4_SIGNING_KEY",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        );
        let url = url::Url::parse(&settings.ethereum_client_url).unwrap();
        let anvil = Anvil::new().port(url.port().unwrap()).spawn();
        forge_command(&[
            "script",
            "MockDeployer",
            "--fork-url",
            &settings.ethereum_client_url,
            "--broadcast",
            "--force",
        ]);
        let mock_addresses = TestSettings::retrieve_mock_addresses();

        // get a blockchain provider so we can interrogate the deployed code
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
        let erc20_code = provider.get_code(mock_addresses.erc20, None).await.unwrap();
        let erc721_code = provider
            .get_code(mock_addresses.erc721, None)
            .await
            .unwrap();
        let erc1155_code = provider
            .get_code(mock_addresses.erc1155, None)
            .await
            .unwrap();
        let erc3525_code = provider
            .get_code(mock_addresses.erc3525, None)
            .await
            .unwrap();
        assert_eq!(erc20_code, ERC20MOCK_DEPLOYED_BYTECODE);
        assert_eq!(erc721_code, ERC721MOCK_DEPLOYED_BYTECODE);
        assert_eq!(erc1155_code, ERC1155MOCK_DEPLOYED_BYTECODE);
        assert_eq!(erc3525_code, ERC3525MOCK_DEPLOYED_BYTECODE);
    }
}
