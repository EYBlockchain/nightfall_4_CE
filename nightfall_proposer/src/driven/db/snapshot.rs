use crate::{
    domain::entities::{
        ProposerSnapshotManifest, RestoreJournal, RestoreJournalCollection, RestoreJournalPhase,
        RestoreJournalStep, SnapshotCollectionManifest, SyncState,
    },
    driven::db::client_transaction_state::{
        remove_all_mempool_client_transactions_with_session,
        restore_selected_transactions_to_mempool_after_block_with_session,
    },
    driven::db::mongo_db::{
        ensure_deposit_indexes, StoredBlock, DB, DEPOSIT_COLLECTION, PROPOSED_BLOCKS_COLLECTION,
        RESTORE_JOURNAL_COLLECTION, SYNC_STATE_COLLECTION,
    },
    initialisation::{
        validate_live_proposer_state_consistency,
        validate_snapshotted_live_proposer_state_consistency,
    },
    ports::db::{PendingBlockDB, RestoreJournalDB, SyncStateDB},
    ports::trees::{CommitmentTree, HistoricRootTree, NullifierTree},
};
use ark_bn254::Fr as Fr254;
use lib::hex_conversion::HexConvertible;
use lib::merkle_trees::trees::TreeMetadata;
use log::{debug, error, warn};
use mongodb::bson::{Bson, Document};
use mongodb::options::ReadConcern;
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    process,
    sync::OnceLock,
};
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

const TEMP_SNAPSHOT_DIR_PREFIX: &str = ".tmp-proposer-snapshot-";
static PROPOSER_STATE_MAINTENANCE_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

#[cfg(test)]
use std::{collections::HashSet, sync::Mutex};

#[cfg(test)]
static ENABLED_FAILPOINTS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
#[cfg(test)]
static SNAPSHOT_TEST_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

#[cfg(test)]
fn enabled_failpoints() -> &'static Mutex<HashSet<String>> {
    ENABLED_FAILPOINTS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn maybe_crash_at_failpoint(_name: &str) {
    #[cfg(test)]
    {
        if enabled_failpoints()
            .lock()
            .expect("failpoint lock poisoned")
            .contains(_name)
        {
            panic!("simulated crash at failpoint {_name}");
        }
    }
}

fn expected_historic_root_sub_tree_count(last_applied_l2_block: u64) -> Result<u64, SnapshotError> {
    last_applied_l2_block.checked_add(2).ok_or_else(|| {
        SnapshotError::InvalidSnapshotSyncState(
            "sync_state last_applied_l2_block overflowed while validating historic roots"
                .to_string(),
        )
    })
}

fn proposer_state_maintenance_lock() -> &'static tokio::sync::Mutex<()> {
    PROPOSER_STATE_MAINTENANCE_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

#[cfg(test)]
pub(crate) async fn snapshot_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
    SNAPSHOT_TEST_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await
}

pub(crate) fn try_acquire_proposer_state_maintenance_guard(
) -> Option<tokio::sync::MutexGuard<'static, ()>> {
    proposer_state_maintenance_lock().try_lock().ok()
}

pub(crate) async fn acquire_proposer_state_maintenance_guard(
) -> tokio::sync::MutexGuard<'static, ()> {
    proposer_state_maintenance_lock().lock().await
}

#[cfg(test)]
struct TestFailpointGuard {
    name: String,
}

#[cfg(test)]
impl TestFailpointGuard {
    fn enable(name: &str) -> Self {
        enabled_failpoints()
            .lock()
            .expect("failpoint lock poisoned")
            .insert(name.to_string());
        Self {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
impl Drop for TestFailpointGuard {
    fn drop(&mut self) {
        enabled_failpoints()
            .lock()
            .expect("failpoint lock poisoned")
            .remove(&self.name);
    }
}

#[derive(Debug)]
pub enum SnapshotError {
    Io(std::io::Error),
    Mongo(mongodb::error::Error),
    SerdeJson(serde_json::Error),
    MissingSyncState,
    MissingSnapshotCollection(String),
    InvalidSnapshotSyncState(String),
    MissingRestoreJournal,
    UnsupportedManifestSchemaVersion(u32),
    UnsupportedManifestStorageFormat(String),
    UnexpectedSnapshotCollection(String),
    DuplicateSnapshotCollection(String),
    InvalidSnapshotFileName(String),
    MissingSnapshotFile(String),
    UnexpectedRestoreJournalPhase {
        expected: String,
        actual: String,
    },
    RestoreInvariantViolation(String),
    CollectionChecksumMismatch {
        collection_name: String,
        expected: String,
        actual: String,
    },
    OverallChecksumMismatch {
        expected: String,
        actual: String,
    },
    DocumentCountMismatch {
        collection_name: String,
        expected: u64,
        actual: u64,
    },
    SnapshotManifestSyncStateMismatch {
        field: &'static str,
        manifest_value: String,
        snapshotted_value: String,
    },
}

impl Display for SnapshotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error while creating snapshot: {error}"),
            Self::Mongo(error) => write!(f, "MongoDB error while creating snapshot: {error}"),
            Self::SerdeJson(error) => {
                write!(f, "Serialization error while creating snapshot: {error}")
            }
            Self::MissingSyncState => write!(
                f,
                "Cannot create proposer snapshot without a persisted sync_state"
            ),
            Self::MissingSnapshotCollection(collection_name) => write!(
                f,
                "Cannot create proposer snapshot because canonical collection {collection_name} is missing"
            ),
            Self::InvalidSnapshotSyncState(message) => {
                write!(f, "Snapshot sync_state is invalid: {message}")
            }
            Self::MissingRestoreJournal => {
                write!(f, "Cannot continue restore flow without a persisted restore_journal")
            }
            Self::UnsupportedManifestSchemaVersion(schema_version) => write!(
                f,
                "Unsupported proposer snapshot manifest schema version: {schema_version}"
            ),
            Self::UnsupportedManifestStorageFormat(storage_format) => write!(
                f,
                "Unsupported proposer snapshot storage format: {storage_format}"
            ),
            Self::UnexpectedSnapshotCollection(collection_name) => write!(
                f,
                "Snapshot manifest contains unsupported proposer collection: {collection_name}"
            ),
            Self::DuplicateSnapshotCollection(collection_name) => write!(
                f,
                "Snapshot manifest contains duplicate proposer collection entry: {collection_name}"
            ),
            Self::InvalidSnapshotFileName(file_name) => write!(
                f,
                "Snapshot manifest contains invalid collection file name: {file_name}"
            ),
            Self::MissingSnapshotFile(file_name) => {
                write!(f, "Snapshot file is missing from snapshot directory: {file_name}")
            }
            Self::UnexpectedRestoreJournalPhase { expected, actual } => write!(
                f,
                "Unexpected restore_journal phase: expected {expected}, got {actual}"
            ),
            Self::RestoreInvariantViolation(message) => {
                write!(f, "Restore invariant violated: {message}")
            }
            Self::CollectionChecksumMismatch {
                collection_name,
                expected,
                actual,
            } => write!(
                f,
                "Snapshot collection checksum mismatch for {collection_name}: expected {expected}, got {actual}"
            ),
            Self::OverallChecksumMismatch { expected, actual } => write!(
                f,
                "Snapshot overall checksum mismatch: expected {expected}, got {actual}"
            ),
            Self::DocumentCountMismatch {
                collection_name,
                expected,
                actual,
            } => write!(
                f,
                "Snapshot document count mismatch for {collection_name}: expected {expected}, got {actual}"
            ),
            Self::SnapshotManifestSyncStateMismatch {
                field,
                manifest_value,
                snapshotted_value,
            } => write!(
                f,
                "Snapshot manifest field {field} does not match snapshotted sync_state: manifest={manifest_value}, snapshot={snapshotted_value}"
            ),
        }
    }
}

impl Error for SnapshotError {}

impl From<std::io::Error> for SnapshotError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<mongodb::error::Error> for SnapshotError {
    fn from(error: mongodb::error::Error) -> Self {
        Self::Mongo(error)
    }
}

impl From<serde_json::Error> for SnapshotError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}

fn snapshot_error_to_mongo(error: SnapshotError) -> mongodb::error::Error {
    match error {
        SnapshotError::Mongo(error) => error,
        other => mongodb::error::Error::custom(other.to_string()),
    }
}

fn mutable_tree_collection_names(tree_name: &str) -> [String; 3] {
    [
        format!("{tree_name}_metadata"),
        format!("{tree_name}_nodes"),
        format!("{tree_name}_cache"),
    ]
}

fn proposer_snapshot_collection_names() -> Vec<String> {
    let mut names = Vec::new();
    names.extend(mutable_tree_collection_names(
        <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
    ));
    names.extend(mutable_tree_collection_names(
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    ));
    names.extend(mutable_tree_collection_names(
        <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME,
    ));
    names.push(format!(
        "{}_indexed_leaves",
        <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME
    ));
    names.push(DEPOSIT_COLLECTION.to_string());
    names.push(PROPOSED_BLOCKS_COLLECTION.to_string());
    names.push(SYNC_STATE_COLLECTION.to_string());
    names
}

fn restore_journal_collections_for_canonical_snapshot() -> Vec<RestoreJournalCollection> {
    proposer_snapshot_collection_names()
        .into_iter()
        .map(|collection_name| RestoreJournalCollection {
            shadow: shadow_collection_name(&collection_name),
            backup: backup_collection_name(&collection_name),
            live: collection_name,
        })
        .collect()
}

pub(crate) fn required_proposer_snapshot_collection_names() -> Vec<String> {
    vec![
        format!(
            "{}_metadata",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        ),
        format!(
            "{}_metadata",
            <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
        ),
        format!(
            "{}_metadata",
            <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME
        ),
        format!(
            "{}_indexed_leaves",
            <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME
        ),
        DEPOSIT_COLLECTION.to_string(),
        PROPOSED_BLOCKS_COLLECTION.to_string(),
        SYNC_STATE_COLLECTION.to_string(),
    ]
}

fn validate_snapshot_manifest_collection_set(
    manifest: &ProposerSnapshotManifest,
) -> Result<(), SnapshotError> {
    let canonical = proposer_snapshot_collection_names();
    let required = required_proposer_snapshot_collection_names();
    let mut seen = std::collections::HashSet::new();
    let manifest_names: std::collections::HashSet<&str> = manifest
        .collections
        .iter()
        .map(|collection| collection.collection_name.as_str())
        .collect();

    for collection in &manifest.collections {
        if !seen.insert(collection.collection_name.as_str()) {
            return Err(SnapshotError::DuplicateSnapshotCollection(
                collection.collection_name.clone(),
            ));
        }

        if !canonical
            .iter()
            .any(|name| name == &collection.collection_name)
        {
            return Err(SnapshotError::UnexpectedSnapshotCollection(
                collection.collection_name.clone(),
            ));
        }
    }

    for required_collection in required {
        if !manifest_names.contains(required_collection.as_str()) {
            return Err(SnapshotError::MissingSnapshotCollection(
                required_collection,
            ));
        }
    }

    Ok(())
}

fn is_optional_snapshot_collection(collection_name: &str) -> bool {
    proposer_snapshot_collection_names()
        .into_iter()
        .any(|name| name == collection_name)
        && !required_proposer_snapshot_collection_names()
            .into_iter()
            .any(|name| name == collection_name)
}

fn validate_snapshot_file_name(file_name: &str) -> Result<(), SnapshotError> {
    let mut components = Path::new(file_name).components();
    match (components.next(), components.next()) {
        (Some(std::path::Component::Normal(component)), None)
            if !component.to_string_lossy().is_empty() =>
        {
            Ok(())
        }
        _ => Err(SnapshotError::InvalidSnapshotFileName(
            file_name.to_string(),
        )),
    }
}

fn shadow_collection_name(live_collection_name: &str) -> String {
    format!("restore_shadow__{live_collection_name}")
}

fn backup_collection_name(live_collection_name: &str) -> String {
    format!("restore_backup__{live_collection_name}")
}

fn restore_collections_in_swap_order(
    collections: &[RestoreJournalCollection],
) -> Vec<RestoreJournalCollection> {
    let mut ordered = collections.to_vec();
    ordered.sort_by(|left, right| {
        match (
            left.live == SYNC_STATE_COLLECTION,
            right.live == SYNC_STATE_COLLECTION,
        ) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => left.live.cmp(&right.live),
        }
    });
    ordered
}

