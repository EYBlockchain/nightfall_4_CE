use bip32::DerivationPath;
use warp::{hyper::StatusCode, path, reject, reply, Filter, Reply};

use crate::drivers::derive_key::ZKPKeys;

use super::models::KeyRequest;

#[derive(Debug)]
struct BadKeyRequest;

impl reject::Reject for BadKeyRequest {}

// POST request to derive a new zkp_key from a mnemonic
pub fn derive_key_mnemonic(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "deriveKey")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_derive_key)
}

pub async fn handle_derive_key(key_request: KeyRequest) -> Result<impl Reply, warp::Rejection> {
    let valid_mnemonic = bip32::Mnemonic::new(key_request.mnemonic, Default::default())
        .map_err(|_| reject::custom(BadKeyRequest))?;
    let valid_derivation_path: DerivationPath = key_request
        .child_path
        .parse()
        .map_err(|_| reject::custom(BadKeyRequest))?;

    if let Ok(key) = ZKPKeys::derive_from_mnemonic(&valid_mnemonic, &valid_derivation_path) {
        Ok(reply::with_status(reply::json(&key), StatusCode::OK))
    } else {
        Err(reject::not_found())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip32::{DerivationPath, Mnemonic};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_rest_key_derivation() {
        let key_request = KeyRequest{
            mnemonic: "spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work".to_string(),
            child_path: "m/44'/60'/0'/0/0".to_string()
        };
        let test_key = ZKPKeys::derive_from_mnemonic(
            &Mnemonic::new(&key_request.mnemonic, Default::default()).unwrap(),
            &DerivationPath::from_str(&key_request.child_path).unwrap(),
        )
        .unwrap();
        let filter = derive_key_mnemonic();
        let res = warp::test::request()
            .method("POST")
            .path("/v1/deriveKey")
            .json(&key_request)
            .reply(&filter)
            .await;
        let key = serde_json::from_slice::<ZKPKeys>(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(key, test_key);
    }
}
