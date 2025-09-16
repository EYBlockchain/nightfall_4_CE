///`Block(uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Block {
    pub commitments_root: ::ethers::core::types::U256,
    pub nullifier_root: ::ethers::core::types::U256,
    pub commitments_root_root: ::ethers::core::types::U256,
    pub transactions: ::std::vec::Vec<OnChainTransaction>,
    pub rollup_proof: ::ethers::core::types::Bytes,
}
///`DecodedTlv(uint256,uint256,(bool,bytes1),uint256,bytes,bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DecodedTlv {
    pub start: ::ethers::core::types::U256,
    pub header_length: ::ethers::core::types::U256,
    pub tag: Tag,
    pub length: ::ethers::core::types::U256,
    pub value: ::ethers::core::types::Bytes,
    pub octets: ::ethers::core::types::Bytes,
    pub depth: ::ethers::core::types::U256,
}
///`Tag(bool,bytes1)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Tag {
    pub is_constructed: bool,
    pub tag_type: [u8; 1],
}
///`OnChainTransaction(uint256,uint256[4],uint256[4],uint256[4])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OnChainTransaction {
    pub fee: ::ethers::core::types::U256,
    pub commitments: [::ethers::core::types::U256; 4],
    pub nullifiers: [::ethers::core::types::U256; 4],
    pub public_data: [::ethers::core::types::U256; 4],
}
///`Proposer(uint256,address,string,address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Proposer {
    pub stake: ::ethers::core::types::U256,
    pub addr: ::ethers::core::types::Address,
    pub url: ::std::string::String,
    pub next_addr: ::ethers::core::types::Address,
    pub previous_addr: ::ethers::core::types::Address,
}
///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G1Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`G2Point(uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G2Point {
    pub x_0: ::ethers::core::types::U256,
    pub x_1: ::ethers::core::types::U256,
    pub y_0: ::ethers::core::types::U256,
    pub y_1: ::ethers::core::types::U256,
}
///`VerificationKey(uint256,uint256,(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,(uint256,uint256),(uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct VerificationKey {
    pub domain_size: ::ethers::core::types::U256,
    pub num_inputs: ::ethers::core::types::U256,
    pub sigma_comms_1: G1Point,
    pub sigma_comms_2: G1Point,
    pub sigma_comms_3: G1Point,
    pub sigma_comms_4: G1Point,
    pub sigma_comms_5: G1Point,
    pub sigma_comms_6: G1Point,
    pub selector_comms_1: G1Point,
    pub selector_comms_2: G1Point,
    pub selector_comms_3: G1Point,
    pub selector_comms_4: G1Point,
    pub selector_comms_5: G1Point,
    pub selector_comms_6: G1Point,
    pub selector_comms_7: G1Point,
    pub selector_comms_8: G1Point,
    pub selector_comms_9: G1Point,
    pub selector_comms_10: G1Point,
    pub selector_comms_11: G1Point,
    pub selector_comms_12: G1Point,
    pub selector_comms_13: G1Point,
    pub selector_comms_14: G1Point,
    pub selector_comms_15: G1Point,
    pub selector_comms_16: G1Point,
    pub selector_comms_17: G1Point,
    pub selector_comms_18: G1Point,
    pub k_1: ::ethers::core::types::U256,
    pub k_2: ::ethers::core::types::U256,
    pub k_3: ::ethers::core::types::U256,
    pub k_4: ::ethers::core::types::U256,
    pub k_5: ::ethers::core::types::U256,
    pub k_6: ::ethers::core::types::U256,
    pub range_table_comm: G1Point,
    pub key_table_comm: G1Point,
    pub table_dom_sep_comm: G1Point,
    pub q_dom_sep_comm: G1Point,
    pub size_inv: ::ethers::core::types::U256,
    pub group_gen: ::ethers::core::types::U256,
    pub group_gen_inv: ::ethers::core::types::U256,
    pub open_key_g: G1Point,
    pub h: G2Point,
    pub beta_h: G2Point,
}
///`WithdrawData(uint256,address,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct WithdrawData {
    pub nf_token_id: ::ethers::core::types::U256,
    pub recipient_address: ::ethers::core::types::Address,
    pub value: ::ethers::core::types::U256,
    pub withdraw_fund_salt: ::ethers::core::types::U256,
}
