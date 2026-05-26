# Proposer Snapshot Restore Design

## Goal

Define a robust proposer snapshot restore flow that remains safe even though MongoDB does not support running `renameCollection` inside a multi-document transaction. The restore flow must therefore rely on:

- startup/readiness gating
- shadow collections
- backup collections
- a persistent `restore_journal`
- resumable crash recovery

This note covers restore design only. It does **not** implement restore code.

## Design Invariants

The restore design relies on the following invariants.

1. Canonical snapshot boundary

`ClientTransactions` is outside the canonical proposer snapshot boundary.

Canonical restore correctness must depend only on the snapshotted proposer state:

- proposer trees
- `StoredBlock`
- `sync_state`
- other explicitly snapshotted canonical collections

`ClientTransactions` must therefore be treated as live auxiliary state during restore.

2. Crash-idempotent non-snapshot cleanup

The non-snapshot cleanup performed after `swap_complete` must be crash-idempotent.

A crash after restored live collections are already in place, but before `restore_journal` is
deleted, must not allow a second cleanup pass to delete or corrupt transactions that were
intentionally preserved by the first pass.

3. Journaled cleanup progress

If non-snapshot cleanup is not intrinsically safe to run multiple times, its progress must be
recorded explicitly in `restore_journal`.

Recovery must be able to distinguish unambiguously between:

- cleanup not yet committed, in which case it must be rerun
- cleanup already committed, in which case it must be skipped on resume

The safest design is to commit cleanup progress in the same Mongo transaction as the non-snapshot
cleanup writes.

4. Post-restore lifecycle reconciliation

Because `ClientTransactions` is outside the canonical snapshot boundary, active client transaction
lifecycle must be reconciled explicitly against canonical proposer state after restore and replay.

In particular:

- `Selected { block_l2 }` entries above restored `sync_state.last_applied_l2_block` must not
  remain trusted after restore
- `Included { block_l2 }` entries above restored `sync_state.last_applied_l2_block` must not
  remain trusted after restore without explicit reconciliation against canonical `StoredBlock`
  contents

Restore may normalize obviously non-canonical lifecycle state, but final lifecycle correctness for
active transactions must be derived from canonical replayed state rather than assumed from
pre-restore live documents.

5. Destructive replay only after restore-journal state is cleared

Destructive replay fallback is only allowed after restore-journal recovery has either cleared the
`restore_journal` and related `restore_backup__*` / `restore_shadow__*` state, or confirmed that no
such state exists.

If restore-journal recovery fails and journal or backup/shadow state may still be present, recovery
must fail closed for manual intervention instead of falling through to destructive reset.

6. Startup cleanup must preserve tree/state coherence

Startup cleanup must leave `sync_state`, `StoredBlock`, `PendingBlock`, and proposer tree state
mutually coherent.

If startup cleanup removes speculative `StoredBlock` or `PendingBlock` state beyond `sync_state`, it
must also roll back or reset the corresponding Merkle tree state to a value coherent with
`sync_state` before startup can continue.

Deleting speculative block state without rolling back trees can leave proposer roots ahead of
`sync_state`.

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
  "current_step": "backup_pending | backup_created | non_snapshot_cleanup_pending | non_snapshot_cleanup_applied | rollback_pending | rollback_started | rollback_applied",
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

### 4a. Non-snapshot cleanup invariants

`ClientTransactions` is intentionally kept outside the canonical proposer snapshot boundary.

That means restore correctness does not depend on snapshotting client transaction lifecycle, but it
also means any live-only recovery state must be normalized explicitly after the canonical snapshot
has been swapped into live collections.

The non-snapshot cleanup phase includes at least:

- deleting persisted `PendingBlock` state
- clearing reserved deposit selections
- deleting ordinary mempool client transactions
- normalizing active client transaction lifecycle that is ahead of the restored `sync_state`

In particular:

- `Selected { block_l2 }` entries with `block_l2 > restored_sync_state.last_applied_l2_block`
  must not remain selected after restore
- `Included { block_l2 }` entries with `block_l2 > restored_sync_state.last_applied_l2_block`
  must not be trusted as canonical after restore without explicit reconciliation

This cleanup must be crash-idempotent.

A crash after live collections have been restored but before `restore_journal` is deleted must not
allow a second cleanup pass to delete transactions that were intentionally preserved by the first
pass.

Therefore, non-snapshot cleanup must either:

- be safe to execute multiple times without changing the final result, or
- persist explicit cleanup progress in `restore_journal` and skip already-committed cleanup work on
  resume

The safest design is to treat non-snapshot cleanup as a journaled step of `swap_complete`, and to
update that journal step in the same Mongo transaction as the non-snapshot cleanup writes.

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
- complete any journaled non-snapshot cleanup that was not durably committed before the crash
- skip non-snapshot cleanup if the journal shows that cleanup was already committed
- only after cleanup and validation succeed, finish removal of `restore_backup__*` and `restore_shadow__*`, then delete the journal
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

## Deferred Follow-Up: Client Transaction Lifecycle Reconciliation

Because `ClientTransactions` is outside the canonical snapshot boundary, restore cannot assume that
live `Selected` or `Included` lifecycle entries above the restored `sync_state` remain correct.

A later replay may reconstruct the same canonical inclusion, or it may replay a different canonical
block sequence after desync recovery, block-hash mismatch, or similar divergence handling.

As a result, lifecycle state for active client transactions should be reconciled explicitly after
restore and replay against canonical `StoredBlock` contents.

A robust reconciliation rule is:

- if the canonical `StoredBlock(block_l2)` contains the transaction commitments, classify the
  transaction as `Included { block_l2 }`
- if `block_l2 <= sync_state.last_applied_l2_block` but the canonical block does not contain the
  transaction, classify it as `Mempool`
- otherwise classify it as `Selected { block_l2 }`

This keeps `ClientTransactions` outside the snapshot while still restoring lifecycle consistency
against canonical proposer state.

This reconciliation should run at the end of listener historical replay catch-up, in the same
finalization path that currently reconciles orphaned selected transactions, before
`apply_listener_caught_up_runtime_state()` marks the proposer synchronised again.

## Deferred Follow-Up: Tree State Upper Bounds Beyond HistoricRoot

Current startup and post-restore validation enforce an exact `HistoricRootTree` progression against
`sync_state.last_applied_l2_block`.

A symmetric "tree is ahead of sync_state" upper-bound check is not yet implemented for
`CommitmentTree` and `NullifierTree`.

- for `CommitmentTree`, an exact bound may be derivable from canonical stored block contents, but
  this is not yet enforced by the current implementation
- for `NullifierTree`, the current persisted canonical state does not include enough information to
  derive a robust exact upper bound during startup or restore validation

As a result, current validation detects empty or missing tree state and `HistoricRootTree`
incoherence, but it does not yet reject every case where commitment/nullifier tree state has
advanced beyond the persisted `sync_state`.

If either tree has advanced beyond `sync_state`, replay may try to re-apply state that is already
materialized locally. For commitments, that can shift local tree progression away from the
canonical replay path and produce divergent roots. For nullifiers, replay may attempt to reinsert
already-present leaves, causing replay failures or inconsistent local state. Any later proof built
against those wrong roots would then be invalid.

A robust follow-up should either:

- persist sufficient canonical per-block nullifier accounting to derive an exact expected bound, or
- persist explicit canonical tree progress counters alongside `sync_state` for startup and restore
  validation
