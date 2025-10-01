pub mod blockchain;
pub mod derive_key;
pub mod rest;

use ark_bn254::Fr as Fr254;
use ark_ff::MontFp;

// The DOMAIN_SHARED_SALT was derived from
/*
const LABEL: &str = "Nightfall|SharedSalt";
    let digest = Sha256::digest(LABEL.as_bytes());
    let fr = Fr254::from_le_bytes_mod_order(&digest);
 */
pub const DOMAIN_SHARED_SALT: Fr254 =
    MontFp!("4832298308599927878911686715232824310149976768223104556783163253807065458");