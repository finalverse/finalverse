use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use finalverse_common::{
    events::HarmonyEvent,
    types::{EchoId, EchoState, EchoType, PlayerId, RegionId},
    FinalverseError, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use finalverse_health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;

#[derive(Debug, Clone)]
pub struct EchoEngineState {
    echoes: HashMap<EchoId, EchoState>,
    player_bonds: HashMap<PlayerId, HashMap<EchoId, f32>>,
}

type SharedEchoState = Arc<RwLock<EchoEngineState>>;

#[derive(Serialize)]
struct ServiceInfo {
    name: String,
    version: String,
    status: String,
    active_echoes: usize,
}

#[derive(Deserialize)]
struct BondRequest {
    player_id: String,
    echo_id: String,
    interaction_type: String,
}

#[derive(Serialize)]
struct BondResponse {
    echo_id: String,
    new_bond_level: f32,
    abilities_unlocked: Vec<String>,
    message: String,
}

#[derive(Deserialize)]
struct TeachingRequest {
    player_id: String,
    echo_id: String,
    skill_requested: String,
}

#[derive(Serialize)]
struct TeachingResponse {
    success: bool,
    skill_learned: Option<String>,
    requirements_met: bool,
    message: String,
}

impl EchoEngineState {
    pub fn new() -> Self {
        let mut echoes = HashMap::new();

        // Initialize the Four First Echoes
        echoes.insert(
            EchoId("lumi".to_string()),
            EchoState {
                echo_type: EchoType::Lumi,
                current_region: Some(RegionId("terra_nova".to_string())),
                bond_levels: HashMap::new(),
                active_teachings: vec![
                    "melody_of_hope".to_string(),
                    "light_weaving".to_string(),
                    "discovery_sense".to_string(),
                ],
            },
        );

        echoes.insert(
            EchoId("kai".to_string()),
            EchoState {
                echo_type: EchoType::KAI,
                current_region: Some(RegionId("technos_prime".to_string())),
                bond_levels: HashMap::new(),
                active_teachings: vec![
                    "logic_harmony".to_string(),
                    "code_weaving".to_string(),
                    "analysis_matrix".to_string(),
                ],
            },
        );

        echoes.insert(
            EchoId("terra".to_string()),
            EchoState {
                echo_type: EchoType::Terra,
                current_region: Some(RegionId("whispering_wilds".to_string())),
                bond_levels: HashMap::new(),
                active_teachings: vec![
                    "nature_song".to_string(),
                    "growth_weaving".to_string(),
                    "resilience_core".to_string(),
                ],
            },
        );

        echoes.insert(
            EchoId("ignis".to_string()),
            EchoState {
                echo_type: EchoType::Ignis,
                current_region: Some(RegionId("star_sailor_expanse".to_string())),
                bond_levels: HashMap::new(),
                active_teachings: vec![
                    "courage_flame".to_string(),
                    "creation_forge".to_string(),
                    "inspiration_burst".to_string(),
                ],
            },
        );

        Self {
            echoes,
            player_bonds: HashMap::new(),
        }
    }

    pub fn get_echo(&self, echo_id: &EchoId) -> Option<&EchoState> {
        self.echoes.get(echo_id)
    }

    pub fn get_bond_level(&self, player_id: &PlayerId, echo_id: &EchoId) -> f32 {
        self.player_bonds
            .get(player_id)
            .and_then(|bonds| bonds.get(echo_id))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn increase_bond(&mut self, player_id: PlayerId, echo_id: EchoId, amount: f32) -> f32 {
        let player_bonds = self.player_bonds.entry(player_id.clone()).or_insert_with(HashMap::new);
        let current_bond = player_bonds.entry(echo_id.clone()).or_insert(0.0);
        *current_bond += amount;
        *current_bond = current_bond.min(100.0); // Cap at 100

        // Update the echo's bond tracking
        if let Some(echo) = self.echoes.get_mut(&echo_id) {
            echo.bond_levels.insert(player_id, *current_bond);
        }

        *current_bond
    }
}


async fn get_echo_info(
    Path(echo_id): Path<String>,
    State(state): State<SharedEchoState>,
) -> impl IntoResponse {
    let echo_state = state.read().unwrap();
    let echo_id = EchoId(echo_id);

    match echo_state.get_echo(&echo_id) {
        Some(echo) => (StatusCode::OK, Json(serde_json::json!({
            "echo_id": echo_id.0,
            "echo_type": echo.echo_type,
            "current_region": echo.current_region,
            "active_teachings": echo.active_teachings,
            "total_bonds": echo.bond_levels.len()
        }))),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Echo not found"
        }))),
    }
}

