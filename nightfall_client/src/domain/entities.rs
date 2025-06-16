use crate::ports::secret_hash::SecretHash;
use crate::ports::{
    commitments::{Commitment, Nullifiable},
    key_provider::KeyProvider,
    proof::{Proof, PublicInputs},
};
use lib::error::HexError;

use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_serialize::SerializationError;
use ark_std::UniformRand;
use lib::serialization::{ark_de_hex, ark_se_hex};

use jf_primitives::poseidon::{FieldHasher, Poseidon, PoseidonError};
use lib::hex_conversion::HexConvertible;
use log::{error, warn};
use nf_curves::ed_on_bn254::{BabyJubjub, Fr as BJJScalar};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::GenericArray, Digest, Keccak256};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    str::{self, FromStr},
};

/// A struct representing the status of an HTTP request
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub status: RequestStatus,
    pub uuid: String,
}

/// Struct to represent the realtionship between request and commitment
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCommitmentMapping {
    pub request_id: String,
    pub commitment_hash: String,
}

/// An enum representing the possible statuses of an HTTP request
#[derive(Serialize, Deserialize, Debug)]
pub enum RequestStatus {
    Queued,
    Submitted,
    Failed,
    Processing,
    ProposerUnreachable,
    Confirmed,
}

impl Display for RequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestStatus::Queued => write!(f, "Queued"),
            RequestStatus::Submitted => write!(f, "Submitted"),
            RequestStatus::Failed => write!(f, "Failed"),
            RequestStatus::Processing => write!(f, "Processing"),
            RequestStatus::ProposerUnreachable => write!(f, "ProposerUnreachable"),
            RequestStatus::Confirmed => write!(f, "Confirmed"),
        }
    }
}

/// A struct representing the synchronisation status of a container
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SynchronisationStatus(bool);

impl SynchronisationStatus {
    /// Create a new instance
    pub fn new(synchronised: bool) -> Self {
        Self(synchronised)
    }
    /// return whether the application is synchronised with the blockchain
    pub fn is_synchronised(&self) -> bool {
        self.0
    }
    /// set the synchronisation status to true
    pub fn set_synchronised(&mut self) {
        self.0 = true;
    }
    /// clear the synchronisation status
    pub fn clear_synchronised(&mut self) {
        self.0 = false;
    }
}

/// a struct representing the states that a commitment can be in
#[derive(Clone, Debug, Deserialize, Serialize, Copy, PartialEq, Default)]
pub enum CommitmentStatus {
    PendingSpend,
    Spent,
    PendingCreation,
    #[default]
    Unspent,
}

/// A struct representing a node in a Merkle Tree
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub index: usize,
}

/// A struct representing summary data about an append-only Merkle Tree
pub struct AppendOnlyTreeMetadata<F> {
    pub main_tree_height: u32,
    pub sub_tree_height: u32,
    pub sub_tree_count: usize,
    pub frontier: Vec<F>,
    pub root: F,
}

/// A struct representing a proposer in a linked list of proposers (used in the ProposerManager contract)
#[derive(Serialize, Deserialize, Debug)]
pub struct Proposer {
    pub stake: ::ethers::core::types::U256,
    pub addr: ::ethers::core::types::Address,
    pub url: ::std::string::String,
    pub next_addr: ::ethers::core::types::Address,
    pub previous_addr: ::ethers::core::types::Address,
}

/// Formalises the compressed secrets in a client proof.  This makes the purpose of the data clearer than using
/// the tuple output of the KEM-DEM function
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct CompressedSecrets {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub cipher_text: [Fr254; 5],
}

pub struct EnvironmentKey;

impl KeyProvider<BJJScalar> for EnvironmentKey {
    fn get_key(key_id: &str) -> Option<BJJScalar> {
        let key_string = env::var(key_id).ok()?;
        BJJScalar::from_str(&key_string).ok()
    }
    fn set_key(key_id: &str, key: BJJScalar) -> Result<(), Box<dyn Error>> {
        let key_string = key.to_string();
        // Acknowledge Possible Risks: we're confident that the use of std::env::set_var is indeed safe in this context
        unsafe {
            env::set_var(key_id, key_string);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Transport {
    OnChain,
    OffChain,
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OperationType {
    #[default]
    Deposit,
    Withdraw,
    Transfer,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "Deposit"),
            OperationType::Withdraw => write!(f, "Withdraw"),
            OperationType::Transfer => write!(f, "Transfer"),
        }
    }
}

impl From<OperationType> for u8 {
    fn from(value: OperationType) -> Self {
        match value {
            OperationType::Deposit => 0,
            OperationType::Withdraw => 1,
            OperationType::Transfer => 2,
        }
    }
}

impl From<u8> for OperationType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperationType::Deposit,
            1 => OperationType::Withdraw,
            2 => OperationType::Transfer,
            _ => OperationType::Deposit,
        }
    }
}

