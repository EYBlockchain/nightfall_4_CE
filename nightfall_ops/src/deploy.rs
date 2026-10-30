use std::{
    collections::BTreeMap,
    fs,
    io::ErrorKind,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use toml_edit::{DocumentMut, Item};

use crate::{
    checks, client, compose, config, model::DeploymentConfig, network, proposer, validation,
    webhook, wizard,
};

const JOIN_TOML: &str = "configuration/toml/join.toml";
const PUBLIC_ENV: &str = "configuration/toml/public.env";
const ADDRESSES_TOML: &str = "configuration/toml/addresses.toml";
const CONTRACT_HASHES_TOML: &str = "configuration/toml/contract_hashes.toml";
const KEY_DIR: &str = "configuration/bin/keys";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Operator,
    Client,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployFile {
    pub role: Role,
    pub network: network::NetworkKind,
    pub profile: String,
    pub chain_id: u64,
    pub rpc_wss: String,
    pub rpc_http: String,
    pub operator_host: String,
    pub configuration_port: u16,
    pub proposer_port: u16,
    pub client_port: u16,
    pub webhook_port: u16,
    pub mock_prover: bool,
    pub block_size: u64,
    pub confirm: String,
    pub late_join: bool,
    pub redeploy: bool,
    pub deployer_key: String,
    pub proposer_key: String,
    pub client_key: String,
    pub expected_deployer_address: String,
    pub expected_proposer_address: String,
    pub expected_client_address: String,
    pub client2_address: String,
    pub join_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JoinDoc {
    network: String,
    profile: String,
    chain_id: u64,
    configuration_url: String,
    proposer_url: String,
    mock_prover: bool,
    block_size: u64,
    genesis_block: u64,
    deployer_address: String,
    default_proposer_address: String,
}

pub fn run(path: &str, check_only: bool, late_join: bool) -> Result<(), String> {
    if !config::required_repo_files_exist() {
        return Err("Run this command from the nightfall_4_CE repository root.".to_string());
    }
    let mut file = load_deploy_file(path)?;
    if late_join {
        if file.role != Role::Client {
            return Err("--late-join is only for the client role on VM-B.".to_string());
        }
        file.late_join = true;
    }
    if file.role == Role::Operator {
        file.operator_host = resolve_operator_host(&file)?;
        println!("Operator host: {}", file.operator_host);
        println!(
            "Join URL: http://{}:{}/configuration/toml/join.toml",
            file.operator_host, file.configuration_port
        );
        println!("Set NF4_OPERATOR_HOST only if VM-B cannot reach that address.");
    }
    if check_only {
        validate_for_check(&file)?;
        return check_wallets(&file);
    }
    validate_deploy_file(&file)?;
    match file.role {
        Role::Operator => run_operator(&file),
        Role::Client => run_client(&file),
    }
}

pub fn load_deploy_file(path: &str) -> Result<DeployFile, String> {
    let source = fs::read_to_string(path).map_err(|err| format!("Failed to read {path}: {err}"))?;
    parse_deploy_file(&source)
}

pub fn parse_deploy_file(source: &str) -> Result<DeployFile, String> {
    let env = parse_env_text(source);
    let role = match env.get("NF4_ROLE").map(String::as_str) {
        Some("operator") => Role::Operator,
        Some("client") => Role::Client,
        Some(other) => return Err(format!("NF4_ROLE must be operator or client, got {other}.")),
        None => {
            return Err(
                "NF4_ROLE is required. Use operator on VM-A or client on VM-B.".to_string(),
            );
        }
    };
    let network = network::NetworkKind::parse(
        env.get("NF4_NETWORK")
            .map(String::as_str)
            .unwrap_or("testnet"),
    )?;
    if network == network::NetworkKind::Local {
        return Err(
            "nf4 deploy is for split VMs. Use nf4 wizard deploy --network local --yes on one machine."
                .to_string(),
        );
    }
    let rpc_wss = required(&env, "NF4_RPC_WSS")?;
    let rpc_http = env
        .get("NF4_RPC_HTTP")
        .cloned()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| http_from_ws(&rpc_wss));
    let deployer_key = env.get("NF4_DEPLOYER_KEY").cloned().unwrap_or_default();
    let proposer_key = env
        .get("NF4_PROPOSER_KEY")
        .cloned()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| deployer_key.clone());
    let client_key = env
        .get("NF4_CLIENT_KEY")
        .cloned()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| deployer_key.clone());

    Ok(DeployFile {
        role,
        network,
        profile: env
            .get("NF4_PROFILE")
            .cloned()
            .unwrap_or_else(|| "sepolia".to_string()),
        chain_id: parse_u64(
            env.get("NF4_CHAIN_ID")
                .map(String::as_str)
                .unwrap_or("11155111"),
            "NF4_CHAIN_ID",
        )?,
        rpc_wss,
        rpc_http,
        operator_host: env.get("NF4_OPERATOR_HOST").cloned().unwrap_or_default(),
        configuration_port: parse_port(
            env.get("NF4_CONFIGURATION_PORT"),
            8080,
            "NF4_CONFIGURATION_PORT",
        )?,
        proposer_port: parse_port(env.get("NF4_PROPOSER_PORT"), 3001, "NF4_PROPOSER_PORT")?,
        client_port: parse_port(env.get("NF4_CLIENT_PORT"), 3000, "NF4_CLIENT_PORT")?,
        webhook_port: parse_port(env.get("NF4_WEBHOOK_PORT"), 8081, "NF4_WEBHOOK_PORT")?,
        mock_prover: parse_bool(
            env.get("NF4_MOCK_PROVER")
                .map(String::as_str)
                .unwrap_or("false"),
        )?,
        block_size: parse_u64(
            env.get("NF4_BLOCK_SIZE")
                .map(String::as_str)
                .unwrap_or("64"),
            "NF4_BLOCK_SIZE",
        )?,
        confirm: env.get("NF4_CONFIRM").cloned().unwrap_or_default(),
        late_join: parse_bool(
            env.get("NF4_LATE_JOIN")
                .map(String::as_str)
                .unwrap_or("false"),
        )?,
        redeploy: parse_bool(
            env.get("NF4_REDEPLOY")
                .map(String::as_str)
                .unwrap_or("false"),
        )?,
        deployer_key,
        proposer_key,
        client_key,
        expected_deployer_address: env.get("NF4_DEPLOYER_ADDRESS").cloned().unwrap_or_default(),
        expected_proposer_address: env.get("NF4_PROPOSER_ADDRESS").cloned().unwrap_or_default(),
        expected_client_address: env.get("NF4_CLIENT_ADDRESS").cloned().unwrap_or_default(),
        client2_address: env.get("NF4_CLIENT2_ADDRESS").cloned().unwrap_or_default(),
        join_url: env.get("NF4_JOIN_URL").cloned().unwrap_or_default(),
    })
}

pub fn validate_deploy_file(file: &DeployFile) -> Result<(), String> {
    network::validate_profile_name(&file.profile)?;
    network::validate_rpc_scheme(file.network, &file.rpc_wss)?;
    if file.confirm.trim() != confirm_phrase(file.network) {
        return Err(format!(
            "Set NF4_CONFIRM={} in the env file. This replaces the wizard Apply? prompt.",
            confirm_phrase(file.network)
        ));
    }
    if !file.network.allows_mock_prover() && file.mock_prover {
        return Err("Mainnet does not allow NF4_MOCK_PROVER=true.".to_string());
    }
    if !matches!(file.block_size, 64 | 256) {
        return Err("NF4_BLOCK_SIZE must be 64 or 256.".to_string());
    }
    match file.role {
        Role::Operator => validate_operator(file),
        Role::Client => validate_client(file),
    }
}

