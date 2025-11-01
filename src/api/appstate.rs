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
    pub config: ServerConfig,
}

#[derive(Debug, Serialize)]
pub enum ServerState {
    Online(PlayersInfo),
    Offline(PlayersInfo),
    Unreachable
}

#[derive(Debug, Serialize)]
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
                config: server.clone()
            })
        }).collect::<ServerStatusList>();
        Self {
            config,
            servers,
        }
    }
}