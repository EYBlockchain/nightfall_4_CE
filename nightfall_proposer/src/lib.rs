pub mod domain;
pub mod driven;
pub mod drivers;
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
use lib::utils::load_key_locally;
use lib::{rollup_circuit_checks::find_file_with_path, utils::load_key_from_server};
use log::warn;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, OnceLock, RwLock},
};
use ark_std::path::PathBuf;
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
        // We'll try to load from the configuration directory first.
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configuration/bin/keys/deposit_proving_key");
        let source_file = find_file_with_path(&path).unwrap();
        if let Some(_key_bytes) = load_key_locally(&source_file) {
            let deposit_proving_key =
                ProvingKey::<UnivariateKzgPCS<Bn254>>::deserialize_compressed_unchecked(
                    &*std::fs::read(source_file).expect("Could not read deposit_proving_key"),
                )
                .expect("Could not deserialise deposit_proving_key");
            return Arc::new(deposit_proving_key);
        }
        warn!("Could not load deposit_proving_key from local file. Loading from server");

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

pub mod initialisation {

    use super::driven::block_assembler::SmartTrigger;
    use crate::{
        driven::block_assembler::BlockAssemblyStatus,
        ports::{
            block_assembly_trigger::BlockAssemblyTrigger,
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
    };
    use ark_bn254::Fr as Fr254;
    use ark_std::sync::Arc;
    use configuration::settings::get_settings;
    use lib::{
        blockchain_client::BlockchainClientConnection, nf_client_proof::Proof,
        wallets::LocalWsClient,
    };
    use mongodb::Client;
    use tokio::sync::{OnceCell, RwLock};

    /// This function is used to provide a singleton database connection across the entire application.
    pub async fn get_db_connection() -> &'static Client {
        static DB_CONNECTION: OnceCell<Client> = OnceCell::const_new();
        DB_CONNECTION
            .get_or_init(|| async {
                // select the proposer to use
                let uri = &get_settings().nightfall_proposer.db_url;
                let client = Client::with_uri_str(uri)
                    .await
                    .expect("Could not create database connection");
                // it's not enough just to connect to a database, we need to initialise some trees in it
                <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
                    .await
                    .expect("Could not create commitment tree");
                <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
                    .await
                    .expect("Could not create historic root tree");
                <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
                    .await
                    .expect("Could not create historic root tree");

                <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
                    &client,
                    &Fr254::from(0u8),
                    true,
                )
                .await
                .expect("Couldn't insert zero leaf into the historic root tree");
                client
            })
            .await
    }

    /// This function is used to provide a singleton blockchain client connection across the entire application.
    pub async fn get_blockchain_client_connection() -> &'static RwLock<LocalWsClient> {
        static BLOCKCHAIN_CLIENT_CONNECTION: OnceCell<RwLock<LocalWsClient>> =
            OnceCell::const_new();
        BLOCKCHAIN_CLIENT_CONNECTION
            .get_or_init(|| async {
                RwLock::new(
                    LocalWsClient::try_from_settings(get_settings())
                        .await
                        .expect("Could not create blockchain client connection"),
                )
            })
            .await
    }

    /// This function is used to provide a singleton trigger for block assembly across the entire application.
    pub async fn get_block_assembly_trigger<P: Proof>(
    ) -> &'static Arc<RwLock<dyn BlockAssemblyTrigger + Send + Sync>> {
        static BLOCK_ASSEMBLY_TRIGGER: OnceCell<
            Arc<RwLock<dyn BlockAssemblyTrigger + Send + Sync>>,
        > = OnceCell::const_new();
        BLOCK_ASSEMBLY_TRIGGER
            .get_or_init(|| async {
                let status = get_block_assembly_status().await;
                let db_client = get_db_connection().await;
                let settings = get_settings();
                let initial_interval_secs = settings
                    .nightfall_proposer
                    .block_assembly_initial_interval_secs;
                let max_wait_secs = settings.nightfall_proposer.block_assembly_max_wait_secs;
                let target_fill_ratio =
                    settings.nightfall_proposer.block_assembly_target_fill_ratio as f32;

                let smart_trigger = SmartTrigger::<P>::new(
                    initial_interval_secs,
                    max_wait_secs,
                    status,
                    db_client,
                    target_fill_ratio,
                );
                Arc::new(RwLock::new(smart_trigger))
                    as Arc<RwLock<dyn BlockAssemblyTrigger + Send + Sync>>
            })
            .await
    }

    /// This function is used to provide a singleton status for the BlockAssemblyTrigger across the entire application.
    pub async fn get_block_assembly_status() -> &'static RwLock<BlockAssemblyStatus> {
        static BLOCK_ASSEMBLY_STATUS: OnceCell<RwLock<BlockAssemblyStatus>> = OnceCell::const_new();
        BLOCK_ASSEMBLY_STATUS
            .get_or_init(|| async { RwLock::new(BlockAssemblyStatus::new()) })
            .await
    }
}
