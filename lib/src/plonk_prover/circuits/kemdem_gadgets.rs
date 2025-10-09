use ark_bn254::Fr as Fr254;
use ark_ec::{twisted_edwards::Affine as TEAffine, AffineRepr};
use nf_curves::ed_on_bn254::BabyJubjub;

use jf_relation::{
    errors::CircuitError,
    gadgets::ecc::{Point, PointVariable},
    Circuit, PlonkCircuit, Variable,
};

use super::{DOMAIN_DEM, DOMAIN_KEM};
use jf_primitives::circuit::poseidon::PoseidonHashGadget;

pub trait KEMDEMCircuit {
    fn kem(&mut self, shared_secret: &PointVariable) -> Result<Variable, CircuitError>;

    fn dem(
        &mut self,
        encryption_key_var: Variable,
        plain_text: &[Variable],
    ) -> Result<Vec<Variable>, CircuitError>;

    fn kemdem(
        &mut self,
        ephemeral_key: Variable,
        shared_secret: &PointVariable,
        plain_text: &[Variable],
    ) -> Result<(PointVariable, Vec<Variable>), CircuitError>;
}

impl KEMDEMCircuit for PlonkCircuit<Fr254> {
    fn kem(&mut self, shared_secret: &PointVariable) -> Result<Variable, CircuitError> {
        let domain_kem_var = self.create_variable(DOMAIN_KEM)?;

        // Compute the encryption key and store the variable index.
        let encryption_key_var =
            self.poseidon_hash(&[shared_secret.get_x(), shared_secret.get_y(), domain_kem_var])?;

        Ok(encryption_key_var)
    }

    fn dem(
        &mut self,
        encryption_key_var: Variable,
        plain_text: &[Variable],
    ) -> Result<Vec<Variable>, CircuitError> {
        let domain_dem_var = self.create_variable(DOMAIN_DEM)?;

        // Create variables for all the cipher texts, store the indices in the array.
        plain_text
            .iter()
            .enumerate()
            .map(|(i, &plain)| {
                let tmp_var = self.create_variable(Fr254::from(i as u64))?;
                let tmp_hash = self
                    .poseidon_hash(vec![encryption_key_var, domain_dem_var, tmp_var].as_slice())?;
                self.add(tmp_hash, plain)
            })
            .collect::<Result<Vec<Variable>, CircuitError>>()
    }

    fn kemdem(
        &mut self,
        ephemeral_key: Variable,
        shared_secret: &PointVariable,
        plain_text: &[Variable],
    ) -> Result<(PointVariable, Vec<Variable>), CircuitError> {
        // Now we run the KEM.
        let encryption_key_var = self.kem(shared_secret)?;

        // Now we run the DEM.
        let cipher_text = self.dem(encryption_key_var, plain_text)?;

        let pub_point = self
            .create_constant_point_variable(&Point::<Fr254>::from(
                TEAffine::<BabyJubjub>::generator(),
            ))?;
        let epk = self.variable_base_scalar_mul::<BabyJubjub>(ephemeral_key, &pub_point)?;

        Ok((epk, cipher_text))
    }
}
