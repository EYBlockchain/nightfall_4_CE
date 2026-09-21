use std::collections::BTreeMap;
use std::process::Command;

use inquire::{Confirm, Password};

/// Foundry Anvil account 0. Local --yes and the local e2e script only.
pub const LOCAL_ANVIL_ACCOUNT0_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const LOCAL_ANVIL_ACCOUNT0_ADDRESS: &str =
    "0xf39Fd6e51aad88F6F4ce6ab8827279cffFb92266";
/// Foundry Anvil account 1 (Client 2 L1).
pub const LOCAL_ANVIL_ACCOUNT1_KEY: &str =
    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
pub const LOCAL_ANVIL_ACCOUNT1_ADDRESS: &str =
    "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NetworkKind {
    Local,
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChainIdMatch {
    Known,
    Unknown,
    Mismatch,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ResolvedNetwork {
    pub kind: Option<NetworkKind>,
    pub source: &'static str,
}

impl NetworkKind {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "testnet" => Ok(Self::Testnet),
            "mainnet" => Ok(Self::Mainnet),
            _ => Err(format!(
                "Unknown network: {value}. Use local, testnet, or mainnet."
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Testnet => "testnet",
            Self::Mainnet => "mainnet",
        }
    }

    pub fn default_profile(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Testnet => "sepolia",
            Self::Mainnet => "mainnet",
        }
    }

    pub fn template_profile(self) -> &'static str {
        match self {
            Self::Local => "development",
            Self::Testnet | Self::Mainnet => "base_sepolia",
        }
    }

    pub fn default_rpc_url(self) -> Option<&'static str> {
        match self {
            Self::Local => Some("ws://127.0.0.1:8545"),
            Self::Testnet | Self::Mainnet => None,
        }
    }

    pub fn default_mock_prover(self) -> bool {
        !matches!(self, Self::Mainnet)
    }

    pub fn allows_mock_prover(self) -> bool {
        !matches!(self, Self::Mainnet)
    }

    pub fn requires_typed_confirm(self) -> bool {
        matches!(self, Self::Mainnet)
    }

    pub fn typed_confirm_phrase(self) -> Option<&'static str> {
        match self {
            Self::Mainnet => Some("deploy to mainnet"),
            Self::Local | Self::Testnet => None,
        }
    }

    pub fn key_prompt_role(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Testnet => "testnet",
            Self::Mainnet => "mainnet",
        }
    }

    pub fn classify_chain_id(self, chain_id: u64) -> ChainIdMatch {
        match (self, kind_for_chain_id(chain_id)) {
            (kind, Some(actual)) if kind == actual => ChainIdMatch::Known,
            (_, Some(_)) => ChainIdMatch::Mismatch,
            (Self::Local, None) => ChainIdMatch::Mismatch,
            (Self::Testnet | Self::Mainnet, None) => ChainIdMatch::Unknown,
        }
    }

    pub fn mismatch_message(self, chain_id: u64) -> String {
        let actual = chain_id_description(chain_id);
        format!(
            "You chose {}, but the RPC chain ID is {actual}. Aborting.",
            self.as_str()
        )
    }
}

impl ResolvedNetwork {
    pub fn label(self) -> &'static str {
        self.kind.map(NetworkKind::as_str).unwrap_or("unknown")
    }
}

pub fn kind_for_chain_id(chain_id: u64) -> Option<NetworkKind> {
    match chain_id {
        31337 | 1337 => Some(NetworkKind::Local),
        11155111 | 84532 => Some(NetworkKind::Testnet),
        1 | 8453 => Some(NetworkKind::Mainnet),
        _ => None,
    }
}

pub fn chain_id_name(chain_id: u64) -> Option<&'static str> {
    match chain_id {
        31337 | 1337 => Some("Anvil"),
        11155111 => Some("Sepolia"),
        84532 => Some("Base Sepolia"),
        1 => Some("Ethereum"),
        8453 => Some("Base"),
        _ => None,
    }
}

pub fn chain_id_description(chain_id: u64) -> String {
    match chain_id_name(chain_id) {
        Some(name) => format!("{chain_id} ({name})"),
        None => chain_id.to_string(),
    }
}

