use super::models::DeEscrowDataReq;
use crate::{
    domain::entities::WithdrawData as NFWithdrawData,
    driven::contract_functions::nightfall_contract::Nightfall, ports::contracts::NightfallContract,
};
use log::{debug, error};
use reqwest::StatusCode;
use warp::{path, reject, Filter, Reply};

/// GET request for a specific commitment by key
pub fn de_escrow() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "de-escrow")
        .and(warp::body::json())
        .and_then(handle_de_escrow)
}

pub async fn handle_de_escrow(data: DeEscrowDataReq) -> Result<impl Reply, warp::Rejection> {
    let token_type = u8::from_str_radix(&data.token_type, 16)
        .map_err(|_| {
            error!("Could not convert token type");
            reject::custom(crate::domain::error::ClientRejection::FailedDeEscrow)
        })?
        .into();
    let withdraw_data: NFWithdrawData = NFWithdrawData::try_from(data.clone()).map_err(|e| {
        error!("Could not convert Withdraw data request to WithdrawData: {e}");
        reject::custom(crate::domain::error::ClientRejection::FailedDeEscrow)
    })?;
    let available = Nightfall::NightfallCalls::withdraw_available(withdraw_data).await;
    match available {
        Ok(b) => {
            if b {
                debug!("Withdraw is on chain, attempting to de-escrow funds");
                Nightfall::NightfallCalls::de_escrow_funds(withdraw_data, token_type)
                    .await
                    .map_err(|e| {
                        error!("Could not de-escrow funds: {e}");
                        reject::custom(crate::domain::error::ClientRejection::FailedDeEscrow)
                    })?;

                Ok(warp::reply::with_status("OK", StatusCode::OK))
            } else {
                debug!("Not yet able to de-escrow funds");
                Err(reject::custom(
                    crate::domain::error::ClientRejection::FailedDeEscrow,
                ))
            }
        }
        Err(e) => {
            debug!("Nightfall contract error: {e}");
            Err(reject::custom(
                crate::domain::error::ClientRejection::FailedDeEscrow,
            ))
        }
    }
}
