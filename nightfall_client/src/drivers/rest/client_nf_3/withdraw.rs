use super::{handle_client_operation, WithdrawResponse};
use crate::{
    domain::{
        entities::{Operation, OperationType, Transport},
        error::TransactionHandlerError,
        notifications::NotificationPayload,
    },
    get_zkp_keys,
    initialisation::get_db_connection,
    ports::{
        contracts::NightfallContract,
        db::{CommitmentDB, CommitmentEntryDB, RequestDB},
    },
    services::commitment_selection::find_usable_commitments,
};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine;
use ark_ff::Zero;
use configuration::addresses::get_addresses;
use lib::{
    client_models::{DeEscrowDataReq, NF3WithdrawRequest},
    commitments::{Commitment, Nullifiable},
    contract_conversions::FrBn254,
    get_fee_token_id,
    hex_conversion::HexConvertible,
    nf_client_proof::{Proof, ProvingEngine},
    nf_token_id::to_nf_token_id_from_str,
    shared_entities::{Preimage, Salt},
};
use tracing::{debug, error, info};
use nf_curves::ed_on_bn254::{BabyJubjub, Fr as BJJScalar};

pub(crate) async fn handle_withdraw<P, E, N>(
    withdraw_req: NF3WithdrawRequest,
    id: &str,
) -> Result<NotificationPayload, TransactionHandlerError>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let NF3WithdrawRequest {
        erc_address,
        token_id,
        value,
        recipient_address,
        fee,
        ..
    } = withdraw_req;

    // add the id to the request database

    // Convert the request into the relevant types.
    let nf_token_id =
        to_nf_token_id_from_str(erc_address.as_str(), token_id.as_str()).map_err(|e| {
            error!(
                "{id} Error when retrieving the Nightfall token id from the erc address and token ID {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;

    let keys = get_zkp_keys().lock().expect("Poisoned Mutex lock").clone();

    let value = Fr254::from_hex_string(value.as_str()).map_err(|e| {
        error!("{id} Error when reading value: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let fee: Fr254 = Fr254::from_hex_string(fee.as_str()).map_err(|e| {
        error!("{id} Error when reading fee: {e}");
        TransactionHandlerError::CustomError(e.to_string())
    })?;

    let recipient_address: Fr254 =
        Fr254::from_hex_string(recipient_address.as_str()).map_err(|e| {
            error!("{id} Error when reading recipeint address: {e}");
            TransactionHandlerError::CustomError(e.to_string())
        })?;
    // For now we just use the commitment selection algorithm to minimise change.
    let spend_commitments;
    let db = get_db_connection().await;

    {
        let fee_token_id = get_fee_token_id();
        let spend_value_commitments = find_usable_commitments(nf_token_id, value,db)
        .await.map_err(|e|{
            error!("{id} Could not find enough usable value commitments to complete this withdraw, suggest depositing more tokens: {e}");
            TransactionHandlerError::CustomError(e.to_string())})?;
        let spend_fee_commitments = if fee.is_zero() {
            [Preimage::default(), Preimage::default()]
        } else {
            match find_usable_commitments(fee_token_id, fee, db).await {
                Ok(commitments) => commitments,
                Err(e) => {
                    error!("{id} Could not find enough usable fee commitments to complete this withdraw, suggest depositing more fee: {e}");
                    // rollback the value commitments to unspent if fails to find fee commitments
                    let value_commitment_ids = spend_value_commitments
                        .iter()
                        .filter_map(|c| match c.hash() {
                            Ok(h) => Some(h),
                            Err(e) => {
                                error!("{id} Failed to hash commitment during rollback, commitment may be orphaned: {e}");
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    for commitment_id in &value_commitment_ids {
                        if let Some(existing) = db.get_commitment(commitment_id).await {
                            let _ = db
                                .mark_commitments_unspent(
                                    &[*commitment_id],
                                    existing.layer_1_transaction_hash,
                                    existing.layer_2_block_number,
                                )
                                .await;
                        }
                    }
                    return Err(TransactionHandlerError::CustomError(e.to_string()));
                }
            }
        };
        spend_commitments = [
            spend_value_commitments[0],
            spend_value_commitments[1],
            spend_fee_commitments[0],
            spend_fee_commitments[1],
        ];
    }
    // Work out how much change is needed.
    let total_token_value = spend_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let token_change = total_token_value - value;

    let total_fee_value = spend_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let fee_change = total_fee_value - fee;

    let nightfall_address = FrBn254::from(get_addresses().nightfall()).0;
    let contract_nf_address = Affine::<BabyJubjub>::new_unchecked(Fr254::zero(), nightfall_address);

    // The first commitment of the withdraw is 0, which will be calculated in the circuit
    // here, we set new_commitment_one to have the withdraw value so we can check that value is conserved for transfer and withdraw in client_operation services.
    // We set public_key of this preimage to the contract_nf_address, so that it won't be added in PendingCommitment later (as we only add preimages in PendingCommitment iff commitment.get_public_key() == zkp_public_key).
    let new_commitment_one = Preimage::new(
        value,
        nf_token_id,
        spend_commitments[0].get_nf_slot_id(),
        contract_nf_address,
        Salt::new_transfer_salt(),
    );

    let new_commitment_two = if !token_change.is_zero() {
        Preimage::new(
            token_change,
            nf_token_id,
            spend_commitments[0].get_nf_slot_id(),
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let fee_token_id = get_fee_token_id();

    let new_commitment_three = if !fee.is_zero() {
        Preimage::new(
            fee,
            fee_token_id,
            fee_token_id,
            contract_nf_address,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };
    let new_commitment_four = if !fee_change.is_zero() {
        Preimage::new(
            fee_change,
            fee_token_id,
            fee_token_id,
            keys.zkp_public_key,
            Salt::new_transfer_salt(),
        )
    } else {
        Preimage::default()
    };

    let new_commitments = [
        new_commitment_one,
        new_commitment_two,
        new_commitment_three,
        new_commitment_four,
    ];

    let secret_preimages = [
        spend_commitments[0].get_secret_preimage(),
        spend_commitments[1].get_secret_preimage(),
        spend_commitments[2].get_secret_preimage(),
        spend_commitments[3].get_secret_preimage(),
    ];
    let op = Operation {
        transport: Transport::OffChain,
        operation_type: OperationType::Withdraw,
    };
    let withdraw_fund_salt = spend_commitments[0]
        .nullifier_hash(&keys.nullifier_key)
        .map_err(|e| {
            error!("{id} Failed to compute nullifier hash: {e}");
            TransactionHandlerError::CustomError(format!("Failed to compute nullifier hash: {e}"))
        })?;
    match handle_client_operation::<P, E, N>(
        op,
        spend_commitments,
        new_commitments,
        BJJScalar::zero(),
        recipient_address,
        secret_preimages,
        id,
    )
    .await
    {
        Ok(res) => {
            let de_escrow_req = DeEscrowDataReq {
                token_id: token_id.clone(),
                erc_address: erc_address.clone(),
                recipient_address: recipient_address.to_hex_string(),
                value: value.to_hex_string(),
                token_type: withdraw_req.token_type.clone(),
                withdraw_fund_salt: withdraw_fund_salt.to_hex_string(),
            };
            match serde_json::to_string(&de_escrow_req) {
                Ok(child_args_json) => {
                    if db
                        .update_request_child_args(id, &child_args_json)
                        .await
                        .is_none()
                    {
                        error!("{id} Failed to store child_request_args in database");
                    } else {
                        debug!("{id} Successfully stored child_request_args in request collection");
                    }
                }
                Err(e) => {
                    error!("{id} Failed to serialize de_escrow_req: {e}");
                }
            }
            res
        }
        Err(e) => {
            // Rollback to UNSPENT status if handle_client_operation fails
            let db = get_db_connection().await;

            // Rollback spend commitments
            let commitment_ids = spend_commitments
                .iter()
                .filter_map(|c| match c.hash() {
                    Ok(h) => Some(h),
                    Err(e) => {
                        error!("{id} Failed to hash commitment during rollback, commitment may be orphaned: {e}");
                        None
                    }
                })
                .collect::<Vec<_>>();

            info!(
                "{id} Rolling back {} spend commitments to Unspent",
                commitment_ids.len()
            );
            for commitment_id in &commitment_ids {
                if let Some(existing) = db.get_commitment(commitment_id).await {
                    let _ = db
                        .mark_commitments_unspent(
                            &[*commitment_id],
                            existing.layer_1_transaction_hash,
                            existing.layer_2_block_number,
                        )
                        .await;
                }
            }

            // Delete new commitments
            let new_commitment_ids = new_commitments
                .iter()
                .filter_map(|c| match c.hash() {
                    Ok(h) => Some(h),
                    Err(e) => {
                        error!("{id} Failed to hash commitment during rollback, commitment may be orphaned: {e}");
                        None
                    }
                })
                .collect::<Vec<_>>();

            info!("{id} Deleting {} new commitments", new_commitment_ids.len());
            let _ = db.delete_commitments(new_commitment_ids).await;
            return Err(e);
        }
    };

    // Build the response
    let withdraw_response = WithdrawResponse {
        success: true,
        message: "Withdraw operation completed successfully".to_string(),
        withdraw_fund_salt: withdraw_fund_salt.to_hex_string(),
    };

    let response = serde_json::to_string(&withdraw_response).map_err(|e| {
        error!("{id} Error when serialising response: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;
    let uuid = serde_json::to_string(&id).map_err(|e| {
        error!("{id} Error when serialising request ID: {e}");
        TransactionHandlerError::JsonConversionError(e)
    })?;

    // Return the response as JSON
    Ok(NotificationPayload::TransactionEvent { response, uuid })
}
