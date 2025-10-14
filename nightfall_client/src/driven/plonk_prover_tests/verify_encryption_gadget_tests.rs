#[cfg(test)]
mod tests {
    use crate::driven::primitives::kemdem_functions::kemdem_encrypt;
    use ark_ec::{twisted_edwards::Affine as TEAffine, AffineRepr, CurveGroup};
    use ark_std::UniformRand;
    use jf_relation::{gadgets::ecc::Point, Circuit, PlonkCircuit};
    use jf_utils::{fr_to_fq, test_rng};
    use lib::{
        nf_token_id::to_nf_token_id_from_fr254,
        plonk_prover::circuits::verify::verify_encryption_gadgets::VerifyEncryptionCircuit,
    };
    use nf_curves::ed_on_bn254::{BabyJubjub as EdwardsConfig, Fq as Fr254, Fr as BJJScalar};
    #[test]
    fn test_encryption_circuit() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<EdwardsConfig>::generator();
        let recipient_public_key = (public_point * recipient_private_key).into();

        let token_id_fr = Fr254::rand(rng);
        let erc_address_fr = Fr254::rand(rng);

        let nf_token_id_fr = to_nf_token_id_from_fr254(erc_address_fr, token_id_fr);

        let plain_text = vec![nf_token_id_fr, Fr254::rand(rng), Fr254::rand(rng)];
        let cipher_text = kemdem_encrypt::<false>(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();

        let mut circuit = PlonkCircuit::<Fr254>::new_ultra_plonk(8);
        let nf_token_id = circuit.create_variable(plain_text[0]).unwrap();
        let nf_slot_id = circuit.create_variable(plain_text[1]).unwrap();
        let value = circuit.create_variable(plain_text[2]).unwrap();

        let shared_secret = (recipient_public_key * ephemeral_private_key).into_affine();

        let shared_secret = circuit
            .create_point_variable(&Point::<Fr254>::from(shared_secret))
            .unwrap();

        let e_private_key = circuit
            .create_variable(fr_to_fq::<Fr254, EdwardsConfig>(&ephemeral_private_key))
            .unwrap();

        let withdraw_address = circuit.zero();
        let withdraw_flag = circuit.create_boolean_variable(false).unwrap();

        let circuit_cipher_text = circuit
            .verify_encryption(
                nf_token_id,
                nf_slot_id,
                value,
                &shared_secret,
                e_private_key,
                withdraw_address,
                withdraw_flag,
            )
            .unwrap();

        for (calc, circuit_cipher) in cipher_text.iter().zip(circuit_cipher_text.iter()) {
            assert_eq!(*calc, circuit.witness(*circuit_cipher).unwrap());
        }

        circuit.check_circuit_satisfiability(&[]).unwrap();
    }
}
