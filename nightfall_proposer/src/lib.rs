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
        // We'll try to load from the configuration directory first.
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

pub mod initialisation {

    use super::driven::block_assembler::SmartTrigger;
    use crate::{
        domain::entities::SyncState,
        driven::block_assembler::BlockAssemblyStatus,
        driven::db::{mongo_db::StoredBlock, snapshot::recover_from_restore_journal},
        driven::nightfall_event::get_expected_layer2_blocknumber,
        drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
        ports::{
            block_assembly_trigger::BlockAssemblyTrigger,
            contracts::NightfallContract,
            db::{BlockStorageDB, RestoreJournalDB, SyncStateDB},
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
        services::snapshot_scheduler::initialize_snapshot_scheduler_state,
    };
    use alloy::primitives::I256;
    use ark_bn254::Fr as Fr254;
    use ark_ff::Zero;
    use ark_std::sync::Arc;
    use configuration::settings::{get_settings, WalletRole};
    use lib::{
        blockchain_client::BlockchainClientConnection, nf_client_proof::Proof,
        wallets::LocalWsClient,
    };
    use lib::{hex_conversion::HexConvertible, merkle_trees::trees::MutableTree};
    use log::{info, warn};
    use mongodb::Client;
    use tokio::sync::{OnceCell, RwLock};

    async fn get_listener_start_block() -> &'static RwLock<usize> {
        static LISTENER_START_BLOCK: OnceCell<RwLock<usize>> = OnceCell::const_new();
        LISTENER_START_BLOCK
            .get_or_init(|| async { RwLock::new(get_settings().genesis_block) })
            .await
    }

    pub async fn get_runtime_listener_start_block() -> usize {
        *get_listener_start_block().await.read().await
    }

    pub async fn set_runtime_listener_start_block(start_block: usize) {
        *get_listener_start_block().await.write().await = start_block;
    }

    async fn ensure_commitment_tree_initialized(client: &Client) {
        if <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
            .await
            .is_ok()
        {
            return;
        }

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
            .await
            .expect("Could not create commitment tree");
    }

    async fn ensure_nullifier_tree_initialized(client: &Client) {
        if <mongodb::Client as MutableTree<Fr254>>::get_root(
            client,
            <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME,
        )
        .await
        .is_ok()
        {
            return;
        }

        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
            .await
            .expect("Could not create nullifier tree");
    }

    async fn ensure_historic_root_tree_initialized(client: &Client) {
        let zero_leaf = Fr254::from(0u8);
        let root = match <mongodb::Client as MutableTree<Fr254>>::get_root(
            client,
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await
        {
            Ok(root) => root,
            Err(_) => {
                <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
                    .await
                    .expect("Could not create historic root tree");
                Fr254::zero()
            }
        };

        let has_zero_leaf =
            <mongodb::Client as HistoricRootTree<Fr254>>::is_historic_root(client, &zero_leaf)
                .await
                .expect("Could not query historic root tree");

        if has_zero_leaf {
            return;
        }

        if !root.is_zero() {
            panic!("Historic root tree exists without zero leaf in a non-empty state");
        }

        <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client, &zero_leaf, true,
        )
        .await
        .expect("Couldn't insert zero leaf into the historic root tree");
    }

    async fn ensure_proposer_db_initialized(client: &Client) {
        ensure_commitment_tree_initialized(client).await;
        ensure_historic_root_tree_initialized(client).await;
        ensure_nullifier_tree_initialized(client).await;
        crate::driven::db::mongo_db::ensure_transfer_receipt_indexes(client)
            .await
            .expect("Could not create transfer receipt indexes");
    }

    fn missing_stored_block_error(last_applied_l2_block: u64) -> String {
        format!(
            "Proposer startup aborted: sync_state references L2 block {last_applied_l2_block}, \
             but StoredBlock at height {last_applied_l2_block} is missing. Local proposer state \
             is inconsistent. Manual intervention is required before restart."
        )
    }

