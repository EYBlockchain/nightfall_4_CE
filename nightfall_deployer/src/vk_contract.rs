use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use configuration::settings::Settings;
use ethers::types::U256;
use jf_plonk::proof_system::structs::{VerifyingKey, VK};

use std::{
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use regex::Regex;

pub fn write_vk_to_nightfall_toml(
    vk: &VerifyingKey<Bn254>,
) -> anyhow::Result<()> {
    ark_std::println!("Writing vk to nightfall.toml...");
    // 1) Compute everything for vk structure
    let domain_size = vk.domain_size();
    let domain_size_fr = Fr254::from(domain_size as u32);
    let domain_size_inv = U256::from_little_endian(
        &domain_size_fr.inverse().unwrap().into_bigint().to_bytes_le(),
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

    let ks_u256: Vec<U256> = vk
        .k
        .iter()
        .map(|k| U256::from_big_endian(&k.into_bigint().to_bytes_be()))
        .collect();

    // These indices mirror your existing generator
    let vk_vec_u256 = Vec::<Fq254>::from(vk.clone())
        .into_iter()
        .map(|x| U256::from_little_endian(&x.into_bigint().to_bytes_le()))
        .collect::<Vec<_>>();

    let range_table_comm_u256 = [vk_vec_u256[56], vk_vec_u256[57]];
    let key_table_comm_u256 = [vk_vec_u256[58], vk_vec_u256[59]];
    let table_dom_sep_comm_u256 = [vk_vec_u256[60], vk_vec_u256[61]];
    let q_dom_sep_comm_u256 = [vk_vec_u256[62], vk_vec_u256[63]];
    let open_key_g = [vk_vec_u256[64], vk_vec_u256[65]];
    let h = [vk_vec_u256[67], vk_vec_u256[66], vk_vec_u256[69], vk_vec_u256[68]];
    let beta_h = [vk_vec_u256[71], vk_vec_u256[70], vk_vec_u256[73], vk_vec_u256[72]];
    ark_std::println!("beta_h: {:?}", beta_h);

    // 2) Locate nightfall.toml (walk up from CWD)
    let mut cwd = std::env::current_dir()?;
    let nightfall_path: PathBuf = loop {
        let candidate = cwd.join("nightfall.toml");
        if candidate.is_file() {
            break candidate;
        }
        cwd = cwd.parent().ok_or_else(|| anyhow::anyhow!("could not find nightfall.toml"))?.to_path_buf();
    };

    // 3) Build a TOML block for [<mode>.verifier]
    let mode = std::env::var("NF4_RUN_MODE").unwrap_or_else(|_| "development".to_string());

    // helper lambdas
    let as_hex = |u: &U256| -> String { format!("{:#x}", u) };               // 0x… (unquoted)
    let as_qhex = |u: &U256| -> String { format!("\"{:#x}\"", u) };          // "0x…"
    let pair_q = |a: &U256, b: &U256| -> String { format!("[{}, {}]", as_qhex(a), as_qhex(b)) };

    let mut block = String::new();
    block.push_str(&format!("[{}.verifier]\n", mode));

    // Domain / inputs (keep unquoted hex like your examples)
    block.push_str(&format!("domain_size = {}\n", as_hex(&domain_size_u256)));
    block.push_str(&format!("num_inputs  = {}\n", as_hex(&num_inputs_u256)));

    // Sigma commitments
    for i in 0..6 {
        let x = &sigma_comms_u256[2 * i];
        let y = &sigma_comms_u256[2 * i + 1];
        block.push_str(&format!("sigma_comms_{} = {}\n", i + 1, pair_q(x, y)));
    }

    // Selector commitments
    for i in 0..18 {
        let x = &selector_comms_u256[2 * i];
        let y = &selector_comms_u256[2 * i + 1];
        block.push_str(&format!("selector_comms_{}  = {}\n", i + 1, pair_q(x, y)));
    }

    // Scalars (quote them to keep TOML valid; Deployer's readUint handles quoted hex)
    block.push_str(&format!("k1 = {}\n", as_qhex(&ks_u256[0])));
    block.push_str(&format!("k2 = {}\n", as_qhex(&ks_u256[1])));
    block.push_str(&format!("k3 = {}\n", as_qhex(&ks_u256[2])));
    block.push_str(&format!("k4 = {}\n", as_qhex(&ks_u256[3])));
    block.push_str(&format!("k5 = {}\n", as_qhex(&ks_u256[4])));
    block.push_str(&format!("k6 = {}\n", as_qhex(&ks_u256[5])));

    // Commitments (G1)
    block.push_str(&format!("range_table_comm     = {}\n", pair_q(&range_table_comm_u256[0], &range_table_comm_u256[1])));
    block.push_str(&format!("key_table_comm       = {}\n", pair_q(&key_table_comm_u256[0], &key_table_comm_u256[1])));
    block.push_str(&format!("table_dom_sep_comm   = {}\n", pair_q(&table_dom_sep_comm_u256[0], &table_dom_sep_comm_u256[1])));
    block.push_str(&format!("q_dom_sep_comm       = {}\n", pair_q(&q_dom_sep_comm_u256[0], &q_dom_sep_comm_u256[1])));

    // Group params (quote them)
    block.push_str(&format!("size_inv      = {}\n", as_qhex(&size_inv)));
    block.push_str(&format!("group_gen     = {}\n", as_qhex(&group_gen)));
    block.push_str(&format!("group_gen_inv = {}\n", as_qhex(&group_gen_inv)));

    // Open key (G1) as array of quoted hex
    block.push_str(&format!("open_key_g = {}\n", pair_q(&open_key_g[0], &open_key_g[1])));

    // G2 points (arrays of quoted hex in x0,x1,y0,y1 order)
    block.push_str("h = [\n");
    block.push_str(&format!("  {},\n", as_qhex(&h[0]))); // x0
    block.push_str(&format!("  {},\n", as_qhex(&h[1]))); // x1
    block.push_str(&format!("  {},\n", as_qhex(&h[2]))); // y0
    block.push_str(&format!("  {}\n",  as_qhex(&h[3]))); // y1
    block.push_str("]\n");

    block.push_str("beta_h = [\n");
    block.push_str(&format!("  {},\n", as_qhex(&beta_h[0]))); // x0
    block.push_str(&format!("  {},\n", as_qhex(&beta_h[1]))); // x1
    block.push_str(&format!("  {},\n", as_qhex(&beta_h[2]))); // y0
    block.push_str(&format!("  {}\n",  as_qhex(&beta_h[3]))); // y1
    block.push_str("]\n");

    // 4) Replace or append the whole [<mode>.verifier] block
    let mut toml_txt = fs::read_to_string(&nightfall_path)?;
    let re = Regex::new(&format!(r"(?ms)^\[\s*{}\s*\.verifier\s*\]\s*.*?(?=^\[|\z)", regex::escape(&mode)))?;
    if re.is_match(&toml_txt) {
        toml_txt = re.replace(&toml_txt, block.as_str()).to_string();
    } else {
        // section doesn't exist; append
        toml_txt.push_str("\n");
        toml_txt.push_str(&block);
    }
    fs::write(&nightfall_path, toml_txt)?;

    ark_std::println!("Updated [{}.verifier] in {}", mode, nightfall_path.display());
    Ok(())
}
