use async_trait::async_trait;
use nightfall_client::ports::proof::{Proof, ProvingEngine};

use super::contracts::NightfallContract;

#[async_trait]
pub trait Synchroniser {
    type Error;

    async fn sync<P: Proof, E: ProvingEngine<P>, N: NightfallContract>(
        &mut self,
        start_block: usize,
    ) -> Result<(), Self::Error>;

    fn is_syncing(&self) -> bool;

    fn new() -> Self;
}
