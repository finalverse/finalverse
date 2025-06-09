// services/ai-orchestra/src/main.rs - Updated version with real LLM integration

mod llm_integration;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use finalverse_common::*;
use finalverse_protocol::*;
use llm_integration::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    model_used: String,
}

#[derive(Clone)]
struct AIOrchestraState {
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    generation_cache: Arc<RwLock<HashMap<String, GenerationResponse>>>,
    total_requests: Arc<RwLock<u64>>,
    llm_manager: Arc<LLMManager>,
    start_time: std::time::Instant,
}

impl AIOrchestraState {
    fn new() -> Self {
        let mut models = HashMap::new();
        
        // Initialize AI models tracking
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
            generation_cache: Arc::new(RwLock::new(HashMap::new())),
            total_requests: Arc::new(RwLock::new(0)),
            llm_manager: Arc::new(LLMManager::new()),
            start_time: std::time::Instant::now(),
        }
    }
    
    async fn generate_content(&self, request: GenerationRequest) -> GenerationResponse {
        // Increment request counter
        let mut total = self.total_requests.write().await;
        *total += 1;
        
        // Check cache first
        let cache_key = format!("{:?}:{:?}", request.request_type, request.context);
        if let Some(cached) = self.generation_cache.read().await.get(&cache_key) {
            return cached.clone();
        }
        
        // Generate content based on request type
        let result = match request.request_type.as_str() {
            "npc_dialogue" => {
                self.generate_npc_dialogue(request.context, request.parameters).await
            }
            "quest_generation" => {
                self.generate_quest(request.context, request.parameters).await
            }
            "world_description" => {
                self.generate_world_description(request.context, request.parameters).await
            }
            "item_lore" => {
                self.generate_item_lore(request.context, request.parameters).await
            }
            _ => {
                GenerationResponse {
                    content: "Unknown generation type".to_string(),
                    confidence: 0.0,
                    tokens_used: 0,
                    model_used: "none".to_string(),
                }
            }
        };
        
        // Cache the result
        self.generation_cache.write().await.insert(cache_key, result.clone());
        
        // Update model usage
        let mut models = self.models.write().await;
        if let Some(model) = models.values_mut().next() {
            model.requests_handled += 1;
        }
        
        result
    }
    
    async fn generate_npc_dialogue(
        &self,
        context: serde_json::Value,
        _parameters: serde_json::Value,
    ) -> GenerationResponse {
        let npc_name = context.get("npc_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        let emotion = context.get("emotion")
            .and_then(|v| v.as_str())
            .unwrap_or("neutral");
        
        let mut context_map = HashMap::new();
        if let Some(player_name) = context.get("player_name") {
            context_map.insert("player_name".to_string(), player_name.clone());
        }
        if let Some(location) = context.get("location") {
            context_map.insert("location".to_string(), location.clone());
        }
        
        match generate_npc_dialogue(&self.llm_manager, npc_name, emotion, &context_map).await {
            Ok(content) => GenerationResponse {
                content,
                confidence: 0.95,
                tokens_used: 50, // Estimate
                model_used: "llm".to_string(),
            },
            Err(_) => {
                // Fallback to simple generation
                let dialogue = match emotion {
                    "happy" => format!("{}: What a wonderful day in the Verse! The Song feels particularly harmonious today.", npc_name),
                    "worried" => format!("{}: I sense disturbances in the Song... The Silence grows stronger.", npc_name),
                    "excited" => format!("{}: Have you heard? A new Songweaver has emerged! Perhaps they can help restore balance.", npc_name),
                    _ => format!("{}: Greetings, traveler. May the Song guide your path.", npc_name),
                };
                
                GenerationResponse {
                    content: dialogue,
                    confidence: 0.8,
                    tokens_used: 30,
                    model_used: "fallback".to_string(),
                }
            }
        }
    }
    
    async fn generate_quest(
        &self,
        context: serde_json::Value,
        parameters: serde_json::Value,
    ) -> GenerationResponse {
        let region = context.get("region")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Region");
        let difficulty = parameters.get("difficulty")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let player_level = parameters.get("player_level")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;
        
        match generate_quest(&self.llm_manager, region, difficulty, player_level).await {
            Ok((title, description)) => {
                let quest_json = serde_json::json!({
                    "title": title,
                    "description": description,
                    "objectives": [
                        {
                            "description": "Begin your journey",
                            "completed": false
                        }
                    ],
                    "rewards": {
                        "resonance": {
                            "creative": 20 * player_level / 10,
                            "exploration": 15 * player_level / 10,
                            "restoration": 10 * player_level / 10,
                        }
                    }
                });
                
                GenerationResponse {
                    content: quest_json.to_string(),
                    confidence: 0.9,
                    tokens_used: 100,
                    model_used: "llm".to_string(),
                }
            }
            Err(_) => {
                // Fallback quest generation
                let quest = match difficulty {
                    "easy" => format!("Help the Lost Melody: A young musician in {} has lost their melody sprite.", region),
                    "hard" => format!("Silence's Shadow: A powerful manifestation threatens {}.", region),
                    _ => format!("Restore the Harmony: The harmony levels in {} are dropping.", region),
                };
                
                GenerationResponse {
                    content: quest,
                    confidence: 0.7,
                    tokens_used: 40,
                    model_used: "fallback".to_string(),
                }
            }
        }
    }
    
    async fn generate_world_description(
        &self,
        context: serde_json::Value,
        _parameters: serde_json::Value,
    ) -> GenerationResponse {
        let location_type = context.get("location_type")
            .and_then(|v| v.as_str())
            .unwrap_or("mysterious place");
        let harmony_level = context.get("harmony_level")
            .and_then(|v| v.as_f64())
            .unwrap_or(50.0) as f32;
        
        match generate_location_description(&self.llm_manager, location_type, harmony_level).await {
            Ok(description) => GenerationResponse {
                content: description,
                confidence: 0.92,
                tokens_used: 80,
                model_used: "llm".to_string(),
            },
            Err(_) => {
                let description = format!(
                    "You find yourself in a {}. The air hums with residual traces of the Song, creating an ethereal atmosphere.",
                    location_type
                );
                
                GenerationResponse {
                    content: description,
                    confidence: 0.75,
                    tokens_used: 30,
                    model_used: "fallback".to_string(),
                }
            }
        }
    }
    
    async fn generate_item_lore(
        &self,
        context: serde_json::Value,
        _parameters: serde_json::Value,
    ) -> GenerationResponse {
        let item_name = context.get("item_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Ancient Artifact");
        let item_type = context.get("item_type")
            .and_then(|v| v.as_str())
            .unwrap_or("mysterious");
        
        let prompt = format!(
            "Create a short lore description for an item called '{}' in Finalverse. 
            It is a {} item connected to the Song of Creation.
            Maximum 2 sentences.",
            item_name, item_type
        );
        
        let llm_request = LLMRequest {
            prompt,
            context: HashMap::new(),
            max_tokens: Some(100),
            temperature: Some(0.8),
        };
        
        match self.llm_manager.generate(llm_request).await {
            Ok(response) => GenerationResponse {
                content: response.content,
                confidence: 0.9,
                tokens_used: response.tokens_used,
                model_used: response.model,
            },
            Err(_) => GenerationResponse {
                content: format!("The {} resonates faintly with the Song of Creation, its true purpose lost to time.", item_name),
                confidence: 0.6,
                tokens_used: 20,
                model_used: "fallback".to_string(),
            },
        }
    }
}