pub fn resolve_network(env_network: Option<&str>, chain_id: Option<u64>) -> ResolvedNetwork {
    if let Some(value) = env_network {
        if let Ok(kind) = NetworkKind::parse(value) {
            return ResolvedNetwork {
                kind: Some(kind),
                source: "local.env",
            };
        }
    }
    if let Some(chain_id) = chain_id {
        if let Some(kind) = kind_for_chain_id(chain_id) {
            return ResolvedNetwork {
                kind: Some(kind),
                source: "chain ID",
            };
        }
    }
    ResolvedNetwork {
        kind: None,
        source: "unknown",
    }
}

pub fn resolve_network_from_env(
    env: &BTreeMap<String, String>,
    chain_id: Option<u64>,
) -> ResolvedNetwork {
    resolve_network(env.get("NF4_NETWORK").map(String::as_str), chain_id)
}

pub fn validate_profile_name(profile: &str) -> Result<(), String> {
    if profile.is_empty()
        || !profile
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err("Profile name must use lowercase letters, digits, or underscores.".to_string());
    }
    if matches!(profile, "development" | "production") {
        return Err(format!(
            "Profile name `{profile}` is reserved. Choose a different name, such as local or sepolia."
        ));
    }
    Ok(())
}

pub fn validate_rpc_scheme(network: NetworkKind, rpc_url: &str) -> Result<(), String> {
    match network {
        NetworkKind::Mainnet if rpc_url.starts_with("wss://") => Ok(()),
        NetworkKind::Mainnet => Err("Mainnet RPC URL must start with wss://.".to_string()),
        _ if rpc_url.starts_with("ws://") || rpc_url.starts_with("wss://") => Ok(()),
        _ => Err("RPC URL must start with ws:// or wss://.".to_string()),
    }
}

pub fn validate_published_configuration_url(network: NetworkKind, url: &str) -> Result<(), String> {
    let url = url.trim();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("Configuration URL must include http:// or https:// and a host.".to_string());
    }
    if url.contains("/configuration/") {
        return Err(
            "Configuration URL must be the base URL, not a /configuration/... path.".to_string(),
        );
    }

    let host = url_host(url).ok_or_else(|| "Configuration URL must include a host.".to_string())?;
    if is_docker_internal_host(host) {
        return Err(format!(
            "{host} is a Docker-internal name. Use a host-reachable URL such as http://127.0.0.1:8080."
        ));
    }

    let has_port = url_port(url).is_some();
    if !has_port && !url.starts_with("https://") {
        return Err(
            "Configuration URL must include the exposed port, for example http://127.0.0.1:8080."
                .to_string(),
        );
    }

    if network == NetworkKind::Mainnet {
        if !url.starts_with("https://") {
            return Err("Mainnet configuration URL must use https://.".to_string());
        }
        if is_private_or_local_host(host) {
            return Err(
                "Mainnet configuration URL cannot be a private or local address.".to_string(),
            );
        }
    }

    Ok(())
}

pub fn compose_rpc_url(network: NetworkKind, rpc_url: &str) -> String {
    if network != NetworkKind::Local {
        return rpc_url.to_string();
    }
    match url_host(rpc_url) {
        Some(host) if is_loopback_host(host) => rewrite_url_host(rpc_url, "anvil"),
        _ => rpc_url.to_string(),
    }
}

pub fn host_reachable_rpc_url(rpc_url: &str) -> String {
    host_reachable_url(rpc_url)
}

/// Convert ws/wss RPC URLs to http/https for Foundry broadcast.
/// Nightfall event listeners still require WebSocket; forge script does not.
pub fn http_rpc_url(url: &str) -> String {
    let scheme_end = url.find("://").map(|i| i + 3).unwrap_or(0);
    let scheme = url[..scheme_end.min(url.len())].to_ascii_lowercase();
    if scheme == "wss://" {
        format!("https://{}", &url[scheme_end..])
    } else if scheme == "ws://" {
        format!("http://{}", &url[scheme_end..])
    } else {
        url.to_string()
    }
}

pub fn host_reachable_url(url: &str) -> String {
    match url_host(url) {
        Some(host) if is_container_only_host(host) => rewrite_url_host(url, "127.0.0.1"),
        _ => url.to_string(),
    }
}

