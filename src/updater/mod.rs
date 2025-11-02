use std::{path::Path, time::Duration};

use anyhow::Result;
use axum::routing::trace;
use log::{debug, trace, warn};
use mc_server_status::{McClient, ServerEdition};
use crate::api::{SharedAppState, appstate::{PlayersInfo, ServerState, ServerStatus}};

pub fn run_as_task(app_state: &SharedAppState) {
    let app_state = app_state.clone();
    debug!("Spawning updater task...");
    tokio::task::spawn(async move {
        run(app_state).await
    });
}

pub async fn run(app_state: SharedAppState) -> Result<()> {
    let polling_duration = Duration::from_secs(app_state.config.polling_interval_seconds);
    debug!("Starting updater task with polling interval {polling_duration:?}...");

    let client = McClient::new()
        .with_timeout(Duration::from_millis(app_state.config.query_timeout_milliseconds))
        .with_max_parallel(10);
    
    let favicon_path = app_state.config.favicon_save_path.as_ref().map(|s| Path::new(s));

    loop {
        trace!("Updater task iteration started for {} servers...", app_state.servers.len());
        for server_state in app_state.servers.values() {
            if let Err(e) = update_server_status(server_state, &client, favicon_path).await {
                warn!("Failed to update status for server {}: {}", server_state.config.name, e);
            }
        }

        trace!("Updater task iteration completed. Sleeping for {polling_duration:?}...");
        tokio::time::sleep(polling_duration).await;
    }
}

async fn update_server_status(server_state: &ServerStatus, client: &McClient, favicon_save_path: Option<&Path>) -> Result<()> {
    trace!("Updating status for server {}...", server_state.config.name);
    let status = client.ping(&server_state.address, ServerEdition::Java).await;

    if let Err(err) = status {
        trace!("Error pinging server {}: {}", server_state.config.name, err);
        *server_state.state.write().unwrap() = ServerState::Unreachable;
    } else if let Ok(server_info) = status {
        trace!("Retrieving status for server {} was successful!", server_state.config.name);
        let new_state = if server_info.online {
            match &server_info.data {
                mc_server_status::ServerData::Java(java_status) => {
                    ServerState::Online(PlayersInfo {
                        online: java_status.players.online as u32,
                        max: java_status.players.max as u32,
                    })
                },
                mc_server_status::ServerData::Bedrock(_bedrock_status) => todo!("Bedrock status handling not implemented"),
            }
        } else {
            ServerState::Offline
        };

        if *server_state.state.read().unwrap() != new_state {
            trace!("Server {} state changed: {:?} -> {:?}", server_state.config.name, *server_state.state.read().unwrap(), new_state);
            *server_state.state.write().unwrap() = new_state;
        }

        if server_state.favicon_path.read().unwrap().is_none() {
            if let Some(save_path) = favicon_save_path {
                let favicon_path = format!("{}/{}", save_path.display(), server_state.config.id);
                trace!("Saving favicon for server {} to file {}...", server_state.config.name, favicon_path);
                match &server_info.data {
                    mc_server_status::ServerData::Java(java_status) => java_status.save_favicon(&favicon_path)?,
                    mc_server_status::ServerData::Bedrock(_bedrock_status) => todo!("Bedrock favicon handling not implemented"),
                }
                *server_state.favicon_path.write().unwrap() = Some(favicon_path);
            }
        }
    }
    debug!("Updated status for server {}", server_state.config.name);

    Ok(())
}