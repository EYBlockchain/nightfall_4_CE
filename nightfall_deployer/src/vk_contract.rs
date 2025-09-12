use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use configuration::settings::Settings;
use ethers::types::U256;
use jf_plonk::proof_system::structs::{VerifyingKey, VK};

use regex::Regex;
use std::{
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn replace_verifier_block(full: &str, mode: &str, new_block: &str) -> String {
    let had_crlf = full.contains("\r\n");
    let mut s = full.replace("\r\n", "\n");

    // 1) Find the header line
    let header_re = Regex::new(&format!(
        r"(?m)^\[\s*{}\s*\.verifier\s*\]\s*",
        regex::escape(mode)
    ))
    .expect("valid regex");
    if let Some(m) = header_re.find(&s) {
        let start = m.start();
        let after_header = m.end();

        // 2) Find the beginning of the next table (any line starting with '[') or EOF
        let next_table_re = Regex::new(r"(?m)^\[").expect("valid regex");
        let end = if let Some(nm) = next_table_re.find(&s[after_header..]) {
            after_header + nm.start()
        } else {
            s.len()
        };

        // 3) Splice
        let mut out = String::with_capacity(s.len() + new_block.len() + 2);
        out.push_str(&s[..start]);
        if start > 0 && !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(new_block);
        if !new_block.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&s[end..]);

        if had_crlf {
            out.replace('\n', "\r\n")
        } else {
            out
        }
    } else {
        // No section present → append
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(new_block);
        if !new_block.ends_with('\n') {
            s.push('\n');
        }
        if had_crlf {
            s.replace('\n', "\r\n")
        } else {
            s
        }
    }
}

