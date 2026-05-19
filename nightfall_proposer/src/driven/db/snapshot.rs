use crate::{
    domain::entities::{
        ProposerSnapshotManifest, RestoreJournal, RestoreJournalCollection, RestoreJournalPhase,
        RestoreJournalStep, SnapshotCollectionManifest, SyncState,
    },
    driven::db::mongo_db::{DB, PROPOSED_BLOCKS_COLLECTION, SYNC_STATE_COLLECTION},
    ports::db::{RestoreJournalDB, SyncStateDB},
    ports::trees::{CommitmentTree, HistoricRootTree, NullifierTree},
};
use ark_bn254::Fr as Fr254;
use log::warn;
use mongodb::bson::{Bson, Document};
use mongodb::options::ReadConcern;
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    process,
};
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

const TEMP_SNAPSHOT_DIR_PREFIX: &str = ".tmp-proposer-snapshot-";

#[cfg(test)]
use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

#[cfg(test)]
static ENABLED_FAILPOINTS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

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
    MissingRestoreJournal,
    UnsupportedManifestSchemaVersion(u32),
    UnsupportedManifestStorageFormat(String),
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
    names.push(PROPOSED_BLOCKS_COLLECTION.to_string());
    names.push(SYNC_STATE_COLLECTION.to_string());
    names
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
        RestoreJournalPhase::SwapComplete => "swap_complete",
    }
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
    let database = client.database(DB);
    // listCollections is not supported inside multi-document transactions.
    // We therefore discover the stable proposer collection set before opening
    // the snapshot read transaction. This is acceptable because these
    // collection names are created during proposer initialization and are not
    // expected to change while a snapshot is taken.
    let existing_collections = database.list_collection_names().await?;
    let snapshot_root_dir = snapshot_root_dir.to_path_buf();
    fs::create_dir_all(&snapshot_root_dir).await?;
    cleanup_orphaned_proposer_snapshot_temp_dirs(&snapshot_root_dir).await?;
    let mut session = client.start_session().await?;
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
                    warn!(
                        "Skipping proposer snapshot collection {collection_name}: collection not found in database {DB}"
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

    Ok(manifest)
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
        let manifest = match load_snapshot_manifest(&snapshot_dir).await {
            Ok(manifest) => manifest,
            Err(error) => {
                warn!(
                    "Skipping proposer snapshot at {}: could not load manifest: {error}",
                    snapshot_dir.display()
                );
                continue;
            }
        };

        if manifest.last_applied_l2_block > max_last_applied_l2_block {
            continue;
        }

        if let Err(error) = validate_snapshot_files(&snapshot_dir, &manifest).await {
            warn!(
                "Skipping proposer snapshot at {}: validation failed: {error}",
                snapshot_dir.display()
            );
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

async fn import_collection_into_shadow(
    database: &mongodb::Database,
    snapshot_dir: &Path,
    manifest: &SnapshotCollectionManifest,
    shadow_collection_name: &str,
) -> Result<(), SnapshotError> {
    let file = File::open(snapshot_dir.join(&manifest.file_name)).await?;
    let mut reader = BufReader::new(file).lines();
    let shadow_collection = database.collection::<Document>(shadow_collection_name);

    while let Some(line) = reader.next_line().await? {
        let document: Document = serde_json::from_str(&line)?;
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

async fn rollback_shadow_swap(
    client: &mongodb::Client,
    journal: &RestoreJournal,
) -> Result<(), SnapshotError> {
    let database = client.database(DB);
    let ordered = restore_collections_in_swap_order(&journal.collections);
    let current_index = journal.current_index.unwrap_or(0) as usize;

    let mut rollback_indices: Vec<usize> = if matches!(
        journal.current_step,
        Some(RestoreJournalStep::BackupCreated)
    ) {
        (0..=current_index).collect()
    } else {
        (0..current_index).collect()
    };
    rollback_indices.reverse();

    for index in rollback_indices {
        let collection = &ordered[index];
        if !collection_exists(&database, &collection.backup).await? {
            return Err(SnapshotError::RestoreInvariantViolation(format!(
                "expected backup collection {} during rollback",
                collection.backup
            )));
        }

        rename_collection(client, &collection.backup, &collection.live, true).await?;
    }

    for collection in &ordered {
        drop_collection_if_exists(&database, &collection.shadow).await?;
        drop_collection_if_exists(&database, &collection.backup).await?;
    }

    client.delete_restore_journal().await?;
    Ok(())
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
            "current_index {} is out of bounds for {} collections",
            start_index,
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
                if collection_exists(&database, &collection.backup).await? {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "backup collection {} already exists before swap",
                        collection.backup
                    )));
                }

                if !collection_exists(&database, &collection.live).await? {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "live collection {} is missing before swap",
                        collection.live
                    )));
                }

                if !collection_exists(&database, &collection.shadow).await? {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "shadow collection {} is missing before swap",
                        collection.shadow
                    )));
                }

                rename_collection(client, &collection.live, &collection.backup, false).await?;

                journal.current_index = Some(index as u32);
                journal.current_step = Some(RestoreJournalStep::BackupCreated);
                journal.updated_at = mongodb::bson::DateTime::now();
                client.upsert_restore_journal(journal).await?;
                maybe_crash_at_failpoint("restore_after_backup_created");

                rename_collection(client, &collection.shadow, &collection.live, false).await?;
            }
            RestoreJournalStep::BackupCreated => {
                if !collection_exists(&database, &collection.backup).await? {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "backup collection {} is missing while resume expects BackupCreated",
                        collection.backup
                    )));
                }

                if !collection_exists(&database, &collection.shadow).await? {
                    return Err(SnapshotError::RestoreInvariantViolation(format!(
                        "shadow collection {} is missing while resume expects BackupCreated",
                        collection.shadow
                    )));
                }

                rename_collection(client, &collection.shadow, &collection.live, false).await?;
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
    journal.current_step = None;
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
    let manifest = load_snapshot_manifest(&snapshot_dir).await?;
    validate_snapshot_files(&snapshot_dir, &manifest).await?;

    let database = client.database(DB);
    let existing_collections = database.list_collection_names().await?;
    let restore_collections: Vec<RestoreJournalCollection> = manifest
        .collections
        .iter()
        .map(|collection| RestoreJournalCollection {
            live: collection.collection_name.clone(),
            shadow: shadow_collection_name(&collection.collection_name),
            backup: backup_collection_name(&collection.collection_name),
        })
        .collect();

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

    for (manifest_collection, restore_collection) in
        manifest.collections.iter().zip(restore_collections.iter())
    {
        import_collection_into_shadow(
            &database,
            &snapshot_dir,
            manifest_collection,
            &restore_collection.shadow,
        )
        .await?;
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
            rollback_shadow_swap(client, &journal).await?;
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

pub async fn recover_from_restore_journal(client: &mongodb::Client) -> Result<(), SnapshotError> {
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
                        rollback_shadow_swap(client, &journal).await?;
                    }
                    Err(error)
                }
            }
        }
        RestoreJournalPhase::SwapComplete => cleanup_after_proposer_shadow_swap(client).await,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        domain::entities::{
            L1Ref, RestoreJournal, RestoreJournalCollection, RestoreJournalPhase,
            RestoreJournalStep, SyncState,
        },
        driven::db::mongo_db::{StoredBlock, RESTORE_JOURNAL_COLLECTION},
        ports::{
            db::{BlockStorageDB, RestoreJournalDB, SyncStateDB},
            trees::{CommitmentTree, HistoricRootTree, NullifierTree},
        },
    };
    use alloy::primitives::{Address, TxHash};
    use ark_ff::Zero;
    use lib::tests_utils::{get_db_connection, get_mongo};
    use tokio::sync::{Mutex, OnceCell};

    async fn snapshot_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
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
                stored_block.hash().to_string(),
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
            stored_block.hash().to_string(),
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
                first_block.hash().to_string(),
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
                second_block.hash().to_string(),
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
            stored_block.hash().to_string(),
            L1Ref {
                block_number: 5678,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );

        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write sync_state");

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
                "shadow collection {} should contain the manifest document count",
                shadow_name
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
                stored_block.hash().to_string(),
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
            snapshot_block.hash().to_string(),
            L1Ref {
                block_number: 5678,
                tx_hash: TxHash::from([8u8; 32]),
                log_index: 4,
            },
            mongodb::bson::DateTime::now(),
        );

        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&snapshot_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write snapshot sync_state");

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
            new_live_block.hash().to_string(),
            L1Ref {
                block_number: 91011,
                tx_hash: TxHash::from([7u8; 32]),
                log_index: 5,
            },
            mongodb::bson::DateTime::now(),
        );
        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&new_live_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write newer live sync_state");

        load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let journal = swap_proposer_shadow_into_live(&client)
            .await
            .expect("swap shadow into live");

        assert_eq!(journal.phase, RestoreJournalPhase::SwapComplete);
        assert_eq!(journal.current_index, None);
        assert_eq!(journal.current_step, None);

        let live_sync_state = client
            .get_sync_state()
            .await
            .expect("live sync_state should exist");
        assert_eq!(live_sync_state.last_applied_l2_block, 8);
        assert_eq!(
            live_sync_state.fingerprint,
            snapshot_block.hash().to_string()
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
                snapshot_block.hash().to_string(),
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
                newer_live_block.hash().to_string(),
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
            snapshot_block.hash().to_string(),
            L1Ref {
                block_number: 1112,
                tx_hash: TxHash::from([6u8; 32]),
                log_index: 6,
            },
            mongodb::bson::DateTime::now(),
        );

        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&snapshot_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write snapshot sync_state");

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
            newer_live_block.hash().to_string(),
            L1Ref {
                block_number: 1314,
                tx_hash: TxHash::from([5u8; 32]),
                log_index: 7,
            },
            mongodb::bson::DateTime::now(),
        );
        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&newer_live_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write newer live sync_state");

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
            snapshot_block.hash().to_string()
        );

        let live_blocks_count = client
            .database(DB)
            .collection::<Document>(PROPOSED_BLOCKS_COLLECTION)
            .count_documents(mongodb::bson::doc! {})
            .await
            .expect("count live proposed blocks");
        assert_eq!(live_blocks_count, 1);

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
            snapshot_block.hash().to_string(),
            L1Ref {
                block_number: 2122,
                tx_hash: TxHash::from([4u8; 32]),
                log_index: 8,
            },
            mongodb::bson::DateTime::now(),
        );

        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&snapshot_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write snapshot sync_state");

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
            newer_live_block.hash().to_string(),
            L1Ref {
                block_number: 2324,
                tx_hash: TxHash::from([3u8; 32]),
                log_index: 9,
            },
            mongodb::bson::DateTime::now(),
        );
        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&newer_live_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write newer live sync_state");

        let mut journal = load_proposer_snapshot_into_shadow(&client, &snapshot_dir)
            .await
            .expect("load snapshot into shadow");
        let ordered = restore_collections_in_swap_order(&journal.collections);
        let first = &ordered[0];

        rename_collection(&client, &first.live, &first.backup, false)
            .await
            .expect("simulate backup rename");

        journal.collections = ordered;
        journal.phase = RestoreJournalPhase::SwapInProgress;
        journal.current_index = Some(0);
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
            snapshot_block.hash().to_string(),
            L1Ref {
                block_number: 2526,
                tx_hash: TxHash::from([2u8; 32]),
                log_index: 10,
            },
            mongodb::bson::DateTime::now(),
        );

        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&snapshot_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write snapshot sync_state");

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
            newer_live_block.hash().to_string(),
            L1Ref {
                block_number: 2728,
                tx_hash: TxHash::from([1u8; 32]),
                log_index: 11,
            },
            mongodb::bson::DateTime::now(),
        );
        let mut session = client.start_session().await.expect("start session");
        session
            .start_transaction()
            .and_run2(async |session| {
                client
                    .update_sync_state_with_session(&newer_live_sync_state, session)
                    .await?;
                Ok::<(), mongodb::error::Error>(())
            })
            .await
            .expect("write newer live sync_state");

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
                snapshot_block.hash().to_string(),
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
                newer_live_block.hash().to_string(),
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
}
