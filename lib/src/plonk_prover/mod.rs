pub mod circuit_builder;
pub mod circuits;
pub mod plonk_proof;

use crate::{
    rollup_circuit_checks::{find_file_with_path, get_configuration_keys_path},
    utils::{load_key_from_server, load_key_locally},
};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
use log::warn;
use std::sync::{Arc, OnceLock};

/// This function is used to retrieve the client proving key.
pub fn get_client_proving_key() -> &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> {
    static PK: OnceLock<Arc<ProvingKey<UnivariateKzgPCS<Bn254>>>> = OnceLock::new();
    PK.get_or_init(|| {
        if let Some(client_pk_path) = get_configuration_keys_path().map(|path| path.join("proving_key")) {
            if let Some(source_file) = find_file_with_path(&client_pk_path) {
                if let Some(key_bytes) = load_key_locally(&source_file) {
                    let proving_key =
                        ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                            &*key_bytes,
                        )
                        .expect("Could not deserialise proving key");
                    return Arc::new(proving_key);
                }
                warn!("Could not load proving_key from local file. Loading from server");
            } else {
                warn!(
                    "Could not find local proving_key at {}. Loading from server",
                    client_pk_path.display()
                );
            }
        } else {
            warn!("Configuration keys path not found. Loading proving_key from server");
        }

        if let Some(key_bytes) = load_key_from_server("proving_key") {
            let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                &*key_bytes,
            )
            .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        panic!("Failed to load proving_key from both local and server");
    })
}
