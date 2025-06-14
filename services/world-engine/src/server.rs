// services/world-engine/src/server.rs
use crate::{WorldEngine, RegionId, PlayerAction};
use std::sync::Arc;
use warp::Filter;

pub async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&serde_json::json!({"status": "healthy"})))
}

pub async fn region_handler(
    id: String,
    engine: Arc<WorldEngine>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(region) = engine.metabolism().get_region(&RegionId(id)).await {
        Ok(warp::reply::json(&region))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "Region not found"})))
    }
}

pub async fn action_handler(
    action: PlayerAction,
    engine: Arc<WorldEngine>,
) -> Result<impl warp::Reply, warp::Rejection> {
    engine.process_action(action).await;
    Ok(warp::reply::json(&serde_json::json!({"success": true})))
}

pub fn create_routes(
    engine: Arc<WorldEngine>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health = warp::path!("health")
        .and(warp::get())
        .and_then(health_handler);

    let engine_get = engine.clone();
    let get_region = warp::path!("region" / String)
        .and(warp::get())
        .and(warp::any().map(move || engine_get.clone()))
        .and_then(region_handler);

    let engine_post = engine.clone();
    let post_action = warp::path!("action")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_post.clone()))
        .and_then(action_handler);

    health.or(get_region).or(post_action)
}