pub mod circuit_builder;
pub mod circuits;
pub mod plonk_proof;

use crate::utils::load_key_from_server;
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
use log::warn;
use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

/// This function is used to retrieve the client proving key.
pub fn get_client_proving_key() -> &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> {
    static PK: OnceLock<Arc<ProvingKey<UnivariateKzgPCS<Bn254>>>> = OnceLock::new();
    PK.get_or_init(|| {
        fn find(path: &Path) -> Option<std::path::PathBuf> {
            if path.is_absolute() {
                match path.is_file() {
                    true => return Some(path.to_path_buf()),
                    false => return None,
                }
            }

            let cwd = std::env::current_dir().ok()?;
            let mut cwd = cwd.as_path();
            loop {
                let file_path = cwd.join(path);
                if file_path.is_file() {
                    return Some(file_path);
                }

                cwd = cwd.parent()?;
            }
        }

        // We'll try to load from the configuration server first.
        if let Some(key_bytes) = load_key_from_server("proving_key") {
            let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                &*key_bytes,
            )
            .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        // If that fails, we'll try to load from a local file
        warn!("Could not load proving key from server. Loading from local file");
        let path = Path::new("./configuration/bin/proving_key");
        let source_file = find(path).expect("Could not find path");
        let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read proving key"),
        )
        .expect("Could not deserialise proving key");
        Arc::new(pk)
    })
}
