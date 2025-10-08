use crate::{ports::keys::KeySpending};

use ark_bn254::Fr as Fr254;
use ark_ec::twisted_edwards::Affine as TEAffine;
use ark_ec::CurveGroup;
use jf_primitives::poseidon::{FieldHasher, Poseidon};
use lib::serialization::{ark_de_hex, ark_se_hex};
use nf_curves::ed_on_bn254::{
    BJJTEAffine as JubJub, BabyJubjub, Fr as BJJScalar, GENERATOR_X, GENERATOR_Y,
};

use ark_ff::{BigInteger, BigInteger256, Field, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use bip32::{DerivationPath, Mnemonic, XPrv};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

// The baby jub jub curve in ark is defined in Edwards form and use a different generator than nightfall_3
// For nightfall_3 compatibility we will use the twisted edwards representation and map them to points in Edwards form
// The affine coordinate mapping from Edwards to its twist is: (x, y) :-> ( x/√a, y) where a is 168700.
// The affine coordinate mapping from Twisted Edwards to Edwards is the inverse: (x * √a, y) :-> ( x, y) where a is 168700.
// This isomorphic mapping is only possible since a is square in Fp.
// # Example
// ```
// const GENERATOR_X_Twist: Fr254 = MontFp!("16540640123574156134436876038791482806971768689494387082833631921987005038935");
// const GENERATOR_X_Edwards: Fr254 = MontFp!("3139523856513067560787432452907379745940282893366922042067385534101160882765");
// const GENERATOR_Y: Fr254 = MontFp!("20819045374670962167435360035096875258406992893633759881276124905556507972311");
// let sqrt_a = MontFp!("168700").sqrt().unwrap()
// assert_eq!(GENERATOR_X_Twist * sqrt_a , GENERATOR_X_Edwards);
// ```
// pub const GENERATOR_X: Fr254 =
//     MontFp!("3139523856513067560787432452907379745940282893366922042067385534101160882765");

// pub const GENERATOR_Y: Fr254 =
//     MontFp!("20819045374670962167435360035096875258406992893633759881276124905556507972311");

/// Prefix for hashes for zkp private ket and nullifier
/// PRIVATE_KEY_PREFIX = keccak256('zkpPrivateKey') % BN128_GROUP_ORDER
const PRIVATE_KEY_PREFIX: &str =
    "2708019456231621178814538244712057499818649907582893776052749473028258908910";
/// PRIVATE_KEY_PREFIX = keccak256('nullifierKey') % BN128_GROUP_ORDER
const NULLIFIER_PREFIX: &str =
    "7805187439118198468809896822299973897593108379494079213870562208229492109015";

#[derive(Debug)]
pub enum KeyError {
    BadKeyDerivation,
    BadPublicKey(SerializationError),
    PrimeFieldRepr(ff_ce::PrimeFieldDecodingError),
    HashError,
    NotFound,
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadKeyDerivation => write!(f, "BadKeyDerivation"),
            Self::BadPublicKey(serialization_error) => write!(f, "{serialization_error}"),
            Self::PrimeFieldRepr(decoding_error) => write!(f, "{decoding_error}"),
            Self::HashError => write!(f, "HashError"),
            Self::NotFound => write!(f, "Key lookup failed"),
        }
    }
}

impl Error for KeyError {}

impl From<bip32::Error> for KeyError {
    fn from(_err: bip32::Error) -> KeyError {
        KeyError::BadKeyDerivation
    }
}

impl From<SerializationError> for KeyError {
    fn from(_err: SerializationError) -> KeyError {
        KeyError::BadPublicKey(_err)
    }
}
impl From<ff_ce::PrimeFieldDecodingError> for KeyError {
    fn from(_err: ff_ce::PrimeFieldDecodingError) -> KeyError {
        KeyError::PrimeFieldRepr(_err)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default, Copy)]
pub struct ZKPKeys {
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub root_key: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub nullifier_key: Fr254,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub zkp_private_key: BJJScalar,
    #[serde(serialize_with = "ark_se_hex", deserialize_with = "ark_de_hex")]
    pub zkp_public_key: JubJub,
}

impl fmt::Display for ZKPKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(rootKey: {}, nullifier_key: {}, zkp_private_key: {}, zkp_public_key: ({}, {}))",
            self.root_key,
            self.nullifier_key,
            self.zkp_private_key,
            self.zkp_public_key.x,
            self.zkp_public_key.y
        )
    }
}

impl ZKPKeys {
    pub fn new(root_key: Fr254) -> Result<ZKPKeys, KeyError> {
        let poseidon: Poseidon<Fr254> = Poseidon::new();
        let [zkp_private_key_bytes, nullifier_key_bytes] = [PRIVATE_KEY_PREFIX, NULLIFIER_PREFIX]
            .map(|prefix_str| {
                let prefix = BigUint::parse_bytes(prefix_str.as_bytes(), 10)
                    .map(|v| v.into())
                    .ok_or(KeyError::HashError)?;
                poseidon
                    .hash(&[root_key, prefix])
                    .map_err(|_| KeyError::HashError)
            });

        let nullifier_key: Fr254 = nullifier_key_bytes?;

        let zkp_private_key = BJJScalar::from_be_bytes_mod_order(
            &BigInteger256::from(zkp_private_key_bytes?).to_bytes_be(),
        );

        // let generator = JubJubAffine::new(GENERATOR_X, GENERATOR_Y);
        let generator = TEAffine::<BabyJubjub>::new(GENERATOR_X, GENERATOR_Y);
        let zkp_public_key = (generator * zkp_private_key).into_affine();
        let zkp_keys = ZKPKeys {
            root_key,
            zkp_private_key,
            zkp_public_key,
            nullifier_key,
        };
        Ok(zkp_keys)
    }

