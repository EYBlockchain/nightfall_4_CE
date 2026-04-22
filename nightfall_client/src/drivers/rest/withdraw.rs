use super::client_nf_3::parse_token_type;
use crate::ports::contracts::NightfallContract;
use ::nightfall_bindings::artifacts::Nightfall;
use lib::{client_models::DeEscrowDataReq, shared_entities::WithdrawData as NFWithdrawData};
use log::{debug, error};
use reqwest::StatusCode;
use warp::{reject, Reply};

pub async fn handle_de_escrow(data: DeEscrowDataReq) -> Result<impl Reply, warp::Rejection> {
    let token_type = parse_token_type(data.token_type.as_str()).map_err(|e| {
        error!("Could not convert token type: {e}");
        reject::custom(crate::domain::error::ClientRejection::FailedDeEscrow)
    })?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_de_escrow_request() -> DeEscrowDataReq {
        DeEscrowDataReq {
            token_id: "0x00".to_string(),
            erc_address: "0x1234567890123456789012345678901234567890".to_string(),
            recipient_address: "0x01".to_string(),
            value: "0x01".to_string(),
            token_type: "00".to_string(),
            withdraw_fund_salt: "0x01".to_string(),
        }
    }

    #[tokio::test]
    async fn test_handle_de_escrow_rejects_invalid_token_type_before_contract_calls() {
        let mut req = sample_de_escrow_request();
        req.token_type = "zz".to_string();

        let err = match handle_de_escrow(req).await {
            Ok(_) => panic!("invalid token type should be rejected"),
            Err(err) => err,
        };
        let rejection = err
            .find::<crate::domain::error::ClientRejection>()
            .expect("expected client rejection");

        assert_eq!(rejection.to_string(), "Failed to de-escrow funds");
    }

    #[tokio::test]
    async fn test_handle_de_escrow_rejects_malformed_payload_before_contract_calls() {
        let mut req = sample_de_escrow_request();
        req.recipient_address = "not-hex".to_string();

        let err = match handle_de_escrow(req).await {
            Ok(_) => panic!("malformed payload should be rejected"),
            Err(err) => err,
        };
        let rejection = err
            .find::<crate::domain::error::ClientRejection>()
            .expect("expected client rejection");

        assert_eq!(rejection.to_string(), "Failed to de-escrow funds");
    }
}
