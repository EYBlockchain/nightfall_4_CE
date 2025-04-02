use ark_bn254::Fr as Fr254;

use nf_curves::ed_on_bn254::{BJJTEProjective as JubJub, Fr as FqJubJub};

pub trait KeySpending {
    fn get_nullifier_key(&self) -> Fr254;
}
pub trait KeyViewing {
    fn get_private_key(&self) -> FqJubJub;
    fn get_public_key(&self) -> JubJub;
}

pub trait KeyZK: KeySpending + KeyViewing {
    fn get_root_key(&self) -> Fr254;
}

pub trait ProvingKey {
    type PK;
    fn get_proving_key(&self) -> Self::PK;
}

pub trait VerifyingKey {
    type VK;
    fn get_verifying_key(&self) -> Self::VK;
}
