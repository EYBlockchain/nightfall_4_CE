/// This module provides functionality to convert various types to and from hexadecimal strings.
/// It uses a bigendian representation for the conversions.


use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger256, BigInteger};
use num_bigint::BigUint;
use ark_ff::PrimeField;
use crate::error::HexError;
use nf_curves::ed_on_bn254::Fr as BJJScalar;


// Define a single trait for hexadecimal conversion
pub trait HexConvertible {
    fn to_hex_string(&self) -> String;
    fn from_hex_string(hex_str: &str) -> Result<Self, HexError>
    where
        Self: Sized;
}

// Implement the trait for Fr254
impl HexConvertible for Fr254 {
    fn to_hex_string(&self) -> String {
        let bytes = self.into_bigint().to_bytes_be();
        hex::encode(bytes)
    }

    fn from_hex_string(hex_str: &str) -> Result<Fr254, HexError> {
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let decoded_bytes = hex::decode(hex_str).map_err(|_| HexError::InvalidHexFormat)?;
        let big_uint = BigUint::from_bytes_be(&decoded_bytes);
        Ok(Fr254::from(big_uint))
    }
}

// Implement the trait for BJJScalar
impl HexConvertible for BJJScalar {
    fn to_hex_string(&self) -> String {
        let bytes = self.into_bigint().to_bytes_be();
        hex::encode(bytes)
    }

    fn from_hex_string(hex_str: &str) -> Result<BJJScalar, HexError> {
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let mut decoded_bytes = hex::decode(hex_str).map_err(|_| HexError::InvalidHexFormat)?;

        while decoded_bytes.len() < 32 {
            decoded_bytes.push(0);
        }

        if decoded_bytes.len() != 32 {
            return Err(HexError::InvalidStringLength);
        }

        let big_uint = BigUint::from_bytes_be(&decoded_bytes);
        Ok(BJJScalar::from(big_uint))
    }
}
// Implement the trait for i64
impl HexConvertible for i64 {
    fn to_hex_string(&self) -> String {
        let i_bytes = self.to_be_bytes().to_vec();
        hex::encode(i_bytes)
    }

    fn from_hex_string(hex_str: &str) -> Result<i64, HexError> {
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        // pad with zero bytes to the left if the length is less than 8
        let padded_hex_str = format!("{:0>16}", hex_str); // Pad with zeros to the left
        let decoded_bytes = hex::decode(padded_hex_str).map_err(|_| HexError::InvalidHexFormat)?;
        if decoded_bytes.len() != 8 {
            return Err(HexError::InvalidStringLength);
        }
        let byte_array = <[u8; 8]>::try_from(decoded_bytes.as_slice())
            .map_err(|_| HexError::InvalidStringLength)?;
        Ok(i64::from_be_bytes(byte_array))
    }
}

impl HexConvertible for BigInteger256 {
    fn to_hex_string(&self) -> String {
        let bytes = self.to_bytes_be();
        hex::encode(bytes)
    }

    fn from_hex_string(hex_str: &str) -> Result<BigInteger256, HexError> {
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let decoded_bytes = hex::decode(hex_str).map_err(|_| HexError::InvalidHexFormat)?;

        // Here, ensure that the byte length matches BigInteger256's expected length
        if decoded_bytes.len() > 32 {
            return Err(HexError::InvalidStringLength);
        }

        let mut padded_bytes = vec![0u8; 32 - decoded_bytes.len()];
        padded_bytes.extend(decoded_bytes);
        let big_uint = BigUint::from_bytes_be(&padded_bytes);

        // Convert BigUint to BigInteger256
        big_uint.try_into().map_err(|_| HexError::InvalidConversion)
    }
}
impl HexConvertible for Vec<u8> {
    fn to_hex_string(&self) -> String {
        hex::encode(self)
    }

