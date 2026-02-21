pub mod circuit_builder;
pub mod circuits;
pub mod plonk_proof;

use crate::rollup_circuit_checks::get_configuration_keys_path;
use crate::utils::load_key_locally;
use crate::{rollup_circuit_checks::find_file_with_path, utils::load_key_from_server};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
use log::warn;
use std::{
    path::Path,
    sync::{Arc, OnceLock},
};
use ark_std::path::PathBuf;
use warp::path;

/// This function is used to retrieve the client proving key.
pub fn get_client_proving_key() -> &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> {
    static PK: OnceLock<Arc<ProvingKey<UnivariateKzgPCS<Bn254>>>> = OnceLock::new();
    PK.get_or_init(|| {
        // We'll try to load from the configuration server first.
        let clienpatht_pk_path = get_configuration_keys_path().expect("Configuration keys path not found").join("proving_key");
        let source_file = find_file_with_path(&clienpatht_pk_path).expect("Could not find path");
        

        if let Some(_key_bytes) = load_key_locally(&source_file) {
            let proving_key =
                ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                    &*std::fs::read(source_file).expect("Could not read proving key"),
                )
                .expect("Could not deserialise proving key");
            return Arc::new(proving_key);
        }
        warn!("Could not load proving_key from local file. Loading from server");

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
