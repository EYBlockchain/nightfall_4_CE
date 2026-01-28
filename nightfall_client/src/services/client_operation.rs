use crate::{
    domain::{
        entities::{DepositSecret, Preimage, Salt},
        error::DepositError,
    },
    initialisation::get_db_connection,
    ports::{
        commitments::{Commitment, Nullifiable},
        contracts::{NightfallContract, TokenContract},
        db::CommitmentDB,
        secret_hash::SecretHash,
        trees::CommitmentTree,
    },
};
use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::BigInteger256;
use ark_std::Zero;
use configuration::addresses::get_addresses;
use jf_primitives::{poseidon::Poseidon, trees::MembershipProof};
use lib::{
    get_fee_token_id,
    hex_conversion::HexConvertible,
    nf_client_proof::{PrivateInputs, Proof, ProvingEngine, PublicInputs},
    shared_entities::{ClientTransaction, CompressedSecrets, TokenType},
};
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};

#[allow(clippy::too_many_arguments)]
pub async fn client_operation<P, E>(
    spend_commitments: &[impl Nullifiable; 4],
    new_commitments: &[impl Commitment; 4],
    root_key: Fr254,
    ephemeral_key: BJJScalar,
    withdraw_address: Fr254,
    secret_preimages: &[impl SecretHash; 4],
    id: &str,
) -> Result<ClientTransaction<P>, &'static str>
where
    P: Proof,
    E: ProvingEngine<P>,
{
    let nf_slot_id = new_commitments[0].get_nf_slot_id();
    // We check that value is conserved for the tokens being transferred and fees.
    let out_value = new_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let in_value = spend_commitments[..2]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let out_fee = new_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();
    let in_fee = spend_commitments[2..]
        .iter()
        .map(|c| c.get_value())
        .sum::<Fr254>();

    let value_conserved = out_value == in_value;
    let fee_conserved = out_fee == in_fee;

    if !(value_conserved && fee_conserved) {
        warn!("{id} Value or fee not conserved in this transaction: rejecting");
        return Err("Value or fee not conserved in this transaction: rejecting");
    }

    // Collect the public keys from the nullified commitments
    let public_keys: [TEAffine<BabyJubJub>; 4] = spend_commitments
        .iter()
        .map(|c| c.get_public_key())
        .collect::<Vec<TEAffine<BabyJubJub>>>()
        .try_into()
        .map_err(|_| "Could not convert to fixed length array")?;

    // Retrieve the Public Key used to create the commitments, if the withdraw address is zero this will be the public key on the first new commitment,
    // If the withdraw address is non-zero its a withdraw and the first output commitment will be zero anyway and so we enforce it is the neutral point.
    let recipient_public_key: TEAffine<BabyJubJub> = if withdraw_address.is_zero() {
        new_commitments[0].get_public_key()
    } else {
        TEAffine::<BabyJubJub>::zero()
    };
    // For each commitment that we are nullifying, we'll need to prove that they were added to the
    // commitment Merkle tree. This Merkle proof is available in the commitment DB because we will
    // have computed and stored it when the commitment was added to the tree (and hopefully updated
    //it since as the Merkle tree root is updated so that we obfuscate which block it was deposited in).
    debug!("{id} Finding membership proofs for spend commitments");
    let (membership_proofs, roots) = {
        let mut proofs = vec![];
        let mut roots = vec![];
        let db = get_db_connection().await;
        for commitment in spend_commitments.iter() {
            // ignore zero commitments here because they won't be in our commitment database. We check
            // for zero commitments by checking the preimage has default token id. We have already adjusted
            // the salt to be random so we can't use that.
            if commitment.get_nf_token_id() == Fr254::zero() {
                proofs.push(MembershipProof::default());
                roots.push(Fr254::zero());
                continue;
            }
            let commitment_hash = commitment.hash().map_err(|_| "Could not hash preimage")?;
            debug!(
                "{id} Looking for commitment with hash {}",
                Fr254::to_hex_string(&commitment_hash)
            );
            let stored = db.get_commitment(&commitment_hash).await;
            if stored.is_none() {
                return Err("Could not find commitment in commitment database");
            };
            // get a membership proof that is computed from the current root. This is more secure than using a membership proof
            // that is computed from the root at the time the commitment was added to the tree.
            let membership_proof = db
                .get_membership_proof(Some(&commitment_hash), None)
                .await
                .map_err(|_| "Could not get membership proof")?;
            let root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(db)
                .await
                .map_err(|_| "Could not get root")?;
            // check that the proof is valid (we may remove this check later if we need more speed)
            let hasher = Poseidon::new();
            membership_proof
                .verify(&root, &hasher)
                .map_err(|_| "Membership proof failed")?;
            proofs.push(membership_proof);
            roots.push(root);
            // while we're at it, we should store the spend commitments nullifier hash. Then, when we see that nullifier
            // on the blockchain, we can mark the commitment as nullified so we don't try to spend it again.
        }
        (proofs, roots)
    };
    let fixed_proofs: [MembershipProof<Fr254>; 4] = membership_proofs
        .try_into()
        .map_err(|_| "Could not convert membership proofs to fixed length array")?;
    let fixed_roots: [Fr254; 4] = roots
        .try_into()
        .map_err(|_| "Could not convert roots into fixed length array")?;
    // Construct Private Inputs [ Commitment value, salt, recipient public_key];
    let nf_address = get_addresses().nightfall();
    let nf_token_id = spend_commitments[0].get_nf_token_id();
    let fee_token_id = get_fee_token_id();
    let (mut public_inputs, mut private_inputs) = (
        PublicInputs::new()
            .fee(new_commitments[2].get_value())
            .roots(&fixed_roots)
            .build(),
        PrivateInputs::new()
            .nf_address(nf_address)
            .value(new_commitments[0].get_value())
            .nf_token_id(nf_token_id)
            .nf_slot_id(nf_slot_id)
            .fee_token_id(fee_token_id)
            .nullifiers_values(&spend_commitments.map(|c| c.get_value()))
            .nullifiers_salts(&spend_commitments.map(|c| c.get_salt()))
            .commitments_values(&[
                new_commitments[1].get_value(),
                new_commitments[3].get_value(),
            ])
            .commitments_salts(&[
                new_commitments[1].get_salt(),
                new_commitments[2].get_salt(),
                new_commitments[3].get_salt(),
            ])
            .public_keys(&public_keys)
            .recipient_public_key(recipient_public_key)
            .root_key(root_key)
            .ephemeral_key(ephemeral_key)
            .membership_proofs(&fixed_proofs)
            .withdraw_address(withdraw_address)
            .secret_preimages(&[
                secret_preimages[0].to_array(),
                secret_preimages[1].to_array(),
                secret_preimages[2].to_array(),
                secret_preimages[3].to_array(),
            ])
            .build(),
    );
    info!("{id} Generating proof");
    let wrapped_proof: Result<P, E::Error> = E::prove(&mut private_inputs, &mut public_inputs);
    debug!("{id} Creating client transaction");
    match wrapped_proof {
        Ok(proof) => Ok(ClientTransaction {
            fee: public_inputs.fee,
            historic_commitment_roots: public_inputs.roots,
            commitments: public_inputs.commitments,
            nullifiers: public_inputs.nullifiers,
            compressed_secrets: CompressedSecrets {
                cipher_text: public_inputs.compressed_secrets,
            },
            proof,
        }),
        Err(e) => {
            error!("{id} Proving error {e:?}");
            Err("Transaction could not be completed due to a proving error.")
        }
    }
}

