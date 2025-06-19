//! This module defines interfaces for different types of smart contract that NIghtfall 4 deals with.
//! These mainly include token contracts and the "Nightfall" contract.

use crate::domain::{
    entities::{DepositSecret, TokenType, WithdrawData},
    error::{NightfallContractError, TokenContractError},
};
use ark_bn254::Fr as Fr254;
use ark_ff::BigInteger256;
use ethers::types::{H160, I256};
use futures::Future;
use nightfall_bindings::nightfall::Block;

/// Interface trait for a token contract.
pub trait TokenContract {
    /// We need to be able to set approval for transferring of funds
    fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> impl Future<Output = Result<(), TokenContractError>> + Send;
}

/// Interface trait for the Nightfall contract we are using.
pub trait NightfallContract {
    /// Function we call when we wish to escrow funds for a deposit.
    /// The values returned will be the Nightfall Token Id and the Nightfall Slot Id.
    fn escrow_funds(
        token_erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
        fee: Fr254,
        deposit_fee: Fr254,
        secret_preimage: DepositSecret,
        token_type: TokenType,
    ) -> impl Future<Output = Result<[Fr254; 2], NightfallContractError>> + Send;

    /// Function to retrieve the address of the contract
    fn get_address() -> Fr254;

    /// Function to de-escrow funds
    fn de_escrow_funds(
        withdraw_data: WithdrawData,
        token_type: TokenType,
    ) -> impl Future<Output = Result<(), NightfallContractError>> + Send;

    /// Function to see if funds are available to withdraw
    fn withdraw_available(
        withdraw_data: WithdrawData,
    ) -> impl Future<Output = Result<bool, NightfallContractError>> + Send;

    fn get_current_layer2_blocknumber(
    ) -> impl Future<Output = Result<I256, NightfallContractError>> + Send;

    fn get_layer2_block_by_number(
        block_number: I256,
    ) -> impl Future<Output = Result<(H160, Block), NightfallContractError>> + Send;
}
