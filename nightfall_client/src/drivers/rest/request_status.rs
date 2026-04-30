use crate::{
    domain::entities::{Request, RequestStatus},
    driven::queue::get_queue,
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    drivers::rest::client_nf_3::SwapChildRequestArgs,
    initialisation::get_db_connection,
    ports::{contracts::NightfallContract, db::RequestDB},
};
use alloy::primitives::I256;
use ark_bn254::Fr as Fr254;
use lib::hex_conversion::HexConvertible;
use log::{debug, warn};
use uuid::Uuid;
use warp::{http::StatusCode, path, reply::Reply, Filter};

/// This module provides an end point for querying the status of a request
pub fn get_request_status<N: NightfallContract>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "request" / String)
        .and(warp::get())
        .and_then(handle_get_request_status::<N>)
}

fn extract_swap_deadline(request: &Request) -> Option<Fr254> {
    let child_args = request.child_request_args.as_ref()?;
    let swap_args = serde_json::from_str::<SwapChildRequestArgs>(child_args).ok()?;
    let deadline_hex = swap_args.deadline?;
    Fr254::from_hex_string(&deadline_hex).ok()
}

fn should_expire_request(request: &Request, current_l2_block: I256) -> bool {
    if !matches!(request.status, RequestStatus::Submitted) {
        return false;
    }

    if current_l2_block < I256::ZERO {
        return false;
    }

    let Some(deadline) = extract_swap_deadline(request) else {
        return false;
    };

    deadline < Fr254::from(current_l2_block.as_u64())
}

pub async fn handle_get_request_status<N: NightfallContract>(
    id: String,
) -> Result<impl Reply, warp::Rejection> {
    // check if the id is a valid uuid
    match Uuid::parse_str(&id) {
        Ok(_) => {}
        Err(_) => {
            return Err(warp::reject::custom(
                crate::domain::error::ClientRejection::InvalidRequestId,
            ));
        }
    };
    let db = get_db_connection().await;
    // get the request
    debug! {"Getting request status for {id}"};
    let mut request = db.get_request(&id).await;
    debug! {"Request status: {request:?}"};

    if let Some(existing_request) = request.as_mut() {
        if matches!(existing_request.status, RequestStatus::Submitted) {
            match get_synchronisation_status::<N>().await {
                Ok(sync_status) if sync_status.is_synchronised() => {
                    match N::get_current_layer2_blocknumber().await {
                        Ok(current_l2_block)
                            if should_expire_request(existing_request, current_l2_block) =>
                        {
                            if db
                                .update_request(&id, RequestStatus::Expired)
                                .await
                                .is_none()
                            {
                                warn!("{id} Failed to persist Expired status after swap deadline");
                            }
                            existing_request.status = RequestStatus::Expired;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            warn!(
                                "{id} Failed to read current L2 block while reconciling request status: {e}"
                            );
                        }
                    }
                }
                Ok(_) => {
                    debug!(
                        "{id} Skipping swap expiry reconciliation while client is not synchronized"
                    );
                }
                Err(e) => {
                    warn!(
                        "{id} Failed to read synchronisation status while reconciling request status: {e}"
                    );
                }
            }
        }
    }

    if let Some(request) = request {
        Ok(warp::reply::with_status(
            serde_json::to_string(&request).unwrap(),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(
            crate::domain::error::ClientRejection::RequestNotFound,
        ))
    }
}

/// This endpoint is used to get the length of thr request queue
pub fn get_queue_length(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "queue")
        .and(warp::get())
        .and_then(handle_get_queue_length)
}
pub async fn handle_get_queue_length() -> Result<impl Reply, warp::Rejection> {
    let length = get_queue().await.read().await.len();
    Ok(warp::reply::with_status(
        serde_json::to_string(&length).unwrap(),
        StatusCode::OK,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(status: RequestStatus, child_request_args: Option<String>) -> Request {
        Request {
            status,
            uuid: "test-request".to_string(),
            child_request_args,
        }
    }

    #[test]
    fn test_extract_swap_deadline_from_child_args() {
        let request = make_request(
            RequestStatus::Submitted,
            Some(r#"{"deadline":"0x10"}"#.to_string()),
        );

        assert_eq!(extract_swap_deadline(&request), Some(Fr254::from(16u64)));
    }

    #[test]
    fn test_extract_swap_deadline_ignores_non_swap_child_args() {
        let request = make_request(
            RequestStatus::Submitted,
            Some(r#"{"tokenId":"0x01","recipientAddress":"0x02"}"#.to_string()),
        );

        assert_eq!(extract_swap_deadline(&request), None);
    }

    #[test]
    fn test_should_expire_request_only_after_deadline() {
        let request = make_request(
            RequestStatus::Submitted,
            Some(r#"{"deadline":"0x10"}"#.to_string()),
        );

        assert!(!should_expire_request(
            &request,
            I256::try_from(16u64).unwrap()
        ));
        assert!(should_expire_request(
            &request,
            I256::try_from(17u64).unwrap()
        ));
    }

    #[test]
    fn test_should_not_expire_non_submitted_request() {
        let request = make_request(
            RequestStatus::Confirmed,
            Some(r#"{"deadline":"0x10"}"#.to_string()),
        );

        assert!(!should_expire_request(
            &request,
            I256::try_from(17u64).unwrap()
        ));
    }

    #[test]
    fn test_should_not_expire_without_deadline_metadata() {
        let request = make_request(RequestStatus::Submitted, None);

        assert!(!should_expire_request(
            &request,
            I256::try_from(17u64).unwrap()
        ));
    }
}
