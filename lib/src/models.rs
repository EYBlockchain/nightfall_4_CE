use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// A structure representing a certificate validation request
pub struct CertificateReq {
    pub certificate_private_key: Vec<u8>,
    pub certificate: Vec<u8>,
}