/// Struct representing a Nightfall operation, together with the transport mechanism
#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub transport: Transport,
    pub operation_type: OperationType,
}

pub struct ParseOperationError;
impl FromStr for Operation {
    type Err = ParseOperationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deposit-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Deposit,
            }),
            "deposit-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Deposit,
            }),
            "withdraw-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Withdraw,
            }),
            "withdraw-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Withdraw,
            }),
            "transfer-onchain" => Ok(Operation {
                transport: Transport::OnChain,
                operation_type: OperationType::Transfer,
            }),
            "transfer-offchain" => Ok(Operation {
                transport: Transport::OffChain,
                operation_type: OperationType::Transfer,
            }),
            _ => Err(ParseOperationError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProofType {
    Groth16,
    Plonk,
}

pub struct ParseProofTypeError;
impl FromStr for ProofType {
    type Err = ParseProofTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Groth16" => Ok(ProofType::Groth16),
            "Plonk" => Ok(ProofType::Plonk),
            _ => Err(ParseProofTypeError),
        }
    }
}

/// Token Type Based on ERC Standards or L2
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    #[default]
    ERC20,
    ERC1155,
    ERC721,
    ERC3525,
}

impl From<TokenType> for u8 {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::ERC20 => 0,
            TokenType::ERC1155 => 1,
            TokenType::ERC721 => 2,
            TokenType::ERC3525 => 3,
        }
    }
}

impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => TokenType::ERC20,
            1 => TokenType::ERC1155,
            2 => TokenType::ERC721,
            3 => TokenType::ERC3525,
            _ => {
                warn!(
                    "TokenType value {} not supported, defaulting to ERC20",
                    value
                );
                TokenType::ERC20
            }
        }
    }
}

/// Transaction struct representing NF transaction
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientTransaction<P> {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub historic_commitment_roots: [Fr254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments: [Fr254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifiers: [Fr254; 4],
    pub compressed_secrets: CompressedSecrets,
    pub proof: P,
}

impl<P: Proof + Debug + Serialize + Clone> ClientTransaction<P> {
    #[allow(dead_code)]
    pub fn hash(&self) -> Result<Vec<u32>, SerializationError> {
        let encoding = serde_json::to_vec(self).map_err(|e| {
            error!("Proof hash computation error {}", e);
            SerializationError::InvalidData
        })?;
        // let encoded: Vec<Token> = self.to_solidity_struct()?;
        // let encoding = encode(&encoded);
        let hash: GenericArray<u8, _> = Keccak256::digest(encoding);
        // convert to u32 because the Mongo Rust driver doesn't support u8
        Ok(hash
            .as_slice()
            .iter()
            .map(|h| u32::from(*h))
            .collect::<Vec<u32>>()
            .to_owned()) // Let's just own it for now
    }
}

impl<P: Proof + Debug + Serialize + Clone> From<&ClientTransaction<P>> for PublicInputs {
    fn from(tx: &ClientTransaction<P>) -> Self {
        PublicInputs {
            fee: tx.fee,
            commitments: tx.commitments,
            nullifiers: tx.nullifiers,
            compressed_secrets: tx.compressed_secrets.cipher_text,
            roots: tx.historic_commitment_roots,
        }
    }
}

/// Enum used for the two different types of salt that can be used in a commitment.
/// The normal randomly generated one and one that is the output of a hash.
#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Salt {
    /// Used in a transfer transaction, randomly generated.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    Transfer(Fr254),
    /// Used in deposits, proving knowledge of this hash preimage allows the depositor to redeem their tokens.
    Deposit(DepositSecret),
}

impl Default for Salt {
    fn default() -> Self {
        Salt::Transfer(Fr254::from(0u8))
    }
}

impl Salt {
    /// Retrieves the actual salt
    pub fn get_salt(&self) -> Fr254 {
        match self {
            Salt::Transfer(f) => *f,
            // Unwrap is safe because the hash only errors for unsupported array lengths and this array length is supported.
            Salt::Deposit(preimage) => preimage.hash().unwrap(),
        }
    }

    /// Makes a new transfer salt
    pub fn new_transfer_salt() -> Self {
        Salt::Transfer(Fr254::rand(&mut ark_std::rand::thread_rng()))
    }
}
// Preimage
#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Preimage {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub value: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_token_id: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_slot_id: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub public_key: TEAffine<BabyJubjub>,
    pub salt: Salt,
}

impl Preimage {
    #[allow(dead_code)]
    pub fn new(
        value: Fr254,
        nf_token_id: Fr254,
        nf_slot_id: Fr254,
        public_key: TEAffine<BabyJubjub>,
        salt: Salt,
    ) -> Preimage {
        Preimage {
            value,
            nf_token_id,
            nf_slot_id,
            public_key,
            salt,
        }
    }
}

