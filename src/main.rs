use std::{collections::HashMap, fs::File, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use axum::Router;
use anyhow::Result;
use env_logger::Env;
use log::{debug, trace};
use serde::Deserialize;


fn main() -> Result<()> {
    init_logging();

    let args = parse_args()?;

    let config = load_config(&args.config_path)?;

    debug!("Loaded configuration: \n{:#?}", config);

    Ok(())
}

struct Args {
    config_path: PathBuf,
}

fn parse_args() -> Result<Args> {
    const USAGE: &str = "Usage: my_app <config_path>";
    let mut args = std::env::args().collect::<Vec<String>>();

    trace!("Program arguments: \n{:#?}", args);

    if args.len() < 2 {
        println!("{}", USAGE);
        anyhow::bail!("Not enough arguments provided");
    } else if args.len() > 2 {
        println!("{}", USAGE);
        anyhow::bail!("Too many arguments provided");
    }

    Ok(Args {
        config_path: args.pop().unwrap().into(),
    })
}

#[derive(Deserialize, Debug)]
struct Server {
    name: String,
    id: String,

    host: String,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    25565
}

#[derive(Deserialize, Debug)]
struct Config {
    servers: Vec<Server>,
}

fn load_config(path: &Path) -> Result<Config> {
    let filehandle = File::open(path)?;
    let config = serde_json::from_reader(filehandle)?;
    Ok(config)
}

fn init_logging() {
    env_logger::init();
}

struct AppState {
    config: Arc<Config>,
    servers: Mutex<ServerStatusList>,
}

type ServerStatusList = HashMap<String, ServerStatus>;

enum ServerState {
    Online,
    Offline,
    Unreachable
}

impl Default for ServerState {
    fn default() -> Self {
        ServerState::Unreachable
    }
}

struct ServerStatus {
    state: ServerState,
    players_online: u32,
    players_max: u32,
}