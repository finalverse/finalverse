use axum::{routing::get, Router};
use finalverse_health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = Arc::new(HealthMonitor::new("api-gateway", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("api-gateway".to_string(), "http://localhost:8080".to_string())
        .await;

    let app = Router::new().merge(monitor.clone().axum_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("API Gateway listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
