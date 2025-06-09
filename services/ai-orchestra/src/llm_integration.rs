// services/ai-orchestra/src/llm_integration.rs
// Real LLM integration module

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub context: HashMap<String, serde_json::Value>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub tokens_used: u32,
    pub model: String,
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse, String>;
    fn name(&self) -> &str;
}

// Local LLM provider using Ollama
pub struct OllamaProvider {
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: &str) -> Self {
        Self {
            base_url: std::env::var("OLLAMA_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse, String> {
        let client = reqwest::Client::new();
        
        let ollama_request = serde_json::json!({
            "model": self.model,
            "prompt": request.prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.7),
                "num_predict": request.max_tokens.unwrap_or(500),
            }
        });
        
        let response = client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&ollama_request)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Ollama returned error: {}", response.status()));
        }
        
        let ollama_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;
        
        Ok(LLMResponse {
            content: ollama_response["response"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            tokens_used: ollama_response["total_duration"]
                .as_u64()
                .unwrap_or(0) as u32 / 1000000, // Convert nanoseconds to rough token estimate
            model: self.model.clone(),
        })
    }
    
    fn name(&self) -> &str {
        "Ollama"
    }
}

// OpenAI-compatible provider (works with OpenAI, Claude, etc.)
pub struct OpenAIProvider {
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse, String> {
        let client = reqwest::Client::new();
        
        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are an AI assistant in the magical world of Finalverse, where the Song of Creation shapes reality. You help create immersive, story-driven content."
            }),
            serde_json::json!({
                "role": "user",
                "content": request.prompt
            }),
        ];
        
        let openai_request = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(500),
        });
        
        let response = client
            .post(&format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&openai_request)
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("OpenAI returned error: {}", response.status()));
        }
        
        let openai_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
        
        Ok(LLMResponse {
            content: openai_response["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            tokens_used: openai_response["usage"]["total_tokens"]
                .as_u64()
                .unwrap_or(0) as u32,
            model: self.model.clone(),
        })
    }
    
    fn name(&self) -> &str {
        "OpenAI"
    }
}

// LLM Manager to handle multiple providers
pub struct LLMManager {
    providers: HashMap<String, Box<dyn LLMProvider>>,
    default_provider: String,
}

impl LLMManager {
    pub fn new() -> Self {
        let mut providers: HashMap<String, Box<dyn LLMProvider>> = HashMap::new();
        
        // Add Ollama provider if available
        if std::env::var("ENABLE_OLLAMA").unwrap_or_else(|_| "true".to_string()) == "true" {
            providers.insert(
                "ollama".to_string(),
                Box::new(OllamaProvider::new("llama2")),
            );
        }
        
        // Add OpenAI provider if API key is available
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            providers.insert(
                "openai".to_string(),
                Box::new(OpenAIProvider::new(
                    "https://api.openai.com",
                    &api_key,
                    "gpt-3.5-turbo",
                )),
            );
        }
        
        // Default to mock if no providers available
        if providers.is_empty() {
            providers.insert("mock".to_string(), Box::new(MockProvider));
        }
        
        let default_provider = providers.keys().next().unwrap().clone();
        
        Self {
            providers,
            default_provider,
        }
    }
    
    pub async fn generate(&self, request: LLMRequest) -> Result<LLMResponse, String> {
        self.generate_with_provider(&self.default_provider, request).await
    }
    
    pub async fn generate_with_provider(
        &self,
        provider_name: &str,
        request: LLMRequest,
    ) -> Result<LLMResponse, String> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| format!("Provider '{}' not found", provider_name))?;
        
        provider.generate(request).await
    }
}

// Mock provider for testing
struct MockProvider;

