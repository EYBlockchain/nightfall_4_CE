use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use configuration::settings::Settings;
use ethers::types::U256;
use jf_plonk::proof_system::structs::{VerifyingKey, VK};

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_vk_contract<const TEST: bool>(vk: &VerifyingKey<Bn254>, settings: &Settings) {
    let domain_size = vk.domain_size();
    let domain_size_fr = Fr254::from(domain_size as u32);
    let domain_size_inv = U256::from_little_endian(
        &domain_size_fr
            .inverse()
            .unwrap()
            .into_bigint()
            .to_bytes_le(),
    );
    let domain = Radix2EvaluationDomain::<Fr254>::new(domain_size).unwrap();
    let size_inv = domain_size_inv;
    let group_gen = U256::from_little_endian(&domain.group_gen().into_bigint().to_bytes_le());
    let group_gen_inv =
        U256::from_little_endian(&domain.group_gen_inv().into_bigint().to_bytes_le());

    let domain_size_u256 = U256::from(domain_size as u32);
    let num_inputs_u256 = U256::from(vk.num_inputs() as u32);
    let sigma_comms_u256: Vec<U256> = vk
        .sigma_comms
        .iter()
        .flat_map(|comm| {
            let x = U256::from_big_endian(&comm.x.into_bigint().to_bytes_be());
            let y = U256::from_big_endian(&comm.y.into_bigint().to_bytes_be());
            vec![x, y]
        })
        .collect();

    let selector_comms_u256: Vec<U256> = vk
        .selector_comms
        .iter()
        .flat_map(|comm| {
            let x = U256::from_big_endian(&comm.x.into_bigint().to_bytes_be());
            let y = U256::from_big_endian(&comm.y.into_bigint().to_bytes_be());
            vec![x, y]
        })
        .collect();
    let ks_u256: Vec<U256> =
        vk.k.iter()
            .map(|k| U256::from_big_endian(&k.into_bigint().to_bytes_be()))
            .collect();
    let vk_vec_u256 = Vec::<Fq254>::from(vk.clone())
        .into_iter()
        .map(|x| U256::from_little_endian(&x.into_bigint().to_bytes_le()))
        .collect::<Vec<_>>();
    let range_table_comm_u256 = [
        vk_vec_u256[56], // x
        vk_vec_u256[57], // y
    ];
    let key_table_comm_u256 = [
        vk_vec_u256[58], // x
        vk_vec_u256[59], // y
    ];
    let table_dom_sep_comm_u256 = [
        vk_vec_u256[60], // x
        vk_vec_u256[61], // y
    ];
    let q_dom_sep_comm_u256 = [
        vk_vec_u256[62], // x
        vk_vec_u256[63], // y
    ];

    let open_key_g = [
        vk_vec_u256[64], // x
        vk_vec_u256[65], // y
    ];

    let h = [
        vk_vec_u256[67], // x1
        vk_vec_u256[66], // x2
        vk_vec_u256[69], // y1
        vk_vec_u256[68], // y2
    ];

    let beta_h = [
        vk_vec_u256[71], // x1
        vk_vec_u256[70], // x2
        vk_vec_u256[73], // y1
        vk_vec_u256[72], // y2
    ];
    let join_path = Path::new(&settings.contracts.assets)
        .parent()
        .unwrap()
        .join("contracts/Nightfall.sol");

    let path_out: PathBuf;
    let cwd = std::env::current_dir().unwrap();
    let mut cwd = cwd.as_path();
    loop {
        let file_path = cwd.join(&join_path);
        if file_path.is_file() {
            path_out = if !TEST {
                file_path
                    .parent()
                    .unwrap()
                    .join("proof_verification/RollupProofVerificationKey.sol")
            } else {
                file_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("contracts/proof_verification/UltraPlonkVerificationKey.sol")
            };
            break;
        }

        cwd = cwd.parent().ok_or("No parent in path").unwrap();
    }

    if path_out.is_file() {
        std::fs::remove_file(&path_out).unwrap();
    }
    if let Some(parent) = path_out.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("Failed to create test_contracts folder: {e}"));
    }
    // let mut file = std::fs::File::create(&path_out).unwrap();

    let mut file: File = File::create(path_out).unwrap();
    let import_path = "./Types.sol";
    let extra_line = if TEST {
        "// This contract is made to test create_vk_contract function.\n"
    } else {
        ""
    };
    file.write_fmt(format_args!(
        "// SPDX-License-Identifier: Apache-2.0 \n
    {} \n
    pragma solidity ^0.8.20; \n
    import \"{}\"; \n
        
    library UltraPlonkVerificationKey {{ \n

        function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) {{ \n
            assembly {{ \n
            // domain_size
            mstore(add(vk, 0x00), {:#x}) \n
            // num_inputs \n
            mstore(add(vk, 0x20), {:#x}) \n
            // sigma_comms_1.x \n
            mstore( \n
                mload(add(vk, 0x40)), \n
                {:#x} \n
            ) \n
            // sigma_comms_1.y \n
            mstore( \n
                add(mload(add(vk, 0x40)), 0x20), \n
                {:#x} \n
            ) \n
            // sigma_comms_2.x \n
            mstore( \n
                mload(add(vk, 0x60)), \n
                {:#x} \n
            ) \n
            // sigma_comms_2.y \n
            mstore( \n
                add(mload(add(vk, 0x60)), 0x20), \n
                {:#x} \n
            ) \n
            // sigma_comms_3.x \n
            mstore( \n
                mload(add(vk, 0x80)), \n
                {:#x} \n
            ) \n
            // sigma_comms_3.y \n
            mstore( \n
                add(mload(add(vk, 0x80)), 0x20), \n
                {:#x} \n
            ) \n
            // sigma_comms_4.x \n
            mstore( \n
                mload(add(vk, 0xa0)), \n
                {:#x} \n
            ) \n
            // sigma_comms_4.y \n 
            mstore( \n
                add(mload(add(vk, 0xa0)), 0x20), \n
                {:#x} \n
            ) \n
            // sigma_comms_5.x \n
            mstore( \n
                mload(add(vk, 0xc0)), \n
                {:#x} \n
            ) \n
            // sigma_comms_5.y \n
            mstore( \n
                add(mload(add(vk, 0xc0)), 0x20), \n
                {:#x} \n
            ) \n
            // sigma_comms_6.x \n
                mstore( \n
                mload(add(vk, 0xe0)), \n
                {:#x} \n
            ) \n
            // sigma_comms_6.y \n
            mstore( \n
                add(mload(add(vk, 0xe0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_1.x \n
            mstore( \n
                mload(add(vk, 0x100)), \n  
                {:#x} \n
            ) \n
            // selector_comms_1.y \n
            mstore( \n
                add(mload(add(vk, 0x100)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_2.x \n
            mstore( \n
                mload(add(vk, 0x120)), \n
                {:#x} \n
            ) \n
            // selector_comms_2.y \n
            mstore( \n
                add(mload(add(vk, 0x120)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_3.x \n
            mstore( \n
                mload(add(vk, 0x140)), \n
                {:#x} \n
            ) \n
            // selector_comms_3.y \n
            mstore( \n
                add(mload(add(vk, 0x140)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_4.x \n
            mstore( \n
                mload(add(vk, 0x160)), \n
                {:#x} \n
            ) \n
            // selector_comms_4.y \n
            mstore( \n
                add(mload(add(vk, 0x160)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_5.x \n
            mstore( \n
                mload(add(vk, 0x180)), \n
                {:#x} \n
            ) \n
            // selector_comms_5.y \n
            mstore( \n
                add(mload(add(vk, 0x180)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_6.x \n
            mstore( \n
                mload(add(vk, 0x1a0)), \n
                {:#x} \n
            ) \n
            // selector_comms_6.y \n
            mstore( \n
                add(mload(add(vk, 0x1a0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_7.x \n
            mstore( \n
                mload(add(vk, 0x1c0)), \n
                {:#x} \n
            ) \n
            // selector_comms_7.y \n
            mstore( \n
                add(mload(add(vk, 0x1c0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_8.x \n
            mstore( \n
                mload(add(vk, 0x1e0)), \n
                {:#x} \n
            ) \n
            // selector_comms_8.y \n
            mstore( \n
                add(mload(add(vk, 0x1e0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_9.x \n
            mstore( \n
                mload(add(vk, 0x200)), \n
                {:#x} \n
            ) \n
            // selector_comms_9.y \n
            mstore( \n
                add(mload(add(vk, 0x200)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_10.x \n
            mstore( \n
                mload(add(vk, 0x220)), \n
                {:#x} \n
            ) \n
            // selector_comms_10.y \n
            mstore( \n
                add(mload(add(vk, 0x220)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_11.x \n
            mstore( \n
                mload(add(vk, 0x240)), \n
                {:#x} \n
            ) \n
            // selector_comms_11.y \n
            mstore( \n
                add(mload(add(vk, 0x240)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_12.x \n
            mstore( \n
                mload(add(vk, 0x260)), \n
                {:#x} \n
            ) \n
            // selector_comms_12.y \n
            mstore( \n
                add(mload(add(vk, 0x260)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_13.x \n
            mstore( \n
                mload(add(vk, 0x280)), \n
                {:#x} \n
            ) \n
            // selector_comms_13.y \n
            mstore( \n
                add(mload(add(vk, 0x280)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_14.x \n
            mstore( \n
                mload(add(vk, 0x2a0)), \n
                {:#x} \n
            ) \n
            // selector_comms_14.y \n
            mstore( \n
                add(mload(add(vk, 0x2a0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_15.x \n
            mstore( \n
                mload(add(vk, 0x2c0)), \n
                {:#x} \n
            ) \n
            // selector_comms_15.y \n
            mstore( \n
                add(mload(add(vk, 0x2c0)), 0x20), \n
                {:#x} \n  
            ) \n
            // selector_comms_16.x \n
            mstore( \n
                mload(add(vk, 0x2e0)), \n
                {:#x} \n
            ) \n
            // selector_comms_16.y \n
            mstore( \n
                add(mload(add(vk, 0x2e0)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_17.x \n
            mstore( \n
                mload(add(vk, 0x300)), \n
                {:#x} \n
            ) \n
            // selector_comms_17.y \n
            mstore( \n
                add(mload(add(vk, 0x300)), 0x20), \n
                {:#x} \n
            ) \n
            // selector_comms_18.x \n
            mstore( \n
                mload(add(vk, 0x320)), \n
                {:#x} \n
            ) \n
            // selector_comms_18.y \n
            mstore( \n
                add(mload(add(vk, 0x320)), 0x20), \n
                {:#x} \n
            ) \n
            // k1 \n
            mstore( \n
                add(vk, 0x340), \n
                {:#x}) \n
            // k2 \n
            mstore( \n
                add(vk, 0x360), \n
                {:#x} \n
            ) \n
            // k3 \n
            mstore( \n
                add(vk, 0x380), \n
                {:#x} \n
            ) \n
            // k4 \n
            mstore( \n
                add(vk, 0x3a0), \n
                {:#x} \n
            ) \n
            // k5 \n
            mstore( \n
                add(vk, 0x3c0), \n
                {:#x} \n
            ) \n
            // k6 \n
            mstore( \n
                add(vk, 0x3e0), \n
                {:#x} \n
            ) \n
            // range_table_comm.x \n
            mstore( \n
                mload(add(vk, 0x400)), \n
                {:#x} \n
            ) \n
            // range_table_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x400)), 0x20), \n
                {:#x} \n
            ) \n
            // key_table_comm.x \n
            mstore( \n
                mload(add(vk, 0x420)), \n
                {:#x} \n
            ) \n
            // key_table_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x420)), 0x20), \n
                {:#x} \n
            ) \n
            // table_dom_sep_comm.x \n
            mstore( \n
                mload(add(vk, 0x440)), \n
                {:#x} \n
            ) \n
            // table_dom_sep_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x440)), 0x20), \n
                {:#x} \n
            ) \n
            // q_dom_sep_comm.x \n
            mstore( \n
                mload(add(vk, 0x460)), \n
                {:#x} \n
            ) \n
            // q_dom_sep_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x460)), 0x20), \n
                {:#x} \n
            ) \n
            // size_inv \n
            mstore( \n
                add(vk, 0x480), \n
                {:#x} \n
            ) \n
            // group_gen \n
            mstore( \n
                add(vk, 0x4a0), \n
                {:#x} \n
            ) \n
            // group_gen_inv \n
            mstore( \n
                add(vk, 0x4c0), \n
                {:#x} \n
            ) \n
            // open_key_g.x \n
            mstore( \n
                mload(add(vk, 0x4e0)),  \n
                {:#x} \n
            ) \n
            // open_key_g.y \n
            mstore( \n
                add(mload(add(vk, 0x4e0)), 0x20),  \n
                {:#x}
            ) \n
            // open_key_h \n
            let h_ptr := mload(0x40) \n
            mstore(add(vk, 0x500), h_ptr) \n
            mstore(h_ptr, {:#x}) // x0  \n
            mstore(add(h_ptr, 0x20), {:#x}) // x1  \n
            mstore(add(h_ptr, 0x40), {:#x}) // y0  \n
            mstore(add(h_ptr, 0x60), {:#x}) // y1  \n
            mstore(0x40, add(h_ptr, 0x80)) \n
            // open_key_beta_h \n
            let beta_h_ptr := mload(0x40) \n
            mstore(add(vk, 0x520), beta_h_ptr) \n
            mstore(beta_h_ptr, {:#x}) // x0 \n
            mstore(add(beta_h_ptr, 0x20), {:#x}) // x1 \n
            mstore(add(beta_h_ptr, 0x40), {:#x}) // y0 \n
            mstore(add(beta_h_ptr, 0x60), {:#x}) // y1 \n
            mstore(0x40, add(beta_h_ptr, 0x80)) \n 
            }}\n
            return vk; \n
            
        }}\n
    }}",
        extra_line,
        import_path,
        domain_size_u256,           // domain_size
        num_inputs_u256,            // num_inputs
        sigma_comms_u256[0],        // sigma_comms_1.x
        sigma_comms_u256[1],        // sigma_comms_1.y
        sigma_comms_u256[2],        // sigma_comms_2.x
        sigma_comms_u256[3],        // sigma_comms_2.y
        sigma_comms_u256[4],        // sigma_comms_3.x
        sigma_comms_u256[5],        // sigma_comms_3.y
        sigma_comms_u256[6],        // sigma_comms_4.x
        sigma_comms_u256[7],        // sigma_comms_4.y
        sigma_comms_u256[8],        // sigma_comms_5.x
        sigma_comms_u256[9],        // sigma_comms_5.y
        sigma_comms_u256[10],       // sigma_comms_6.x
        sigma_comms_u256[11],       // sigma_comms_6.y
        selector_comms_u256[0],     // selector_comms_1.x
        selector_comms_u256[1],     // selector_comms_1.y
        selector_comms_u256[2],     // selector_comms_2.x
        selector_comms_u256[3],     // selector_comms_2.y
        selector_comms_u256[4],     // selector_comms_3.x
        selector_comms_u256[5],     // selector_comms_3.y
        selector_comms_u256[6],     // selector_comms_4.x
        selector_comms_u256[7],     // selector_comms_4.y
        selector_comms_u256[8],     // selector_comms_5.x
        selector_comms_u256[9],     // selector_comms_5.y
        selector_comms_u256[10],    // selector_comms_6.x
        selector_comms_u256[11],    // selector_comms_6.y
        selector_comms_u256[12],    // selector_comms_7.x
        selector_comms_u256[13],    // selector_comms_7.y
        selector_comms_u256[14],    // selector_comms_8.x
        selector_comms_u256[15],    // selector_comms_8.y
        selector_comms_u256[16],    // selector_comms_9.x
        selector_comms_u256[17],    // selector_comms_9.y
        selector_comms_u256[18],    // selector_comms_10.x
        selector_comms_u256[19],    // selector_comms_10.y
        selector_comms_u256[20],    // selector_comms_11.x
        selector_comms_u256[21],    // selector_comms_11.y
        selector_comms_u256[22],    // selector_comms_12.x
        selector_comms_u256[23],    // selector_comms_12.y
        selector_comms_u256[24],    // selector_comms_13.x
        selector_comms_u256[25],    // selector_comms_13.y
        selector_comms_u256[26],    // selector_comms_14.x
        selector_comms_u256[27],    // selector_comms_14.y
        selector_comms_u256[28],    // selector_comms_15.x
        selector_comms_u256[29],    // selector_comms_15.y
        selector_comms_u256[30],    // selector_comms_16.x
        selector_comms_u256[31],    // selector_comms_16.y
        selector_comms_u256[32],    // selector_comms_17.x
        selector_comms_u256[33],    // selector_comms_17.y
        selector_comms_u256[34],    // selector_comms_18.x
        selector_comms_u256[35],    // selector_comms_18.y
        ks_u256[0],                 // k1
        ks_u256[1],                 // k2
        ks_u256[2],                 // k3
        ks_u256[3],                 // k4
        ks_u256[4],                 // k5
        ks_u256[5],                 // k6
        range_table_comm_u256[0],   // range_table_comm.x
        range_table_comm_u256[1],   // range_table_comm.y
        key_table_comm_u256[0],     // key_table_comm.x
        key_table_comm_u256[1],     // key_table_comm.y
        table_dom_sep_comm_u256[0], // table_dom_sep_comm.x
        table_dom_sep_comm_u256[1], // table_dom_sep_comm.y
        q_dom_sep_comm_u256[0],     // q_dom_sep_comm.x
        q_dom_sep_comm_u256[1],     // q_dom_sep_comm.y
        size_inv,                   // size_inv
        group_gen,                  // group_gen
        group_gen_inv,              // group_gen_inv
        open_key_g[0],              // open_key.x
        open_key_g[1],              // open_key.y
        h[0],                       // h.x1
        h[1],                       // h.x2
        h[2],                       // h.y1
        h[3],                       // h.y2
        beta_h[0],                  // beta_h.x1
        beta_h[1],                  // beta_h.x2
        beta_h[2],                  // beta_h.y1
        beta_h[3],                  // beta_h.y2
    ))
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use ethers::types::Bytes;
    use jf_plonk::{
        proof_system::{PlonkKzgSnark, UniversalSNARK},
        transcript::SolidityTranscript,
    };
    use jf_relation::{Arithmetization, Circuit, PlonkCircuit};

    use ark_bn254::Bn254;
    use ark_ed_on_bn254::Fq;
    use ark_serialize::Write;
    use ark_std::vec::Vec;
    use configuration::settings::Settings;
    use jf_utils::test_rng;

    #[test]
    fn test_verifier_contract() {
        let settings: Settings = Settings::new().unwrap();
        let mut circuit = PlonkCircuit::<Fq>::new_ultra_plonk(8);
        let _ = circuit.create_public_variable(Fq::from(2)).unwrap();

        circuit.finalize_for_arithmetization().unwrap();
        let mut rng = test_rng();
        let srs_size = circuit.srs_size().unwrap();
        let srs = PlonkKzgSnark::<Bn254>::universal_setup_for_testing(srs_size, &mut rng).unwrap();
        let (pk, vk) = PlonkKzgSnark::<Bn254>::preprocess(&srs, &circuit).unwrap();
        create_vk_contract::<true>(&vk, &settings);

        let proof = PlonkKzgSnark::<Bn254>::prove::<_, _, SolidityTranscript>(
            &mut rng, &circuit, &pk, None,
        )
        .unwrap();

        // convert proof to bytes
        let proof_vec: Vec<Fq254> = proof.clone().into();
        let proof_bytes = proof_vec
            .into_iter()
            .flat_map(|x| {
                let mut bytes: Vec<u8> = x.into_bigint().to_bytes_le();
                bytes.resize(32, 0u8);
                bytes.reverse();
                bytes
            })
            .collect::<Vec<u8>>();

        let proof_bytes_final = Bytes::from(proof_bytes);
        let join_path = Path::new(&settings.contracts.assets)
            .parent()
            .unwrap()
            .join("contracts/Nightfall.sol");

        let path_out: PathBuf;
        let cwd = std::env::current_dir().unwrap();
        let mut cwd = cwd.as_path();
        loop {
            let file_path = cwd.join(&join_path);
            if file_path.is_file() {
                path_out = file_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("test_contracts/ultraPlonkProof.json");
                break;
            }

            cwd = cwd.parent().ok_or("No parent in path").unwrap();
        }

        if let Some(parent) = path_out.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("Failed to create test_contracts folder: {e}"));
        }
        let mut file = std::fs::File::create(&path_out).unwrap();
        write!(file, "{}", hex::encode(&proof_bytes_final)).unwrap();
        file.flush().unwrap();
        file.sync_all().unwrap();

        // We run `forge test` now to update all the contracts
        let output = std::process::Command::new("forge")
            .args(["test"])
            .output()
            .unwrap();
        match output.status.code() {
            Some(0) => (),
            Some(code) => panic!(
                "Forge failed with code {}, stdout is: {}, stderr is : {}",
                code,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
            None => panic!("Forge failed with no exit code"),
        }
    }
}
