#![cfg(test)]

use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{
    domain::entities::{ClientTransaction, CommitmentStatus, HexConvertible, Preimage, Salt},
    drivers::{derive_key::ZKPKeys, rest::models::PreimageReq},
    get_fee_token_id,
    ports::{
        commitments::Commitment,
        db::*,
        proof::{PrivateInputs, Proof, ProvingEngine, PublicInputs},
    },
};
use ark_bn254::Fr as Fr254;
use ark_ec::AffineRepr;
use ark_ff::{BigInt, BigInteger, Field, PrimeField};
use ark_serialize::SerializationError;
use ark_std::Zero;
use ark_std::{rand::Rng, UniformRand};
use async_trait::async_trait;
use ethers::types::{Bytes, H256, I256};
use jf_primitives::poseidon::PoseidonError;
use nf_curves::ed_on_bn254::{
    BJJTEAffine as JubJubAffine, BJJTEProjective as JubJub, Fr as FqJubJub,
};
use num::BigUint;
use serde::{Deserialize, Serialize};
use std::fmt::Error;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct MockCommitmentEntry {
    pub preimage: Preimage,
    pub nullifier: Fr254,
    pub status: CommitmentStatus,
}
impl MockCommitmentEntry {
    pub fn rand(mut rng: impl Rng) -> Self {
        let root_key = Fr254::rand(&mut rng);
        let zkp_keys = ZKPKeys::new(root_key).unwrap();
        let bytes = Fr254::rand(&mut rng).into_bigint().to_bytes_le();
        let rand_biguin = BigUint::from_bytes_le(&bytes) >> 4;
        let nf_token_id = Fr254::from(rand_biguin);
        let preimage = Preimage {
            value: Fr254::rand(&mut rng),
            nf_token_id,
            nf_slot_id: nf_token_id,
            public_key: zkp_keys.zkp_public_key,
            salt: Salt::new_transfer_salt(),
        };
        Self {
            preimage,
            nullifier: Fr254::zero(),
            status: CommitmentStatus::Unspent,
        }
    }
}
impl CommitmentEntryDB for MockCommitmentEntry {
    fn new(preimage: Preimage, _key: Fr254, nullifier: Fr254, status: CommitmentStatus) -> Self {
        Self {
            preimage,
            nullifier,
            status,
        }
    }

    fn get_status(&self) -> CommitmentStatus {
        todo!()
    }
}

impl Commitment for MockCommitmentEntry {
    fn get_preimage(&self) -> Preimage {
        self.preimage
    }
    fn get_nf_slot_id(&self) -> Fr254 {
        self.preimage.nf_slot_id
    }
    fn get_public_key(&self) -> nf_curves::ed_on_bn254::BJJTEAffine {
        self.preimage.public_key
    }
    fn get_salt(&self) -> Fr254 {
        self.preimage.get_salt()
    }
    fn get_nf_token_id(&self) -> Fr254 {
        self.preimage.nf_token_id
    }
    fn get_value(&self) -> Fr254 {
        self.preimage.value
    }
    fn hash(&self) -> Result<Fr254, PoseidonError> {
        self.preimage.hash()
    }
    fn get_secret_preimage(&self) -> crate::domain::entities::DepositSecret {
        self.preimage.get_secret_preimage()
    }
}
#[async_trait]
impl CommitmentDB<Fr254, MockCommitmentEntry> for HashMap<Fr254, MockCommitmentEntry> {
    async fn get_all_commitments(
        &self,
    ) -> Result<Vec<(Fr254, MockCommitmentEntry)>, mongodb::error::Error> {
        let mut v = vec![];
        for (&key, value) in self.iter() {
            v.push((key, value.clone()));
        }
        Ok(v)
    }

    async fn get_commitment(&self, k: &Fr254) -> Option<MockCommitmentEntry> {
        self.get(k).cloned()
    }

