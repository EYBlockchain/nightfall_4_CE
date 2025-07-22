use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_block_assembly_trigger, get_blockchain_client_connection},
    ports::{contracts::NightfallContract, proving::RecursiveProvingEngine},
    services::assemble_block::assemble_block,
};
use ark_serialize::SerializationError;
use configuration::addresses::get_addresses;
use jf_plonk::errors::PlonkError;
use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, error, info, warn};
use nightfall_client::{
    domain::error::{ConversionError, EventHandlerError, NightfallContractError},
    ports::proof::Proof,
};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use alloy::sol;
sol!(
    #[sol(rpc)]     // Add Debug trait to x509CheckReturn
    RoundRobin, "/Users/Swati.Rawal/nightfall_4_PV/blockchain_assets/artifacts/RoundRobin.sol/RoundRobin.json");

#[derive(Debug)]
pub enum BlockAssemblyError {
    FailedToAssembleBlock(String),
    FailedToProposeBlock,
    FailedToGetReceipt,
    FailedToGetCalldata,
    FailedToGetDepositData(NightfallContractError),
    InsufficientTransactions,
    SerializationError(SerializationError),
    ConversionError(ConversionError),
    ProvingError(String),
    ContractError(String),
    ProviderError(String),
    EventHandlerError(EventHandlerError),
}

impl From<EventHandlerError> for BlockAssemblyError {
    fn from(e: EventHandlerError) -> Self {
        BlockAssemblyError::EventHandlerError(e)
    }
}

impl Error for BlockAssemblyError {}
impl Display for BlockAssemblyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToAssembleBlock(s) => write!(f, "Failed to assemble block: {}", s),
            Self::FailedToProposeBlock => write!(f, "Failed to propose block"),
            Self::FailedToGetReceipt => write!(f, "Failed to get receipt for block"),
            Self::FailedToGetCalldata => write!(f, "Failed to get calldata for block"),
            Self::InsufficientTransactions => {
                write!(f, "At least two transactions are required per block")
            }
            Self::SerializationError(e) => {
                write!(f, "{e}")
            }
            Self::ConversionError(e) => {
                write!(f, "{e}")
            }
            Self::FailedToGetDepositData(e) => write!(f, "Failed to acquire deposit data: {}", e),
            Self::ProvingError(s) => write!(f, "Error occurred while proving: {} ", s),
            Self::ContractError(s) => write!(f, "Contract error: {}", s),
            Self::ProviderError(s) => write!(f, "Provider error: {}", s),
            Self::EventHandlerError(e) => write!(f, "Event handling error: {}", e),
        }
    }
}

impl From<SerializationError> for BlockAssemblyError {
    fn from(e: SerializationError) -> Self {
        BlockAssemblyError::SerializationError(e)
    }
}

impl From<ConversionError> for BlockAssemblyError {
    fn from(e: ConversionError) -> Self {
        BlockAssemblyError::ConversionError(e)
    }
}

impl From<NightfallContractError> for BlockAssemblyError {
    fn from(e: NightfallContractError) -> Self {
        BlockAssemblyError::FailedToGetDepositData(e)
    }
}

impl From<PlonkError> for BlockAssemblyError {
    fn from(e: PlonkError) -> Self {
        BlockAssemblyError::ProvingError(format!("PlonkError: {}", e))
    }
}

// once called this function will trigger the block assembly process whenever
// certain conditions are met
// Any errors that propogate back up to here will cause a panic.
pub async fn start_block_assembly<P, R, N>() -> Result<(), BlockAssemblyError>
where
    P: Proof,
    R: RecursiveProvingEngine<P> + Send + Sync + 'static,
    N: NightfallContract,
{
    let blockchain_client = get_blockchain_client_connection()
    .await
    .read()
    .await
    .get_client();

    let round_robin_instance = RoundRobin::new(
        get_addresses().round_robin,
        blockchain_client.root(),
    );
    debug!("Starting block assembly");
    loop {
        debug!("Checking proposer status...");
        // Step 1: Get current proposer address from smart contract
        let current_proposer = match round_robin_instance
            .get_current_proposer_address()
            .call()
            .await
        {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to get current proposer: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        }._0;

        let our_address = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_address();

        // Step 2: If we are not the proposer, wait and retry
        if current_proposer != our_address {
            info!(
                "We are not the current proposer. Current proposer is: {:?}",
                current_proposer
            );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

        // Step 3: We are the current proposer. Wait for trigger.
        info!("We are the current proposer. Awaiting trigger...");
        get_block_assembly_trigger::<P>()
            .await
            .read()
            .await
            .await_trigger()
            .await;
        let current_proposer_after_trigger = match round_robin_instance
            .get_current_proposer_address()
            .call()
            .await
        {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to get current proposer after trigger: {}", e);
                continue;
            }
        }._0;

        let our_address = get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_address();

        if current_proposer_after_trigger != our_address {
            info!(
        "Proposer has changed after trigger. Skipping block assembly. New proposer is: {:?}",
        current_proposer_after_trigger
    );
            continue;
        }
        // Step 4: check if we're synchronised.
        // Go round again if we're not because we can't make new blocks
        let mut sync_status = get_synchronisation_status().await.write().await;
        let current_block_number = N::get_current_layer2_blocknumber().await.map_err(|_| {
            BlockAssemblyError::FailedToAssembleBlock(
                "Failed to get current block number".to_string(),
            )
        })?;
        if current_block_number.is_zero() {
            // if we're at block 0, we're automatically synchronised because no blocks have been made yet
            sync_status.set_synchronised();
        }
        if !sync_status.is_synchronised() {
            warn!("We are not synchronised. We won't make blocks until we are");
            continue;
        }

        debug!("Triggered block assembly");
        let block_result = assemble_block::<P, R>().await;
        // now we have a result, check if it's in error; if it is and it's just that we don't have >=2 transactions
        // yet (an InsufficientTransactions error) we can continue, otherwise panic (at least for now)
        let block = match block_result {
            Ok(block) => block,
            Err(e) => match e {
                BlockAssemblyError::InsufficientTransactions => continue,
                _ => {
                    error!("Block assembly failed with error {}", e);
                    continue;
                }
            },
        };

        // Step 6: Propose the block on-chain
        // send the block to the blockchain by calling Nighfall.sol's proposeBlock function
        info!("*Proposing block*");
        match N::propose_block(block).await {
            Ok(_) => {
                info!("Block proposed successfully");
            }
            Err(e) => {
                error!("Failed to propose block: {}", e);
            }
        }; // we'll move on to the next block even if this block fails
    }
}
