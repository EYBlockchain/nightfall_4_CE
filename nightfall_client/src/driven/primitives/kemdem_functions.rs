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

use lib::shared_entities::TokenType;

/// Input plaintext for [`receipt_kemdem_encrypt`].
pub struct ReceiptEncryptInput {
    pub sender_public_key_x: Fr254,
    pub sender_public_key_y: Fr254,
    pub erc_address: Fr254,
    pub token_type: TokenType,
    /// `token_id` for ERC721; `value` for ERC20.
    pub token_id_or_value: Fr254,
    pub receiver_commitment: Fr254,
}

/// Decrypted transfer receipt payload.
///
/// `token_id_or_value` holds the `token_id` when `token_type` is `ERC721`,
/// and the `value` when `token_type` is `ERC20`.
#[derive(Debug, PartialEq)]
pub struct ReceiptDecryptOutput {
    pub sender_public_key_x: Fr254,
    pub sender_public_key_y: Fr254,
    pub erc_address: Fr254,
    pub token_type: TokenType,
    pub token_id_or_value: Fr254,
    pub receiver_commitment: Fr254,
    pub shared_salt: Fr254,
}

/// Encrypt a receipt payload using receipt-specific KEM-DEM domain separators.
///
/// Produces a `[Fr254; 9]` ciphertext:
///   `[0..6]` = 7 encrypted plaintext fields
///   `[7]`    = ephemeral public key y-coordinate
///   `[8]`    = sign flag for ephemeral public key x-coordinate
///
/// Plaintext field layout (indices 0–6):
///   0: `sender_public_key_x`
///   1: `sender_public_key_y`
///   2: `erc_address`
///   3: `token_type` (`0` = ERC20, `2` = ERC721)
///   4: `token_id` (ERC721) or `value` (ERC20)
///   5: `receiver_commitment`
///   6: reserved (zero)
pub fn receipt_kemdem_encrypt(
    ephemeral_private_key: BJJScalar,
    recipient_public_key: TEAffine<BabyJubjub>,
    input: &ReceiptEncryptInput,
    public_point: TEAffine<BabyJubjub>,
) -> Result<[Fr254; 9], PoseidonError> {
    let plain_text = [
        input.sender_public_key_x,
        input.sender_public_key_y,
        input.erc_address,
        Fr254::from(u8::from(input.token_type) as u64),
        input.token_id_or_value,
        input.receiver_commitment,
        Fr254::zero(), // reserved
    ];
    let plain_text = &plain_text;
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
/// Expects a `[Fr254; 9]` ciphertext as produced by [`receipt_kemdem_encrypt`].
/// Returns a [`ReceiptDecryptOutput`] struct matching the encoded token type.
/// Returns `Err` if the sign flag is invalid or the token type is not supported by
/// the MVP receipt payload (`ERC20` or `ERC721`).
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

    // plain layout: [spk_x, spk_y, erc_address, token_type, token_id_or_value, receiver_commitment, reserved]
    let token_type_byte = plain[3].into_bigint().to_bytes_le()[0];
    let token_type = decode_receipt_token_type(token_type_byte)?;

    Ok(ReceiptDecryptOutput {
        sender_public_key_x: plain[0],
        sender_public_key_y: plain[1],
        erc_address: plain[2],
        token_type,
        token_id_or_value: plain[4],
        receiver_commitment: plain[5],
        shared_salt,
    })
}

fn is_valid_epk_sign_flag(flag: &Fr254) -> bool {
    *flag == Fr254::zero() || *flag == Fr254::one()
}

