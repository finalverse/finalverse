use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use finalverse_common::{
    events::FinalverseEvent,
    types::{PlayerId, Quest, QuestType, QuestObjective, QuestReward, RewardType},
};
use finalverse_common::error::{FinalverseError, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::RwLock;
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use finalverse_health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StoryEngineState {
    player_chronicles: HashMap<PlayerId, PlayerChronicle>,
    active_quests: HashMap<String, Quest>,
    completed_quests: HashMap<PlayerId, Vec<String>>,
    world_events: Vec<FinalverseEvent>,
    ai_service_url: String,
}

type SharedStoryState = Arc<RwLock<StoryEngineState>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChronicle {
    pub player_id: PlayerId,
    pub achievements: Vec<Achievement>,
    pub story_arcs: Vec<StoryArc>,
    pub relationships: HashMap<String, f32>, // NPC relationships
    pub reputation: HashMap<String, f32>,    // Faction reputations
    pub memorable_moments: Vec<MemorableMoment>,
    pub character_growth: CharacterGrowth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub achieved_at: DateTime<Utc>,
    pub significance: AchievementSignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementSignificance {
    Personal,
    Community,
    Regional,
    Global,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryArc {
    pub id: String,
    pub title: String,
    pub current_chapter: u32,
    pub completed: bool,
    pub key_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorableMoment {
    pub id: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub emotional_impact: f32,
    pub witnesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGrowth {
    pub personality_traits: HashMap<String, f32>,
    pub learned_lessons: Vec<String>,
    pub evolved_beliefs: Vec<String>,
    pub mentor_relationships: Vec<String>,
}

#[derive(Serialize)]
struct ServiceInfo {
    name: String,
    version: String,
    status: String,
    active_chronicles: usize,
    active_quests: usize,
}

#[derive(Deserialize)]
struct QuestGenerationRequest {
    player_id: String,
    context: String,
    difficulty: Option<String>,
    quest_type: Option<String>,
}

#[derive(Serialize)]
struct QuestGenerationResponse {
    quest: Quest,
    narrative_hook: String,
    estimated_duration: u32,
}

#[derive(Deserialize)]
struct ChronicleUpdateRequest {
    player_id: String,
    event_type: String,
    description: String,
    emotional_impact: Option<f32>,
    witnesses: Option<Vec<String>>,
}

#[derive(Serialize)]
struct ChronicleUpdateResponse {
    success: bool,
    chronicle_updated: bool,
    new_achievements: Vec<String>,
    story_progression: Option<String>,
}

impl StoryEngineState {
    pub fn new() -> Self {
        Self {
            player_chronicles: HashMap::new(),
            active_quests: HashMap::new(),
            completed_quests: HashMap::new(),
            world_events: Vec::new(),
            ai_service_url: "http://localhost:3001".to_string(),
        }
    }

    pub fn get_or_create_chronicle(&mut self, player_id: PlayerId) -> &mut PlayerChronicle {
        self.player_chronicles.entry(player_id.clone()).or_insert_with(|| {
            PlayerChronicle {
                player_id: player_id.clone(),
                achievements: Vec::new(),
                story_arcs: vec![
                    StoryArc {
                        id: "songweaver_awakening".to_string(),
                        title: "The Songweaver's Awakening".to_string(),
                        current_chapter: 1,
                        completed: false,
                        key_decisions: Vec::new(),
                    }
                ],
                relationships: HashMap::new(),
                reputation: HashMap::new(),
                memorable_moments: Vec::new(),
                character_growth: CharacterGrowth {
                    personality_traits: HashMap::new(),
                    learned_lessons: Vec::new(),
                    evolved_beliefs: Vec::new(),
                    mentor_relationships: Vec::new(),
                },
            }
        })
    }

    pub async fn generate_quest_with_ai(&self, context: &str, difficulty: &str, quest_type: &str) -> Result<Quest> {
        let client = reqwest::Client::new();

        let response = client
            .post(&format!("{}/api/quest", self.ai_service_url))
            .json(&serde_json::json!({
                "player_context": context,
                "world_state": "Current harmony levels and active events",
                "difficulty": difficulty,
                "quest_type": quest_type
            }))
            .send()
            .await
            .map_err(|e| FinalverseError::ServiceError(format!("AI service error: {}", e)))?;

        if response.status().is_success() {
            let ai_response: serde_json::Value = response.json().await
                .map_err(|e| FinalverseError::NetworkError(e))?;

            // Extract narrative from AI response and create quest
            let narrative = ai_response["quest_narrative"]
                .as_str()
                .unwrap_or("Embark on a journey to strengthen the Song of Creation");

            let quest_id = Uuid::new_v4().to_string();

            let quest_type_enum = match quest_type {
                "exploration" => QuestType::Exploration,
                "harmony" => QuestType::Harmony,
                "creation" => QuestType::Creation,
                "protection" => QuestType::Protection,
                "discovery" => QuestType::Discovery,
                _ => QuestType::Social,
            };

            Ok(Quest {
                id: quest_id.clone(),
                title: format!("Quest: {}", quest_type),
                description: narrative.to_string(),
                quest_type: quest_type_enum,
                objectives: vec![
                    QuestObjective {
                        id: format!("{}_obj1", quest_id),
                        description: "Complete the primary objective".to_string(),
                        completed: false,
                        progress: 0.0,
                        target: 1.0,
                    }
                ],
                rewards: vec![
                    QuestReward {
                        reward_type: RewardType::Resonance,
                        amount: match difficulty {
                            "easy" => 50,
                            "normal" => 100,
                            "hard" => 200,
                            _ => 75,
                        },
                        item_id: None,
                    }
                ],
                prerequisites: Vec::new(),
            })
        } else {
            Err(FinalverseError::AIServiceError("Failed to generate quest".to_string()))
        }
    }

    pub fn add_memorable_moment(&mut self, player_id: &PlayerId, description: String, emotional_impact: f32, witnesses: Vec<String>) {
        let chronicle = self.get_or_create_chronicle(player_id.clone());

        let moment = MemorableMoment {
            id: Uuid::new_v4().to_string(),
            description,
            timestamp: Utc::now(),
            emotional_impact,
            witnesses,
        };

        chronicle.memorable_moments.push(moment);

        // Check for achievement triggers
        self.check_achievement_triggers(player_id.clone());
    }

    fn check_achievement_triggers(&mut self, player_id: PlayerId) {
        let chronicle = self.get_or_create_chronicle(player_id);
        let moment_count = chronicle.memorable_moments.len();

        // Example achievement triggers
        if moment_count == 1 && !chronicle.achievements.iter().any(|a| a.id == "first_memory") {
            chronicle.achievements.push(Achievement {
                id: "first_memory".to_string(),
                title: "First Memory".to_string(),
                description: "Created your first memorable moment in Finalverse".to_string(),
                achieved_at: Utc::now(),
                significance: AchievementSignificance::Personal,
            });
        }

        if moment_count >= 10 && !chronicle.achievements.iter().any(|a| a.id == "chronicler") {
            chronicle.achievements.push(Achievement {
                id: "chronicler".to_string(),
                title: "Chronicler".to_string(),
                description: "Accumulated 10 memorable moments".to_string(),
                achieved_at: Utc::now(),
                significance: AchievementSignificance::Community,
            });
        }
    }
}


async fn generate_quest(
    State(state): State<SharedStoryState>,
    Json(request): Json<QuestGenerationRequest>,
) -> impl IntoResponse {
    let player_id = PlayerId(request.player_id);
    let difficulty = request.difficulty.unwrap_or_else(|| "normal".to_string());
    let quest_type = request.quest_type.unwrap_or_else(|| "harmony".to_string());

    let quest = {
        let story_state = state.read().await;
        match story_state.generate_quest_with_ai(&request.context, &difficulty, &quest_type).await {
            Ok(quest) => quest,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to generate quest: {}", e)
            }))),
        }
    };

    // Store the quest
    {
        let mut story_state = state.write().await;
        story_state.active_quests.insert(quest.id.clone(), quest.clone());
    }

    let response = QuestGenerationResponse {
        narrative_hook: format!("A new quest awaits: {}", quest.description),
        estimated_duration: 30, // minutes
        quest: quest,
    };
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn update_chronicle(
    State(state): State<SharedStoryState>,
    Json(request): Json<ChronicleUpdateRequest>,
) -> impl IntoResponse {
    let player_id = PlayerId(request.player_id);
    let emotional_impact = request.emotional_impact.unwrap_or(1.0);
    let witnesses = request.witnesses.unwrap_or_default();

    let (new_achievements, story_progression) = {
        let mut story_state = state.write().await;
        let initial_achievement_count = story_state.get_or_create_chronicle(player_id.clone()).achievements.len();

        story_state.add_memorable_moment(&player_id, request.description, emotional_impact, witnesses);

        let final_achievement_count = story_state.get_or_create_chronicle(player_id.clone()).achievements.len();
        let new_achievements: Vec<String> = if final_achievement_count > initial_achievement_count {
            story_state.get_or_create_chronicle(player_id.clone())
                .achievements
                .iter()
                .skip(initial_achievement_count)
                .map(|a| a.title.clone())
                .collect()
        } else {
            Vec::new()
        };

        let story_progression = if !new_achievements.is_empty() {
            Some("Your legend grows, Songweaver!".to_string())
        } else {
            None
        };

        (new_achievements, story_progression)
    };

    let response = ChronicleUpdateResponse {
        success: true,
        chronicle_updated: true,
        new_achievements,
        story_progression,
    };
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn get_player_chronicle(
    Path(player_id): Path<String>,
    State(state): State<SharedStoryState>,
) -> impl IntoResponse {
    let player_id = PlayerId(player_id);

    let chronicle = {
        let mut story_state = state.write().await;
        story_state.get_or_create_chronicle(player_id).clone()
    };

    (StatusCode::OK, Json(chronicle))
}

async fn get_quest(
    Path(quest_id): Path<String>,
    State(state): State<SharedStoryState>,
) -> impl IntoResponse {
    let story_state = state.read().await;

    match story_state.active_quests.get(&quest_id) {
        Some(quest) => {
            let json_quest = serde_json::to_value(quest).unwrap();
            (StatusCode::OK, Json(json_quest))
        }
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Quest not found"
        }))),
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(StoryEngineState::new()));

    let monitor = Arc::new(HealthMonitor::new("story-engine", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("story-engine".to_string(), "http://localhost:3003".to_string())
        .await;

    let app = Router::new()
        .merge(monitor.clone().axum_routes())
        .route("/api/quest/generate", post(generate_quest))
        .route("/api/chronicle/update", post(update_chronicle))
        .route("/api/chronicle/:player_id", get(get_player_chronicle))
        .route("/api/quest/:quest_id", get(get_quest))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    println!("Story Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}