# Proposer Snapshot Restore Design

## Goal

Define a robust proposer snapshot restore flow that remains safe even though MongoDB does not support running `renameCollection` inside a multi-document transaction. The restore flow must therefore rely on:

- startup/readiness gating
- shadow collections
- backup collections
- a persistent `restore_journal`
- resumable crash recovery

This note covers restore design only. It does **not** implement restore code.

## Journal Model

`restore_journal` is a single-document Mongo collection.

Recommended document shape:

```json
{
  "_id": "proposer",
  "schema_version": 1,
  "snapshot_id": "proposer-l2-123-1715440000000",
  "snapshot_dir": "/path/to/snapshots/proposer-l2-123-1715440000000",
  "manifest_overall_sha256": "...",
  "phase": "loading_shadow | swap_in_progress | swap_complete",
  "current_index": 0,
  "current_step": "backup_pending | backup_created",
  "collections": [
    {
      "live": "Commitments_nodes",
      "shadow": "restore_shadow__Commitments_nodes",
      "backup": "restore_backup__Commitments_nodes"
    }
  ],
  "started_at": "BSON datetime",
  "updated_at": "BSON datetime"
}
```

`idle` is represented by **absence** of the journal document. That keeps the steady state simple: if no restore is in progress or pending cleanup, there is no journal.

## Naming

Shadow collections:

- `restore_shadow__{live_collection_name}`

Backup collections:

- `restore_backup__{live_collection_name}`

These names are deterministic, easy to inspect manually, and clearly distinct from live collections.

## Phases And Transitions

Allowed phase transitions:

- `idle` -> `loading_shadow`
- `loading_shadow` -> `swap_in_progress`
- `swap_in_progress` -> `swap_complete`
- `swap_complete` -> `idle`

Illegal transitions:

- `swap_in_progress` -> `loading_shadow`
- `swap_complete` -> `loading_shadow`
- direct `loading_shadow` -> `idle` after live swap has begun

Phase meaning:

- `loading_shadow`: snapshot files are being validated and imported into shadow collections. Live collections remain untouched.
- `swap_in_progress`: live collections are being replaced one by one using `live -> backup`, then `shadow -> live`.
- `swap_complete`: all live collections now point to restored data; only cleanup remains (`backup` and leftover `shadow` removal, then journal removal).

## Exact Operation Order

### 1. Enter restore mode

- proposer remains **not READY**
- create journal with `phase=loading_shadow`
- drop any stale `restore_shadow__*` and `restore_backup__*` collections from a previous interrupted attempt
- load `manifest.json`
- validate `schema_version`, `storage_format`, per-collection checksums, and `overall_sha256`

### 2. Load shadow collections

- import each snapshot file into its matching `restore_shadow__*` collection
- verify imported document counts match the manifest
- keep live collections and live `sync_state` untouched during this phase
- when all shadow collections are loaded, update journal to `phase=swap_in_progress`, `current_index=0`, `current_step=backup_pending`

### 3. Swap live collections

Collection order must be deterministic. Recommended order:

- all tree and block collections first
- `sync_state` **last**

For each collection entry at index `i`:

1. ensure `restore_backup__*` does not already exist for this collection
2. rename `live -> backup`
3. update journal: `current_index=i`, `current_step=backup_created`
4. rename `shadow -> live`
5. update journal: advance to the next collection with `current_step=backup_pending`

`sync_state` is swapped last so it never advertises the restored snapshot before the other restored collections are already live.

### 4. Cleanup after swap

- update journal to `phase=swap_complete`
- drop all `restore_backup__*` collections
- drop any leftover `restore_shadow__*` collections
- delete the `restore_journal` document, returning to `idle`

## Crash Recovery On Startup

### `idle` (journal absent)

- normal startup path
- validate live `sync_state` as usual

### `loading_shadow`

Live state was never modified. Recovery should:

- drop all `restore_shadow__*`
- drop all `restore_backup__*`
- delete the journal
- restart restore from scratch if restore is still desired

This phase is restartable and should not try to resume partially imported shadow collections.

### `swap_in_progress`

Default rule: **resume forward**, not rollback.

Reason: backups and shadows are specifically kept so the swap can continue safely while the proposer remains gated and non-READY.

Recovery logic:

- read `current_index` and `current_step`
- for already completed indices `< current_index`, keep their existing `backup` collections
- for the current index:
  - if `current_step=backup_pending`, no swap started for that collection yet; continue with `live -> backup`
  - if `current_step=backup_created`, `live -> backup` already happened; continue with `shadow -> live`

Automatic rollback is only used if startup finds broken invariants, for example:

- the journal says `backup_created` but the backup collection is missing
- the shadow collection needed for resume is missing
- namespace layout no longer matches the journal

In that case, rollback should:

- restore previously completed collections in reverse order using `backup -> live`
- restore the current collection from backup if needed
- drop remaining shadow collections
- keep the proposer non-READY and fail closed if rollback cannot be completed cleanly

Implementation note and deferred follow-up:

- when recovery resumes a `rollback_in_progress` journal at an **optional** collection where the journal step is `rollback_started`, `backup` is absent, and `live` is present, recovery now drops `live` only if it can still verify that `live` matches the restored snapshot state; otherwise it fails closed and requires manual intervention
- when recovery resumes a `rollback_in_progress` journal at a **required** collection where the journal step is `rollback_started`, `backup` is absent, and `live` is present, the current implementation still treats this as the backup having already been renamed back to live
- that assumption is correct for the legitimate crash window between `rename_collection(backup, live)` and the `rollback_applied` journal write
- it is **not** correct if a required backup collection was lost externally (for example storage corruption or manual deletion) while the journal was at `rollback_started`
- in that case recovery may incorrectly mark the collection as rolled back and may boot from a mixed old/new state
- if external backup loss is suspected, manual intervention is required before restart
- a robust fix would persist a per-collection fingerprint in the journal at the moment `live -> backup` succeeds, then verify `live` against that fingerprint when resuming rollback; this is deferred as a follow-up

### `swap_complete`

Live collections already point to restored data. Recovery should:

- validate live `sync_state` against the restored `StoredBlock`
- if valid, finish cleanup of `restore_backup__*` and `restore_shadow__*`, then delete the journal
- if invalid and backups still exist, attempt rollback from backups
- if invalid and backups are gone, fail closed and require manual intervention

## Rollback vs Resume Rules

During `swap_in_progress`:

- **resume** is the default startup behavior
- **rollback** is a fallback only when journal/namespace invariants are broken badly enough that resume is unsafe

This keeps the happy path simple while still preserving a safety net for partial namespace damage.

## `sync_state` Handling

- live `sync_state` is **not** deleted at restore start
- snapshot `sync_state` is imported into `restore_shadow__sync_state`
- live `sync_state` is swapped only as the **final** collection in `swap_in_progress`
- after restore cleanup, the live `sync_state` must be the one from the snapshot

This ensures `sync_state` only points to the restored snapshot after the corresponding restored collections are already live.

## When The Proposer Becomes READY Again

The proposer must stay **not READY** for the entire restore flow, including journal cleanup.

The proposer can become READY again only after all of the following are true:

- `restore_journal` is back to `idle` (document absent)
- live collections have passed restore validation
- live `sync_state` validates against the restored `StoredBlock`
- post-restore replay from `sync_state.l1_ref.block_number` has completed and the proposer is back at the current L1/L2 tip

Restore completion alone is not enough to become READY; replay catch-up must also succeed.
