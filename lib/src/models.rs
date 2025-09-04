use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use warp::http::StatusCode;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// A structure representing a certificate validation request
pub struct CertificateReq {
    pub certificate_private_key: Vec<u8>,
    pub certificate: Vec<u8>,
}
#[derive(Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: &'a str,
}

type JsonWithStatus = warp::reply::WithStatus<warp::reply::Json>;

fn json_error(status: StatusCode, code: &'static str, message: &'static str) -> JsonWithStatus {
    let body = warp::reply::json(&ErrorBody { code, message });
    warp::reply::with_status(body, status)
}

pub fn bad_request(msg: &'static str) -> JsonWithStatus {
    json_error(StatusCode::BAD_REQUEST, "bad_request", msg)
}