use axum::Router;
use finalverse_health::HealthMonitor;
use service_registry::LocalServiceRegistry;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = Arc::new(HealthMonitor::new("behavior-ai", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("behavior-ai".to_string(), "http://localhost:3011".to_string())
        .await;

    let app = Router::new().merge(monitor.clone().axum_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3011));
    println!("Behavior AI listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
