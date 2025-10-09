#![cfg(test)]

use crate::{
    domain::entities::{ClientTransaction, Preimage, Salt},
    drivers::{derive_key::ZKPKeys, rest::models::PreimageReq},
    get_fee_token_id,
    ports::{
        commitments::Commitment,
    },
};
use alloy::primitives::Bytes;
use ark_bn254::Fr as Fr254;
use ark_ec::AffineRepr;
use ark_ff::{BigInt, BigInteger, Field};
use ark_serialize::SerializationError;
use ark_std::Zero;
use lib::{hex_conversion::HexConvertible, nf_client_proof::{PrivateInputs, Proof, ProvingEngine, PublicInputs}};
use nf_curves::ed_on_bn254::{
    BJJTEAffine as JubJubAffine, BJJTEProjective as JubJub, Fr as FqJubJub,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Error;

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
