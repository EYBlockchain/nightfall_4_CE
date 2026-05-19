use crate::driven::db::snapshot::{
    cleanup_orphaned_proposer_snapshot_temp_dirs, create_proposer_snapshot,
    find_latest_valid_proposer_snapshot, SnapshotError,
};
use configuration::settings::get_settings;
use log::{debug, info, warn};
use mongodb::Client;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::{
    fs,
    sync::{OnceCell, RwLock},
};

static SNAPSHOT_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

async fn get_last_snapshot_l2_block() -> &'static RwLock<u64> {
    static LAST_SNAPSHOT_L2_BLOCK: OnceCell<RwLock<u64>> = OnceCell::const_new();
    LAST_SNAPSHOT_L2_BLOCK
        .get_or_init(|| async { RwLock::new(0) })
        .await
}

struct SnapshotTaskGuard;

impl SnapshotTaskGuard {
    fn try_acquire() -> Option<Self> {
        SNAPSHOT_IN_PROGRESS
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .ok()
            .map(|_| Self)
    }
}

impl Drop for SnapshotTaskGuard {
    fn drop(&mut self) {
        SNAPSHOT_IN_PROGRESS.store(false, Ordering::Release);
    }
}

fn snapshot_root_dir() -> PathBuf {
    PathBuf::from(&get_settings().nightfall_proposer.snapshot_root_dir)
}

async fn initialize_snapshot_scheduler_state_for_root(root: &Path) -> Result<(), SnapshotError> {
    cleanup_orphaned_proposer_snapshot_temp_dirs(root).await?;
    let latest_snapshot_l2_block = match find_latest_valid_proposer_snapshot(root, u64::MAX).await?
    {
        Some((_, manifest)) => manifest.last_applied_l2_block,
        None => 0,
    };

    *get_last_snapshot_l2_block().await.write().await = latest_snapshot_l2_block;
    Ok(())
}

pub fn maybe_should_snapshot(
    current_l2_block: u64,
    last_snapshot_l2_block: u64,
    current_l1_block: u64,
    applied_block_l1_block: u64,
    snapshot_interval_l2_blocks: u64,
    snapshot_min_l1_confirmations: u64,
) -> bool {
    current_l2_block.saturating_sub(last_snapshot_l2_block) >= snapshot_interval_l2_blocks
        && current_l1_block.saturating_sub(applied_block_l1_block) >= snapshot_min_l1_confirmations
}

pub async fn initialize_snapshot_scheduler_state() -> Result<(), SnapshotError> {
    initialize_snapshot_scheduler_state_for_root(&snapshot_root_dir()).await
}

async fn prune_old_snapshots_by_manifest(
    snapshot_root_dir: &Path,
    retention_count: usize,
) -> Result<(), SnapshotError> {
    if retention_count == 0 || !fs::try_exists(snapshot_root_dir).await? {
        return Ok(());
    }

    let mut entries = fs::read_dir(snapshot_root_dir).await?;
    let mut snapshots = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let path = entry.path();
        let manifest_path = path.join("manifest.json");
        if !fs::try_exists(&manifest_path).await? {
            continue;
        }

        let manifest_bytes = match fs::read(&manifest_path).await {
            Ok(bytes) => bytes,
            Err(error) => {
                warn!(
                    "Skipping snapshot directory {} during prune: could not read manifest: {}",
                    path.display(),
                    error
                );
                continue;
            }
        };
        let manifest: crate::domain::entities::ProposerSnapshotManifest =
            match serde_json::from_slice(&manifest_bytes) {
                Ok(manifest) => manifest,
                Err(error) => {
                    warn!(
                        "Skipping snapshot directory {} during prune: manifest is invalid JSON: {}",
                        path.display(),
                        error
                    );
                    continue;
                }
            };
        snapshots.push((path, manifest.last_applied_l2_block, manifest.snapshot_id));
    }

    snapshots.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| right.2.cmp(&left.2)));

    for (path, _, _) in snapshots.into_iter().skip(retention_count) {
        fs::remove_dir_all(&path).await?;
    }

    Ok(())
}

async fn create_snapshot_task(client: Client, snapshot_root_dir: PathBuf, retention_count: usize) {
    let _guard = match SnapshotTaskGuard::try_acquire() {
        Some(guard) => guard,
        None => {
            debug!("Skipping proposer snapshot creation because another snapshot is already in progress");
            return;
        }
    };

    match create_proposer_snapshot(&client, &snapshot_root_dir).await {
        Ok(manifest) => {
            *get_last_snapshot_l2_block().await.write().await = manifest.last_applied_l2_block;

            if let Err(error) =
                prune_old_snapshots_by_manifest(&snapshot_root_dir, retention_count).await
            {
                warn!(
                    "Proposer snapshot {} created, but pruning old snapshots failed: {}",
                    manifest.snapshot_id, error
                );
            } else {
                info!(
                    "Created proposer snapshot {} at L2 block {}",
                    manifest.snapshot_id, manifest.last_applied_l2_block
                );
            }
        }
        Err(error) => {
            warn!("Automatic proposer snapshot creation failed: {error}");
        }
    }
}

