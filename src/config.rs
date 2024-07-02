use std::io::Read;
use serde::Deserialize;
use fs_err as fs;
use crate::errors::HttpErrors;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub extra: ExtraConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: String,
    pub root: String,
    pub threading: ServerThreadingConfig,
    pub async_: ServerAsyncConfig,
}

#[derive(Deserialize)]
pub struct ServerThreadingConfig {
    pub enable: bool,
    pub max_threads: u32,
}

#[derive(Deserialize)]
pub struct ServerAsyncConfig {
    pub enable: bool,
    pub max_tasks: u32,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub default_level: String,
    pub log_file_level: String,
    pub log_file: String,
}

#[derive(Deserialize)]
pub struct ExtraConfig {
    pub panic_if_not_impl: bool,
}

pub fn parse() -> Result<Config, HttpErrors>  {
    let mut config_file = fs::File::open(format!("./{CONFIG_FILE_NAME}")).map_err(|e| HttpErrors::ConfigReadFailure(e.to_string()))?;
    let mut config: String = String::new();
    config_file.read_to_string(&mut config).map_err(|e| HttpErrors::ConfigReadFailure(e.to_string()))?;

    let toml = toml::from_str::<Config>(&config).map_err(|e| HttpErrors::ConfigParseFailure(e.to_string()))?;

    Ok(toml)
}
