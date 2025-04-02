use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::{BigInteger, One, PrimeField, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use jf_primitives::poseidon::{FieldHasher, Poseidon, PoseidonError};
use nf_curves::ed_on_bn254::{BabyJubjub, Fq as Fr254, Fr as BJJScalar};

use super::*;

#[allow(dead_code)]
// Function for performing KEMDEM outside of a circuit.
// Accepts a borrow and returns a vector - easier to use with Solidity Rust bindings and Rust in general
pub fn kemdem_encrypt<const IS_WITHDRAW: bool>(
    ephemeral_private_key: BJJScalar,
    recipient_public_key: TEAffine<BabyJubjub>,
    plain_text: &[Fr254],
    public_point: TEAffine<BabyJubjub>,
) -> Result<Vec<Fr254>, PoseidonError> {
    if plain_text.len() != 3 {
        return Err(PoseidonError::InvalidInputs);
    }

    if IS_WITHDRAW {
        Ok([plain_text, &[Fr254::zero(), Fr254::zero()]].concat())
    } else {
        // First we do the KEM.
        // Compute the shared secret.
        let shared_secret: TEAffine<BabyJubjub> =
            (recipient_public_key * ephemeral_private_key).into();
        let poseidon = Poseidon::<Fr254>::new();

        // Now we calculate the ephemeral Public Key.
        let ephemeral_public_key: TEAffine<BabyJubjub> =
            (public_point * ephemeral_private_key).into();

        let mut bytes = Vec::<u8>::new();
        ephemeral_public_key
            .serialize_compressed(&mut bytes)
            .unwrap();
        // Compute the encryption key.
        let encryption_key = poseidon.hash(&[shared_secret.x, shared_secret.y, DOMAIN_KEM])?;

        // Now we do the DEM.
        let mut cipher_text = vec![];
        for (i, plain) in plain_text.iter().enumerate() {
            let tmp: Fr254 = poseidon.hash(&[encryption_key, DOMAIN_DEM, Fr254::from(i as u64)])?;
            cipher_text.push(tmp + *plain);
        }

        let x = ephemeral_public_key.x;

        let flag = if x > -x { Fr254::one() } else { Fr254::zero() };

        cipher_text.push(ephemeral_public_key.y);
        cipher_text.push(flag);
        Ok(cipher_text)
    }
}

// this is for use in circuits, which need fixed sized arrays. It consumes and returns
// fixed arrays.
pub fn kemdem_encrypt_fixed_array<const N: usize, const IS_WITHDRAW: bool>(
    ephemeral_private_key: BJJScalar,
    recipient_public_key: TEAffine<BabyJubjub>,
    plain_text: [Fr254; N],
    public_point: TEAffine<BabyJubjub>,
) -> Result<[Fr254; N], PoseidonError> {
    let cipher_text = kemdem_encrypt::<IS_WITHDRAW>(
        ephemeral_private_key,
        recipient_public_key,
        &plain_text,
        public_point,
    )?;
    let mut cipher_text_fixed = [Fr254::zero(); N];
    cipher_text_fixed[..N].copy_from_slice(&cipher_text[..N]);
    Ok(cipher_text_fixed)
}

pub fn kemdem_decrypt(
    recipient_private_key: BJJScalar,
    cipher_text: &[Fr254],
) -> Result<Vec<Fr254>, PoseidonError> {
    // First we decompress the epk
    let mut point_bytes = cipher_text[3].into_bigint().to_bytes_le();
    let flag = cipher_text[4].into_bigint().to_bytes_le()[0] << 7;
    // We can hard index into point_bytes because we know it is 32 bytes long.
    point_bytes[31] += flag;

    let epk = TEAffine::<BabyJubjub>::deserialize_compressed(&*point_bytes)
        .map_err(|_| PoseidonError::InvalidInputs)?;

    // compute the shared secret and thence a decryption key (= encryption key)
    let shared_secret: TEAffine<BabyJubjub> = (epk * recipient_private_key).into();
    let poseidon = Poseidon::<Fr254>::new();
    let decryption_key = poseidon.hash(&[shared_secret.x, shared_secret.y, DOMAIN_KEM])?;
    // now we can decrypt
    let mut plain_text = vec![];
    for (i, cipher) in cipher_text.iter().take(3).enumerate() {
        let tmp: Fr254 = poseidon.hash(&[decryption_key, DOMAIN_DEM, Fr254::from(i as u64)])?;
        plain_text.push(*cipher - tmp);
    }
    plain_text.push(shared_secret.y);
    Ok(plain_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::drivers::derive_key::{GENERATOR_X, GENERATOR_Y};
    use ark_ff::UniformRand;
    use ark_std::test_rng;
    use nf_curves::ed_on_bn254::{GENERATOR_X, GENERATOR_Y};

    #[test]
    fn test_shared_secret() {
        let rng = &mut test_rng();
        let recipient_private_key: BJJScalar = BJJScalar::rand(rng);
        let ephemeral_private_key: BJJScalar = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();
        let ephemeral_public_key: TEAffine<BabyJubjub> =
            (public_point * ephemeral_private_key).into();
        let shared_secret1: TEAffine<BabyJubjub> =
            (recipient_public_key * ephemeral_private_key).into();
        let shared_secret2: TEAffine<BabyJubjub> =
            (ephemeral_public_key * recipient_private_key).into();
        assert_eq!(shared_secret1, shared_secret2);
    }

    #[test]
    fn test_kemdem_transfer() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key = (public_point * recipient_private_key).into();
        let plain_text = vec![Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng)];
        let cipher_text = kemdem_encrypt::<false>(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();
        let decrypted = kemdem_decrypt(recipient_private_key, &cipher_text).unwrap();
        for (plain_text, decrypted_text) in plain_text.iter().zip(decrypted.iter()) {
            assert_eq!(*plain_text, *decrypted_text);
        }

        let shared_secret: TEAffine<BabyJubjub> =
            (recipient_public_key * ephemeral_private_key).into();

        assert_eq!(shared_secret.y, decrypted[3]);
    }

    #[test]
    fn test_kemdem_withdraw() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key = (public_point * recipient_private_key).into();

        let plain_text = vec![Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng)];
        let cipher_text = kemdem_encrypt::<true>(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();

        assert_eq!(&plain_text, &cipher_text[..3]);
    }
}
