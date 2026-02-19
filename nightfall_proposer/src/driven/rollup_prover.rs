//! This module contains the code for block proving. It builds a struct and implements the `RecursiveProver` trait from nightfish_CE and from the `ports` module.

use crate::{
    domain::entities::ClientTransactionWithMetaData,
    drivers::blockchain::block_assembly::BlockAssemblyError,
    get_deposit_proving_key,
    initialisation::get_db_connection,
    ports::{
        proving::RecursiveProvingEngine,
        trees::{CommitmentTree, HistoricRootTree, NullifierTree},
    },
};
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};

use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::cfg_iter;
use hex::FromHex;
use itertools::{izip, Itertools};
use jf_plonk::{
    errors::PlonkError,
    nightfall::{
        accumulation::accumulation_structs::AtomicInstance,
        ipa_structs::{ProvingKey, VerifyingKey},
        mle::mle_structs::MLEProvingKey,
        FFTPlonk,
    },
    proof_system::{
        structs::{ProvingKey as PlonkProvingKey, VerifyingKey as PlonkVerifyingKey},
        RecursiveOutput, UniversalRecursiveSNARK,
    },
    recursion::{
        circuits::{Kzg, Zmorph},
        RecursiveProof, RecursiveProver,
    },
    transcript::RescueTranscript,
};
use jf_primitives::{
    pcs::prelude::{expected_sha256_for_label, UnivariateKzgPCS},
    rescue::sponge::RescueCRHF,
};
use jf_relation::{errors::CircuitError, PlonkCircuit, Variable};
use log::{debug, warn};
use mongodb::{bson::doc, Client};

use lib::{rollup_circuit_checks::get_configuration_keys_path, utils::load_key_locally};
use lib::{
    deposit_circuit::deposit_circuit_builder,
    error::ConversionError,
    merkle_trees::trees::{MerkleTreeError, MutableTree, TreeMetadata},
    nf_client_proof::PublicInputs,
    plonk_prover::{get_client_proving_key, plonk_proof::PlonkProof},
    rollup_circuit_checks::{get_configuration_path, RollupKeyGenerator},
    serialization::{ark_de_hex, ark_se_hex},
    shared_entities::DepositData,
    utils::load_key_from_server,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    vec,
};

#[derive(Debug)]
pub enum RollupProofError {
    ConversionError(ConversionError),
    SerializationError(SerializationError),
    ProvingError(PlonkError),
    MerkleTreeError(MerkleTreeError<mongodb::error::Error>),
    ParameterError(String),
}

impl Error for RollupProofError {}
impl Display for RollupProofError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RollupProofError::ConversionError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::SerializationError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::ProvingError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::MerkleTreeError(e) => write!(f, "RollupProofError: {e}"),
            RollupProofError::ParameterError(s) => {
                write!(f, "RollupProofError: ParameterError: {s}")
            }
        }
    }
}

impl From<ConversionError> for RollupProofError {
    fn from(e: ConversionError) -> Self {
        RollupProofError::ConversionError(e)
    }
}

impl From<SerializationError> for RollupProofError {
    fn from(e: SerializationError) -> Self {
        RollupProofError::SerializationError(e)
    }
}

impl From<PlonkError> for RollupProofError {
    fn from(e: PlonkError) -> Self {
        RollupProofError::ProvingError(e)
    }
}

impl From<MerkleTreeError<mongodb::error::Error>> for RollupProofError {
    fn from(e: MerkleTreeError<mongodb::error::Error>) -> Self {
        RollupProofError::MerkleTreeError(e)
    }
}

impl From<RollupProofError> for BlockAssemblyError {
    fn from(e: RollupProofError) -> Self {
        BlockAssemblyError::ProvingError(e.to_string())
    }
}