/// Rewrite Docker-only or loopback hosts to a LAN host for wizard defaults.
/// `nf4` prompts on the host; containers cannot use `host.docker.internal` or `127.0.0.1`.
pub fn published_url_for_lan(url: &str, lan_host: &str) -> String {
    if lan_host.is_empty() || is_loopback_host(lan_host) {
        return url.to_string();
    }
    match url_host(url) {
        Some(host) if is_container_only_host(host) || is_loopback_host(host) => {
            rewrite_url_host(url, lan_host)
        }
        _ => url.to_string(),
    }
}

/// Replace loopback, Docker-only, and private/LAN hosts with the current LAN IP.
/// Public hostnames are left unchanged.
pub fn with_lan_host(url: &str, lan_host: &str) -> String {
    if lan_host.is_empty() || is_loopback_host(lan_host) {
        return published_url_for_lan(url, lan_host);
    }
    match url_host(url) {
        Some(host) if should_refresh_published_host(host) => rewrite_url_host(url, lan_host),
        _ => url.to_string(),
    }
}

fn should_refresh_published_host(host: &str) -> bool {
    is_loopback_host(host) || is_container_only_host(host) || is_private_or_local_host(host)
}

pub fn detect_lan_host() -> String {
    hostname_ip()
        .or_else(|| ipconfig_ip("en0"))
        .or_else(|| ipconfig_ip("en1"))
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

pub fn host_from_hostname_output(output: &str) -> Option<String> {
    output
        .split_whitespace()
        .find(|ip| !ip.starts_with("127.") && !ip.starts_with("172."))
        .map(ToString::to_string)
}

fn hostname_ip() -> Option<String> {
    let output = Command::new("hostname").arg("-I").output().ok()?;
    if !output.status.success() {
        return None;
    }
    host_from_hostname_output(&String::from_utf8_lossy(&output.stdout))
}

fn ipconfig_ip(interface: &str) -> Option<String> {
    let output = Command::new("ipconfig")
        .args(["getifaddr", interface])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .filter(|ip| !ip.starts_with("127."))
        .map(ToString::to_string)
}

pub fn is_container_only_host(host: &str) -> bool {
    is_docker_internal_host(host)
        || matches!(
            host,
            "host.docker.internal" | "docker.for.mac.localhost" | "gateway.docker.internal"
        )
}

pub fn is_docker_internal_host(host: &str) -> bool {
    matches!(
        host,
        "configuration"
            | "nf4_configuration"
            | "anvil"
            | "proposer"
            | "proposer2"
            | "indie-proposer"
            | "client"
            | "client2"
            | "nf4_indie_proposer"
            | "nf4_indie_client"
            | "nf4_indie_client2"
    )
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RpcFailure {
    ConnectionRefused,
    UnreachableHost,
    Timeout,
    Tls,
    Unauthorized,
    RateLimited,
    DockerInternal,
    Unknown,
}

pub fn classify_rpc_failure(rpc_url: &str, detail: &str) -> RpcFailure {
    if url_host(rpc_url).is_some_and(is_docker_internal_host) {
        return RpcFailure::DockerInternal;
    }

    let detail = detail.to_ascii_lowercase();
    if contains_any(
        &detail,
        &["connection refused", "econnrefused", "os error 61", "actively refused"],
    ) {
        return RpcFailure::ConnectionRefused;
    }
    if contains_any(
        &detail,
        &[
            "could not resolve",
            "failed to lookup",
            "nodename nor servname",
            "name or service not known",
            "no such host",
        ],
    ) {
        return RpcFailure::UnreachableHost;
    }
    if contains_any(&detail, &["timed out", "timeout", "deadline exceeded"]) {
        return RpcFailure::Timeout;
    }
    if contains_any(&detail, &["certificate", "tls", "ssl handshake"]) {
        return RpcFailure::Tls;
    }
    if contains_any(
        &detail,
        &["unauthorized", "invalid api key", "status code: 401", "status code: 403"],
    ) {
        return RpcFailure::Unauthorized;
    }
    if contains_any(&detail, &["rate limit", "too many requests", "status code: 429"]) {
        return RpcFailure::RateLimited;
    }
    RpcFailure::Unknown
}

pub fn explain_rpc_error(
    network: Option<NetworkKind>,
    rpc_url: &str,
    detail: &str,
) -> String {
    let failure = classify_rpc_failure(rpc_url, detail);
    format!(
        "{}\n\nNext:\n{}\n\nDetail: {detail}",
        rpc_failure_summary(failure, network, rpc_url),
        rpc_failure_next_step(failure, network, rpc_url)
    )
}

fn rpc_failure_summary(
    failure: RpcFailure,
    network: Option<NetworkKind>,
    rpc_url: &str,
) -> String {
    match failure {
        RpcFailure::ConnectionRefused if network == Some(NetworkKind::Local) => {
            format!("Could not reach the local Anvil RPC at {rpc_url}.")
        }
        RpcFailure::ConnectionRefused => {
            format!("Could not connect to the RPC at {rpc_url}.")
        }
        RpcFailure::UnreachableHost => {
            format!("The RPC hostname in {rpc_url} could not be resolved.")
        }
        RpcFailure::Timeout => {
            format!("The RPC at {rpc_url} timed out.")
        }
        RpcFailure::Tls => {
            format!("TLS/SSL failed for the RPC at {rpc_url}.")
        }
        RpcFailure::Unauthorized => {
            "The RPC rejected this request (auth or API key).".to_string()
        }
        RpcFailure::RateLimited => "The RPC rate-limited this request.".to_string(),
        RpcFailure::DockerInternal => {
            format!("{rpc_url} is a Docker-internal name and is not reachable from the host.")
        }
        RpcFailure::Unknown => format!("The RPC request to {rpc_url} failed."),
    }
}

fn rpc_failure_next_step(
    failure: RpcFailure,
    network: Option<NetworkKind>,
    rpc_url: &str,
) -> String {
    match (failure, network) {
        (RpcFailure::ConnectionRefused, Some(NetworkKind::Local)) => {
            "  Start Anvil, then confirm it answers:\n    docker compose --profile anvil up -d\n    cast chain-id --rpc-url ws://127.0.0.1:8545\n  Then retry:\n    ./scripts/nf4 wizard deploy --network local"
                .to_string()
        }
        (RpcFailure::DockerInternal, _) => {
            "  From the host, use ws://127.0.0.1:8545 for Anvil or the published configuration port.\n  Docker service names such as anvil or configuration only work inside Compose."
                .to_string()
        }
        (RpcFailure::UnreachableHost, _) => {
            "  Check the RPC URL for typos and that this machine can resolve the hostname."
                .to_string()
        }
        (RpcFailure::Unauthorized, _) | (RpcFailure::RateLimited, _) => {
            "  Check the RPC API key, plan limits, and that the URL scheme is wss:// for public testnet/mainnet."
                .to_string()
        }
        (RpcFailure::Tls, _) => {
            "  Use a wss:// URL for public RPCs and check for a proxy or TLS intercepting the connection."
                .to_string()
        }
        (RpcFailure::Timeout, Some(NetworkKind::Local)) => {
            "  Check that Anvil is running and not stuck:\n    docker compose --profile anvil ps\n    docker compose --profile anvil logs anvil"
                .to_string()
        }
        _ => format!(
            "  Confirm the RPC with:\n    cast chain-id --rpc-url {rpc_url}\n  Then retry the nf4 command."
        ),
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

pub fn parse_u64_output(bytes: &[u8]) -> Result<u64, String> {
    numeric_line(bytes)
        .ok_or_else(|| "command returned no numeric output".to_string())?
        .parse::<u64>()
        .map_err(|err| format!("failed to parse numeric output: {err}"))
}

pub fn command_failure_detail(output: &std::process::Output) -> Option<String> {
    command_failure_detail_from_bytes(&output.stderr, &output.stdout)
}

fn command_failure_detail_from_bytes(stderr: &[u8], stdout: &[u8]) -> Option<String> {
    useful_lines(stderr)
        .into_iter()
        .chain(useful_lines(stdout))
        .find(|line| line.to_ascii_lowercase().contains("error"))
        .or_else(|| {
            useful_lines(stderr)
                .into_iter()
                .chain(useful_lines(stdout))
                .next()
        })
}

fn useful_lines(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("Warning:"))
        .map(ToString::to_string)
        .collect()
}

fn numeric_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && line.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToString::to_string)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedWallet {
    pub address: String,
    pub private_key: String,
}

pub fn parse_cast_wallet_new(output: &str) -> Result<GeneratedWallet, String> {
    let mut address = None;
    let mut private_key = None;
    for line in output.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Address:") {
            address = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("Private key:") {
            private_key = Some(rest.trim().to_string());
        }
    }
    match (address, private_key) {
        (Some(address), Some(private_key))
            if address.starts_with("0x") && private_key.starts_with("0x") =>
        {
            Ok(GeneratedWallet {
                address,
                private_key,
            })
        }
        _ => Err("cast wallet new did not print an address and private key.".to_string()),
    }
}

pub fn generate_wallet() -> Result<GeneratedWallet, String> {
    let output = Command::new("cast")
        .args(["wallet", "new"])
        .output()
        .map_err(|err| format!("Failed to run cast wallet new: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cast wallet new failed: {}",
            command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    parse_cast_wallet_new(&String::from_utf8_lossy(&output.stdout))
}

pub fn account_balance_wei(rpc_url: &str, address: &str) -> Result<String, String> {
    let rpc = host_reachable_rpc_url(rpc_url);
    let output = Command::new("cast")
        .args(["balance", address, "--rpc-url", &rpc])
        .output()
        .map_err(|err| format!("Failed to run cast balance: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cast balance failed: {}",
            command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("Warning:"))
        .map(ToString::to_string)
        .ok_or_else(|| "cast balance returned no output.".to_string())
}

pub fn account_nonce(rpc_url: &str, address: &str, block_tag: &str) -> Result<u64, String> {
    let rpc = host_reachable_rpc_url(rpc_url);
    let output = Command::new("cast")
        .args(["nonce", address, "--block", block_tag, "--rpc-url", &rpc])
        .output()
        .map_err(|err| format!("Failed to run cast nonce: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cast nonce failed: {}",
            command_failure_detail(&output).unwrap_or_else(|| "no output".to_string())
        ));
    }
    parse_u64_output(&output.stdout).map_err(|err| format!("Failed to parse cast nonce: {err}"))
}

pub fn verify_clean_mempool(rpc_url: &str, address: &str) -> Result<(), String> {
    let latest = account_nonce(rpc_url, address, "latest")?;
    let pending = account_nonce(rpc_url, address, "pending")?;
    if pending > latest {
        return Err(format!(
            "Deployer address {address} has pending transactions in mempool (confirmed nonce: {latest}, pending nonce: {pending}). \
             Wait for pending transactions to confirm or clear them before deploying."
        ));
    }
    Ok(())
}

fn balance_is_zero(balance: &str) -> bool {
    let digits = balance.trim().trim_start_matches("0x").trim_start_matches('0');
    digits.is_empty()
}

/// Paste a key, reuse local.env, or (testnet only) generate a new account and wait for funding.
pub fn prompt_l1_signing_key(
    role: &str,
    network: NetworkKind,
    existing: Option<&str>,
    rpc_url: &str,
) -> Result<String, String> {
    let existing = existing.map(str::trim).filter(|value| !value.is_empty());
    let can_generate = network == NetworkKind::Testnet;

    println!();
    if can_generate {
        println!(
            "Paste a funded L1 testnet {role} private key, or press Enter to generate a new account."
        );
        println!(
            "A generated account starts with 0 ETH. Fund the printed address before Nightfall sends transactions."
        );
    } else {
        println!(
            "Paste the funded L1 {} {role} private key.",
            network.key_prompt_role()
        );
    }
    println!("Input is hidden if you paste a key, so nothing will appear while typing.");
    println!("Include 0x if your key has it.");
    println!("Never share this key in chat or commit local.env.");

    let prompt = if existing.is_some() {
        format!("{role} private key [hidden; Enter to reuse local.env]")
    } else if can_generate {
        format!("{role} private key [hidden; Enter to generate a new testnet account]")
    } else {
        format!("{role} private key [hidden input]")
    };

    let value = Password::new(&prompt)
        .without_confirmation()
        .prompt()
        .map_err(|err| err.to_string())?;
    let value = value.trim();

    if !value.is_empty() {
        return Ok(value.to_string());
    }
    if let Some(existing) = existing {
        println!("Reusing {role} key from local.env.");
        return Ok(existing.to_string());
    }
    if can_generate {
        return generate_and_confirm_testnet_key(role, rpc_url);
    }
    Err(format!("{role} private key is required."))
}

fn generate_and_confirm_testnet_key(role: &str, rpc_url: &str) -> Result<String, String> {
    println!();
    println!("No {role} key pasted. Generating a new testnet account with `cast wallet new`...");
    let wallet = generate_wallet()?;
    println!();
    println!("Generated a new testnet {role} account.");
    println!("  address:     {}", wallet.address);
    println!("  private key: {}", wallet.private_key);
    println!();
    println!(
        "This account has no ETH yet. Fund {} , then continue.",
        wallet.address
    );
    println!("The key is shown once here and will be written to local.env (gitignored).");
    println!("Do not commit local.env or paste the key into chat or a report.");
    wait_until_funded(rpc_url, &wallet.address)?;
    Ok(wallet.private_key)
}

fn wait_until_funded(rpc_url: &str, address: &str) -> Result<(), String> {
    loop {
        match account_balance_wei(rpc_url, address) {
            Ok(balance) if !balance_is_zero(&balance) => {
                println!("  on-chain balance: {balance} wei");
                return Ok(());
            }
            Ok(_) => println!("  {address} still has 0 wei."),
            Err(err) => println!("  could not read balance: {err}"),
        }
        let again = Confirm::new("Recheck balance after funding?")
            .with_default(true)
            .prompt()
            .map_err(|err| err.to_string())?;
        if !again {
            println!(
                "  Continuing without a confirmed balance. Later transactions will fail until this address is funded."
            );
            return Ok(());
        }
    }
}

fn url_host(url: &str) -> Option<&str> {
    let (_, rest) = url.split_once("://")?;
    let authority = rest.split('/').next().unwrap_or(rest);
    if authority.is_empty() {
        return None;
    }
    match authority.rsplit_once(':') {
        Some((host, port)) if port.chars().all(|ch| ch.is_ascii_digit()) && !host.is_empty() => {
            Some(host)
        }
        _ => Some(authority),
    }
}

fn url_port(url: &str) -> Option<u16> {
    let (_, rest) = url.split_once("://")?;
    let authority = rest.split('/').next().unwrap_or(rest);
    authority.rsplit_once(':')?.1.parse().ok()
}

fn rewrite_url_host(url: &str, new_host: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else {
        return url.to_string();
    };
    let authority = rest.split('/').next().unwrap_or(rest);
    let path = rest.strip_prefix(authority).unwrap_or("");
    let rewritten = match authority.rsplit_once(':') {
        Some((_, port)) if port.chars().all(|ch| ch.is_ascii_digit()) => {
            format!("{scheme}://{new_host}:{port}{path}")
        }
        _ => format!("{scheme}://{new_host}{path}"),
    };
    rewritten
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1")
}

fn is_private_or_local_host(host: &str) -> bool {
    if is_loopback_host(host) || is_docker_internal_host(host) || host == "0.0.0.0" {
        return true;
    }
    if host.starts_with("10.") || host.starts_with("192.168.") {
        return true;
    }
    if let Some(rest) = host.strip_prefix("172.") {
        if let Some(second) = rest
            .split('.')
            .next()
            .and_then(|part| part.parse::<u8>().ok())
        {
            return (16..=31).contains(&second);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{
        ChainIdMatch, NetworkKind, RpcFailure, classify_rpc_failure,
        command_failure_detail_from_bytes, compose_rpc_url, explain_rpc_error,
        host_reachable_rpc_url, host_reachable_url, http_rpc_url, kind_for_chain_id,
        parse_cast_wallet_new,
        parse_u64_output, published_url_for_lan, resolve_network, with_lan_host,
        validate_profile_name, validate_published_configuration_url, validate_rpc_scheme,
    };

    #[test]
    fn parses_network_kind() {
        assert_eq!(NetworkKind::parse("local").unwrap(), NetworkKind::Local);
        assert_eq!(NetworkKind::parse("TESTNET").unwrap(), NetworkKind::Testnet);
        assert_eq!(NetworkKind::parse("mainnet").unwrap(), NetworkKind::Mainnet);
        assert!(NetworkKind::parse("prod").is_err());
        assert!(NetworkKind::parse("production").is_err());
    }

    #[test]
    fn classifies_known_chain_ids() {
        assert_eq!(kind_for_chain_id(31337), Some(NetworkKind::Local));
        assert_eq!(kind_for_chain_id(11155111), Some(NetworkKind::Testnet));
        assert_eq!(kind_for_chain_id(84532), Some(NetworkKind::Testnet));
        assert_eq!(kind_for_chain_id(1), Some(NetworkKind::Mainnet));
        assert_eq!(kind_for_chain_id(8453), Some(NetworkKind::Mainnet));
        assert_eq!(
            NetworkKind::Local.classify_chain_id(1),
            ChainIdMatch::Mismatch
        );
        assert_eq!(
            NetworkKind::Testnet.classify_chain_id(31337),
            ChainIdMatch::Mismatch
        );
        assert_eq!(
            NetworkKind::Testnet.classify_chain_id(8453),
            ChainIdMatch::Mismatch
        );
        assert_eq!(
            NetworkKind::Testnet.classify_chain_id(11155111),
            ChainIdMatch::Known
        );
        assert_eq!(
            NetworkKind::Testnet.classify_chain_id(999),
            ChainIdMatch::Unknown
        );
        assert_eq!(
            NetworkKind::Local.classify_chain_id(999),
            ChainIdMatch::Mismatch
        );
    }

    #[test]
    fn rejects_reserved_profile_names() {
        assert!(validate_profile_name("sepolia").is_ok());
        assert!(validate_profile_name("local").is_ok());
        assert!(validate_profile_name("development").is_err());
        assert!(validate_profile_name("production").is_err());
        assert!(validate_profile_name("Sepolia").is_err());
    }

    #[test]
    fn validates_rpc_schemes() {
        assert!(validate_rpc_scheme(NetworkKind::Local, "ws://127.0.0.1:8545").is_ok());
        assert!(validate_rpc_scheme(NetworkKind::Testnet, "wss://example.test").is_ok());
        assert!(validate_rpc_scheme(NetworkKind::Testnet, "ws://example.test").is_ok());
        assert!(validate_rpc_scheme(NetworkKind::Mainnet, "ws://example.test").is_err());
        assert!(validate_rpc_scheme(NetworkKind::Mainnet, "wss://example.test").is_ok());
    }

    #[test]
    fn rejects_docker_and_mainnet_config_urls() {
        assert!(
            validate_published_configuration_url(NetworkKind::Local, "http://127.0.0.1:8080")
                .is_ok()
        );
        assert!(
            validate_published_configuration_url(NetworkKind::Testnet, "http://10.0.0.8:8080")
                .is_ok()
        );
        assert!(
            validate_published_configuration_url(NetworkKind::Local, "http://configuration:80")
                .is_err()
        );
        assert!(
            validate_published_configuration_url(
                NetworkKind::Testnet,
                "http://10.0.0.8:8080/configuration/toml/addresses.toml"
            )
            .is_err()
        );
        assert!(
            validate_published_configuration_url(
                NetworkKind::Mainnet,
                "http://config.example.com:8080"
            )
            .is_err()
        );
        assert!(
            validate_published_configuration_url(NetworkKind::Mainnet, "https://10.0.0.8:8080")
                .is_err()
        );
        assert!(
            validate_published_configuration_url(
                NetworkKind::Mainnet,
                "https://config.example.com:8080"
            )
            .is_ok()
        );
        assert!(
            validate_published_configuration_url(
                NetworkKind::Mainnet,
                "https://config.example.com"
            )
            .is_ok()
        );
    }

    #[test]
    fn rewrites_local_loopback_rpc_for_compose() {
        assert_eq!(
            compose_rpc_url(NetworkKind::Local, "ws://127.0.0.1:8545"),
            "ws://anvil:8545"
        );
        assert_eq!(
            compose_rpc_url(NetworkKind::Local, "ws://localhost:8545"),
            "ws://anvil:8545"
        );
        assert_eq!(
            compose_rpc_url(NetworkKind::Testnet, "wss://example.test"),
            "wss://example.test"
        );
        assert_eq!(
            host_reachable_rpc_url("ws://anvil:8545"),
            "ws://127.0.0.1:8545"
        );
        assert_eq!(
            host_reachable_url("http://host.docker.internal:8080"),
            "http://127.0.0.1:8080"
        );
        assert_eq!(
            published_url_for_lan("http://host.docker.internal:8080", "192.168.1.5"),
            "http://192.168.1.5:8080"
        );
        assert_eq!(
            published_url_for_lan("http://127.0.0.1:3001", "192.168.1.5"),
            "http://192.168.1.5:3001"
        );
        assert_eq!(
            with_lan_host("http://192.168.1.5:8080", "10.173.191.159"),
            "http://10.173.191.159:8080"
        );
        assert_eq!(
            with_lan_host("https://config.example.com:8080", "10.173.191.159"),
            "https://config.example.com:8080"
        );
    }

    #[test]
    fn converts_websocket_rpc_urls_to_http() {
        assert_eq!(
            http_rpc_url("wss://eth-sepolia.example/v2/key"),
            "https://eth-sepolia.example/v2/key"
        );
        assert_eq!(http_rpc_url("ws://anvil:8545"), "http://anvil:8545");
        assert_eq!(http_rpc_url("https://rpc.example"), "https://rpc.example");
    }

    #[test]
    fn parses_numeric_cast_output_after_warnings() {
        assert_eq!(
            parse_u64_output(
                b"Warning: Found unknown `debug` config for profile `default` defined in foundry.toml.\n11155111\n"
            )
            .unwrap(),
            11155111
        );
    }

    #[test]
    fn skips_foundry_warnings_in_command_errors() {
        assert_eq!(
            command_failure_detail_from_bytes(
                b"Warning: Found unknown `debug` config for profile `default` defined in foundry.toml.\nError: Could not resolve host: anvil\n",
                b"",
            )
            .unwrap(),
            "Error: Could not resolve host: anvil"
        );
    }

    #[test]
    fn resolves_network_from_env_then_chain_id() {
        let from_env = resolve_network(Some("testnet"), Some(1));
        assert_eq!(from_env.kind, Some(NetworkKind::Testnet));
        assert_eq!(from_env.source, "local.env");

        let inferred = resolve_network(None, Some(31337));
        assert_eq!(inferred.kind, Some(NetworkKind::Local));
        assert_eq!(inferred.source, "chain ID");

        let unknown = resolve_network(None, None);
        assert_eq!(unknown.kind, None);
    }

    #[test]
    fn local_and_testnet_default_to_mock_prover() {
        assert!(NetworkKind::Local.default_mock_prover());
        assert!(NetworkKind::Testnet.default_mock_prover());
        assert!(!NetworkKind::Mainnet.default_mock_prover());
        assert!(!NetworkKind::Mainnet.allows_mock_prover());
    }

    #[test]
    fn explains_local_connection_refused() {
        let detail = "cast chain-id failed: Error: Internal transport error: IO error: Connection refused (os error 61) with ws://127.0.0.1:8545/";
        assert_eq!(
            classify_rpc_failure("ws://127.0.0.1:8545", detail),
            RpcFailure::ConnectionRefused
        );
        let message = explain_rpc_error(
            Some(NetworkKind::Local),
            "ws://127.0.0.1:8545",
            detail,
        );
        assert!(message.starts_with("Could not reach the local Anvil RPC"));
        assert!(message.contains("docker compose --profile anvil up -d"));
        assert!(message.contains("Detail: "));
        assert!(!message.starts_with("cast chain-id failed"));
    }

    #[test]
    fn parses_cast_wallet_new_output() {
        let wallet = parse_cast_wallet_new(
            "Successfully created new keypair.\nAddress:     0x1111111111111111111111111111111111111111\nPrivate key: 0x2222222222222222222222222222222222222222222222222222222222222222\n",
        )
        .unwrap();
        assert_eq!(wallet.address, "0x1111111111111111111111111111111111111111");
        assert_eq!(
            wallet.private_key,
            "0x2222222222222222222222222222222222222222222222222222222222222222"
        );
        assert!(parse_cast_wallet_new("Address: 0x1111111111111111111111111111111111111111").is_err());
    }

    #[test]
    fn explains_docker_internal_rpc_from_host() {
        let message = explain_rpc_error(
            Some(NetworkKind::Local),
            "ws://anvil:8545",
            "Could not resolve host: anvil",
        );
        assert!(message.contains("Docker-internal"));
        assert!(message.contains("ws://127.0.0.1:8545"));
    }
}
