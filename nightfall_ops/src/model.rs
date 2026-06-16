#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    pub profile: String,
    pub rpc_url: String,
    pub chain_id: u64,
    pub genesis_block: u64,
    pub configuration_url: String,
    pub configuration_port: u16,
    pub deployer_signing_key: String,
    pub deployer_address: String,
    pub default_proposer_address: String,
    pub default_proposer_url: String,
    pub mock_prover: bool,
    pub block_size: u64,
}

impl DeploymentConfig {
    pub fn prover_mode(&self) -> &'static str {
        if self.mock_prover { "mock" } else { "real" }
    }
}
