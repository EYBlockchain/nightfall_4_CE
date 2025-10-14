use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use jf_plonk::{
    errors::PlonkError,
    nightfall::{
        ipa_structs::{Proof as JFProof, ProvingKey, VerificationKeyId, VerifyingKey, VK},
        reproduce_transcript, FFTPlonk,
    },
    proof_system::{RecursiveOutput, UniversalRecursiveSNARK},
    transcript::RescueTranscript,
};

use crate::{
    nf_client_proof::{PrivateInputs, Proof, ProvingEngine, PublicInputs},
    plonk_prover::{circuit_builder::CircuitBuilder, get_client_proving_key},
    serialization::{ark_de_hex, ark_se_hex},
};
use alloy::primitives::Bytes;
use jf_primitives::{pcs::prelude::UnivariateKzgPCS, rescue::sponge::RescueCRHF};
use jf_relation::{Circuit, PlonkCircuit};
use log::{debug, error};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
#[derive(
    Default, Debug, Serialize, Deserialize, CanonicalSerialize, CanonicalDeserialize, Clone,
)]
pub struct PlonkProof {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    proof: JFProof<UnivariateKzgPCS<Bn254>>,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pi_hash: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    vk_id: Option<VerificationKeyId>,
}

#[derive(Debug)]
pub struct PlonkProvingEngine;

impl Proof for PlonkProof {
    fn compress_proof(&self) -> Result<Bytes, SerializationError> {
        let bytes_vec = bincode::serialize(self).map_err(|_| SerializationError::InvalidData)?;
        Ok(Bytes::from_iter(bytes_vec))
    }

    fn from_compressed(compressed: Bytes) -> Result<Self, SerializationError>
    where
        Self: Sized,
    {
        let proof = bincode::deserialize::<PlonkProof>(&compressed)
            .map_err(|_| SerializationError::InvalidData)?;

        Ok(proof)
    }
}

impl ProvingEngine<PlonkProof> for PlonkProvingEngine {
    type Error = PlonkError;

    fn prove(
        private_inputs: &mut PrivateInputs,
        public_inputs: &mut PublicInputs,
    ) -> Result<PlonkProof, Self::Error> {
        let mut rng = ark_std::rand::thread_rng();
        let mut circuit = PlonkCircuit::<Fr254>::build_circuit(public_inputs, private_inputs)?;
        // add an extra check for circuit satisfiability. It's more compute but it gives better information in case of failure
        circuit.finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()?;
        let pi = circuit.public_input()?;
        circuit.check_circuit_satisfiability(&pi)?;
        debug!("Retrieving proving and verifying keys");
        let pk: &'static Arc<ProvingKey<UnivariateKzgPCS<Bn254>>> = get_client_proving_key();
        let output =
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<_, _, RescueTranscript<Fr254>>(
                &mut rng, &circuit, pk, None,
            )
            .map_err(|e| {
                error!("Error generating proof: {e:?}");
                e
            })?;
        debug!("Plonk proof generated");
        Ok(PlonkProof::from_recursive_output(output, &pk.vk))
    }

    fn verify(proof: &PlonkProof, public_inputs: &PublicInputs) -> Result<bool, Self::Error> {
        let input = public_inputs
            .iter()
            .map(|msg| Fq254::from_le_bytes_mod_order(&msg.into_bigint().to_bytes_le()))
            .collect::<Vec<Fq254>>();

        let output = RescueCRHF::<Fq254>::sponge_with_bit_padding(&input, 1)[0];

        let hash = Fr254::from_le_bytes_mod_order(&output.into_bigint().to_bytes_le()[..31]);

        if hash != proof.pi_hash {
            return Err(PlonkError::PublicInputsDoNotMatch);
        }
        let vk = &get_client_proving_key().vk;

        let output =
            RecursiveOutput::<UnivariateKzgPCS<Bn254>, _, RescueTranscript<Fr254>>::try_from(
                proof.clone(),
            )?;

        Ok(
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify_recursive_proof::<RescueTranscript<Fr254>>(
                vk, &output, None,
            )
            .is_ok(),
        )
    }
}

impl PlonkProof {
    /// Creates a new [`PlonkProof`]
    pub fn new(
        proof: JFProof<UnivariateKzgPCS<Bn254>>,
        pi_hash: Fr254,
        vk_id: Option<VerificationKeyId>,
    ) -> Self {
        Self {
            proof,
            pi_hash,
            vk_id,
        }
    }

    /// Creates a [`PlonkProof`] from a [`RecursiveOutput`] and its corresponding verification key.
    pub fn from_recursive_output(
        output: RecursiveOutput<
            UnivariateKzgPCS<Bn254>,
            FFTPlonk<UnivariateKzgPCS<Bn254>>,
            RescueTranscript<Fr254>,
        >,
        vk: &VerifyingKey<UnivariateKzgPCS<Bn254>>,
    ) -> Self {
        let RecursiveOutput { proof, pi_hash, .. } = output;
        let vk_id = vk.id();
        Self {
            proof,
            pi_hash,
            vk_id,
        }
    }
}

impl TryFrom<PlonkProof>
    for RecursiveOutput<
        UnivariateKzgPCS<Bn254>,
        FFTPlonk<UnivariateKzgPCS<Bn254>>,
        RescueTranscript<Fr254>,
    >
{
    type Error = PlonkError;

    fn try_from(client_proof: PlonkProof) -> Result<Self, Self::Error> {
        let PlonkProof {
            proof,
            pi_hash,
            vk_id,
        } = client_proof;

        let transcript =
            reproduce_transcript::<UnivariateKzgPCS<Bn254>, _, Fq254, RescueTranscript<Fr254>>(
                vk_id, pi_hash, &proof,
            )?;
        Ok(RecursiveOutput {
            proof,
            pi_hash,
            transcript,
        })
    }
}
