use alloy::primitives::U256;
use ark_bn254::{Bn254, Fq as Fq254, Fr as Fr254};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::vec::Vec;
use jf_plonk::proof_system::structs::{VerifyingKey, VK};
use regex::Regex;
use std::path::PathBuf;

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
    let domain_size_inv = U256::from_le_bytes::<32>(
        domain_size_fr
            .inverse()
            .unwrap()
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .expect("Failed to convert Vec<u8> to [u8; 32]"),
    );
    let domain = Radix2EvaluationDomain::<Fr254>::new(domain_size).unwrap();
    let size_inv = domain_size_inv;
    let group_gen = U256::from_le_bytes::<32>(
        domain
            .group_gen()
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .expect("Failed to convert Vec<u8> to [u8; 32]"),
    );
    let group_gen_inv = U256::from_le_bytes::<32>(
        domain
            .group_gen_inv()
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .expect("Failed to convert Vec<u8> to [u8; 32]"),
    );

    let domain_size_u256 = U256::from(domain_size as u32);
    let num_inputs_u256 = U256::from(vk.num_inputs() as u32);
    let sigma_comms_u256: Vec<U256> = vk
        .sigma_comms
        .iter()
        .flat_map(|comm| {
            let x = U256::from_be_bytes::<32>(
                comm.x
                    .into_bigint()
                    .to_bytes_be()
                    .try_into()
                    .expect("Failed to convert Vec<u8> to [u8; 32]"),
            );
            let y = U256::from_be_bytes::<32>(
                comm.y
                    .into_bigint()
                    .to_bytes_be()
                    .try_into()
                    .expect("Failed to convert Vec<u8> to [u8; 32]"),
            );
            vec![x, y]
        })
        .collect();

    let selector_comms_u256: Vec<U256> = vk
        .selector_comms
        .iter()
        .flat_map(|comm| {
            let x = U256::from_be_bytes::<32>(
                comm.x
                    .into_bigint()
                    .to_bytes_be()
                    .try_into()
                    .expect("Failed to convert Vec<u8> to [u8; 32]"),
            );
            let y = U256::from_be_bytes::<32>(
                comm.y
                    .into_bigint()
                    .to_bytes_be()
                    .try_into()
                    .expect("Failed to convert Vec<u8> to [u8; 32]"),
            );
            vec![x, y]
        })
        .collect();
    let ks_u256: Vec<U256> =
        vk.k.iter()
            .map(|k| {
                U256::from_be_bytes::<32>(
                    k.into_bigint()
                        .to_bytes_be()
                        .try_into()
                        .expect("Failed to convert Vec<u8> to [u8; 32]"),
                )
            })
            .collect();
    let vk_vec_u256 = Vec::<Fq254>::from(vk.clone())
        .into_iter()
        .map(|x| {
            U256::from_le_bytes::<32>(
                x.into_bigint()
                    .to_bytes_le()
                    .try_into()
                    .expect("Failed to convert Vec<u8> to [u8; 32]"),
            )
        })
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
    let header = format!("[{mode}.verifier]");

    let as_hex = |u: &U256| -> String { format!("{u:#x}") }; // bare 0x… (for domain_size/num_inputs)
    let as_q = |u: &U256| -> String { format!("\"{u:#x}\"") }; // "0x…"
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
    block.push('\n');

    // ===== Replace or append the section =====

    let had_crlf = toml_txt.contains("\r\n");
    toml_txt = replace_verifier_block(&toml_txt, &mode, &block);
    if had_crlf {
        toml_txt = toml_txt.replace('\n', "\r\n");
    }
    fs::write(&nightfall_path, toml_txt)?;
    Ok(())
}
