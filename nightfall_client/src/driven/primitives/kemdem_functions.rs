use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ff::{BigInteger, One, PrimeField, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use jf_primitives::poseidon::{FieldHasher, Poseidon, PoseidonError};
use lib::plonk_prover::circuits::{DOMAIN_RECEIPT_DEM, DOMAIN_RECEIPT_KEM, DOMAIN_SHARED_SALT};
use log::error;
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
    if cipher_text.len() < 5 || !is_valid_epk_sign_flag(&cipher_text[4]) {
        return Err(PoseidonError::InvalidInputs);
    }

    // First we decompress the epk
    let mut point_bytes = cipher_text[3].into_bigint().to_bytes_le();
    if point_bytes.len() != 32 {
        return Err(PoseidonError::InvalidInputs);
    }
    let flag = cipher_text[4].into_bigint().to_bytes_le()[0] << 7;
    // The BabyJubjub compressed point encoding is exactly 32 bytes with the x-sign bit stored in the high bit.
    point_bytes[31] |= flag;

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

    let poseidon = Poseidon::<Fr254>::new();
    // Derive a shared salt from the shared secret using domain-separated Poseidon hash.
    let shared_salt = poseidon
        .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
        .map_err(|e| {
            error!("Failed to derive shared salt with Poseidon: {e}");
            PoseidonError::InvalidInputs
        })?;

    plain_text.push(shared_salt);
    Ok(plain_text)
}

#[derive(Debug, PartialEq)]
pub struct ReceiptDecryptOutput {
    pub nf_token_id: Fr254,
    pub nf_slot_id: Fr254,
    pub value: Fr254,
    pub sender_public_key_x: Fr254,
    pub sender_public_key_y: Fr254,
    pub erc_address: Fr254,
    pub token_id: Fr254,
    pub shared_salt: Fr254,
}

/// Encrypt a receipt payload using receipt-specific KEM-DEM domain separators.
///
/// Produces a [Fr254; 9] ciphertext:
///   [0..6] = 7 encrypted plaintext fields
///   [7]    = ephemeral public key y-coordinate
///   [8]    = sign flag for ephemeral public key x-coordinate
///
/// Plaintext fields (in order):
///   nf_token_id, nf_slot_id, value,
///   sender_public_key_x, sender_public_key_y, erc_address, token_id
pub fn receipt_kemdem_encrypt(
    ephemeral_private_key: BJJScalar,
    recipient_public_key: TEAffine<BabyJubjub>,
    plain_text: &[Fr254; 7],
    public_point: TEAffine<BabyJubjub>,
) -> Result<[Fr254; 9], PoseidonError> {
    let shared_secret: TEAffine<BabyJubjub> = (recipient_public_key * ephemeral_private_key).into();
    let poseidon = Poseidon::<Fr254>::new();

    let ephemeral_public_key: TEAffine<BabyJubjub> = (public_point * ephemeral_private_key).into();

    // Derive encryption key using receipt-specific KEM domain separator.
    let encryption_key = poseidon.hash(&[shared_secret.x, shared_secret.y, DOMAIN_RECEIPT_KEM])?;

    // Encrypt all 7 plaintext fields using receipt-specific DEM domain separator.
    let mut cipher_text = [Fr254::zero(); 9];
    for (i, plain) in plain_text.iter().enumerate() {
        let pad: Fr254 =
            poseidon.hash(&[encryption_key, DOMAIN_RECEIPT_DEM, Fr254::from(i as u64)])?;
        cipher_text[i] = pad + *plain;
    }

    // Append compressed ephemeral public key.
    let flag = if ephemeral_public_key.x > -ephemeral_public_key.x {
        Fr254::one()
    } else {
        Fr254::zero()
    };
    cipher_text[7] = ephemeral_public_key.y;
    cipher_text[8] = flag;

    Ok(cipher_text)
}

/// Decrypt a receipt ciphertext using receipt-specific KEM-DEM domain separators.
///
/// Expects a [Fr254; 9] ciphertext as produced by `receipt_kemdem_encrypt`.
/// Returns `ReceiptDecryptOutput` with all 7 plaintext fields and the shared salt.
pub fn receipt_kemdem_decrypt(
    recipient_private_key: BJJScalar,
    cipher_text: &[Fr254; 9],
) -> Result<ReceiptDecryptOutput, PoseidonError> {
    if !is_valid_epk_sign_flag(&cipher_text[8]) {
        return Err(PoseidonError::InvalidInputs);
    }

    // Reconstruct the ephemeral public key from cipher_text[7] (epk.y) and [8] (flag).
    let mut point_bytes = cipher_text[7].into_bigint().to_bytes_le();
    if point_bytes.len() != 32 {
        return Err(PoseidonError::InvalidInputs);
    }
    let flag = cipher_text[8].into_bigint().to_bytes_le()[0] << 7;
    // The BabyJubjub compressed point encoding is exactly 32 bytes with the x-sign bit stored in the high bit.
    point_bytes[31] |= flag;

    let epk = TEAffine::<BabyJubjub>::deserialize_compressed(&*point_bytes)
        .map_err(|_| PoseidonError::InvalidInputs)?;

    // Compute shared secret and derive decryption key.
    let shared_secret: TEAffine<BabyJubjub> = (epk * recipient_private_key).into();
    let poseidon = Poseidon::<Fr254>::new();
    let decryption_key = poseidon.hash(&[shared_secret.x, shared_secret.y, DOMAIN_RECEIPT_KEM])?;

    // Decrypt the 7 plaintext fields.
    let mut plain = [Fr254::zero(); 7];
    for (i, ct) in cipher_text.iter().take(7).enumerate() {
        let pad: Fr254 =
            poseidon.hash(&[decryption_key, DOMAIN_RECEIPT_DEM, Fr254::from(i as u64)])?;
        plain[i] = *ct - pad;
    }

    // Derive shared salt with the standard domain separator.
    let shared_salt = poseidon.hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])?;

    Ok(ReceiptDecryptOutput {
        nf_token_id: plain[0],
        nf_slot_id: plain[1],
        value: plain[2],
        sender_public_key_x: plain[3],
        sender_public_key_y: plain[4],
        erc_address: plain[5],
        token_id: plain[6],
        shared_salt,
    })
}

