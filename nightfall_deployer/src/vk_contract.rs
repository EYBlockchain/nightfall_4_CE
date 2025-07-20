use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{domain, EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use configuration::settings::Settings;
use ethers::types::{H256, U256};
use jf_plonk::proof_system::structs::{VerifyingKey, VK};

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_vk_contract<const TEST: bool>(vk: &VerifyingKey<Bn254>, settings: &Settings) {
    let vk_hash_bytes: [u8; 32] = vk.hash().into_bigint().to_bytes_be().try_into().unwrap();
    let vk_hash = H256::from(vk_hash_bytes);
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

    let omega = U256::from_little_endian(&domain.group_gen().into_bigint().to_bytes_le());
    let omega_inv = U256::from_little_endian(&domain.group_gen_inv().into_bigint().to_bytes_le());
    ark_std::println!(
        "creating vk contract:vk:{:?} ",
        vk.clone()
    );

    let domain_size_U256 = U256::from(domain_size as u32);
    let num_inputs_U256 = U256::from(vk.num_inputs() as u32);
    let sigma_comms_U256: Vec<U256> = vk.sigma_comms
    .iter()
    .flat_map(|comm| {
        let x = U256::from_big_endian(&comm.x.into_bigint().to_bytes_be());
        let y = U256::from_big_endian(&comm.y.into_bigint().to_bytes_be());
        vec![x, y]
    })
    .collect();
    let selector_comms_U256: Vec<U256> = vk
        .selector_comms
        .iter()
        .flat_map(|comm| {
            let x = U256::from_big_endian(&comm.x.into_bigint().to_bytes_be());
            let y = U256::from_big_endian(&comm.y.into_bigint().to_bytes_be());
            vec![x, y]
        })
        .collect();
    let ks_u256: Vec<U256> = vk
        .k
        .iter()
        .map(|k| U256::from_big_endian(&k.into_bigint().to_bytes_be()))
        .collect();

    let vk_vec = Vec::<Fq254>::from(vk.clone())
        .into_iter()
        .map(|x| U256::from_little_endian(&x.into_bigint().to_bytes_le()))
        .collect::<Vec<_>>();
    ark_std::println!(
        "creating vk contract:vk_vec:{:?} ",
        vk_vec.clone()
    );
    let vk_vec_u256 = Vec::<Fq254>::from(vk.clone())
    .into_iter()
    .map(|x| U256::from_little_endian(&x.into_bigint().to_bytes_le()))
    .collect::<Vec<_>>();
let range_table_comm_u256 = vec![
    vk_vec_u256[56].clone(), // x
    vk_vec_u256[57].clone(), // y
];
let key_table_comm_u256 = vec![
    vk_vec_u256[58].clone(), // x
    vk_vec_u256[59].clone(), // y
];
let table_dom_sep_comm_u256 = vec![
    vk_vec_u256[60].clone(), // x
    vk_vec_u256[61].clone(), // y
];
let q_dom_sep_comm_u256 = vec![
    vk_vec_u256[60].clone(), // x
    vk_vec_u256[61].clone(), // y
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
                ark_std::println!("Creating contract proof_verification/VerificationKeyJJ.sol");
                file_path
                    .parent()
                    .unwrap()
                    .join("proof_verification/VerificationKeyJJ.sol")
            } else {
                ark_std::println!("Creating contract test_contracts/TestVerificationKeyJJ.sol");
                file_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("test_contracts/TestVerificationKeyJJ.sol")
            };
            break;
        }

        cwd = cwd.parent().ok_or("No parent in path").unwrap();
    }

    if path_out.is_file() {
        std::fs::remove_file(&path_out).unwrap();
    }

    let mut file: File = File::create(path_out).unwrap();
    let import_path = "./Types.sol";
    file.write_fmt(format_args!(
    "// SPDX-License-Identifier: Apache-2.0 \n
    pragma solidity ^0.8.20; \n
    import \"{}\"; \n
        
    library {{UltraPlonkVerificationKey}} {{ \n

        function getVerificationKey() internal pure returns (Types.VerificationKey memory vk) {{ \n
            assembly {{ \n
            // domain_size
            mstore(add(vk, 0x00), {}) \n
            // num_inputs \n
            mstore(add(vk, 0x20), {}) \n
            // sigma_comms_1.x \n
            mstore( \n
                mload(add(vk, 0x40)), \n
                {} \n
            ) \n
            // sigma_comms_1.y \n
            mstore( \n
                add(mload(add(vk, 0x40)), 0x20), \n
                {} \n
            ) \n
            // sigma_comms_2.x \n
            mstore( \n
                mload(add(vk, 0x60)), \n
                {} \n
            ) \n
            // sigma_comms_2.y \n
            mstore( \n
                add(mload(add(vk, 0x60)), 0x20), \n
                {} \n
            ) \n
            // sigma_comms_3.x \n
            mstore( \n
                mload(add(vk, 0x80)), \n
                {} \n
            ) \n
            // sigma_comms_3.y \n
            mstore( \n
                add(mload(add(vk, 0x80)), 0x20), \n
                {} \n
            ) \n
            // sigma_comms_4.x \n
            mstore( \n
                mload(add(vk, 0xa0)), \n
                {} \n
            ) \n
            // sigma_comms_4.y \n 
            mstore( \n
                add(mload(add(vk, 0xa0)), 0x20), \n
                {} \n
            ) \n
            // sigma_comms_5.x \n
            mstore( \n
                mload(add(vk, 0xc0)), \n
                {} \n
            ) \n
            // sigma_comms_5.y \n
            mstore( \n
                add(mload(add(vk, 0xc0)), 0x20), \n
                {} \n
            ) \n
            // sigma_comms_6.x \n
                mstore( \n
                mload(add(vk, 0xe0)), \n
                {} \n
            ) \n
            // sigma_comms_6.y \n
            mstore( \n
                add(mload(add(vk, 0xe0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_1.x \n
            mstore( \n
                mload(add(vk, 0x100)), \n  
                {} \n
            ) \n
            // selector_comms_1.y \n
            mstore( \n
                add(mload(add(vk, 0x100)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_2.x \n
            mstore( \n
                mload(add(vk, 0x120)), \n
                {} \n
            ) \n
            // selector_comms_2.y \n
            mstore( \n
                add(mload(add(vk, 0x120)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_3.x \n
            mstore( \n
                mload(add(vk, 0x140)), \n
                {} \n
            ) \n
            // selector_comms_3.y \n
            mstore( \n
                add(mload(add(vk, 0x140)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_4.x \n
            mstore( \n
                mload(add(vk, 0x160)), \n
                {} \n
            ) \n
            // selector_comms_4.y \n
            mstore( \n
                add(mload(add(vk, 0x160)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_5.x \n
            mstore( \n
                mload(add(vk, 0x180)), \n
                {} \n
            ) \n
            // selector_comms_5.y \n
            mstore( \n
                add(mload(add(vk, 0x180)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_6.x \n
            mstore( \n
                mload(add(vk, 0x1a0)), \n
                {} \n
            ) \n
            // selector_comms_6.y \n
            mstore( \n
                add(mload(add(vk, 0x1a0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_7.x \n
            mstore( \n
                mload(add(vk, 0x1c0)), \n
                {} \n
            ) \n
            // selector_comms_7.y \n
            mstore( \n
                add(mload(add(vk, 0x1c0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_8.x \n
            mstore( \n
                mload(add(vk, 0x1e0)), \n
                {} \n
            ) \n
            // selector_comms_8.y \n
            mstore( \n
                add(mload(add(vk, 0x1e0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_9.x \n
            mstore( \n
                mload(add(vk, 0x200)), \n
                {} \n
            ) \n
            // selector_comms_9.y \n
            mstore( \n
                add(mload(add(vk, 0x200)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_10.x \n
            mstore( \n
                mload(add(vk, 0x220)), \n
                {} \n
            ) \n
            // selector_comms_10.y \n
            mstore( \n
                add(mload(add(vk, 0x220)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_11.x \n
            mstore( \n
                mload(add(vk, 0x240)), \n
                {} \n
            ) \n
            // selector_comms_11.y \n
            mstore( \n
                add(mload(add(vk, 0x240)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_12.x \n
            mstore( \n
                mload(add(vk, 0x260)), \n
                {} \n
            ) \n
            // selector_comms_12.y \n
            mstore( \n
                add(mload(add(vk, 0x260)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_13.x \n
            mstore( \n
                mload(add(vk, 0x280)), \n
                {} \n
            ) \n
            // selector_comms_13.y \n
            mstore( \n
                add(mload(add(vk, 0x280)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_14.x \n
            mstore( \n
                mload(add(vk, 0x2a0)), \n
                {} \n
            ) \n
            // selector_comms_14.y \n
            mstore( \n
                add(mload(add(vk, 0x2a0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_15.x \n
            mstore( \n
                mload(add(vk, 0x2c0)), \n
                {} \n
            ) \n
            // selector_comms_15.y \n
            mstore( \n
                add(mload(add(vk, 0x2c0)), 0x20), \n
                {} \n  
            ) \n
            // selector_comms_16.x \n
            mstore( \n
                mload(add(vk, 0x2e0)), \n
                {} \n
            ) \n
            // selector_comms_16.y \n
            mstore( \n
                add(mload(add(vk, 0x2e0)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_17.x \n
            mstore( \n
                mload(add(vk, 0x300)), \n
                {} \n
            ) \n
            // selector_comms_17.y \n
            mstore( \n
                add(mload(add(vk, 0x300)), 0x20), \n
                {} \n
            ) \n
            // selector_comms_18.x \n
            mstore( \n
                mload(add(vk, 0x320)), \n
                {} \n
            ) \n
            // selector_comms_18.y \n
            mstore( \n
                add(mload(add(vk, 0x320)), 0x20), \n
                {} \n
            ) \n
            // k1 \n
            mstore(add(vk, 0x340), 1) \n
            // k2 \n
            mstore( \n
                add(vk, 0x360), \n
                {} \n
            ) \n
            // k3 \n
            mstore( \n
                add(vk, 0x380), \n
                {} \n
            ) \n
            // k4 \n
            mstore( \n
                add(vk, 0x3a0), \n
                {} \n
            ) \n
            // k5 \n
            mstore( \n
                add(vk, 0x3c0), \n
                {} \n
            ) \n
            // k6 \n
            mstore( \n
                add(vk, 0x3e0), \n
                {} \n
            ) \n
            // range_table_comm.x \n
            mstore( \n
                mload(add(vk, 0x400)), \n
                {} \n
            ) \n
            // range_table_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x400)), 0x20), \n
                {} \n
            ) \n
            // key_table_comm.x \n
            mstore( \n
                mload(add(vk, 0x420)), \n
                {} \n
            ) \n
            // key_table_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x420)), 0x20), \n
                {} \n
            ) \n
            // table_dom_sep_comm.x \n
            mstore( \n
                mload(add(vk, 0x440)), \n
                {} \n
            ) \n
            // table_dom_sep_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x440)), 0x20), \n
                {} \n
            ) \n
            // q_dom_sep_comm.x \n
            mstore( \n
                mload(add(vk, 0x460)), \n
                {} \n
            ) \n
            // q_dom_sep_comm.y \n
            mstore( \n
                add(mload(add(vk, 0x460)), 0x20), \n
                {} \n
            ) \n
            }}\n
        }}",
        import_path,
        domain_size_U256, // domain_size
        num_inputs_U256, // num_inputs
        sigma_comms_U256[0], // sigma_comms_1.x
        sigma_comms_U256[1], // sigma_comms_1.y
        sigma_comms_U256[3], // sigma_comms_2.x
        sigma_comms_U256[4], // sigma_comms_2.y
        sigma_comms_U256[5], // sigma_comms_3.x
        sigma_comms_U256[6], // sigma_comms_3.y
        sigma_comms_U256[7], // sigma_comms_4.x
        sigma_comms_U256[8], // sigma_comms_4.y
        sigma_comms_U256[9],// sigma_comms_5.x
        sigma_comms_U256[10],// sigma_comms_5.y
        sigma_comms_U256[11],// sigma_comms_6.x
        sigma_comms_U256[12],// sigma_comms_6.y
        selector_comms_U256[0],// selector_comms_1.x
        selector_comms_U256[1],// selector_comms_1.y
        selector_comms_U256[2],// selector_comms_2.x
        selector_comms_U256[3],// selector_comms_2.y
        selector_comms_U256[4],// selector_comms_3.x
        selector_comms_U256[5],// selector_comms_3.y
        selector_comms_U256[6],// selector_comms_4.x
        selector_comms_U256[7],// selector_comms_4.y
        selector_comms_U256[8],// selector_comms_5.x
        selector_comms_U256[9],// selector_comms_5.y
        selector_comms_U256[10],// selector_comms_6.x
        selector_comms_U256[11],// selector_comms_6.y
        selector_comms_U256[12],// selector_comms_7.x
        selector_comms_U256[13],// selector_comms_7.y
        selector_comms_U256[14],// selector_comms_8.x
        selector_comms_U256[15],// selector_comms_8.y
        selector_comms_U256[16],// selector_comms_9.x
        selector_comms_U256[17],// selector_comms_9.y
        selector_comms_U256[18],// selector_comms_10.x
        selector_comms_U256[19],// selector_comms_10.y
        selector_comms_U256[20],// selector_comms_11.x
        selector_comms_U256[21],// selector_comms_11.y
        selector_comms_U256[22],// selector_comms_12.x
        selector_comms_U256[23],// selector_comms_12.y
        selector_comms_U256[24],// selector_comms_13.x
        selector_comms_U256[25],// selector_comms_13.y
        selector_comms_U256[26],// selector_comms_14.x
        selector_comms_U256[27],// selector_comms_14.y
        selector_comms_U256[28],// selector_comms_15.x
        selector_comms_U256[29],// selector_comms_15.y
        selector_comms_U256[30],// selector_comms_16.x
        selector_comms_U256[31],// selector_comms_16.y
        selector_comms_U256[32],// selector_comms_17.x
        selector_comms_U256[33],// selector_comms_17.y
        selector_comms_U256[34],// selector_comms_18.x
        selector_comms_U256[35],// selector_comms_18.y
        ks_u256[0],// k1
        ks_u256[1],// k2
        ks_u256[2],// k3
        ks_u256[3],// k4
        ks_u256[4],// k5
        ks_u256[5],// k6
        range_table_comm_u256[0],// range_table_comm.x
        range_table_comm_u256[1],// range_table_comm.y
        key_table_comm_u256[0],// key_table_comm.x
       key_table_comm_u256[1],// key_table_comm.y
        table_dom_sep_comm_u256[0],// table_dom_sep_comm.x
        table_dom_sep_comm_u256[1],// table_dom_sep_comm.y
        q_dom_sep_comm_u256[0],// q_dom_sep_comm.x
        q_dom_sep_comm_u256[1],// q_dom_sep_comm.y
    ))
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use ethers::types::Bytes;

    use jf_plonk::{
        errors::PlonkError,
        proof_system::{PlonkKzgSnark, UniversalSNARK},
        transcript::SolidityTranscript,
    };
    use jf_relation::{Arithmetization, Circuit, PlonkCircuit};

    use ark_bn254::{Bn254, Fr as Fr254};
    use ark_ff::One;

    use ark_std::vec::Vec;
    use configuration::settings::Settings;

    #[test]
    fn test_verifier_contract() {
        let settings: Settings = Settings::new().unwrap();
        let mut rng = jf_utils::test_rng();

        let circuit = gen_circuit_for_test(2, 2).unwrap();

        let srs_size = circuit.srs_size().unwrap();
        let srs = PlonkKzgSnark::<Bn254>::universal_setup_for_testing(srs_size, &mut rng).unwrap();
        let (pk, vk) = PlonkKzgSnark::<Bn254>::preprocess(&srs, &circuit).unwrap();
        create_vk_contract::<true>(&vk, &settings);
        ark_std::println!("vk:{:?}", vk);

        // let proof = PlonkKzgSnark::<Bn254>::prove::<_, _, SolidityTranscript>(
        //     &mut rng, &circuit, &pk, None,
        // )
        // .unwrap();
        // let proof_vec = Vec::<Fq254>::from(proof)
        //     .into_iter()
        //     .flat_map(|x| {
        //         let mut bytes: Vec<u8> = x.into_bigint().to_bytes_le();
        //         bytes.resize(32, 0u8);
        //         bytes.reverse();
        //         bytes
        //     })
        //     .collect::<Vec<u8>>();

        // let bytes = Bytes::from_iter(proof_vec);

        // let join_path = Path::new(&settings.contracts.assets)
        //     .parent()
        //     .unwrap()
        //     .join("contracts/Nightfall.sol");

        // let path_out: PathBuf;
        // let cwd = std::env::current_dir().unwrap();
        // let mut cwd = cwd.as_path();
        // loop {
        //     let file_path = cwd.join(&join_path);
        //     if file_path.is_file() {
        //         path_out = file_path
        //             .parent()
        //             .unwrap()
        //             .parent()
        //             .unwrap()
        //             .join("test_contracts/testproof.txt");
        //         break;
        //     }

        //     cwd = cwd.parent().ok_or("No parent in path").unwrap();
        // }
        // let mut file = File::create(path_out.clone()).unwrap();
        // file.write_fmt(format_args!("{:0x}", bytes)).unwrap();

        // // We run `forge test` now to update all the contracts
        // let output = std::process::Command::new("forge")
        //     .args(["test", "--force", "--match-test", "testRollupVerifier"])
        //     .output()
        //     .unwrap();
        // //    std::fs::remove_file(path_out).unwrap();

        // match output.status.code() {
        //     Some(0) => (),
        //     Some(code) => panic!(
        //         "Forge failed with code {}, stdout is: {}, stderr is : {}",
        //         code,
        //         String::from_utf8_lossy(&output.stdout),
        //         String::from_utf8_lossy(&output.stderr)
        //     ),
        //     None => panic!("Forge failed with no exit code"),
        // }
    }

    pub(crate) fn gen_circuit_for_test(
        m: usize,
        a0: usize,
    ) -> Result<PlonkCircuit<Fr254>, PlonkError> {
        let mut cs = PlonkCircuit::<Fr254>::new_ultra_plonk(5);
        // Create variables
        let mut a = vec![];
        for i in a0..(a0 + 4 * m) {
            a.push(cs.create_variable(Fr254::from(i as u64))?);
        }
        let b = [
            cs.create_public_variable(Fr254::from(m as u64 * 2))?,
            cs.create_public_variable(Fr254::from(a0 as u64 * 2 + m as u64 * 4 - 1))?,
        ];
        let c = cs.create_public_variable(
            (cs.witness(b[1])? + cs.witness(a[0])?) * (cs.witness(b[1])? - cs.witness(a[0])?),
        )?;

        // Create gates:
        // 1. a0 + ... + a_{4*m-1} = b0 * b1
        // 2. (b1 + a0) * (b1 - a0) = c
        // 3. b0 = 2 * m
        let mut acc = cs.zero();
        a.iter().for_each(|&elem| acc = cs.add(acc, elem).unwrap());
        let b_mul = cs.mul(b[0], b[1])?;
        cs.enforce_equal(acc, b_mul)?;
        let b1_plus_a0 = cs.add(b[1], a[0])?;
        let b1_minus_a0 = cs.sub(b[1], a[0])?;
        cs.mul_gate(b1_plus_a0, b1_minus_a0, c)?;
        cs.enforce_constant(b[0], Fr254::from(m as u64 * 2))?;

        // Create range gates
        // 1. range_table = {0, 1, ..., 31}
        // 2. a_i \in range_table for i = 0..m-1
        // 3. b0 \in range_table
        for &var in a.iter().take(m) {
            cs.add_range_check_variable(var)?;
        }
        cs.add_range_check_variable(b[0])?;

        // Create variable table lookup gates
        // 1. table = [(a0, a2), (a1, a3), (b0, a0)]
        let table_vars = [(a[0], a[2]), (a[1], a[3]), (b[0], a[0])];
        // 2. lookup_witness = [(1, a0+1, a0+3), (2, 2m, a0)]
        let key0 = cs.one();
        let key1 = cs.create_variable(Fr254::from(2u8))?;
        let two_m = cs.create_public_variable(Fr254::from(m as u64 * 2))?;
        let a1 = cs.add_constant(a[0], &Fr254::one())?;
        let a3 = cs.add_constant(a[0], &Fr254::from(3u8))?;
        let lookup_vars = [(key0, a1, a3), (key1, two_m, a[0])];
        cs.create_table_and_lookup_variables(&lookup_vars, &table_vars)?;

        // Finalize the circuit.
        cs.finalize_for_arithmetization()?;
        Ok(cs)
    }
}
