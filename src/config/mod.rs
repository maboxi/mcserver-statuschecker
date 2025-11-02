use anyhow::Result;
use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

const DEFAULT_SERVER_PORT: u16 = 25565;
const DEFAULT_API_PORT: u16 = 9235;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub servers: Vec<ServerConfig>,
    #[serde(default = "defaults::default_api_port")]
    pub port: u16,
    #[serde(default = "defaults::default_polling_interval_seconds")]
    pub polling_interval_seconds: u64,
    #[serde(default = "defaults::default_query_timeout_milliseconds")]
    pub query_timeout_milliseconds: u64,
    pub favicon_save_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub id: String,

    pub host: String,
    #[serde(default = "defaults::default_server_port")]
    pub port: u16,
}

pub fn load_config(path: &Path) -> Result<Config> {
    let filehandle = File::open(path)?;
    let appstate = serde_json::from_reader(filehandle)?;
    Ok(appstate)
}

mod defaults {
    use crate::config::DEFAULT_SERVER_PORT;

    pub fn default_server_port() -> u16 {
        DEFAULT_SERVER_PORT
    }

    pub fn default_polling_interval_seconds() -> u64 {
        60
    }

    pub fn default_query_timeout_milliseconds() -> u64 {
        100
    }

    pub fn default_api_port() -> u16 {
        super::DEFAULT_API_PORT
    }
}