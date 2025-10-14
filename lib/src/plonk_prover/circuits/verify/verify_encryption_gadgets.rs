use crate::plonk_prover::circuits::kemdem_gadgets::KEMDEMCircuit;
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
                "Calculated cipher text has length not equal to 5".to_string(),
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
