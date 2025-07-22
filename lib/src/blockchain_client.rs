use alloy::pubsub::PubSubConnect;
use async_trait::async_trait;
use alloy::signers::Signer;
use alloy::providers::{Provider};
use alloy::primitives::{Address, U256};
use alloy::signers::local::PrivateKeySigner;
use serde::Deserialize;
use std::marker::Sync;
use url::Url;
use std::sync::Arc;

use crate::error::BlockchainClientConnectionError;

/// A blockchain client is able to connect to a blockchain node and sign transactions.
/// It is also able to listen for events on the blockchain (although it does not need
/// signing capabilty to do this).
///
#[async_trait]
pub trait BlockchainClientConnection: Clone + Send + Sync {
    type W: Signer + 'static;
    type T: PubSubConnect  + 'static;
    type S: for<'a> Deserialize<'a>;

    async fn is_connected(&self) -> bool;

    async fn get_balance(&self) -> Option<U256>;

    fn get_address(&self) -> Address;

    fn get_client(&self) -> Arc<dyn Provider>;

    fn get_signer(&self) -> PrivateKeySigner
    where
        Self: Sized;

    async fn new(
        url: Url,
        wallet: Self::W,
    ) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;

    async fn try_from_settings(settings: &Self::S) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;
}