/// Function to find an absolute path to a file.
fn find(path: &Path) -> Option<std::path::PathBuf> {
    if path.is_absolute() {
        match path.is_file() {
            true => return Some(path.to_path_buf()),
            false => return None,
        }
    }

    let cwd = std::env::current_dir().ok()?;
    let mut cwd = cwd.as_path();
    loop {
        let file_path = cwd.join(path);
        if file_path.is_file() {
            return Some(file_path);
        }

        cwd = cwd.parent()?;
    }
}

/// This function is used to retrieve the base grumpkin proving key.
pub fn get_base_grumpkin_proving_key() -> &'static Arc<MLEProvingKey<Zmorph>> {
    static PK: OnceLock<Arc<MLEProvingKey<Zmorph>>> = OnceLock::new();
    PK.get_or_init(|| {
        // We'll try to load key locally first, if it fails we will load from server.
        let path = Path::new("./configuration/bin/keys/base_grumpkin_pk");
        let source_file = find(path).unwrap();
        if let Some(key_bytes) = load_key_locally(&source_file) {
            let base_grumpkin_proving_key =
                MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(&*key_bytes)
                    .expect("Could not deserialise base_grumpkin_proving_key");
            return Arc::new(base_grumpkin_proving_key);
        }
        warn!("Could not load base_grumpkin_proving_key from local file. Loading from server");
        if let Some(key_bytes) = load_key_from_server("base_grumpkin_pk") {
            let base_grumpkin_proving_key =
                MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(&*key_bytes)
                    .expect("Could not deserialise base_grumpkin_proving_key");
            return Arc::new(base_grumpkin_proving_key);
        }
        // 3) If both fail, blow up loudly (this is critical infra)
        panic!("Failed to load base_grumpkin_proving_key from both local and server");
    })
}

/// This function is used to retrieve the base bn254 proving key.
pub fn get_base_bn254_proving_key() -> &'static Arc<ProvingKey<Kzg>> {
    static PK: OnceLock<Arc<ProvingKey<Kzg>>> = OnceLock::new();
    PK.get_or_init(|| {
        // 1) We'll try to load key locally first, if it fails we will load from server.
        let path = Path::new("./configuration/bin/keys/base_bn254_pk");
        let source_file = find(path).unwrap();
        if let Some(key_bytes) = load_key_locally(&source_file) {
            let base_bn254_proving_key =
                ProvingKey::<Kzg>::deserialize_compressed_unchecked(&*key_bytes)
                    .expect("Could not deserialise base_bn254_proving_key");
            return Arc::new(base_bn254_proving_key);
        }
        warn!("Could not load base_bn254_proving_key from local file. Loading from server");
        // 2) Try server
        if let Some(key_bytes) = load_key_from_server("base_bn254_pk") {
            let base_bn254_proving_key =
                ProvingKey::<Kzg>::deserialize_compressed_unchecked(&*key_bytes)
                    .expect("Could not deserialise base_bn254_proving_key");
            return Arc::new(base_bn254_proving_key);
        }
        // 3) If both fail, blow up loudly (this is critical infra)
        panic!("Failed to load base_bn254_proving_key from both local and server");
    })
}

/// This function is used to retrieve the decider proving key.
pub fn get_decider_proving_key() -> &'static Arc<PlonkProvingKey<Bn254>> {
    static PK: OnceLock<Arc<PlonkProvingKey<Bn254>>> = OnceLock::new();

    PK.get_or_init(|| {
        let path = Path::new("./configuration/bin/keys/decider_pk");
        let source_file = find(path).expect("Could not locate decider_pk file");

        // 1) We'll try to load key locally first, if it fails we will load from server.
        if let Some(bytes) = load_key_locally(&source_file) {
            if let Ok(pk) =
                PlonkProvingKey::<Bn254>::deserialize_compressed_unchecked(bytes.as_ref())
            {
                return Arc::new(pk);
            } else {
                warn!("Failed to deserialize local decider_pk, trying server");
            }
        } else {
            warn!("Could not read local decider_proving_key, trying server");
        }

        // 2) Try server
        if let Some(key_bytes) = load_key_from_server("decider_pk") {
            let pk = PlonkProvingKey::<Bn254>::deserialize_compressed_unchecked(key_bytes.as_ref())
                .expect("Could not deserialise decider_pk from server");
            return Arc::new(pk);
        }

        // 3) If both fail, blow up loudly (this is critical infra)
        panic!("Failed to load decider proving key from both local file and server");
    })
}

