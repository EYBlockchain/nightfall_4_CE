use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use jf_plonk::{nightfall::FFTPlonk, proof_system::UniversalRecursiveSNARK, transcript::RescueTranscript};
use jf_primitives::{pcs::prelude::UnivariateKzgPCS, rescue::sponge::RescueCRHF};
use lib::{
    error::UnifiedProofError,
    nf_client_proof::{PrivateInputs, PublicInputs},
    plonk_prover::circuits::unified_circuit::unified_circuit_builder,
    plonk_prover::{get_client_proving_key, plonk_proof::PlonkProof},
    shared_entities::DepositData,
};

pub(crate) fn create_unified_deposit_proof(
    deposit_data: &[DepositData; 4],
    public_inputs: &mut PublicInputs,
) -> Result<PlonkProof, UnifiedProofError> {
    let mut private_inputs = PrivateInputs::for_deposit(deposit_data);
    *public_inputs = PublicInputs::for_deposit();
    let mut circuit = unified_circuit_builder(public_inputs, &mut private_inputs)
        .map_err(UnifiedProofError::from)?;
    circuit
        .finalize_for_recursive_arithmetization::<RescueCRHF<Fq254>>()
        .map_err(UnifiedProofError::from)?;
    let pk = get_client_proving_key();

    let output = FFTPlonk::<UnivariateKzgPCS<Bn254>>::recursive_prove::<
        _,
        _,
        RescueTranscript<Fr254>,
    >(&mut ark_std::rand::thread_rng(), &circuit, pk, None, true)
    .map_err(UnifiedProofError::from)?;
    Ok(PlonkProof::from_recursive_output(output, &pk.vk))
}