/// This function is called when a deposit is being made. Since no proof is generated by the client in this case
/// we only need to deal with escrowing funds.
/// The function returns a tuple of Preimages. The first Preimage is for the value being deposited and the second
/// Preimage is for the deposit fee. If the deposit fee is zero, the second Preimage is None.
#[allow(clippy::too_many_arguments)]
pub async fn deposit_operation<T: TokenContract, N: NightfallContract>(
    erc_address: Fr254,
    value: Fr254,
    fee: Fr254,
    deposit_fee: Fr254,
    token_id: BigInteger256,
    secret_preimage: DepositSecret,
    token_type: TokenType,
    id: &str,
) -> Result<(Preimage, Option<Preimage>), DepositError> {
    // First we set approval for the nightfall instance.
    info!("{id} Setting transfer approval");
    T::set_approval(erc_address, value, token_id).await?;

    // Next we escrow the funds
    info!("{id} Escrowing funds");
    let [nf_token_id, nf_slot_id] = N::escrow_funds(
        erc_address,
        value,
        token_id,
        fee,
        deposit_fee,
        secret_preimage,
        token_type,
    )
    .await?;

    let fee_token_id = get_fee_token_id();

    // Check if the deposit_fee is zero
    if deposit_fee.is_zero() {
        // If deposit_fee is zero, return only `value` Preimage
        Ok((
            Preimage::new(
                value,
                nf_token_id,
                nf_slot_id,
                TEAffine::<BabyJubJub>::zero(),
                Salt::Deposit(secret_preimage),
            ),
            None, // No second Preimage
        ))
    } else {
        // Otherwise, return two Preimages `value` and `deposit_fee`
        Ok((
            Preimage::new(
                value,
                nf_token_id,
                nf_slot_id,
                TEAffine::<BabyJubJub>::zero(),
                Salt::Deposit(secret_preimage),
            ),
            Some(Preimage::new(
                deposit_fee,
                fee_token_id,
                fee_token_id,
                TEAffine::<BabyJubJub>::zero(),
                Salt::Deposit(secret_preimage),
            )),
        ))
    }
}