    fn from_hex_string(hex_str: &str) -> Result<Vec<u8>, HexError> {
        let s_int = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        if s_int.len() % 2 != 0 || s_int.is_empty() {
            return Err(HexError::InvalidStringLength);
        }
        hex::decode(s_int).map_err(|_| HexError::InvalidString)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_ff::BigInt;
    use ark_std::rand::{self, Rng, RngCore};
    use ark_std::test_rng;
    use nf_curves::ed_on_bn254::Fr as BJJScalar;
    use ark_ff::UniformRand;

    #[test]
    fn correctly_manipulate_strings() {
        // Test Vec<u8> <-> hex string
        let test_vec: Vec<u8> = vec![
            0x01, 0xd5, 0xed, 0x4c, 0x6c, 0x7a, 0x9d, 0xff, 0x2f, 0x5e, 0x38, 0x53, 0x3c, 0x06, 0x73,
            0xe2, 0x52, 0xd0, 0xe7, 0x61, 0xa6, 0x21, 0xfb, 0x01, 0xd7, 0x50, 0x40, 0xda, 0x65,
            0xa5, 0x4a, 0x4a, 0x00
        ];
        let test_string = "01d5ed4c6c7a9dff2f5e38533c0673e252d0e761a621fb01d75040da65a54a4a00";
        let encoded = test_vec.to_hex_string();
        assert_eq!(encoded, test_string);
        let decoded = Vec::<u8>::from_hex_string(&encoded).unwrap();
        assert_eq!(test_vec, decoded);

        // Test Fr254 <-> hex string
        let test_fr254 = Fr254::from(BigInt::new([
            0x5f2415beff697c2a,
            0x5a65d1024be34f75,
            0xc84c19680f1279d5,
            0x302b6d99eae12fb5,
        ]));
        let hex_from_fr254 = test_fr254.to_hex_string();
        let fr254_from_hex = Fr254::from_hex_string(&hex_from_fr254).unwrap();
        assert_eq!(test_fr254, fr254_from_hex);

        // Test i64 <-> hex string
        let test_i64: i64 = -1234567890123456789;
        let hex_from_i64 = test_i64.to_hex_string();
        let i64_from_hex = i64::from_hex_string(&hex_from_i64).unwrap();
        assert_eq!(test_i64, i64_from_hex);
    }

    #[test]
    fn correctly_convert_fr_254() {
        let test_fr254 = Fr254::from(BigInt::new([
            0x5f2415beff697c2a,
            0x5a65d1024be34f75,
            0xc84c19680f1279d5,
            0x302b6d99eae12fb5,
        ]));
        let hex_from_fr254 = Fr254::to_hex_string(&test_fr254);
        println!("Hex from Fr254: {}", hex_from_fr254);
        let fr254_from_hex = Fr254::from_hex_string(&hex_from_fr254).unwrap();
        assert_eq!(test_fr254, fr254_from_hex);
    }

    #[test]
    fn correctly_convert_bjj_scalar() {
        let rng = &mut test_rng();
        let test_bjj_scalar = BJJScalar::rand(rng);
        // Convert BJJScalar to hex string
        let hex_string_from_bjj_scalar = BJJScalar::to_hex_string(&test_bjj_scalar);
        // Convert hex string back to BJJScalar
        let parsed_bjj_scalar =
            BJJScalar::from_hex_string(&hex_string_from_bjj_scalar).unwrap();
        assert_eq!(test_bjj_scalar, parsed_bjj_scalar);
    }

    #[test]
    fn correctly_convert_i64() {
        let mut rng = rand::thread_rng();
        let test_i64: i64 = rng.gen();

        // Convert i64 to hex string
        let hex_string_from_i64 = i64::to_hex_string(&test_i64);

        // Convert hex string back to i64
        let parsed_i64 = i64::from_hex_string(&hex_string_from_i64).unwrap();

        assert_eq!(test_i64, parsed_i64);

        // Check it works if the string isn't eight bytes long
        let test_hex = "0x04";
        let parsed_i64 = i64::from_hex_string(test_hex).unwrap();
        assert_eq!(parsed_i64, 4);
    }

    #[test]
    fn test_bigint256_hex_conversion() {
        let original_bigint256 = BigInteger256::new([1; 4]);
        let hex_string = original_bigint256.to_hex_string();
        let parsed_bigint256 = BigInteger256::from_hex_string(&hex_string)
            .expect("Failed to convert hex string back to BigInteger256");
        assert_eq!(
            original_bigint256, parsed_bigint256,
            "BigInteger256 conversion failed"
        );
    }

    #[test]
    fn test_vec_u8_hex_conversion() {
        let mut original_vec = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut original_vec);
        let hex_string = original_vec.to_hex_string();
        let parsed_vec = Vec::<u8>::from_hex_string(&hex_string)
            .expect("Failed to convert hex string back to Vec<u8>");
        assert_eq!(original_vec, parsed_vec, "Vec<u8> conversion failed");
    }

    #[test]
    fn test_vec_u8_hex_conversion_with_prefix() {
        // Generate a random 16-byte vector (for testing with a "0x" prefix)
        let mut original_vec = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut original_vec);

        // Convert to hex string and add "0x" prefix
        let hex_string = format!("0x{}", original_vec.to_hex_string());

        // Convert back from hex string to Vec<u8>
        let parsed_vec = Vec::<u8>::from_hex_string(&hex_string)
            .expect("Failed to convert hex string back to Vec<u8>");

        // Assert that the original and parsed vectors are the same
        assert_eq!(
            original_vec, parsed_vec,
            "Vec<u8> conversion with prefix failed"
        );
    }
}
