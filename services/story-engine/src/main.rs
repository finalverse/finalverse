// services/story-engine/src/main.rs

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
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayerChronicle {
    player_id: PlayerId,
    legends: Vec<Legend>,
    current_quest: Option<Quest>,
    quest_history: Vec<CompletedQuest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Legend {
    id: Uuid,
    title: String,
    description: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    impact: LegendImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LegendImpact {
    Minor,
    Notable,
    Major,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quest {
    id: Uuid,
    title: String,
    description: String,
    objectives: Vec<Objective>,
    reward_resonance: Resonance,
    quest_giver: String,
    region: RegionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Objective {
    id: Uuid,
    description: String,
    completed: bool,
    progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompletedQuest {
    quest: Quest,
    completed_at: chrono::DateTime<chrono::Utc>,
    resonance_gained: Resonance,
}

#[derive(Clone)]
struct StoryEngineState {
    chronicles: Arc<RwLock<HashMap<PlayerId, PlayerChronicle>>>,
    active_quests: Arc<RwLock<HashMap<Uuid, Quest>>>,
    event_bus: Arc<dyn EventBus>,
    ai_service_url: String,
    start_time: std::time::Instant,
}

impl StoryEngineState {
    fn new(event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            chronicles: Arc::new(RwLock::new(HashMap::new())),
            active_quests: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
            ai_service_url: std::env::var("AI_SERVICE_URL")
                .unwrap_or_else(|_| "http://ai-orchestra:3004".to_string()),
            start_time: std::time::Instant::now(),
        }
    }
    
    async fn record_legend(&self, player_id: PlayerId, title: String, description: String, impact: LegendImpact) {
        let mut chronicles = self.chronicles.write().await;
        let chronicle = chronicles.entry(player_id.clone()).or_insert_with(|| PlayerChronicle {
            player_id: player_id.clone(),
            legends: Vec::new(),
            current_quest: None,
            quest_history: Vec::new(),
        });
        
        let legend = Legend {
            id: Uuid::new_v4(),
            title,
            description,
            timestamp: chrono::Utc::now(),
            impact,
        };
        
        chronicle.legends.push(legend.clone());
        
        info!("Recorded legend for player {:?}: {}", player_id, legend.title);
    }
    
    async fn generate_quest(&self, player_id: PlayerId, context: serde_json::Value) -> Result<Quest, FinalverseError> {
        // Call AI service to generate quest
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/quest/generate", self.ai_service_url))
            .json(&serde_json::json!({
                "context": context,
                "parameters": {
                    "difficulty": "medium",
                    "player_id": player_id.0.to_string()
                }
            }))
            .send()
            .await
            .map_err(|e| FinalverseError::ServiceError(format!("AI service error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(FinalverseError::ServiceError("Failed to generate quest".to_string()));
        }
        
        let ai_response: serde_json::Value = response.json().await
            .map_err(|e| FinalverseError::ServiceError(format!("Failed to parse AI response: {}", e)))?;
        
        // Extract quest from AI response
        let quest = Quest {
            id: Uuid::new_v4(),
            title: ai_response["quest"]["title"].as_str().unwrap_or("Mystery Quest").to_string(),
            description: ai_response["quest"]["description"].as_str().unwrap_or("A new adventure awaits").to_string(),
            objectives: vec![
                Objective {
                    id: Uuid::new_v4(),
                    description: "Begin your journey".to_string(),
                    completed: false,
                    progress: 0.0,
                }
            ],
            reward_resonance: Resonance {
                creative: 20,
                exploration: 15,
                restoration: 10,
            },
            quest_giver: "The Song itself".to_string(),
            region: RegionId(Uuid::new_v4()),
        };
        
        Ok(quest)
    }
}

// API handlers
async fn get_service_info(State(state): State<StoryEngineState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "story-engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn get_player_chronicle(
    State(state): State<StoryEngineState>,
    Path(player_id): Path<String>,
) -> Result<Json<PlayerChronicle>, StatusCode> {
    let player_uuid = Uuid::parse_str(&player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let chronicles = state.chronicles.read().await;
    let chronicle = chronicles.get(&player_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(chronicle.clone()))
}

async fn generate_personal_quest(
    State(state): State<StoryEngineState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<Quest>, StatusCode> {
    let player_id = request.get("player_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let player_uuid = Uuid::parse_str(player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    // Get player's chronicle for context
    let chronicles = state.chronicles.read().await;
    let chronicle = chronicles.get(&player_id);
    
    let context = serde_json::json!({
        "player_legends": chronicle.map(|c| c.legends.len()).unwrap_or(0),
        "completed_quests": chronicle.map(|c| c.quest_history.len()).unwrap_or(0),
        "region": request.get("region").cloned().unwrap_or(serde_json::json!("Terra Nova")),
    });
    
    match state.generate_quest(player_id.clone(), context).await {
        Ok(quest) => {
            // Store quest
            let mut quests = state.active_quests.write().await;
            quests.insert(quest.id, quest.clone());
            
            // Update player's current quest
            let mut chronicles = state.chronicles.write().await;
            if let Some(chronicle) = chronicles.get_mut(&player_id) {
                chronicle.current_quest = Some(quest.clone());
            }
            
            Ok(Json(quest))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn complete_quest(
    State(state): State<StoryEngineState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_id = request.get("player_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let quest_id = request.get("quest_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let player_uuid = Uuid::parse_str(player_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let player_id = PlayerId(player_uuid);
    
    let quest_uuid = Uuid::parse_str(quest_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut chronicles = state.chronicles.write().await;
    let chronicle = chronicles.get_mut(&player_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if let Some(current_quest) = chronicle.current_quest.take() {
        if current_quest.id == quest_uuid {
            let completed = CompletedQuest {
                quest: current_quest.clone(),
                completed_at: chrono::Utc::now(),
                resonance_gained: current_quest.reward_resonance.clone(),
            };

            chronicle.quest_history.push(completed.clone());
            
            // Record as a legend
            state.record_legend(
                player_id.clone(),
                format!("Completed: {}", current_quest.title),
                current_quest.description.clone(),
                LegendImpact::Notable,
            ).await;
            
            // Publish event
            let _ = state.event_bus.publish(FinalverseEvent::PlayerConnected {
                player: player_id,
                timestamp: chrono::Utc::now(),
            }).await;
            
            Ok(Json(serde_json::json!({
                "success": true,
                "resonance_gained": completed.resonance_gained,
                "new_legend": format!("Completed: {}", current_quest.title),
            })))
        } else {
            chronicle.current_quest = Some(current_quest);
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// Event handler
async fn handle_events(state: StoryEngineState) {
    let mut receiver = state.event_bus.subscribe("story-engine").await.unwrap();
    
    while let Some(event) = receiver.recv().await {
        match event {
            FinalverseEvent::HarmonyRestored { region, restorer, amount } => {
                if amount > 20.0 {
                    state.record_legend(
                        restorer,
                        "Harmony Restorer".to_string(),
                        format!("Restored significant harmony to region {:?}", region),
                        LegendImpact::Major,
                    ).await;
                }
            }
            FinalverseEvent::EchoBondIncreased { player, echo, new_level } => {
                if new_level == 100 {
                    state.record_legend(
                        player,
                        format!("True Friend of {}", echo.0),
                        format!("Achieved maximum bond with Echo {}", echo.0),
                        LegendImpact::Legendary,
                    ).await;
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting Story Engine Service...");
    
    let event_bus = Arc::new(InMemoryEventBus::new());
    let state = StoryEngineState::new(event_bus);
    
    // Start event handler
    let event_state = state.clone();
    tokio::spawn(handle_events(event_state));
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/chronicle/:player_id", get(get_player_chronicle))
        .route("/quest/generate", post(generate_personal_quest))
        .route("/quest/complete", post(complete_quest))
        .with_state(state);
    
    let addr = "0.0.0.0:3005";
    info!("Story Engine listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// services/story-engine/Cargo.toml
/*
[package]
name = "story-engine"
version.workspace = true
edition.workspace = true

[[bin]]
name = "story-engine"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
reqwest = { version = "0.11", features = ["json"] }
*/