async fn interact_with_echo(
    State(state): State<SharedEchoState>,
    Json(request): Json<BondRequest>,
) -> impl IntoResponse {
    let player_id = PlayerId(request.player_id);
    let echo_id = EchoId(request.echo_id);

    let mut echo_state = state.write().unwrap();

    // Check if echo exists
    if !echo_state.echoes.contains_key(&echo_id) {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Echo not found"
        })));
    }

    // Calculate bond increase based on interaction type
    let bond_increase = match request.interaction_type.as_str() {
        "help_others" => 5.0,
        "solve_puzzle" => 3.0,
        "creative_act" => 4.0,
        "brave_action" => 6.0,
        "discovery" => 3.5,
        _ => 1.0,
    };

    let new_bond_level = echo_state.increase_bond(player_id.clone(), echo_id.clone(), bond_increase);

    // Determine abilities unlocked based on bond level
    let abilities_unlocked = match new_bond_level {
        level if level >= 25.0 && level < 30.0 => vec!["basic_melody".to_string()],
        level if level >= 50.0 && level < 55.0 => vec!["intermediate_harmony".to_string()],
        level if level >= 75.0 && level < 80.0 => vec!["advanced_symphony".to_string()],
        _ => vec![],
    };

    let message = format!(
        "Your bond with {} has increased to {:.1}. {}",
        echo_id.0,
        new_bond_level,
        if !abilities_unlocked.is_empty() {
            "New abilities unlocked!"
        } else {
            "Keep strengthening your bond to unlock new abilities."
        }
    );

    let response = BondResponse {
        echo_id: echo_id.0,
        new_bond_level,
        abilities_unlocked,
        message,
    };
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn request_teaching(
    State(state): State<SharedEchoState>,
    Json(request): Json<TeachingRequest>,
) -> impl IntoResponse {
    let player_id = PlayerId(request.player_id);
    let echo_id = EchoId(request.echo_id);

    let echo_state = state.read().unwrap();

    // Check if echo exists
    let echo = match echo_state.get_echo(&echo_id) {
        Some(echo) => echo,
        None => return (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Echo not found"
        }))),
    };

    // Check bond level requirement
    let bond_level = echo_state.get_bond_level(&player_id, &echo_id);
    let required_bond = match request.skill_requested.as_str() {
        "basic_melody" => 25.0,
        "intermediate_harmony" => 50.0,
        "advanced_symphony" => 75.0,
        _ => 10.0,
    };

    if bond_level < required_bond {
        let response = TeachingResponse {
            success: false,
            skill_learned: None,
            requirements_met: false,
            message: format!(
                "Your bond level ({:.1}) is too low. Required: {:.1}",
                bond_level, required_bond
            ),
        };
        let json_response = serde_json::to_value(response).unwrap();
        return (StatusCode::OK, Json(json_response));
    }

    // Check if echo can teach this skill
    if !echo.active_teachings.contains(&request.skill_requested) {
        let response = TeachingResponse {
            success: false,
            skill_learned: None,
            requirements_met: true,
            message: format!("{} cannot teach this skill.", echo_id.0),
        };
        let json_response = serde_json::to_value(response).unwrap();
        return (StatusCode::OK, Json(json_response));
    }

    let response = TeachingResponse {
        success: true,
        skill_learned: Some(request.skill_requested.clone()),
        requirements_met: true,
        message: format!(
            "{} has taught you {}! Practice it well, Songweaver.",
            echo_id.0, request.skill_requested
        ),
    };
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn get_player_bonds(
    Path(player_id): Path<String>,
    State(state): State<SharedEchoState>,
) -> impl IntoResponse {
    let player_id = PlayerId(player_id);
    let echo_state = state.read().unwrap();

    let bonds = echo_state.player_bonds
        .get(&player_id)
        .cloned()
        .unwrap_or_default();

    (StatusCode::OK, Json(serde_json::json!({
        "player_id": player_id.0,
        "bonds": bonds
    })))
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(EchoEngineState::new()));
    let monitor = Arc::new(HealthMonitor::new("echo-engine", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("echo-engine".to_string(), "http://localhost:3004".to_string())
        .await;

    let app = Router::new()
        .merge(monitor.clone().axum_routes())
        .route("/echo/:echo_id", get(get_echo_info))
        .route("/interact", post(interact_with_echo))
        .route("/teach", post(request_teaching))
        .route("/bonds/:player_id", get(get_player_bonds))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3004));
    println!("Echo Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}