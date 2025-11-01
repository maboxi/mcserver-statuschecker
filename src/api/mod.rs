use std::sync::Arc;

use anyhow::Result;
use axum::{Router, http::{Method, header::CONTENT_TYPE}, routing::get};
use log::debug;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::{api::appstate::AppState, config};

pub mod appstate;
pub mod service;

pub async fn start_service(config: config::Config) -> Result<()> {
    let app_state = Arc::new(AppState::from(config.clone()));

    debug!("App state deduced from config: \n{:#?}", app_state);

    let router = Router::new()
        .route("/api/servers", get(service::get_servers))
        .route("/api/servers/{id}/status", get(service::get_server_status))
        .route("/api/servers/{id}/code", get(service::get_server_status_returncode))
        .with_state(app_state.clone())
        .layer(CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_private_network(true)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE])
        );

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    axum::serve(listener, router).await?;
    Ok(())
}