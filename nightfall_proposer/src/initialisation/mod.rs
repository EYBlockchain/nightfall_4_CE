use super::driven::block_assembler::SmartTrigger;
use crate::{
    driven::block_assembler::BlockAssemblyStatus,
    ports::block_assembly_trigger::BlockAssemblyTrigger,
};
use ark_std::sync::Arc;
use configuration::settings::{get_settings, WalletRole};
use lib::{
    blockchain_client::BlockchainClientConnection, nf_client_proof::Proof, wallets::LocalWsClient,
};
use mongodb::Client;
use tokio::sync::{OnceCell, RwLock};

mod bootstrap;
mod cleanup;
mod consistency;
mod db;
mod runtime_listener;

pub use bootstrap::bootstrap_proposer_startup_state;
pub(crate) use cleanup::clear_all_reserved_deposits;
pub(crate) use consistency::validate_live_proposer_state_consistency;
pub use runtime_listener::{
    get_runtime_listener_resume_cursor, get_runtime_listener_start_block,
    set_runtime_listener_resume_cursor, set_runtime_listener_start_block,
};

use bootstrap::{bootstrap_proposer_startup_state_with_db, recover_then_initialize_proposer_db};
use db::ensure_proposer_db_initialized;

#[cfg(test)]
mod tests;

async fn get_raw_db_connection() -> &'static Client {
    static RAW_DB_CONNECTION: OnceCell<Client> = OnceCell::const_new();
    RAW_DB_CONNECTION
        .get_or_init(|| async {
            let uri = &get_settings().nightfall_proposer.db_url;
            Client::with_uri_str(uri)
                .await
                .expect("Could not create database connection")
        })
        .await
}

async fn ensure_singleton_proposer_db_initialized(client: &Client) {
    static DB_INITIALIZED: OnceCell<()> = OnceCell::const_new();
    DB_INITIALIZED
        .get_or_init(|| async {
            ensure_proposer_db_initialized(client).await;
        })
        .await;
}

/// This function is used to provide a singleton database connection across the entire application.
pub async fn get_db_connection() -> &'static Client {
    let client = get_raw_db_connection().await;
    ensure_singleton_proposer_db_initialized(client).await;
    client
}

/// This function is used to provide a singleton blockchain client connection across the entire application.
pub async fn get_blockchain_client_connection() -> &'static RwLock<LocalWsClient> {
    static BLOCKCHAIN_CLIENT_CONNECTION: OnceCell<RwLock<LocalWsClient>> = OnceCell::const_new();
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
    static BLOCK_ASSEMBLY_TRIGGER: OnceCell<Arc<RwLock<dyn BlockAssemblyTrigger + Send + Sync>>> =
        OnceCell::const_new();
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
