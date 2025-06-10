use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
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

    let app = Router::new()
        .merge(monitor.clone().axum_routes())
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("API Gateway listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    let token = format!("token-{}", payload.username);
    Json(LoginResponse { token })
}