    fn fingerprint_mismatch_error(
        last_applied_l2_block: u64,
        stored_fingerprint: &str,
        sync_state_fingerprint: &str,
    ) -> String {
        format!(
            "Proposer startup aborted: sync_state references L2 block {last_applied_l2_block}, \
             but StoredBlock fingerprint ({stored_fingerprint}) does not match sync_state \
             fingerprint ({sync_state_fingerprint}). Local proposer state is inconsistent. \
             Manual intervention is required before restart."
        )
    }

    fn unsupported_sync_state_schema_error(schema_version: u32) -> String {
        format!(
            "Proposer startup aborted: sync_state uses unsupported schema version \
             {schema_version}. Manual intervention is required before restart."
        )
    }

    fn ahead_of_chain_error(next_expected_block: u64, onchain_next_block: u64) -> String {
        format!(
            "Proposer startup aborted: local proposer state expects next L2 block \
             {next_expected_block}, but chain reports {onchain_next_block}. Local state is ahead \
             of chain. This may indicate an L1 reorg, chain rollback, or dev chain reset. \
             Manual recovery is required before restart."
        )
    }

    fn validate_sync_state_against_block(
        sync_state: &SyncState,
        stored_block: &StoredBlock,
    ) -> Result<(), String> {
        let stored_fingerprint = stored_block.hash().to_hex_string();
        if stored_fingerprint != sync_state.fingerprint {
            return Err(fingerprint_mismatch_error(
                sync_state.last_applied_l2_block,
                &stored_fingerprint,
                &sync_state.fingerprint,
            ));
        }

        if sync_state.schema_version != SyncState::SCHEMA_VERSION {
            return Err(unsupported_sync_state_schema_error(
                sync_state.schema_version,
            ));
        }

        Ok(())
    }

    async fn bootstrap_proposer_startup_state_with_db<N>(
        db: &Client,
        initialize_snapshot_scheduler: bool,
    ) -> Result<(), String>
    where
        N: NightfallContract,
    {
        if let Err(error) = recover_from_restore_journal(db).await {
            if db.get_restore_journal().await.is_none() {
                warn!(
                    "Proposer restore recovery completed by rolling back incomplete restore state before bootstrap: {error}"
                );
            } else {
                return Err(format!(
                    "Proposer restore recovery failed before bootstrap: {error}"
                ));
            }
        }
        if initialize_snapshot_scheduler {
            if let Err(error) = initialize_snapshot_scheduler_state().await {
                warn!("Could not initialize proposer snapshot scheduler state: {error}");
            }
        }

        let onchain_next_block_i256 = N::get_current_layer2_blocknumber()
            .await
            .map_err(|e| format!("Could not fetch current L2 block number: {e}"))?;

        if onchain_next_block_i256 < I256::ZERO {
            return Err(format!(
                "Nightfall returned negative current L2 block number {onchain_next_block_i256}"
            ));
        }

        let onchain_next_block: u64 = onchain_next_block_i256
            .try_into()
            .map_err(|_| "Current L2 block number does not fit into u64".to_string())?;

        let mut expected_block_number = get_expected_layer2_blocknumber().await.write().await;
        let mut sync_status = get_synchronisation_status().await.write().await;
        let mut listener_start_block = get_listener_start_block().await.write().await;

        *listener_start_block = get_settings().genesis_block;

        match db.get_sync_state().await {
            Some(sync_state) => {
                let stored_block = db
                    .get_block_by_number(sync_state.last_applied_l2_block)
                    .await
                    .ok_or_else(|| missing_stored_block_error(sync_state.last_applied_l2_block))?;

                validate_sync_state_against_block(&sync_state, &stored_block)?;

                let next_expected_block = sync_state
                    .last_applied_l2_block
                    .checked_add(1)
                    .ok_or_else(|| {
                        "last_applied_l2_block overflowed when computing next expected block"
                            .to_string()
                    })?;

                if next_expected_block > onchain_next_block {
                    return Err(ahead_of_chain_error(
                        next_expected_block,
                        onchain_next_block,
                    ));
                }

                *expected_block_number = I256::try_from(next_expected_block).map_err(|_| {
                    "Next expected L2 block number does not fit into I256".to_string()
                })?;

                let sync_state_l1_block_number: usize =
                    sync_state.l1_ref.block_number.try_into().map_err(|_| {
                        format!(
                            "sync_state L1 block number {} does not fit into usize",
                            sync_state.l1_ref.block_number
                        )
                    })?;
                *listener_start_block = sync_state_l1_block_number;

                if next_expected_block == onchain_next_block {
                    sync_status.set_synchronised();
                    info!(
                        "Recovered proposer state at L2 tip {} from sync_state; listener will resume from L1 block {} to safely skip already applied events if replayed",
                        sync_state.last_applied_l2_block,
                        sync_state.l1_ref.block_number
                    );
                } else {
                    sync_status.clear_synchronised();
                    info!(
                        "Recovered proposer state at L2 block {}, behind chain tip {}; listener replay will start from L1 block {}",
                        sync_state.last_applied_l2_block,
                        onchain_next_block.saturating_sub(1),
                        sync_state.l1_ref.block_number
                    );
                }
            }
            None => {
                *expected_block_number = I256::ZERO;
                if onchain_next_block == 0 {
                    sync_status.set_synchronised();
                    info!("No proposer sync_state found and chain is at genesis; starting fresh");
                } else {
                    sync_status.clear_synchronised();
                    warn!(
                        "No proposer sync_state found while chain is already at L2 block {}; replay is required",
                        onchain_next_block.saturating_sub(1)
                    );
                }
            }
        }

        Ok(())
    }

