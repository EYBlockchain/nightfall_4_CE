use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use configuration::settings::Settings;
use alloy::{primitives::{B256, U256}};
use jf_plonk::proof_system::structs::{VerifyingKey, VK};

use std::{
    fs::File,
    io::{Write},
    path::{Path, PathBuf},
};

pub fn create_vk_contract<const TEST: bool>(vk: &VerifyingKey<Bn254>, settings: &Settings) {
    let vk_hash_bytes: [u8; 32] = vk.hash()
        .into_bigint()
        .to_bytes_be()
        .try_into()
        .expect("Failed to convert Vec<u8> to [u8; 32]");
    println!("vk_hash_bytes: {:?}", vk_hash_bytes);
    let vk_hash = B256::from_slice(&vk_hash_bytes);
    println!("vk_hash: {:?}", vk_hash);
    let domain_size = vk.domain_size();
    let domain_size_fr = Fr254::from(domain_size as u32);
    let domain_size_inv = U256::from_le_bytes::<32>(
        domain_size_fr
            .inverse()
            .unwrap()
            .into_bigint()
            .to_bytes_le().try_into().expect("Failed to convert Vec<u8> to [u8; 32]"),
    );
    let domain = Radix2EvaluationDomain::<Fr254>::new(domain_size).unwrap();
    let omega = U256::from_le_bytes::<32>(domain.group_gen().into_bigint().to_bytes_le().try_into().expect("Failed to convert Vec<u8> to [u8; 32]"));
    let omega_inv = U256::from_le_bytes::<32>(domain.group_gen_inv().into_bigint().to_bytes_le().try_into().expect("Failed to convert Vec<u8> to [u8; 32]"));
    let vk_vec = Vec::<Fq254>::from(vk.clone())
        .into_iter()
        .map(|x| U256::from_le_bytes::<32>(x.into_bigint().to_bytes_le().try_into().expect("Failed to convert Vec<u8> to [u8; 32]")))
        .collect::<Vec<_>>();

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
                    .join("proof_verification/VerificationKey.sol")
            } else {
                file_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("test_contracts/TestVerificationKey.sol")
            };
            break;
        }

        cwd = cwd.parent().ok_or("No parent in path").unwrap();
    }

    if path_out.is_file() {
        std::fs::remove_file(&path_out).unwrap();
    }

    let mut file: File = File::create(path_out).unwrap();

    let name = if !TEST { "UltraVK" } else { "TestVK" };

    file.write_fmt(format_args!(
        "// SPDX-License-Identifier: Apache-2.0 \n
        pragma solidity >=0.8.4; \n
        
        library {name} {{ \n

            function getVerificationKeyHash() internal pure returns (bytes32) {{ \n
        return {:#x};
    }} \n

           function loadVerificationKey( \n
             uint256 _vk, \n
               uint256 _second_loc \n
           ) internal pure {{ \n
              assembly {{ \n
                 mstore( 
                      add(_vk, 0x00), 
                      {:#x} 
                  ) // DOMAIN_SIZE \n
                   mstore( \n
                       add(_vk, 0x20), \n
                  {:#x} \n
                    ) // NUM_PUBLIC_INPUTS \n
                    mstore(
                        add(_vk, 0x40),
                        {:#x}
                    ) // SIGMA1_X \n
                    mstore(
                        add(_vk, 0x60),
                        {:#x}
                    ) // SIGMA1_Y \n
                    mstore(
                        add(_vk, 0x80),
                        {:#x}
                    ) // SIGMA2_X \n
                    mstore(
                        add(_vk, 0xa0),
                        {:#x}
                    ) // SIGMA2_Y \n
                    mstore(
                        add(_vk, 0xc0),
                        {:#x}
                    ) // SIGMA3_X \n
                    mstore(
                        add(_vk, 0xe0),
                        {:#x}
                    ) // SIGMA3_Y \n
                    mstore(
                        add(_vk, 0x100),
                        {:#x}
                    ) // SIGMA4_X \n
                    mstore(
                        add(_vk, 0x120),
                        {:#x}
                    ) // SIGMA4_Y \n
                    mstore(
                        add(_vk, 0x140),
                        {:#x}
                    ) // SIGMA5_X \n
                    mstore(
                        add(_vk, 0x160),
                        {:#x}
                    ) // SIGMA5_Y \n
                    mstore(
                        add(_vk, 0x180),
                        {:#x}
                    ) // SIGMA6_X \n
                    mstore(
                        add(_vk, 0x1a0),
                        {:#x}
                    ) // SIGMA6_Y \n
                    mstore(
                        add(_vk, 0x1c0),
                        {:#x}
                    ) // QLC1_X \n
                    mstore(
                        add(_vk, 0x1e0),
                        {:#x}
                    ) // QLC1_Y \n
                    mstore(
                        add(_vk, 0x200),
                        {:#x}
                    ) // QLC2_X \n
                    mstore(
                        add(_vk, 0x220),
                        {:#x}
                    ) // QLC2_Y \n
                    mstore(
                        add(_vk, 0x240),
                        {:#x}
                    ) // QLC3_X \n
                    mstore(
                        add(_vk, 0x260),
                        {:#x}
                    ) // QLC3_Y \n
                    mstore(
                        add(_vk, 0x280),
                        {:#x}
                    ) // QLC4_X \n
                    mstore(
                        add(_vk, 0x2a0),
                        {:#x}
                    ) // QLC4_Y \n
                    mstore(
                        add(_vk, 0x2c0),
                        {:#x}
                    ) // QMUL1_X \n
                    mstore(
                        add(_vk, 0x2e0),
                        {:#x}
                    ) // QMUL1_Y \n
                    mstore(
                        add(_vk, 0x300),
                        {:#x}
                    ) // QMUL2_X \n
                    mstore(
                        add(_vk, 0x320),
                        {:#x}
                    ) // QMUL2_Y \n
                    mstore(
                        add(_vk, 0x340),
                        {:#x}
                    ) // QHASH1_X \n
                    mstore(
                        add(_vk, 0x360),
                        {:#x}
                    ) // QHASH1_Y \n
                    mstore(
                        add(_vk, 0x380),
                        {:#x}
                    ) // QHASH2_X \n
                    mstore(
                        add(_vk, 0x3a0),
                        {:#x}
                    ) // QHASH2_Y \n
                    mstore(
                        add(_vk, 0x3c0),
                        {:#x}
                    ) // QHASH3_X \n
                    mstore(
                        add(_vk, 0x3e0),
                        {:#x}
                    ) // QHASH3_Y \n
                    mstore(
                        add(_vk, 0x400),
                        {:#x}
                    ) // QHASH4_X \n
                    mstore(
                        add(_vk, 0x420),
                        {:#x}
                    ) // QHASH4_Y \n
                    mstore(
                        add(_vk, 0x440),
                        {:#x}
                    ) // QOUT_X \n
                    mstore(
                        add(_vk, 0x460),
                        {:#x}
                    ) // QOUT_Y \n
                    mstore(
                        add(_vk, 0x480),
                        {:#x}
                    ) // QCONST_X \n
                    mstore(
                        add(_vk, 0x4a0),
                        {:#x}
                    ) // QCONST_Y \n
                    mstore(
                        add(_vk, 0x4c0),
                        {:#x}
                    ) // QECC_X \n
                    mstore(
                        add(_vk, 0x4e0),
                        {:#x}
                    ) // QECC_Y \n
                    mstore(
                        add(_vk, 0x500),
                        {:#x}
                    ) // QSWX1_X \n
                    mstore(
                        add(_vk, 0x520),
                        {:#x}
                    ) // QSWX1_Y \n
                    mstore(
                        add(_vk, 0x540),
                        {:#x}
                    ) // QSWX2_X \n
                    mstore(
                        add(_vk, 0x560),
                        {:#x}
                    ) // QSWX2_Y \n
                    mstore(
                        add(_vk, 0x580),
                        {:#x}
                    ) // QSWY1_X \n
                    mstore(
                        add(_vk, 0x5a0),
                        {:#x}
                    ) // QSWY1_Y \n
                    mstore(
                        add(_vk, 0x5c0),
                        {:#x}
                    ) // QSWY2_X \n
                    mstore(
                        add(_vk, 0x5e0),
                        {:#x}
                    ) // QSWY2_Y \n
                    mstore(
                        add(_vk, 0x600),
                        {:#x} 
                    ) // QLOOKUP_X \n
                    mstore(
                        add(_vk, 0x620),
                        {:#x}
                    ) // QLOOKUP_Y \n
                    mstore(add(_vk, 0x640), {:#x}) // K1 \n
                    mstore(add(_vk, 0x660), {:#x}) // K2 \n
                    mstore(
                        add(_vk, 0x680),
                        {:#x}
                    ) // K3 \n
                    mstore(
                        add(_vk, 0x6a0),
                        {:#x}
                    ) // K4 \n
                    mstore(
                        add(_vk, 0x6c0),
                        {:#x}
                    ) // K5 \n
                    mstore(
                        add(_vk, 0x6e0),
                        {:#x}
                    ) // K6 \n
                     mstore(
                        add(_vk, 0x840),
                        {:#x}
                    ) // PLRANGE_X \n
                    mstore(
                        add(_vk, 0x860),
                        {:#x}
                    ) // PLRANGE_Y \n
                    mstore(
                        add(_vk, 0x880),
                        {:#x}
                    ) // PLKEY_X \n
                    mstore(
                        add(_vk, 0x8a0),
                        {:#x}
                    ) // PLKEY_Y \n
                    mstore(
                        add(_vk, 0x8c0),
                        {:#x}
                    ) // PLTDS_X \n
                    mstore(
                        add(_vk, 0x8e0),
                        {:#x}
                    ) // PLTDS_Y \n
                     mstore(
                        add(_vk, 0x900),
                        {:#x}
                    ) // QDOMSEP_X \n
                     mstore(
                        add(_vk, 0x920),
                        {:#x}
                    ) // QDOMSEP_Y \n
                    mstore(
                        add(_vk, 0x700),
                        {:#x}
                    ) // OKG1_X \n
                    mstore(
                        add(_vk, 0x720),
                        {:#x}
                    ) // OKG1_Y \n
                    mstore(
                        add(_vk, 0x740),
                        {:#x}
                    ) // OKG2_X1 \n
                    mstore(
                        add(_vk, 0x760),
                        {:#x}
                    ) // OKG2_X2 \n
                    mstore(
                        add(_vk, 0x780),
                        {:#x}
                    ) // OKG2_Y1 \n
                    mstore(
                        add(_vk, 0x7a0),
                        {:#x} 
                    ) // OKG2_Y2 \n
                    mstore(
                        add(_vk, 0x7c0),
                        {:#x}
                    ) // OKG3_X1 \n
                    mstore(
                        add(_vk, 0x7e0),
                        {:#x}
                    ) // OKG3_X2 \n
                    mstore(
                        add(_vk, 0x800),
                        {:#x}
                    ) // OKG3_Y1 \n
                    mstore(
                        add(_vk, 0x820),
                        {:#x}
                    ) // OKG3_Y2 \n
                    
                    mstore(_second_loc, {:#x}) // N_INV_LOCATION \n
                    mstore(add(_second_loc, 0x20), {:#x}) // OMEGA_LOCATION \n
                    mstore(add(_second_loc, 0x40), {:#x}) // OMEGA_INV_LOCATION \n
                }}
            }}
        }}",
        vk_hash,
        vk_vec[0],
        vk_vec[1],
        vk_vec[2],
        vk_vec[3],
        vk_vec[4],
        vk_vec[5],
        vk_vec[6],
        vk_vec[7],
        vk_vec[8],
        vk_vec[9],
        vk_vec[10],
        vk_vec[11],
        vk_vec[12],
        vk_vec[13],
        vk_vec[14],
        vk_vec[15],
        vk_vec[16],
        vk_vec[17],
        vk_vec[18],
        vk_vec[19],
        vk_vec[20],
        vk_vec[21],
        vk_vec[22],
        vk_vec[23],
        vk_vec[24],
        vk_vec[25],
        vk_vec[26],
        vk_vec[27],
        vk_vec[28],
        vk_vec[29],
        vk_vec[30],
        vk_vec[31],
        vk_vec[32],
        vk_vec[33],
        vk_vec[34],
        vk_vec[35],
        vk_vec[36],
        vk_vec[37],
        vk_vec[38],
        vk_vec[39],
        vk_vec[40],
        vk_vec[41],
        vk_vec[42],
        vk_vec[43],
        vk_vec[44],
        vk_vec[45],
        vk_vec[46],
        vk_vec[47],
        vk_vec[48],
        vk_vec[49],
        vk_vec[50],
        vk_vec[51],
        vk_vec[52],
        vk_vec[53],
        vk_vec[54],
        vk_vec[55],
        vk_vec[56],
        vk_vec[57],
        vk_vec[58],
        vk_vec[59],
        vk_vec[60],
        vk_vec[61],
        vk_vec[62],
        vk_vec[63],
        vk_vec[64],
        vk_vec[65],
        vk_vec[66],
        vk_vec[67],
        vk_vec[68],
        vk_vec[69],
        vk_vec[70],
        vk_vec[71],
        vk_vec[72],
        vk_vec[73],
        domain_size_inv,
        omega,
        omega_inv
    ))
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloy::primitives::Bytes;

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

        let proof = PlonkKzgSnark::<Bn254>::prove::<_, _, SolidityTranscript>(
            &mut rng, &circuit, &pk, None,
        )
        .unwrap();
        let proof_vec = Vec::<Fq254>::from(proof)
            .into_iter()
            .flat_map(|x| {
                let mut bytes: Vec<u8> = x.into_bigint().to_bytes_le();
                bytes.resize(32, 0u8);
                bytes.reverse();
                bytes
            })
            .collect::<Vec<u8>>();

        let bytes = Bytes::from_iter(proof_vec);

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
                    .join("test_contracts/testproof.txt");
                break;
            }

            cwd = cwd.parent().ok_or("No parent in path").unwrap();
        }
        let mut file = File::create(path_out.clone()).unwrap();
        file.write_fmt(format_args!("{:0x}", bytes)).unwrap();

        // We run `forge test` now to update all the contracts
        let output = std::process::Command::new("forge")
            .args(["test", "--force", "--match-test", "testRollupVerifier"])
            .output()
            .unwrap();
        //    std::fs::remove_file(path_out).unwrap();
        println!(" forge test output: {:?}", output);
println!(
            "Forge output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        println!(
            "Forge error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
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
