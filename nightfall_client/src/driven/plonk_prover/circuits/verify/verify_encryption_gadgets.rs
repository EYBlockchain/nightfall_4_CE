use crate::driven::plonk_prover::circuits::kemdem_gadgets::KEMDEMCircuit;
use jf_relation::{
    errors::CircuitError, gadgets::ecc::PointVariable, BoolVar, Circuit, PlonkCircuit, Variable,
};
use nf_curves::ed_on_bn254::Fq as Fr254;

pub trait VerifyEncryptionCircuit {
    #[allow(clippy::too_many_arguments)]
    fn verify_encryption(
        &mut self,
        nf_token_id: Variable,
        nf_slot_id: Variable,
        new_commitments_value: Variable,
        shared_secret: &PointVariable,
        ephemeral_key: Variable,
        withdraw_address: Variable,
        withdraw_flag: BoolVar,
    ) -> Result<Vec<Variable>, CircuitError>;
}
impl VerifyEncryptionCircuit for PlonkCircuit<Fr254> {
    fn verify_encryption(
        &mut self,
        nf_token_id: Variable,
        nf_slot_id: Variable,
        new_commitments_value: Variable,
        shared_secret: &PointVariable,
        ephemeral_key: Variable,
        withdraw_address: Variable,
        withdraw_flag: BoolVar,
    ) -> Result<Vec<Variable>, CircuitError> {

        let (epk, mut cipher_text_kem_dem) = self.kemdem(
            ephemeral_key,
            shared_secret,
            &[nf_token_id, nf_slot_id, new_commitments_value],
        )?;

        let x = epk.get_x();

        let neg_x = self.sub(self.zero(), x)?;

        let flag = self.is_lt(neg_x, x)?;

        cipher_text_kem_dem.push(epk.get_y());
        cipher_text_kem_dem.push(flag.0);

        if cipher_text_kem_dem.len() != 5 {
            return Err(CircuitError::ParameterError(
                "Calculated cipher text has length not equal to 4".to_string(),
            ));
        }

        let withdraw_cipher_text = [
            nf_token_id,
            withdraw_address,
            new_commitments_value,
            self.zero(),
            self.zero(),
        ];
        // Output the one of the two calculated cipher texts based on the withdraw flag
        cipher_text_kem_dem
            .into_iter()
            .zip(withdraw_cipher_text)
            .map(|(cipher, withdraw)| self.conditional_select(withdraw_flag, cipher, withdraw))
            .collect::<Result<Vec<Variable>, CircuitError>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        driven::primitives::kemdem_functions::kemdem_encrypt,
        drivers::rest::utils::to_nf_token_id_from_fr254,
    };
    use ark_ec::{twisted_edwards::Affine as TEAffine, AffineRepr, CurveGroup};
    use ark_std::UniformRand;
    use jf_relation::gadgets::ecc::Point;
    use jf_utils::{fr_to_fq, test_rng};
    use nf_curves::ed_on_bn254::{BabyJubjub as EdwardsConfig, Fr as BJJScalar};
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
