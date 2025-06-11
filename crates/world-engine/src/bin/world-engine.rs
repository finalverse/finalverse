//! Binary for running the World Engine service.

use crate::{ecosystem, ActionType, MetabolismSimulator, Observer, PlayerAction, RegionState};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use fv_common::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use health::HealthMonitor;
use service_registry::LocalServiceRegistry;

#[derive(Debug, Clone)]
struct Region {
    id: RegionId,
    name: String,
    harmony_level: f32,
    weather: Weather,
    active_players: Vec<PlayerId>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Weather {
    Clear,
    Rain,
    Storm,
    DissonanceStorm,
}

#[derive(Clone)]
struct WorldEngineState {
    regions: Arc<RwLock<std::collections::HashMap<RegionId, Region>>>,
    cosmic_time: Arc<RwLock<u64>>, // In-game time
    start_time: std::time::Instant,
    ecosystems: Arc<RwLock<std::collections::HashMap<RegionId, ecosystem::Ecosystem>>>,
    observer: Arc<RwLock<Observer>>, 
}

impl WorldEngineState {
    fn new() -> Self {
        let mut initial_regions = std::collections::HashMap::new();
        
        // Create some initial regions for MVP
        let terra_nova = Region {
            id: RegionId(uuid::Uuid::new_v4()),
            name: "Terra Nova".to_string(),
            harmony_level: 75.0,
            weather: Weather::Clear,
            active_players: vec![],
        };
        
        let whispering_wilds = Region {
            id: RegionId(uuid::Uuid::new_v4()),
            name: "Whispering Wilds".to_string(),
            harmony_level: 60.0,
            weather: Weather::Rain,
            active_players: vec![],
        };
        
        initial_regions.insert(terra_nova.id.clone(), terra_nova);
        initial_regions.insert(whispering_wilds.id.clone(), whispering_wilds);

        let mut observer = Observer::new(MetabolismSimulator::new(300));

        {
            let meta = &mut observer.metabolism;
            for region in initial_regions.values() {
                meta.world_map.insert(
                    region.name.clone(),
                    RegionState {
                        harmony: region.harmony_level,
                        dissonance: 0.0,
                        resources: 50.0,
                        political_tension: 0.0,
                    },
                );
            }
        }

        Self {
            regions: Arc::new(RwLock::new(initial_regions)),
            cosmic_time: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
            ecosystems: Arc::new(Default::default()),
            observer: Arc::new(RwLock::new(observer)),
        }
    }
    
