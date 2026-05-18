use crate::{
    domain::error::DepositError,
    initialisation::get_db_connection,
    ports::{
        contracts::{NightfallContract, TokenContract},
        db::CommitmentDB,
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
    commitments::{Commitment, Nullifiable},
    derive_key::ZKPKeys,
    get_fee_token_id,
    hex_conversion::HexConvertible,
    nf_client_proof::{PrivateInputs, Proof, ProvingEngine, PublicInputs},
    secret_hash::SecretHash,
    shared_entities::{
        ClientTransaction, CompressedSecrets, DepositData, DepositSecret, Preimage, Salt, TokenType,
    },
};
use log::{debug, error, info, warn};
use nf_curves::ed_on_bn254::{BabyJubjub as BabyJubJub, Fr as BJJScalar};

const MEMBERSHIP_PROOF_SNAPSHOT_ATTEMPTS: usize = 3;

async fn collect_membership_proofs_with_stable_root(
    spend_commitments: &[impl Nullifiable; 4],
    id: &str,
) -> Result<(Vec<MembershipProof<Fr254>>, Fr254), &'static str> {
    let db = get_db_connection().await;
    let hasher = Poseidon::new();

    for attempt in 1..=MEMBERSHIP_PROOF_SNAPSHOT_ATTEMPTS {
        let target_root = <mongodb::Client as CommitmentTree<Fr254>>::get_root(db)
            .await
            .map_err(|_| "Could not get root")?;
        let mut proofs = Vec::with_capacity(spend_commitments.len());
        let mut saw_non_zero_commitment = false;
        let mut should_retry = false;

        for commitment in spend_commitments.iter() {
            if commitment.get_nf_token_id() == Fr254::zero() {
                proofs.push(MembershipProof::default());
                continue;
            }

            saw_non_zero_commitment = true;
            let commitment_hash = commitment.hash().map_err(|_| "Could not hash preimage")?;
            debug!(
                "{id} Looking for commitment with hash {}",
                Fr254::to_hex_string(&commitment_hash)
            );

            if db.get_commitment(&commitment_hash).await.is_none() {
                return Err("Could not find commitment in commitment database");
            }

            let membership_proof = db
                .get_membership_proof(Some(&commitment_hash), None)
                .await
                .map_err(|_| "Could not get membership proof")?;

            if membership_proof.verify(&target_root, &hasher).is_err() {
                warn!(
                    "{id} Commitment tree root changed while collecting membership proofs (attempt {attempt})"
                );
                should_retry = true;
                break;
            }

            proofs.push(membership_proof);
        }

        if !should_retry {
            let root = if saw_non_zero_commitment {
                target_root
            } else {
                Fr254::zero()
            };
            return Ok((proofs, root));
        }
    }

    error!("{id} Commitment tree root kept changing while collecting membership proofs");
    Err("Commitment tree root changed while building membership proofs")
}

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
    let (membership_proofs, root) =
        collect_membership_proofs_with_stable_root(spend_commitments, id).await?;
    let fixed_proofs: [MembershipProof<Fr254>; 4] = membership_proofs
        .try_into()
        .map_err(|_| "Could not convert membership proofs to fixed length array")?;
    let keys = ZKPKeys::new(root_key)
        .map_err(|_| "Transaction could not be completed due to an invalid root key.")?;
    // Construct Private Inputs [ Commitment value, salt, recipient public_key];
    let nf_address = get_addresses().nightfall();
    let nf_token_id = spend_commitments[0].get_nf_token_id();
    let fee_token_id = get_fee_token_id();
    let (mut public_inputs, mut private_inputs) = (
        PublicInputs::new()
            .fee(new_commitments[2].get_value())
            .root(root)
            .build(),
        PrivateInputs::new()
            .nf_address(nf_address)
            .value_a(new_commitments[0].get_value())
            .nf_token_a_id(nf_token_id)
            .nf_slot_id(nf_slot_id)
            .fee_token_id(fee_token_id)
            .nullifiers_values(&spend_commitments.map(|c| c.get_value()))
            .nullifiers_salts(&spend_commitments.map(|c| c.get_salt()))
            .commitments_values(&[
                new_commitments[1].get_value(),
                new_commitments[3].get_value(),
            ])
            .sender_commitment_salts(&[
                new_commitments[1].get_salt(),
                new_commitments[2].get_salt(),
                new_commitments[3].get_salt(),
            ])
            .public_keys(&public_keys)
            .party_a_public_key(keys.zkp_public_key)
            .recipient_public_key(recipient_public_key)
            .nf_token_b_id(Fr254::zero())
            .value_b(Fr254::zero())
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
            .deposit_data(&[DepositData::default(); 4])
            .build(),
    );
    info!("{id} Generating proof");
    let wrapped_proof: Result<P, E::Error> = E::prove(&mut private_inputs, &mut public_inputs);
    debug!("{id} Creating client transaction");
    match wrapped_proof {
        Ok(proof) => Ok(ClientTransaction {
            fee: public_inputs.fee,
            historic_commitment_root: public_inputs.root,
            commitments: public_inputs.commitments,
            nullifiers: public_inputs.nullifiers,
            compressed_secrets: CompressedSecrets {
                cipher_text: public_inputs.compressed_secrets,
            },
            swap_link: public_inputs.swap_link,
            deadline: public_inputs.deadline,
            swap_side: public_inputs.swap_side,
            proof,
        }),
        Err(e) => {
            error!("{id} Proving error {e:?}");
            Err("Transaction could not be completed due to a proving error.")
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn swap_operation<P, E>(
    spend_commitments: &[impl Nullifiable; 4],
    new_commitments: &[impl Commitment; 4],
    root_key: Fr254,
    ephemeral_key: BJJScalar,
    secret_preimages: &[impl SecretHash; 4],
    // === SWAP PARAMS ===
    party_a_public_key: TEAffine<BabyJubJub>,
    party_b_public_key: TEAffine<BabyJubJub>,
    nf_token_a_id: Fr254,
    value_a: Fr254,
    nf_token_b_id: Fr254,
    value_b: Fr254,
    swap_nonce: Fr254,
    deadline: Fr254,
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

    debug!("{id} Finding membership proofs for spend commitments");
    let (membership_proofs, root) =
        collect_membership_proofs_with_stable_root(spend_commitments, id).await?;
    let fixed_proofs: [MembershipProof<Fr254>; 4] = membership_proofs
        .try_into()
        .map_err(|_| "Could not convert membership proofs to fixed length array")?;

    let nf_address = get_addresses().nightfall();
    let (mut public_inputs, mut private_inputs) = (
        PublicInputs::new()
            .fee(new_commitments[2].get_value())
            .root(root)
            .deadline(deadline)
            .build(),
        PrivateInputs::new()
            .nf_address(nf_address)
            .party_a_public_key(party_a_public_key)
            .party_b_public_key(party_b_public_key)
            .value_a(value_a)
            .nf_token_a_id(nf_token_a_id)
            .nf_token_b_id(nf_token_b_id)
            .value_b(value_b)
            .nf_slot_id(nf_slot_id)
            .swap_nonce(swap_nonce)
            .deadline(deadline)
            .fee_token_id(get_fee_token_id())
            .nullifiers_values(&spend_commitments.map(|c| c.get_value()))
            .nullifiers_salts(&spend_commitments.map(|c| c.get_salt()))
            .commitments_values(&[
                new_commitments[1].get_value(),
                new_commitments[3].get_value(),
            ])
            .sender_commitment_salts(&[
                new_commitments[1].get_salt(),
                new_commitments[2].get_salt(),
                new_commitments[3].get_salt(),
            ])
            .public_keys(&public_keys)
            .root_key(root_key)
            .ephemeral_key(ephemeral_key)
            .membership_proofs(&fixed_proofs)
            .withdraw_address(Fr254::zero()) // No withdraw in swap
            .secret_preimages(&[
                secret_preimages[0].to_array(),
                secret_preimages[1].to_array(),
                secret_preimages[2].to_array(),
                secret_preimages[3].to_array(),
            ])
            .build(),
    );

    info!("{id} Generating swap proof");
    let wrapped_proof: Result<P, E::Error> = E::prove(&mut private_inputs, &mut public_inputs);
    debug!("{id} Creating swap client transaction");

    match wrapped_proof {
        Ok(proof) => Ok(ClientTransaction {
            fee: public_inputs.fee,
            historic_commitment_root: public_inputs.root,
            commitments: public_inputs.commitments,
            nullifiers: public_inputs.nullifiers,
            compressed_secrets: CompressedSecrets {
                cipher_text: public_inputs.compressed_secrets,
            },
            swap_link: public_inputs.swap_link,
            deadline: public_inputs.deadline,
            swap_side: public_inputs.swap_side,
            proof,
        }),
        Err(e) => {
            error!("{id} Swap proving error {e:?}");
            Err("Swap transaction could not be completed due to a proving error.")
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

#[cfg(test)]
mod tests {
    use ark_bn254::Fr as Fr254;
    use ark_crypto_primitives::sponge::{CryptographicSponge, FieldBasedCryptographicSponge};
    use ark_ec::twisted_edwards::Affine as TEAffine;
    use ark_ff::PrimeField;
    use ark_std::UniformRand;
    use ark_std::Zero;
    use jf_primitives::poseidon::{
        sponge::{PoseidonSponge, CRHF_RATE},
        PoseidonPerm,
    };
    use jf_utils::test_rng;
    use nf_curves::ed_on_bn254::BabyJubjub as BabyJubJub;

    #[test]
    fn test_swap_link_computation() {
        let rng = &mut test_rng();

        let party_a_pk = TEAffine::<BabyJubJub>::rand(rng);
        let party_b_pk = TEAffine::<BabyJubJub>::rand(rng);
        let token_a_id = Fr254::rand(rng);
        let token_b_id = Fr254::rand(rng);
        let value_a = Fr254::rand(rng);
        let value_b = Fr254::rand(rng);
        let swap_nonce = Fr254::from(u64::rand(rng));

        let sponge_perm = PoseidonPerm::<Fr254>::perm().unwrap();
        let mut sponge = PoseidonSponge::<Fr254, CRHF_RATE>::new(&sponge_perm);
        sponge.absorb(&vec![
            Fr254::from_le_bytes_mod_order(b"SWAP_V1"),
            party_a_pk.x,
            party_a_pk.y,
            party_b_pk.x,
            party_b_pk.y,
            token_a_id,
            value_a,
            token_b_id,
            value_b,
            swap_nonce,
        ]);
        let swap_link = sponge.squeeze_native_field_elements(1)[0];

        // Verify swap_link is non-zero (valid swap)
        assert!(!swap_link.is_zero(), "Swap link should be non-zero");

        // Verify determinism - same inputs = same output
        let sponge_perm_2 = PoseidonPerm::<Fr254>::perm().unwrap();
        let mut sponge_2 = PoseidonSponge::<Fr254, CRHF_RATE>::new(&sponge_perm_2);
        sponge_2.absorb(&vec![
            Fr254::from_le_bytes_mod_order(b"SWAP_V1"),
            party_a_pk.x,
            party_a_pk.y,
            party_b_pk.x,
            party_b_pk.y,
            token_a_id,
            value_a,
            token_b_id,
            value_b,
            swap_nonce,
        ]);
        let swap_link_2 = sponge_2.squeeze_native_field_elements(1)[0];

        assert_eq!(swap_link, swap_link_2, "Swap link should be deterministic");
    }
}
