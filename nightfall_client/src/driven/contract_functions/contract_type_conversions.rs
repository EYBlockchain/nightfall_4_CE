use crate::domain::entities::Proposer;
use nightfall_bindings::artifacts::RoundRobin;
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
