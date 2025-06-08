// services/song-engine/src/main.rs

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use finalverse_common::*;
use finalverse_protocol::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Clone)]
struct SongEngineState {
    harmonies: Arc<RwLock<std::collections::HashMap<RegionId, f32>>>,
    melodies_performed: Arc<RwLock<u64>>,
    start_time: std::time::Instant,
}

impl SongEngineState {
    fn new() -> Self {
        Self {
            harmonies: Arc::new(RwLock::new(std::collections::HashMap::new())),
            melodies_performed: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
        }
    }
    
    async fn process_melody(&self, player_id: PlayerId, melody: Melody, region: RegionId) -> f32 {
        let harmony_change = match melody {
            Melody::Healing { power } => power * 1.2,
            Melody::Creation { .. } => 2.0,
            Melody::Discovery { range } => range * 0.8,
            Melody::Courage { intensity } => intensity * 1.5,
        };
        
        let mut harmonies = self.harmonies.write().await;
        let current = harmonies.entry(region.clone()).or_insert(50.0);
        *current = (*current + harmony_change).clamp(0.0, 100.0);
        
        let mut count = self.melodies_performed.write().await;
        *count += 1;
        
        info!("Melody performed by {:?}: harmony change {}", player_id, harmony_change);
        
        harmony_change
    }
}

// API handlers
async fn get_service_info(State(state): State<SongEngineState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "song-engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn perform_melody(
    State(state): State<SongEngineState>,
    Json(request): Json<grpc::PerformMelodyRequest>,
) -> Result<Json<grpc::PerformMelodyResponse>, StatusCode> {
    let player_id = PlayerId(
        uuid::Uuid::parse_str(&request.player_id)
            .map_err(|_| StatusCode::BAD_REQUEST)?
    );
    
    // For MVP, assume region based on coordinates
    let region = RegionId(uuid::Uuid::new_v4());
    
    let harmony_change = state.process_melody(player_id, request.melody, region).await;
    
    Ok(Json(grpc::PerformMelodyResponse {
        success: true,
        harmony_change,
        resonance_gained: Resonance {
            creative: 10,
            exploration: 5,
            restoration: 15,
        },
    }))
}

async fn get_harmonies(State(state): State<SongEngineState>) -> Json<serde_json::Value> {
    let harmonies = state.harmonies.read().await;
    let data: Vec<_> = harmonies
        .iter()
        .map(|(region, harmony)| {
            serde_json::json!({
                "region": region.0.to_string(),
                "harmony": harmony,
            })
        })
        .collect();
    
    Json(serde_json::json!({
        "harmonies": data,
        "total_melodies": *state.melodies_performed.read().await,
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Song Engine Service...");
    
    let state = SongEngineState::new();
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/melody", post(perform_melody))
        .route("/harmonies", get(get_harmonies))
        .with_state(state);
    
    let addr = "0.0.0.0:3001";
    info!("Song Engine listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}