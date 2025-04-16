//! File contains utility functions used by the REST API, such as ones for converting from erc address and token id to
//! Nightfall token id.
use crate::domain::{entities::HexConvertible, error::ConversionError};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, PrimeField};
use ethers::{
    abi::{encode, Tokenizable},
    types::{H160, U256},
};
use log::debug;
use num::BigUint;
use sha2::{Digest, Sha256};

pub fn reverse_hex_string(hex_string: &str) -> String {
    let mut bytes = Vec::<u8>::from_hex_string(hex_string).unwrap();
    bytes.reverse();
    hex::encode(bytes)
}

#[allow(dead_code)]
pub fn to_nf_token_id_from_str(
    erc_address: &str,
    token_id: &str,
) -> Result<Fr254, ConversionError> {
    debug!(
        "Converting erc_address: {} and token_id: {}",
        erc_address, token_id
    );
    let mut erc_vec =
        Vec::<u8>::from_hex_string(erc_address).map_err(|_| ConversionError::ParseFailed)?;
    if erc_vec.len() < 20 {
        erc_vec.resize(20, 0);
    }
    let erc_bytes: [u8; 20] = erc_vec
        .try_into()
        .map_err(|_| ConversionError::FixedLengthArrayError)?;
    let mut token_vec =
        Vec::<u8>::from_hex_string(token_id).map_err(|_| ConversionError::ParseFailed)?;
    if token_vec.len() < 32 {
        token_vec.resize(32, 0);
    }
    token_vec.reverse();
    let token_bytes: [u8; 32] = token_vec
        .try_into()
        .map_err(|_| ConversionError::FixedLengthArrayError)?;

    let solidity_erc_address: H160 = H160(erc_bytes);
    let solidity_token_id: U256 = U256::from_big_endian(&token_bytes);

    // Initialize SHA-256 hasher
    let mut hasher = Sha256::new();

    // Apply the same encoding as before
    let encoded_input = encode(&[
        solidity_erc_address.into_token(),
        solidity_token_id.into_token(),
    ]);

    // Hash the encoded input
    hasher.update(encoded_input);
    let sha256_result = hasher.finalize();

    // Convert hash output to BigUint and apply right shift
    let hash_out: BigUint = BigUint::from_bytes_be(&sha256_result) >> 4;
    Ok(Fr254::from(hash_out))
}

pub fn to_nf_token_id_from_fr254(erc_address: Fr254, token_id: Fr254) -> Fr254 {
    let erc_address_bytes = {
        let mut bytes = erc_address.into_bigint().to_bytes_be();
        while bytes.len() < 32 {
            bytes.insert(0, 0);
        }
        // Reverse only the last 20 bytes
        let (prefix, suffix) = bytes.split_at(12);
        let mut reversed_suffix = suffix.to_vec();
        reversed_suffix.reverse(); // reverse the last 20 bytes

        let mut final_bytes = prefix.to_vec(); // first 12 bytes unchanged
        final_bytes.extend(reversed_suffix); // append reversed 20 bytes
        final_bytes
    };

    let token_id_bytes = {
        let mut bytes = token_id.into_bigint().to_bytes_be();
        while bytes.len() < 32 {
            bytes.insert(0, 0); // Left pad to 32 bytes
        }
        bytes
    };

    let mut input_bytes = Vec::new();
    input_bytes.extend_from_slice(&erc_address_bytes);
    input_bytes.extend_from_slice(&token_id_bytes);

    // Hash the result
    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    let digest = hasher.finalize();

    // Shift digest right by 4 bits as in Solidity implementation (to fit into Fr)
    let mut nf_token_id = BigUint::from_bytes_be(&digest);
    nf_token_id >>= 4;

    Fr254::from(nf_token_id)
}