#[async_trait]
impl LLMProvider for MockProvider {
    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse, String> {
        let response = match request.prompt.to_lowercase() {
            p if p.contains("quest") => {
                "Journey to the Crystal Caves: Ancient texts speak of harmonious crystals deep within the Whispering Caverns. Seek them out to restore balance to the troubled waters of Lake Serenity."
            }
            p if p.contains("dialogue") || p.contains("npc") => {
                "Ah, a Songweaver! I can sense the harmony within you. The Silence grows stronger each day, but your presence gives me hope."
            }
            p if p.contains("describe") || p.contains("location") => {
                "The ancient grove stands in eternal twilight, its boughs heavy with crystalline fruit that chime softly in the breeze. The very air here resonates with the Song of Creation."
            }
            p if p.contains("lore") || p.contains("history") => {
                "Long ago, when the First Architects wove the Song of Creation, they imbued every living thing with a unique melody. Those who can hear and shape these melodies are known as Songweavers."
            }
            _ => "The Song of Creation resonates through all things, binding the Verse in harmony.",
        };
        
        Ok(LLMResponse {
            content: response.to_string(),
            tokens_used: response.split_whitespace().count() as u32,
            model: "mock".to_string(),
        })
    }
    
    fn name(&self) -> &str {
        "Mock"
    }
}

// Content generation functions using LLM
pub async fn generate_npc_dialogue(
    llm: &LLMManager,
    npc_name: &str,
    emotion: &str,
    context: &HashMap<String, serde_json::Value>,
) -> Result<String, String> {
    let prompt = format!(
        "Generate dialogue for an NPC named {} in Finalverse. 
        They are feeling {} and speaking to a player.
        Context: The world is threatened by the Silence, and players are Songweavers who can use melodies to restore harmony.
        Additional context: {:?}
        
        Generate a single, immersive line of dialogue that fits their emotion and the world's lore.",
        npc_name, emotion, context
    );
    
    let request = LLMRequest {
        prompt,
        context: context.clone(),
        max_tokens: Some(100),
        temperature: Some(0.8),
    };
    
    let response = llm.generate(request).await?;
    Ok(response.content)
}

pub async fn generate_quest(
    llm: &LLMManager,
    region: &str,
    difficulty: &str,
    player_level: u32,
) -> Result<(String, String), String> {
    let prompt = format!(
        "Create a quest for Finalverse in the {} region.
        Difficulty: {}
        Player Level: {}
        
        The quest should involve the Song of Creation, the threat of the Silence, and be appropriate for the difficulty level.
        
        Format your response as:
        TITLE: [Quest Title]
        DESCRIPTION: [Quest Description in 2-3 sentences]",
        region, difficulty, player_level
    );
    
    let request = LLMRequest {
        prompt,
        context: HashMap::new(),
        max_tokens: Some(200),
        temperature: Some(0.9),
    };
    
    let response = llm.generate(request).await?;
    
    // Parse the response
    let lines: Vec<&str> = response.content.lines().collect();
    let title = lines
        .iter()
        .find(|l| l.starts_with("TITLE:"))
        .map(|l| l.replace("TITLE:", "").trim().to_string())
        .unwrap_or_else(|| "Mystery Quest".to_string());
    
    let description = lines
        .iter()
        .find(|l| l.starts_with("DESCRIPTION:"))
        .map(|l| l.replace("DESCRIPTION:", "").trim().to_string())
        .unwrap_or_else(|| response.content.clone());
    
    Ok((title, description))
}

pub async fn generate_location_description(
    llm: &LLMManager,
    location_type: &str,
    harmony_level: f32,
) -> Result<String, String> {
    let harmony_state = if harmony_level > 80.0 {
        "highly harmonious, thriving with life and positive energy"
    } else if harmony_level > 50.0 {
        "balanced, showing signs of both harmony and discord"
    } else {
        "troubled by the Silence, showing signs of decay and dissonance"
    };
    
    let prompt = format!(
        "Describe a {} location in Finalverse that is {}.
        Keep it atmospheric and mystical, mentioning how the Song of Creation manifests here.
        Maximum 3 sentences.",
        location_type, harmony_state
    );
    
    let request = LLMRequest {
        prompt,
        context: HashMap::new(),
        max_tokens: Some(150),
        temperature: Some(0.7),
    };
    
    let response = llm.generate(request).await?;
    Ok(response.content)
}