fn validate_operator(file: &DeployFile) -> Result<(), String> {
    if split_vm_host_error(&file.operator_host).is_some() || file.operator_host.contains(':') {
        return Err(format!(
            "NF4_OPERATOR_HOST must be the IPv4 clients use for :{} and :{}. Not 127.0.0.1, localhost, or a Docker bridge.",
            file.configuration_port, file.proposer_port
        ));
    }
    let configuration_url = configuration_url(file);
    let proposer_url = proposer_url(file);
    network::validate_published_configuration_url(file.network, &configuration_url)?;
    network::validate_published_configuration_url(file.network, &proposer_url)?;
    reject_split_vm_url(&configuration_url, "configuration URL")?;
    reject_split_vm_url(&proposer_url, "proposer URL")?;
    require_key(&file.deployer_key, "NF4_DEPLOYER_KEY")?;
    require_key(&file.proposer_key, "NF4_PROPOSER_KEY")?;
    require_key(&file.client_key, "NF4_CLIENT_KEY")?;
    validate_optional_address(&file.expected_deployer_address, "NF4_DEPLOYER_ADDRESS")?;
    validate_optional_address(&file.expected_proposer_address, "NF4_PROPOSER_ADDRESS")?;
    validate_optional_address(&file.expected_client_address, "NF4_CLIENT_ADDRESS")?;
    if !is_address(&file.client2_address) {
        return Err(
            "NF4_CLIENT2_ADDRESS must be Client 2's L1 address from VM-B. Do not put KEY2 here."
                .to_string(),
        );
    }
    Ok(())
}

fn validate_client(file: &DeployFile) -> Result<(), String> {
    if !file.late_join {
        return Err(
            "Refusing to start Client 2. Set NF4_LATE_JOIN=yes only after Client 1 deposits are confirmed."
                .to_string(),
        );
    }
    if !file.join_url.contains("/configuration/toml/join.toml") {
        return Err(
            "NF4_JOIN_URL must be the operator join.toml URL, for example http://10.0.0.8:8080/configuration/toml/join.toml."
                .to_string(),
        );
    }
    reject_split_vm_url(&file.join_url, "NF4_JOIN_URL")?;
    require_key(&file.client_key, "NF4_CLIENT_KEY")?;
    validate_optional_address(&file.expected_client_address, "NF4_CLIENT_ADDRESS")?;
    if config::local_env_exists() {
        let env = config::read_local_env();
        if env.contains_key("DEPLOYER_SIGNING_KEY") {
            return Err(
                "This VM's local.env has DEPLOYER_SIGNING_KEY. Client 2 must use its own clone, not a copied operator env."
                    .to_string(),
            );
        }
    }
    Ok(())
}

fn run_operator(file: &DeployFile) -> Result<(), String> {
    checks::deployer()?;
    let chain_id = cast_u64("chain-id", &file.rpc_wss)?;
    if chain_id != file.chain_id {
        return Err(format!(
            "NF4_CHAIN_ID is {}, but the RPC chain ID is {}. Aborting.",
            file.chain_id,
            network::chain_id_description(chain_id)
        ));
    }
    match file.network.classify_chain_id(chain_id) {
        network::ChainIdMatch::Known => {}
        network::ChainIdMatch::Mismatch => return Err(file.network.mismatch_message(chain_id)),
        network::ChainIdMatch::Unknown => {
            return Err(format!(
                "RPC chain ID {} is not a known {} ID. Aborting.",
                network::chain_id_description(chain_id),
                file.network.as_str()
            ));
        }
    }
    let genesis_block = cast_u64("block-number", &file.rpc_wss)?;
    let wallets = resolve_operator_wallets(file)?;
    print_wallets(&wallets, &file.client2_address);
    require_operator_balances(file, &wallets)?;
    network::verify_clean_mempool(&file.rpc_wss, &wallets.deployer)?;
    if !same_address(&wallets.proposer, &wallets.deployer) {
        network::verify_clean_mempool(&file.rpc_wss, &wallets.proposer)?;
    }

    let configuration_url = configuration_url(file);
    let proposer_url = proposer_url(file);
    prepare_configuration_toml()?;
    if contracts_live(&file.rpc_http)? && !file.redeploy {
        println!("Contracts already have code. Skipping Forge deploy.");
        println!("Set NF4_REDEPLOY=true only when you intend to deploy a new set of contracts.");
        ensure_sticky_urls(file, &proposer_url, &wallets.proposer)?;
        ensure_same_prover_mode(file)?;
        config::merge_local_env(&[
            ("NF4_CONTRACTS__DEPLOY_CONTRACTS", "false".to_string()),
            ("NF4_NETWORK", file.network.as_str().to_string()),
            ("NF4_RUN_MODE", file.profile.clone()),
            ("NF4_MOCK_PROVER", file.mock_prover.to_string()),
            ("NF4_ETHEREUM_CLIENT_URL", file.rpc_wss.clone()),
            ("NF4_CONFIGURATION_URL", configuration_url.clone()),
            ("NF4_NIGHTFALL_PROPOSER__URL", proposer_url.clone()),
            ("CLIENT2_ADDRESS", file.client2_address.clone()),
        ])?;
        compose::up_configuration()?;
    } else {
        if file.mock_prover {
            println!("Warning: NF4_MOCK_PROVER=true. The Sepolia report expects a real prover.");
        }
        let deployment = DeploymentConfig {
            network: file.network,
            profile: file.profile.clone(),
            rpc_url: file.rpc_wss.clone(),
            chain_id,
            genesis_block,
            configuration_url: configuration_url.clone(),
            configuration_port: file.configuration_port,
            deployer_signing_key: file.deployer_key.clone(),
            deployer_address: wallets.deployer.clone(),
            default_proposer_address: wallets.proposer.clone(),
            default_proposer_url: proposer_url.clone(),
            mock_prover: file.mock_prover,
            block_size: file.block_size,
        };
        print_operator_review(&deployment, &wallets.client, &file.client2_address);
        wizard::apply_deployment(&deployment)?;
    }

    let genesis_block = profile_u64(&file.profile, &["genesis_block"])?;
    let join = JoinDoc {
        network: file.network.as_str().to_string(),
        profile: file.profile.clone(),
        chain_id,
        configuration_url: configuration_url.clone(),
        proposer_url: proposer_url.clone(),
        mock_prover: file.mock_prover,
        block_size: file.block_size,
        genesis_block,
        deployer_address: wallets.deployer.clone(),
        default_proposer_address: wallets.proposer.clone(),
    };
    write_join(&join)?;
    let addresses = read_addresses()?;
    write_public_env(file, &join, &addresses, &wallets.client)?;
    let join_url = format!("{configuration_url}/configuration/toml/join.toml");
    download(&join_url, "temp/join-self-check.toml")?;
    validation::configuration_endpoints(&configuration_url)?;
    println!("Join URL: {join_url}");

    // Health is not a current binary. The Oct 28 proposer stayed healthy and
    // kept estimating certification as the zero address.
    refresh_app(
        "nf4_indie_proposer",
        "temp/nf4-proposer.stamp",
        &["lib", "nightfall_proposer", "Cargo.toml", "Cargo.lock"],
        file.proposer_port,
        || {
            proposer::start_with(proposer::ProposerStart {
                signing_key: file.proposer_key.clone(),
                proposer_url: proposer_url.clone(),
                configuration_url: configuration_url.clone(),
            })
        },
    )?;

    refresh_app(
        "nf4_indie_client",
        "temp/nf4-client.stamp",
        &["lib", "nightfall_client", "Cargo.toml", "Cargo.lock"],
        file.client_port,
        || {
            let webhook = webhook::ensure_local(file.webhook_port)?;
            client::start_with(client::ClientStart {
                signing_key: file.client_key.clone(),
                client_address: wallets.client.clone(),
                client2_address: file.client2_address.clone(),
                client2_signing_key: None,
                proposer_url: join.proposer_url,
                configuration_url,
                webhook_url: format!(
                    "http://{}:{}/webhook",
                    network::detect_lan_host(),
                    file.webhook_port
                ),
                webhook_events: Some(webhook.events_path.display().to_string()),
                client_port: file.client_port,
            })
        },
    )?;
    println!(
        "Operator deploy finished. Certify from scripts/nf4-deploy, then run the test script."
    );
    Ok(())
}

