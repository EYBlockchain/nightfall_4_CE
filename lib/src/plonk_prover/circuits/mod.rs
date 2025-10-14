pub mod kemdem_gadgets;

pub mod unified_circuit;
pub mod verify;

use ark_bn254::Fr as Fr254;
use ark_ff::MontFp;

pub const DOMAIN_KEM: Fr254 =
    MontFp!("21033365405711675223813179268586447041622169155539365736392974498519442361181");
pub const DOMAIN_DEM: Fr254 =
    MontFp!("1241463701002173366467794894814691939898321302682516549591039420117995599097");

// The DOMAIN_SHARED_SALT was derived from
/*
const LABEL: &str = "Nightfall|SharedSalt";
    let digest = Sha256::digest(LABEL.as_bytes());
    let fr = Fr254::from_le_bytes_mod_order(&digest);
 */
pub const DOMAIN_SHARED_SALT: Fr254 =
    MontFp!("4832298308599927878911686715232824310149976768223104556783163253807065458");
