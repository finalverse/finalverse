// services/harmony-service/src/main.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use finalverse_common::*;
use finalverse_protocol::{event_bus::InMemoryEventBus, *};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayerProgression {
    player_id: PlayerId,
    resonance: Resonance,
    attunement_tier: AttunementTier,
    unlocked_melodies: Vec<UnlockedMelody>,
    unlocked_harmonies: Vec<UnlockedHarmony>,
    total_actions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AttunementTier {
    Novice,          // 0-100 total resonance
    Apprentice,      // 100-500
    Journeyman,      // 500-2000
    Expert,          // 2000-10000
    Master,          // 10000-50000
    Grandmaster,     // 50000+
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnlockedMelody {
    id: String,
    name: String,
    melody_type: MelodyType,
    power_level: u32,
    unlocked_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MelodyType {
    Healing,
    Creation,
    Discovery,
    Courage,
    Protection,
    Transformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnlockedHarmony {
    id: String,
    name: String,
    description: String,
    required_echoes: Vec<EchoType>,
    unlocked_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
struct HarmonyServiceState {
    progressions: Arc<RwLock<HashMap<PlayerId, PlayerProgression>>>,
    melody_library: Arc<RwLock<HashMap<String, MelodyDefinition>>>,
    harmony_library: Arc<RwLock<HashMap<String, HarmonyDefinition>>>,
    event_bus: Arc<dyn EventBus>,
    start_time: std::time::Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MelodyDefinition {
    id: String,
    name: String,
    melody_type: MelodyType,
    base_power: u32,
    resonance_requirement: Resonance,
    echo_requirement: Option<(EchoType, u32)>, // Echo type and bond level
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HarmonyDefinition {
    id: String,
    name: String,
    description: String,
    required_echoes: Vec<(EchoType, u32)>, // Echo types and minimum bond levels
    resonance_requirement: Resonance,
}

impl HarmonyServiceState {
    fn new(event_bus: Arc<dyn EventBus>) -> Self {
        let mut melody_library = HashMap::new();
        let mut harmony_library = HashMap::new();
        
        // Initialize basic melodies
        melody_library.insert("healing_touch".to_string(), MelodyDefinition {
            id: "healing_touch".to_string(),
            name: "Healing Touch".to_string(),
            melody_type: MelodyType::Healing,
            base_power: 10,
            resonance_requirement: Resonance { creative: 0, exploration: 0, restoration: 10 },
            echo_requirement: None,
        });
        
        melody_library.insert("light_of_hope".to_string(), MelodyDefinition {
            id: "light_of_hope".to_string(),
            name: "Light of Hope".to_string(),
            melody_type: MelodyType::Discovery,
            base_power: 15,
            resonance_requirement: Resonance { creative: 20, exploration: 30, restoration: 0 },
            echo_requirement: Some((EchoType::Lumi, 20)),
        });
        
        melody_library.insert("forge_of_will".to_string(), MelodyDefinition {
            id: "forge_of_will".to_string(),
            name: "Forge of Will".to_string(),
            melody_type: MelodyType::Creation,
            base_power: 25,
            resonance_requirement: Resonance { creative: 50, exploration: 0, restoration: 20 },
            echo_requirement: Some((EchoType::Ignis, 30)),
        });
        
        // Initialize harmonies
        harmony_library.insert("harmony_of_balance".to_string(), HarmonyDefinition {
            id: "harmony_of_balance".to_string(),
            name: "Harmony of Balance".to_string(),
            description: "Unite all four Echoes in perfect balance".to_string(),
            required_echoes: vec![
                (EchoType::Lumi, 50),
                (EchoType::KAI, 50),
                (EchoType::Terra, 50),
                (EchoType::Ignis, 50),
            ],
            resonance_requirement: Resonance { creative: 100, exploration: 100, restoration: 100 },
        });
        
        Self {
            progressions: Arc::new(RwLock::new(HashMap::new())),
            melody_library: Arc::new(RwLock::new(melody_library)),
            harmony_library: Arc::new(RwLock::new(harmony_library)),
            event_bus,
            start_time: std::time::Instant::now(),
        }
    }
    
    fn calculate_attunement_tier(resonance: &Resonance) -> AttunementTier {
        let total = resonance.creative + resonance.exploration + resonance.restoration;
        match total {
            0..=99 => AttunementTier::Novice,
            100..=499 => AttunementTier::Apprentice,
            500..=1999 => AttunementTier::Journeyman,
            2000..=9999 => AttunementTier::Expert,
            10000..=49999 => AttunementTier::Master,
            _ => AttunementTier::Grandmaster,
        }
    }
    
    async fn grant_resonance(&self, player_id: PlayerId, resonance_gain: Resonance) -> PlayerProgression {
        let mut progressions = self.progressions.write().await;
        let progression = progressions.entry(player_id.clone()).or_insert_with(|| PlayerProgression {
            player_id: player_id.clone(),
            resonance: Resonance { creative: 0, exploration: 0, restoration: 0 },
            attunement_tier: AttunementTier::Novice,
            unlocked_melodies: vec![],
            unlocked_harmonies: vec![],
            total_actions: 0,
        });
        
        // Add resonance
        progression.resonance.creative += resonance_gain.creative;
        progression.resonance.exploration += resonance_gain.exploration;
        progression.resonance.restoration += resonance_gain.restoration;
        progression.total_actions += 1;
        
        // Update tier
        let new_tier = Self::calculate_attunement_tier(&progression.resonance);
        let tier_changed = !matches!((&progression.attunement_tier, &new_tier), 
            (AttunementTier::Novice, AttunementTier::Novice) |
            (AttunementTier::Apprentice, AttunementTier::Apprentice) |
            (AttunementTier::Journeyman, AttunementTier::Journeyman) |
            (AttunementTier::Expert, AttunementTier::Expert) |
            (AttunementTier::Master, AttunementTier::Master) |
            (AttunementTier::Grandmaster, AttunementTier::Grandmaster)
        );
        
        if tier_changed {
            progression.attunement_tier = new_tier;
            info!("Player {:?} advanced to {:?} tier!", player_id, progression.attunement_tier);
        }
        
        progression.clone()
    }
    
    async fn check_melody_unlocks(&self, player_id: &PlayerId, progression: &PlayerProgression, echo_bonds: Option<HashMap<EchoType, u32>>) {
        let library = self.melody_library.read().await;
        let mut progressions = self.progressions.write().await;
        
        if let Some(prog) = progressions.get_mut(player_id) {
            for (id, definition) in library.iter() {
                // Check if already unlocked
                if prog.unlocked_melodies.iter().any(|m| m.id == *id) {
                    continue;
                }
                
                // Check resonance requirements
                let meets_resonance = prog.resonance.creative >= definition.resonance_requirement.creative
                    && prog.resonance.exploration >= definition.resonance_requirement.exploration
                    && prog.resonance.restoration >= definition.resonance_requirement.restoration;
                
                if !meets_resonance {
                    continue;
                }
                
                // Check echo requirements
                let meets_echo = if let Some((echo_type, required_bond)) = &definition.echo_requirement {
                    echo_bonds.as_ref()
                        .and_then(|bonds| bonds.get(echo_type))
                        .map(|bond| *bond >= *required_bond)
                        .unwrap_or(false)
                } else {
                    true
                };
                
                if meets_echo {
                    prog.unlocked_melodies.push(UnlockedMelody {
                        id: id.clone(),
                        name: definition.name.clone(),
                        melody_type: definition.melody_type.clone(),
                        power_level: definition.base_power,
                        unlocked_at: chrono::Utc::now(),
                    });
                    
                    info!("Player {:?} unlocked melody: {}", player_id, definition.name);
                }
            }
        }
    }
}

// API handlers
async fn get_service_info(State(state): State<HarmonyServiceState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "harmony-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn get_player_progression(
    State(state): State<HarmonyServiceState>,
    Path(player_id): Path<String>,
) -> Result<Json<PlayerProgression>> {
    let player_uuid = uuid::Uuid::parse_str(&player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let progressions = state.progressions.read().await;
    let progression = progressions.get(&player_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(progression.clone()))
}

async fn grant_resonance(
    State(state): State<HarmonyServiceState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<PlayerProgression>> {
    let player_id = request.get("player_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let resonance_gain = Resonance {
        creative: request.get("creative").and_then(|v| v.as_u64()).unwrap_or(0),
        exploration: request.get("exploration").and_then(|v| v.as_u64()).unwrap_or(0),
        restoration: request.get("restoration").and_then(|v| v.as_u64()).unwrap_or(0),
    };
    
    let player_uuid = uuid::Uuid::parse_str(player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let progression = state.grant_resonance(player_id.clone(), resonance_gain).await;
    
    // Check for new melody unlocks
    let echo_bonds = request.get("echo_bonds")
        .and_then(|v| serde_json::from_value::<HashMap<String, u32>>(v.clone()).ok())
        .map(|bonds| {
            bonds.into_iter().map(|(k, v)| {
                let echo_type = match k.as_str() {
                    "lumi" => EchoType::Lumi,
                    "kai" => EchoType::KAI,
                    "terra" => EchoType::Terra,
                    "ignis" => EchoType::Ignis,
                    _ => EchoType::Lumi, // Default
                };
                (echo_type, v)
            }).collect::<HashMap<_, _>>()
        });
    
    state.check_melody_unlocks(&player_id, &progression, echo_bonds).await;
    
    // Get updated progression
    let progressions = state.progressions.read().await;
    let updated_progression = progressions.get(&player_id).unwrap();
    
    Ok(Json(updated_progression.clone()))
}

async fn get_available_melodies(
    State(state): State<HarmonyServiceState>,
    Path(player_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let player_uuid = uuid::Uuid::parse_str(&player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let progressions = state.progressions.read().await;
    let progression = progressions.get(&player_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let library = state.melody_library.read().await;
    
    let available: Vec<_> = library.values()
        .filter(|def| {
            // Check if player meets requirements but hasn't unlocked yet
            let meets_resonance = progression.resonance.creative >= def.resonance_requirement.creative
                && progression.resonance.exploration >= def.resonance_requirement.exploration
                && progression.resonance.restoration >= def.resonance_requirement.restoration;
            
            let not_unlocked = !progression.unlocked_melodies.iter().any(|m| m.id == def.id);
            
            meets_resonance && not_unlocked
        })
        .cloned()
        .collect();
    
    Ok(Json(serde_json::json!({
        "unlocked": progression.unlocked_melodies,
        "available_to_unlock": available,
        "total_melodies": library.len(),
    })))
}

async fn get_harmonies(
    State(state): State<HarmonyServiceState>,
    Path(player_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let player_uuid = uuid::Uuid::parse_str(&player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let progressions = state.progressions.read().await;
    let progression = progressions.get(&player_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let library = state.harmony_library.read().await;
    
    Ok(Json(serde_json::json!({
        "unlocked": progression.unlocked_harmonies,
        "available": library.values().collect::<Vec<_>>(),
    })))
}

// Event handler
async fn handle_events(state: HarmonyServiceState) {
    let mut receiver = state.event_bus.subscribe("harmony-service").await.unwrap();
    
    while let Some(event) = receiver.recv().await {
        match event {
            FinalverseEvent::MelodyPerformed { player, melody, .. } => {
                // Grant resonance based on melody type
                let resonance_gain = match melody {
                    Melody::Healing { power } => Resonance {
                        creative: 0,
                        exploration: 0,
                        restoration: (power * 2.0) as u64,
                    },
                    Melody::Creation { .. } => Resonance {
                        creative: 20,
                        exploration: 5,
                        restoration: 5,
                    },
                    Melody::Discovery { range } => Resonance {
                        creative: 5,
                        exploration: (range * 1.5) as u64,
                        restoration: 0,
                    },
                    Melody::Courage { intensity } => Resonance {
                        creative: (intensity * 1.2) as u64,
                        exploration: 10,
                        restoration: 5,
                    },
                };
                
                state.grant_resonance(player, resonance_gain).await;
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting Harmony Service...");
    
    let event_bus = Arc::new(InMemoryEventBus::new());
    let state = HarmonyServiceState::new(event_bus);
    
    // Start event handler
    let event_state = state.clone();
    tokio::spawn(handle_events(event_state));
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/progression/:player_id", get(get_player_progression))
        .route("/grant", post(grant_resonance))
        .route("/melodies/:player_id", get(get_available_melodies))
        .route("/harmonies/:player_id", get(get_harmonies))
        .with_state(state);
    
    let addr = "0.0.0.0:3006";
    info!("Harmony Service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}