fn run_client(file: &DeployFile) -> Result<(), String> {
    checks::deployer()?;
    let join_text = curl_text(&file.join_url)?;
    let join = parse_join(&join_text)?;
    if join.chain_id != file.chain_id {
        return Err(format!(
            "Join chain_id is {}, NF4_CHAIN_ID is {}.",
            join.chain_id, file.chain_id
        ));
    }
    let rpc_chain = cast_u64("chain-id", &file.rpc_wss)?;
    if rpc_chain != join.chain_id {
        return Err(format!(
            "Join chain_id is {}, but this VM's RPC chain ID is {}.",
            join.chain_id,
            network::chain_id_description(rpc_chain)
        ));
    }
    reject_split_vm_url(&join.configuration_url, "join configuration_url")?;
    reject_split_vm_url(&join.proposer_url, "join proposer_url")?;
    prepare_configuration_toml()?;
    fs::create_dir_all(KEY_DIR).map_err(|err| format!("Failed to create {KEY_DIR}: {err}"))?;
    remove_if_unwritable(Path::new(JOIN_TOML))?;
    fs::write(JOIN_TOML, render_join(&join))
        .map_err(|err| format!("Failed to write {JOIN_TOML}: {err}"))?;
    download(
        &format!(
            "{}/configuration/toml/addresses.toml",
            join.configuration_url
        ),
        ADDRESSES_TOML,
    )?;
    download(
        &format!(
            "{}/configuration/toml/contract_hashes.toml",
            join.configuration_url
        ),
        CONTRACT_HASHES_TOML,
    )?;
    for key in proposer::required_key_files(join.mock_prover) {
        download(
            &format!("{}/configuration/bin/keys/{key}", join.configuration_url),
            &format!("{KEY_DIR}/{key}"),
        )?;
    }
    let client_address = cast_address(&file.client_key)?;
    expect_address(
        &client_address,
        &file.expected_client_address,
        "NF4_CLIENT_ADDRESS",
    )?;
    println!("Client wallet: {client_address}");
    require_balance(&file.rpc_http, &client_address, "client")?;
    let deployment = DeploymentConfig {
        network: file.network,
        profile: join.profile.clone(),
        rpc_url: file.rpc_wss.clone(),
        chain_id: join.chain_id,
        genesis_block: join.genesis_block,
        configuration_url: join.configuration_url.clone(),
        configuration_port: url_port(&join.configuration_url).unwrap_or(8080),
        deployer_signing_key: String::new(),
        deployer_address: join.deployer_address.clone(),
        default_proposer_address: join.default_proposer_address.clone(),
        default_proposer_url: join.proposer_url.clone(),
        mock_prover: join.mock_prover,
        block_size: join.block_size,
    };
    let _ = config::backup_config_files();
    config::update_nightfall_toml(&deployment)?;
    config::set_profile_deploy_contracts(&join.profile, false)?;
    config::merge_local_env(&[
        ("NF4_RUN_MODE", join.profile.clone()),
        ("NF4_NETWORK", join.network.clone()),
        ("NF4_MOCK_PROVER", join.mock_prover.to_string()),
        ("NF4_CONTRACTS__DEPLOY_CONTRACTS", "false".to_string()),
        ("NF4_ETHEREUM_CLIENT_URL", file.rpc_wss.clone()),
        ("NF4_CONFIGURATION_URL", join.configuration_url.clone()),
        ("NF4_NIGHTFALL_PROPOSER__URL", join.proposer_url.clone()),
        ("CLIENT_SIGNING_KEY", file.client_key.clone()),
        ("NF4_SIGNING_KEY", file.client_key.clone()),
        ("CLIENT_ADDRESS", client_address.clone()),
    ])?;
    if !curl_ok(&format!("http://127.0.0.1:{}/v1/health", file.client_port)) {
        let webhook = webhook::ensure_local(file.webhook_port)?;
        client::start_with(client::ClientStart {
            signing_key: file.client_key.clone(),
            client_address: client_address.clone(),
            client2_address: client_address,
            client2_signing_key: None,
            proposer_url: join.proposer_url,
            configuration_url: join.configuration_url,
            webhook_url: format!(
                "http://{}:{}/webhook",
                network::detect_lan_host(),
                file.webhook_port
            ),
            webhook_events: Some(webhook.events_path.display().to_string()),
            client_port: file.client_port,
        })?;
    } else {
        println!("Client already healthy on :{}.", file.client_port);
    }
    println!("Client deploy finished. Do not run wizard deploy on this VM.");
    Ok(())
}

fn confirm_phrase(network: network::NetworkKind) -> &'static str {
    network.typed_confirm_phrase().unwrap_or("apply")
}

pub fn operator_host_candidates(output: &str) -> Vec<String> {
    let mut hosts = Vec::new();
    for token in output.split_whitespace() {
        let host = token.trim().trim_matches(|ch| ch == ',' || ch == ';');
        if host.is_empty() || host.contains(':') || host.starts_with("169.254.") {
            continue;
        }
        if split_vm_host_error(host).is_some() {
            continue;
        }
        if !hosts.iter().any(|existing: &String| existing == host) {
            hosts.push(host.to_string());
        }
    }
    hosts
}

pub fn choose_operator_host(
    explicit: &str,
    detected: &[String],
    reachable: &[bool],
) -> Result<String, String> {
    if !explicit.trim().is_empty() {
        return Ok(explicit.trim().to_string());
    }
    let mut ordered = Vec::new();
    for host in detected {
        if is_private_lan(host) {
            ordered.push(host.clone());
        }
    }
    for host in detected {
        if !is_private_lan(host) {
            ordered.push(host.clone());
        }
    }
    for host in &ordered {
        let index = detected.iter().position(|candidate| candidate == host);
        if index.is_some_and(|index| reachable.get(index) == Some(&true)) {
            return Ok(host.clone());
        }
    }
    Err(
        "Could not detect an operator host that this machine can reach on the configuration port. Set NF4_OPERATOR_HOST to the address VM-B uses."
            .to_string(),
    )
}

fn resolve_operator_host(file: &DeployFile) -> Result<String, String> {
    let explicit = file.operator_host.trim();
    if !explicit.is_empty() {
        if !host_is_reachable(explicit, file.configuration_port) {
            return Err(format!(
                "This machine cannot open TCP to {explicit}:{}. Connection refused is fine before deploy; a timeout is not. Pick another NF4_OPERATOR_HOST.",
                file.configuration_port
            ));
        }
        return Ok(explicit.to_string());
    }
    let detected = detect_operator_hosts();
    let reachable: Vec<bool> = detected
        .iter()
        .map(|host| host_is_reachable(host, file.configuration_port))
        .collect();
    let chosen = choose_operator_host("", &detected, &reachable)?;
    println!("Detected operator host {chosen} from local interfaces.");
    Ok(chosen)
}

fn detect_operator_hosts() -> Vec<String> {
    let mut found = String::new();
    if let Some(output) = command_stdout("hostname", &["-I"]) {
        found.push_str(&output);
        found.push(' ');
    }
    for interface in ["en0", "en1"] {
        if let Some(output) = command_stdout("ipconfig", &["getifaddr", interface]) {
            found.push_str(&output);
            found.push(' ');
        }
    }
    operator_host_candidates(&found)
}

fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn host_is_reachable(host: &str, port: u16) -> bool {
    let Ok(addresses) = (host, port).to_socket_addrs() else {
        return false;
    };
    addresses.take(4).any(|address| tcp_open(address))
}

fn tcp_open(address: SocketAddr) -> bool {
    match TcpStream::connect_timeout(&address, Duration::from_secs(2)) {
        Ok(_) => true,
        Err(err) => err.kind() == std::io::ErrorKind::ConnectionRefused,
    }
}

fn is_private_lan(host: &str) -> bool {
    host.starts_with("10.") || host.starts_with("192.168.") || is_172_lan(host)
}

fn is_172_lan(host: &str) -> bool {
    let mut parts = host.split('.');
    let Some("172") = parts.next() else {
        return false;
    };
    let Ok(second) = parts.next().unwrap_or("").parse::<u8>() else {
        return false;
    };
    (16..=31).contains(&second) && second != 17 && second != 28
}

fn configuration_url(file: &DeployFile) -> String {
    format!("http://{}:{}", file.operator_host, file.configuration_port)
}

fn proposer_url(file: &DeployFile) -> String {
    format!("http://{}:{}", file.operator_host, file.proposer_port)
}

pub fn split_vm_host_error(host: &str) -> Option<&'static str> {
    let host = host.trim().trim_matches(['[', ']']);
    if host.is_empty()
        || host.eq_ignore_ascii_case("localhost")
        || host == "0.0.0.0"
        || host.starts_with("127.")
        || host.starts_with("172.17.")
        || host.starts_with("172.28.")
        || network::is_docker_internal_host(host)
    {
        Some("loopback or docker-only host")
    } else {
        None
    }
}

fn reject_split_vm_url(url: &str, label: &str) -> Result<(), String> {
    let host = url_host(url).unwrap_or("");
    if split_vm_host_error(host).is_some() {
        return Err(format!(
            "{label} uses a loopback or Docker-only host. Use the operator VM's LAN or public IP."
        ));
    }
    Ok(())
}

fn contracts_live(rpc_http: &str) -> Result<bool, String> {
    if !Path::new(ADDRESSES_TOML).is_file() {
        return Ok(false);
    }
    let addresses = read_addresses()?;
    let Some(nightfall) = addresses.get("nightfall") else {
        return Ok(false);
    };
    if !is_address(nightfall) {
        return Ok(false);
    }
    contract_has_code(nightfall, rpc_http)
}

