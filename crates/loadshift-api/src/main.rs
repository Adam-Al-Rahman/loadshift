#![forbid(unsafe_code)]

use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "healthy" })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!(
        address = %listener.local_addr()?,
        "Loadshift API listening"
    );

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::health;

    #[tokio::test]
    async fn health_handler_returns_healthy_status() {
        let response = health().await;

        assert_eq!(response.0.status, "healthy");
    }
}