fn restore_phase_name(phase: &RestoreJournalPhase) -> &'static str {
    match phase {
        RestoreJournalPhase::LoadingShadow => "loading_shadow",
        RestoreJournalPhase::SwapInProgress => "swap_in_progress",
        RestoreJournalPhase::RollbackInProgress => "rollback_in_progress",
        RestoreJournalPhase::SwapComplete => "swap_complete",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RestoreCollectionLayout {
    live_exists: bool,
    shadow_exists: bool,
    backup_exists: bool,
}

impl RestoreCollectionLayout {
    fn is_pending_swap(self) -> bool {
        !self.backup_exists && self.shadow_exists
    }

    fn is_backup_created(self) -> bool {
        self.backup_exists && !self.live_exists
    }

    fn is_live_restored(self) -> bool {
        self.live_exists && !self.shadow_exists
    }

    fn needs_current_rollback(self) -> bool {
        self.backup_exists || self.is_live_restored()
    }
}

async fn inspect_restore_collection_layout(
    database: &mongodb::Database,
    collection: &RestoreJournalCollection,
) -> Result<RestoreCollectionLayout, SnapshotError> {
    Ok(RestoreCollectionLayout {
        live_exists: collection_exists(database, &collection.live).await?,
        shadow_exists: collection_exists(database, &collection.shadow).await?,
        backup_exists: collection_exists(database, &collection.backup).await?,
    })
}

async fn rollback_indices_from_swap_state(
    database: &mongodb::Database,
    journal: &RestoreJournal,
    ordered: &[RestoreJournalCollection],
) -> Result<Vec<usize>, SnapshotError> {
    let collection_count = ordered.len();
    let current_index = journal.current_index.unwrap_or(0) as usize;
    if current_index >= collection_count && collection_count > 0 {
        return Err(SnapshotError::RestoreInvariantViolation(format!(
            "current_index {current_index} is out of bounds for {collection_count} collections"
        )));
    }

    let include_current = if let Some(collection) = ordered.get(current_index) {
        let layout = inspect_restore_collection_layout(database, collection).await?;
        let include_current = layout.needs_current_rollback();
        if matches!(
            journal.current_step,
            Some(RestoreJournalStep::BackupCreated)
        ) && !include_current
        {
            return Err(SnapshotError::RestoreInvariantViolation(format!(
                "rollback expected current collection {} to retain restorable state after BackupCreated, but its namespace layout no longer does",
                collection.live
            )));
        }
        include_current
    } else {
        false
    };

    let mut rollback_indices: Vec<usize> = if include_current {
        (0..=current_index).collect()
    } else {
        (0..current_index).collect()
    };
    rollback_indices.reverse();
    Ok(rollback_indices)
}

fn compute_overall_checksum(collections: &[SnapshotCollectionManifest]) -> String {
    let mut overall_checksum = Sha256::new();
    let mut collections_for_checksum: Vec<&SnapshotCollectionManifest> =
        collections.iter().collect();
    collections_for_checksum
        .sort_by(|left, right| left.collection_name.cmp(&right.collection_name));
    for collection in collections_for_checksum {
        overall_checksum.update(collection.collection_name.as_bytes());
        overall_checksum.update(b"|");
        overall_checksum.update(collection.file_name.as_bytes());
        overall_checksum.update(b"|");
        overall_checksum.update(collection.document_count.to_le_bytes());
        overall_checksum.update(b"|");
        overall_checksum.update(collection.sha256.as_bytes());
        overall_checksum.update(b"\n");
    }

    hex::encode(overall_checksum.finalize())
}

pub async fn cleanup_orphaned_proposer_snapshot_temp_dirs(
    snapshot_root_dir: &Path,
) -> Result<(), SnapshotError> {
    if !fs::try_exists(snapshot_root_dir).await? {
        return Ok(());
    }

    let mut entries = fs::read_dir(snapshot_root_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name
            .to_string_lossy()
            .starts_with(TEMP_SNAPSHOT_DIR_PREFIX)
        {
            fs::remove_dir_all(entry.path()).await?;
        }
    }

    Ok(())
}

async fn export_collection(
    database: &mongodb::Database,
    snapshot_dir: &Path,
    collection_name: &str,
    session: &mut mongodb::ClientSession,
) -> Result<SnapshotCollectionManifest, SnapshotError> {
    let file_name = format!("{collection_name}.jsonl");
    let file_path = snapshot_dir.join(&file_name);
    let mut file = File::create(file_path).await?;
    let collection = database.collection::<Document>(collection_name);
    let mut cursor = collection
        .find(mongodb::bson::doc! {})
        .session(&mut *session)
        .await?;
    let mut document_count = 0_u64;
    let mut checksum = Sha256::new();

    while cursor.advance(&mut *session).await? {
        let document = cursor.deserialize_current()?;
        let json_value = Bson::Document(document).into_relaxed_extjson();
        let line = serde_json::to_string(&json_value)?;
        file.write_all(line.as_bytes()).await?;
        file.write_all(b"\n").await?;
        checksum.update(line.as_bytes());
        checksum.update(b"\n");
        document_count += 1;
    }

    file.flush().await?;

    Ok(SnapshotCollectionManifest {
        collection_name: collection_name.to_string(),
        file_name,
        document_count,
        sha256: hex::encode(checksum.finalize()),
    })
}

pub async fn create_proposer_snapshot(
    client: &mongodb::Client,
    snapshot_root_dir: &Path,
) -> Result<ProposerSnapshotManifest, SnapshotError> {
    let _maintenance_guard = acquire_proposer_state_maintenance_guard().await;
    create_proposer_snapshot_unlocked(client, snapshot_root_dir).await
}

pub(crate) async fn create_proposer_snapshot_unlocked(
    client: &mongodb::Client,
    snapshot_root_dir: &Path,
) -> Result<ProposerSnapshotManifest, SnapshotError> {
    let database = client.database(DB);
    // listCollections is not supported inside multi-document transactions.
    // We therefore discover the stable proposer collection set before opening
    // the snapshot read transaction. This is acceptable because these
    // collection names are created during proposer initialization and are not
    // expected to change while a snapshot is taken.
    let existing_collections = database.list_collection_names().await?;
    for collection_name in required_proposer_snapshot_collection_names() {
        if !existing_collections
            .iter()
            .any(|name| name == &collection_name)
        {
            return Err(SnapshotError::MissingSnapshotCollection(collection_name));
        }
    }
    let snapshot_root_dir = snapshot_root_dir.to_path_buf();
    fs::create_dir_all(&snapshot_root_dir).await?;
    cleanup_orphaned_proposer_snapshot_temp_dirs(&snapshot_root_dir).await?;
    let mut session = client.start_session().await?;
    // temp_snapshot_dir must stay a sibling of the final snapshot_dir under
    // snapshot_root_dir so the final rename remains atomic and intra-filesystem.
    // Do not move temp snapshot staging to a different filesystem (for example
    // /tmp) without adding an explicit copy fallback for EXDEV.
    let temp_snapshot_dir = snapshot_root_dir.join(format!(
        "{TEMP_SNAPSHOT_DIR_PREFIX}{}-{}",
        mongodb::bson::DateTime::now().timestamp_millis(),
        process::id()
    ));
    fs::create_dir_all(&temp_snapshot_dir).await?;
    let temp_snapshot_dir_for_export = temp_snapshot_dir.clone();
    let snapshot_result = session
        .start_transaction()
        .read_concern(ReadConcern::snapshot())
        .and_run2(async move |session| {
            let sync_state = database
                .collection::<SyncState>(SYNC_STATE_COLLECTION)
                .find_one(mongodb::bson::doc! { "_id": SyncState::DOCUMENT_ID })
                .session(&mut *session)
                .await?
                .ok_or_else(|| snapshot_error_to_mongo(SnapshotError::MissingSyncState))?;

            validate_snapshot_state_with_session(&database, &sync_state, session)
                .await
                .map_err(snapshot_error_to_mongo)?;

            let created_at = mongodb::bson::DateTime::now();
            let snapshot_id = format!(
                "proposer-l2-{}-{}",
                sync_state.last_applied_l2_block,
                created_at.timestamp_millis()
            );

            let mut collections = Vec::new();
            for collection_name in proposer_snapshot_collection_names() {
                if existing_collections.iter().any(|name| name == &collection_name) {
                    collections.push(
                        export_collection(
                            &database,
                            &temp_snapshot_dir_for_export,
                            &collection_name,
                            session,
                        )
                        .await
                        .map_err(snapshot_error_to_mongo)?,
                    );
                    if collections.len() == 1 {
                        maybe_crash_at_failpoint("snapshot_after_first_collection_exported");
                    }
                } else {
                    debug!(
                        "Skipping optional proposer snapshot collection {collection_name}: collection not found in database {DB}"
                    );
                }
            }

            Ok::<
                (
                    SyncState,
                    Vec<SnapshotCollectionManifest>,
                    String,
                    mongodb::bson::DateTime,
                ),
                mongodb::error::Error,
            >((sync_state, collections, snapshot_id, created_at))
        })
        .await;

    let (sync_state, collections, snapshot_id, created_at) = match snapshot_result {
        Ok(result) => result,
        Err(error) => {
            let _ = fs::remove_dir_all(&temp_snapshot_dir).await;
            return Err(SnapshotError::from(error));
        }
    };
    let snapshot_dir = snapshot_root_dir.join(&snapshot_id);

    let overall_checksum = compute_overall_checksum(&collections);
    let manifest = ProposerSnapshotManifest::new(
        snapshot_id,
        created_at,
        DB.to_string(),
        &sync_state,
        collections,
        overall_checksum,
    );
    let result = async {
        let manifest_path: PathBuf = temp_snapshot_dir.join("manifest.json");
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        fs::write(manifest_path, manifest_bytes).await?;
        fs::rename(&temp_snapshot_dir, &snapshot_dir).await?;
        Ok::<(), SnapshotError>(())
    }
    .await;

    if let Err(error) = result {
        let _ = fs::remove_dir_all(&temp_snapshot_dir).await;
        return Err(error);
    }

    Ok(manifest)
}

async fn load_snapshot_manifest(
    snapshot_dir: &Path,
) -> Result<ProposerSnapshotManifest, SnapshotError> {
    let manifest_path = snapshot_dir.join("manifest.json");
    let manifest_bytes = fs::read(manifest_path).await?;
    let manifest: ProposerSnapshotManifest = serde_json::from_slice(&manifest_bytes)?;

    if manifest.schema_version != ProposerSnapshotManifest::SCHEMA_VERSION {
        return Err(SnapshotError::UnsupportedManifestSchemaVersion(
            manifest.schema_version,
        ));
    }

    if manifest.storage_format != ProposerSnapshotManifest::STORAGE_FORMAT {
        return Err(SnapshotError::UnsupportedManifestStorageFormat(
            manifest.storage_format,
        ));
    }

    validate_snapshot_manifest_collection_set(&manifest)?;

    Ok(manifest)
}

pub(crate) async fn load_and_validate_snapshot_manifest(
    snapshot_dir: &Path,
) -> Result<ProposerSnapshotManifest, SnapshotError> {
    let manifest = load_snapshot_manifest(snapshot_dir).await?;
    validate_snapshot_files(snapshot_dir, &manifest).await?;
    let snapshotted_sync_state = load_snapshotted_sync_state(snapshot_dir, &manifest).await?;
    validate_manifest_against_snapshotted_sync_state(&manifest, &snapshotted_sync_state)?;
    Ok(manifest)
}

async fn tree_sub_tree_count_with_session(
    database: &mongodb::Database,
    tree_name: &str,
    session: &mut mongodb::ClientSession,
) -> Result<u64, SnapshotError> {
    let metadata_collection_name = format!("{tree_name}_metadata");
    let metadata = database
        .collection::<TreeMetadata<Fr254>>(&metadata_collection_name)
        .find_one(mongodb::bson::doc! { "_id": 0 })
        .session(&mut *session)
        .await?
        .ok_or_else(|| {
            SnapshotError::InvalidSnapshotSyncState(format!(
                "tree metadata for {tree_name} is missing while creating a snapshot"
            ))
        })?;
    Ok(metadata.sub_tree_count)
}

async fn highest_stored_block_with_session(
    database: &mongodb::Database,
    session: &mut mongodb::ClientSession,
) -> Result<Option<StoredBlock>, SnapshotError> {
    database
        .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
        .find_one(mongodb::bson::doc! {})
        .sort(mongodb::bson::doc! { "layer2_block_number": -1_i32 })
        .session(&mut *session)
        .await
        .map_err(SnapshotError::from)
}

async fn reserved_deposit_count_with_session(
    database: &mongodb::Database,
    session: &mut mongodb::ClientSession,
) -> Result<u64, SnapshotError> {
    database
        .collection::<Document>(DEPOSIT_COLLECTION)
        .count_documents(mongodb::bson::doc! { "reserved": true })
        .session(&mut *session)
        .await
        .map_err(SnapshotError::from)
}

async fn validate_snapshot_state_with_session(
    database: &mongodb::Database,
    sync_state: &SyncState,
    session: &mut mongodb::ClientSession,
) -> Result<(), SnapshotError> {
    if sync_state.schema_version != SyncState::SCHEMA_VERSION {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "sync_state uses unsupported schema version {}",
            sync_state.schema_version
        )));
    }

    let stored_block = database
        .collection::<StoredBlock>(PROPOSED_BLOCKS_COLLECTION)
        .find_one(mongodb::bson::doc! {
            "layer2_block_number": sync_state.last_applied_l2_block as i64
        })
        .session(&mut *session)
        .await?
        .ok_or_else(|| {
            SnapshotError::InvalidSnapshotSyncState(format!(
                "sync_state references L2 block {}, but StoredBlock at that height is missing",
                sync_state.last_applied_l2_block
            ))
        })?;

    let stored_fingerprint = stored_block.hash().to_hex_string();
    if stored_fingerprint != sync_state.fingerprint {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "sync_state fingerprint {} does not match StoredBlock fingerprint {} at L2 block {}",
            sync_state.fingerprint, stored_fingerprint, sync_state.last_applied_l2_block
        )));
    }

    if let Some(highest_stored_block) = highest_stored_block_with_session(database, session).await?
    {
        if highest_stored_block.layer2_block_number > sync_state.last_applied_l2_block {
            return Err(SnapshotError::InvalidSnapshotSyncState(format!(
                "highest StoredBlock {} is ahead of sync_state-applied block {}",
                highest_stored_block.layer2_block_number, sync_state.last_applied_l2_block
            )));
        }
    }

    let reserved_deposit_count = reserved_deposit_count_with_session(database, session).await?;
    if reserved_deposit_count > 0 {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "Deposits contains {reserved_deposit_count} reserved selection(s) ahead of sync_state-applied block {}",
            sync_state.last_applied_l2_block
        )));
    }

    let commitment_sub_tree_count = tree_sub_tree_count_with_session(
        database,
        <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
        session,
    )
    .await?;
    let historic_root_sub_tree_count = tree_sub_tree_count_with_session(
        database,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
        session,
    )
    .await?;
    let expected_historic_root_sub_tree_count =
        expected_historic_root_sub_tree_count(sync_state.last_applied_l2_block)?;

    if historic_root_sub_tree_count <= 1 {
        return Err(SnapshotError::InvalidSnapshotSyncState(
            "historic root tree only contains the zero leaf".to_string(),
        ));
    }

    if historic_root_sub_tree_count > expected_historic_root_sub_tree_count {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "historic root tree sub_tree_count {} is ahead of sync_state-applied block {} (maximum coherent value {})",
            historic_root_sub_tree_count,
            sync_state.last_applied_l2_block,
            expected_historic_root_sub_tree_count
        )));
    }

    if historic_root_sub_tree_count < expected_historic_root_sub_tree_count {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "historic root tree sub_tree_count {} is behind sync_state-applied block {} (required coherent value {})",
            historic_root_sub_tree_count,
            sync_state.last_applied_l2_block,
            expected_historic_root_sub_tree_count
        )));
    }

    if commitment_sub_tree_count == 0 && !stored_block.commitments.is_empty() {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "commitment tree is empty while StoredBlock {} contains commitments",
            sync_state.last_applied_l2_block
        )));
    }

    Ok(())
}

async fn ensure_empty_shadow_collection(
    database: &mongodb::Database,
    shadow_collection_name: &str,
) -> Result<(), SnapshotError> {
    database.create_collection(shadow_collection_name).await?;
    Ok(())
}

async fn ensure_restored_snapshot_collection_indexes(
    client: &mongodb::Client,
) -> Result<(), SnapshotError> {
    // Deposits is currently the only snapshot-restored collection with required
    // secondary indexes beyond MongoDB's default _id index.
    ensure_deposit_indexes(client).await?;
    Ok(())
}

pub async fn find_latest_valid_proposer_snapshot(
    snapshot_root_dir: &Path,
    max_last_applied_l2_block: u64,
) -> Result<Option<(PathBuf, ProposerSnapshotManifest)>, SnapshotError> {
    if !fs::try_exists(snapshot_root_dir).await? {
        return Ok(None);
    }

    let mut entries = fs::read_dir(snapshot_root_dir).await?;
    let mut candidates = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        if !file_type.is_dir() {
            continue;
        }

        let snapshot_dir = entry.path();
        let Some(snapshot_dir_name) = snapshot_dir.file_name().and_then(|name| name.to_str())
        else {
            continue;
        };
        if snapshot_dir_name.starts_with(TEMP_SNAPSHOT_DIR_PREFIX) {
            continue;
        }
        let manifest = match load_and_validate_snapshot_manifest(&snapshot_dir).await {
            Ok(manifest) => manifest,
            Err(error) => {
                warn!(
                    "Skipping proposer snapshot at {}: validation failed: {error}",
                    snapshot_dir.display()
                );
                continue;
            }
        };

        if manifest.last_applied_l2_block > max_last_applied_l2_block {
            continue;
        }

        candidates.push((snapshot_dir, manifest));
    }

    candidates.sort_by(|left, right| {
        right
            .1
            .last_applied_l2_block
            .cmp(&left.1.last_applied_l2_block)
            .then_with(|| right.1.snapshot_id.cmp(&left.1.snapshot_id))
    });

    Ok(candidates.into_iter().next())
}

async fn validate_snapshot_files(
    snapshot_dir: &Path,
    manifest: &ProposerSnapshotManifest,
) -> Result<(), SnapshotError> {
    for collection in &manifest.collections {
        validate_snapshot_file_name(&collection.file_name)?;
        let file_path = snapshot_dir.join(&collection.file_name);
        if !fs::try_exists(&file_path).await? {
            return Err(SnapshotError::MissingSnapshotFile(
                collection.file_name.clone(),
            ));
        }

        let file = File::open(file_path).await?;
        let mut reader = BufReader::new(file).lines();
        let mut checksum = Sha256::new();
        let mut document_count = 0_u64;

        while let Some(line) = reader.next_line().await? {
            checksum.update(line.as_bytes());
            checksum.update(b"\n");
            document_count += 1;
        }

        let actual_checksum = hex::encode(checksum.finalize());
        if actual_checksum != collection.sha256 {
            return Err(SnapshotError::CollectionChecksumMismatch {
                collection_name: collection.collection_name.clone(),
                expected: collection.sha256.clone(),
                actual: actual_checksum,
            });
        }

        if document_count != collection.document_count {
            return Err(SnapshotError::DocumentCountMismatch {
                collection_name: collection.collection_name.clone(),
                expected: collection.document_count,
                actual: document_count,
            });
        }
    }

    let overall_checksum = compute_overall_checksum(&manifest.collections);
    if overall_checksum != manifest.overall_sha256 {
        return Err(SnapshotError::OverallChecksumMismatch {
            expected: manifest.overall_sha256.clone(),
            actual: overall_checksum,
        });
    }

    Ok(())
}

fn snapshot_collection_manifest<'a>(
    manifest: &'a ProposerSnapshotManifest,
    collection_name: &str,
) -> Result<&'a SnapshotCollectionManifest, SnapshotError> {
    manifest
        .collections
        .iter()
        .find(|collection| collection.collection_name == collection_name)
        .ok_or_else(|| SnapshotError::MissingSnapshotCollection(collection_name.to_string()))
}

async fn load_snapshotted_sync_state(
    snapshot_dir: &Path,
    manifest: &ProposerSnapshotManifest,
) -> Result<SyncState, SnapshotError> {
    let sync_state_manifest = snapshot_collection_manifest(manifest, SYNC_STATE_COLLECTION)?;
    validate_snapshot_file_name(&sync_state_manifest.file_name)?;

    let file = File::open(snapshot_dir.join(&sync_state_manifest.file_name)).await?;
    let mut reader = BufReader::new(file).lines();
    let Some(line) = reader.next_line().await? else {
        return Err(SnapshotError::InvalidSnapshotSyncState(
            "sync_state snapshot file is empty".to_string(),
        ));
    };
    let sync_state_json: serde_json::Value = serde_json::from_str(&line)?;
    let sync_state_bson = Bson::try_from(sync_state_json).map_err(|error| {
        SnapshotError::InvalidSnapshotSyncState(format!(
            "could not parse sync_state snapshot extended JSON: {error}"
        ))
    })?;
    let Bson::Document(sync_state_document) = sync_state_bson else {
        return Err(SnapshotError::InvalidSnapshotSyncState(
            "sync_state snapshot line is not a BSON document".to_string(),
        ));
    };
    let sync_state: SyncState =
        mongodb::bson::from_document(sync_state_document).map_err(|error| {
            SnapshotError::InvalidSnapshotSyncState(format!(
                "could not deserialize sync_state snapshot document: {error}"
            ))
        })?;

    if reader.next_line().await?.is_some() {
        return Err(SnapshotError::InvalidSnapshotSyncState(
            "sync_state snapshot file contains multiple documents".to_string(),
        ));
    }

    if sync_state.id != SyncState::DOCUMENT_ID {
        return Err(SnapshotError::InvalidSnapshotSyncState(format!(
            "unexpected sync_state document id {}",
            sync_state.id
        )));
    }

    Ok(sync_state)
}

