use crate::error::BlockchainClientConnectionError;
use crate::wallets::WalletType;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::pubsub::PubSubConnect;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use async_trait::async_trait;
use serde::Deserialize;
use std::{marker::Sync, sync::Arc};
use url::Url;

/// A blockchain client is able to connect to a blockchain node and sign transactions.
/// It is also able to listen for events on the blockchain (although it does not need
/// signing capabilty to do this).
///
#[async_trait]
pub trait BlockchainClientConnection: Clone + Send + Sync {
    type W: Signer + 'static;
    type T: PubSubConnect + 'static;
    type S: for<'a> Deserialize<'a>;

    async fn is_connected(&self) -> bool;

    async fn get_balance(&self) -> Option<U256>;

    fn get_address(&self) -> Address;

    fn get_client(&self) -> Arc<dyn Provider>;

    fn get_wallet_type(&self) -> &WalletType;

    fn get_signer(&self) -> Arc<PrivateKeySigner>
    where
        Self: Sized;

    async fn new(url: Url, wallet: Self::W) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;

    async fn try_from_settings(settings: &Self::S) -> Result<Self, BlockchainClientConnectionError>
    where
        Self: Sized;
}
