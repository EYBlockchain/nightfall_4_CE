use std::path::Path;

pub const NIGHTFALL_TOML: &str = "nightfall.toml";
pub const DOCKER_COMPOSE_YML: &str = "docker-compose.yml";

pub fn required_repo_files_exist() -> bool {
    [NIGHTFALL_TOML, DOCKER_COMPOSE_YML]
        .iter()
        .all(|path| Path::new(path).is_file())
}