fn contract_has_code(address: &str, rpc_url: &str) -> Result<bool, String> {
    let output = Command::new("cast")
        .args(["code", address, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("Failed to run cast code: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cast code failed: {}",
            network::command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    let code = String::from_utf8_lossy(&output.stdout);
    let code = code
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("0x"));
    Ok(code.is_some_and(|value| value != "0x" && value != "0x0"))
}

fn ensure_sticky_urls(
    file: &DeployFile,
    proposer_url: &str,
    proposer_address: &str,
) -> Result<(), String> {
    let stored = profile_string(
        &file.profile,
        &["nightfall_deployer", "default_proposer_url"],
    )?;
    if !stored.is_empty() && stored != proposer_url {
        return Err(format!(
            "On-chain proposer URL is {stored}, but this file would publish {proposer_url}. Changing the operator IP requires a new deploy (NF4_REDEPLOY=true), not a rewrite."
        ));
    }
    let stored_address = profile_string(
        &file.profile,
        &["nightfall_deployer", "default_proposer_address"],
    )?;
    if !stored_address.is_empty() && !same_address(&stored_address, proposer_address) {
        return Err(format!(
            "Designated proposer is {stored_address}, but this file's proposer key is {proposer_address}. A re-run does not rotate an existing wallet."
        ));
    }
    Ok(())
}

fn ensure_same_prover_mode(file: &DeployFile) -> Result<(), String> {
    let Some(stored) = stored_mock_prover(&file.profile)? else {
        return Err(
            "Existing contracts have no recorded NF4_MOCK_PROVER. Refusing to publish the flag from this file. Deploy a new profile if the prover mode is unknown."
                .to_string(),
        );
    };
    if stored != file.mock_prover {
        return Err(format!(
            "This deployment was created with NF4_MOCK_PROVER={stored}. This file has {}. Do not flip the flag. Deploy a new profile instead.",
            file.mock_prover
        ));
    }
    Ok(())
}

fn stored_mock_prover(profile: &str) -> Result<Option<bool>, String> {
    let env = config::read_local_env();
    if let Some(value) = env.get("NF4_MOCK_PROVER") {
        return Ok(Some(parse_bool(value)?));
    }
    let Ok(source) = fs::read_to_string(config::NIGHTFALL_TOML) else {
        return Ok(None);
    };
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse nightfall.toml: {err}"))?;
    Ok(doc
        .get(profile)
        .and_then(|item| item.get("mock_prover"))
        .and_then(Item::as_bool))
}

struct OperatorWallets {
    deployer: String,
    proposer: String,
    client: String,
}

fn resolve_operator_wallets(file: &DeployFile) -> Result<OperatorWallets, String> {
    let deployer = cast_address(&file.deployer_key)?;
    let proposer = cast_address(&file.proposer_key)?;
    let client = cast_address(&file.client_key)?;
    expect_address(
        &deployer,
        &file.expected_deployer_address,
        "NF4_DEPLOYER_ADDRESS",
    )?;
    expect_address(
        &proposer,
        &file.expected_proposer_address,
        "NF4_PROPOSER_ADDRESS",
    )?;
    expect_address(&client, &file.expected_client_address, "NF4_CLIENT_ADDRESS")?;
    Ok(OperatorWallets {
        deployer,
        proposer,
        client,
    })
}

fn print_wallets(wallets: &OperatorWallets, client2: &str) {
    println!("Wallets");
    println!("  deployer: {}", wallets.deployer);
    println!("  proposer: {}", wallets.proposer);
    println!("  client1: {}", wallets.client);
    println!("  client2: {client2}");
    if same_address(&wallets.proposer, &wallets.deployer)
        && same_address(&wallets.client, &wallets.deployer)
    {
        println!("  deployer, proposer, and Client 1 are the same existing wallet.");
    } else if !same_address(&wallets.proposer, &wallets.deployer) {
        println!("  proposer is a different wallet. It will be the designated proposer.");
    }
}

fn require_operator_balances(file: &DeployFile, wallets: &OperatorWallets) -> Result<(), String> {
    require_balance(&file.rpc_http, &wallets.deployer, "deployer")?;
    if !same_address(&wallets.proposer, &wallets.deployer) {
        require_balance(&file.rpc_http, &wallets.proposer, "proposer")?;
    }
    if !same_address(&wallets.client, &wallets.deployer) {
        require_balance(&file.rpc_http, &wallets.client, "client 1")?;
    }
    require_balance(&file.rpc_http, &file.client2_address, "client 2")?;
    Ok(())
}

fn validate_for_check(file: &DeployFile) -> Result<(), String> {
    network::validate_rpc_scheme(file.network, &file.rpc_wss)?;
    match file.role {
        Role::Operator => {
            require_key(&file.deployer_key, "NF4_DEPLOYER_KEY")?;
            require_key(&file.proposer_key, "NF4_PROPOSER_KEY")?;
            require_key(&file.client_key, "NF4_CLIENT_KEY")?;
            validate_optional_address(&file.expected_deployer_address, "NF4_DEPLOYER_ADDRESS")?;
            validate_optional_address(&file.expected_proposer_address, "NF4_PROPOSER_ADDRESS")?;
            validate_optional_address(&file.expected_client_address, "NF4_CLIENT_ADDRESS")?;
            if !is_address(&file.client2_address) {
                return Err(
                    "NF4_CLIENT2_ADDRESS must be Client 2's existing L1 address. Do not put the key here."
                        .to_string(),
                );
            }
            Ok(())
        }
        Role::Client => {
            require_key(&file.client_key, "NF4_CLIENT_KEY")?;
            validate_optional_address(&file.expected_client_address, "NF4_CLIENT_ADDRESS")?;
            Ok(())
        }
    }
}

fn check_wallets(file: &DeployFile) -> Result<(), String> {
    let chain_id = cast_u64("chain-id", &file.rpc_wss)?;
    if chain_id != file.chain_id {
        return Err(format!(
            "NF4_CHAIN_ID is {}, but the RPC chain ID is {}.",
            file.chain_id,
            network::chain_id_description(chain_id)
        ));
    }
    match file.role {
        Role::Operator => {
            let wallets = resolve_operator_wallets(file)?;
            print_wallets(&wallets, &file.client2_address);
            require_operator_balances(file, &wallets)?;
            network::verify_clean_mempool(&file.rpc_wss, &wallets.deployer)?;
            println!("Wallet check OK. No contracts were deployed.");
        }
        Role::Client => {
            let client_address = cast_address(&file.client_key)?;
            expect_address(
                &client_address,
                &file.expected_client_address,
                "NF4_CLIENT_ADDRESS",
            )?;
            println!("Client 2 wallet: {client_address}");
            println!("Send this line to VM-A. Do not send the key.");
            println!("NF4_CLIENT2_ADDRESS={client_address}");
            require_balance(&file.rpc_http, &client_address, "client 2")?;
            println!("Wallet check OK. Client 2 was not started.");
        }
    }
    Ok(())
}

pub fn expect_address(derived: &str, expected: &str, name: &str) -> Result<(), String> {
    if expected.trim().is_empty() {
        return Ok(());
    }
    if !is_address(expected) {
        return Err(format!("{name} must be a 20-byte hex address, or omitted."));
    }
    if !same_address(derived, expected) {
        return Err(format!(
            "{name} is {expected}, but the key in this file derives {derived}. Refusing to use the wrong wallet."
        ));
    }
    Ok(())
}

fn same_address(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

fn validate_optional_address(value: &str, name: &str) -> Result<(), String> {
    if value.trim().is_empty() || is_address(value) {
        Ok(())
    } else {
        Err(format!("{name} must be a 20-byte hex address, or omitted."))
    }
}

fn write_join(join: &JoinDoc) -> Result<(), String> {
    prepare_configuration_toml()?;
    remove_if_unwritable(Path::new(JOIN_TOML))?;
    fs::write(JOIN_TOML, render_join(join))
        .map_err(|err| format!("Failed to write {JOIN_TOML}: {err}"))
}

fn prepare_configuration_toml() -> Result<(), String> {
    let dir = Path::new("configuration/toml");
    let replaced = reclaim_unwritable_dir(dir)?;
    if replaced && configuration_container_running() {
        compose::recreate_configuration()?;
    }
    Ok(())
}

/// Docker creates this bind-mount source as root when it is missing. Rename
/// needs write permission on the parent, not on the directory, so a root-owned
/// `configuration/toml` can be replaced without a password when the repo is
/// writable. Readable files, including `addresses.toml`, are copied back.
fn reclaim_unwritable_dir(dir: &Path) -> Result<bool, String> {
    if is_writable_dir(dir) {
        return Ok(false);
    }
    if !dir.exists() {
        if let Some(parent) = dir.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
        }
        fs::create_dir_all(dir)
            .map_err(|err| format!("Failed to create {}: {err}", dir.display()))?;
        if !is_writable_dir(dir) {
            return Err(format!(
                "Created {} but it is still not writable",
                dir.display()
            ));
        }
        return Ok(false);
    }

    let parent = dir.parent().filter(|parent| !parent.as_os_str().is_empty());
    if parent.is_some_and(is_writable_dir) {
        let aside = aside_path(dir);
        fs::rename(dir, &aside).map_err(|err| {
            format!(
                "Failed to move unwritable {} to {}: {err}",
                dir.display(),
                aside.display()
            )
        })?;
        if let Err(err) = recreate_copied_dir(dir, &aside) {
            let _ = fs::remove_dir_all(dir);
            if let Err(restore) = fs::rename(&aside, dir) {
                return Err(format!(
                    "{err}. Also failed to restore {} from {}: {restore}",
                    dir.display(),
                    aside.display()
                ));
            }
            return Err(err);
        }
        println!(
            "Moved unwritable {} to {} and recreated it as the current user. Readable files were copied back. Not setting NF4_REDEPLOY.",
            dir.display(),
            aside.display()
        );
        return Ok(true);
    }

    if sudo_chown_dir(dir).is_ok() && is_writable_dir(dir) {
        println!(
            "Changed ownership of {} so this user can write join.toml. Not setting NF4_REDEPLOY.",
            dir.display()
        );
        return Ok(false);
    }
    Err(format!(
        "{} is not writable, and its parent is not writable. Run: sudo chown -R \"$USER:$USER\" {}. Do not set NF4_REDEPLOY and do not delete addresses.toml.",
        dir.display(),
        dir.display()
    ))
}

fn remove_if_unwritable(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    match fs::OpenOptions::new().write(true).open(path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            fs::remove_file(path).map_err(|rm| {
                format!(
                    "Cannot replace unwritable {}: {rm}. Not setting NF4_REDEPLOY.",
                    path.display()
                )
            })?;
            println!(
                "Removed unwritable {} so it can be rewritten.",
                path.display()
            );
            Ok(())
        }
        Err(err) => Err(format!("Failed to open {}: {err}", path.display())),
    }
}

fn is_writable_dir(dir: &Path) -> bool {
    if !dir.is_dir() {
        return false;
    }
    let probe = dir.join(".nf4-write-probe");
    let written = fs::write(&probe, b"ok").is_ok();
    if written {
        let _ = fs::remove_file(&probe);
    }
    written
}

fn aside_path(path: &Path) -> PathBuf {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "dir".to_string());
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let mut candidate = path.with_file_name(format!("{name}.unwritable-{secs}"));
    let mut n = 1;
    while candidate.exists() {
        candidate = path.with_file_name(format!("{name}.unwritable-{secs}-{n}"));
        n += 1;
    }
    candidate
}

fn recreate_copied_dir(dir: &Path, aside: &Path) -> Result<(), String> {
    fs::create_dir_all(dir)
        .map_err(|err| format!("Failed to create {}: {err}", dir.display()))?;
    copy_readable_tree(aside, dir)?;
    if !is_writable_dir(dir) {
        return Err(format!(
            "Recreated {} but it is still not writable",
            dir.display()
        ));
    }
    Ok(())
}

fn copy_readable_tree(from: &Path, to: &Path) -> Result<(), String> {
    for entry in
        fs::read_dir(from).map_err(|err| format!("Failed to read {}: {err}", from.display()))?
    {
        let entry = entry.map_err(|err| format!("Failed to read {}: {err}", from.display()))?;
        let src = entry.path();
        let dest = to.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|err| format!("Failed to inspect {}: {err}", src.display()))?;
        if file_type.is_dir() {
            fs::create_dir_all(&dest)
                .map_err(|err| format!("Failed to create {}: {err}", dest.display()))?;
            copy_readable_tree(&src, &dest)?;
        } else if file_type.is_file() {
            match fs::copy(&src, &dest) {
                Ok(_) => {}
                Err(err) if err.kind() == ErrorKind::PermissionDenied => {
                    if src.file_name().and_then(|name| name.to_str()) == Some("addresses.toml") {
                        return Err(format!(
                            "Cannot read {} to copy it into the recreated directory: {err}",
                            src.display()
                        ));
                    }
                    println!("Skipping unreadable {}", src.display());
                }
                Err(err) => return Err(format!("Failed to copy {}: {err}", src.display())),
            }
        }
    }
    Ok(())
}

