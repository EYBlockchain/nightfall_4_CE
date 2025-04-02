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
