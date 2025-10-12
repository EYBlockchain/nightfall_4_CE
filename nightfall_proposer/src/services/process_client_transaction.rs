use crate::driven::nightfall_client_transaction::process_nightfall_client_transaction;
use lib::{nf_client_proof::{Proof, ProvingEngine}, shared_entities::ClientTransaction};
use std::error::Error;

pub async fn process_client_transaction<P, E>(
    client_transaction: ClientTransaction<P>,
) -> Result<(), Box<dyn Error>>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    // This just calls through to the repository, because both the event processor and the REST API
    // need to call the following function. Thus, it can't really be located in services, otherwise we'd be
    // doing a reentrant call from repository to services, which is a bit of an odd pattern.
    process_nightfall_client_transaction::<P, E>(client_transaction).await?;
    Ok(())
}
