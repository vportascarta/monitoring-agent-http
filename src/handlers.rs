//! HTTP handlers for monitoring agent endpoints.

use axum::{extract::Path, response::IntoResponse};
use std::sync::Arc;

use crate::config::Config;
use crate::metrics::get_metrics;
use crate::checks::{check_service, check_http_port};

/// Handler for root endpoint.
pub async fn root_handler() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "monitoring agent is running\n")
}

/// Handler for /metrics endpoint (all metrics, services, ports).
pub async fn metrics_handler(config: Arc<Config>) -> impl IntoResponse {
    let metrics = get_metrics();
    let mut status_ok = true;
    let mut output = String::new();
    output += &format!("cpu usage: {:.2}%\n", metrics.cpu);
    output += &format!("cpu usage 1min: {:.2}%\n", metrics.load1);
    output += &format!("cpu_usage 5min: {:.2}%\n", metrics.load5);
    output += &format!("cpu usage 15min: {:.2}%\n", metrics.load15);
    output += &format!("memory usage: {:.2}%\n", metrics.mem_percent);
    for (name, percent) in metrics.disk_stats {
        output += &format!("disk {} usage: {:.2}%\n", name, percent);
    }
    for svc in &config.services {
        let ok = check_service(svc);
        output += &format!(
            "service {}: {}\n",
            svc,
            if ok { "active" } else { "inactive" }
        );
        if !ok {
            status_ok = false;
        }
    }
    for (name, port) in &config.http_ports {
        let ok = check_http_port(*port);
        output += &format!(
            "http port {} ({}): {}\n",
            port,
            name,
            if ok { "up" } else { "down" }
        );
        if !ok {
            status_ok = false;
        }
    }
    let status = if status_ok { 200 } else { 500 };
    (axum::http::StatusCode::from_u16(status).unwrap(), output)
}

/// Handler for /service/{name} endpoint.
pub async fn service_handler(Path(name): Path<String>, config: Arc<Config>) -> impl IntoResponse {
    if !config.services.contains(&name) {
        return (
            axum::http::StatusCode::NOT_FOUND,
            format!("service {} not in config", name),
        );
    }
    let ok = check_service(&name);
    let status = if ok { 200 } else { 500 };
    let output = format!(
        "service {}: {}\n",
        name,
        if ok { "active" } else { "inactive" }
    );
    (axum::http::StatusCode::from_u16(status).unwrap(), output)
}

/// Handler for /port/{name} endpoint.
pub async fn port_handler(Path(name): Path<String>, config: Arc<Config>) -> impl IntoResponse {
    let port = config.http_ports.get(&name);
    if port.is_none() {
        return (
            axum::http::StatusCode::NOT_FOUND,
            format!("port name '{}' not in config", name),
        );
    }
    let port = *port.unwrap();
    let ok = check_http_port(port);
    let status = if ok { 200 } else { 500 };
    let output = format!(
        "http_port {} ({}): {}\n",
        port,
        name,
        if ok { "up" } else { "down" }
    );
    (axum::http::StatusCode::from_u16(status).unwrap(), output)
}

/// Handler for /cpu endpoint.
pub async fn cpu_handler() -> impl IntoResponse {
    let metrics = get_metrics();
    (axum::http::StatusCode::OK, format!("{:.2}\n", metrics.cpu))
}

/// Handler for /memory endpoint.
pub async fn memory_handler() -> impl IntoResponse {
    let metrics = get_metrics();
    (axum::http::StatusCode::OK, format!("{:.2}\n", metrics.mem_percent))
}

/// Handler for /load endpoint (returns 15min load as before).
pub async fn load_handler() -> impl IntoResponse {
    let metrics = get_metrics();
    (axum::http::StatusCode::OK, format!("{:.2}\n", metrics.load15))
}

/// Handler for /disk endpoint (returns all disk stats as lines).
pub async fn disk_handler() -> impl IntoResponse {
    let metrics = get_metrics();
    let mut output = String::new();
    for (name, percent) in metrics.disk_stats {
        output += &format!("{}: {:.2}%\n", name, percent);
    }
    (axum::http::StatusCode::OK, output)
}
