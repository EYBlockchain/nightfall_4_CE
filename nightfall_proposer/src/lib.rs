pub mod domain;
pub mod driven;
pub mod drivers;
pub mod ports;
pub mod services;

use ark_bn254::Fr as Fr254;
use jf_primitives::{
    poseidon::Poseidon,
    trees::{
        imt::{IndexedMerkleTree, LeafDBEntry},
        timber::Timber,
    },
};
use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
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
            db::{BlockStorageDB, SyncStateDB},
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
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

    pub async fn bootstrap_proposer_startup_state<N>() -> Result<(), String>
    where
        N: NightfallContract,
    {
        let db = get_db_connection().await;
        recover_from_restore_journal(db)
            .await
            .map_err(|e| format!("Proposer restore recovery failed before bootstrap: {e}"))?;

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