fn sudo_chown_dir(dir: &Path) -> Result<(), String> {
    let user = command_stdout_trimmed("id", &["-un"])?;
    let group = command_stdout_trimmed("id", &["-gn"])?;
    let target = dir
        .to_str()
        .ok_or_else(|| format!("{} is not valid unicode", dir.display()))?;
    let status = Command::new("sudo")
        .args(["-n", "chown", "-R", &format!("{user}:{group}"), target])
        .status()
        .map_err(|err| format!("Failed to run sudo chown: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("sudo chown exited with {status}"))
    }
}

fn command_stdout_trimmed(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("Failed to run {program}: {err}"))?;
    if !output.status.success() {
        return Err(format!("{program} exited with {}", output.status));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let text = text.trim();
    if text.is_empty() {
        return Err(format!("{program} returned an empty name"));
    }
    Ok(text.to_string())
}

fn configuration_container_running() -> bool {
    Command::new("docker")
        .args(["inspect", "-f", "{{.State.Running}}", "nf4_configuration"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| String::from_utf8_lossy(&output.stdout).trim() == "true")
}

fn render_join(join: &JoinDoc) -> String {
    format!(
        "network = \"{}\"\nprofile = \"{}\"\nchain_id = {}\nconfiguration_url = \"{}\"\nproposer_url = \"{}\"\nmock_prover = {}\nblock_size = {}\ngenesis_block = {}\ndeployer_address = \"{}\"\ndefault_proposer_address = \"{}\"\n",
        join.network,
        join.profile,
        join.chain_id,
        join.configuration_url,
        join.proposer_url,
        join.mock_prover,
        join.block_size,
        join.genesis_block,
        join.deployer_address,
        join.default_proposer_address
    )
}

fn parse_join(source: &str) -> Result<JoinDoc, String> {
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse join.toml: {err}"))?;
    Ok(JoinDoc {
        network: join_str(&doc, "network")?,
        profile: join_str(&doc, "profile")?,
        chain_id: join_u64(&doc, "chain_id")?,
        configuration_url: join_str(&doc, "configuration_url")?,
        proposer_url: join_str(&doc, "proposer_url")?,
        mock_prover: doc
            .get("mock_prover")
            .and_then(Item::as_bool)
            .ok_or_else(|| "join.toml is missing mock_prover".to_string())?,
        block_size: join_u64(&doc, "block_size")?,
        genesis_block: join_u64(&doc, "genesis_block")?,
        deployer_address: join_str(&doc, "deployer_address")?,
        default_proposer_address: join_str(&doc, "default_proposer_address")?,
    })
}

fn write_public_env(
    file: &DeployFile,
    join: &JoinDoc,
    addresses: &BTreeMap<String, String>,
    client_address: &str,
) -> Result<(), String> {
    let mut preserved = BTreeMap::new();
    if let Ok(existing) = fs::read_to_string(PUBLIC_ENV) {
        for (key, value) in parse_env_text(&existing) {
            if matches!(
                key.as_str(),
                "TOKEN20"
                    | "TOKEN721"
                    | "TOKEN1155"
                    | "TOKEN3525"
                    | "C1_ZKP"
                    | "C2_ZKP"
                    | "ID20"
                    | "ID721_C1"
                    | "ID721_C2"
                    | "ID1155"
                    | "ID3525_C1"
                    | "ID3525_C2"
            ) {
                preserved.insert(key, value);
            }
        }
    }
    let mut lines = vec![
        format!("CONFIG_URL='{}'", join.configuration_url),
        format!("PROPOSER_URL='{}'", join.proposer_url),
        format!("OPERATOR_HOST='{}'", file.operator_host),
        format!("CHAIN_ID='{}'", join.chain_id),
        format!("PROFILE='{}'", join.profile),
        format!("MOCK_PROVER='{}'", join.mock_prover),
        format!("C1='{client_address}'"),
        format!("C2='{}'", file.client2_address),
        format!(
            "NF='{}'",
            addresses.get("nightfall").cloned().unwrap_or_default()
        ),
        format!(
            "ROUND_ROBIN='{}'",
            addresses.get("round_robin").cloned().unwrap_or_default()
        ),
        format!(
            "X509='{}'",
            addresses.get("x509").cloned().unwrap_or_default()
        ),
        format!(
            "VERIFIER='{}'",
            addresses.get("verifier").cloned().unwrap_or_default()
        ),
    ];
    for (key, value) in preserved {
        lines.push(format!("{key}='{value}'"));
    }
    lines.push(String::new());
    remove_if_unwritable(Path::new(PUBLIC_ENV))?;
    fs::write(PUBLIC_ENV, lines.join("\n"))
        .map_err(|err| format!("Failed to write {PUBLIC_ENV}: {err}"))
}

fn print_operator_review(config: &DeploymentConfig, client_address: &str, client2: &str) {
    println!();
    println!("Review deployment settings");
    println!("  network: {}", config.network.as_str());
    println!("  profile: {}", config.profile);
    println!("  chain_id: {}", config.chain_id);
    println!("  genesis_block: {}", config.genesis_block);
    println!("  configuration_url: {}", config.configuration_url);
    println!("  default_proposer_url: {}", config.default_proposer_url);
    println!("  deployer_address: {}", config.deployer_address);
    println!("  client1_address: {client_address}");
    println!("  client2_address: {client2}");
    println!("  prover_mode: {}", config.prover_mode());
    println!("  block_size: {}", config.block_size);
    println!();
}

fn require_balance(rpc_http: &str, address: &str, role: &str) -> Result<(), String> {
    let balance = network::account_balance_wei(rpc_http, address)?;
    let digits = balance
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches('0');
    if digits.is_empty() {
        return Err(format!(
            "{role} address {address} has 0 balance. Fund it, then rerun. About 0.5 Sepolia ETH."
        ));
    }
    println!("{role} balance: {balance}");
    Ok(())
}

fn read_addresses() -> Result<BTreeMap<String, String>, String> {
    let source = fs::read_to_string(ADDRESSES_TOML)
        .map_err(|err| format!("Failed to read {ADDRESSES_TOML}: {err}"))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {ADDRESSES_TOML}: {err}"))?;
    let mut addresses = BTreeMap::new();
    for key in ["nightfall", "round_robin", "x509", "verifier"] {
        let address = doc
            .get(key)
            .and_then(Item::as_str)
            .unwrap_or("")
            .to_string();
        addresses.insert(key.to_string(), address);
    }
    Ok(addresses)
}

fn profile_string(profile: &str, path: &[&str]) -> Result<String, String> {
    let source = fs::read_to_string(config::NIGHTFALL_TOML)
        .map_err(|err| format!("Failed to read {}: {err}", config::NIGHTFALL_TOML))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse {}: {err}", config::NIGHTFALL_TOML))?;
    let mut item = doc
        .get(profile)
        .ok_or_else(|| format!("nightfall.toml is missing [{profile}]"))?;
    for key in path {
        item = item
            .get(key)
            .ok_or_else(|| format!("nightfall.toml [{profile}] is missing {}", path.join(".")))?;
    }
    item.as_str().map(ToString::to_string).ok_or_else(|| {
        format!(
            "nightfall.toml [{}] {} is not a string",
            profile,
            path.join(".")
        )
    })
}

fn profile_u64(profile: &str, path: &[&str]) -> Result<u64, String> {
    let source = fs::read_to_string(config::NIGHTFALL_TOML)
        .map_err(|err| format!("Failed to read nightfall.toml: {err}"))?;
    let doc = source
        .parse::<DocumentMut>()
        .map_err(|err| format!("Failed to parse nightfall.toml: {err}"))?;
    let mut item = doc
        .get(profile)
        .ok_or_else(|| format!("nightfall.toml is missing [{profile}]"))?;
    for key in path {
        item = item
            .get(key)
            .ok_or_else(|| format!("nightfall.toml is missing {}", path.join(".")))?;
    }
    item.as_integer()
        .and_then(|value| u64::try_from(value).ok())
        .ok_or_else(|| format!("nightfall.toml {} is not an integer", path.join(".")))
}

fn download(url: &str, dest: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(dest).parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
    }
    let status = Command::new("curl")
        .args([
            "-fsS",
            "--retry",
            "5",
            "--retry-delay",
            "2",
            "--max-time",
            "180",
            "-o",
            dest,
            url,
        ])
        .status()
        .map_err(|err| format!("Failed to run curl: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to download {url}"))
    }
}