fn validate_manifest_against_snapshotted_sync_state(
    manifest: &ProposerSnapshotManifest,
    snapshotted_sync_state: &SyncState,
) -> Result<(), SnapshotError> {
    if manifest.last_applied_l2_block != snapshotted_sync_state.last_applied_l2_block {
        return Err(SnapshotError::SnapshotManifestSyncStateMismatch {
            field: "last_applied_l2_block",
            manifest_value: manifest.last_applied_l2_block.to_string(),
            snapshotted_value: snapshotted_sync_state.last_applied_l2_block.to_string(),
        });
    }

    if manifest.fingerprint != snapshotted_sync_state.fingerprint {
        return Err(SnapshotError::SnapshotManifestSyncStateMismatch {
            field: "fingerprint",
            manifest_value: manifest.fingerprint.clone(),
            snapshotted_value: snapshotted_sync_state.fingerprint.clone(),
        });
    }

    if manifest.l1_ref.block_number != snapshotted_sync_state.l1_ref.block_number {
        return Err(SnapshotError::SnapshotManifestSyncStateMismatch {
            field: "l1_ref.block_number",
            manifest_value: manifest.l1_ref.block_number.to_string(),
            snapshotted_value: snapshotted_sync_state.l1_ref.block_number.to_string(),
        });
    }

    if manifest.l1_ref.tx_hash != snapshotted_sync_state.l1_ref.tx_hash {
        return Err(SnapshotError::SnapshotManifestSyncStateMismatch {
            field: "l1_ref.tx_hash",
            manifest_value: manifest.l1_ref.tx_hash.to_string(),
            snapshotted_value: snapshotted_sync_state.l1_ref.tx_hash.to_string(),
        });
    }

    if manifest.l1_ref.log_index != snapshotted_sync_state.l1_ref.log_index {
        return Err(SnapshotError::SnapshotManifestSyncStateMismatch {
            field: "l1_ref.log_index",
            manifest_value: manifest.l1_ref.log_index.to_string(),
            snapshotted_value: snapshotted_sync_state.l1_ref.log_index.to_string(),
        });
    }

    Ok(())
}

async fn import_collection_into_shadow(
    database: &mongodb::Database,
    snapshot_dir: &Path,
    manifest: &SnapshotCollectionManifest,
    shadow_collection_name: &str,
) -> Result<(), SnapshotError> {
    validate_snapshot_file_name(&manifest.file_name)?;
    database.create_collection(shadow_collection_name).await?;
    let file = File::open(snapshot_dir.join(&manifest.file_name)).await?;
    let mut reader = BufReader::new(file).lines();
    let shadow_collection = database.collection::<Document>(shadow_collection_name);

    while let Some(line) = reader.next_line().await? {
        let json_value: serde_json::Value = serde_json::from_str(&line)?;
        let bson_value = Bson::try_from(json_value).map_err(|error| {
            SnapshotError::SerdeJson(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "could not parse snapshot line for collection {} as extended JSON: {error}",
                    manifest.collection_name
                ),
            )))
        })?;
        let Bson::Document(document) = bson_value else {
            return Err(SnapshotError::SerdeJson(serde_json::Error::io(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "snapshot line for collection {} is not a BSON document",
                        manifest.collection_name
                    ),
                ),
            )));
        };
        shadow_collection.insert_one(document).await?;
    }

    let actual_count = shadow_collection
        .count_documents(mongodb::bson::doc! {})
        .await?;
    if actual_count != manifest.document_count {
        return Err(SnapshotError::DocumentCountMismatch {
            collection_name: manifest.collection_name.clone(),
            expected: manifest.document_count,
            actual: actual_count,
        });
    }

    Ok(())
}

async fn collection_exists(
    database: &mongodb::Database,
    collection_name: &str,
) -> Result<bool, SnapshotError> {
    let names = database.list_collection_names().await?;
    Ok(names.iter().any(|name| name == collection_name))
}

async fn snapshot_checksum_for_collection(
    database: &mongodb::Database,
    collection_name: &str,
) -> Result<(u64, String), SnapshotError> {
    let collection = database.collection::<Document>(collection_name);
    let mut cursor = collection.find(mongodb::bson::doc! {}).await?;
    let mut document_count = 0_u64;
    let mut checksum = Sha256::new();

    while cursor.advance().await? {
        let document = cursor.deserialize_current()?;
        let json_value = Bson::Document(document).into_relaxed_extjson();
        let line = serde_json::to_string(&json_value)?;
        checksum.update(line.as_bytes());
        checksum.update(b"\n");
        document_count += 1;
    }

    Ok((document_count, hex::encode(checksum.finalize())))
}

async fn optional_live_matches_expected_snapshot_state(
    database: &mongodb::Database,
    journal: &RestoreJournal,
    collection: &RestoreJournalCollection,
) -> Result<bool, SnapshotError> {
    let manifest = load_and_validate_snapshot_manifest(Path::new(&journal.snapshot_dir)).await?;
    if manifest.overall_sha256 != journal.manifest_overall_sha256 {
        return Err(SnapshotError::RestoreInvariantViolation(format!(
            "snapshot manifest checksum {} no longer matches restore journal checksum {} while verifying optional collection {}",
            manifest.overall_sha256,
            journal.manifest_overall_sha256,
            collection.live,
        )));
    }

    let (document_count, checksum) =
        snapshot_checksum_for_collection(database, &collection.live).await?;

    let expected = manifest
        .collections
        .iter()
        .find(|manifest_collection| manifest_collection.collection_name == collection.live);

    Ok(match expected {
        Some(expected) => document_count == expected.document_count && checksum == expected.sha256,
        None => document_count == 0,
    })
}

async fn rename_collection(
    client: &mongodb::Client,
    from: &str,
    to: &str,
    drop_target: bool,
) -> Result<(), SnapshotError> {
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {
            "renameCollection": format!("{DB}.{from}"),
            "to": format!("{DB}.{to}"),
            "dropTarget": drop_target,
        })
        .await?;
    Ok(())
}

async fn drop_collection_if_exists(
    database: &mongodb::Database,
    collection_name: &str,
) -> Result<(), SnapshotError> {
    if collection_exists(database, collection_name).await? {
        database
            .collection::<Document>(collection_name)
            .drop()
            .await?;
    }
    Ok(())
}

async fn begin_rollback_from_swap_journal(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    let ordered = restore_collections_in_swap_order(&journal.collections);
    let database = client.database(DB);
    let rollback_indices = rollback_indices_from_swap_state(&database, journal, &ordered).await?;

    journal.collections = ordered;
    journal.phase = RestoreJournalPhase::RollbackInProgress;
    journal.current_index = rollback_indices.first().map(|index| *index as u32);
    journal.current_step = journal
        .current_index
        .map(|_| RestoreJournalStep::RollbackPending);
    journal.updated_at = mongodb::bson::DateTime::now();
    client.upsert_restore_journal(journal).await?;
    Ok(())
}

async fn begin_rollback_from_swap_complete_journal(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    let ordered = restore_collections_in_swap_order(&journal.collections);

    journal.collections = ordered.clone();
    journal.phase = RestoreJournalPhase::RollbackInProgress;
    journal.current_index = ordered.len().checked_sub(1).map(|index| index as u32);
    journal.current_step = journal
        .current_index
        .map(|_| RestoreJournalStep::RollbackPending);
    journal.updated_at = mongodb::bson::DateTime::now();
    client.upsert_restore_journal(journal).await?;
    Ok(())
}

async fn complete_rollback_from_journal(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    if journal.phase != RestoreJournalPhase::RollbackInProgress {
        return Err(SnapshotError::UnexpectedRestoreJournalPhase {
            expected: "rollback_in_progress".to_string(),
            actual: restore_phase_name(&journal.phase).to_string(),
        });
    }

    let database = client.database(DB);
    let ordered = restore_collections_in_swap_order(&journal.collections);
    journal.collections = ordered.clone();

    let mut first_iteration = true;
    while let Some(current_index) = journal.current_index {
        let index = current_index as usize;
        if index >= ordered.len() {
            return Err(SnapshotError::RestoreInvariantViolation(format!(
                "rollback current_index {index} is out of bounds for {} collections",
                ordered.len()
            )));
        }

        let collection = &ordered[index];
        let current_step = journal
            .current_step
            .clone()
            .unwrap_or(RestoreJournalStep::RollbackPending);
        match current_step {
            RestoreJournalStep::RollbackPending | RestoreJournalStep::RollbackStarted => {
                let resuming_started_step =
                    matches!(current_step, RestoreJournalStep::RollbackStarted);
                if matches!(current_step, RestoreJournalStep::RollbackPending) {
                    journal.current_step = Some(RestoreJournalStep::RollbackStarted);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;
                    if first_iteration {
                        maybe_crash_at_failpoint("restore_after_first_rollback_progress_persist");
                    }
                }

                let layout = inspect_restore_collection_layout(&database, collection).await?;

                if layout.backup_exists {
                    rename_collection(client, &collection.backup, &collection.live, true).await?;
                    if first_iteration {
                        maybe_crash_at_failpoint("restore_after_first_rollback_live_restore");
                    }

                    journal.current_step = Some(RestoreJournalStep::RollbackApplied);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;
                } else if is_optional_snapshot_collection(&collection.live) && layout.live_exists {
                    if resuming_started_step
                        && !optional_live_matches_expected_snapshot_state(
                            &database, journal, collection,
                        )
                        .await?
                    {
                        return Err(SnapshotError::RestoreInvariantViolation(format!(
                            "rollback cannot prove optional live collection {} still matches the restored snapshot state after crash; manual intervention required",
                            collection.live
                        )));
                    }

                    drop_collection_if_exists(&database, &collection.live).await?;

                    journal.current_step = Some(RestoreJournalStep::RollbackApplied);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;
                } else if (resuming_started_step
                    && layout.live_exists
                    && !is_optional_snapshot_collection(&collection.live))
                    || (is_optional_snapshot_collection(&collection.live)
                        && (layout.shadow_exists || resuming_started_step))
                {
                    journal.current_step = Some(RestoreJournalStep::RollbackApplied);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;
                } else {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "rollback cannot restore live collection {} because backup {} is missing; manual intervention required",
                        collection.live, collection.backup
                    )));
                }
            }
            RestoreJournalStep::RollbackApplied => {}
            step => {
                return Err(SnapshotError::RestoreInvariantViolation(format!(
                    "rollback expected rollback step at index {index}, found {step:?}"
                )));
            }
        }

        journal.current_index = index.checked_sub(1).map(|next| next as u32);
        journal.current_step = journal
            .current_index
            .map(|_| RestoreJournalStep::RollbackPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client.upsert_restore_journal(journal).await?;
        first_iteration = false;
    }

    maybe_crash_at_failpoint("restore_after_rollback_renames_complete");

    for collection in &ordered {
        drop_collection_if_exists(&database, &collection.shadow).await?;
        drop_collection_if_exists(&database, &collection.backup).await?;
    }

    client.delete_restore_journal().await?;
    Ok(())
}

async fn rollback_shadow_swap(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    if journal.phase == RestoreJournalPhase::SwapInProgress {
        begin_rollback_from_swap_journal(client, journal).await?;
    }

    complete_rollback_from_journal(client, journal).await
}

async fn rollback_completed_swap_from_backups(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    if journal.phase == RestoreJournalPhase::SwapComplete {
        begin_rollback_from_swap_complete_journal(client, journal).await?;
    }

    complete_rollback_from_journal(client, journal).await
}

async fn cleanup_loading_shadow_restore(
    client: &mongodb::Client,
    journal: &RestoreJournal,
) -> Result<(), SnapshotError> {
    let database = client.database(DB);
    for collection in &journal.collections {
        drop_collection_if_exists(&database, &collection.shadow).await?;
        drop_collection_if_exists(&database, &collection.backup).await?;
    }
    client.delete_restore_journal().await?;
    Ok(())
}

async fn complete_shadow_swap_from_journal(
    client: &mongodb::Client,
    journal: &mut RestoreJournal,
) -> Result<(), SnapshotError> {
    if journal.phase != RestoreJournalPhase::SwapInProgress {
        return Err(SnapshotError::UnexpectedRestoreJournalPhase {
            expected: "swap_in_progress".to_string(),
            actual: restore_phase_name(&journal.phase).to_string(),
        });
    }

    let database = client.database(DB);
    let ordered = restore_collections_in_swap_order(&journal.collections);
    journal.collections = ordered.clone();

    let start_index = journal.current_index.unwrap_or(0) as usize;
    if start_index >= ordered.len() {
        return Err(SnapshotError::RestoreInvariantViolation(format!(
            "current_index {start_index} is out of bounds for {} collections",
            ordered.len()
        )));
    }

    for (index, collection) in ordered.iter().enumerate().skip(start_index) {
        let step = if index == start_index {
            journal
                .current_step
                .clone()
                .unwrap_or(RestoreJournalStep::BackupPending)
        } else {
            RestoreJournalStep::BackupPending
        };

        match step {
            RestoreJournalStep::BackupPending => {
                let layout = inspect_restore_collection_layout(&database, collection).await?;

                if layout.is_live_restored() {
                    if !layout.backup_exists {
                        if !is_optional_snapshot_collection(&collection.live) {
                            return Err(SnapshotError::RestoreInvariantViolation(format!(
                                "live collection {} is already restored but backup {} is missing; resume cannot prove the pre-restore state still exists",
                                collection.live, collection.backup
                            )));
                        }

                        let matches_snapshot = optional_live_matches_expected_snapshot_state(
                            &database,
                            journal,
                            collection,
                        )
                        .await
                        .map_err(|error| {
                            SnapshotError::RestoreInvariantViolation(format!(
                                "optional live collection {} is present while both shadow {} and backup {} are missing, and recovery could not verify it matches the snapshot state: {}",
                                collection.live, collection.shadow, collection.backup, error
                            ))
                        })?;

                        if !matches_snapshot {
                            return Err(SnapshotError::RestoreInvariantViolation(format!(
                                "optional live collection {} is present while both shadow {} and backup {} are missing; resume cannot prove snapshot state replaced the pre-restore live collection",
                                collection.live, collection.shadow, collection.backup
                            )));
                        }
                    }
                } else if layout.is_backup_created() {
                    journal.current_index = Some(index as u32);
                    journal.current_step = Some(RestoreJournalStep::BackupCreated);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;

                    if !layout.shadow_exists {
                        return Err(SnapshotError::RestoreInvariantViolation(format!(
                            "shadow collection {} is missing while resume expects BackupCreated",
                            collection.shadow
                        )));
                    }

                    rename_collection(client, &collection.shadow, &collection.live, false).await?;
                } else if layout.is_pending_swap() && layout.live_exists {
                    rename_collection(client, &collection.live, &collection.backup, false).await?;
                    maybe_crash_at_failpoint("restore_after_live_rename_before_backup_created");

                    journal.current_index = Some(index as u32);
                    journal.current_step = Some(RestoreJournalStep::BackupCreated);
                    journal.updated_at = mongodb::bson::DateTime::now();
                    client.upsert_restore_journal(journal).await?;
                    maybe_crash_at_failpoint("restore_after_backup_created");
                    rename_collection(client, &collection.shadow, &collection.live, false).await?;
                } else if layout.is_pending_swap()
                    && is_optional_snapshot_collection(&collection.live)
                {
                    rename_collection(client, &collection.shadow, &collection.live, false).await?;
                } else {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "collection {} cannot resume swap from BackupPending with namespace layout live_exists={}, shadow_exists={}, backup_exists={}",
                        collection.live,
                        layout.live_exists,
                        layout.shadow_exists,
                        layout.backup_exists
                    )));
                }
            }
            RestoreJournalStep::BackupCreated => {
                let layout = inspect_restore_collection_layout(&database, collection).await?;

                if layout.is_live_restored() {
                    if !layout.backup_exists && !is_optional_snapshot_collection(&collection.live) {
                        return Err(SnapshotError::RestoreInvariantViolation(format!(
                            "live collection {} is already restored but backup {} is missing while resume expects BackupCreated",
                            collection.live, collection.backup
                        )));
                    }
                } else if !layout.backup_exists {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "backup collection {} is missing while resume expects BackupCreated",
                        collection.backup
                    )));
                } else if !layout.shadow_exists {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "shadow collection {} is missing while resume expects BackupCreated",
                        collection.shadow
                    )));
                } else {
                    rename_collection(client, &collection.shadow, &collection.live, false).await?;
                }
            }
            step => {
                return Err(SnapshotError::RestoreInvariantViolation(format!(
                    "swap expected backup step at index {index}, found {step:?}"
                )));
            }
        }

        if index + 1 < ordered.len() {
            journal.current_index = Some((index + 1) as u32);
            journal.current_step = Some(RestoreJournalStep::BackupPending);
            journal.updated_at = mongodb::bson::DateTime::now();
            client.upsert_restore_journal(journal).await?;
        }
    }

    journal.phase = RestoreJournalPhase::SwapComplete;
    journal.current_index = None;
    journal.current_step = Some(RestoreJournalStep::NonSnapshotCleanupPending);
    journal.updated_at = mongodb::bson::DateTime::now();
    client.upsert_restore_journal(journal).await?;
    maybe_crash_at_failpoint("restore_after_swap_complete");

    Ok(())
}