#[derive(Debug, Clone)]
/// The prover struct for the rollup prover. It contains the vk_hash_list and the key_store.
pub struct RollupProver;

impl RecursiveProver for RollupProver {
    // these checks are implementation of RecursiveProver in Nightfish and will be called by each corresponding circuit
    fn base_bn254_extra_checks(
        specific_pis: &[Variable],
        root_m_proof_length: usize,
        commitment_info_length: usize,
        nullifier_info_length: usize,
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::base_bn254_extra_checks(
            specific_pis,
            root_m_proof_length,
            commitment_info_length,
            nullifier_info_length,
            circuit,
        )
    }

    fn base_bn254_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::base_bn254_checks(specific_pis, circuit)
    }

    fn base_grumpkin_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fq254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::base_grumpkin_checks(specific_pis, circuit)
    }

    fn bn254_merge_circuit_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::bn254_merge_circuit_checks(specific_pis, circuit)
    }

    fn grumpkin_merge_circuit_checks(
        specific_pis: &[Vec<Variable>],
        circuit: &mut PlonkCircuit<Fq254>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::grumpkin_merge_circuit_checks(specific_pis, circuit)
    }

    fn decider_circuit_checks(
        specific_pis: &[Vec<Variable>],
        root_m_proof_length: usize,
        circuit: &mut PlonkCircuit<Fr254>,
        lookup_vars: &mut Vec<(Variable, Variable, Variable)>,
    ) -> Result<Vec<Variable>, CircuitError> {
        RollupKeyGenerator::decider_circuit_checks(
            specific_pis,
            root_m_proof_length,
            circuit,
            lookup_vars,
        )
    }

    fn get_vk_list() -> Vec<VerifyingKey<Kzg>> {
        RollupKeyGenerator::get_vk_list()
    }

    fn get_base_grumpkin_pk() -> MLEProvingKey<Zmorph> {
        get_base_grumpkin_proving_key().deref().clone()
    }

    fn get_base_bn254_pk() -> ProvingKey<Kzg> {
        get_base_bn254_proving_key().deref().clone()
    }

    fn get_merge_grumpkin_pks() -> Vec<MLEProvingKey<Zmorph>> {
        static GRUMPKIN_MERGE_PKS: OnceLock<Vec<Arc<MLEProvingKey<Zmorph>>>> = OnceLock::new();

        GRUMPKIN_MERGE_PKS
            .get_or_init(|| {
                let config_path = get_configuration_keys_path()
                    .expect("Configuration path not found");

                let mut pks = Vec::new();
                let mut i = 0;
                loop {
                    let filename = format!("merge_grumpkin_pk_{i}");
                    let path: PathBuf = config_path.join(&filename);
                    if let Some(source_file) = find(&path) {
                        let pk = MLEProvingKey::<Zmorph>::deserialize_compressed_unchecked(
                            &*std::fs::read(source_file).expect("Could not read MLE proving key"),
                        )
                        .expect("Could not deserialise MLE proving key");
                        pks.push(Arc::new(pk));
                        i += 1;
                    } else {
                        break;
                    }
                }
                pks
            })
            .iter()
            .map(|arc_pk| (**arc_pk).clone())
            .collect()
    }

    fn get_merge_bn254_pks() -> Vec<ProvingKey<Kzg>> {
        static BN254_MERGE_PKS: OnceLock<Vec<Arc<ProvingKey<Kzg>>>> = OnceLock::new();

        BN254_MERGE_PKS
            .get_or_init(|| {
                let config_path = get_configuration_keys_path()
                    .expect("Configuration path not found");

                let mut pks = Vec::new();
                let mut i = 0;
                loop {
                    let filename = format!("merge_bn254_pk_{i}");
                    let path: PathBuf = config_path.join(&filename);
                    if let Some(source_file) = find(&path) {
                        let pk = ProvingKey::<Kzg>::deserialize_compressed_unchecked(
                            &*std::fs::read(source_file).expect("Could not read proving key"),
                        )
                        .expect("Could not deserialise proving key");
                        pks.push(Arc::new(pk));
                        i += 1;
                    } else {
                        break;
                    }
                }
                pks
            })
            .iter()
            .map(|arc_pk| (**arc_pk).clone())
            .collect()
    }

    fn get_decider_pk() -> PlonkProvingKey<Bn254> {
        get_decider_proving_key().deref().clone()
    }

    fn get_decider_vk() -> PlonkVerifyingKey<Bn254> {
        let path = get_configuration_path()
            .expect("Configuration path not found")
            .join("bin/decider_vk");
        let source_file = find(&path).unwrap();
        PlonkVerifyingKey::<Bn254>::deserialize_compressed_unchecked(
            &*std::fs::read(source_file).expect("Could not read verifying key"),
        )
        .expect("Could not deserialise verifying key")
    }

    fn store_base_grumpkin_pk(pk: MLEProvingKey<Zmorph>) -> Option<()> {
        RollupKeyGenerator::store_base_grumpkin_pk(pk)
    }

    fn store_base_bn254_pk(pk: ProvingKey<Kzg>) -> Option<()> {
        RollupKeyGenerator::store_base_bn254_pk(pk)
    }

    fn store_merge_grumpkin_pks(pks: Vec<MLEProvingKey<Zmorph>>) -> Option<()> {
        RollupKeyGenerator::store_merge_grumpkin_pks(pks)
    }

    fn store_merge_bn254_pks(pks: Vec<ProvingKey<Kzg>>) -> Option<()> {
        RollupKeyGenerator::store_merge_bn254_pks(pks)
    }

    fn store_decider_pk(pk: PlonkProvingKey<Bn254>) -> Option<()> {
        RollupKeyGenerator::store_decider_pk(pk)
    }

    fn store_decider_vk(vk: &PlonkVerifyingKey<Bn254>) {
        RollupKeyGenerator::store_decider_vk(vk)
    }

    fn generate_vk_check_constraint(
        check_hash: Fr254,
        vk_hashes: &[Fr254],
        circuit: &mut PlonkCircuit<Fr254>,
    ) -> Result<(), CircuitError> {
        RollupKeyGenerator::generate_vk_check_constraint(check_hash, vk_hashes, circuit)
    }
}