fn curl_text(url: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args([
            "-fsS",
            "--retry",
            "5",
            "--retry-delay",
            "2",
            "--max-time",
            "60",
            url,
        ])
        .output()
        .map_err(|err| format!("Failed to run curl: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "Failed to fetch {url}: {}",
            network::command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn curl_ok(url: &str) -> bool {
    Command::new("curl")
        .args(["-fsS", "--max-time", "5", "-o", "/dev/null", url])
        .status()
        .is_ok_and(|status| status.success())
}

fn cast_u64(command: &str, rpc_url: &str) -> Result<u64, String> {
    let output = Command::new("cast")
        .args([command, "--rpc-url", rpc_url])
        .output()
        .map_err(|err| format!("Failed to run cast {command}: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cast {command} failed: {}",
            network::command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    network::parse_u64_output(&output.stdout)
        .map_err(|err| format!("Failed to parse cast {command}: {err}"))
}

fn cast_address(private_key: &str) -> Result<String, String> {
    let output = Command::new("cast")
        .args(["wallet", "address", "--private-key", private_key])
        .output()
        .map_err(|err| format!("Failed to derive address: {err}"))?;
    if !output.status.success() {
        return Err(
            "cast wallet address failed. Check the private key in the env file.".to_string(),
        );
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("0x"))
        .map(ToString::to_string)
        .ok_or_else(|| "cast wallet address returned no address.".to_string())
}

fn required(env: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    env.get(key)
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{key} is required."))
}

fn require_key(value: &str, name: &str) -> Result<(), String> {
    let hex = value.trim().trim_start_matches("0x");
    if hex.len() == 64 && hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(format!("{name} must be a 32-byte hex private key."))
    }
}

fn is_address(value: &str) -> bool {
    let hex = value.trim().trim_start_matches("0x");
    hex.len() == 40 && hex.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value.trim() {
        "true" | "yes" | "1" => Ok(true),
        "false" | "no" | "0" | "" => Ok(false),
        other => Err(format!("Expected true or false, got {other}.")),
    }
}

fn parse_u64(value: &str, name: &str) -> Result<u64, String> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("{name} must be an integer."))
}

fn parse_port(value: Option<&String>, default: u16, name: &str) -> Result<u16, String> {
    let Some(value) = value else {
        return Ok(default);
    };
    if value.trim().is_empty() {
        return Ok(default);
    }
    value
        .trim()
        .parse::<u16>()
        .map_err(|_| format!("{name} must be a port."))
        .and_then(|port| {
            if port == 0 {
                Err(format!("{name} must be a port."))
            } else {
                Ok(port)
            }
        })
}

fn http_from_ws(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("wss://") {
        format!("https://{rest}")
    } else if let Some(rest) = url.strip_prefix("ws://") {
        format!("http://{rest}")
    } else {
        url.to_string()
    }
}

fn parse_env_text(source: &str) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        values.insert(
            key.trim().to_string(),
            strip_quotes(value.trim()).to_string(),
        );
    }
    values
}

fn strip_quotes(value: &str) -> &str {
    value
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
        .or_else(|| {
            value
                .strip_prefix('"')
                .and_then(|value| value.strip_suffix('"'))
        })
        .unwrap_or(value)
}

fn join_str(doc: &DocumentMut, key: &str) -> Result<String, String> {
    doc.get(key)
        .and_then(Item::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("join.toml is missing {key}"))
}

fn join_u64(doc: &DocumentMut, key: &str) -> Result<u64, String> {
    doc.get(key)
        .and_then(Item::as_integer)
        .and_then(|value| u64::try_from(value).ok())
        .ok_or_else(|| format!("join.toml is missing {key}"))
}

fn url_host(url: &str) -> Option<&str> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let host = rest.split(['/', ':', '?']).next()?;
    if host.is_empty() {
        None
    } else {
        Some(host)
    }
}

fn url_port(url: &str) -> Option<u16> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let host_port = rest.split(['/', '?']).next()?;
    host_port.split(':').nth(1)?.parse().ok()
}

#[cfg(test)]
fn refresh_app(
    container: &str,
    stamp_path: &str,
    roots: &[&str],
    port: u16,
    start: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let fingerprint = source_fingerprint(roots)?;
    let image = container_image_id(container);
    let health = format!("http://127.0.0.1:{port}/v1/health");
    if curl_ok(&health) && stamp_matches(stamp_path, &fingerprint, image.as_deref()) {
        println!("{container} already healthy on the current source. Not rebuilding.");
        return Ok(());
    }
    if curl_ok(&health) {
        println!(
            "{container} is healthy, but it is not the current source. Rebuilding before certification."
        );
    }
    start()?;
    let image = container_image_id(container).ok_or_else(|| {
        format!("{container} did not start, so its image stamp was not written")
    })?;
    write_stamp(stamp_path, &fingerprint, &image)
}

fn stamp_matches(path: &str, fingerprint: &str, image: Option<&str>) -> bool {
    let Some(image) = image else {
        return false;
    };
    let Ok(text) = fs::read_to_string(path) else {
        return false;
    };
    let mut lines = text.lines();
    lines.next() == Some(fingerprint) && lines.next() == Some(image)
}

fn write_stamp(path: &str, fingerprint: &str, image: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, format!("{fingerprint}\n{image}\n"))
        .map_err(|err| format!("Failed to write {path}: {err}"))
}

fn container_image_id(container: &str) -> Option<String> {
    let output = Command::new("docker")
        .args(["inspect", "-f", "{{.Image}}", container])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if id.is_empty() { None } else { Some(id) }
}

pub(crate) fn source_fingerprint(roots: &[&str]) -> Result<String, String> {
    let mut files = Vec::new();
    for root in roots {
        let path = Path::new(root);
        if path.is_file() {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            collect_source_files(path, &mut files)?;
        }
    }
    files.sort();
    let mut hash: u64 = 0xcbf29ce484222325;
    for path in files {
        let bytes = fs::read(&path)
            .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
        hash = fnv1a(hash, path.to_string_lossy().as_bytes());
        hash = fnv1a(hash, &bytes);
    }
    Ok(format!("{hash:016x}"))
}

fn collect_source_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if path
        .file_name()
        .is_some_and(|name| name == "target" || name == ".git")
    {
        return Ok(());
    }
    let meta = fs::metadata(path)
        .map_err(|err| format!("Failed to stat {}: {err}", path.display()))?;
    if meta.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|err| format!("Failed to read {}: {err}", path.display()))?
        {
            let entry = entry.map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
            collect_source_files(&entry.path(), files)?;
        }
        return Ok(());
    }
    if meta.is_file() {
        files.push(path.to_path_buf());
    }
    Ok(())
}