pub async fn load_proposer_snapshot_into_shadow(
    client: &mongodb::Client,
    snapshot_dir: &Path,
) -> Result<RestoreJournal, SnapshotError> {
    let snapshot_dir = snapshot_dir.to_path_buf();
    let manifest = load_and_validate_snapshot_manifest(&snapshot_dir).await?;
    let manifest_collections_by_name: std::collections::HashMap<_, _> = manifest
        .collections
        .iter()
        .map(|collection| (collection.collection_name.as_str(), collection))
        .collect();

    let database = client.database(DB);
    let existing_collections = database.list_collection_names().await?;
    let restore_collections = restore_journal_collections_for_canonical_snapshot();

    let now = mongodb::bson::DateTime::now();
    let mut journal = RestoreJournal::new_loading_shadow(
        manifest.snapshot_id.clone(),
        snapshot_dir.to_string_lossy().to_string(),
        manifest.overall_sha256.clone(),
        restore_collections.clone(),
        now,
    );
    client.upsert_restore_journal(&journal).await?;

    for restore_collection in &restore_collections {
        if existing_collections
            .iter()
            .any(|name| name == &restore_collection.shadow)
        {
            database
                .collection::<Document>(&restore_collection.shadow)
                .drop()
                .await?;
        }

        if existing_collections
            .iter()
            .any(|name| name == &restore_collection.backup)
        {
            database
                .collection::<Document>(&restore_collection.backup)
                .drop()
                .await?;
        }
    }

    for restore_collection in &restore_collections {
        if let Some(manifest_collection) =
            manifest_collections_by_name.get(restore_collection.live.as_str())
        {
            import_collection_into_shadow(
                &database,
                &snapshot_dir,
                manifest_collection,
                &restore_collection.shadow,
            )
            .await?;
        } else {
            ensure_empty_shadow_collection(&database, &restore_collection.shadow).await?;
        }
        maybe_crash_at_failpoint("restore_after_first_shadow_collection_imported");
    }

    journal.updated_at = mongodb::bson::DateTime::now();
    client.upsert_restore_journal(&journal).await?;

    Ok(journal)
}

pub async fn restore_proposer_snapshot(
    client: &mongodb::Client,
    snapshot_dir: &Path,
) -> Result<SyncState, SnapshotError> {
    let _maintenance_guard = acquire_proposer_state_maintenance_guard().await;
    load_proposer_snapshot_into_shadow(client, snapshot_dir).await?;
    swap_proposer_shadow_into_live(client).await?;
    cleanup_after_proposer_shadow_swap(client).await?;

    client
        .get_sync_state()
        .await
        .ok_or(SnapshotError::MissingSyncState)
}

pub async fn swap_proposer_shadow_into_live(
    client: &mongodb::Client,
) -> Result<RestoreJournal, SnapshotError> {
    let mut journal = client
        .get_restore_journal()
        .await
        .ok_or(SnapshotError::MissingRestoreJournal)?;

    if journal.phase != RestoreJournalPhase::LoadingShadow {
        return Err(SnapshotError::UnexpectedRestoreJournalPhase {
            expected: "loading_shadow".to_string(),
            actual: format!("{:?}", journal.phase),
        });
    }

    let ordered = restore_collections_in_swap_order(&journal.collections);
    journal.collections = ordered.clone();
    journal.phase = RestoreJournalPhase::SwapInProgress;
    journal.current_index = Some(0);
    journal.current_step = Some(RestoreJournalStep::BackupPending);
    journal.updated_at = mongodb::bson::DateTime::now();
    client.upsert_restore_journal(&journal).await?;

    if let Err(error) = complete_shadow_swap_from_journal(client, &mut journal).await {
        if matches!(error, SnapshotError::RestoreInvariantViolation(_)) {
            rollback_shadow_swap(client, &mut journal).await?;
        }
        return Err(error);
    }

    Ok(journal)
}

pub async fn cleanup_after_proposer_shadow_swap(
    client: &mongodb::Client,
) -> Result<(), SnapshotError> {
    let journal = client
        .get_restore_journal()
        .await
        .ok_or(SnapshotError::MissingRestoreJournal)?;

    if journal.phase != RestoreJournalPhase::SwapComplete {
        return Err(SnapshotError::UnexpectedRestoreJournalPhase {
            expected: "swap_complete".to_string(),
            actual: format!("{:?}", journal.phase),
        });
    }

    validate_restored_snapshot_state_before_cleanup(client).await?;
    complete_cleanup_after_proposer_shadow_swap(client, &journal).await
}

async fn validate_restored_snapshot_state_before_cleanup(
    client: &mongodb::Client,
) -> Result<(), SnapshotError> {
    ensure_restored_snapshot_collection_indexes(client).await?;
    validate_snapshotted_live_proposer_state_consistency(client)
        .await
        .map_err(SnapshotError::RestoreInvariantViolation)
}

