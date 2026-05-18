# Changelog

All notable changes to this repository will be documented in this file.

## [Unreleased]

### Added
- Atomic swap support across client/prover/proposer flow, including `swap_link`, `deadline`, and `swap_side`.
- Advisory swap cancel request flow `POST /v1/swap/cancel-request`, which asks proposers to best-effort remove matching swap legs from proposer mempools without unlocking client commitments.
- Trustless expired-swap settlement API `POST /v1/swap/settle-expired`, which unlocks locally reserved swap commitments (`PendingSpend` -> `Unspent`) only after local safety checks succeed.

### Breaking Changes
- `ClientTransaction` serialization now includes swap metadata fields (`swap_link`, `deadline`, `swap_side`).
- Public input serialization/hash format changed to include swap outputs (`swap_link`, `deadline`, `swap_side`).
- Public input schema tag bumped from `public_inputsversion1` to `public_inputsversion2`.
- Legacy proofs generated before this change are not compatible with the new public-input format.
- Legacy serialized records/payloads that do not include the new swap fields may fail to deserialize unless defaults/migration are applied.

### Migration
1. Roll out client and proposer from the same compatible version.
2. Drain/clear old queued or mempool transactions created before this upgrade.
3. Regenerate/re-submit transactions so proofs are produced with the new public-input format.