    pub async fn bootstrap_proposer_startup_state<N>() -> Result<(), String>
    where
        N: NightfallContract,
    {
        let db = get_db_connection().await;
        bootstrap_proposer_startup_state_with_db::<N>(db, true).await
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            domain::entities::{L1Ref, RestoreJournalPhase, RestoreJournalStep, SyncState},
            driven::db::{
                mongo_db::{StoredBlock, DB},
                snapshot::{create_proposer_snapshot, load_proposer_snapshot_into_shadow},
            },
            driven::nightfall_event::get_expected_layer2_blocknumber,
            drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
            ports::db::{BlockStorageDB, RestoreJournalDB, SyncStateDB},
        };
        use alloy::primitives::{Address, TxHash};
        use lib::hex_conversion::HexConvertible;
        use lib::tests_utils::{get_db_connection, get_mongo};
        use mongodb::bson::{doc, Document};
        use std::{fs, path::PathBuf};
        use tokio::sync::{Mutex, OnceCell};

        struct MockNightfallContract;

        fn mock_onchain_next_block() -> &'static std::sync::atomic::AtomicU64 {
            static ONCHAIN_NEXT_BLOCK: std::sync::OnceLock<std::sync::atomic::AtomicU64> =
                std::sync::OnceLock::new();
            ONCHAIN_NEXT_BLOCK.get_or_init(|| std::sync::atomic::AtomicU64::new(0))
        }

        impl MockNightfallContract {
            fn set_onchain_next_block(block_number: u64) {
                mock_onchain_next_block().store(block_number, std::sync::atomic::Ordering::SeqCst);
            }
        }

        #[async_trait::async_trait]
        impl NightfallContract for MockNightfallContract {
            async fn propose_block(
                _block: crate::domain::entities::Block,
            ) -> Result<(), lib::error::NightfallContractError> {
                unreachable!("propose_block is not used in bootstrap tests")
            }

            async fn get_current_layer2_blocknumber(
            ) -> Result<I256, lib::error::NightfallContractError> {
                Ok(I256::try_from(
                    mock_onchain_next_block().load(std::sync::atomic::Ordering::SeqCst),
                )
                .expect("mock block number fits into I256"))
            }
        }

        async fn bootstrap_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
            static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
            LOCK.get_or_init(|| async { Mutex::new(()) })
                .await
                .lock()
                .await
        }

        async fn persist_sync_state(client: &mongodb::Client, sync_state: &SyncState) {
            let mut session = client.start_session().await.expect("start session");
            let client = client.clone();
            let sync_state = sync_state.clone();
            session
                .start_transaction()
                .and_run2(async move |session| {
                    client
                        .update_sync_state_with_session(&sync_state, session)
                        .await?;
                    Ok::<(), mongodb::error::Error>(())
                })
                .await
                .expect("write sync_state");
        }

        async fn create_snapshot_fixture(
            client: &mongodb::Client,
            snapshot_root_prefix: &str,
            layer2_block_number: u64,
            l1_block_number: u64,
        ) -> (PathBuf, StoredBlock, SyncState) {
            ensure_proposer_db_initialized(client).await;

            let stored_block = StoredBlock {
                layer2_block_number,
                commitments: vec![format!("0x{layer2_block_number:02x}")],
                proposer_address: Address::from([layer2_block_number as u8; 20]),
            };
            client
                .store_block(&stored_block)
                .await
                .expect("store block for snapshot fixture");

            let sync_state = SyncState::new(
                stored_block.layer2_block_number,
                stored_block.hash().to_hex_string(),
                L1Ref {
                    block_number: l1_block_number,
                    tx_hash: TxHash::from([layer2_block_number as u8; 32]),
                    log_index: layer2_block_number,
                },
                mongodb::bson::DateTime::now(),
            );
            persist_sync_state(client, &sync_state).await;

            let snapshot_root = std::env::temp_dir().join(format!(
                "{snapshot_root_prefix}-{}",
                mongodb::bson::DateTime::now().timestamp_millis()
            ));
            create_proposer_snapshot(client, &snapshot_root)
                .await
                .expect("create snapshot fixture");

            (snapshot_root, stored_block, sync_state)
        }

        async fn set_live_sync_state(
            client: &mongodb::Client,
            layer2_block_number: u64,
            commitment_tag: &str,
            l1_block_number: u64,
        ) -> SyncState {
            let stored_block = StoredBlock {
                layer2_block_number,
                commitments: vec![commitment_tag.to_string()],
                proposer_address: Address::from([layer2_block_number as u8; 20]),
            };
            client
                .store_block(&stored_block)
                .await
                .expect("store live block");

            let sync_state = SyncState::new(
                stored_block.layer2_block_number,
                stored_block.hash().to_hex_string(),
                L1Ref {
                    block_number: l1_block_number,
                    tx_hash: TxHash::from([layer2_block_number as u8; 32]),
                    log_index: layer2_block_number,
                },
                mongodb::bson::DateTime::now(),
            );
            persist_sync_state(client, &sync_state).await;
            sync_state
        }

        fn snapshot_dir(snapshot_root: &PathBuf) -> PathBuf {
            fs::read_dir(snapshot_root)
                .expect("read snapshot root")
                .next()
                .expect("snapshot dir should exist")
                .expect("read snapshot dir entry")
                .path()
        }

        async fn reset_runtime_bootstrap_state() {
            *get_expected_layer2_blocknumber().await.write().await = I256::ZERO;
            get_synchronisation_status()
                .await
                .write()
                .await
                .clear_synchronised();
            set_runtime_listener_start_block(0).await;
        }

        async fn rename_collection(
            client: &mongodb::Client,
            from: &str,
            to: &str,
        ) -> Result<(), mongodb::error::Error> {
            client
                .database("admin")
                .run_command(doc! {
                    "renameCollection": format!("{DB}.{from}"),
                    "to": format!("{DB}.{to}"),
                    "dropTarget": false,
                })
                .await?;
            Ok(())
        }

        fn ordered_restore_collections(
            journal: &[crate::domain::entities::RestoreJournalCollection],
        ) -> Vec<crate::domain::entities::RestoreJournalCollection> {
            let mut ordered = journal.to_vec();
            ordered.sort_by(|left, right| {
                match (
                    left.live == crate::driven::db::mongo_db::SYNC_STATE_COLLECTION,
                    right.live == crate::driven::db::mongo_db::SYNC_STATE_COLLECTION,
                ) {
                    (false, true) => std::cmp::Ordering::Less,
                    (true, false) => std::cmp::Ordering::Greater,
                    _ => left.live.cmp(&right.live),
                }
            });
            ordered
        }

        #[tokio::test]
        async fn bootstrap_cleans_loading_shadow_restore_journal_and_keeps_live_state() {
            let _lock = bootstrap_test_lock().await;
            let container = get_mongo().await;
            let client = get_db_connection(&container).await;

            let (snapshot_root, _, _) =
                create_snapshot_fixture(&client, "nf4-bootstrap-loading-shadow", 8, 800).await;
            let snapshot_dir = snapshot_dir(&snapshot_root);

            let newer_live_sync_state = set_live_sync_state(&client, 9, "0xlive", 900).await;
            let journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
                .await
                .expect("load snapshot into shadow");
            assert_eq!(journal.phase, RestoreJournalPhase::LoadingShadow);

            MockNightfallContract::set_onchain_next_block(10);
            reset_runtime_bootstrap_state().await;

            bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
                .await
                .expect("bootstrap should clean loading shadow and continue");

            assert_eq!(client.get_restore_journal().await, None);
            assert_eq!(
                client.get_sync_state().await,
                Some(newer_live_sync_state.clone())
            );
            assert_eq!(
                *get_expected_layer2_blocknumber().await.read().await,
                I256::try_from(10_u64).expect("10 fits into I256")
            );
            assert_eq!(get_runtime_listener_start_block().await, 900);
            assert!(get_synchronisation_status()
                .await
                .read()
                .await
                .is_synchronised());

            let collection_names = client
                .database(DB)
                .list_collection_names()
                .await
                .expect("list collections");
            assert!(!collection_names
                .iter()
                .any(|name| name.starts_with("restore_shadow__")));
            assert!(!collection_names
                .iter()
                .any(|name| name.starts_with("restore_backup__")));

            tokio::fs::remove_dir_all(snapshot_root)
                .await
                .expect("cleanup snapshot directory");
        }

        #[tokio::test]
        async fn bootstrap_resumes_swap_in_progress_restore_and_marks_tip_synchronised() {
            let _lock = bootstrap_test_lock().await;
            let container = get_mongo().await;
            let client = get_db_connection(&container).await;

            let (snapshot_root, snapshot_block, _) =
                create_snapshot_fixture(&client, "nf4-bootstrap-swap-resume", 12, 1200).await;
            let snapshot_dir = snapshot_dir(&snapshot_root);

            set_live_sync_state(&client, 13, "0xnewer-live", 1300).await;
            let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
                .await
                .expect("load snapshot into shadow");
            let ordered = ordered_restore_collections(&journal.collections);
            rename_collection(&client, &ordered[0].live, &ordered[0].backup)
                .await
                .expect("simulate backup rename");
            journal.collections = ordered;
            journal.phase = RestoreJournalPhase::SwapInProgress;
            journal.current_index = Some(0);
            journal.current_step = Some(RestoreJournalStep::BackupCreated);
            journal.updated_at = mongodb::bson::DateTime::now();
            client
                .upsert_restore_journal(&journal)
                .await
                .expect("persist mid-swap journal");

            MockNightfallContract::set_onchain_next_block(snapshot_block.layer2_block_number + 1);
            reset_runtime_bootstrap_state().await;

            bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
                .await
                .expect("bootstrap should resume swap and continue");

            let resumed = client
                .get_restore_journal()
                .await
                .expect("swap_complete journal should remain for cleanup");
            assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);

            let live_sync_state = client
                .get_sync_state()
                .await
                .expect("restored sync_state should exist");
            assert_eq!(live_sync_state.last_applied_l2_block, 12);
            assert_eq!(live_sync_state.l1_ref.block_number, 1200);
            assert_eq!(
                *get_expected_layer2_blocknumber().await.read().await,
                I256::try_from(13_u64).expect("13 fits into I256")
            );
            assert_eq!(get_runtime_listener_start_block().await, 1200);
            assert!(get_synchronisation_status()
                .await
                .read()
                .await
                .is_synchronised());

            tokio::fs::remove_dir_all(snapshot_root)
                .await
                .expect("cleanup snapshot directory");
        }

        #[tokio::test]
        async fn bootstrap_keeps_resumed_restore_desynchronised_when_one_block_behind_tip() {
            let _lock = bootstrap_test_lock().await;
            let container = get_mongo().await;
            let client = get_db_connection(&container).await;

            let (snapshot_root, snapshot_block, _) =
                create_snapshot_fixture(&client, "nf4-bootstrap-one-behind", 14, 1400).await;
            let snapshot_dir = snapshot_dir(&snapshot_root);

            set_live_sync_state(&client, 15, "0xnewer-live", 1500).await;
            let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
                .await
                .expect("load snapshot into shadow");
            let ordered = ordered_restore_collections(&journal.collections);
            rename_collection(&client, &ordered[0].live, &ordered[0].backup)
                .await
                .expect("simulate backup rename");
            journal.collections = ordered;
            journal.phase = RestoreJournalPhase::SwapInProgress;
            journal.current_index = Some(0);
            journal.current_step = Some(RestoreJournalStep::BackupCreated);
            journal.updated_at = mongodb::bson::DateTime::now();
            client
                .upsert_restore_journal(&journal)
                .await
                .expect("persist mid-swap journal");

            MockNightfallContract::set_onchain_next_block(snapshot_block.layer2_block_number + 2);
            reset_runtime_bootstrap_state().await;

            bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
                .await
                .expect("bootstrap should resume swap and remain desynchronised");

            assert_eq!(
                *get_expected_layer2_blocknumber().await.read().await,
                I256::try_from(15_u64).expect("15 fits into I256")
            );
            assert_eq!(get_runtime_listener_start_block().await, 1400);
            assert!(!get_synchronisation_status()
                .await
                .read()
                .await
                .is_synchronised());

            tokio::fs::remove_dir_all(snapshot_root)
                .await
                .expect("cleanup snapshot directory");
        }

        #[tokio::test]
        async fn bootstrap_continues_after_restore_rollback_clears_the_journal() {
            let _lock = bootstrap_test_lock().await;
            let container = get_mongo().await;
            let client = get_db_connection(&container).await;

            let (snapshot_root, _, _) =
                create_snapshot_fixture(&client, "nf4-bootstrap-rollback", 20, 2000).await;
            let snapshot_dir = snapshot_dir(&snapshot_root);

            let newer_live_sync_state =
                set_live_sync_state(&client, 21, "0xnewer-live", 2100).await;
            let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
                .await
                .expect("load snapshot into shadow");
            let ordered = ordered_restore_collections(&journal.collections);
            rename_collection(&client, &ordered[0].live, &ordered[0].backup)
                .await
                .expect("rename first live to backup");
            rename_collection(&client, &ordered[0].shadow, &ordered[0].live)
                .await
                .expect("rename first shadow to live");
            journal.collections = ordered;
            journal.phase = RestoreJournalPhase::SwapInProgress;
            journal.current_index = Some(1);
            journal.current_step = Some(RestoreJournalStep::BackupPending);
            journal.updated_at = mongodb::bson::DateTime::now();
            client
                .upsert_restore_journal(&journal)
                .await
                .expect("persist rollback fixture journal");

            client
                .database(DB)
                .collection::<Document>(&journal.collections[1].live)
                .drop()
                .await
                .expect("drop live collection to force rollback");

            MockNightfallContract::set_onchain_next_block(22);
            reset_runtime_bootstrap_state().await;

            bootstrap_proposer_startup_state_with_db::<MockNightfallContract>(&client, false)
                .await
                .expect("bootstrap should continue after rollback recovery");

            assert_eq!(client.get_restore_journal().await, None);
            assert_eq!(
                client.get_sync_state().await,
                Some(newer_live_sync_state.clone())
            );
            assert_eq!(
                *get_expected_layer2_blocknumber().await.read().await,
                I256::try_from(22_u64).expect("22 fits into I256")
            );
            assert_eq!(get_runtime_listener_start_block().await, 2100);
            assert!(get_synchronisation_status()
                .await
                .read()
                .await
                .is_synchronised());

            tokio::fs::remove_dir_all(snapshot_root)
                .await
                .expect("cleanup snapshot directory");
        }
    }

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
                ensure_proposer_db_initialized(&client).await;
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
                    LocalWsClient::try_from_settings(get_settings(), WalletRole::Proposer)
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