fn fnv1a(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

mod tests {
    use super::{
        choose_operator_host, expect_address, is_writable_dir, operator_host_candidates,
        parse_deploy_file, parse_join, reclaim_unwritable_dir, render_join, source_fingerprint,
        split_vm_host_error, stamp_matches, validate_deploy_file, JoinDoc,
    };
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn operator_env() -> String {
        r#"
NF4_ROLE=operator
NF4_NETWORK=testnet
NF4_PROFILE=sepolia
NF4_CHAIN_ID=11155111
NF4_RPC_WSS=wss://example.invalid/ws
NF4_OPERATOR_HOST=10.0.0.8
NF4_CONFIRM=apply
NF4_DEPLOYER_KEY=0x1111111111111111111111111111111111111111111111111111111111111111
NF4_CLIENT2_ADDRESS=0x2222222222222222222222222222222222222222
"#
        .to_string()
    }

    #[test]
    fn a_healthy_container_without_the_current_stamp_is_not_current() {
        assert!(!stamp_matches(
            "temp/does-not-exist-nf4.stamp",
            "abc",
            Some("sha256:old")
        ));
        assert!(!stamp_matches(
            "temp/does-not-exist-nf4.stamp",
            "abc",
            None
        ));
        let stamp = std::env::temp_dir().join(format!(
            "nf4-stamp-match-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::write(&stamp, "abc\nsha256:new\n").unwrap();
        assert!(stamp_matches(stamp.to_str().unwrap(), "abc", Some("sha256:new")));
        assert!(!stamp_matches(
            stamp.to_str().unwrap(),
            "abc",
            Some("sha256:old")
        ));
        let _ = fs::remove_file(&stamp);
    }

    #[test]
    fn source_fingerprint_changes_when_a_file_changes() {
        let dir = std::env::temp_dir().join(format!(
            "nf4-stamp-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("lib.rs");
        fs::write(&file, b"estimate without from").unwrap();
        let before = source_fingerprint(&[dir.to_str().unwrap()]).unwrap();
        fs::write(&file, b"estimate from signer").unwrap();
        let after = source_fingerprint(&[dir.to_str().unwrap()]).unwrap();
        assert_ne!(before, after);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parses_operator_file_and_defaults_client_key() {
        let file = parse_deploy_file(&operator_env()).unwrap();
        assert_eq!(file.client_key, file.deployer_key);
        assert_eq!(file.rpc_http, "https://example.invalid/ws");
        assert!(validate_deploy_file(&file).is_ok());
    }

    #[test]
    fn rejects_loopback_operator_host_and_missing_confirm() {
        let mut source = operator_env();
        source = source.replace("10.0.0.8", "127.0.0.1");
        let file = parse_deploy_file(&source).unwrap();
        assert!(validate_deploy_file(&file).is_err());
        assert!(split_vm_host_error("172.28.0.2").is_some());
        assert!(split_vm_host_error("10.1.2.3").is_none());

        let file =
            parse_deploy_file(&operator_env().replace("NF4_CONFIRM=apply", "NF4_CONFIRM=yes"))
                .unwrap();
        assert!(validate_deploy_file(&file)
            .unwrap_err()
            .contains("NF4_CONFIRM=apply"));
    }

    #[test]
    fn detects_operator_host_and_prefers_a_reachable_lan_address() {
        let detected = operator_host_candidates(
            "127.0.0.1 172.17.0.1 172.28.0.1 169.254.1.1 203.0.113.8 10.0.0.8",
        );
        assert_eq!(
            detected,
            vec!["203.0.113.8".to_string(), "10.0.0.8".to_string()]
        );
        assert_eq!(
            choose_operator_host("", &detected, &[true, true]).unwrap(),
            "10.0.0.8"
        );
        assert_eq!(
            choose_operator_host("", &detected, &[true, false]).unwrap(),
            "203.0.113.8"
        );
        assert_eq!(
            choose_operator_host("10.1.1.1", &detected, &[false, false]).unwrap(),
            "10.1.1.1"
        );
        assert!(choose_operator_host("", &detected, &[false, false]).is_err());
    }

    #[test]
    fn existing_wallet_pin_must_match_the_key() {
        assert!(expect_address("0xAbCd", "", "NF4_DEPLOYER_ADDRESS").is_ok());
        assert!(expect_address(
            "0x2222222222222222222222222222222222222222",
            "0x2222222222222222222222222222222222222222",
            "NF4_DEPLOYER_ADDRESS"
        )
        .is_ok());
        assert!(expect_address(
            "0x2222222222222222222222222222222222222222",
            "0x3333333333333333333333333333333333333333",
            "NF4_DEPLOYER_ADDRESS"
        )
        .unwrap_err()
        .contains("wrong wallet"));
    }

    #[test]
    fn client_role_refuses_to_start_before_late_join() {
        let source = r#"
NF4_ROLE=client
NF4_NETWORK=testnet
NF4_RPC_WSS=wss://example.invalid/ws
NF4_CONFIRM=apply
NF4_JOIN_URL=http://10.0.0.8:8080/configuration/toml/join.toml
NF4_CLIENT_KEY=0x1111111111111111111111111111111111111111111111111111111111111111
NF4_LATE_JOIN=false
"#;
        let file = parse_deploy_file(source).unwrap();
        assert!(validate_deploy_file(&file)
            .unwrap_err()
            .contains("NF4_LATE_JOIN"));
    }

    #[test]
    fn join_roundtrip_has_no_secret_fields() {
        let join = JoinDoc {
            network: "testnet".to_string(),
            profile: "sepolia".to_string(),
            chain_id: 11155111,
            configuration_url: "http://10.0.0.8:8080".to_string(),
            proposer_url: "http://10.0.0.8:3001".to_string(),
            mock_prover: false,
            block_size: 64,
            genesis_block: 9,
            deployer_address: "0x2222222222222222222222222222222222222222".to_string(),
            default_proposer_address: "0x2222222222222222222222222222222222222222".to_string(),
        };
        let rendered = render_join(&join);
        assert!(!rendered.to_ascii_lowercase().contains("key"));
        assert!(!rendered.contains("1111111111111111"));
        assert_eq!(parse_join(&rendered).unwrap(), join);
    }

    #[test]
    fn reclaims_unwritable_configuration_toml_and_keeps_addresses() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("nf4-toml-reclaim-{stamp}"));
        let dir = root.join("configuration").join("toml");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("addresses.toml"), "nightfall = \"0xabc\"\n").unwrap();
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o555)).unwrap();
        assert!(!is_writable_dir(&dir));

        assert!(reclaim_unwritable_dir(&dir).unwrap());
        assert!(is_writable_dir(&dir));
        assert_eq!(
            fs::read_to_string(dir.join("addresses.toml")).unwrap(),
            "nightfall = \"0xabc\"\n"
        );
        assert!(fs::write(dir.join("join.toml"), "network = \"testnet\"\n").is_ok());
        assert!(fs::read_dir(root.join("configuration"))
            .unwrap()
            .any(|entry| {
                entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("toml.unwritable-")
            }));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn leaves_a_writable_configuration_toml_in_place() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("nf4-toml-writable-{stamp}"));
        fs::create_dir_all(&dir).unwrap();
        assert!(!reclaim_unwritable_dir(&dir).unwrap());
        assert!(is_writable_dir(&dir));
        let aside_prefix = format!("{}.unwritable-", dir.file_name().unwrap().to_string_lossy());
        assert!(!dir.parent().unwrap().read_dir().unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(&aside_prefix)
        }));
        let _ = fs::remove_dir_all(&dir);
    }
}