    /// Derives ZKP Keys from a Bip32 compatible Mnemonic (24-word phrase) and a
    /// derivation path (e.g., "m/44'/60'/0'/0/0)
    pub fn derive_from_mnemonic(
        mnemonic: &Mnemonic,
        path: &DerivationPath,
    ) -> Result<ZKPKeys, KeyError> {
        let seed = mnemonic.to_seed("");
        let root_key_bytes: [u8; 32] = XPrv::derive_from_path(seed, path)?.to_bytes();
        let root_key: Fr254 = Fr254::from_be_bytes_mod_order(&root_key_bytes);
        ZKPKeys::new(root_key)
    }

    /// Compress the public key
    /// retains the y-coordinate and includes a flag of the parity of the x-coordinate.
    /// Returns a Little Endian vector of bytes.
    #[allow(dead_code)]
    pub fn compressed_public_key(&self) -> Result<Vec<u8>, KeyError> {
        let mut compressed_bytes = Vec::new();
        self.zkp_public_key
            .serialize_compressed(&mut compressed_bytes)?;
        Ok(compressed_bytes)
    }

    /// Decompress the public key
    #[allow(dead_code)]
    pub fn decompress_public_key(compressed: Vec<u8>) -> Result<JubJub, KeyError> {
        JubJub::deserialize_compressed(&*compressed).map_err(KeyError::BadPublicKey)
    }

    /// For completeness, this is a helper function to map
    /// the default edwards to the twisted affine coordinate we are more used to in Nightfall
    #[allow(dead_code)]
    pub fn pubkey_twisted_ed_affine(zkp_public_key: JubJub) -> (Fr254, Fr254) {
        let sqrt_a = Fr254::from(168700)
            .sqrt()
            .expect("Square root should not fail for this number"); // This is safe to unwrap
        (zkp_public_key.x / sqrt_a, zkp_public_key.y)
    }
}

impl KeySpending for ZKPKeys {
    fn get_nullifier_key(&self) -> Fr254 {
        self.nullifier_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::mocks::Mocks;
    use ark_ff::BigInt;
    use std::str::FromStr;
    #[ignore]
    #[test]
    // these test vectors originate from the key derivation function in NF_3, so therefore this test is
    // testing against NF_3.
    fn generate_from_rootkey() {
        let test_zkp_keys = ZKPKeys::new(Mocks::get_root_key()).unwrap();

        assert_eq!(test_zkp_keys.root_key, Mocks::get_root_key());
        assert_eq!(test_zkp_keys.nullifier_key, Mocks::get_nullifier_key());
        assert_eq!(test_zkp_keys.zkp_private_key, Mocks::get_zkp_private_key());

        let nf3 = ZKPKeys::pubkey_twisted_ed_affine(test_zkp_keys.zkp_public_key);

        assert_eq!(nf3.0, Mocks::get_zkp_public_key_x());
        assert_eq!(nf3.1, Mocks::get_zkp_public_key_y());
        // Note that NF_3 compressed keys are a compression of a twisted Edwards point, whereas NF_4 compresses an Edwards point.
        // We cannot change this easily as Ark uses an Edwards implementation.
        // The sign of the x ordinate can differ therefore, and this affects the sign bit (the two may not agree). The test vector has had
        // its sign flipped from the pure NF_3 result in this case.
        assert_eq!(
            test_zkp_keys.compressed_public_key().unwrap(),
            Mocks::get_compressed_zkp_public_key()
        );
        let decompressed_zkp_public_key =
            ZKPKeys::decompress_public_key(Mocks::get_compressed_zkp_public_key()).unwrap();
        assert_eq!(decompressed_zkp_public_key, Mocks::get_zkp_public_key());
        assert_eq!(
            hex::decode(hex::encode(Mocks::get_compressed_zkp_public_key())).unwrap(),
            Mocks::get_compressed_zkp_public_key()
        );
    }
    #[test]
    fn derive_from_mnemonic() {
        let root_key = Fr254::from(BigInt::new([
            3734087178892589020,
            2931610968744645457,
            310893676995116520,
            1017407045758727050,
        ]));
        // note the rust BIP32 crate only supports 24 words phrases so NF_3 mnemonics are not compatible with NF_4
        let mnemonic = Mnemonic::new(
             "spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work", 
            Default::default()).unwrap();
        let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0").unwrap();
        let zkp_keys_mnemonic = ZKPKeys::derive_from_mnemonic(&mnemonic, &derivation_path).unwrap();
        let zkp_keys = ZKPKeys::new(root_key).unwrap();
        assert_eq!(zkp_keys_mnemonic, zkp_keys);
    }
}