pub fn write_vk_to_nightfall_toml(vk: &VerifyingKey<Bn254>) -> anyhow::Result<()> {
    use std::fs;

    // ===== Build values =====
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
        .flat_map(|c| {
            let x = U256::from_big_endian(&c.x.into_bigint().to_bytes_be());
            let y = U256::from_big_endian(&c.y.into_bigint().to_bytes_be());
            [x, y]
        })
        .collect();

    let selector_comms_u256: Vec<U256> = vk
        .selector_comms
        .iter()
        .flat_map(|c| {
            let x = U256::from_big_endian(&c.x.into_bigint().to_bytes_be());
            let y = U256::from_big_endian(&c.y.into_bigint().to_bytes_be());
            [x, y]
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

    let range_table_comm_u256 = [vk_vec_u256[56], vk_vec_u256[57]];
    let key_table_comm_u256 = [vk_vec_u256[58], vk_vec_u256[59]];
    let table_dom_sep_comm_u256 = [vk_vec_u256[60], vk_vec_u256[61]];
    let q_dom_sep_comm_u256 = [vk_vec_u256[62], vk_vec_u256[63]];
    let open_key_g = [vk_vec_u256[64], vk_vec_u256[65]];
    let h = [
        vk_vec_u256[67],
        vk_vec_u256[66],
        vk_vec_u256[69],
        vk_vec_u256[68],
    ];
    let beta_h = [
        vk_vec_u256[71],
        vk_vec_u256[70],
        vk_vec_u256[73],
        vk_vec_u256[72],
    ];

    // print all the values we are going to write
    ark_std::println!("domain_size = {}", domain_size_u256);
    ark_std::println!("num_inputs  = {}", num_inputs_u256);
    for i in 0..6 {
        let x = &sigma_comms_u256[2 * i];
        let y = &sigma_comms_u256[2 * i + 1];
        ark_std::println!("sigma_comms_{} = [{}, {}]", i + 1, x, y);
    }
    for i in 0..18 {
        let x = &selector_comms_u256[2 * i];
        let y = &selector_comms_u256[2 * i + 1];
        ark_std::println!("selector_comms_{} = [{}, {}]", i + 1, x, y);
    }
    for i in 0..6 {
        ark_std::println!("k{} = {}", i + 1, ks_u256[i]);
    }
    ark_std::println!(
        "range_table_comm     = [{}, {}]",
        range_table_comm_u256[0], range_table_comm_u256[1]
    );
    ark_std::println!(
        "key_table_comm       = [{}, {}]",
        key_table_comm_u256[0], key_table_comm_u256[1]
    );
    ark_std::println!(
        "table_dom_sep_comm   = [{}, {}]",
        table_dom_sep_comm_u256[0], table_dom_sep_comm_u256[1]
    );
    ark_std::println!(
        "q_dom_sep_comm       = [{}, {}]",
        q_dom_sep_comm_u256[0], q_dom_sep_comm_u256[1]
    );
    ark_std::println!("size_inv      = {}", size_inv);
    ark_std::println!("group_gen     = {}", group_gen);
    ark_std::println!("group_gen_inv = {}", group_gen_inv);
    ark_std::println!("open_key_g = [{}, {}]", open_key_g[0], open_key_g[1]);
    ark_std::println!(
        "h = [{}, {}, {}, {}]",
        h[0], h[1], h[2], h[3]
    );
    ark_std::println!(
        "beta_h = [{}, {}, {}, {}]",
        beta_h[0], beta_h[1], beta_h[2], beta_h[3]
    );
    

    // ===== Locate nightfall.toml =====
    let mut cwd = std::env::current_dir()?;
    let nightfall_path: PathBuf = loop {
        let p = cwd.join("nightfall.toml");
        if p.is_file() {
            break p;
        }
        cwd = cwd
            .parent()
            .ok_or_else(|| anyhow::anyhow!("could not find nightfall.toml"))?
            .to_path_buf();
    };
    let mut toml_txt = fs::read_to_string(&nightfall_path)?;

    // ===== Build new block text =====
    let mode = std::env::var("NF4_RUN_MODE").unwrap_or_else(|_| "development".to_string());
    let header = format!("[{}.verifier]", mode);

    let as_hex = |u: &U256| -> String { format!("{:#x}", u) }; // bare 0x… (for domain_size/num_inputs)
    let as_q = |u: &U256| -> String { format!("\"{:#x}\"", u) }; // "0x…"
    let pair_q = |a: &U256, b: &U256| -> String { format!("[{}, {}]", as_q(a), as_q(b)) };

    let mut block = String::new();
    block.push_str(&header);
    block.push('\n');
    block.push_str("# Auto-generated by key_generation: VK values that the UUPS provider will initialize with\n");
    block.push_str(&format!("domain_size = {}\n", as_hex(&domain_size_u256)));
    block.push_str(&format!("num_inputs  = {}\n", as_hex(&num_inputs_u256)));

    for i in 0..6 {
        let x = &sigma_comms_u256[2 * i];
        let y = &sigma_comms_u256[2 * i + 1];
        block.push_str(&format!("sigma_comms_{} = {}\n", i + 1, pair_q(x, y)));
    }
    for i in 0..18 {
        let x = &selector_comms_u256[2 * i];
        let y = &selector_comms_u256[2 * i + 1];
        block.push_str(&format!("selector_comms_{} = {}\n", i + 1, pair_q(x, y)));
    }

    block.push_str(&format!("k1 = {}\n", as_q(&ks_u256[0])));
    block.push_str(&format!("k2 = {}\n", as_q(&ks_u256[1])));
    block.push_str(&format!("k3 = {}\n", as_q(&ks_u256[2])));
    block.push_str(&format!("k4 = {}\n", as_q(&ks_u256[3])));
    block.push_str(&format!("k5 = {}\n", as_q(&ks_u256[4])));
    block.push_str(&format!("k6 = {}\n", as_q(&ks_u256[5])));

    block.push_str(&format!(
        "range_table_comm     = {}\n",
        pair_q(&range_table_comm_u256[0], &range_table_comm_u256[1])
    ));
    block.push_str(&format!(
        "key_table_comm       = {}\n",
        pair_q(&key_table_comm_u256[0], &key_table_comm_u256[1])
    ));
    block.push_str(&format!(
        "table_dom_sep_comm   = {}\n",
        pair_q(&table_dom_sep_comm_u256[0], &table_dom_sep_comm_u256[1])
    ));
    block.push_str(&format!(
        "q_dom_sep_comm       = {}\n",
        pair_q(&q_dom_sep_comm_u256[0], &q_dom_sep_comm_u256[1])
    ));

    block.push_str(&format!("size_inv      = {}\n", as_q(&size_inv)));
    block.push_str(&format!("group_gen     = {}\n", as_q(&group_gen)));
    block.push_str(&format!("group_gen_inv = {}\n", as_q(&group_gen_inv)));

    block.push_str(&format!(
        "open_key_g = {}\n",
        pair_q(&open_key_g[0], &open_key_g[1])
    ));
    // h / beta_h inline
    block.push_str(&format!(
        "h = [{}, {}, {}, {}]\n",
        as_q(&h[0]),
        as_q(&h[1]),
        as_q(&h[2]),
        as_q(&h[3])
    ));
    block.push_str(&format!(
        "beta_h = [{}, {}, {}, {}]\n",
        as_q(&beta_h[0]),
        as_q(&beta_h[1]),
        as_q(&beta_h[2]),
        as_q(&beta_h[3])
    ));
    // (optional) visual spacer before next table
    block.push_str("\n");
    // block.push_str("h = [\n");
    // block.push_str(&format!("  {},\n", as_q(&h[0]))); // x0
    // block.push_str(&format!("  {},\n", as_q(&h[1]))); // x1
    // block.push_str(&format!("  {},\n", as_q(&h[2]))); // y0
    // block.push_str(&format!("  {}\n",  as_q(&h[3]))); // y1
    // block.push_str("]\n");

    // block.push_str("beta_h = [\n");
    // block.push_str(&format!("  {},\n", as_q(&beta_h[0]))); // x0
    // block.push_str(&format!("  {},\n", as_q(&beta_h[1]))); // x1
    // block.push_str(&format!("  {},\n", as_q(&beta_h[2]))); // y0
    // block.push_str(&format!("  {}\n",  as_q(&beta_h[3]))); // y1
    // block.push_str("]\n");

    // ===== Replace or append the section =====

    // println!("Updated [{} .verifier] in {}", mode, nightfall_path.display());
    let had_crlf = toml_txt.contains("\r\n");
    toml_txt = replace_verifier_block(&toml_txt, &mode, &block);
    if had_crlf {
        toml_txt = toml_txt.replace('\n', "\r\n");
    }
    fs::write(&nightfall_path, toml_txt)?;
    Ok(())
}
