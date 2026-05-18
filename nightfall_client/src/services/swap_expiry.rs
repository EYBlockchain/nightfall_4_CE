use crate::{
    domain::entities::{CommitmentStatus, Request, RequestStatus},
    driven::db::mongo::CommitmentEntry,
    ports::db::{CommitmentDB, RequestDB},
};
use alloy::primitives::{TxHash, I256};
use ark_bn254::Fr as Fr254;
use async_trait::async_trait;
use lib::hex_conversion::HexConvertible;
use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SwapChildRequestArgs {
    #[serde(default)]
    pub deadline: Option<String>,
    #[serde(default)]
    pub swap_link: Option<String>,
    #[serde(default)]
    pub spend_commitment_ids: Vec<String>,
    #[serde(default)]
    pub cancel_requested_at: Option<String>,
    #[serde(default)]
    pub cancel_request_status: Option<String>,
    #[serde(default)]
    pub cancel_request_message: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SwapExpiryReconciliation {
    pub unlocked: usize,
    pub already_unlocked: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SwapExpiryError {
    IncompatibleStatus(RequestStatus),
    MissingChildArgs,
    InvalidChildArgs,
    InvalidCommitmentStates { skipped: usize },
    NoUnlockableCommitments { skipped: usize },
    DatabaseError,
}

#[async_trait]
pub(crate) trait SwapExpiryStore {
    async fn get_request(&self, request_id: &str) -> Option<Request>;
    async fn get_commitment(&self, commitment_id: &Fr254) -> Option<CommitmentEntry>;
    async fn set_request_status(&self, request_id: &str, status: RequestStatus) -> Option<()>;
    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()>;
    async fn mark_commitments_unspent(
        &self,
        commitments: &[Fr254],
        layer_1_transaction_hash: Option<TxHash>,
        layer_2_block_number: Option<i64>,
    ) -> Option<()>;
}

#[async_trait]
impl SwapExpiryStore for mongodb::Client {
    async fn get_request(&self, request_id: &str) -> Option<Request> {
        RequestDB::get_request(self, request_id).await
    }

    async fn get_commitment(&self, commitment_id: &Fr254) -> Option<CommitmentEntry> {
        CommitmentDB::get_commitment(self, commitment_id).await
    }

    async fn set_request_status(&self, request_id: &str, status: RequestStatus) -> Option<()> {
        RequestDB::update_request(self, request_id, status).await
    }

    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()> {
        RequestDB::update_request_child_args(self, request_id, child_args).await
    }

    async fn mark_commitments_unspent(
        &self,
        commitments: &[Fr254],
        layer_1_transaction_hash: Option<TxHash>,
        layer_2_block_number: Option<i64>,
    ) -> Option<()> {
        CommitmentDB::mark_commitments_unspent(
            self,
            commitments,
            layer_1_transaction_hash,
            layer_2_block_number,
        )
        .await
    }
}

#[async_trait]
impl SwapExpiryStore for &mongodb::Client {
    async fn get_request(&self, request_id: &str) -> Option<Request> {
        RequestDB::get_request(*self, request_id).await
    }

    async fn get_commitment(&self, commitment_id: &Fr254) -> Option<CommitmentEntry> {
        CommitmentDB::get_commitment(*self, commitment_id).await
    }

    async fn set_request_status(&self, request_id: &str, status: RequestStatus) -> Option<()> {
        RequestDB::update_request(*self, request_id, status).await
    }

    async fn update_request_child_args(&self, request_id: &str, child_args: &str) -> Option<()> {
        RequestDB::update_request_child_args(*self, request_id, child_args).await
    }

    async fn mark_commitments_unspent(
        &self,
        commitments: &[Fr254],
        layer_1_transaction_hash: Option<TxHash>,
        layer_2_block_number: Option<i64>,
    ) -> Option<()> {
        CommitmentDB::mark_commitments_unspent(
            *self,
            commitments,
            layer_1_transaction_hash,
            layer_2_block_number,
        )
        .await
    }
}

pub(crate) fn extract_swap_deadline(request: &Request) -> Option<Fr254> {
    let child_args = request.child_request_args.as_ref()?;
    let swap_args = serde_json::from_str::<SwapChildRequestArgs>(child_args).ok()?;
    let deadline_hex = swap_args.deadline?;
    Fr254::from_hex_string(&deadline_hex).ok()
}

pub(crate) fn swap_deadline_has_passed(request: &Request, current_l2_block: I256) -> bool {
    if current_l2_block < I256::ZERO {
        return false;
    }

    let Some(deadline) = extract_swap_deadline(request) else {
        return false;
    };

    deadline < Fr254::from(current_l2_block.as_u64())
}

pub(crate) fn should_expire_request(request: &Request, current_l2_block: I256) -> bool {
    matches!(
        request.status,
        RequestStatus::Submitted | RequestStatus::Failed | RequestStatus::ProposerUnreachable
    ) && swap_deadline_has_passed(request, current_l2_block)
}

pub(crate) async fn reconcile_expired_swap_request(
    db: &impl SwapExpiryStore,
    request: &Request,
) -> Result<SwapExpiryReconciliation, SwapExpiryError> {
    if !matches!(
        request.status,
        RequestStatus::Submitted
            | RequestStatus::Failed
            | RequestStatus::ProposerUnreachable
            | RequestStatus::Expired
    ) {
        return Err(SwapExpiryError::IncompatibleStatus(request.status));
    }

    let Some(child_args_json) = request.child_request_args.as_ref() else {
        if matches!(request.status, RequestStatus::Expired) {
            return Ok(SwapExpiryReconciliation {
                unlocked: 0,
                already_unlocked: 0,
            });
        }
        return Err(SwapExpiryError::MissingChildArgs);
    };

    let child_args: SwapChildRequestArgs =
        serde_json::from_str(child_args_json).map_err(|_| SwapExpiryError::InvalidChildArgs)?;

    if child_args.spend_commitment_ids.is_empty()
        && matches!(request.status, RequestStatus::Expired)
    {
        return Ok(SwapExpiryReconciliation {
            unlocked: 0,
            already_unlocked: 0,
        });
    }

    let mut pending_unlock_entries = Vec::new();
    let mut already_unlocked = 0usize;
    let mut skipped = 0usize;

    for commitment_hex in &child_args.spend_commitment_ids {
        let commitment_id = match Fr254::from_hex_string(commitment_hex) {
            Ok(id) => id,
            Err(e) => {
                warn!(
                    "{} Skipping invalid swap commitment id {commitment_hex}: {e}",
                    request.uuid
                );
                skipped += 1;
                continue;
            }
        };

        let Some(existing) = db.get_commitment(&commitment_id).await else {
            warn!(
                "{} Skipping missing swap commitment {}",
                request.uuid,
                commitment_id.to_hex_string()
            );
            skipped += 1;
            continue;
        };

        match existing.status {
            CommitmentStatus::PendingSpend => pending_unlock_entries.push(existing),
            CommitmentStatus::Unspent => already_unlocked += 1,
            _ => {
                warn!(
                    "{} Refusing to unlock commitment {} with status {:?}",
                    request.uuid,
                    commitment_id.to_hex_string(),
                    existing.status
                );
                skipped += 1;
            }
        }
    }

    if pending_unlock_entries.is_empty() && already_unlocked == 0 {
        return Err(SwapExpiryError::NoUnlockableCommitments { skipped });
    }

    if skipped > 0 {
        return Err(SwapExpiryError::InvalidCommitmentStates { skipped });
    }

    if db
        .set_request_status(&request.uuid, RequestStatus::Expired)
        .await
        .is_none()
    {
        error!("{} Failed to persist Expired status", request.uuid);
        return Err(SwapExpiryError::DatabaseError);
    }

    let unlocked = pending_unlock_entries.len();
    for (layer_1_transaction_hash, layer_2_block_number, commitment_ids) in
        group_commitments_by_origin(pending_unlock_entries)
    {
        if db
            .mark_commitments_unspent(
                &commitment_ids,
                layer_1_transaction_hash,
                layer_2_block_number,
            )
            .await
            .is_none()
        {
            error!(
                "{} Failed to unlock swap commitments {:?}",
                request.uuid,
                commitment_ids
                    .iter()
                    .map(|id| id.to_hex_string())
                    .collect::<Vec<_>>()
            );
            return Err(SwapExpiryError::DatabaseError);
        }
    }

    let mut settled_child_args = child_args;
    settled_child_args.spend_commitment_ids.clear();
    let settled_child_args_json =
        serde_json::to_string(&settled_child_args).map_err(|_| SwapExpiryError::DatabaseError)?;

    if db
        .update_request_child_args(&request.uuid, &settled_child_args_json)
        .await
        .is_none()
    {
        error!(
            "{} Failed to persist settled child_request_args",
            request.uuid
        );
        return Err(SwapExpiryError::DatabaseError);
    }

    Ok(SwapExpiryReconciliation {
        unlocked,
        already_unlocked,
    })
}

fn group_commitments_by_origin(
    commitments: Vec<CommitmentEntry>,
) -> Vec<(Option<TxHash>, Option<i64>, Vec<Fr254>)> {
    let mut groups: Vec<(Option<TxHash>, Option<i64>, Vec<Fr254>)> = Vec::new();

    for commitment in commitments {
        if let Some((_, _, commitment_ids)) = groups.iter_mut().find(|(l1_hash, l2_block, _)| {
            *l1_hash == commitment.layer_1_transaction_hash
                && *l2_block == commitment.layer_2_block_number
        }) {
            commitment_ids.push(commitment.key);
        } else {
            groups.push((
                commitment.layer_1_transaction_hash,
                commitment.layer_2_block_number,
                vec![commitment.key],
            ));
        }
    }

    groups
}
