use crate::{
    domain::{
        entities::{CompressedSecrets, Proposer, WithdrawData},
        error::ConversionError,
    },
    ports::db::ToHexString,
};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInt, BigInteger, BigInteger256, PrimeField};
use ark_std::Zero;
use core::fmt::Debug;
use ethers::types::{Address, U256};
use nightfall_bindings::{
    nightfall::WithdrawData as NightfallWithdrawData, round_robin::Proposer as NightfallProposer,
};
use num_bigint::BigUint;
use serde::{
    de::{self, Visitor},
    {Deserialize, Deserializer, Serialize},
};
use std::{fmt, ops::Add, str::FromStr};

/// enables conversion between a Proposer as used in the ProposerManager contract, and a for suitable for serialisation
impl From<NightfallProposer> for Proposer {
    fn from(proposer: NightfallProposer) -> Self {
        Proposer {
            stake: proposer.stake,
            addr: proposer.addr,
            url: proposer.url,
            next_addr: proposer.next_addr,
            previous_addr: proposer.previous_addr,
        }
    }
}

impl From<WithdrawData> for NightfallWithdrawData {
    fn from(data: WithdrawData) -> NightfallWithdrawData {
        let nf_token_id = Uint256::from(data.nf_token_id).0;
        let recipient_address = Address::from(
            Addr::try_from(data.withdraw_address)
                .expect("Could not convert WithdrawData withdraw address to Solidity address"),
        );
        let value = Uint256::from(data.value).0;
        let withdraw_fund_salt = Uint256::from(data.withdraw_fund_salt).0;
        NightfallWithdrawData {
            nf_token_id,
            recipient_address,
            value,
            withdraw_fund_salt,
        }
    }
}

/// A wrapper around the ark bn254 scalar field, which implements serde and other conversions using the newtype pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FrBn254(pub ark_bn254::Fr);
impl TryFrom<U256> for FrBn254 {
    type Error = ConversionError;
    fn try_from(u256: U256) -> Result<Self, ConversionError> {
        let max_positive = Uint256::from(<Fr254 as PrimeField>::MODULUS).0;
        if u256 > max_positive {
            return Err(ConversionError::Overflow);
        }
        Ok(FrBn254(Fr254::new(BigInt::<4>::from(Uint256(u256)))))
    }
}
impl From<FrBn254> for Fr254 {
    fn from(fq: FrBn254) -> Self {
        fq.0
    }
}

impl Zero for FrBn254 {
    fn zero() -> Self {
        Self(Fr254::zero())
    }

    fn is_zero(&self) -> bool {
        self.0 == Fr254::zero()
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

impl Add for FrBn254 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<Uint256> for BigInt<4> {
    fn from(uint256: Uint256) -> Self {
        BigInt::<4>::new(uint256.0 .0)
    }
}

/// Enables a wrapped Fr254 field to be deserialised with Serde. Fr254 itself does not appear to implement Serde
/// for serialisation and deserialisation.
impl<'de> Deserialize<'de> for FrBn254 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FrBn254;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Byte string to be converted into a field")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s: &str = Deserialize::deserialize(deserializer)?;
                let bn = BigUint::parse_bytes(s.as_bytes(), 16)
                    .ok_or(de::Error::custom("Failed to parse bytes"))?;
                let key: Fr254 = bn.into();
                Ok(FrBn254(key))
            }
        }
        deserializer.deserialize_newtype_struct("Fr254", FieldVisitor)
    }
}
/// Enables a wrapped Fr254 field to be serialised with Serde.
impl Serialize for FrBn254 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let fq = self.0;
        let bn: BigUint = fq.into();
        serializer.serialize_newtype_struct("Fr254", &bn.to_str_radix(16))
    }
}

impl ToHexString for FrBn254 {
    fn to_hex(&self) -> String {
        let fq = self.0;
        let bn: BigUint = fq.into();
        bn.to_str_radix(16)
    }
}

// leaving this here for now in case we change our minds
#[derive(Debug)]
pub struct Uint256(pub(crate) U256);
impl From<Fr254> for Uint256 {
    fn from(fq: Fr254) -> Self {
        let mut digits = [0u64; 4];
        let big_uint = BigUint::from(fq);
        for (i, digit) in big_uint.iter_u64_digits().enumerate() {
            digits[i] = digit;
        }
        Uint256(U256(digits))
    }
}
impl From<BigInt<4>> for Uint256 {
    fn from(value: BigInt<4>) -> Self {
        let mut digits = [0u64; 4];
        let big_uint = BigUint::from(value);
        for (i, digit) in big_uint.iter_u64_digits().enumerate() {
            digits[i] = digit;
        }
        Uint256(U256(digits))
    }
}

