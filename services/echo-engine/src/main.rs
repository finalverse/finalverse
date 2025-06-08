// services/echo-engine/src/main.rs

use axum::{
    extract::{Path, State},
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

#[derive(Debug, Clone)]
struct Echo {
    id: EchoId,
    echo_type: EchoType,
    personality: String,
    current_location: Coordinates,
    player_bonds: std::collections::HashMap<PlayerId, u32>, // Bond level 0-100
}

impl Echo {
    fn new(echo_type: EchoType) -> Self {
        let (personality, location) = match &echo_type {
            EchoType::Lumi => (
                "Curious and hopeful, always seeking new discoveries".to_string(),
                Coordinates { x: 100.0, y: 50.0, z: 200.0 }
            ),
            EchoType::KAI => (
                "Logical and wise, understanding the patterns of the universe".to_string(),
                Coordinates { x: -200.0, y: 100.0, z: 0.0 }
            ),
            EchoType::Terra => (
                "Patient and nurturing, connected to all living things".to_string(),
                Coordinates { x: 0.0, y: 0.0, z: -150.0 }
            ),
            EchoType::Ignis => (
                "Passionate and brave, inspiring courage in others".to_string(),
                Coordinates { x: 300.0, y: 200.0, z: 100.0 }
            ),
        };
        
        Self {
            id: echo_type.id(),
            echo_type,
            personality,
            current_location: location,
            player_bonds: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct EchoEngineState {
    echoes: Arc<RwLock<std::collections::HashMap<EchoId, Echo>>>,
    interactions: Arc<RwLock<u64>>,
    start_time: std::time::Instant,
}

impl EchoEngineState {
    fn new() -> Self {
        let mut echoes = std::collections::HashMap::new();
        
        // Initialize all four Echoes
        for echo_type in [EchoType::Lumi, EchoType::KAI, EchoType::Terra, EchoType::Ignis] {
            let echo = Echo::new(echo_type);
            echoes.insert(echo.id.clone(), echo);
        }
        
        Self {
            echoes: Arc::new(RwLock::new(echoes)),
            interactions: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
        }
    }
    
    async fn interact_with_echo(&self, player_id: PlayerId, echo_id: EchoId) -> Result<(u32, String), FinalverseError> {
        let mut echoes = self.echoes.write().await;
        let echo = echoes.get_mut(&echo_id)
            .ok_or_else(|| FinalverseError::InvalidRequest("Echo not found".to_string()))?;
        
        // Increase bond level
        let bond_level = echo.player_bonds.entry(player_id).or_insert(0);
        *bond_level = (*bond_level + 5).min(100);
        
        let mut interactions = self.interactions.write().await;
        *interactions += 1;
        
        // Generate response based on Echo personality and bond level
        let response = match (&echo.echo_type, *bond_level) {
            (EchoType::Lumi, level) if level < 20 => "Lumi glows softly, curious about you.".to_string(),
            (EchoType::Lumi, level) if level < 50 => "Lumi's light brightens! She seems happy to see you.".to_string(),
            (EchoType::Lumi, _) => "Lumi shines brilliantly, radiating hope and joy at your presence!".to_string(),
            
            (EchoType::KAI, level) if level < 20 => "KAI observes you with analytical interest.".to_string(),
            (EchoType::KAI, level) if level < 50 => "KAI nods in acknowledgment, processing your growth.".to_string(),
            (EchoType::KAI, _) => "KAI's form pulses with understanding. 'Your progress is remarkable.'".to_string(),
            
            (EchoType::Terra, level) if level < 20 => "Terra watches you calmly, roots gently stirring.".to_string(),
            (EchoType::Terra, level) if level < 50 => "Terra's presence feels warm and protective.".to_string(),
            (EchoType::Terra, _) => "Terra embraces you with nature's strength. You feel deeply connected.".to_string(),
            
            (EchoType::Ignis, level) if level < 20 => "Ignis burns with intensity, assessing your courage.".to_string(),
            (EchoType::Ignis, level) if level < 50 => "Ignis's flames dance with approval!".to_string(),
            (EchoType::Ignis, _) => "Ignis roars with pride! 'Together, we are unstoppable!'".to_string(),
        };
        
        Ok((*bond_level, response))
    }
}

// API handlers
async fn get_service_info(State(state): State<EchoEngineState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "echo-engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn get_all_echoes(State(state): State<EchoEngineState>) -> Json<serde_json::Value> {
    let echoes = state.echoes.read().await;
    let data: Vec<_> = echoes
        .values()
        .map(|echo| {
            serde_json::json!({
                "id": echo.id.0,
                "type": format!("{:?}", echo.echo_type),
                "personality": echo.personality,
                "location": echo.current_location,
                "total_bonds": echo.player_bonds.len(),
            })
        })
        .collect();
    
    Json(serde_json::json!({
        "echoes": data,
        "total_interactions": *state.interactions.read().await,
    }))
}

async fn interact_with_echo(
    State(state): State<EchoEngineState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_id = request.get("player_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let echo_id = request.get("echo_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let player_uuid = uuid::Uuid::parse_str(player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    let echo_id = EchoId(echo_id.to_string());
    
    match state.interact_with_echo(player_id, echo_id).await {
        Ok((bond_level, response)) => Ok(Json(serde_json::json!({
            "success": true,
            "bond_level": bond_level,
            "response": response,
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_player_bonds(
    State(state): State<EchoEngineState>,
    Path(player_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_uuid = uuid::Uuid::parse_str(&player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let echoes = state.echoes.read().await;
    let bonds: Vec<_> = echoes
        .values()
        .filter_map(|echo| {
            echo.player_bonds.get(&player_id).map(|level| {
                serde_json::json!({
                    "echo_id": echo.id.0,
                    "echo_type": format!("{:?}", echo.echo_type),
                    "bond_level": level,
                })
            })
        })
        .collect();
    
    Ok(Json(serde_json::json!({
        "player_id": player_id.0.to_string(),
        "bonds": bonds,
    })))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting Echo Engine Service...");
    
    let state = EchoEngineState::new();
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/echoes", get(get_all_echoes))
        .route("/interact", post(interact_with_echo))
        .route("/bonds/:player_id", get(get_player_bonds))
        .with_state(state);
    
    let addr = "0.0.0.0:3003";
    info!("Echo Engine listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}