    async fn get_available_commitments(
        &self,
        nf_token_id: Fr254,
    ) -> Option<Vec<MockCommitmentEntry>> {
        let f = self
            .values()
            .filter(|v| v.get_nf_token_id() == nf_token_id && v.status == CommitmentStatus::Unspent)
            .cloned()
            .collect::<Vec<_>>();
        if f.is_empty() {
            return None;
        }
        Some(f)
    }

    async fn store_commitment(&self, _commitment: MockCommitmentEntry) -> Option<()> {
        todo!()
    }

    async fn store_commitments(
        &self,
        _commitments: &[MockCommitmentEntry],
        _dup_key_check: bool,
    ) -> Option<()> {
        todo!()
    }

    async fn get_balance(&self, _k: &Fr254) -> Option<Fr254> {
        todo!()
    }

    async fn mark_commitments_pending_spend(&self, _commitments: Vec<Fr254>) -> Option<()> {
        todo!()
    }

    async fn mark_commitments_spent(&self, _commitments: Vec<Fr254>) -> Option<()> {
        todo!()
    }

    async fn mark_commitments_unspent(
        &self,
        _commitments: &[Fr254],
        _l1_hash: Option<H256>,
        _l2_blocknumber: Option<I256>,
    ) -> Option<()> {
        todo!()
    }

    async fn mark_commitments_pending_creation(&self, _commitments: Vec<Fr254>) -> Option<()> {
        todo!()
    }

    async fn add_nullifier(&self, _key: &Fr254, _nullifier: Fr254) -> Option<()> {
        todo!()
    }
}
// test helper function (lets one instantiate a mock DB containing random data)
pub fn random_mock_db(
    entry_count: u32,
) -> std::sync::Arc<tokio::sync::Mutex<HashMap<Fr254, MockCommitmentEntry>>> {
    let mut rng = ark_std::test_rng();
    let mut test_hashmap: HashMap<Fr254, MockCommitmentEntry> = HashMap::new();
    for k in 0..entry_count {
        test_hashmap.insert(Fr254::from(k), MockCommitmentEntry::rand(&mut rng));
    }
    Arc::new(Mutex::new(test_hashmap))
}
// define a mock proof, a bit G16-like, which returns a fixed answer
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MockProof {
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}

#[derive(Debug)]
pub struct MockProvingEngine;

impl Proof for MockProof {
    fn compress_proof(&self) -> Result<Bytes, SerializationError> {
        Ok([
            &[self.a.len() as u8],
            self.a.as_slice(),
            &[self.b.len() as u8],
            self.b.as_slice(),
            self.c.as_slice(),
        ]
        .concat()
        .into())
    }

    fn from_compressed(compressed_proof: Bytes) -> Result<Self, SerializationError> {
        let bytes = compressed_proof.to_vec();
        let a_len = bytes[0];
        let a = bytes[1..(1 + a_len as usize)].to_vec();
        let b_len = bytes[1 + a_len as usize];
        let b = bytes[(2 + a_len as usize)..(2 + a_len as usize + b_len as usize)].to_vec();
        let c = bytes[(2 + a_len as usize + b_len as usize)..].to_vec();
        Ok(MockProof { a, b, c })
    }
}

impl ProvingEngine<MockProof> for MockProvingEngine {
    type Error = Error;
    fn prove(
        _private_inputs: &mut PrivateInputs,
        _public_inputs: &mut PublicInputs,
    ) -> Result<MockProof, Self::Error> {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];

        Ok(MockProof { a, b, c })
    }
    fn verify(_proof: &MockProof, _public_inputs: &PublicInputs) -> Result<bool, Error> {
        Ok(true)
    }
}

// A struct containing useful constant values for test purposes. The constants are self-consistent
// so that (for example) the nullifier_key will derive from the root_key
pub struct Mocks;