impl From<Uint256> for U256 {
    fn from(uint256: Uint256) -> Self {
        uint256.0
    }
}
impl From<u64> for Uint256 {
    fn from(value: u64) -> Self {
        let mut digits = [0u64; 4];
        digits[0] = value;
        Uint256(U256(digits))
    }
}
/// For converting from Fr254 to an Ethereum address. Does an overflow check.
#[derive(Debug)]
pub struct Addr(pub(crate) Address);
impl TryFrom<Fr254> for Addr {
    type Error = ConversionError;
    fn try_from(fq: Fr254) -> Result<Self, ConversionError> {
        let max_addr: Fr254 = Fr254::from_str("2923003274661805836407369665432566039311865085951")
            .map_err(|_| ConversionError::ParseFailed)?;
        if fq > max_addr {
            return Err(ConversionError::Overflow);
        }
        let big_int = BigInt::<4>::from(fq);
        let address = Address::from_slice(&big_int.to_bytes_be()[12..32]);
        Ok(Addr(address))
    }
}
impl From<Addr> for Address {
    fn from(addr: Addr) -> Self {
        addr.0
    }
}
impl From<Address> for FrBn254 {
    fn from(address: Address) -> Self {
        let mut digits = [0u64; 4];
        let big_uint = BigUint::from_bytes_be(address.as_bytes());
        for (i, digit) in big_uint.iter_u64_digits().enumerate() {
            digits[i] = digit;
        }
        FrBn254(Fr254::new(BigInteger256::new(digits)))
    }
}
impl From<[U256; 4]> for CompressedSecrets {
    fn from(ciphertext: [U256; 4]) -> CompressedSecrets {
        let mut compressed_point = [0u8; 32];
        ciphertext[3].to_little_endian(&mut compressed_point);
        // We need to work out what the 255th bit is set to
        let top_bit = compressed_point[31] >> 7;
        let to_subtract = BigUint::from(top_bit) << 255;
        let y_coord = BigUint::from_bytes_le(&compressed_point) - to_subtract;

        let final_secrets = [
            <FrBn254 as Into<Fr254>>::into(
                FrBn254::try_from(ciphertext[0])
                    .expect("Conversion from U256 to Fr should never fail"),
            ),
            <FrBn254 as Into<Fr254>>::into(
                FrBn254::try_from(ciphertext[1])
                    .expect("Conversion from U256 to Fr should never fail"),
            ),
            <FrBn254 as Into<Fr254>>::into(
                FrBn254::try_from(ciphertext[2])
                    .expect("Conversion from U256 to Fr should never fail"),
            ),
            Fr254::from(y_coord),
            Fr254::from(top_bit),
        ];

        CompressedSecrets {
            cipher_text: final_secrets, // if the conversion fails, it's not recoverable so ok to panic
        }
    }
}
impl From<CompressedSecrets> for [U256; 4] {
    fn from(compressed_secrets: CompressedSecrets) -> [U256; 4] {
        let secrets_4: BigUint = compressed_secrets.cipher_text[3].into();
        let flag: BigUint = compressed_secrets.cipher_text[4].into();

        let compressed_point: BigUint = secrets_4 + (flag << 255);

        let final_secret = U256::from_little_endian(&compressed_point.to_bytes_le());
        [
            Uint256::from(compressed_secrets.cipher_text[0]).0,
            Uint256::from(compressed_secrets.cipher_text[1]).0,
            Uint256::from(compressed_secrets.cipher_text[2]).0,
            final_secret,
        ]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use ark_std::Zero;
    use std::str::FromStr;
    #[test]
    fn test_conversion_from_u256_to_frbn254() {
        let p_minus_1 = Fr254::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let zero = Fr254::zero();
        let minus_1 = Fr254::from(-1i64);
        let p_minus_1_over_2 = <Fr254 as PrimeField>::MODULUS_MINUS_ONE_DIV_TWO;

        let minus_1_test =
            Fr254::from(FrBn254::try_from(U256::from(Uint256::from(minus_1))).unwrap());
        let p_minus_1_test =
            Fr254::from(FrBn254::try_from(U256::from(Uint256::from(p_minus_1))).unwrap());
        let zero_test = Fr254::from(FrBn254::try_from(U256::from(Uint256::from(zero))).unwrap());
        let p_minus_1_over_2_test =
            Fr254::from(FrBn254::try_from(U256::from(Uint256::from(p_minus_1_over_2))).unwrap());
        assert_eq!(p_minus_1, p_minus_1_test);
        assert_eq!(zero, zero_test);
        assert_eq!(p_minus_1_over_2, p_minus_1_over_2_test.into());
        assert_eq!(minus_1, minus_1_test);
        // check that modulus works too
        let u = U256::from_dec_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        assert_eq!(U256::from(Uint256::from(minus_1)), u);
    }
    #[test]
    fn test_conversion_from_address_to_fr254() {
        let address = Address::from([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ]);
        let fq: Fr254 = FrBn254::from(address).into();
        let address_test: Address = Addr::try_from(fq).unwrap().into();
        assert_eq!(address, address_test);
    }
    #[test]
    fn test_parsing_key_string() {
        let key: Fr254 = Fr254::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let key_string = key.to_string();
        let key_field = Fr254::from_str(&key_string).unwrap();
        assert_eq!(key_field, key);
    }
}
