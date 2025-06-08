// services/ai-orchestra/src/main.rs

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use finalverse_common::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AIModel {
    name: String,
    model_type: ModelType,
    status: ModelStatus,
    requests_handled: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ModelType {
    NarrativeGeneration,
    DialogueGeneration,
    QuestGeneration,
    WorldDescription,
    CharacterBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ModelStatus {
    Ready,
    Processing,
    Loading,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenerationRequest {
    request_type: String,
    context: serde_json::Value,
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenerationResponse {
    content: String,
    confidence: f32,
    tokens_used: u32,
}

#[derive(Clone)]
struct AIOrchestraState {
    models: Arc<RwLock<std::collections::HashMap<String, AIModel>>>,
    _generation_cache: Arc<RwLock<std::collections::HashMap<String, GenerationResponse>>>,
    total_requests: Arc<RwLock<u64>>,
    start_time: std::time::Instant,
}

impl AIOrchestraState {
    fn new() -> Self {
        let mut models = std::collections::HashMap::new();
        
        // Initialize mock AI models
        models.insert("narrative-gen-v1".to_string(), AIModel {
            name: "narrative-gen-v1".to_string(),
            model_type: ModelType::NarrativeGeneration,
            status: ModelStatus::Ready,
            requests_handled: 0,
        });
        
        models.insert("dialogue-gen-v1".to_string(), AIModel {
            name: "dialogue-gen-v1".to_string(),
            model_type: ModelType::DialogueGeneration,
            status: ModelStatus::Ready,
            requests_handled: 0,
        });
        
        models.insert("quest-gen-v1".to_string(), AIModel {
            name: "quest-gen-v1".to_string(),
            model_type: ModelType::QuestGeneration,
            status: ModelStatus::Ready,
            requests_handled: 0,
        });
        
        Self {
            models: Arc::new(RwLock::new(models)),
            _generation_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            total_requests: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
        }
    }
    
    async fn generate_content(&self, request: GenerationRequest) -> GenerationResponse {
        // Increment request counter
        let mut total = self.total_requests.write().await;
        *total += 1;
        
        // Mock AI generation based on request type
        let (content, confidence) = match request.request_type.as_str() {
            "npc_dialogue" => {
                let npc_name = request.context.get("npc_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let emotion = request.context.get("emotion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("neutral");
                
                let dialogue = match emotion {
                    "happy" => format!("{}: What a wonderful day in the Verse! The Song feels particularly harmonious today.", npc_name),
                    "worried" => format!("{}: I sense disturbances in the Song... The Silence grows stronger.", npc_name),
                    "excited" => format!("{}: Have you heard? A new Songweaver has emerged! Perhaps they can help restore balance.", npc_name),
                    _ => format!("{}: Greetings, traveler. May the Song guide your path.", npc_name),
                };
                
                (dialogue, 0.95)
            }
            
            "quest_generation" => {
                let region = request.context.get("region")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Region");
                let difficulty = request.parameters.get("difficulty")
                    .and_then(|v| v.as_str())
                    .unwrap_or("medium");
                
                let quest = match difficulty {
                    "easy" => format!("Help the Lost Melody: A young musician in {} has lost their melody sprite. Help them find it in the nearby Whispering Grove.", region),
                    "hard" => format!("Silence's Shadow: A powerful manifestation of the Silence threatens {}. Gather allies and confront this dark force before it corrupts the entire region.", region),
                    _ => format!("Restore the Harmony: The harmony levels in {} are dropping. Investigate the cause and perform acts of restoration to heal the land.", region),
                };
                
                (quest, 0.88)
            }
            
            "world_description" => {
                let location = request.context.get("location")
                    .and_then(|v| v.as_str())
                    .unwrap_or("mysterious place");
                
                let description = format!(
                    "You find yourself in {}. The air hums with residual traces of the Song, \
                    creating an ethereal atmosphere. Crystalline formations pulse with soft light, \
                    responding to your presence. In the distance, you can hear the faint echo of \
                    an ancient melody, calling to those who would listen.",
                    location
                );
                
                (description, 0.92)
            }
            
            _ => ("The AI contemplates your request, weaving threads of the Song into coherent thought...".to_string(), 0.75),
        };
        
        // Update model usage
        let mut models = self.models.write().await;
        if let Some(model) = models.values_mut().next() {
            model.requests_handled += 1;
        }
        
        GenerationResponse {
            content,
            confidence,
            tokens_used: (confidence * 500.0) as u32 + 100,
        }
    }
}

// API handlers
async fn get_service_info(State(state): State<AIOrchestraState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        name: "ai-orchestra".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

async fn get_models(State(state): State<AIOrchestraState>) -> Json<serde_json::Value> {
    let models = state.models.read().await;
    let model_list: Vec<_> = models.values().cloned().collect();
    
    Json(serde_json::json!({
        "models": model_list,
        "total_requests": *state.total_requests.read().await,
    }))
}

async fn generate(
    State(state): State<AIOrchestraState>,
    Json(request): Json<GenerationRequest>,
) -> Result<Json<GenerationResponse>, StatusCode> {
    let response = state.generate_content(request).await;
    Ok(Json(response))
}

async fn generate_npc_dialogue(
    State(state): State<AIOrchestraState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let gen_request = GenerationRequest {
        request_type: "npc_dialogue".to_string(),
        context: request.get("context").cloned().unwrap_or(serde_json::json!({})),
        parameters: request.get("parameters").cloned().unwrap_or(serde_json::json!({})),
    };
    
    let response = state.generate_content(gen_request).await;
    
    Ok(Json(serde_json::json!({
        "dialogue": response.content,
        "confidence": response.confidence,
        "emotion_detected": request.get("context")
            .and_then(|c| c.get("emotion"))
            .and_then(|e| e.as_str())
            .unwrap_or("neutral"),
    })))
}

async fn generate_quest(
    State(state): State<AIOrchestraState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let gen_request = GenerationRequest {
        request_type: "quest_generation".to_string(),
        context: request.get("context").cloned().unwrap_or(serde_json::json!({})),
        parameters: request.get("parameters").cloned().unwrap_or(serde_json::json!({})),
    };
    
    let response = state.generate_content(gen_request).await;
    
    Ok(Json(serde_json::json!({
        "quest": {
            "title": "Generated Quest",
            "description": response.content,
            "difficulty": request.get("parameters")
                .and_then(|p| p.get("difficulty"))
                .and_then(|d| d.as_str())
                .unwrap_or("medium"),
            "rewards": {
                "resonance": {
                    "creative": 20,
                    "exploration": 15,
                    "restoration": 10,
                }
            }
        },
        "confidence": response.confidence,
    })))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting AI Orchestra Service...");
    
    let state = AIOrchestraState::new();
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/models", get(get_models))
        .route("/generate", post(generate))
        .route("/npc/dialogue", post(generate_npc_dialogue))
        .route("/quest/generate", post(generate_quest))
        .with_state(state);
    
    let addr = "0.0.0.0:3004";
    info!("AI Orchestra listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}