/// This struct is used for the recursive proving of the rollup prover.
/// It is the result of running the `prepare_state_transition` function.
///
///
#[derive(Debug)]
pub struct RollupPreppedInfo {
    pub outputs_and_circuit_type: Vec<(Bn254Output, VerifyingKey<Kzg>)>,
    pub specific_pi: Vec<Vec<Fr254>>,
    pub extra_info: Vec<Vec<Fr254>>,
    pub extra_decider_info: Vec<Fr254>,
    pub srs_digest: [u8; 32],
    pub rollup_size: u32,
}

/// The output of the rollup prover, it contains the UltraPlonk proof and the KZG accumulators.
#[derive(Debug, Clone, Serialize, Deserialize, CanonicalSerialize, CanonicalDeserialize)]
pub struct RollupProof {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub fee_sum: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub accumulator_one: [Fq254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub accumulator_two: [Fq254; 4],
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub proof: Vec<Fq254>,
}

impl From<RecursiveProof> for RollupProof {
    fn from(proof: RecursiveProof) -> Self {
        let RecursiveProof {
            proof,
            accumulators,
            pi,
        } = proof;
        let [AtomicInstance {
            comm: comm_1,
            opening_proof: op_1,
            ..
        }, AtomicInstance {
            comm: comm_2,
            opening_proof: op_2,
            ..
        }] = accumulators;

        RollupProof {
            fee_sum: pi[0],
            accumulator_one: [comm_1.x, comm_1.y, op_1.proof.x, op_1.proof.y],
            accumulator_two: [comm_2.x, comm_2.y, op_2.proof.x, op_2.proof.y],
            proof: Vec::<Fq254>::from(proof),
        }
    }
}

impl From<RollupProof> for Vec<Fq254> {
    fn from(proof: RollupProof) -> Self {
        let RollupProof {
            fee_sum,
            accumulator_one,
            accumulator_two,
            proof,
        } = proof;
        let mut vec = vec![Fq254::from_le_bytes_mod_order(
            &fee_sum.into_bigint().to_bytes_le(),
        )];
        vec.extend_from_slice(&accumulator_one);
        vec.extend_from_slice(&accumulator_two);
        vec.extend_from_slice(&proof);
        vec
    }
}

pub(crate) type Bn254Output = RecursiveOutput<Kzg, FFTPlonk<Kzg>, RescueTranscript<Fr254>>;

impl RecursiveProvingEngine<PlonkProof> for RollupProver {
    type PreppedInfo = RollupPreppedInfo;