// API handlers remain the same...
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
        "llm_provider": "LLM Manager",
    }))
}

async fn generate(
    State(state): State<AIOrchestraState>,
    Json(request): Json<GenerationRequest>,
) -> Result<Json<GenerationResponse>, StatusCode> {
    let response = state.generate_content(request).await;
    Ok(Json(response))
}

async fn generate_npc_dialogue_endpoint(
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
        "model_used": response.model_used,
        "tokens_used": response.tokens_used,
    })))
}

async fn generate_quest_endpoint(
    State(state): State<AIOrchestraState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let gen_request = GenerationRequest {
        request_type: "quest_generation".to_string(),
        context: request.get("context").cloned().unwrap_or(serde_json::json!({})),
        parameters: request.get("parameters").cloned().unwrap_or(serde_json::json!({})),
    };
    
    let response = state.generate_content(gen_request).await;
    
    // Parse the response if it's JSON
    if let Ok(quest_data) = serde_json::from_str::<serde_json::Value>(&response.content) {
        Ok(Json(serde_json::json!({
            "quest": quest_data,
            "confidence": response.confidence,
            "model_used": response.model_used,
        })))
    } else {
        // Fallback for non-JSON responses
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
            "model_used": response.model_used,
        })))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting AI Orchestra Service with LLM Integration...");
    
    let state = AIOrchestraState::new();
    
    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/info", get(get_service_info))
        .route("/models", get(get_models))
        .route("/generate", post(generate))
        .route("/npc/dialogue", post(generate_npc_dialogue_endpoint))
        .route("/quest/generate", post(generate_quest_endpoint))
        .with_state(state);
    
    let addr = "0.0.0.0:3004";
    info!("AI Orchestra listening on {}", addr);
    info!("LLM Provider: {}", if std::env::var("OPENAI_API_KEY").is_ok() { "OpenAI" } else { "Ollama/Mock" });
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}