pub fn to_nf_token_id_from_solidity(
    solidity_token_address: H160,
    solidity_token_id: U256,
) -> Fr254 {
    // Convert Solidity token address to raw bytes (20 bytes)
    let mut erc_address_bytes: Vec<u8> = solidity_token_address.0.to_vec();

    // Ensure the address is correctly padded to 20 bytes (matches `to_nf_token_id_from_fr254` behavior)
    while erc_address_bytes.len() < 32 {
        erc_address_bytes.insert(0, 0);
    }

    // Convert Solidity token ID to bytes (32 bytes, big-endian)
    let mut token_id_bytes = [0u8; 32];
    solidity_token_id.to_big_endian(&mut token_id_bytes);

    let mut input_bytes = Vec::new();
    input_bytes.extend_from_slice(&erc_address_bytes); // 20 bytes
    input_bytes.extend_from_slice(&token_id_bytes); // 32 bytes

    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    let sha256_result = hasher.finalize();

    // Convert hash output to BigUint and apply right shift
    let mut hash_out = BigUint::from_bytes_be(&sha256_result);
    hash_out >>= 4;

    Fr254::from(hash_out)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::driven::contract_functions::contract_type_conversions::FrBn254;
    use ark_std::UniformRand;
    use jf_primitives::circuit::sha256::Sha256HashGadget;
    use jf_relation::{Circuit, PlonkCircuit, Variable};
    use rand::Rng;
    #[test]
    fn test_reverse_hex_string() {
        let hex_string = "0x01234567890abcde0f";
        let reversed = reverse_hex_string(hex_string);
        assert_eq!(reversed, "0fdebc0a8967452301");
    }
    #[test]
    fn test_nf_token_id_consistency() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();

            // Generate 20-byte Ethereum address
            let erc_address: [u8; 20] = rng.gen();
            let erc_address_string = format!("0x{}", hex::encode(erc_address));
            // Generate random token ID
            let mut rng = jf_utils::test_rng();
            let token_id_fr = Fr254::rand(&mut rng);
            let token_id_string = Fr254::to_hex_string(&token_id_fr);

            // ---------Compute using `to_nf_token_id_from_str`---------
            let nf_token_id_from_str =
                to_nf_token_id_from_str(&erc_address_string, &token_id_string).unwrap();

            // ---------Compute using `to_nf_token_id_from_solidity`---------
            let solidity_erc_address =
                H160::from_slice(&Vec::<u8>::from_hex_string(&erc_address_string).unwrap());
            let mut token_id_bytes = Vec::<u8>::from_hex_string(&token_id_string).unwrap();
            token_id_bytes.resize(32, 0);
            token_id_bytes.reverse();
            let solidity_token_id = U256::from_big_endian(&token_id_bytes);
            let nf_token_id_from_solidity =
                to_nf_token_id_from_solidity(solidity_erc_address, solidity_token_id);

            // ---------Compute using `to_nf_token_id_from_fr254`---------
            let erc_address_fr = Fr254::from_hex_string(&erc_address_string).unwrap();
            let token_id_fr = Fr254::from_hex_string(&token_id_string).unwrap();

            let nf_token_id_from_fr254 = to_nf_token_id_from_fr254(erc_address_fr, token_id_fr);

            // ---------Compute using `full_shifted_sha256_hash` in the circuit---------

            let erc_address_fr = Fr254::from(FrBn254::from(solidity_erc_address));
            let mut circuit: PlonkCircuit<Fr254> = PlonkCircuit::new_ultra_plonk(4);
            let mut lookup_vars = Vec::<(Variable, Variable, Variable)>::new();
            let erc_address_var = circuit.create_variable(erc_address_fr).unwrap();
            let token_id_var = circuit.create_variable(token_id_fr).unwrap();
            let (_, nf_token_id_var) = circuit
                .full_shifted_sha256_hash(&[erc_address_var, token_id_var], &mut lookup_vars)
                .unwrap();
            circuit.finalize_for_arithmetization().unwrap();
            let nf_token_id_in_circuit = circuit.witness(nf_token_id_var).unwrap();

            // Test consistency of the computed values from different methods
            assert_eq!(
                nf_token_id_from_str, nf_token_id_from_solidity,
                "Mismatch between string-derived token ID and Solidity input"
            );
            assert_eq!(
                nf_token_id_from_str, nf_token_id_from_fr254,
                "Mismatch between string-derived token ID and Fr254 representation"
            );
            assert_eq!(
                nf_token_id_from_str, nf_token_id_in_circuit,
                "Mismatch between string-derived token ID and circuit input"
            );
        }
    }
}
