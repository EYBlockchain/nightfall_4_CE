use crate::domain::entities::{Proposer, WithdrawData};
use alloy::primitives::Address;
use lib::contract_conversions::{Addr, Uint256};
use nightfall_bindings::artifacts::{Nightfall, RoundRobin};
/// enables conversion between a Proposer as used in the ProposerManager contract, and a for suitable for serialisation
impl From<RoundRobin::Proposer> for Proposer {
    fn from(proposer: RoundRobin::Proposer) -> Self {
        Proposer {
            stake: proposer.stake,
            addr: proposer.addr,
            url: proposer.url,
            next_addr: proposer.next_addr,
            previous_addr: proposer.previous_addr,
        }
    }
}

impl From<WithdrawData> for Nightfall::WithdrawData {
    fn from(data: WithdrawData) -> Nightfall::WithdrawData {
        let nf_token_id = Uint256::from(data.nf_token_id).0;
        let recipient_address = Address::from(
            Addr::try_from(data.withdraw_address)
                .expect("Could not convert WithdrawData withdraw address to Solidity address"),
        );
        let value = Uint256::from(data.value).0;
        let withdraw_fund_salt = Uint256::from(data.withdraw_fund_salt).0;
        Nightfall::WithdrawData {
            nf_token_id,
            recipient_address,
            value,
            withdraw_fund_salt,
        }
    }
}
