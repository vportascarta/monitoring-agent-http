
//! Main entry point for the monitoring agent.
//!
//! Wires up the HTTP API and launches the server.

mod config;
mod metrics;
mod checks;
mod handlers;

use axum::{routing::get, Router};
use std::sync::Arc;
use config::Config;
use handlers::*;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::load());
    let port = config.app_port.unwrap_or(8081);
    let app = Router::new()
        .route("/", get(root_handler))
        .route(
            "/metrics",
            get({
                let config = config.clone();
                move || metrics_handler(config.clone())
            }),
        )
        .route("/cpu", get(cpu_handler))
        .route("/memory", get(memory_handler))
        .route("/load", get(load_handler))
        .route("/disk", get(disk_handler))
        .route(
            "/service/{name}",
            get({
                let config = config.clone();
                move |path| service_handler(path, config.clone())
            }),
        )
        .route(
            "/port/{name}",
            get({
                let config = config.clone();
                move |path| port_handler(path, config.clone())
            }),
        );
    println!("Serving metrics on http://0.0.0.0:{}/metrics", port);
    axum::serve(
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
