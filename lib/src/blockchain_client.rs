use async_trait::async_trait;
use ethers::{
    middleware::SignerMiddleware,
    providers::{JsonRpcClient, Provider, PubsubClient},
    types::{Address, U256},
};
use serde::Deserialize;
use std::{marker::Sync, sync::Arc};
use url::Url;

use crate::error::BlockchainClientConnectionError;

/// A blockchain client is able to connect to a blockchain node and sign transactions.
/// It is also able to listen for events on the blockchain (although it does not need
/// signing capabilty to do this).
///
#[async_trait]
pub trait BlockchainClientConnection: Clone + Send + Sync {
    type W: ethers::signers::Signer + 'static;
    type T: JsonRpcClient + PubsubClient + 'static;
    type S: for<'a> Deserialize<'a>;

    async fn is_connected(&self) -> bool;

    async fn get_balance(&self) -> Option<U256>;

    fn get_address(&self) -> Option<Address>;

    fn get_client(&self) -> Arc<SignerMiddleware<Provider<Self::T>, Self::W>>
    where
        Self: Sized;

    async fn new(
        url: Url,
        wallet: Self::W,
        chain_id: u64,
    ) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;

    async fn try_from_settings(settings: &Self::S) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;
}
