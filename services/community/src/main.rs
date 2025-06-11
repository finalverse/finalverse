use axum::Router;
use health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = Arc::new(HealthMonitor::new("community", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("community".to_string(), "http://localhost:3008".to_string())
        .await;

    let app = Router::new().merge(monitor.clone().axum_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3008));
    println!("Community Service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