impl Mocks {
    pub fn get_root_key() -> Fr254 {
        Fr254::from(BigInt::new([
            0x1ac2d320c71b5a14,
            0x05a64f99c2ff8da5,
            0x667e6f5309ae9775,
            0x0cec95addb6e305a,
        ]))
    }
    pub fn get_zkp_public_key_x() -> Fr254 {
        Fr254::new(BigInt::new([
            0xe76ec60cb5e983a1,
            0x53687d3f515b7f4e,
            0xe9ea297240f1fa07,
            0x26f0c9e063a7b5d9,
        ]))
    }
    pub fn get_zkp_public_key_y() -> Fr254 {
        Fr254::new(BigInt::new([
            0xd75040da65a54a4a,
            0x52d0e761a621fb01,
            0x2f5e38533c0673e2,
            0xd5ed4c6c7a9dff,
        ]))
    }
    pub fn get_zkp_public_key() -> JubJub {
        let root_a = Fr254::from(168700u32).sqrt().unwrap();
        JubJubAffine {
            x: Self::get_zkp_public_key_x() * root_a,
            y: Self::get_zkp_public_key_y(),
        }
        .into_group()
    }
    pub fn get_nullifier_key() -> Fr254 {
        Fr254::from(BigInt::new([
            0x5f2415beff697c2a,
            0x5a65d1024be34f75,
            0xc84c19680f1279d5,
            0x302b6d99eae12fb5,
        ]))
    }
    pub fn get_compressed_zkp_public_key() -> Vec<u8> {
        BigInt::new([
            0xd75040da65a54a4a,
            0x52d0e761a621fb01,
            0x2f5e38533c0673e2,
            0xd5ed4c6c7a9dff,
        ])
        .to_bytes_le()
    }
    pub fn get_zkp_key() -> ZKPKeys {
        ZKPKeys::new(Self::get_root_key()).unwrap()
    }
    pub fn get_zkp_private_key() -> FqJubJub {
        FqJubJub::from(BigInt::new([
            0xbdb92fca1b98236c,
            0x38050479a484a35a,
            0x84f2115b52fb35a9,
            0x2fd309fe873fade,
        ]))
    }
    pub fn get_preimage() -> Preimage {
        Preimage {
            value: Fr254::from(16),
            nf_token_id: get_fee_token_id(),
            nf_slot_id: get_fee_token_id(),
            public_key: Self::get_zkp_key().zkp_public_key,
            salt: Salt::Transfer(Fr254::new(BigInt::new([
                0x7d1faf1a18c7788f,
                0x04e53984ebf57f9a,
                0xcf6d1069ea03ff3c,
                0x02f01189eb498b10,
            ]))),
        }
    }
    pub fn get_preimage_req() -> PreimageReq {
        PreimageReq {
            value: "10".to_string(),
            erc_address: "ea730722cfF77681312747bE5Fe9B39eAac67DC6".to_string(),
            public_key: hex::encode(Self::get_zkp_key().compressed_public_key().unwrap()),
            token_id: "00".to_string(),
            salt: Fr254::to_hex_string(&Fr254::new(BigInt::new([
                0x7d1faf1a18c7788f,
                0x04e53984ebf57f9a,
                0xcf6d1069ea03ff3c,
                0x02f01189eb498b10,
            ]))),
        }
    }

    pub fn get_transaction() -> ClientTransaction<MockProof> {
        ClientTransaction::<MockProof> {
            fee: Fr254::from(2), // fee cannot be zero as we need to incentivize the proposer
            historic_commitment_roots: Default::default(),
            commitments: [
                Self::get_preimage().hash().unwrap(),
                Fr254::zero(),
                Fr254::zero(),
                Fr254::zero(),
            ],
            nullifiers: [Fr254::from(0), Fr254::from(0), Fr254::zero(), Fr254::zero()],
            compressed_secrets: Default::default(),
            proof: Self::get_mock_proof(),
        }
    }

    pub fn get_mock_proof() -> MockProof {
        MockProof {
            a: vec![1, 2, 3],
            b: vec![4, 5, 6],
            c: vec![7, 8, 9],
        }
    }
}
