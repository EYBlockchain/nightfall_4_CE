use crate::serialization::{ark_de_hex, ark_se_hex};
use ark_bn254::Fr as Fr254;
use serde::{Deserialize, Serialize};
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