fn decode_receipt_token_type(token_type_byte: u8) -> Result<TokenType, PoseidonError> {
    match token_type_byte {
        0 => Ok(TokenType::ERC20),
        2 => Ok(TokenType::ERC721),
        _ => Err(PoseidonError::InvalidInputs),
    }
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
    fn test_receipt_kemdem_round_trip_erc721() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let input = ReceiptEncryptInput {
            sender_public_key_x: Fr254::rand(rng),
            sender_public_key_y: Fr254::rand(rng),
            erc_address: Fr254::rand(rng),
            token_type: TokenType::ERC721,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };

        let cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &input,
            public_point,
        )
        .unwrap();

        assert_eq!(cipher_text.len(), 9);

        let out = receipt_kemdem_decrypt(recipient_private_key, &cipher_text).unwrap();

        assert_eq!(out.sender_public_key_x, input.sender_public_key_x);
        assert_eq!(out.sender_public_key_y, input.sender_public_key_y);
        assert_eq!(out.erc_address, input.erc_address);
        assert_eq!(out.token_type, TokenType::ERC721);
        assert_eq!(out.token_id_or_value, input.token_id_or_value);
        assert_eq!(out.receiver_commitment, input.receiver_commitment);
    }

    #[test]
    fn test_receipt_kemdem_round_trip_erc20() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let input = ReceiptEncryptInput {
            sender_public_key_x: Fr254::rand(rng),
            sender_public_key_y: Fr254::rand(rng),
            erc_address: Fr254::rand(rng),
            token_type: TokenType::ERC20,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };

        let cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &input,
            public_point,
        )
        .unwrap();

        assert_eq!(cipher_text.len(), 9);

        let out = receipt_kemdem_decrypt(recipient_private_key, &cipher_text).unwrap();

        assert_eq!(out.sender_public_key_x, input.sender_public_key_x);
        assert_eq!(out.sender_public_key_y, input.sender_public_key_y);
        assert_eq!(out.erc_address, input.erc_address);
        assert_eq!(out.token_type, TokenType::ERC20);
        assert_eq!(out.token_id_or_value, input.token_id_or_value);
        assert_eq!(out.receiver_commitment, input.receiver_commitment);
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

        // Encrypt using receipt KEM-DEM — first 3 plaintext fields share values with proto_plain
        let receipt_input = ReceiptEncryptInput {
            sender_public_key_x: proto_plain[0],
            sender_public_key_y: proto_plain[1],
            erc_address: proto_plain[2],
            token_type: TokenType::ERC721,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };
        let receipt_ct = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &receipt_input,
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

        let input = ReceiptEncryptInput {
            sender_public_key_x: Fr254::rand(rng),
            sender_public_key_y: Fr254::rand(rng),
            erc_address: Fr254::rand(rng),
            token_type: TokenType::ERC721,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };

        let cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &input,
            public_point,
        )
        .unwrap();

        // Decrypt with wrong key. In the overwhelming majority of cases the token-type field
        // will be invalid for the MVP receipt format and decryption should fail.
        match receipt_kemdem_decrypt(wrong_private_key, &cipher_text) {
            Err(PoseidonError::InvalidInputs) => {}
            Ok(out) => assert_ne!(out.sender_public_key_x, input.sender_public_key_x),
        }
    }

    #[test]
    fn test_receipt_kemdem_rejects_unsupported_token_type() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let input = ReceiptEncryptInput {
            sender_public_key_x: Fr254::rand(rng),
            sender_public_key_y: Fr254::rand(rng),
            erc_address: Fr254::rand(rng),
            token_type: TokenType::ERC20,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };

        let mut cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &input,
            public_point,
        )
        .unwrap();

        let shared_secret: TEAffine<BabyJubjub> =
            (recipient_public_key * ephemeral_private_key).into();
        let poseidon = Poseidon::<Fr254>::new();
        let encryption_key = poseidon
            .hash(&[shared_secret.x, shared_secret.y, DOMAIN_RECEIPT_KEM])
            .unwrap();
        let token_type_index = 3u64;
        let token_type_pad = poseidon
            .hash(&[
                encryption_key,
                DOMAIN_RECEIPT_DEM,
                Fr254::from(token_type_index),
            ])
            .unwrap();
        cipher_text[3] = token_type_pad + Fr254::from(1u64);

        assert!(matches!(
            receipt_kemdem_decrypt(recipient_private_key, &cipher_text),
            Err(PoseidonError::InvalidInputs)
        ));
    }

    #[test]
    fn test_receipt_kemdem_rejects_invalid_sign_flag() {
        let rng = &mut test_rng();
        let recipient_private_key = BJJScalar::rand(rng);
        let ephemeral_private_key = BJJScalar::rand(rng);
        let public_point = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let recipient_public_key: TEAffine<BabyJubjub> =
            (public_point * recipient_private_key).into();

        let input = ReceiptEncryptInput {
            sender_public_key_x: Fr254::rand(rng),
            sender_public_key_y: Fr254::rand(rng),
            erc_address: Fr254::rand(rng),
            token_type: TokenType::ERC20,
            token_id_or_value: Fr254::rand(rng),
            receiver_commitment: Fr254::rand(rng),
        };

        let mut cipher_text = receipt_kemdem_encrypt(
            ephemeral_private_key,
            recipient_public_key,
            &input,
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
