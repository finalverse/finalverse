// server/src/main.rs
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;
use world_engine::{WorldEngine, WorldState};

mod handlers;
mod server_manager;

use crate::server_manager::ServerManager;

#[tokio::main]
async fn main() {
    println!("Starting Finalverse Server...");

    // Initialize world engine
    let world_engine = Arc::new(WorldEngine::new());

    // Initialize server manager
    let mut server_manager = ServerManager::new();

    // Start services
    server_manager.start_services().await;

    // Clone for the update task
    let world_engine_clone = world_engine.clone();

    // Start world update loop
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            world_engine_clone.update(0.1).await; // 100ms = 0.1s
        }
    });

    // Set up routes
    let health = warp::path("health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let world_state = {
        let engine = world_engine.clone();
        warp::path("world")
            .and(warp::path("state"))
            .and_then(move || {
                let engine = engine.clone();
                async move {
                    let state = engine.get_state().await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&state))
                }
            })
    };

    let routes = health.or(world_state);

    // Start server
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("Server listening on {}", addr);

    warp::serve(routes)
        .run(addr)
        .await;
}