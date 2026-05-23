use crate::domain::entities::L1Ref;
use configuration::settings::get_settings;
use tokio::sync::{OnceCell, RwLock};

pub(super) async fn get_listener_start_block() -> &'static RwLock<usize> {
    static LISTENER_START_BLOCK: OnceCell<RwLock<usize>> = OnceCell::const_new();
    LISTENER_START_BLOCK
        .get_or_init(|| async { RwLock::new(get_settings().genesis_block) })
        .await
}

pub(super) async fn get_listener_resume_cursor() -> &'static RwLock<Option<L1Ref>> {
    static LISTENER_RESUME_CURSOR: OnceCell<RwLock<Option<L1Ref>>> = OnceCell::const_new();
    LISTENER_RESUME_CURSOR
        .get_or_init(|| async { RwLock::new(None) })
        .await
}

pub async fn get_runtime_listener_start_block() -> usize {
    *get_listener_start_block().await.read().await
}

pub async fn set_runtime_listener_start_block(start_block: usize) {
    *get_listener_start_block().await.write().await = start_block;
}

pub async fn get_runtime_listener_resume_cursor() -> Option<L1Ref> {
    get_listener_resume_cursor().await.read().await.clone()
}

pub async fn set_runtime_listener_resume_cursor(cursor: Option<L1Ref>) {
    *get_listener_resume_cursor().await.write().await = cursor;
}
