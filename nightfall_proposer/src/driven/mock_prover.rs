//! Contains code for a mocked prover for use when working without a large machine to run a full proposer.

use ark_std::cfg_iter;
use itertools::{izip, Itertools};
use jf_plonk::{
    errors::PlonkError,
    nightfall::ipa_structs::VerifyingKey,
    nightfall::FFTPlonk,
    proof_system::{RecursiveOutput, UniversalRecursiveSNARK},
    recursion::circuits::Kzg,
    transcript::RescueTranscript,
};
use jf_primitives::{pcs::prelude::UnivariateKzgPCS, rescue::sponge::RescueCRHF};
use jf_utils::fr_to_fq;
use lib::{
    deposit_circuit::deposit_circuit_builder,
    shared_entities::DepositData,
    merkle_trees::trees::{MerkleTreeError, MutableTree, TreeMetadata},
    nf_client_proof::PublicInputs,
    plonk_prover::{get_client_proving_key, plonk_proof::PlonkProof},
};
use std::collections::HashMap;

use log::debug;
use mongodb::{bson::doc, Client};

use super::rollup_prover::RollupProofError;
use crate::{
    domain::entities::ClientTransactionWithMetaData,
    driven::rollup_prover::Bn254Output,
    get_deposit_proving_key,
    initialisation::get_db_connection,
    ports::proving::RecursiveProvingEngine,
    ports::trees::{CommitmentTree, HistoricRootTree, NullifierTree},
};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_std::Zero;
pub struct MockProver;

impl RecursiveProvingEngine<PlonkProof> for MockProver {
    type PreppedInfo = Fr254;
    type Error = RollupProofError;
    type RecursiveProof = Vec<Fq254>;

    async fn prepare_state_transition(
        deposit_transactions: &[(PlonkProof, PublicInputs)],
        transactions: &[ClientTransactionWithMetaData<PlonkProof>],
    ) -> Result<(Self::PreppedInfo, [Fr254; 3]), Self::Error> {
        // compute fee_sum for transfer and withdraw tx
        let fee_sum = transactions
            .iter()
            .fold(Fr254::zero(), |acc, tx| acc + tx.client_transaction.fee);
        // We retrieve both types of proving keys
        let deposit_pk = get_deposit_proving_key();
        let client_pk = get_client_proving_key();

        // First lets get all the public inputs from the deposit transactions and the client transactions
        let (_outputs_and_circuit_type, public_inputs): (
            Vec<(Bn254Output, VerifyingKey<Kzg>)>,
            Vec<PublicInputs>,
        ) = cfg_iter!(deposit_transactions)
            .map(|(proof, pi)| {
                let output = RecursiveOutput::try_from(proof.clone())?;
                Result::<_, PlonkError>::Ok((output, deposit_pk.vk.clone(), *pi))
            })
            .chain(cfg_iter!(transactions).map(|tx| {
                let output = RecursiveOutput::try_from(tx.client_transaction.proof.clone())?;
                Result::<_, PlonkError>::Ok((
                    output,
                    client_pk.vk.clone(),
                    PublicInputs::from(&tx.client_transaction),
                ))
            }))
            .collect::<Result<Vec<_>, PlonkError>>()?
            .into_iter()
            .map(|(output, vk, pi)| ((output, vk), pi))
            .unzip();

        // Get all the commitments and nullifiers from the public inputs
        let new_commitments = public_inputs
            .iter()
            .flat_map(|pi| pi.commitments)
            .collect::<Vec<Fr254>>();

        let insert_nullifiers = public_inputs
            .iter()
            .flat_map(|pi| pi.nullifiers)
            .collect::<Vec<Fr254>>();

        // work out what the new historic root would be if we were to add these new commitments
        let db = get_db_connection().await;

        // get the current historic root
        let current_historic_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await?;
        // Create the commitments circuit info
        let commitment_circuit_info =
            <Client as CommitmentTree<Fr254>>::batch_insert_with_circuit_info(db, &new_commitments)
                .await?;
        // Create the nullifier circuit info
        debug!("Inserting nullifiers");
        let nullifier_circuit_info =
            <Client as NullifierTree<Fr254>>::batch_insert_with_circuit_info(
                db,
                &insert_nullifiers,
            )
            .await?;
        // use the final commitment circuit info to get the new root of the commitment tree.
        let new_commitment_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as CommitmentTree<Fr254>>::TREE_NAME,
        )
        .await?;

        // We also need check each of the roots in the client proofs is valid so we construct the membership proofs for them here.
        let mut root_proofs = HashMap::<Fr254, Vec<Fr254>>::new();
        let mut root_membership_proofs = Vec::<Vec<Fr254>>::new();
        let mut root_m_proof_len = 0;
        for pi in public_inputs.iter() {
            let mut m_proofs = Vec::<Fr254>::new();
            for root in pi.roots.iter() {
                if !root_proofs.contains_key(root) {
                    let proof = <Client as HistoricRootTree<Fr254>>::get_membership_proof(
                        db,
                        Some(root),
                        None,
                    )
                    .await?;
                    let mut proof_vec = Vec::<Fr254>::from(proof);
                    root_m_proof_len = proof_vec.len();
                    proof_vec.push(current_historic_root);
                    root_proofs.insert(*root, proof_vec.clone());
                    m_proofs.extend(proof_vec.iter());
                } else {
                    let proof_vec =
                        root_proofs
                            .get(root)
                            .ok_or(RollupProofError::ParameterError(
                                "Error retrieving Historic root Membership proof from temporary DB"
                                    .to_string(),
                            ))?;
                    m_proofs.extend(proof_vec.iter());
                }
            }
            root_membership_proofs.push(m_proofs);
        }