fn is_valid_epk_sign_flag(flag: &Fr254) -> bool {
    *flag == Fr254::zero() || *flag == Fr254::one()
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let poseidon = Poseidon::<Fr254>::new();
        // Derive a shared salt from the shared secret using domain-separated Poseidon hash.
        let shared_salt = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_SHARED_SALT])
            .unwrap();
        assert_eq!(shared_salt, decrypted[3]);
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

    #[test]
    fn compute_receipt_domain_constants() {
        use sha2::{Digest, Sha256};

        let kem_digest = Sha256::digest("Nightfall|ReceiptKEM".as_bytes());
        let dem_digest = Sha256::digest("Nightfall|ReceiptDEM".as_bytes());

        let expected_kem = Fr254::from_le_bytes_mod_order(&kem_digest);
        let expected_dem = Fr254::from_le_bytes_mod_order(&dem_digest);

        assert_eq!(expected_kem, DOMAIN_RECEIPT_KEM);
        assert_eq!(expected_dem, DOMAIN_RECEIPT_DEM);
    }

    #[test]
    fn test_receipt_kemdem_round_trip() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let plain_text: [Fr254; 7] = [
            Fr254::rand(rng), // nf_token_id
            Fr254::rand(rng), // nf_slot_id
            Fr254::rand(rng), // value
            Fr254::rand(rng), // sender_public_key_x
            Fr254::rand(rng), // sender_public_key_y
            Fr254::rand(rng), // erc_address
            Fr254::rand(rng), // token_id
        ];

        let cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();

        assert_eq!(cipher_text.len(), 9);

        let out = receipt_kemdem_decrypt(recipient_private_key, &cipher_text).unwrap();

        assert_eq!(out.nf_token_id, plain_text[0]);
        assert_eq!(out.nf_slot_id, plain_text[1]);
        assert_eq!(out.value, plain_text[2]);
        assert_eq!(out.sender_public_key_x, plain_text[3]);
        assert_eq!(out.sender_public_key_y, plain_text[4]);
        assert_eq!(out.erc_address, plain_text[5]);
        assert_eq!(out.token_id, plain_text[6]);
    }

    #[test]
    fn test_receipt_kemdem_ecdh_consistency() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();
        let ephemeral_public_key: TEAffine<BabyJubjub> =
            (public_point * ephemeral_private_key).into();

        let shared_secret_sender: TEAffine<BabyJubjub> =
            (recipient_public_key * ephemeral_private_key).into();
        let shared_secret_recipient: TEAffine<BabyJubjub> =
            (ephemeral_public_key * recipient_private_key).into();

        assert_eq!(shared_secret_sender, shared_secret_recipient);
    }

    #[test]
    fn test_receipt_kemdem_domain_separation() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        // Encrypt 3 fields using the regular protocol KEM-DEM
        let proto_plain = vec![Fr254::rand(rng), Fr254::rand(rng), Fr254::rand(rng)];
        let proto_ct = kemdem_encrypt::<false>(
            ephemeral_private_key,
            recipient_public_key,
            &proto_plain,
            public_point,
        )
        .unwrap();

        // Encrypt using receipt KEM-DEM with same first 3 fields
        let receipt_plain = [
            proto_plain[0],
            proto_plain[1],
            proto_plain[2],
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
        ];
        let receipt_ct = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &receipt_plain,
            public_point,
        )
        .unwrap();

        // Domain separation: the first 3 ciphertext elements must differ
        // even though the plaintext and ephemeral key are the same.
        assert_ne!(proto_ct[0], receipt_ct[0]);
        assert_ne!(proto_ct[1], receipt_ct[1]);
        assert_ne!(proto_ct[2], receipt_ct[2]);
    }

    #[test]
    fn test_receipt_kemdem_wrong_key_fails() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let wrong_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let plain_text: [Fr254; 7] = [
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
        ];

        let cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();

        // Decrypt with wrong key — should succeed syntactically but produce wrong plaintext.
        let out = receipt_kemdem_decrypt(wrong_private_key, &cipher_text).unwrap();
        assert_ne!(out.nf_token_id, plain_text[0]);
    }

    #[test]
    fn test_receipt_kemdem_rejects_invalid_sign_flag() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let plain_text: [Fr254; 7] = [
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
            Fr254::rand(rng),
        ];

        let mut cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &plain_text,
            public_point,
        )
        .unwrap();
        cipher_text[8] = Fr254::from(2u64);

        assert!(receipt_kemdem_decrypt(recipient_private_key, &cipher_text).is_err());
    }

    #[test]
    fn test_receipt_ciphertext_size() {
        assert_eq!(9 * 32, 288);
        assert_eq!(288 * 2, 576);
    }
}
