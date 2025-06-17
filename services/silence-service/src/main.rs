use axum::Router;
use finalverse_health::HealthMonitor;
use service_registry::LocalServiceRegistry;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use finalverse_logging as logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init(None);
    let monitor = Arc::new(HealthMonitor::new("silence-service", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("silence-service".to_string(), "http://localhost:3009".to_string())
        .await;

    let app = Router::new().merge(monitor.clone().axum_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3009));
    info!("Silence Service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
