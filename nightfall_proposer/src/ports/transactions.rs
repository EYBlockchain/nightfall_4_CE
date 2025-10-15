use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ed_on_bn254::EdwardsConfig;
use ark_ff::BigInteger256;
use lib::{
    nf_client_proof::{Proof, PublicInputs},
    shared_entities::TokenType,
};
use serde::Serialize;
use std::fmt::Debug;

pub trait ClientTx<P: Proof + Debug + Serialize + Send>: Send {
    fn verify_transaction(&self) -> bool;
    fn get_value(&self) -> Fr254;
    fn get_fee(&self) -> Fr254;
    fn get_historic_commitment_roots(&self) -> Vec<Fr254>;
    fn get_circuit_hash(&self) -> Fr254;
    fn get_token_type(&self) -> TokenType;
    fn get_token_id(&self) -> BigInteger256;
    fn get_erc_address(&self) -> Fr254;
    fn get_recipient_address(&self) -> Fr254;
    fn get_commitments(&self) -> Vec<Fr254>;
    fn get_nullifiers(&self) -> Vec<Fr254>;
    fn get_compressed_secrets(&self) -> Vec<Fr254>;
    fn get_roots(&self) -> Vec<Fr254>;
    fn get_fee_address(&self) -> Fr254;
    fn get_pub_point(&self) -> TEAffine<EdwardsConfig>;
    fn get_proof(&self) -> P;
    fn get_public_inputs(&self) -> PublicInputs;
}
