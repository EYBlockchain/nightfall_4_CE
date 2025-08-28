use crate::{hex_conversion::HexConvertible, merkle_trees::trees::MerkleTreeError};
use ark_bn254::Fr;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
use mongodb::error::Error as MongoError;
use serde::{ser::Serialize, Deserialize, Deserializer, Serializer};
use std::fmt::{Debug, Write};

pub fn ark_se_bytes<S, A: CanonicalSerialize>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes = vec![];
    a.serialize_with_mode(&mut bytes, Compress::Yes)
        .map_err(serde::ser::Error::custom)?;
    s.serialize_bytes(&bytes)
}

pub fn ark_de_bytes<'de, D, A: CanonicalDeserialize>(data: D) -> Result<A, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: Vec<u8> = serde::de::Deserialize::deserialize(data)?;
    let a = A::deserialize_with_mode(s.as_slice(), Compress::Yes, Validate::Yes)
        .map_err(serde::de::Error::custom)?;
    Ok(a)
}

/// Serializes an arkworks element to a hex string using a bigendian representation.
pub fn ark_se_hex<S, A: CanonicalSerialize>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes = vec![];
    a.serialize_with_mode(&mut bytes, Compress::Yes)
        .map_err(serde::ser::Error::custom)?;
    bytes.reverse(); // Convert to big-endian (the arkworks format is little-endian and that's what their serializer produces)
    let hex_str = bytes.to_hex_string();
    s.serialize_str(&hex_str)
}

pub fn ark_de_hex<'de, D, A: CanonicalDeserialize>(data: D) -> Result<A, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let hex_str: &str = serde::de::Deserialize::deserialize(data)?;

    let mut bytes = Vec::<u8>::from_hex_string(hex_str).map_err(serde::de::Error::custom)?;
    bytes.reverse(); // Convert to little-endian (which is the expected format for the arkworks deserialiser)
    let a = A::deserialize_with_mode(&bytes[..], Compress::Yes, Validate::Yes)
        .map_err(serde::de::Error::custom)?;
    Ok(a)
}

pub fn serialize_to_padded_hex<T: CanonicalSerialize>(
    value: &T,
) -> Result<String, MerkleTreeError<MongoError>> {
    let mut value_hex_bytes = vec![];
    value
        .serialize_compressed(&mut value_hex_bytes)
        .map_err(|e: SerializationError| {
            MerkleTreeError::DatabaseError(MongoError::custom(e.to_string()))
        })?;

    let value_big_endian_bytes = bigint_to_big_endian(value_hex_bytes);
    Ok(bytes_to_hex_lpadded(&value_big_endian_bytes, 32))
}

pub fn bytes_to_hex_lpadded(bytes: &[u8], max_bytes: usize) -> String {
    assert!(
        bytes.len() <= max_bytes,
        "too many bytes to fit in max_bytes"
    );

    let mut hex_str = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut hex_str, "{byte:02x}").unwrap();
    }
    format!("{:0>width$}", hex_str, width = max_bytes * 2)
}

pub fn bigint_to_big_endian(bytes: Vec<u8>) -> Vec<u8> {
    let mut reversed_bytes = bytes;
    reversed_bytes.reverse();
    reversed_bytes
}

fn bytes_to_little_endian(bytes: Vec<u8>) -> Vec<u8> {
    let mut little_endian_bytes = bytes;
    little_endian_bytes.reverse();
    little_endian_bytes
}

pub fn serialize_fr_padded<S, A: CanonicalSerialize + Debug>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes = vec![];
    a.serialize_with_mode(&mut bytes, Compress::Yes)
        .map_err(serde::ser::Error::custom)?;
    let big_endian_bytes = bigint_to_big_endian(bytes);
    let hex_str = bytes_to_hex_lpadded(&big_endian_bytes, 32);
    s.serialize_str(&hex_str)
}

pub fn deserialize_fr_padded<'de, D, Fr: CanonicalDeserialize + Debug>(
    data: D,
) -> Result<Fr, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let hex_str: &str = Deserialize::deserialize(data)?;
    let bytes = hex::decode(hex_str).map_err(serde::de::Error::custom)?;

    let unpadded_bytes = if bytes.len() > 32 {
        bytes[bytes.len() - 32..].to_vec()
    } else {
        bytes.clone()
    };

    let little_endian_bytes = bytes_to_little_endian(unpadded_bytes);
    let fr =
        Fr::deserialize_with_mode(little_endian_bytes.as_slice(), Compress::Yes, Validate::Yes)
            .map_err(serde::de::Error::custom)?;
    Ok(fr)
}

struct FrWrapper(Fr);

impl Serialize for FrWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ark_se_bytes(&self.0, serializer).map_err(serde::ser::Error::custom)
    }
}

impl<'de> Deserialize<'de> for FrWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fr: Fr = ark_de_bytes(deserializer)?;
        Ok(FrWrapper(fr))
    }
}
pub struct FrWrapperhex(pub Fr);

impl Serialize for FrWrapperhex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ark_se_hex(&self.0, serializer).map_err(serde::ser::Error::custom)
    }
}

