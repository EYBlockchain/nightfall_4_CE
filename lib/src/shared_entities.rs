use crate::{
    commitments::{Commitment, Nullifiable},
    contract_conversions::{Addr, FrBn254, Uint256},
    nf_client_proof::{Proof, PublicInputs},
    secret_hash::SecretHash,
    serialization::{ark_de_hex, ark_se_hex},
};
use alloy::primitives::Address;
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::PrimeField;
use ark_serialize::SerializationError;
use ark_std::UniformRand;
use jf_primitives::poseidon::{FieldHasher, Poseidon, PoseidonError};
use log::{error, warn};
use nf_curves::ed_on_bn254::BabyJubjub as BabyJubJub;
use nightfall_bindings::artifacts::Nightfall;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::fmt::Debug;

/// Struct used to represent deposit data, used in making deposit proofs by the proposer.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct DepositData {
    /// The Nightfall token ID
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_token_id: Fr254,
    /// The Nightfall slot ID
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nf_slot_id: Fr254,
    /// The value of the deposit
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub value: Fr254,
    /// The secret hash used to redeem the deposit
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub secret_hash: Fr254,
}

/// A struct representing the synchronisation status of a container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynchronisationPhase {
    /// Client is fully caught up with the on-chain state.
    Synchronized,
    /// Client is ahead of the chain and No need to resync.
    AheadOfChain { blocks_ahead: usize },
    /// Client is out-of-sync and must restart syncing.
    Desynchronized,
}

/// A struct representing the synchronisation status of a container
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SynchronisationStatus {
    phase: SynchronisationPhase,
}

impl SynchronisationStatus {
    /// Create a new instance
    pub fn new(phase: SynchronisationPhase) -> Self {
        Self { phase }
    }
    /// Get the current synchronisation phase
    pub fn phase(&self) -> SynchronisationPhase {
        self.phase
    }
    /// return whether the application is synchronised with the blockchain
    pub fn is_synchronised(&self) -> bool {
        matches!(self.phase, SynchronisationPhase::Synchronized)
    }
    /// Set the synchronisation status to fully synchronised
    pub fn set_synchronised(&mut self) {
        self.phase = SynchronisationPhase::Synchronized;
    }
    /// clear the synchronisation status
    pub fn clear_synchronised(&mut self) {
        self.phase = SynchronisationPhase::Desynchronized;
    }
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

/// Formalises the compressed secrets in a client proof.  This makes the purpose of the data clearer than using
/// the tuple output of the KEM-DEM function
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct CompressedSecrets {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub cipher_text: [Fr254; 5],
}

/// Transaction struct representing NF on chain transaction
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct OnChainTransaction {
    // The fee paid to the proposer.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee: Fr254,
    // List of new commitments created by this transaction.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub commitments: [Fr254; 4],
    // List of nullifiers consumed by this transaction.
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifiers: [Fr254; 4],
    // public data (public inputs) associated with this transaction.
    pub public_data: CompressedSecrets,
}
/// Converts the NF_4 smart contract representation of an on-chain transaction (i.e. a transaction that is
/// rolled up into a block), into a form more sutiable for manipulation in Rust.
impl From<Nightfall::OnChainTransaction> for OnChainTransaction {
    fn from(ntx: Nightfall::OnChainTransaction) -> Self {
        Self {
            fee: FrBn254::try_from(ntx.fee)
                .expect("Conversion of on-chain fee into field element should never fail")
                .0,
            commitments: ntx.commitments.map(|c| {
                FrBn254::try_from(c)
                    .expect(
                        "Conversion of on-chain commitments into field elements should never fail",
                    )
                    .0
            }),
            nullifiers: ntx.nullifiers.map(|n| {
                FrBn254::try_from(n)
                    .expect(
                        "Conversion of on-chain commitments into field elements should never fail",
                    )
                    .0
            }),
            public_data: ntx.public_data.into(),
        }
    }
}

/// Converts the Domain representation of an onchain transaction (i.e. one that is rolled up into a block)
/// into one suitable for interacting with the smart contract
impl From<OnChainTransaction> for Nightfall::OnChainTransaction {
    fn from(otx: OnChainTransaction) -> Self {
        Self {
            fee: Uint256::from(otx.fee).into(),
            commitments: otx.commitments.map(|c| Uint256::from(c).into()),
            nullifiers: otx.nullifiers.map(|n| Uint256::from(n).into()),
            public_data: otx.public_data.into(),
        }
    }
}

/// Converts a ClientTransaction into a form suitable for rolling into a block.
impl<P> From<&ClientTransaction<P>> for OnChainTransaction {
    fn from(client_transaction: &ClientTransaction<P>) -> Self {
        Self {
            fee: client_transaction.fee,
            commitments: client_transaction.commitments,
            nullifiers: client_transaction.nullifiers,
            public_data: client_transaction.compressed_secrets,
        }
    }
}

impl From<&PublicInputs> for OnChainTransaction {
    fn from(p: &PublicInputs) -> Self {
        OnChainTransaction {
            fee: p.fee,
            commitments: p.commitments,
            nullifiers: p.nullifiers,
            public_data: CompressedSecrets {
                cipher_text: p.compressed_secrets,
            },
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
                warn!("TokenType value {value} not supported, defaulting to ERC20");
                TokenType::ERC20
            }
        }
    }
}

/// Transaction struct representing NF client transaction
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
            error!("Proof hash computation error {e}");
            SerializationError::InvalidData
        })?;
        let hash = Keccak256::digest(encoding);
        // convert to u32 because the Mongo Rust driver doesn't support u8
        Ok(hash.iter().map(|&b| b as u32).collect())
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
    pub public_key: TEAffine<BabyJubJub>,
    pub salt: Salt,
}

impl Preimage {
    #[allow(dead_code)]
    pub fn new(
        value: Fr254,
        nf_token_id: Fr254,
        nf_slot_id: Fr254,
        public_key: TEAffine<BabyJubJub>,
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
    fn get_public_key(&self) -> TEAffine<BabyJubJub> {
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

        let key = match &self.salt {
            Salt::Deposit(secret) if self.public_key == TEAffine::<BabyJubJub>::zero() => {
                // Deposit: use hash(secret_preimage, DOMAIN)
                let arr = secret.to_array();
                poseidon.hash(&[
                    arr[0],
                    arr[1],
                    arr[2],
                    Fr254::from_le_bytes_mod_order(b"DEPOSIT_NULLIFIER_V1"),
                ])?
            }
            _ => *nullifier_key, // Transfer: use nullifier_key
        };

        poseidon.hash(&[key, commitment_hash])
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

impl From<WithdrawData> for Nightfall::WithdrawData {
    fn from(data: WithdrawData) -> Nightfall::WithdrawData {
        let nf_token_id = Uint256::from(data.nf_token_id).0;
        let recipient_address = Address::from(
            Addr::try_from(data.withdraw_address)
                .expect("Could not convert WithdrawData withdraw address to Solidity address"),
        );
        let value = Uint256::from(data.value).0;
        let withdraw_fund_salt = Uint256::from(data.withdraw_fund_salt).0;
        Nightfall::WithdrawData {
            nf_token_id,
            recipient_address,
            value,
            withdraw_fund_salt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;
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
}
