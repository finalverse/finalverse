use axum::{routing::get, Router, Json};
use config_core::{load_default_config, GrpcServiceRegistry};
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = load_default_config()?;
    let registry = Arc::new(config.grpc_services);
    let app = Router::new().route(
        "/services/grpc",
        get({
            let registry = registry.clone();
            move || async move { Json(registry.services.clone()) }
        }),
    );
    let addr: SocketAddr = std::env::var("FINALVERSE_CONFIG_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:7070".to_string())
        .parse()?;
    println!("config-core listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
