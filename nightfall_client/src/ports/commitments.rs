use std::fmt::Debug;

use ark_bn254::Fr as Fr254;
use nf_curves::ed_on_bn254::BJJTEAffine as JubJub;

use crate::domain::entities::{DepositSecret, Preimage};
use jf_primitives::{poseidon::PoseidonError, trees::MembershipProof};

pub trait Commitment: Debug {
    fn hash(&self) -> Result<Fr254, PoseidonError>;
    fn get_preimage(&self) -> Preimage;
    fn get_value(&self) -> Fr254;
    fn get_salt(&self) -> Fr254;
    fn get_nf_token_id(&self) -> Fr254;
    fn get_public_key(&self) -> JubJub;
    fn get_nf_slot_id(&self) -> Fr254;
    fn get_secret_preimage(&self) -> DepositSecret;
}

pub trait Nullifiable: Commitment + Copy {
    fn nullifier_hash(&self, nullifier_key: &Fr254) -> Result<Fr254, PoseidonError>;
}

pub trait TreeMember {
    fn get_membership_proof(&self) -> MembershipProof<Fr254>;
    fn get_root(&self) -> Fr254;
}