        let nullifier_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as NullifierTree<Fr254>>::TREE_NAME,
        )
        .await?;

        // work out what the new historic root tree root would be if we were to add this new historic root
        let old_historic_root = <Client as MutableTree<Fr254>>::get_root(
            db,
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
        )
        .await?;

        let metadata_collection_name = format!(
            "{}_{}",
            <Client as HistoricRootTree<Fr254>>::TREE_NAME,
            "metadata"
        );
        let metadata_collection = db
            .database(<Client as MutableTree<Fr254>>::MUT_DB_NAME)
            .collection::<TreeMetadata<Fr254>>(&metadata_collection_name);
        let metadata: TreeMetadata<Fr254> = metadata_collection
            .find_one(doc! {"_id": 0})
            .await
            .map_err(MerkleTreeError::DatabaseError)?
            .ok_or(MerkleTreeError::TreeNotFound)?;
        let updated_historic_root =
            <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(
                db,
                &new_commitment_root,
                false,
            )
            .await?;

        let historic_root_proof = <Client as HistoricRootTree<Fr254>>::get_membership_proof(
            db,
            None,
            Some(metadata.sub_tree_count),
        )
        .await?;
        let root_proof_len_field = Fr254::from(root_m_proof_len as u64);

        let _extra_info = izip!(
            public_inputs.chunks(4),
            root_membership_proofs.chunks(4),
            commitment_circuit_info.chunks(2),
            nullifier_circuit_info.into_iter().chunks(2).into_iter()
        )
        .map(
            |(pis, root_m_proof_chunk, commitment_info, nullifier_info)| {
                let commitment_info_vec_0 = Vec::<Fr254>::from(commitment_info[0].clone());
                let commitment_info_vec_1 = Vec::<Fr254>::from(commitment_info[1].clone());
                let nullifier_info_vecs = nullifier_info
                    .into_iter()
                    .map(|info| info.into())
                    .collect::<Vec<Vec<Fr254>>>();
                let commitment_info_len = Fr254::from(commitment_info_vec_0.len() as u64);
                let nullifier_info_len = Fr254::from(nullifier_info_vecs[0].len() as u64);
                [
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[0].roots, pis[1].roots].concat(),
                    root_m_proof_chunk[0]
                        .iter()
                        .chain(root_m_proof_chunk[1].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_0,
                    nullifier_info_vecs[0].clone(),
                    vec![
                        root_proof_len_field,
                        commitment_info_len,
                        nullifier_info_len,
                    ],
                    [pis[2].roots, pis[3].roots].concat(),
                    root_m_proof_chunk[2]
                        .iter()
                        .chain(root_m_proof_chunk[3].iter())
                        .copied()
                        .collect(),
                    commitment_info_vec_1,
                    nullifier_info_vecs[1].clone(),
                ]
                .concat()
            },
        )
        .collect::<Vec<Vec<Fr254>>>();

        let _specific_pi = public_inputs
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<Fr254>>>();
        let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
        let historic_root_proof_length = Fr254::from(extra_info_vec.len() as u64);
        extra_info_vec.insert(0, historic_root_proof_length);
        extra_info_vec.push(old_historic_root);
        Ok((
            (fee_sum),
            [new_commitment_root, nullifier_root, updated_historic_root],
        ))
    }
    fn recursive_prove(info: Self::PreppedInfo) -> Result<Vec<Fq254>, Self::Error> {
        // Compute the real fee_sum
        let fee_sum = fr_to_fq::<ark_bn254::Fq, ark_bn254::g1::Config>(&info);
        // we need to make first element the real fee_sum, otherwise the assertion in verify_rollup_proof will fail.
        Ok(vec![fee_sum; 10]) // the mock proof must be this long to get through the verifier manipulation in Nightfall.sol
    }
    fn create_deposit_proof(
        deposit_data: &[DepositData; 4],
        public_inputs: &mut PublicInputs,
    ) -> Result<PlonkProof, Self::Error> {
        let mut circuit =
            deposit_circuit_builder(deposit_data, public_inputs).map_err(PlonkError::from)?;
        circuit
            .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
            .map_err(PlonkError::from)?;
        let pk = get_deposit_proving_key();

        let output = FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<
            _,
            _,
            RescueTranscript<Fr254>,
        >(&mut ark_std::rand::thread_rng(), &circuit, pk, None, true)?;
        Ok(PlonkProof::from_recursive_output(output, &pk.vk))
    }
}
