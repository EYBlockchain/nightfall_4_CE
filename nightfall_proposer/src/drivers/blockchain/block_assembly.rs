use crate::{
    drivers::blockchain::nightfall_event_listener::get_synchronisation_status,
    initialisation::{get_block_assembly_trigger, get_blockchain_client_connection},
    ports::{contracts::NightfallContract, proving::RecursiveProvingEngine},
    services::assemble_block::assemble_block,
};
use ark_serialize::SerializationError;
use configuration::addresses::get_addresses;

use ethers::providers::ProviderError;
use jf_plonk::errors::PlonkError;
use lib::blockchain_client::BlockchainClientConnection;
use log::{debug, error, info, warn};
use nightfall_bindings::round_robin::RoundRobin;
use nightfall_client::{
    domain::error::{ConversionError, NightfallContractError},
    ports::proof::Proof,
};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

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
    ProviderError(ProviderError),
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
            Self::ProviderError(e) => write!(f, "Provider error: {}", e),
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
    let round_robin_instance = RoundRobin::new(
        get_addresses().round_robin,
        get_blockchain_client_connection()
            .await
            .read()
            .await
            .get_client(),
    );
    debug!("Starting block assembly");
    loop {
        debug!("Waiting for trigger");
        get_block_assembly_trigger::<P>()
            .await
            .read()
            .await
            .await_trigger()
            .await;
        // check if we're the current proposer. Go round again if we're not
        match round_robin_instance
            .get_current_proposer_address()
            .call()
            .await
        {
            Ok(current_proposer) => {
                info!("The current proposer is: {:?}", current_proposer);
                if current_proposer
                    != get_blockchain_client_connection()
                        .await
                        .read()
                        .await
                        .get_client()
                        .address()
                {
                    info!("We are not the current proposer");
                    continue; // in this case we're not the current proposer
                }
            }
            Err(e) => panic!("Failed to get current proposer: {}", e), // in this case we have a problem and can't recover.
        }
        // check if we're synchronised. Go round again if we're not because we can't make new blocks
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

        info!("We are the current proposer");
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
