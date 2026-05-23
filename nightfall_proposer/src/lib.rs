pub mod domain;
pub mod driven;
pub mod drivers;
pub mod initialisation;
pub mod ports;
pub mod services;

use ark_bn254::{Bn254, Fr as Fr254};
use ark_serialize::CanonicalDeserialize;
use jf_plonk::nightfall::ipa_structs::ProvingKey;
use jf_primitives::{
    pcs::prelude::UnivariateKzgPCS,
    poseidon::Poseidon,
    trees::{
        imt::{IndexedMerkleTree, LeafDBEntry},
        timber::Timber,
    },
};
use lib::{
    rollup_circuit_checks::{find_file_with_path, get_configuration_keys_path},
    utils::{load_key_from_server, load_key_locally},
};
use log::warn;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

type AppendOnlyTree = Timber<Fr254, Poseidon<Fr254>>;

type NullifierTree = IndexedMerkleTree<Fr254, Poseidon<Fr254>, HashMap<Fr254, LeafDBEntry<Fr254>>>;

/// This function is used so that we can work with one nullifier tree across the entire application.
pub fn get_nullifier_tree() -> &'static RwLock<NullifierTree> {
    static IMT_TREE: OnceLock<RwLock<NullifierTree>> = OnceLock::new();
    IMT_TREE.get_or_init(|| {
        RwLock::new(
            IndexedMerkleTree::new(Poseidon::<Fr254>::new(), 32)
                .expect("Invalid indexed Merkle tree"),
        )
    })
}

/// This function is used so that we can work with one historic root tree across the entire application.
pub fn get_historic_root_tree() -> &'static RwLock<AppendOnlyTree> {
    static ROOT_TREE: OnceLock<RwLock<AppendOnlyTree>> = OnceLock::new();
    ROOT_TREE.get_or_init(|| {
        let mut tree = Timber::new(Poseidon::<Fr254>::new(), 32);
        tree.insert_leaf(Fr254::from(0u8))
            .expect("Couldn't insert zero leaf into the tree");
        RwLock::new(tree)
    })
}

/// This function is used to retrieve the deposit proving key.
pub fn get_deposit_proving_key() -> &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> {
    static PK: OnceLock<Arc<ProvingKey<UnivariateKzgPCS<Bn254>>>> = OnceLock::new();
    PK.get_or_init(|| {
        if let Some(path) =
            get_configuration_keys_path().map(|path| path.join("deposit_proving_key"))
        {
            if let Some(source_file) = find_file_with_path(&path) {
                if let Some(key_bytes) = load_key_locally(&source_file) {
                    let deposit_proving_key =
                        ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                            &*key_bytes,
                        )
                        .expect("Could not deserialise deposit_proving_key");
                    return Arc::new(deposit_proving_key);
                }
                warn!("Could not load deposit_proving_key from local file. Loading from server");
            } else {
                warn!(
                    "Could not find local deposit_proving_key at {}. Loading from server",
                    path.display()
                );
            }
        } else {
            warn!("Configuration keys path not found. Loading deposit_proving_key from server");
        }

        if let Some(key_bytes) = load_key_from_server("deposit_proving_key") {
            let pk = ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                &*key_bytes,
            )
            .expect("Could not deserialise proving key");
            return Arc::new(pk);
        }
        panic!("Failed to load deposit_proving_key from both local and server");
    })
}