impl Commitment for Preimage {
    fn hash(&self) -> Result<Fr254, PoseidonError> {
        let poseidon: Poseidon<Fr254> = Poseidon::new();
        poseidon.hash(&[
            self.nf_token_id,
            self.nf_slot_id,
            self.value,
            self.public_key.x,
            self.public_key.y,
            self.salt.get_salt(),
        ])
    }
    fn get_preimage(&self) -> Preimage {
        Preimage { ..(*self) }
    }
    fn get_value(&self) -> Fr254 {
        self.value
    }
    fn get_salt(&self) -> Fr254 {
        self.salt.get_salt()
    }
    fn get_public_key(&self) -> TEAffine<BabyJubjub> {
        self.public_key
    }
    fn get_nf_token_id(&self) -> Fr254 {
        self.nf_token_id
    }
    fn get_nf_slot_id(&self) -> Fr254 {
        self.nf_slot_id
    }
    fn get_secret_preimage(&self) -> DepositSecret {
        match self.salt {
            Salt::Transfer(_) => DepositSecret::default(),
            Salt::Deposit(d) => d,
        }
    }
}

impl Nullifiable for Preimage {
    fn nullifier_hash(&self, nullifier_key: &Fr254) -> Result<Fr254, PoseidonError> {
        let commitment_hash = self.hash()?;
        let poseidon: Poseidon<Fr254> = Poseidon::new();
        poseidon.hash(&[*nullifier_key, commitment_hash])
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DepositSecret {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub preimage_one: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub preimage_two: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub preimage_three: Fr254,
}

impl SecretHash for DepositSecret {
    fn hash(&self) -> Result<Fr254, PoseidonError> {
        let poseidon: Poseidon<Fr254> = Poseidon::new();
        poseidon.hash(&[self.preimage_one, self.preimage_two, self.preimage_three])
    }
    fn to_array(&self) -> [Fr254; 3] {
        [self.preimage_one, self.preimage_two, self.preimage_three]
    }
}

impl DepositSecret {
    /// Create a new instance from three secrets
    pub fn new(preimage_one: Fr254, preimage_two: Fr254, preimage_three: Fr254) -> Self {
        Self {
            preimage_one,
            preimage_two,
            preimage_three,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WithdrawData {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_token_id: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub withdraw_address: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub value: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub withdraw_fund_salt: Fr254,
}
impl WithdrawData {
    /// Create a new instance
    pub fn new(
        nf_token_id: Fr254,
        withdraw_address: Fr254,
        value: Fr254,
        nullifier_one: Fr254,
    ) -> Self {
        Self {
            nf_token_id,
            withdraw_address,
            value,
            withdraw_fund_salt: nullifier_one,
        }
    }
}

impl<P: Proof> From<&ClientTransaction<P>> for WithdrawData {
    fn from(value: &ClientTransaction<P>) -> Self {
        WithdrawData {
            nf_token_id: value.compressed_secrets.cipher_text[0],
            withdraw_address: value.compressed_secrets.cipher_text[1],
            value: value.compressed_secrets.cipher_text[2],
            withdraw_fund_salt: value.nullifiers[0],
        }
    }
}
pub struct ERCAddress;
impl ERCAddress {
    #[allow(dead_code)]
    pub fn try_from_hex_string(h: &str) -> Result<Fr254, HexError> {
        let bytes = Vec::<u8>::from_hex_string(h)?;
        let uint = BigUint::from_bytes_be(&bytes);
        // Check the address is no more than 20 bytes long
        if uint > BigUint::from_bytes_be(&[255; 20]) {
            return Err(HexError::InvalidStringLength);
        }
        Ok(Fr254::from(uint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;
    mod operation_tests {
        use super::*;
        #[test]
        fn enum_correct() {
            let o = Operation::from_str("deposit-onchain");
            assert!(o.is_ok());
        }

        #[test]
        fn enum_incorrect() {
            let o = Operation::from_str("clearly_wrong");
            assert!(o.is_err());
        }
    }
    mod proof_type_tests {
        use super::*;
        #[test]
        fn enum_correct() {
            let o = ProofType::from_str("Groth16");
            assert!(o.is_ok());
        }

        #[test]
        fn enum_incorrect() {
            let o = ProofType::from_str("clearly_wrong");
            assert!(o.is_err());
        }
    }
    mod token_type_tests {
        use super::*;
        #[test]
        fn return_correct_number() {
            let t_erc20 = TokenType::ERC20;
            let u_erc20 = u8::from(t_erc20);
            assert_eq!(u_erc20, 0);
            let t_erc1155 = TokenType::ERC1155;
            let u_erc1155 = u8::from(t_erc1155);
            assert_eq!(u_erc1155, 1);
            let t_erc721 = TokenType::ERC721;
            let u_erc721 = u8::from(t_erc721);
            assert_eq!(u_erc721, 2);
            let t_erc3525 = TokenType::ERC3525;
            let u_erc3525 = u8::from(t_erc3525);
            assert_eq!(u_erc3525, 3);
        }
    }
    mod preimage_tests {
        use super::*;
        use ark_ff::BigInt;
        use nf_curves::ed_on_bn254::BJJTEProjective;

        #[test]
        // This test takes fixed, randomly chosen values for all of the preimage components, then
        // compares the preimage hash (from the Commitment trait) with that created by
        // manually packing the preimage and hashing with the Poseidon hash.
        // it doesn't therefore test the poseidon hash itself, just the preimage bit-twiddling.
        // It also tests the Nullifier hash.
        fn compute_correct_hashes() {
            let value = Fr254::from(10);
            let erc_address = Fr254::new(BigInt([
                0x5fe9b39eaac67dc6,
                0xcff77681312747be,
                0xea730722,
                0x00,
            ]));
            let token_id = Fr254::new(BigInt::new([
                0x94c25463ca1c3fbe,
                0x042da2de98c064cf,
                0xf46bfbdbb7949e00,
                0xaaddd44f7e3b786e,
            ]));
            let public_key = BJJTEProjective::new(
                Fr254::new(BigInt::new([
                    12932170579734557803,
                    8516061745511572932,
                    1673910578125676425,
                    3321572574588525558,
                ])),
                Fr254::new(BigInt::new([
                    10483523837209188168,
                    16160152051684956071,
                    6754854840592244876,
                    2043532635058116748,
                ])),
                Fr254::new(BigInt::new([
                    17253370541782799919,
                    163006934830020888,
                    13286636799765123940,
                    852659491963929648,
                ])),
                Fr254::new(BigInt::new([
                    10218970634224697192,
                    14503578833116929737,
                    11535629639282784339,
                    1178388109415204005,
                ])),
            )
            .into_affine();
            let salt = Salt::Transfer(Fr254::new(BigInt::new([
                0x7d1faf1a18c7788f,
                0x04e53984ebf57f9a,
                0xcf6d1069ea03ff3c,
                0x02f01189eb498b10,
            ])));
            let p = Preimage::new(value, erc_address, token_id, public_key, salt);
            let poseidon: Poseidon<Fr254> = Poseidon::new();
            let test_hash = poseidon.hash(&[
                erc_address,
                token_id,
                value,
                public_key.x,
                public_key.y,
                salt.get_salt(),
            ]);
            let computed_hash = p.hash();
            assert_eq!(test_hash.unwrap(), computed_hash.unwrap());
            let nullifier_key = Fr254::new(BigInt::new([
                9016117505638758543,
                352751388875653018,
                14946620785396285244,
                211688466542070544,
            ]));
            let nullifier_key_compute = p.nullifier_hash(&nullifier_key);
            let poseidon: Poseidon<Fr254> = Poseidon::new();
            let nullifier_key_test = poseidon.hash(&[nullifier_key, p.hash().unwrap()]);
            assert_eq!(nullifier_key_test.unwrap(), nullifier_key_compute.unwrap());
        }
    }
    mod erc_address_tests {
        use super::*;
        use ark_ff::BigInt;
        #[test]
        fn create_erc_address_from_hex() {
            let test_address = Fr254::new(BigInt([
                0x5fe9b39eaac67dc6,
                0xcff77681312747be,
                0xea730722,
                0x00,
            ]));
            let test_address_2 = Fr254::new(BigInt([
                0x5fe9b39eaac67dc6,
                0xcff77681312747be,
                0x00730722,
                0x00,
            ]));

            let address = "0xea730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address
            );
            assert_eq!(
                ERCAddress::try_from_hex_string(&address[2..]).unwrap(),
                test_address
            );
            let address = "0x00ea730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address
            );
            // make sure leading zeros are correctly handled
            let address = "0x00730722cfF77681312747bE5Fe9B39eAac67DC6";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                test_address_2
            );
        }
        #[test]
        fn address_too_big() {
            let address = "0x010000000000000000000000000000000000000000";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap_err(),
                HexError::InvalidStringLength
            );
            let address = "0xffffffffffffffffffffffffffffffffffffffff";
            assert_eq!(
                ERCAddress::try_from_hex_string(address).unwrap(),
                Fr254::from(BigInt::new([
                    0xffffffffffffffff,
                    0xffffffffffffffff,
                    0xffffffff,
                    0,
                ],))
            );
        }
    }
}
