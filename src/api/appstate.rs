use std::{collections::HashMap, sync::RwLock};

use serde::Serialize;

use crate::config::{Config, ServerConfig};

#[derive(Debug)]
pub struct AppState {
    pub config: Config,
    pub servers: ServerStatusList,
}

pub type ServerStatusList = HashMap<String, ServerStatus>;

#[derive(Debug)]
pub struct ServerStatus {
    pub state: RwLock<ServerState>,

    pub favicon_path: RwLock<Option<String>>,

    pub config: ServerConfig,
    pub address: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ServerState {
    Online(PlayersInfo),
    Offline,
    Unreachable
}

#[derive(Debug, Serialize, PartialEq)]
pub struct PlayersInfo {
    pub online: u32,
    pub max: u32,
}

impl Default for ServerState {
    fn default() -> Self {
        ServerState::Unreachable
    }
}

impl From<Config> for AppState {
    fn from(config: Config) -> Self {
        let servers = config.servers.iter().map(|server| {
            (server.id.clone(), ServerStatus {
                state: RwLock::new(ServerState::default()),
                favicon_path: RwLock::new(None),
                config: server.clone(),
                address: format!("{}:{}", server.host, server.port),
            })
        }).collect::<ServerStatusList>();
        Self {
            config,
            servers,
        }
    }
}