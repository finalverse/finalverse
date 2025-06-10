use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use finalverse_health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod llm_integration;
pub use llm_integration::{LLMOrchestra, GenerationRequest, GenerationResponse};

#[derive(Debug, Clone)]
pub struct AIState {
    orchestra: LLMOrchestra,
    active_sessions: u32,
}

type SharedAIState = Arc<RwLock<AIState>>;

#[derive(Serialize)]
struct ServiceInfo {
    name: String,
    version: String,
    status: String,
    active_sessions: u32,
}

#[derive(Deserialize)]
struct QuestGenerationRequest {
    player_context: String,
    world_state: String,
    quest_type: Option<String>,
}

#[derive(Serialize)]
struct QuestGenerationResponse {
    quest_narrative: String,
    quest_id: String,
    estimated_duration: u32,
}

#[derive(Deserialize)]
struct DialogueRequest {
    npc_id: String,
    personality: String,
    conversation_context: String,
    player_history: String,
}

#[derive(Serialize)]
struct DialogueResponse {
    dialogue: String,
    npc_emotion: String,
    suggested_responses: Vec<String>,
}

#[derive(Deserialize)]
struct WorldDescriptionRequest {
    region_name: String,
    harmony_level: f32,
    time_of_day: String,
    weather: Option<String>,
}

#[derive(Serialize)]
struct WorldDescriptionResponse {
    description: String,
    atmospheric_details: Vec<String>,
    suggested_activities: Vec<String>,
}

impl AIState {
    pub fn new() -> Self {
        Self {
            orchestra: LLMOrchestra::new(),
            active_sessions: 0,
        }
    }
}


async fn generate_text(
    State(state): State<SharedAIState>,
    Json(request): Json<GenerationRequest>,
) -> impl IntoResponse {
    let orchestra = {
        let ai_state = state.read().unwrap();
        ai_state.orchestra.clone()
    };

    match orchestra.generate(request).await {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenerationResponse {
                text: format!("Error generating text: {}", e),
                model_used: "error".to_string(),
                tokens_used: 0,
            }),
        ),
    }
}

async fn generate_quest(
    State(state): State<SharedAIState>,
    Json(request): Json<QuestGenerationRequest>,
) -> impl IntoResponse {
    let orchestra = {
        let ai_state = state.read().unwrap();
        ai_state.orchestra.clone()
    };

    match llm_integration::generate_quest_narrative(
        &orchestra,
        &request.player_context,
        &request.world_state,
    ).await {
        Ok(narrative) => {
            let quest_id = uuid::Uuid::new_v4().to_string();
            (
                StatusCode::OK,
                Json(QuestGenerationResponse {
                    quest_narrative: narrative,
                    quest_id,
                    estimated_duration: 30, // minutes
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(QuestGenerationResponse {
                quest_narrative: format!("Error generating quest: {}", e),
                quest_id: "error".to_string(),
                estimated_duration: 0,
            }),
        ),
    }
}

async fn generate_dialogue(
    State(state): State<SharedAIState>,
    Json(request): Json<DialogueRequest>,
) -> impl IntoResponse {
    let orchestra = {
        let ai_state = state.read().unwrap();
        ai_state.orchestra.clone()
    };

    match llm_integration::generate_npc_dialogue(
        &orchestra,
        &request.personality,
        &request.conversation_context,
        &request.player_history,
    ).await {
        Ok(dialogue) => (
            StatusCode::OK,
            Json(DialogueResponse {
                dialogue,
                npc_emotion: "neutral".to_string(),
                suggested_responses: vec![
                    "Tell me more".to_string(),
                    "How can I help?".to_string(),
                    "Thank you".to_string(),
                ],
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DialogueResponse {
                dialogue: format!("Error generating dialogue: {}", e),
                npc_emotion: "confused".to_string(),
                suggested_responses: vec!["Goodbye".to_string()],
            }),
        ),
    }
}

async fn generate_world_description(
    State(state): State<SharedAIState>,
    Json(request): Json<WorldDescriptionRequest>,
) -> impl IntoResponse {
    let orchestra = {
        let ai_state = state.read().unwrap();
        ai_state.orchestra.clone()
    };

    match llm_integration::generate_world_description(
        &orchestra,
        &request.region_name,
        request.harmony_level,
        &request.time_of_day,
    ).await {
        Ok(description) => (
            StatusCode::OK,
            Json(WorldDescriptionResponse {
                description,
                atmospheric_details: vec![
                    "Gentle melodies drift through the air".to_string(),
                    "Colors seem more vibrant than usual".to_string(),
                    "A sense of peace pervades the area".to_string(),
                ],
                suggested_activities: vec![
                    "Explore the nearby groves".to_string(),
                    "Listen for hidden melodies".to_string(),
                    "Practice songweaving".to_string(),
                ],
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WorldDescriptionResponse {
                description: format!("Error generating description: {}", e),
                atmospheric_details: vec![],
                suggested_activities: vec![],
            }),
        ),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let state = Arc::new(RwLock::new(AIState::new()));
    let monitor = Arc::new(HealthMonitor::new("ai-orchestra", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("ai-orchestra".to_string(), "http://localhost:3001".to_string())
        .await;

    let app = Router::new()
        .merge(monitor.clone().axum_routes())
        .route("/api/generate", post(generate_text))
        .route("/api/quest", post(generate_quest))
        .route("/api/dialogue", post(generate_dialogue))
        .route("/api/world-description", post(generate_world_description))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("AI Orchestra listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}