impl<'de> Deserialize<'de> for FrWrapperhex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fr: Fr = ark_de_hex(deserializer)?;
        Ok(FrWrapperhex(fr))
    }
}
pub struct FrWrapperPadded(pub Fr);

impl Serialize for FrWrapperPadded {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_fr_padded(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for FrWrapperPadded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_fr_padded(deserializer).map(FrWrapperPadded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr as Fr254;
    use std::str::FromStr;
    #[test]
    fn test_ark_se_and_ark_de_1() {
        let element = Fr::from_str(
            "21711016731996786641919559690090612179745446532414748301311487203631771980383",
        )
        .unwrap();
        let wrapper = FrWrapper(element);
        let serialized = serde_json::to_string(&wrapper).unwrap();

        let deserialized: FrWrapper = serde_json::from_str(&serialized).unwrap();
        let deserialized_element = deserialized.0;
        assert_eq!(element, deserialized_element);
    }
    #[test]
    fn test_ark_se_and_ark_de() {
        let element = Fr::from_str("10").unwrap();
        let wrapper = FrWrapper(element);
        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: FrWrapper = serde_json::from_str(&serialized).unwrap();
        let deserialized_element = deserialized.0;
        assert_eq!(element, deserialized_element);
    }
    #[test]
    fn test_ark_se_hex_and_ark_de_hex() {
        // 43981 is the decimal representation of 0xABCD
        let element = Fr254::from_str("43981").unwrap();
        let wrapper = FrWrapperhex(element);
        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: FrWrapperhex = serde_json::from_str(&serialized).unwrap();
        let deserialized_element = deserialized.0;
        assert_eq!(element, deserialized_element);
    }
    #[test]
    fn test_fr_padded_serialization() {
        let element = Fr::from_str("10").unwrap();
        let wrapper = FrWrapperPadded(element);

        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: FrWrapperPadded = serde_json::from_str(&serialized).unwrap();

        assert_eq!(element, deserialized.0);
    }

    #[test]
    fn test_fr_padded_with_large_value() {
        let element = Fr::from_str(
            "21711016731996786641919559690090612179745446532414748301311487203631771980383",
        )
        .unwrap();
        let wrapper = FrWrapperPadded(element);
        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: FrWrapperPadded = serde_json::from_str(&serialized).unwrap();

        assert_eq!(element, deserialized.0);
    }

    #[test]
    fn test_fr_padded_with_largest_value_among_four() {
        // Define 4 values of different sizes
        let element1 = Fr::from_str("10").unwrap();
        let element2 = Fr::from_str("1").unwrap();
        let element3 = Fr::from_str("2").unwrap();
        let element4 = Fr::from_str("0").unwrap();

        // Wrap each element in a wrapper
        let wrapper1 = FrWrapperPadded(element1);
        let wrapper2 = FrWrapperPadded(element2);
        let wrapper3 = FrWrapperPadded(element3);
        let wrapper4 = FrWrapperPadded(element4);

        // Serialize the elements
        let serialized1 = serde_json::to_string(&wrapper1).unwrap();
        let serialized2 = serde_json::to_string(&wrapper2).unwrap();
        let serialized3 = serde_json::to_string(&wrapper3).unwrap();
        let serialized4 = serde_json::to_string(&wrapper4).unwrap();

        println!("Serialized value 1: {serialized1}");
        println!("Serialized value 2: {serialized2}");
        println!("Serialized value 3: {serialized3}");
        println!("Serialized value 4: {serialized4}");

        // Compare serialized values and find the largest
        let largest_serialized = vec![
            serialized1.clone(),
            serialized2.clone(),
            serialized3.clone(),
            serialized4.clone(),
        ]
        .into_iter()
        .max()
        .unwrap(); // Use `max` to get the largest serialized value

        // Print the largest serialized value
        println!("The largest serialized value is: {largest_serialized}");

        // Deserialize each element
        let deserialized1: FrWrapperPadded = serde_json::from_str(&serialized1).unwrap();
        let deserialized2: FrWrapperPadded = serde_json::from_str(&serialized2).unwrap();
        let deserialized3: FrWrapperPadded = serde_json::from_str(&serialized3).unwrap();
        let deserialized4: FrWrapperPadded = serde_json::from_str(&serialized4).unwrap();

        // Print values after deserialization
        println!("Value after deserialization 1: {0:?}", deserialized1.0);
        println!("Value after deserialization 2: {0:?}", deserialized2.0);
        println!("Value after deserialization 3: {0:?}", deserialized3.0);
        println!("Value after deserialization 4: {0:?}", deserialized4.0);

        // Assert equality after deserialization
        assert_eq!(element1, deserialized1.0);
        assert_eq!(element2, deserialized2.0);
        assert_eq!(element3, deserialized3.0);
        assert_eq!(element4, deserialized4.0);

        // Check if the largest serialized value matches the initial value
        if largest_serialized == serialized1 {
            println!("The largest value is element1: {element1:?}");
        } else if largest_serialized == serialized2 {
            println!("The largest value is element2: {element2:?}");
        } else if largest_serialized == serialized3 {
            println!("The largest value is element3: {element3:?}");
        } else if largest_serialized == serialized4 {
            println!("The largest value is element4: {element4:?}");
        }
    }
}