pub async fn maybe_schedule_snapshot_for_applied_block(
    client: &Client,
    current_l2_block: u64,
    current_l1_block: u64,
    applied_block_l1_block: u64,
) {
    let settings = &get_settings().nightfall_proposer;
    if !settings.snapshot_enabled {
        return;
    }

    let last_snapshot_l2_block = *get_last_snapshot_l2_block().await.read().await;
    if !maybe_should_snapshot(
        current_l2_block,
        last_snapshot_l2_block,
        current_l1_block,
        applied_block_l1_block,
        settings.snapshot_interval_l2_blocks,
        settings.snapshot_min_l1_confirmations,
    ) {
        return;
    }

    let client = client.clone();
    let snapshot_root_dir = snapshot_root_dir();
    let retention_count = settings.snapshot_retention_count as usize;

    tokio::spawn(async move {
        create_snapshot_task(client, snapshot_root_dir, retention_count).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::{Mutex, OnceCell as TokioOnceCell};

    async fn scheduler_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
        static LOCK: TokioOnceCell<Mutex<()>> = TokioOnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) })
            .await
            .lock()
            .await
    }
    use mongodb::bson::DateTime;

    #[test]
    fn maybe_should_snapshot_requires_interval_and_confirmations() {
        assert!(maybe_should_snapshot(200, 100, 500, 480, 100, 12));
        assert!(!maybe_should_snapshot(199, 100, 500, 480, 100, 12));
        assert!(!maybe_should_snapshot(200, 100, 491, 480, 100, 12));
    }

    #[tokio::test]
    async fn prune_old_snapshots_keeps_only_most_recent_snapshots() {
        let _scheduler_test_lock = scheduler_test_lock().await;
        let root = std::env::temp_dir().join(format!(
            "nf4-proposer-prune-test-{}",
            DateTime::now().timestamp_millis()
        ));
        fs::create_dir_all(&root).await.expect("create root");

        for index in 0..7_u64 {
            let snapshot_dir = root.join(format!("snapshot-{index}"));
            fs::create_dir_all(&snapshot_dir)
                .await
                .expect("create snapshot dir");
            let manifest = crate::domain::entities::ProposerSnapshotManifest {
                snapshot_id: format!("snapshot-{index}"),
                schema_version: crate::domain::entities::ProposerSnapshotManifest::SCHEMA_VERSION,
                created_at: DateTime::now(),
                storage_format: crate::domain::entities::ProposerSnapshotManifest::STORAGE_FORMAT
                    .to_string(),
                database: "proposer".to_string(),
                last_applied_l2_block: index,
                fingerprint: format!("fingerprint-{index}"),
                l1_ref: crate::domain::entities::L1Ref {
                    block_number: index,
                    tx_hash: alloy::primitives::TxHash::from([index as u8; 32]),
                    log_index: 0,
                },
                collections: Vec::new(),
                overall_sha256: format!("sha-{index}"),
            };
            fs::write(
                snapshot_dir.join("manifest.json"),
                serde_json::to_vec(&manifest).expect("serialize manifest"),
            )
            .await
            .expect("write manifest");
        }

        prune_old_snapshots_by_manifest(&root, 5)
            .await
            .expect("prune old snapshots");

        let mut remaining = fs::read_dir(&root).await.expect("list root");
        let mut names = Vec::new();
        while let Some(entry) = remaining.next_entry().await.expect("read entry") {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
        names.sort();

        assert_eq!(
            names,
            vec![
                "snapshot-2".to_string(),
                "snapshot-3".to_string(),
                "snapshot-4".to_string(),
                "snapshot-5".to_string(),
                "snapshot-6".to_string(),
            ]
        );

        fs::remove_dir_all(root).await.expect("cleanup prune root");
    }

    #[tokio::test]
    async fn initialize_snapshot_scheduler_state_cleans_orphan_temp_dirs_and_tracks_latest() {
        let _scheduler_test_lock = scheduler_test_lock().await;
        let root = std::env::temp_dir().join(format!(
            "nf4-proposer-scheduler-init-test-{}",
            DateTime::now().timestamp_millis()
        ));
        fs::create_dir_all(&root).await.expect("create root");

        fs::create_dir_all(root.join(".tmp-proposer-snapshot-orphan"))
            .await
            .expect("create orphan temp dir");

        for index in [3_u64, 9_u64] {
            let snapshot_dir = root.join(format!("snapshot-{index}"));
            fs::create_dir_all(&snapshot_dir)
                .await
                .expect("create snapshot dir");
            let manifest = crate::domain::entities::ProposerSnapshotManifest {
                snapshot_id: format!("snapshot-{index}"),
                schema_version: crate::domain::entities::ProposerSnapshotManifest::SCHEMA_VERSION,
                created_at: DateTime::now(),
                storage_format: crate::domain::entities::ProposerSnapshotManifest::STORAGE_FORMAT
                    .to_string(),
                database: "proposer".to_string(),
                last_applied_l2_block: index,
                fingerprint: format!("fingerprint-{index}"),
                l1_ref: crate::domain::entities::L1Ref {
                    block_number: index,
                    tx_hash: alloy::primitives::TxHash::from([index as u8; 32]),
                    log_index: 0,
                },
                collections: Vec::new(),
                overall_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            };
            fs::write(
                snapshot_dir.join("manifest.json"),
                serde_json::to_vec(&manifest).expect("serialize manifest"),
            )
            .await
            .expect("write manifest");
        }

        initialize_snapshot_scheduler_state_for_root(&root)
            .await
            .expect("initialize scheduler state");

        let names: Vec<String> = {
            let mut entries = fs::read_dir(&root).await.expect("read root");
            let mut names = Vec::new();
            while let Some(entry) = entries.next_entry().await.expect("read entry") {
                names.push(entry.file_name().to_string_lossy().to_string());
            }
            names
        };
        assert!(!names
            .iter()
            .any(|name| name.starts_with(".tmp-proposer-snapshot-")));
        assert_eq!(*get_last_snapshot_l2_block().await.read().await, 9);

        fs::remove_dir_all(root)
            .await
            .expect("cleanup scheduler init root");
    }
}