    type Error = RollupProofError;

    type RecursiveProof = RollupProof;

    async fn prepare_state_transition(
        deposit_transactions: &[(PlonkProof, PublicInputs)],
        transactions: &[ClientTransactionWithMetaData<PlonkProof>],
    ) -> Result<(Self::PreppedInfo, [Fr254; 3]), Self::Error> {
        // We retrieve both types of proving keys
        let deposit_pk = get_deposit_proving_key();
        let client_pk = get_client_proving_key();

        // First lets get all the public inputs from the deposit transactions and the client transactions
        // RecursiveOutput {
        //     proof,
        //     pi_hash,
        //     transcript,
        // }
        // get <outputs_and_circuit_type({proofs, pi_hashes, transcriptes},vks), public inputs> from the deposit transactions and client transactions
        let (outputs_and_circuit_type, public_inputs): (
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

        let n_total_transactions = 64u32;
        const MAX_KZG_DEGREE: usize = 26;
        let srs_digest_hex = expected_sha256_for_label(format!("{MAX_KZG_DEGREE}").as_str())
            .ok_or(PlonkError::InvalidParameters(
                "Failed to generate SHA256 label".to_string(),
            ))?;
        let srs_digest = <[u8; 32]>::from_hex(srs_digest_hex)
            .map_err(|e| PlonkError::InvalidParameters(format!("Hex conversion error: {e}")))?;

        // Get all the commitments and nullifiers from the public inputs
        // Flattens all commitments and nullifiers from the public inputs into vectors.
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
        debug!("Inserting commitments");
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

        // Historic Root Membership Proof
        let historic_root_proof = <Client as HistoricRootTree<Fr254>>::get_membership_proof(
            db,
            None,
            Some(metadata.sub_tree_count),
        )
        .await?;
        let root_proof_len_field = Fr254::from(root_m_proof_len as u64);

        //Zips together chunks of public inputs, membership proofs, and circuit info to build the extra info needed for the recursive circuits.
        // Structure of extra_info: Vec<Vec<Fr254>>
        // The outer Vec: Each element represents a chunk (typically a group of 4 transactions, matching the recursion tree's arity).
        // The inner Vec<Fr254>: This is not just 4 elements!
        // It is a flattened, concatenated vector containing all the auxiliary data needed for that chunk of transactions.
        // This includes:
        // Length fields (for parsing)
        // Roots for the transactions in the chunk
        // Membership proofs for those roots
        // Commitment insertion info
        // Nullifier insertion info

        let extra_info = izip!(
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

        // Format Public Inputs for Circuits
        // Converts each public input into a vector of field elements for use in the circuits.
        let specific_pi = public_inputs
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<Fr254>>>();
        // Prepares the extra info for the decider circuit, including the historic_root_proof and old_historic_root.
        let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
        let historic_root_proof_length = Fr254::from(extra_info_vec.len() as u64);
        extra_info_vec.insert(0, historic_root_proof_length);
        extra_info_vec.push(old_historic_root);
        // 1. RollupPreppedInfo
        // This struct contains all the data needed to generate the recursive rollup proof for a block. It includes:

        // outputs_and_circuit_type:
        // A vector of tuples, each containing:

        // The recursive proof output for a transaction (used in recursive aggregation).
        // The verifying key for the circuit that produced the proof. This is used to verify and aggregate all the individual transaction proofs into a single block proof.
        // specific_pi:
        // A vector of vectors, where each inner vector contains the public inputs for a transaction, formatted as field elements.
        // These are the public data (like commitments, nullifiers, roots, etc.) that the circuits use as inputs.

        // extra_info:
        // A vector of vectors, each containing additional circuit-specific data needed for the recursive circuits.
        // This typically includes membership proofs, insertion info, and other auxiliary data required by the circuits to validate the state transitions.

        // extra_decider_info:
        // A vector containing extra data for the final "decider" circuit, such as the historic root membership proof and the old root.
        // This is used in the final aggregation step to ensure the block’s state transition is valid.
        Ok((
            RollupPreppedInfo {
                outputs_and_circuit_type,
                specific_pi,
                extra_info,
                extra_decider_info: extra_info_vec,
                srs_digest,
                rollup_size: n_total_transactions,
            },
            [new_commitment_root, nullifier_root, updated_historic_root],
        ))
    }

    fn recursive_prove(info: Self::PreppedInfo) -> Result<Self::RecursiveProof, Self::Error> {
        Ok(RollupProof::from(
            <RollupProver as RecursiveProver>::prove(
                &info.outputs_and_circuit_type,
                &info.specific_pi,
                &info.extra_decider_info,
                &info.extra_info,
                &info.srs_digest,
                info.rollup_size,
            )
            .map_err(RollupProofError::from)?,
        ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::Zero;
    use jf_plonk::{nightfall::mle::MLEPlonk, proof_system::UniversalSNARK};
    use jf_primitives::{
        poseidon::Poseidon,
        trees::{
            imt::{IndexedMerkleTree, LeafDBEntry},
            timber::Timber,
            MembershipProof,
        },
    };
    use std::collections::HashMap;
    #[test]
    #[ignore = "Very long test"]
    fn test_preprocess() {
        let mut rng = ark_std::rand::thread_rng();

        let ipa_srs = MLEPlonk::<Zmorph>::universal_setup_for_testing(1 << 18, &mut rng).unwrap();
        let kzg_srs =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(1 << 25, &mut rng)
                .unwrap();

        let mut d_proofs = Vec::new();
        let mut public_input_vec = Vec::new();
        let mut public_inputs = PublicInputs::new();
        let deposit_array = [DepositData::default(); 4];

        let mut circuit = deposit_circuit_builder(&deposit_array, &mut public_inputs).unwrap();
        circuit
            .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
            .unwrap();
        let deposit_pk = get_deposit_proving_key();

        let output =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<_, _, RescueTranscript<Fr254>>(
                &mut ark_std::rand::thread_rng(),
                &circuit,
                deposit_pk,
                None,
                true,
            )
            .unwrap();

        (0..64).for_each(|_| {
            d_proofs.push((output.clone(), deposit_pk.vk.clone()));
            public_input_vec.push(public_inputs);
        });
        // We need to make dummy trees for to build circuit insertion info.
        let poseidon = Poseidon::<Fr254>::new();
        let mut timber: Timber<Fr254, Poseidon<Fr254>> =
            Timber::<Fr254, Poseidon<Fr254>>::new(poseidon, 32);
        let mut imt: IndexedMerkleTree<Fr254, Poseidon<Fr254>, _> =
            IndexedMerkleTree::<Fr254, Poseidon<Fr254>, HashMap<Fr254, LeafDBEntry<Fr254>>>::new(
                poseidon, 32,
            )
            .unwrap();
        let mut historic_root_tree: Timber<Fr254, Poseidon<Fr254>> =
            Timber::<Fr254, Poseidon<Fr254>>::new(poseidon, 32);

        // Get all the commitments and nullifiers from the public inputs
        let new_commitments = public_input_vec
            .iter()
            .flat_map(|pi| pi.commitments)
            .collect::<Vec<Fr254>>();

        let insert_nullifiers = public_input_vec
            .iter()
            .flat_map(|pi| pi.nullifiers)
            .collect::<Vec<Fr254>>();

        historic_root_tree.insert_leaf(Fr254::zero()).unwrap();

        let commitment_circuit_info = timber.batch_insert_for_circuit(&new_commitments).unwrap();

        let nullifier_circuit_info = imt.batch_insert_for_circuit(&insert_nullifiers).unwrap();

        let path = historic_root_tree
            .get_sibling_path(Fr254::zero(), 0)
            .unwrap();

        let m_proof = MembershipProof::<Fr254> {
            node_value: Fr254::zero(),
            sibling_path: path,
            leaf_index: 0,
        };

        let mut m_proof_vec = Vec::<Fr254>::from(m_proof);
        let root_proof_len_field = Fr254::from(m_proof_vec.len() as u64);
        m_proof_vec.push(public_inputs.roots[0]);
        let root_m_proofs_inner = vec![m_proof_vec.clone(); 4].concat();
        let root_membership_proofs = vec![root_m_proofs_inner.clone(); 64];

        let extra_base_info = izip!(
            public_input_vec.chunks(4),
            root_membership_proofs.chunks(4),
            commitment_circuit_info.chunks(2),
            nullifier_circuit_info.chunks(2)
        )
        .map(
            |(pis, root_m_proof_chunk, commitment_info, nullifier_info)| {
                let commitment_info_vec_0 = Vec::<Fr254>::from(commitment_info[0].clone());
                let commitment_info_vec_1 = Vec::<Fr254>::from(commitment_info[1].clone());
                let nullifier_info_vec_0: Vec<Fr254> = nullifier_info[0].clone().into();
                let nullifier_info_vec_1: Vec<Fr254> = nullifier_info[1].clone().into();
                let commitment_info_len = Fr254::from(commitment_info_vec_0.len() as u64);
                let nullifier_info_len = Fr254::from(nullifier_info_vec_0.len() as u64);
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
                    nullifier_info_vec_0,
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
                    nullifier_info_vec_1,
                ]
                .concat()
            },
        )
        .collect::<Vec<Vec<Fr254>>>();

        let specific_pi = public_input_vec
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<Fr254>>>();

        let new_commitment_root = timber.root;

        let old_historic_root = historic_root_tree.root;

        historic_root_tree.insert_leaf(new_commitment_root).unwrap();

        let historic_root_path = historic_root_tree
            .get_sibling_path(new_commitment_root, 1)
            .unwrap();

        let historic_root_proof = MembershipProof::<Fr254> {
            node_value: new_commitment_root,
            sibling_path: historic_root_path,
            leaf_index: 1,
        };

        let mut extra_info_vec: Vec<Fr254> = historic_root_proof.into();
        extra_info_vec.insert(0, root_proof_len_field);
        extra_info_vec.push(old_historic_root);
        let rollup_size = 64;
        let srs_digest = [0u8; 32];

        RollupProver::preprocess(
            &d_proofs,
            &specific_pi,
            &extra_base_info,
            &extra_info_vec,
            &ipa_srs,
            &kzg_srs,
            &srs_digest,
            rollup_size,
        )
        .unwrap();
    }
}
