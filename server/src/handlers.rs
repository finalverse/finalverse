// server/src/handlers.rs
use warp::{Reply, Rejection};
use std::sync::Arc;
use tokio::sync::RwLock;
use world_engine::WorldEngine;

pub async fn handle_world_update(
    world_engine: Arc<RwLock<WorldEngine>>,
) -> Result<impl Reply, Rejection> {
    let engine = world_engine.read().await;
    Ok(warp::reply::json(&serde_json::json!({
        "status": "world_updated"
    })))
}