use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse};

use crate::api::appstate::{AppState, ServerState};

pub async fn get_servers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    axum::Json(state.config.servers.clone())
}

pub async fn get_server_status(State(state): State<Arc<AppState>>, Path(server_id): Path<String>) -> impl IntoResponse {
    if let Some(server_info) = state.servers.get(&server_id) {
        (axum::http::StatusCode::OK, server_state_to_json(&*server_info.state.read().unwrap()))
    } else {
        (axum::http::StatusCode::NOT_FOUND, format!("{{\"error\": \"Server {} not found!\"}}", server_id))
    }
}

pub fn server_state_to_json(state: &ServerState) -> String {
    #[derive(serde::Serialize)]
    struct ServerStateHelper <'a> {
        state: &'a ServerState,
    }
    serde_json::to_string(&ServerStateHelper { state }).unwrap()
}

pub async fn get_server_status_returncode(State(state): State<Arc<AppState>>, Path(server_id): Path<String>) -> impl IntoResponse {
    if let Some(server_info) = state.servers.get(&server_id) {
        let status = match &*server_info.state.read().unwrap() {
            ServerState::Online(_) => axum::http::StatusCode::OK,
            ServerState::Offline => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            ServerState::Unreachable => axum::http::StatusCode::BAD_GATEWAY,
        };
        (status, "".into())
    } else {
        (axum::http::StatusCode::NOT_FOUND, format!("{{\"error\": \"Server {} not found!\"}}", server_id))
    }
}