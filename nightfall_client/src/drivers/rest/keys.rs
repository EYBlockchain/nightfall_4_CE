use bip32::DerivationPath;
use warp::{hyper::StatusCode, path, reject, reply, Filter, Rejection, Reply};

use crate::get_zkp_keys;
use bip32::Mnemonic;

use lib::{
    client_models::KeyRequest,
    derive_key::{ZKPKeys, ZKPPubKey},
};

#[derive(Debug)]
struct BadKeyRequest;

impl reject::Reject for BadKeyRequest {}

// POST request to derive a new zkp_key from a mnemonic
pub fn derive_key_mnemonic(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "deriveKey")
        .and(warp::post())
        // make body optional
        .and(
            warp::body::json()
                .map(Some)
                .or_else(|_| async { Ok::<(Option<KeyRequest>,), Rejection>((None,)) }),
        )
        .and_then(handle_derive_key)
}

pub async fn handle_derive_key(
    key_request: Option<KeyRequest>,
) -> Result<impl Reply, warp::Rejection> {
    if let Some(req) = key_request {
        // validate mnemonic and path
        let valid_mnemonic = Mnemonic::new(req.mnemonic, Default::default())
            .map_err(|_| reject::custom(BadKeyRequest))?;
        let valid_derivation_path: DerivationPath = req
            .child_path
            .parse()
            .map_err(|_| reject::custom(BadKeyRequest))?;

        if let Ok(key) = ZKPKeys::derive_from_mnemonic(&valid_mnemonic, &valid_derivation_path) {
            // update the static
            let mut zkpk = get_zkp_keys().lock().expect("Poisoned lock");
            *zkpk = key.clone(); // store derived key
            let pubkey = ZKPPubKey::from(&key);
            Ok(reply::with_status(reply::json(&pubkey), StatusCode::OK))
        } else {
            Err(reject::not_found())
        }
    } else {
        // no body -> return existing static keys
        let zkpk = get_zkp_keys().lock().expect("Poisoned lock");
        // only serialize a safe view
        let pubkey = ZKPPubKey::from(&*zkpk);
        Ok(reply::with_status(reply::json(&pubkey), StatusCode::OK))
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
        let key = serde_json::from_slice::<ZKPPubKey>(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(key.zkp_public_key, test_key.zkp_public_key);
    }
}