    async fn update_cosmic_metabolism(&self) {
        let mut time = self.cosmic_time.write().await;
        *time += 1;

        // Advance regional metabolism
        {
            let mut obs = self.observer.write().await;
            obs.metabolism.tick();
        }
        
        // Update weather based on harmony levels
        let mut regions = self.regions.write().await;
        for (_, region) in regions.iter_mut() {
            if region.harmony_level < 30.0 && !matches!(region.weather, Weather::DissonanceStorm) {
                region.weather = Weather::DissonanceStorm;
                info!("Dissonance Storm forming in {}", region.name);
            } else if region.harmony_level > 70.0 && matches!(region.weather, Weather::DissonanceStorm) {
                region.weather = Weather::Clear;
                info!("Weather clearing in {}", region.name);
            }
        }
    }
}

// API handlers
async fn get_service_info(State(state): State<WorldEngineState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "world-engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn get_all_regions(State(state): State<WorldEngineState>) -> Json<serde_json::Value> {
    let regions = state.regions.read().await;
    let data: Vec<_> = regions
        .values()
        .map(|region| {
            serde_json::json!({
                "id": region.id.0.to_string(),
                "name": region.name,
                "harmony_level": region.harmony_level,
                "weather": region.weather,
                "active_players": region.active_players.len(),
            })
        })
        .collect();
    
    Json(serde_json::json!({
        "regions": data,
        "cosmic_time": *state.cosmic_time.read().await,
    }))
}

async fn get_region(
    State(state): State<WorldEngineState>,
    Path(region_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let region_uuid = uuid::Uuid::parse_str(&region_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let region_id = RegionId(region_uuid);
    
    let regions = state.regions.read().await;
    let region = regions.get(&region_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(serde_json::json!({
        "id": region.id.0.to_string(),
        "name": region.name,
        "harmony_level": region.harmony_level,
        "weather": region.weather,
        "active_players": region.active_players.iter()
            .map(|p| p.0.to_string())
            .collect::<Vec<_>>(),
    })))
}

async fn update_harmony(
    State(state): State<WorldEngineState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    let region_id = request.get("region_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let harmony_change = request.get("harmony_change")
        .and_then(|v| v.as_f64())
        .ok_or(StatusCode::BAD_REQUEST)? as f32;
    
    let region_uuid = uuid::Uuid::parse_str(region_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let region_id = RegionId(region_uuid);
    
    let mut regions = state.regions.write().await;
    let region = regions.get_mut(&region_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    region.harmony_level = (region.harmony_level + harmony_change).clamp(0.0, 100.0);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "new_harmony": region.harmony_level,
    })))
}

#[derive(serde::Deserialize)]
struct PlayerActionRequest {
    player_id: String,
    action_type: String,
    region: String,
}

async fn post_player_action(
    State(state): State<WorldEngineState>,
    Json(req): Json<PlayerActionRequest>,
) -> Result<Json<serde_json::Value>> {
    let action_type = match req.action_type.as_str() {
        "CompleteQuest" => ActionType::CompleteQuest,
        "BuildStructure" => ActionType::BuildStructure,
        "Ritual" => ActionType::Ritual,
        "PvPConflict" => ActionType::PvPConflict,
        _ => return Err(StatusCode::BAD_REQUEST.into()),
    };

    let action = PlayerAction {
        player_id: req.player_id,
        action_type,
        region: req.region,
    };
    state.observer.write().await.interpret_action(action);
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn get_metabolism_state(
    State(state): State<WorldEngineState>,
    Path(region): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let obs = state.observer.read().await;
    if let Some(rs) = obs.metabolism.get_state(&region) {
        Ok(Json(serde_json::json!({
            "harmony": rs.harmony,
            "dissonance": rs.dissonance,
            "resources": rs.resources,
            "political_tension": rs.political_tension,
        })))
    } else {
        Err(StatusCode::NOT_FOUND.into())
    }
}

// Background task to simulate world dynamics
async fn cosmic_metabolism_task(state: WorldEngineState) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    
    loop {
        interval.tick().await;
        state.update_cosmic_metabolism().await;
    }
}

// Handle ecosystem queries
async fn get_region_ecosystem(
    State(state): State<WorldEngineState>,
    Path(region_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let region_uuid = uuid::Uuid::parse_str(&region_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let region_id = RegionId(region_uuid);
    
    // For MVP, generate a simple ecosystem response
    let regions = state.regions.read().await;
    let region = regions.get(&region_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Create sample ecosystem data
    let notable_creatures = vec![
        serde_json::json!({
            "species": "Star-Horned Stag",
            "x": 100.0,
            "z": 200.0,
            "behavior": "Foraging"
        }),
        serde_json::json!({
            "species": "Melody Bird",
            "x": 250.0,
            "z": 150.0,
            "behavior": "Singing"
        }),
        serde_json::json!({
            "species": "Grotto Turtle",
            "x": 50.0,
            "z": 300.0,
            "behavior": "Resting"
        }),
    ];
    
    Ok(Json(serde_json::json!({
        "region_id": region_id.0.to_string(),
        "biodiversity_index": 0.75,
        "creature_count": 12,
        "flora_count": 25,
        "harmony_influence": region.harmony_level / 100.0,
        "notable_creatures": notable_creatures,
        "ecosystem_health": if region.harmony_level > 70.0 { "Thriving" } else if region.harmony_level > 40.0 { "Stable" } else { "Declining" }
    })))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting World Engine Service...");
    
    let state = WorldEngineState::new();
    let monitor = Arc::new(HealthMonitor::new("world-engine", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("world-engine".to_string(), "http://localhost:3002".to_string())
        .await;
    
    // Start background task
    let bg_state = state.clone();
    tokio::spawn(cosmic_metabolism_task(bg_state));
    
    // Build router
    let app = Router::new()
        .route("/info", get(get_service_info))
        .route("/regions", get(get_all_regions))
        .route("/regions/:id", get(get_region))
        .route("/harmony", post(update_harmony))
        .route("/regions/:id/ecosystem", get(get_region_ecosystem))
        .route("/action", post(post_player_action))
        .route("/metabolism/:region", get(get_metabolism_state))
        .with_state(state.clone())
        .merge(monitor.clone().axum_routes())
        ;
    
    let addr = "0.0.0.0:3002";
    info!("World Engine listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}