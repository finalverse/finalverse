use axum::{routing::get, extract::ws::WebSocketUpgrade, Router};
use axum::extract::ws::WebSocket;
use finalverse_realtime_gateway::{discover_plugins, LoadedPlugin};
use std::path::PathBuf;
use tracing::info;

async fn handle_socket(mut plugin: Box<dyn finalverse_realtime_gateway::WebSocketPlugin>, ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| async move {
        plugin.handle(socket).await;
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let dir = PathBuf::from("./realtime-plugins");
    let mut plugins = discover_plugins(&dir).await;
    let mut app = Router::new();
    for p in &mut plugins {
        let mut instance = p.take();
        let path = instance.register_ws_path();
        app = app.route(path, get(move |ws: WebSocketUpgrade| handle_socket(instance, ws)));
    }
    let addr = "0.0.0.0:8081";
    info!("Realtime gateway listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
