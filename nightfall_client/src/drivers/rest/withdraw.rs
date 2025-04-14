use super::models::WithdrawDataReq;
use crate::{
    domain::entities::{TokenType, WithdrawData},
    ports::contracts::NightfallContract,
};
use lib::wallets::LocalWsClient;
use log::{error, info};
use nightfall_bindings::nightfall::Nightfall;
use reqwest::StatusCode;
use std::{error::Error, fmt::Debug};
use warp::{path, reject, reply, Filter, Reply};

#[derive(Debug)]
pub struct FailedDeEscrow;

impl Error for FailedDeEscrow {}

impl std::fmt::Display for FailedDeEscrow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to de-escrow funds")
    }
}

impl reject::Reject for FailedDeEscrow {}

/// GET request for a specific commitment by key
pub fn de_escrow() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "de-escrow")
        .and(warp::body::json())
        .and_then(handle_de_escrow)
}

pub async fn handle_de_escrow(data: WithdrawDataReq) -> Result<impl Reply, warp::Rejection> {
    let token_type: TokenType = u8::from_str_radix(&data.token_type, 16)
        .map_err(|_| {
            error!("Could not convert token type");
            reject::custom(FailedDeEscrow)
        })?
        .into();
    let withdraw_data = WithdrawData::try_from(data).map_err(|e| {
        error!(
            "Could not convert Withdraw data request to WithdrawData: {}",
            e
        );
        reject::custom(FailedDeEscrow)
    })?;
    let available = Nightfall::<LocalWsClient>::withdraw_available(withdraw_data).await;
    match available {
        Ok(b) => {
            if b == 1 {
                info!("Withdraw is on chain, attempting to de-escrow funds");
                Nightfall::<LocalWsClient>::de_escrow_funds(withdraw_data, token_type)
                    .await
                    .map_err(|e| {
                        error!("Could not de-escrow funds: {}", e);
                        reject::custom(FailedDeEscrow)
                    })?;

                Ok(reply::with_status(reply::json(&b), StatusCode::OK))
            } else {
                info!("Not yet able to de-escrow funds");
                Ok(reply::with_status(reply::json(&b), StatusCode::NOT_FOUND))
            }
        }
        Err(e) => {
            error!("Nightfall contract error: {}", e);
            Err(reject::custom(FailedDeEscrow))
        }
    }
}
