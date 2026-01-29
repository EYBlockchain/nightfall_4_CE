//! Defines interface for something to be used as a secret hash in Nightfall 4.
use ark_bn254::Fr as Fr254;
use jf_primitives::poseidon::PoseidonError;

/// Secret hash interface
pub trait SecretHash {
    /// Create the secret hash from the pre-image.
    fn hash(&self) -> Result<Fr254, PoseidonError>;
    /// Turn it into an array for circuit purposes.
    fn to_array(&self) -> [Fr254; 3];
}