async fn complete_cleanup_after_proposer_shadow_swap(
    client: &mongodb::Client,
    journal: &RestoreJournal,
) -> Result<(), SnapshotError> {
    let cleanup_stats = apply_non_snapshot_cleanup_if_needed(client, journal).await?;
    maybe_crash_at_failpoint("restore_after_non_snapshot_cleanup_applied");
    log_non_snapshot_cleanup_stats(&cleanup_stats);
    validate_live_proposer_state_consistency(client)
        .await
        .map_err(SnapshotError::RestoreInvariantViolation)?;

    let database = client.database(DB);
    let ordered = restore_collections_in_swap_order(&journal.collections);

    for collection in &ordered {
        drop_collection_if_exists(&database, &collection.backup).await?;
        maybe_crash_at_failpoint("restore_after_first_backup_cleanup");
    }

    for collection in &ordered {
        drop_collection_if_exists(&database, &collection.shadow).await?;
    }

    client.delete_restore_journal().await?;
    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct NonSnapshotCleanupStats {
    last_applied_l2_block: u64,
    deleted_pending_blocks: u64,
    cleared_reserved_deposits: u64,
    removed_mempool_transactions: u64,
    restored_selected_transactions: u64,
}

async fn apply_non_snapshot_cleanup_if_needed(
    client: &mongodb::Client,
    journal: &RestoreJournal,
) -> Result<NonSnapshotCleanupStats, SnapshotError> {
    match journal.current_step.clone() {
        Some(RestoreJournalStep::NonSnapshotCleanupApplied) => {
            Ok(NonSnapshotCleanupStats::default())
        }
        Some(RestoreJournalStep::NonSnapshotCleanupPending) | None => {
            cleanup_non_snapshot_restore_state(client, journal).await
        }
        Some(step) => Err(SnapshotError::RestoreInvariantViolation(format!(
            "swap_complete cleanup expected non_snapshot_cleanup step, found {step:?}"
        ))),
    }
}

async fn get_sync_state_with_session(
    client: &mongodb::Client,
    session: &mut mongodb::ClientSession,
) -> Result<SyncState, mongodb::error::Error> {
    client
        .database(DB)
        .collection::<SyncState>(SYNC_STATE_COLLECTION)
        .find_one(mongodb::bson::doc! { "_id": SyncState::DOCUMENT_ID })
        .session(&mut *session)
        .await?
        .ok_or_else(|| {
            mongodb::error::Error::custom(
                "Could not load restored sync_state during snapshot restore finalization",
            )
        })
}

async fn clear_all_reserved_deposits_with_session(
    client: &mongodb::Client,
    session: &mut mongodb::ClientSession,
) -> Result<u64, mongodb::error::Error> {
    client
        .database(DB)
        .collection::<Document>(DEPOSIT_COLLECTION)
        .update_many(
            mongodb::bson::doc! { "reserved": true },
            mongodb::bson::doc! { "$set": { "reserved": false } },
        )
        .session(&mut *session)
        .await
        .map(|result| result.modified_count)
}

async fn upsert_restore_journal_with_session(
    client: &mongodb::Client,
    journal: &RestoreJournal,
    session: &mut mongodb::ClientSession,
) -> Result<(), mongodb::error::Error> {
    let result = client
        .database(DB)
        .collection::<RestoreJournal>(RESTORE_JOURNAL_COLLECTION)
        .replace_one(
            mongodb::bson::doc! { "_id": RestoreJournal::DOCUMENT_ID },
            journal,
        )
        .upsert(true)
        .session(&mut *session)
        .await?;

    if result.matched_count == 0 && result.upserted_id.is_none() {
        return Err(mongodb::error::Error::custom(
            "Failed to upsert proposer restore_journal",
        ));
    }

    Ok(())
}

async fn cleanup_non_snapshot_restore_state(
    client: &mongodb::Client,
    journal: &RestoreJournal,
) -> Result<NonSnapshotCleanupStats, SnapshotError> {
    let journal_for_update = journal.clone();
    let client_for_transaction = client.clone();
    let mut session = client.start_session().await?;
    session
        .start_transaction()
        .and_run2(async move |session| {
            let sync_state = get_sync_state_with_session(&client_for_transaction, session).await?;
            let deleted_pending_blocks = client_for_transaction
                .delete_all_pending_blocks_with_session(session)
                .await
                .map_err(|error| {
                    mongodb::error::Error::custom(format!(
                        "Could not delete persisted PendingBlocks during snapshot restore finalization: {error}"
                    ))
                })?;
            let cleared_reserved_deposits =
                clear_all_reserved_deposits_with_session(&client_for_transaction, session)
                    .await
                    .map_err(|error| {
                        mongodb::error::Error::custom(format!(
                            "Could not clear proposer Deposits reservations during snapshot restore finalization: {error}"
                        ))
                    })?;
            let removed_mempool_transactions =
                remove_all_mempool_client_transactions_with_session(
                    &client_for_transaction,
                    session,
                )
                .await
                .map_err(mongodb::error::Error::custom)?;
            let restored_selected_transactions =
                restore_selected_transactions_to_mempool_after_block_with_session(
                    &client_for_transaction,
                    sync_state.last_applied_l2_block,
                    session,
                )
                .await
                .map_err(mongodb::error::Error::custom)?;

            let mut updated_journal = journal_for_update.clone();
            updated_journal.current_step = Some(RestoreJournalStep::NonSnapshotCleanupApplied);
            updated_journal.updated_at = mongodb::bson::DateTime::now();
            upsert_restore_journal_with_session(
                &client_for_transaction,
                &updated_journal,
                session,
            )
            .await?;

            Ok::<NonSnapshotCleanupStats, mongodb::error::Error>(NonSnapshotCleanupStats {
                last_applied_l2_block: sync_state.last_applied_l2_block,
                deleted_pending_blocks,
                cleared_reserved_deposits,
                removed_mempool_transactions,
                restored_selected_transactions,
            })
        })
        .await
        .map_err(SnapshotError::Mongo)
}

fn log_non_snapshot_cleanup_stats(stats: &NonSnapshotCleanupStats) {
    if stats.deleted_pending_blocks > 0
        || stats.cleared_reserved_deposits > 0
        || stats.removed_mempool_transactions > 0
        || stats.restored_selected_transactions > 0
    {
        warn!(
            "Snapshot restore discarded {} persisted PendingBlock(s), cleared {} reserved deposit selection(s), dropped {} mempool client transaction(s), and restored {} selected client transaction(s) beyond restored sync_state L2 block {} to the mempool after validating restored snapshot collections and before final live-state validation",
            stats.deleted_pending_blocks,
            stats.cleared_reserved_deposits,
            stats.removed_mempool_transactions,
            stats.restored_selected_transactions,
            stats.last_applied_l2_block
        );
    }
}

pub async fn recover_from_restore_journal(client: &mongodb::Client) -> Result<(), SnapshotError> {
    let _maintenance_guard = acquire_proposer_state_maintenance_guard().await;
    recover_from_restore_journal_unlocked(client).await
}

async fn recover_from_restore_journal_unlocked(
    client: &mongodb::Client,
) -> Result<(), SnapshotError> {
    let journal = match client.get_restore_journal().await {
        Some(journal) => journal,
        None => return Ok(()),
    };

    match journal.phase {
        RestoreJournalPhase::LoadingShadow => {
            cleanup_loading_shadow_restore(client, &journal).await?;
            Ok(())
        }
        RestoreJournalPhase::SwapInProgress => {
            let mut journal = journal;
            match complete_shadow_swap_from_journal(client, &mut journal).await {
                Ok(()) => Ok(()),
                Err(error) => {
                    if matches!(error, SnapshotError::RestoreInvariantViolation(_)) {
                        if let Err(rollback_error) =
                            rollback_shadow_swap(client, &mut journal).await
                        {
                            error!(
                                "Proposer restore rollback cannot complete cleanly: {rollback_error}. Manual intervention required."
                            );
                            return Err(rollback_error);
                        }
                    }
                    Err(error)
                }
            }
        }
        RestoreJournalPhase::RollbackInProgress => {
            let mut journal = journal;
            if let Err(error) = complete_rollback_from_journal(client, &mut journal).await {
                error!(
                    "Proposer restore rollback remains incomplete: {error}. Manual intervention required."
                );
                return Err(error);
            }
            Err(SnapshotError::RestoreInvariantViolation(
                "restore was rolled back after a crash mid-rollback; original restore did not complete".to_string(),
            ))
        }
        RestoreJournalPhase::SwapComplete => {
            let mut journal = journal;
            match validate_restored_snapshot_state_before_cleanup(client).await {
                Ok(()) => complete_cleanup_after_proposer_shadow_swap(client, &journal).await,
                Err(error) => {
                    if matches!(error, SnapshotError::RestoreInvariantViolation(_)) {
                        if let Err(rollback_error) =
                            rollback_completed_swap_from_backups(client, &mut journal).await
                        {
                            error!(
                                "Proposer restore rollback from swap_complete cannot complete cleanly: {rollback_error}. Manual intervention required."
                            );
                            return Err(rollback_error);
                        }
                    }
                    Err(error)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        domain::entities::{
            Block, ClientTransactionWithMetaData, DepositDatawithFee, L1Ref, PendingBlock,
            PendingBlockState, RestoreJournal, RestoreJournalCollection, RestoreJournalPhase,
            RestoreJournalStep, SyncState, TxLifecycle,
        },
        driven::db::mongo_db::{
            ensure_deposit_indexes, StoredBlock, DEPOSIT_COLLECTION, RESTORE_JOURNAL_COLLECTION,
        },
        ports::{
            db::{BlockStorageDB, PendingBlockDB, RestoreJournalDB, SyncStateDB, TransactionsDB},
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
    };
    use alloy::primitives::{Address, Bytes, TxHash};
    use ark_ff::Zero;
    use ark_serialize::SerializationError;
    use lib::merkle_trees::trees::{MutableTree, TreeMetadata};
    use lib::nf_client_proof::Proof;
    use lib::shared_entities::{ClientTransaction, CompressedSecrets, DepositData};
    use lib::tests_utils::{get_db_connection, get_mongo};
    use serde::{Deserialize, Serialize};
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::Duration,
    };
    use tokio::sync::{Mutex, OnceCell};

    pub(crate) async fn snapshot_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) })
            .await
            .lock()
            .await
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    struct MockProof;

    impl Proof for MockProof {
        fn compress_proof(&self) -> Result<Bytes, SerializationError> {
            Ok(Bytes::new())
        }

        fn from_compressed(_compressed: Bytes) -> Result<Self, SerializationError> {
            Ok(Self)
        }
    }

    fn test_selected_client_transaction(
        seed: u32,
        block_l2: u64,
        commitment: Fr254,
    ) -> ClientTransactionWithMetaData<MockProof> {
        ClientTransactionWithMetaData {
            client_transaction: ClientTransaction {
                commitments: [commitment, Fr254::zero(), Fr254::zero(), Fr254::zero()],
                compressed_secrets: CompressedSecrets::default(),
                proof: MockProof,
                ..Default::default()
            },
            lifecycle: TxLifecycle::Selected { block_l2 },
            hash: vec![seed, seed + 1, seed + 2],
            historic_roots: vec![],
            receipt_token: None,
        }
    }

    async fn persist_sync_state(client: &mongodb::Client, sync_state: &SyncState) {
        let mut session = client.start_session().await.expect("start session");
        let client_for_write = client.clone();
        let sync_state_for_write = sync_state.clone();
        session
            .start_transaction()
            .and_run2(async move |session| {
                client_for_write
                    .update_sync_state_with_session(&sync_state_for_write, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write sync_state");

        let stored_block = client
            .get_block_by_number(sync_state.last_applied_l2_block)
            .await
            .expect("stored block should exist for test sync_state");

        let commitment_metadata = client
            .database(DB)
            .collection::<TreeMetadata<Fr254>>(&format!(
                "{}_metadata",
                <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
            ))
            .find_one(mongodb::bson::doc! { "_id": 0 })
            .await
            .expect("read commitment metadata");
        if commitment_metadata
            .as_ref()
            .is_some_and(|metadata| metadata.sub_tree_count == 0)
            && !stored_block.commitments.is_empty()
        {
            <mongodb::Client as MutableTree<Fr254>>::insert_leaf(
                client,
                Fr254::from(sync_state.last_applied_l2_block + 1),
                true,
                <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME,
            )
            .await
            .expect("materialize commitment tree state for snapshot test");
        }

        let target_historic_root_sub_tree_count = sync_state
            .last_applied_l2_block
            .checked_add(2)
            .expect("historic root count should not overflow in snapshot tests");
        let mut historic_root_sub_tree_count = client
            .database(DB)
            .collection::<TreeMetadata<Fr254>>(&format!(
                "{}_metadata",
                <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME
            ))
            .find_one(mongodb::bson::doc! { "_id": 0 })
            .await
            .expect("read historic root metadata")
            .expect("historic root metadata should exist for snapshot tests")
            .sub_tree_count;
        while historic_root_sub_tree_count < target_historic_root_sub_tree_count {
            let commitment_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
                .await
                .expect("read commitment root");
            <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
                client,
                &commitment_root,
                true,
            )
            .await
            .expect("materialize historic root state for snapshot test");
            historic_root_sub_tree_count += 1;
        }
    }

    async fn initialize_snapshot_test_trees(client: &mongodb::Client) {
        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(client)
            .await
            .expect("create deposit indexes");
    }

    async fn create_snapshot_fixture(
        client: &mongodb::Client,
        layer2_block_number: u64,
        commitment: &str,
        proposer_byte: u8,
        l1_block_number: u64,
        snapshot_root_prefix: &str,
    ) -> (PathBuf, ProposerSnapshotManifest, StoredBlock, SyncState) {
        let stored_block = StoredBlock {
            layer2_block_number,
            commitments: vec![commitment.to_string()],
            proposer_address: Address::from([proposer_byte; 20]),
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
                tx_hash: TxHash::from([proposer_byte; 32]),
                log_index: proposer_byte as u64,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(client, &sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "{snapshot_root_prefix}-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(client, &snapshot_root)
            .await
            .expect("create snapshot fixture");

        (snapshot_root, manifest, stored_block, sync_state)
    }

    async fn first_existing_live_restore_collection(
        client: &mongodb::Client,
        journal: &[RestoreJournalCollection],
    ) -> (usize, RestoreJournalCollection) {
        let existing_collections = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collections");
        restore_collections_in_swap_order(journal)
            .into_iter()
            .enumerate()
            .find(|(_, collection)| {
                existing_collections
                    .iter()
                    .any(|name| name == &collection.live)
            })
            .expect("at least one live restore collection should exist")
    }

    async fn next_required_live_restore_collection(
        client: &mongodb::Client,
        journal: &[RestoreJournalCollection],
        after_index: usize,
    ) -> (usize, RestoreJournalCollection) {
        let existing_collections = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collections");
        restore_collections_in_swap_order(journal)
            .into_iter()
            .enumerate()
            .skip(after_index + 1)
            .find(|(_, collection)| {
                existing_collections
                    .iter()
                    .any(|name| name == &collection.live)
                    && required_proposer_snapshot_collection_names()
                        .iter()
                        .any(|name| name == &collection.live)
            })
            .expect(
                "expected another required live restore collection after the first processed one",
            )
    }

    async fn write_manifest(snapshot_dir: &Path, manifest: &ProposerSnapshotManifest) {
        fs::write(
            snapshot_dir.join("manifest.json"),
            serde_json::to_vec_pretty(manifest).expect("serialize manifest"),
        )
        .await
        .expect("write manifest");
    }

    async fn read_manifest(snapshot_dir: &Path) -> ProposerSnapshotManifest {
        serde_json::from_slice(
            &fs::read(snapshot_dir.join("manifest.json"))
                .await
                .expect("read manifest"),
        )
        .expect("deserialize manifest")
    }

    async fn prepare_mid_swap_pending_state(
        client: &mongodb::Client,
        snapshot_root_prefix: &str,
        snapshot_block_number: u64,
        newer_live_block_number: u64,
    ) -> (PathBuf, RestoreJournal, SyncState) {
        initialize_snapshot_test_trees(client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            client,
            snapshot_block_number,
            "0xsnapshot",
            snapshot_block_number as u8,
            snapshot_block_number * 100,
            snapshot_root_prefix,
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: newer_live_block_number,
            commitments: vec!["0xnewer-live".to_string()],
            proposer_address: Address::from([newer_live_block_number as u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: newer_live_block_number * 100,
                tx_hash: TxHash::from([newer_live_block_number as u8; 32]),
                log_index: newer_live_block_number,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(client, &newer_live_sync_state).await;

        let mut journal = load_proposer_snapshot_into_shadow(client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        assert!(
            ordered.len() > 1,
            "expected multiple collections in restore order"
        );
        let (first_index, first) =
            first_existing_live_restore_collection(client, &journal.collections).await;

        rename_collection(client, &first.live, &first.backup, false)
            .await
            .expect("rename first live to backup");
        rename_collection(client, &first.shadow, &first.live, false)
            .await
            .expect("rename first shadow to live");

        let (next_index, _) =
            next_required_live_restore_collection(client, &journal.collections, first_index).await;

        journal.collections = ordered;
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(next_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist mid-swap journal");

        (snapshot_root, journal, newer_live_sync_state)
    }

    #[tokio::test]
    async fn recover_from_restore_journal_is_noop_when_idle() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        recover_from_restore_journal(&client)
            .await
            .expect("idle recover should be a no-op");

        assert_eq!(client.get_restore_journal().await, None);
    }

    #[tokio::test]
    async fn proposer_state_maintenance_try_lock_skips_while_held() {
        let _snapshot_test_lock = snapshot_test_lock().await;

        let guard = acquire_proposer_state_maintenance_guard().await;
        assert!(
            try_acquire_proposer_state_maintenance_guard().is_none(),
            "best-effort snapshot work should skip while maintenance is in progress"
        );
        drop(guard);

        assert!(
            try_acquire_proposer_state_maintenance_guard().is_some(),
            "maintenance lock should become available again after release"
        );
    }

    #[tokio::test]
    async fn proposer_state_maintenance_critical_path_waits_for_release() {
        let _snapshot_test_lock = snapshot_test_lock().await;

        let guard = acquire_proposer_state_maintenance_guard().await;
        let acquired = Arc::new(AtomicBool::new(false));
        let acquired_for_task = Arc::clone(&acquired);
        tokio::spawn(async move {
            let guard = acquire_proposer_state_maintenance_guard().await;
            acquired_for_task.store(true, Ordering::Release);
            drop(guard);
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(
            !acquired.load(Ordering::Acquire),
            "critical restore path should wait while a snapshot-style holder still owns the lock"
        );
        drop(guard);
    }

    #[tokio::test]
    async fn create_proposer_snapshot_waits_for_maintenance_lock_release() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let stored_block = StoredBlock {
            layer2_block_number: 1,
            commitments: vec!["0xlock-test".to_string()],
            proposer_address: Address::from([7; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");
        persist_sync_state(
            &client,
            &SyncState::new(
                stored_block.layer2_block_number,
                stored_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 100,
                    tx_hash: TxHash::from([7; 32]),
                    log_index: 0,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-lock-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let guard = acquire_proposer_state_maintenance_guard().await;
        let snapshot_client = client.clone();
        let snapshot_root_for_task = snapshot_root.clone();
        let handle = tokio::spawn(async move {
            create_proposer_snapshot(&snapshot_client, &snapshot_root_for_task).await
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(
            !handle.is_finished(),
            "public snapshot API should wait for the maintenance lock rather than racing restore work"
        );

        drop(guard);
        let manifest = handle
            .await
            .expect("snapshot task should join")
            .expect("snapshot should succeed after lock release");
        assert!(
            fs::try_exists(snapshot_root.join(manifest.snapshot_id))
                .await
                .expect("check snapshot output"),
            "snapshot directory should exist after successful snapshot creation"
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot lock root");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_cleans_loading_shadow_leftovers() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        let database = client.database(DB);

        let collections = vec![
            RestoreJournalCollection {
                live: PROPOSED_BLOCKS_COLLECTION.to_string(),
                shadow: shadow_collection_name(PROPOSED_BLOCKS_COLLECTION),
                backup: backup_collection_name(PROPOSED_BLOCKS_COLLECTION),
            },
            RestoreJournalCollection {
                live: SYNC_STATE_COLLECTION.to_string(),
                shadow: shadow_collection_name(SYNC_STATE_COLLECTION),
                backup: backup_collection_name(SYNC_STATE_COLLECTION),
            },
        ];

        for collection in &collections {
            database
                .collection::<Document>(&collection.shadow)
                .insert_one(mongodb::bson::doc! { "_id": 1, "value": "shadow" })
                .await
                .expect("seed shadow");
            database
                .collection::<Document>(&collection.backup)
                .insert_one(mongodb::bson::doc! { "_id": 1, "value": "backup" })
                .await
                .expect("seed backup");
        }

        let journal = RestoreJournal::new_loading_shadow(
            "snapshot-loading-shadow".to_string(),
            "/tmp/snapshot-loading-shadow".to_string(),
            "deadbeef".to_string(),
            collections.clone(),
            mongodb::bson::DateTime::now(),
        );
        client.upsert_restore_journal(&journal).await.unwrap();

        recover_from_restore_journal(&client)
            .await
            .expect("loading_shadow recover should clean leftovers");

        assert_eq!(client.get_restore_journal().await, None);
        let names = database
            .list_collection_names()
            .await
            .expect("list collections");
        for collection in collections {
            assert!(!names.iter().any(|name| name == &collection.shadow));
            assert!(!names.iter().any(|name| name == &collection.backup));
        }
    }

    #[tokio::test]
    async fn create_proposer_snapshot_crash_leaves_temp_dir_that_can_be_cleaned() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let stored_block = StoredBlock {
            layer2_block_number: 21,
            commitments: vec!["0xcrash".to_string()],
            proposer_address: Address::from([21u8; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");

        persist_sync_state(
            &client,
            &SyncState::new(
                stored_block.layer2_block_number,
                stored_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2100,
                    tx_hash: TxHash::from([21u8; 32]),
                    log_index: 0,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let failpoint = TestFailpointGuard::enable("snapshot_after_first_collection_exported");
        let crash_client = client.clone();
        let crash_root = snapshot_root.clone();
        let join_error =
            tokio::spawn(async move { create_proposer_snapshot(&crash_client, &crash_root).await })
                .await
                .expect_err("snapshot creation should panic at failpoint");
        assert!(join_error.is_panic());
        drop(failpoint);

        let mut temp_dirs = Vec::new();
        let mut entries = fs::read_dir(&snapshot_root)
            .await
            .expect("read snapshot root");
        while let Some(entry) = entries
            .next_entry()
            .await
            .expect("read snapshot root entry")
        {
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with(TEMP_SNAPSHOT_DIR_PREFIX)
            {
                temp_dirs.push(entry.path());
            }
        }
        assert!(
            !temp_dirs.is_empty(),
            "crash should leave at least one temp dir"
        );

        cleanup_orphaned_proposer_snapshot_temp_dirs(&snapshot_root)
            .await
            .expect("cleanup orphaned temp dirs");

        let mut entries = fs::read_dir(&snapshot_root)
            .await
            .expect("read cleaned snapshot root");
        while let Some(entry) = entries.next_entry().await.expect("read cleaned root entry") {
            assert!(
                !entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(TEMP_SNAPSHOT_DIR_PREFIX),
                "orphaned temp dir should have been removed"
            );
        }

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn create_proposer_snapshot_writes_manifest_and_collections() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let stored_block = StoredBlock {
            layer2_block_number: 7,
            commitments: vec!["0xabc".to_string()],
            proposer_address: Address::from([3u8; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");

        let sync_state = SyncState::new(
            stored_block.layer2_block_number,
            stored_block.hash().to_hex_string(),
            L1Ref {
                block_number: 1234,
                tx_hash: TxHash::from([9u8; 32]),
                log_index: 2,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        assert!(snapshot_dir.join("manifest.json").exists());
        assert!(manifest
            .collections
            .iter()
            .any(|collection| collection.collection_name == DEPOSIT_COLLECTION));
        assert!(manifest
            .collections
            .iter()
            .any(|collection| collection.collection_name == PROPOSED_BLOCKS_COLLECTION));
        assert!(manifest
            .collections
            .iter()
            .any(|collection| collection.collection_name == SYNC_STATE_COLLECTION));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_wrong_schema_version() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            31,
            "0xschema",
            31,
            3100,
            "nf4-proposer-schema-version-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest.schema_version = 999;
        write_manifest(&snapshot_dir, &broken_manifest).await;

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject unsupported schema version");
        assert!(matches!(
            error,
            SnapshotError::UnsupportedManifestSchemaVersion(999)
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_wrong_storage_format() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            32,
            "0xstorage",
            32,
            3200,
            "nf4-proposer-storage-format-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest.storage_format = "broken-format".to_string();
        write_manifest(&snapshot_dir, &broken_manifest).await;

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject unsupported storage format");
        assert!(matches!(
            error,
            SnapshotError::UnsupportedManifestStorageFormat(ref actual)
                if actual == "broken-format"
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_unexpected_manifest_collection() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            33,
            "0xunexpected-collection",
            33,
            3300,
            "nf4-proposer-unexpected-collection-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest
            .collections
            .push(SnapshotCollectionManifest {
                collection_name: "TotallyUnexpectedCollection".to_string(),
                file_name: "TotallyUnexpectedCollection.jsonl".to_string(),
                document_count: 0,
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            });
        write_manifest(&snapshot_dir, &broken_manifest).await;
        fs::write(snapshot_dir.join("TotallyUnexpectedCollection.jsonl"), b"")
            .await
            .expect("write unexpected snapshot file");

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject unexpected manifest collection");
        assert!(matches!(
            error,
            SnapshotError::UnexpectedSnapshotCollection(ref actual)
                if actual == "TotallyUnexpectedCollection"
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_invalid_manifest_file_name() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            34,
            "0xinvalid-file-name",
            34,
            3400,
            "nf4-proposer-invalid-file-name-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest
            .collections
            .first_mut()
            .expect("manifest should contain at least one collection")
            .file_name = "../../outside.jsonl".to_string();
        write_manifest(&snapshot_dir, &broken_manifest).await;

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject invalid manifest file names");
        assert!(matches!(
            error,
            SnapshotError::InvalidSnapshotFileName(ref actual) if actual == "../../outside.jsonl"
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_missing_snapshot_file() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            33,
            "0xmissing-file",
            33,
            3300,
            "nf4-proposer-missing-file-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let removed_file = manifest
            .collections
            .first()
            .expect("manifest should contain at least one collection")
            .file_name
            .clone();
        fs::remove_file(snapshot_dir.join(&removed_file))
            .await
            .expect("remove snapshot file");

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject missing snapshot file");
        assert!(matches!(
            error,
            SnapshotError::MissingSnapshotFile(ref actual) if actual == &removed_file
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_bad_collection_checksum() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            34,
            "0xbad-sha",
            34,
            3400,
            "nf4-proposer-bad-collection-sha-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let target_collection = manifest
            .collections
            .iter()
            .find(|collection| collection.document_count > 0)
            .unwrap_or_else(|| {
                manifest
                    .collections
                    .first()
                    .expect("manifest should contain at least one collection")
            });
        fs::write(
            snapshot_dir.join(&target_collection.file_name),
            b"{\"corrupted\":true}\n",
        )
        .await
        .expect("corrupt collection file");

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject bad collection checksum");
        assert!(matches!(
            error,
            SnapshotError::CollectionChecksumMismatch { ref collection_name, .. }
                if collection_name == &target_collection.collection_name
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_bad_overall_checksum() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            35,
            "0xbad-overall",
            35,
            3500,
            "nf4-proposer-bad-overall-sha-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest.overall_sha256 = "definitely-wrong".to_string();
        write_manifest(&snapshot_dir, &broken_manifest).await;

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject bad overall checksum");
        assert!(matches!(
            error,
            SnapshotError::OverallChecksumMismatch { ref expected, .. }
                if expected == "definitely-wrong"
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_rejects_manifest_sync_state_mismatch() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let (snapshot_root, manifest, _, _) = create_snapshot_fixture(
            &client,
            36,
            "0xmanifest-mismatch",
            36,
            3600,
            "nf4-proposer-manifest-mismatch-test",
        )
        .await;
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);
        let mut broken_manifest = read_manifest(&snapshot_dir).await;
        broken_manifest.last_applied_l2_block += 1;
        write_manifest(&snapshot_dir, &broken_manifest).await;

        let error = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect_err("loading snapshot should reject manifest metadata drift");
        assert!(matches!(
            error,
            SnapshotError::SnapshotManifestSyncStateMismatch {
                field: "last_applied_l2_block",
                ..
            }
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn find_latest_valid_proposer_snapshot_prefers_highest_restorable_block() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let first_block = StoredBlock {
            layer2_block_number: 4,
            commitments: vec!["0xaaa".to_string()],
            proposer_address: Address::from([4u8; 20]),
        };
        client
            .store_block(&first_block)
            .await
            .expect("store first block");
        persist_sync_state(
            &client,
            &SyncState::new(
                first_block.layer2_block_number,
                first_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 100,
                    tx_hash: TxHash::from([4u8; 32]),
                    log_index: 0,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-discovery-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let first_manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create first snapshot");

        let second_block = StoredBlock {
            layer2_block_number: 7,
            commitments: vec!["0xbbb".to_string()],
            proposer_address: Address::from([7u8; 20]),
        };
        client
            .store_block(&second_block)
            .await
            .expect("store second block");
        persist_sync_state(
            &client,
            &SyncState::new(
                second_block.layer2_block_number,
                second_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 200,
                    tx_hash: TxHash::from([7u8; 32]),
                    log_index: 0,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;
        let second_manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create second snapshot");

        let (selected_dir, selected_manifest) =
            find_latest_valid_proposer_snapshot(&snapshot_root, 4)
                .await
                .expect("discover snapshot")
                .expect("snapshot should exist");
        assert_eq!(selected_manifest.last_applied_l2_block, 4);
        assert_eq!(selected_manifest.snapshot_id, first_manifest.snapshot_id);
        assert_eq!(
            selected_dir,
            snapshot_root.join(&first_manifest.snapshot_id)
        );

        let (_, selected_manifest) = find_latest_valid_proposer_snapshot(&snapshot_root, 7)
            .await
            .expect("discover snapshot")
            .expect("snapshot should exist");
        assert_eq!(selected_manifest.last_applied_l2_block, 7);
        assert_eq!(selected_manifest.snapshot_id, second_manifest.snapshot_id);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn find_latest_valid_proposer_snapshot_ignores_temp_snapshot_dirs() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let canonical_block = StoredBlock {
            layer2_block_number: 5,
            commitments: vec!["0xaaa".to_string()],
            proposer_address: Address::from([5u8; 20]),
        };
        client
            .store_block(&canonical_block)
            .await
            .expect("store canonical block");
        persist_sync_state(
            &client,
            &SyncState::new(
                canonical_block.layer2_block_number,
                canonical_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 150,
                    tx_hash: TxHash::from([5u8; 32]),
                    log_index: 0,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-temp-discovery-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create canonical snapshot");
        let canonical_snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let temp_snapshot_dir = snapshot_root.join(format!(
            "{TEMP_SNAPSHOT_DIR_PREFIX}{}-{}",
            manifest.last_applied_l2_block + 1,
            manifest.snapshot_id
        ));
        fs::create_dir_all(&temp_snapshot_dir)
            .await
            .expect("create temp snapshot dir");
        for collection in &manifest.collections {
            fs::copy(
                canonical_snapshot_dir.join(&collection.file_name),
                temp_snapshot_dir.join(&collection.file_name),
            )
            .await
            .expect("copy collection snapshot into temp dir");
        }
        let mut temp_manifest = manifest.clone();
        temp_manifest.snapshot_id = temp_snapshot_dir
            .file_name()
            .and_then(|name| name.to_str())
            .expect("temp snapshot dir name should be valid utf-8")
            .to_string();
        temp_manifest.last_applied_l2_block += 1;
        write_manifest(&temp_snapshot_dir, &temp_manifest).await;

        let (selected_dir, selected_manifest) = find_latest_valid_proposer_snapshot(
            &snapshot_root,
            temp_manifest.last_applied_l2_block,
        )
        .await
        .expect("discover snapshot")
        .expect("snapshot should exist");

        assert_eq!(selected_dir, canonical_snapshot_dir);
        assert_eq!(selected_manifest.snapshot_id, manifest.snapshot_id);
        assert_eq!(
            selected_manifest.last_applied_l2_block,
            manifest.last_applied_l2_block
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn load_proposer_snapshot_into_shadow_populates_shadow_collections_and_journal() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(&client)
            .await
            .expect("create deposit indexes");

        let stored_block = StoredBlock {
            layer2_block_number: 8,
            commitments: vec!["0xdef".to_string()],
            proposer_address: Address::from([4u8; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");

        let sync_state = SyncState::new(
            stored_block.layer2_block_number,
            stored_block.hash().to_hex_string(),
            L1Ref {
                block_number: 5678,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-shadow-load-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");

        let stored_journal = client
            .get_restore_journal()
            .await
            .expect("restore_journal should exist");
        assert_eq!(stored_journal, journal);
        assert_eq!(
            stored_journal.phase,
            crate::domain::entities::RestoreJournalPhase::LoadingShadow
        );

        let restore_journal_docs = client
            .database(DB)
            .collection::<Document>(RESTORE_JOURNAL_COLLECTION)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count restore_journal docs");
        assert_eq!(restore_journal_docs, 1);

        for collection in &manifest.collections {
            let shadow_name = format!("restore_shadow__{}", collection.collection_name);
            let shadow_count = client
                .database(DB)
                .collection::<Document>(&shadow_name)
                .count_documents(mongodb::bson::doc! {})
                .await
                .expect("count shadow documents");
            assert_eq!(
                shadow_count, collection.document_count,
                "shadow collection {shadow_name} should contain the manifest document count"
            );
        }

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_cleans_real_loading_shadow_crash() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let stored_block = StoredBlock {
            layer2_block_number: 22,
            commitments: vec!["0xshadow".to_string()],
            proposer_address: Address::from([22u8; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");

        persist_sync_state(
            &client,
            &SyncState::new(
                stored_block.layer2_block_number,
                stored_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2200,
                    tx_hash: TxHash::from([22u8; 32]),
                    log_index: 1,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-loading-shadow-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let failpoint =
            TestFailpointGuard::enable("restore_after_first_shadow_collection_imported");
        let crash_client = client.clone();
        let crash_snapshot_dir = snapshot_dir.clone();
        let join_error = tokio::spawn(async move {
            load_proposer_snapshot_into_shadow(&crash_client, &crash_snapshot_dir).await
        })
        .await
        .expect_err("loading shadow should panic at failpoint");
        assert!(join_error.is_panic());
        drop(failpoint);

        let journal = client
            .get_restore_journal()
            .await
            .expect("journal should exist after loading_shadow crash");
        assert_eq!(journal.phase, RestoreJournalPhase::LoadingShadow);

        recover_from_restore_journal(&client)
            .await
            .expect("recover should clean loading_shadow leftovers");

        assert_eq!(client.get_restore_journal().await, None);
        let names = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collection names");
        for collection in journal.collections {
            assert!(!names.iter().any(|name| name == &collection.shadow));
            assert!(!names.iter().any(|name| name == &collection.backup));
        }

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn swap_proposer_shadow_into_live_marks_swap_complete_and_keeps_backups() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(&client)
            .await
            .expect("create deposit indexes");

        let snapshot_block = StoredBlock {
            layer2_block_number: 8,
            commitments: vec!["0xdef".to_string()],
            proposer_address: Address::from([4u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 5678,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-shadow-swap-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let new_live_block = StoredBlock {
            layer2_block_number: 9,
            commitments: vec!["0x123".to_string()],
            proposer_address: Address::from([5u8; 20]),
        };
        client
            .store_block(&new_live_block)
            .await
            .expect("store newer live block");

        let new_live_sync_state = SyncState::new(
            new_live_block.layer2_block_number,
            new_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 91011,
                tx_hash: TxHash::from([7u8; 32]),
                log_index: 5,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &new_live_sync_state).await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let journal = swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        assert_eq!(journal.phase, RestoreJournalPhase::SwapComplete);
        assert_eq!(journal.current_index, None);
        assert_eq!(
            journal.current_step,
            Some(RestoreJournalStep::NonSnapshotCleanupPending)
        );

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("live sync_state should exist");
        assert_eq!(live_sync_state.last_applied_l2_block, 8);
        assert_eq!(
            live_sync_state.fingerprint,
            snapshot_block.hash().to_hex_string()
        );

        let live_blocks_count = client
            .database(DB)
            .collection::<Document>(PROPOSED_BLOCKS_COLLECTION)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count live proposed blocks");
        assert_eq!(live_blocks_count, 1);

        let backup_blocks_count = client
            .database(DB)
            .collection::<Document>(&backup_collection_name(PROPOSED_BLOCKS_COLLECTION))
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count backup proposed blocks");
        assert_eq!(backup_blocks_count, 2);

        let shadow_sync_state_exists = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collections")
            .iter()
            .any(|name| name == &shadow_collection_name(SYNC_STATE_COLLECTION));
        assert!(
            !shadow_sync_state_exists,
            "shadow sync_state should have been renamed away"
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_resumes_real_mid_swap_crash() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 23,
            commitments: vec!["0xresume".to_string()],
            proposer_address: Address::from([23u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        persist_sync_state(
            &client,
            &SyncState::new(
                snapshot_block.layer2_block_number,
                snapshot_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2300,
                    tx_hash: TxHash::from([23u8; 32]),
                    log_index: 2,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-mid-swap-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 24,
            commitments: vec!["0xlive".to_string()],
            proposer_address: Address::from([24u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        persist_sync_state(
            &client,
            &SyncState::new(
                newer_live_block.layer2_block_number,
                newer_live_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2400,
                    tx_hash: TxHash::from([24u8; 32]),
                    log_index: 3,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");

        let failpoint = TestFailpointGuard::enable("restore_after_backup_created");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { swap_proposer_shadow_into_live(&crash_client).await })
                .await
                .expect_err("swap should panic at failpoint");
        assert!(join_error.is_panic());
        drop(failpoint);

        let journal = client
            .get_restore_journal()
            .await
            .expect("journal should exist after mid-swap crash");
        assert_eq!(journal.phase, RestoreJournalPhase::SwapInProgress);
        assert_eq!(
            journal.current_step,
            Some(RestoreJournalStep::BackupCreated)
        );

        recover_from_restore_journal(&client)
            .await
            .expect("recover should resume swap");

        let resumed = client
            .get_restore_journal()
            .await
            .expect("journal should remain at swap_complete after resume");
        assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("restored live sync_state should exist");
        assert_eq!(
            live_sync_state.last_applied_l2_block,
            snapshot_block.layer2_block_number
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_rolls_back_mid_swap_crash_before_backup_created_journal_persist(
    ) {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 27,
            commitments: vec!["0xpre-journal-crash".to_string()],
            proposer_address: Address::from([27u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        persist_sync_state(
            &client,
            &SyncState::new(
                snapshot_block.layer2_block_number,
                snapshot_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2700,
                    tx_hash: TxHash::from([27u8; 32]),
                    log_index: 2,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-pre-backup-created-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 28,
            commitments: vec!["0xnewer-live".to_string()],
            proposer_address: Address::from([28u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2800,
                tx_hash: TxHash::from([28u8; 32]),
                log_index: 3,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");

        let failpoint =
            TestFailpointGuard::enable("restore_after_live_rename_before_backup_created");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { swap_proposer_shadow_into_live(&crash_client).await })
                .await
                .expect_err("swap should panic after live rename and before journal persist");
        assert!(join_error.is_panic());
        drop(failpoint);

        let journal = client
            .get_restore_journal()
            .await
            .expect("journal should exist after pre-journal mid-swap crash");
        assert_eq!(journal.phase, RestoreJournalPhase::SwapInProgress);
        assert_eq!(
            journal.current_step,
            Some(RestoreJournalStep::BackupPending)
        );

        let ordered = restore_collections_in_swap_order(&journal.collections);
        let current_collection = &ordered[journal.current_index.expect("current_index") as usize];
        client
            .database(DB)
            .collection::<Document>(&current_collection.shadow)
            .drop()
            .await
            .expect("drop current shadow collection to force rollback");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("recovery should roll back after resume detects broken current shadow");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("pre-restore live sync_state should be restored");
        assert_eq!(live_sync_state, newer_live_sync_state);

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_rolls_back_broken_invariant_and_restores_live_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let (snapshot_root, journal, newer_live_sync_state) =
            prepare_mid_swap_pending_state(&client, "nf4-proposer-rollback-success-test", 41, 42)
                .await;

        let ordered = restore_collections_in_swap_order(&journal.collections);
        let broken_collection = &ordered[journal.current_index.expect("current_index") as usize];
        client
            .database(DB)
            .collection::<Document>(&broken_collection.live)
            .drop()
            .await
            .expect("drop live collection to force invariant violation");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("recovery should surface invariant violation after rollback");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));

        assert_eq!(
            client.get_restore_journal().await,
            None,
            "successful rollback should clear restore_journal"
        );

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("pre-swap live sync_state should be restored");
        assert_eq!(live_sync_state, newer_live_sync_state);

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_resumes_real_mid_rollback_crash() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let (snapshot_root, journal, newer_live_sync_state) =
            prepare_mid_swap_pending_state(&client, "nf4-proposer-mid-rollback-crash-test", 51, 52)
                .await;

        let ordered = restore_collections_in_swap_order(&journal.collections);
        let broken_collection = &ordered[journal.current_index.expect("current_index") as usize];
        client
            .database(DB)
            .collection::<Document>(&broken_collection.live)
            .drop()
            .await
            .expect("drop live collection to force rollback path");

        let failpoint = TestFailpointGuard::enable("restore_after_first_rollback_progress_persist");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { recover_from_restore_journal(&crash_client).await })
                .await
                .expect_err("rollback recovery should panic at failpoint");
        assert!(join_error.is_panic());
        drop(failpoint);

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after rollback crash");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        let expected_rollback_index = journal
            .current_index
            .and_then(|index| index.checked_sub(1))
            .expect("mid-rollback crash should start after at least one processed collection");
        assert_eq!(
            persisted_journal.current_index,
            Some(expected_rollback_index)
        );
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("resume should surface original invariant violation after rollback");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("pre-swap live sync_state should be restored after resumed rollback");
        assert_eq!(live_sync_state, newer_live_sync_state);

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_resumes_after_crash_post_backup_restore_pre_rollback_applied(
    ) {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let (snapshot_root, mut journal, newer_live_sync_state) = prepare_mid_swap_pending_state(
            &client,
            "nf4-proposer-mid-rollback-live-restore-crash-test",
            61,
            62,
        )
        .await;

        let ordered = restore_collections_in_swap_order(&journal.collections);
        let (first_processed_index, _) =
            first_existing_live_restore_collection(&client, &journal.collections).await;
        let rollback_trigger_index = first_processed_index + 1;
        assert!(
            rollback_trigger_index < ordered.len(),
            "expected another collection after the first processed live collection"
        );
        let broken_collection = &ordered[rollback_trigger_index];
        journal.collections = ordered.clone();
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(rollback_trigger_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist rollback live-restore crash fixture journal");

        drop_collection_if_exists(&client.database(DB), &broken_collection.live)
            .await
            .expect("drop current live collection when present");
        client
            .database(DB)
            .collection::<Document>(&broken_collection.shadow)
            .drop()
            .await
            .expect("drop current shadow collection to force rollback path");

        let failpoint = TestFailpointGuard::enable("restore_after_first_rollback_live_restore");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { recover_from_restore_journal(&crash_client).await })
                .await
                .expect_err(
                    "rollback recovery should panic after restoring the first live collection",
                );
        assert!(join_error.is_panic());
        drop(failpoint);

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after rollback crash");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        let expected_rollback_index = rollback_trigger_index
            .checked_sub(1)
            .expect("rollback should start at the first processed collection")
            as u32;
        assert_eq!(
            persisted_journal.current_index,
            Some(expected_rollback_index)
        );
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("resume should complete rollback and surface the original restore failure");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("pre-swap live sync_state should be restored after resumed rollback");
        assert_eq!(live_sync_state, newer_live_sync_state);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_resumes_optional_drop_after_crash_at_rollback_started() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 63,
            commitments: Vec::new(),
            proposer_address: Address::from([63u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 6300,
                tx_hash: TxHash::from([63u8; 32]),
                log_index: 63,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-optional-rollback-drop-resume-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let commitment_nodes_collection = format!(
            "{}_nodes",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        );
        assert!(
            !manifest
                .collections
                .iter()
                .any(|collection| collection.collection_name == commitment_nodes_collection),
            "snapshot fixture should omit empty optional commitment nodes collection"
        );
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 64,
            commitments: Vec::new(),
            proposer_address: Address::from([64u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 6400,
                tx_hash: TxHash::from([64u8; 32]),
                log_index: 64,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let target_index = ordered
            .iter()
            .position(|collection| collection.live == commitment_nodes_collection)
            .expect("target optional collection should exist in restore order");
        let rollback_trigger_index = target_index + 1;
        assert!(
            rollback_trigger_index < ordered.len(),
            "expected a collection after the optional rollback target"
        );

        let database = client.database(DB);
        for collection in ordered.iter().take(target_index + 1) {
            let layout = inspect_restore_collection_layout(&database, collection)
                .await
                .expect("inspect processed prefix layout");
            if layout.live_exists {
                rename_collection(&client, &collection.live, &collection.backup, false)
                    .await
                    .expect("move processed live collection to backup");
            }
            if layout.shadow_exists {
                rename_collection(&client, &collection.shadow, &collection.live, false)
                    .await
                    .expect("move processed shadow collection into live");
            }
        }

        let target = &ordered[target_index];
        assert!(
            collection_exists(&database, &target.live)
                .await
                .expect("inspect optional live collection after forward swap"),
            "optional target should be live before rollback begins"
        );
        assert!(
            !collection_exists(&database, &target.backup)
                .await
                .expect("inspect optional backup collection"),
            "optional target should have no backup when rollback action is drop"
        );

        let broken_collection = &ordered[rollback_trigger_index];
        database
            .collection::<Document>(&broken_collection.live)
            .drop()
            .await
            .expect("drop next live collection to force rollback path");

        journal.collections = ordered.clone();
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(rollback_trigger_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist rollback crash fixture journal");

        let failpoint = TestFailpointGuard::enable("restore_after_first_rollback_progress_persist");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { recover_from_restore_journal(&crash_client).await })
                .await
                .expect_err("rollback recovery should panic after persisting RollbackStarted");
        assert!(join_error.is_panic());
        drop(failpoint);

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after rollback crash");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        assert_eq!(persisted_journal.current_index, Some(target_index as u32));
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );
        assert!(
            collection_exists(&database, &target.live)
                .await
                .expect("inspect optional live collection before resumed rollback"),
            "optional live collection should still exist immediately after the crash"
        );

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("resume should complete rollback and surface the original restore failure");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);
        assert_eq!(client.get_sync_state().await, Some(newer_live_sync_state));
        assert!(
            !collection_exists(&database, &target.live)
                .await
                .expect("inspect optional live collection after resumed rollback"),
            "resumed rollback should drop the optional live collection when no backup existed"
        );

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_fails_closed_when_optional_live_state_changes_after_rollback_started(
    ) {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 65,
            commitments: Vec::new(),
            proposer_address: Address::from([65u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 6500,
                tx_hash: TxHash::from([65u8; 32]),
                log_index: 65,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-optional-rollback-stale-live-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let commitment_nodes_collection = format!(
            "{}_nodes",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        );
        assert!(
            !manifest
                .collections
                .iter()
                .any(|collection| collection.collection_name == commitment_nodes_collection),
            "snapshot fixture should omit empty optional commitment nodes collection"
        );
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 66,
            commitments: Vec::new(),
            proposer_address: Address::from([66u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 6600,
                tx_hash: TxHash::from([66u8; 32]),
                log_index: 66,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let target_index = ordered
            .iter()
            .position(|collection| collection.live == commitment_nodes_collection)
            .expect("target optional collection should exist in restore order");
        let rollback_trigger_index = target_index + 1;
        assert!(
            rollback_trigger_index < ordered.len(),
            "expected a collection after the optional rollback target"
        );

        let database = client.database(DB);
        for collection in ordered.iter().take(target_index + 1) {
            let layout = inspect_restore_collection_layout(&database, collection)
                .await
                .expect("inspect processed prefix layout");
            if layout.live_exists {
                rename_collection(&client, &collection.live, &collection.backup, false)
                    .await
                    .expect("move processed live collection to backup");
            }
            if layout.shadow_exists {
                rename_collection(&client, &collection.shadow, &collection.live, false)
                    .await
                    .expect("move processed shadow collection into live");
            }
        }

        let target = &ordered[target_index];
        let broken_collection = &ordered[rollback_trigger_index];
        database
            .collection::<Document>(&broken_collection.live)
            .drop()
            .await
            .expect("drop next live collection to force rollback path");

        journal.collections = ordered.clone();
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(rollback_trigger_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist rollback crash fixture journal");

        let failpoint = TestFailpointGuard::enable("restore_after_first_rollback_progress_persist");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { recover_from_restore_journal(&crash_client).await })
                .await
                .expect_err("rollback recovery should panic after persisting RollbackStarted");
        assert!(join_error.is_panic());
        drop(failpoint);

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after rollback crash");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        assert_eq!(persisted_journal.current_index, Some(target_index as u32));
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );

        client
            .database(DB)
            .collection::<Document>(&target.live)
            .insert_one(mongodb::bson::doc! { "_id": 4242_i64, "value": "stale-after-crash" })
            .await
            .expect("mutate optional live collection after crash to simulate stale state");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("resume should fail closed when optional live state no longer matches the restored snapshot");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after fail-closed optional rollback resume");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        assert_eq!(persisted_journal.current_index, Some(target_index as u32));
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );
        assert!(
            collection_exists(&database, &target.live)
                .await
                .expect("inspect optional live collection after fail-closed recovery"),
            "fail-closed recovery should leave the optional live collection in place for manual inspection"
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_keeps_journal_when_rollback_fails() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        let (snapshot_root, journal, _) =
            prepare_mid_swap_pending_state(&client, "nf4-proposer-rollback-failure-test", 43, 44)
                .await;

        let (first_processed_index, first_processed) =
            first_existing_live_restore_collection(&client, &journal.collections).await;
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let broken_collection = &ordered[journal.current_index.expect("current_index") as usize];

        client
            .database(DB)
            .collection::<Document>(&broken_collection.live)
            .drop()
            .await
            .expect("drop live collection to trigger invariant violation");
        client
            .database(DB)
            .collection::<Document>(&first_processed.backup)
            .drop()
            .await
            .expect("drop backup collection to make rollback impossible");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("recovery should fail when rollback itself cannot complete");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain when rollback fails");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        assert_eq!(
            persisted_journal.current_index,
            Some(first_processed_index as u32)
        );
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn cleanup_after_shadow_swap_returns_to_idle_and_keeps_live_restored_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(&client)
            .await
            .expect("create deposit indexes");

        let snapshot_pending_deposit = DepositDatawithFee {
            fee: Fr254::from(3u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(101u64),
                nf_slot_id: Fr254::from(102u64),
                value: Fr254::from(103u64),
                secret_hash: Fr254::from(104u64),
            },
            reserved: false,
        };
        <mongodb::Client as TransactionsDB<lib::plonk_prover::plonk_proof::PlonkProof>>::set_mempool_deposits(
            &client,
            vec![snapshot_pending_deposit],
        )
        .await
        .expect("store snapshot pending deposit");

        let snapshot_block = StoredBlock {
            layer2_block_number: 10,
            commitments: vec!["0xbeef".to_string()],
            proposer_address: Address::from([6u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 1112,
                tx_hash: TxHash::from([6u8; 32]),
                log_index: 6,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-shadow-cleanup-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 11,
            commitments: vec!["0xcafe".to_string()],
            proposer_address: Address::from([7u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");

        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 1314,
                tx_hash: TxHash::from([5u8; 32]),
                log_index: 7,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        let stale_live_only_deposit = DepositDatawithFee {
            fee: Fr254::from(8u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(201u64),
                nf_slot_id: Fr254::from(202u64),
                value: Fr254::from(203u64),
                secret_hash: Fr254::from(204u64),
            },
            reserved: false,
        };
        <mongodb::Client as TransactionsDB<lib::plonk_prover::plonk_proof::PlonkProof>>::set_mempool_deposits(
            &client,
            vec![stale_live_only_deposit],
        )
        .await
        .expect("store stale live-only deposit");

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");
        cleanup_after_proposer_shadow_swap(&client)
            .await
            .expect("cleanup after swap");

        assert_eq!(
            client.get_restore_journal().await,
            None,
            "cleanup should return the restore flow to idle"
        );

        let collection_names = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collections");
        for collection in manifest.collections {
            assert!(
                !collection_names
                    .iter()
                    .any(|name| name == &backup_collection_name(&collection.collection_name)),
                "backup collection should be removed for {}",
                collection.collection_name
            );
            assert!(
                !collection_names
                    .iter()
                    .any(|name| name == &shadow_collection_name(&collection.collection_name)),
                "shadow collection should be removed for {}",
                collection.collection_name
            );
        }

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("live sync_state should still exist");
        assert_eq!(live_sync_state.last_applied_l2_block, 10);
        assert_eq!(
            live_sync_state.fingerprint,
            snapshot_block.hash().to_hex_string()
        );

        let live_blocks_count = client
            .database(DB)
            .collection::<Document>(PROPOSED_BLOCKS_COLLECTION)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count live proposed blocks");
        assert_eq!(live_blocks_count, 1);

        let restored_deposits = <mongodb::Client as TransactionsDB<
            lib::plonk_prover::plonk_proof::PlonkProof,
        >>::get_mempool_deposits(&client)
        .await
        .expect("restored deposits should exist");
        assert_eq!(restored_deposits, vec![snapshot_pending_deposit]);

        let duplicate_insert_error = client
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION)
            .insert_one(snapshot_pending_deposit)
            .await
            .expect_err("restored deposit collection should keep its unique index");
        assert!(duplicate_insert_error.to_string().contains("E11000"));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn restore_proposer_snapshot_cleans_non_snapshotted_recovery_state_before_validation() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let selected_commitment = Fr254::from(777u64);
        let snapshot_block = StoredBlock {
            layer2_block_number: 10,
            commitments: vec![selected_commitment.to_hex_string()],
            proposer_address: Address::from([8u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2000,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 8,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-restore-cleans-nonsnapshotted-state-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let reserved_deposit = DepositDatawithFee {
            fee: Fr254::from(9u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(301u64),
                nf_slot_id: Fr254::from(302u64),
                value: Fr254::from(303u64),
                secret_hash: Fr254::from(304u64),
            },
            reserved: true,
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![reserved_deposit],
        )
        .await
        .expect("store reserved deposit outside the snapshot");

        let selected_transaction = test_selected_client_transaction(
            41,
            snapshot_sync_state.last_applied_l2_block,
            selected_commitment,
        );
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store lingering selected transaction outside the snapshot");

        let speculative_selected_transaction = test_selected_client_transaction(
            42,
            snapshot_sync_state.last_applied_l2_block + 1,
            Fr254::from(902u64),
        );
        client
            .store_transaction(speculative_selected_transaction.clone())
            .await
            .expect("store speculative selected transaction outside the snapshot");

        client
            .store_pending_block(&PendingBlock {
                layer2_block_number: snapshot_sync_state.last_applied_l2_block + 1,
                state: PendingBlockState::ReadyToPropose,
                broadcast_tx_hash: None,
                broadcast_receipt_checks: 0,
                block: Some(Block::default()),
                selected_deposits: vec![vec![reserved_deposit]],
                selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
            })
            .await
            .expect("store stale pending block outside the snapshot");

        let removed_mempool_transaction =
            test_selected_client_transaction(51, 0, Fr254::from(901u64));
        let removed_mempool_transaction = ClientTransactionWithMetaData {
            lifecycle: TxLifecycle::Mempool,
            ..removed_mempool_transaction
        };
        client
            .store_transaction(removed_mempool_transaction.clone())
            .await
            .expect("store stale mempool transaction outside the snapshot");

        let restored_sync_state = restore_proposer_snapshot(&client, &snapshot_dir)
            .await
            .expect("restore should clean non-snapshotted recovery state before validation");
        assert_eq!(restored_sync_state, snapshot_sync_state);

        assert!(
            client
                .get_all_pending_blocks()
                .await
                .expect("read pending blocks after restore")
                .is_empty(),
            "restore should discard lingering PendingBlocks that are not part of the snapshot"
        );
        assert_eq!(
            client
                .database(DB)
                .collection::<Document>(DEPOSIT_COLLECTION)
                .count_documents(mongodb::bson::doc! { "reserved": true })
                .await
                .expect("count reserved deposits after restore"),
            0,
            "restore should clear lingering reserved deposits before validation"
        );
        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect("selected transaction should still exist after restore cleanup")
            .lifecycle
            .is_selected(),
            "restore should preserve selected transactions already canonical at the restored sync_state height"
        );
        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &speculative_selected_transaction.hash,
            )
            .await
            .expect("speculative selected transaction should still exist after restore cleanup")
            .lifecycle
            .is_mempool(),
            "restore should normalize speculative Selected transactions beyond the restored sync_state back to the mempool before validation"
        );
        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &removed_mempool_transaction.hash,
            )
            .await
            .is_none(),
            "restore should still drop non-selected mempool transactions during recovery"
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn restore_replaces_absent_optional_live_collections_with_empty_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 3,
            commitments: Vec::new(),
            proposer_address: Address::from([3u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 303,
                tx_hash: TxHash::from([3u8; 32]),
                log_index: 3,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-optional-live-reset-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let commitment_nodes_collection = format!(
            "{}_nodes",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        );
        assert!(
            !manifest
                .collections
                .iter()
                .any(|collection| collection.collection_name == commitment_nodes_collection),
            "snapshot fixture should omit optional empty commitment nodes collection"
        );
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        client
            .database(DB)
            .collection::<Document>(&commitment_nodes_collection)
            .insert_one(mongodb::bson::doc! { "_id": 999_i64, "value": "stale" })
            .await
            .expect("seed stale live commitment nodes");

        let newer_live_block = StoredBlock {
            layer2_block_number: 4,
            commitments: Vec::new(),
            proposer_address: Address::from([4u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 404,
                tx_hash: TxHash::from([4u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        restore_proposer_snapshot(&client, &snapshot_dir)
            .await
            .expect("restore snapshot");

        let live_commitment_nodes_count = client
            .database(DB)
            .collection::<Document>(&commitment_nodes_collection)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count restored commitment nodes");
        assert_eq!(live_commitment_nodes_count, 0);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_rolls_back_when_optional_shadow_disappears_before_swap() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 5,
            commitments: Vec::new(),
            proposer_address: Address::from([5u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 505,
                tx_hash: TxHash::from([5u8; 32]),
                log_index: 5,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-optional-shadow-missing-recovery-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let commitment_nodes_collection = format!(
            "{}_nodes",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        );
        assert!(
            !manifest
                .collections
                .iter()
                .any(|collection| collection.collection_name == commitment_nodes_collection),
            "snapshot fixture should omit empty optional commitment nodes collection"
        );
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        client
            .database(DB)
            .collection::<Document>(&commitment_nodes_collection)
            .insert_one(mongodb::bson::doc! { "_id": 111_i64, "value": "stale-live-node" })
            .await
            .expect("seed stale optional live collection");

        let newer_live_block = StoredBlock {
            layer2_block_number: 6,
            commitments: Vec::new(),
            proposer_address: Address::from([6u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 606,
                tx_hash: TxHash::from([6u8; 32]),
                log_index: 6,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let target_index = ordered
            .iter()
            .position(|collection| collection.live == commitment_nodes_collection)
            .expect("target optional collection should be part of restore ordering");

        let database = client.database(DB);
        for collection in ordered.iter().take(target_index) {
            let layout = inspect_restore_collection_layout(&database, collection)
                .await
                .expect("inspect processed prefix layout");
            if layout.live_exists {
                rename_collection(&client, &collection.live, &collection.backup, false)
                    .await
                    .expect("move processed live collection to backup");
            }
            if layout.shadow_exists {
                rename_collection(&client, &collection.shadow, &collection.live, false)
                    .await
                    .expect("move processed shadow collection into live");
            }
        }

        let target = &ordered[target_index];
        database
            .collection::<Document>(&target.shadow)
            .drop()
            .await
            .expect("drop optional shadow collection before it is swapped");

        journal.collections = ordered;
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(target_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist mid-swap journal at optional collection");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("recovery should roll back when an optional shadow disappears");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);
        assert_eq!(
            client.get_sync_state().await,
            Some(newer_live_sync_state.clone())
        );

        let live_commitment_nodes_count = client
            .database(DB)
            .collection::<Document>(&commitment_nodes_collection)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count optional live collection after rollback");
        assert_eq!(
            live_commitment_nodes_count, 0,
            "rollback should not keep the stale optional live collection when its shadow vanished before swap"
        );

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_accepts_optional_collection_already_restored_without_backup(
    ) {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 7,
            commitments: Vec::new(),
            proposer_address: Address::from([7u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 707,
                tx_hash: TxHash::from([7u8; 32]),
                log_index: 7,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-optional-live-restored-recovery-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let commitment_nodes_collection = format!(
            "{}_nodes",
            <mongodb::Client as CommitmentTree<Fr254>>::TREE_NAME
        );
        assert!(
            !manifest
                .collections
                .iter()
                .any(|collection| collection.collection_name == commitment_nodes_collection),
            "snapshot fixture should omit empty optional commitment nodes collection"
        );
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 8,
            commitments: Vec::new(),
            proposer_address: Address::from([8u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 808,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 8,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let target_index = ordered
            .iter()
            .position(|collection| collection.live == commitment_nodes_collection)
            .expect("target optional collection should be part of restore ordering");

        let database = client.database(DB);
        for collection in ordered.iter().take(target_index) {
            let layout = inspect_restore_collection_layout(&database, collection)
                .await
                .expect("inspect processed prefix layout");
            if layout.live_exists {
                rename_collection(&client, &collection.live, &collection.backup, false)
                    .await
                    .expect("move processed live collection to backup");
            }
            if layout.shadow_exists {
                rename_collection(&client, &collection.shadow, &collection.live, false)
                    .await
                    .expect("move processed shadow collection into live");
            }
        }

        let target = &ordered[target_index];
        rename_collection(&client, &target.shadow, &target.live, false)
            .await
            .expect("simulate optional shadow already renamed into live");

        journal.collections = ordered;
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(target_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupPending);
        journal.updated_at = mongodb::bson::DateTime::now();
        client
            .upsert_restore_journal(&journal)
            .await
            .expect("persist mid-swap journal at already-restored optional collection");

        recover_from_restore_journal(&client).await.expect(
            "recovery should accept optional collection already restored from the snapshot",
        );

        let resumed = client
            .get_restore_journal()
            .await
            .expect("journal should remain at swap_complete after resume");
        assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);
        assert_eq!(client.get_sync_state().await, Some(snapshot_sync_state));

        let live_commitment_nodes_count = client
            .database(DB)
            .collection::<Document>(&commitment_nodes_collection)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count optional live collection after resume");
        assert_eq!(live_commitment_nodes_count, 0);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_resumes_swap_in_progress() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(&client)
            .await
            .expect("create deposit indexes");

        let snapshot_block = StoredBlock {
            layer2_block_number: 12,
            commitments: vec!["0xaaa".to_string()],
            proposer_address: Address::from([8u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2122,
                tx_hash: TxHash::from([4u8; 32]),
                log_index: 8,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-recover-swap-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 13,
            commitments: vec!["0xbbb".to_string()],
            proposer_address: Address::from([9u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");

        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2324,
                tx_hash: TxHash::from([3u8; 32]),
                log_index: 9,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;

        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let (first_index, first) =
            first_existing_live_restore_collection(&client, &journal.collections).await;

        rename_collection(&client, &first.live, &first.backup, false)
            .await
            .expect("simulate backup rename");

        journal.collections = ordered;
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(first_index as u32);
        journal.current_step = Some(RestoreJournalStep::BackupCreated);
        journal.updated_at = mongodb::bson::DateTime::now();
        client.upsert_restore_journal(&journal).await.unwrap();

        recover_from_restore_journal(&client)
            .await
            .expect("recover should resume swap");

        let resumed = client
            .get_restore_journal()
            .await
            .expect("journal should remain at swap_complete after resume");
        assert_eq!(resumed.phase, RestoreJournalPhase::SwapComplete);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("live sync_state should exist");
        assert_eq!(live_sync_state.last_applied_l2_block, 12);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn cleanup_after_shadow_swap_keeps_backups_when_restored_state_is_incoherent() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 6,
            commitments: Vec::new(),
            proposer_address: Address::from([6u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 606,
                tx_hash: TxHash::from([6u8; 32]),
                log_index: 6,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-cleanup-validation-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 7,
            commitments: Vec::new(),
            proposer_address: Address::from([7u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 707,
                tx_hash: TxHash::from([7u8; 32]),
                log_index: 7,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        let reserved_deposit = DepositDatawithFee {
            fee: Fr254::from(17u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(701u64),
                nf_slot_id: Fr254::from(702u64),
                value: Fr254::from(703u64),
                secret_hash: Fr254::from(704u64),
            },
            reserved: true,
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![reserved_deposit],
        )
        .await
        .expect("store reserved deposit outside the snapshot");

        let selected_transaction = test_selected_client_transaction(
            61,
            newer_live_sync_state.last_applied_l2_block,
            Fr254::from(761u64),
        );
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store lingering selected transaction outside the snapshot");

        client
            .store_pending_block(&PendingBlock {
                layer2_block_number: newer_live_sync_state.last_applied_l2_block + 1,
                state: PendingBlockState::ReadyToPropose,
                broadcast_tx_hash: None,
                broadcast_receipt_checks: 0,
                block: Some(Block::default()),
                selected_deposits: vec![vec![reserved_deposit]],
                selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
            })
            .await
            .expect("store stale pending block outside the snapshot");

        let mempool_transaction = ClientTransactionWithMetaData {
            lifecycle: TxLifecycle::Mempool,
            ..test_selected_client_transaction(71, 0, Fr254::from(771u64))
        };
        client
            .store_transaction(mempool_transaction.clone())
            .await
            .expect("store mempool transaction outside the snapshot");

        client
            .delete_block_by_number(snapshot_block.layer2_block_number)
            .await
            .expect("delete restored stored block");

        let error = cleanup_after_proposer_shadow_swap(&client)
            .await
            .expect_err("cleanup should refuse to discard backups for incoherent restore");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert!(
            client.get_restore_journal().await.is_some(),
            "restore journal should be preserved for manual recovery"
        );
        assert!(
            client
                .database(DB)
                .list_collection_names()
                .await
                .expect("list collections")
                .iter()
                .any(|name| name == &backup_collection_name(PROPOSED_BLOCKS_COLLECTION)),
            "backup collections should remain available for rollback"
        );
        assert_eq!(
            client
                .get_all_pending_blocks()
                .await
                .expect("read pending blocks after failed cleanup")
                .len(),
            1,
            "failed cleanup should not discard non-snapshotted PendingBlocks before validation succeeds"
        );
        assert_eq!(
            client
                .database(DB)
                .collection::<Document>(DEPOSIT_COLLECTION)
                .count_documents(mongodb::bson::doc! { "reserved": true })
                .await
                .expect("count reserved deposits after failed cleanup"),
            1,
            "failed cleanup should not clear reserved deposits before snapshot validation succeeds"
        );
        assert!(matches!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect("selected transaction should still exist after failed cleanup")
            .lifecycle,
            TxLifecycle::Selected { .. }
        ));
        assert!(matches!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &mempool_transaction.hash,
            )
            .await
            .expect("mempool transaction should still exist after failed cleanup")
            .lifecycle,
            TxLifecycle::Mempool
        ));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn create_proposer_snapshot_rejects_tree_state_ahead_of_sync_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let stored_block = StoredBlock {
            layer2_block_number: 0,
            commitments: Vec::new(),
            proposer_address: Address::from([1u8; 20]),
        };
        client
            .store_block(&stored_block)
            .await
            .expect("store block");
        let sync_state = SyncState::new(
            stored_block.layer2_block_number,
            stored_block.hash().to_hex_string(),
            L1Ref {
                block_number: 100,
                tx_hash: TxHash::from([1u8; 32]),
                log_index: 1,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &sync_state).await;
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::from(1u64),
            true,
        )
        .await
        .expect("append applied historic root");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::from(2u64),
            true,
        )
        .await
        .expect("append extra historic root");

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-inconsistent-state-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let error = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect_err("snapshot should reject tree state ahead of sync_state");
        assert!(error.to_string().contains("creating snapshot"));
    }

    #[tokio::test]
    async fn create_proposer_snapshot_rejects_stored_block_ahead_of_sync_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let applied_block = StoredBlock {
            layer2_block_number: 0,
            commitments: Vec::new(),
            proposer_address: Address::from([2u8; 20]),
        };
        client
            .store_block(&applied_block)
            .await
            .expect("store applied block");
        persist_sync_state(
            &client,
            &SyncState::new(
                applied_block.layer2_block_number,
                applied_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 200,
                    tx_hash: TxHash::from([2u8; 32]),
                    log_index: 2,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let speculative_block = StoredBlock {
            layer2_block_number: 1,
            commitments: vec!["0xspeculative".to_string()],
            proposer_address: Address::from([3u8; 20]),
        };
        client
            .store_block(&speculative_block)
            .await
            .expect("store speculative block ahead of sync_state");

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-ahead-stored-block-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let error = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect_err("snapshot should reject speculative StoredBlocks ahead of sync_state");
        assert!(error
            .to_string()
            .contains("highest StoredBlock 1 is ahead of sync_state-applied block 0"));
    }

    #[tokio::test]
    async fn create_proposer_snapshot_rejects_reserved_deposits_ahead_of_sync_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let applied_block = StoredBlock {
            layer2_block_number: 0,
            commitments: Vec::new(),
            proposer_address: Address::from([4u8; 20]),
        };
        client
            .store_block(&applied_block)
            .await
            .expect("store applied block");
        persist_sync_state(
            &client,
            &SyncState::new(
                applied_block.layer2_block_number,
                applied_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 400,
                    tx_hash: TxHash::from([4u8; 32]),
                    log_index: 4,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![DepositDatawithFee {
                fee: Fr254::from(4u64),
                deposit_data: DepositData {
                    nf_token_id: Fr254::from(41u64),
                    nf_slot_id: Fr254::from(42u64),
                    value: Fr254::from(43u64),
                    secret_hash: Fr254::from(44u64),
                },
                reserved: true,
            }],
        )
        .await
        .expect("store reserved deposit");

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-snapshot-reserved-deposits-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let error = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect_err("snapshot should reject reserved deposits ahead of sync_state");
        assert!(error.to_string().contains(
            "Deposits contains 1 reserved selection(s) ahead of sync_state-applied block 0"
        ));
    }

    #[tokio::test]
    async fn recover_from_restore_journal_completes_swap_complete_cleanup() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(&client, 29, 3)
            .await
            .expect("create commitment tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(&client, 32)
            .await
            .expect("create historic root tree");
        <mongodb::Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
            &client,
            &Fr254::zero(),
            true,
        )
        .await
        .expect("append zero historic root");
        <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(&client, 29, 3)
            .await
            .expect("create nullifier tree");
        ensure_deposit_indexes(&client)
            .await
            .expect("create deposit indexes");

        let snapshot_block = StoredBlock {
            layer2_block_number: 14,
            commitments: vec!["0xccc".to_string()],
            proposer_address: Address::from([10u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2526,
                tx_hash: TxHash::from([2u8; 32]),
                log_index: 10,
            },
            mongodb::bson::DateTime::now(),
        );

        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-recover-complete-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));

        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 15,
            commitments: vec!["0xddd".to_string()],
            proposer_address: Address::from([11u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");

        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2728,
                tx_hash: TxHash::from([1u8; 32]),
                log_index: 11,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        recover_from_restore_journal(&client)
            .await
            .expect("recover should clean swap_complete state");

        assert_eq!(client.get_restore_journal().await, None);

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_rolls_back_incoherent_swap_complete_state() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 29,
            commitments: Vec::new(),
            proposer_address: Address::from([29u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2900,
                tx_hash: TxHash::from([29u8; 32]),
                log_index: 29,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-recover-swap-complete-rollback-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 30,
            commitments: Vec::new(),
            proposer_address: Address::from([30u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 3000,
                tx_hash: TxHash::from([30u8; 32]),
                log_index: 30,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &newer_live_sync_state).await;
        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        let reserved_deposit = DepositDatawithFee {
            fee: Fr254::from(27u64),
            deposit_data: DepositData {
                nf_token_id: Fr254::from(2901u64),
                nf_slot_id: Fr254::from(2902u64),
                value: Fr254::from(2903u64),
                secret_hash: Fr254::from(2904u64),
            },
            reserved: true,
        };
        <mongodb::Client as TransactionsDB<MockProof>>::set_mempool_deposits(
            &client,
            vec![reserved_deposit],
        )
        .await
        .expect("store reserved deposit outside the snapshot");

        let selected_transaction = test_selected_client_transaction(
            81,
            newer_live_sync_state.last_applied_l2_block,
            Fr254::from(2981u64),
        );
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store lingering selected transaction outside the snapshot");

        client
            .store_pending_block(&PendingBlock {
                layer2_block_number: newer_live_sync_state.last_applied_l2_block + 1,
                state: PendingBlockState::ReadyToPropose,
                broadcast_tx_hash: None,
                broadcast_receipt_checks: 0,
                block: Some(Block::default()),
                selected_deposits: vec![vec![reserved_deposit]],
                selected_client_transaction_hashes: vec![selected_transaction.hash.clone()],
            })
            .await
            .expect("store stale pending block outside the snapshot");

        let mempool_transaction = ClientTransactionWithMetaData {
            lifecycle: TxLifecycle::Mempool,
            ..test_selected_client_transaction(91, 0, Fr254::from(2991u64))
        };
        client
            .store_transaction(mempool_transaction.clone())
            .await
            .expect("store mempool transaction outside the snapshot");

        client
            .delete_block_by_number(snapshot_block.layer2_block_number)
            .await
            .expect("delete restored stored block");

        let error = recover_from_restore_journal(&client)
            .await
            .expect_err("recovery should roll back incoherent swap_complete state");
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));
        assert_eq!(client.get_restore_journal().await, None);
        assert_eq!(
            client.get_sync_state().await,
            Some(newer_live_sync_state.clone())
        );
        assert_eq!(
            client
                .get_all_pending_blocks()
                .await
                .expect("read pending blocks after rollback")
                .len(),
            1,
            "swap_complete rollback should leave non-snapshotted PendingBlocks untouched"
        );
        assert!(matches!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect("selected transaction should still exist after rollback")
            .lifecycle,
            TxLifecycle::Selected { .. }
        ));
        assert!(matches!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &mempool_transaction.hash,
            )
            .await
            .expect("mempool transaction should still exist after rollback")
            .lifecycle,
            TxLifecycle::Mempool
        ));

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

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_keeps_journal_when_swap_complete_rollback_fails() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;
        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 31,
            commitments: Vec::new(),
            proposer_address: Address::from([31u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");
        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 3100,
                tx_hash: TxHash::from([31u8; 32]),
                log_index: 31,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;
        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-recover-swap-complete-rollback-failure-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 32,
            commitments: Vec::new(),
            proposer_address: Address::from([32u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        let _newer_live_sync_state = SyncState::new(
            newer_live_block.layer2_block_number,
            newer_live_block.hash().to_hex_string(),
            L1Ref {
                block_number: 3200,
                tx_hash: TxHash::from([32u8; 32]),
                log_index: 32,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &_newer_live_sync_state).await;
        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        client
            .delete_block_by_number(snapshot_block.layer2_block_number)
            .await
            .expect("delete restored stored block");
        client
            .database(DB)
            .collection::<Document>(&backup_collection_name(PROPOSED_BLOCKS_COLLECTION))
            .drop()
            .await
            .expect("drop required backup collection");

        let error = recover_from_restore_journal(&client).await.expect_err(
            "recovery should fail closed when swap_complete rollback loses a required backup",
        );
        assert!(matches!(error, SnapshotError::RestoreInvariantViolation(_)));

        let persisted_journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain when swap_complete rollback fails");
        assert_eq!(
            persisted_journal.phase,
            RestoreJournalPhase::RollbackInProgress
        );
        assert_eq!(
            persisted_journal.current_step,
            Some(RestoreJournalStep::RollbackStarted)
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_cleans_real_swap_complete_crash() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 25,
            commitments: vec!["0xcleanup".to_string()],
            proposer_address: Address::from([25u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        persist_sync_state(
            &client,
            &SyncState::new(
                snapshot_block.layer2_block_number,
                snapshot_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2500,
                    tx_hash: TxHash::from([25u8; 32]),
                    log_index: 4,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-cleanup-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 26,
            commitments: vec!["0xnewer".to_string()],
            proposer_address: Address::from([26u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        persist_sync_state(
            &client,
            &SyncState::new(
                newer_live_block.layer2_block_number,
                newer_live_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2600,
                    tx_hash: TxHash::from([26u8; 32]),
                    log_index: 5,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        let failpoint = TestFailpointGuard::enable("restore_after_first_backup_cleanup");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { cleanup_after_proposer_shadow_swap(&crash_client).await })
                .await
                .expect_err("cleanup should panic at failpoint");
        assert!(join_error.is_panic());
        drop(failpoint);

        recover_from_restore_journal(&client)
            .await
            .expect("recover should finish swap_complete cleanup");

        assert_eq!(client.get_restore_journal().await, None);
        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("restored live sync_state should still exist");
        assert_eq!(
            live_sync_state.last_applied_l2_block,
            snapshot_block.layer2_block_number
        );

        let names = client
            .database(DB)
            .list_collection_names()
            .await
            .expect("list collection names");
        assert!(!names
            .iter()
            .any(|name| name.starts_with("restore_shadow__")));
        assert!(!names
            .iter()
            .any(|name| name.starts_with("restore_backup__")));

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }

    #[tokio::test]
    async fn recover_from_restore_journal_skips_non_snapshot_cleanup_once_applied() {
        let _snapshot_test_lock = snapshot_test_lock().await;
        let container = get_mongo().await;
        let client = get_db_connection(&container).await;

        initialize_snapshot_test_trees(&client).await;

        let snapshot_block = StoredBlock {
            layer2_block_number: 25,
            commitments: vec!["0xcleanup-selected".to_string()],
            proposer_address: Address::from([31u8; 20]),
        };
        client
            .store_block(&snapshot_block)
            .await
            .expect("store snapshot block");

        let snapshot_sync_state = SyncState::new(
            snapshot_block.layer2_block_number,
            snapshot_block.hash().to_hex_string(),
            L1Ref {
                block_number: 2500,
                tx_hash: TxHash::from([31u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );
        persist_sync_state(&client, &snapshot_sync_state).await;

        let snapshot_root = std::env::temp_dir().join(format!(
            "nf4-proposer-cleanup-applied-crash-test-{}",
            mongodb::bson::DateTime::now().timestamp_millis()
        ));
        let manifest = create_proposer_snapshot(&client, &snapshot_root)
            .await
            .expect("create snapshot");
        let snapshot_dir = snapshot_root.join(&manifest.snapshot_id);

        let newer_live_block = StoredBlock {
            layer2_block_number: 26,
            commitments: vec!["0xnewer".to_string()],
            proposer_address: Address::from([32u8; 20]),
        };
        client
            .store_block(&newer_live_block)
            .await
            .expect("store newer live block");
        persist_sync_state(
            &client,
            &SyncState::new(
                newer_live_block.layer2_block_number,
                newer_live_block.hash().to_hex_string(),
                L1Ref {
                    block_number: 2600,
                    tx_hash: TxHash::from([32u8; 32]),
                    log_index: 5,
                },
                mongodb::bson::DateTime::now(),
            ),
        )
        .await;

        let selected_transaction = test_selected_client_transaction(
            401,
            snapshot_block.layer2_block_number + 1,
            Fr254::from(444_u64),
        );
        client
            .store_transaction(selected_transaction.clone())
            .await
            .expect("store selected transaction ahead of snapshot state");

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        let failpoint = TestFailpointGuard::enable("restore_after_non_snapshot_cleanup_applied");
        let crash_client = client.clone();
        let join_error =
            tokio::spawn(async move { cleanup_after_proposer_shadow_swap(&crash_client).await })
                .await
                .expect_err("cleanup should panic after non-snapshot cleanup commit");
        assert!(join_error.is_panic());
        drop(failpoint);

        let journal = client
            .get_restore_journal()
            .await
            .expect("journal should remain after cleanup-applied crash");
        assert_eq!(journal.phase, RestoreJournalPhase::SwapComplete);
        assert_eq!(
            journal.current_step,
            Some(RestoreJournalStep::NonSnapshotCleanupApplied)
        );
        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect("selected transaction should still exist after cleanup-applied crash")
            .lifecycle
            .is_mempool(),
            "cleanup should have already restored the selected transaction to the mempool before the crash"
        );

        recover_from_restore_journal(&client)
            .await
            .expect("recover should skip already-applied non-snapshot cleanup");

        assert_eq!(client.get_restore_journal().await, None);
        assert!(
            <mongodb::Client as TransactionsDB<MockProof>>::get_transaction(
                &client,
                &selected_transaction.hash,
            )
            .await
            .expect("transaction should still exist after swap_complete recovery")
            .lifecycle
            .is_mempool(),
            "recovery should not delete transactions preserved by the first cleanup pass"
        );

        fs::remove_dir_all(snapshot_root)
            .await
            .expect("cleanup snapshot directory");
    }
}
