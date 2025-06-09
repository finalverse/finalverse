use finalverse_common::types::PlayerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LLMOrchestra {
    models: HashMap<String, LLMProvider>,
    default_model: String,
}

#[derive(Debug, Clone)]
pub enum LLMProvider {
    Ollama(OllamaProvider),
    OpenAI(OpenAIProvider),
    Local(LocalProvider),
}

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    base_url: String,
    model_name: String,
}

#[derive(Debug, Clone)]
pub struct OpenAIProvider {
    base_url: String,
    api_key: String,
    model_name: String,
}

#[derive(Debug, Clone)]
pub struct LocalProvider {
    model_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub prompt: String,
    pub context: Option<String>,
    pub player_id: Option<PlayerId>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub text: String,
    pub model_used: String,
    pub tokens_used: u32,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    #[serde(rename = "num_predict")]
    max_tokens: u32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    total_tokens: u32,
}

impl LLMOrchestra {
    pub fn new() -> Self {
        let mut models = HashMap::new();
        
        // Add default Ollama provider
        models.insert(
            "ollama".to_string(),
            LLMProvider::Ollama(OllamaProvider {
                base_url: "http://localhost:11434".to_string(),
                model_name: "llama2".to_string(),
            }),
        );

        Self {
            models,
            default_model: "ollama".to_string(),
        }
    }

    pub fn add_provider(&mut self, name: String, provider: LLMProvider) {
        self.models.insert(name, provider);
    }

    pub async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>> {
        let provider = self.models.get(&self.default_model)
            .ok_or("Default model not found")?;

        match provider {
            LLMProvider::Ollama(ollama) => self.generate_ollama(ollama, request).await,
            LLMProvider::OpenAI(openai) => self.generate_openai(openai, request).await,
            LLMProvider::Local(_local) => {
                // TODO: Implement local model generation
                Err("Local model generation not implemented yet".into())
            }
        }
    }

    async fn generate_ollama(
        &self,
        provider: &OllamaProvider,
        request: GenerationRequest,
    ) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        
        let ollama_request = OllamaRequest {
            model: provider.model_name.clone(),
            prompt: request.prompt,
            stream: false,
            options: OllamaOptions {
                temperature: request.temperature.unwrap_or(0.7),
                max_tokens: request.max_tokens.unwrap_or(2048),
            },
        };

        let response = client
            .post(&format!("{}/api/generate", provider.base_url))
            .json(&ollama_request)
            .send()
            .await?;

        if response.status().is_success() {
            let ollama_response: OllamaResponse = response.json().await?;
            Ok(GenerationResponse {
                text: ollama_response.response,
                model_used: provider.model_name.clone(),
                tokens_used: 0, // Ollama doesn't return token count in this format
            })
        } else {
            Err(format!("Ollama request failed with status: {}", response.status()).into())
        }
    }

    async fn generate_openai(
        &self,
        provider: &OpenAIProvider,
        request: GenerationRequest,
    ) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        
        let messages = vec![OpenAIMessage {
            role: "user".to_string(),
            content: request.prompt,
        }];

        let openai_request = OpenAIRequest {
            model: provider.model_name.clone(),
            messages,
            temperature: request.temperature.unwrap_or(0.7),
            max_tokens: request.max_tokens.unwrap_or(2048),
        };

        let response = client
            .post(&format!("{}/v1/chat/completions", provider.base_url))
            .header("Authorization", format!("Bearer {}", provider.api_key))
            .json(&openai_request)
            .send()
            .await?;

        if response.status().is_success() {
            let openai_response: OpenAIResponse = response.json().await?;
            
            if let Some(choice) = openai_response.choices.first() {
                Ok(GenerationResponse {
                    text: choice.message.content.clone(),
                    model_used: provider.model_name.clone(),
                    tokens_used: openai_response.usage.total_tokens,
                })
            } else {
                Err("No choices returned from OpenAI".into())
            }
        } else {
            Err(format!("OpenAI request failed with status: {}", response.status()).into())
        }
    }
}

// Narrative AI functions
pub async fn generate_quest_narrative(
    orchestra: &LLMOrchestra,
    player_context: &str,
    world_state: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let prompt = format!(
        "Generate a quest narrative for Finalverse based on the following context:\n\
        Player Context: {}\n\
        World State: {}\n\n\
        The quest should involve the Song of Creation and align with the principles of \
        Symbiotic Creation, Empathetic Exploration, or Living Wonder. \
        Keep it engaging and age-appropriate.",
        player_context, world_state
    );

    let request = GenerationRequest {
        prompt,
        context: None,
        player_id: None,
        temperature: Some(0.8),
        max_tokens: Some(1024),
    };

    let response = orchestra.generate(request).await?;
    Ok(response.text)
}

pub async fn generate_npc_dialogue(
    orchestra: &LLMOrchestra,
    npc_personality: &str,
    conversation_context: &str,
    player_history: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let prompt = format!(
        "Generate dialogue for an NPC in Finalverse with the following personality: {}\n\
        Conversation Context: {}\n\
        Player History: {}\n\n\
        The dialogue should be consistent with the character's personality and \
        acknowledge the player's past actions. Keep it natural and engaging.",
        npc_personality, conversation_context, player_history
    );

    let request = GenerationRequest {
        prompt,
        context: None,
        player_id: None,
        temperature: Some(0.7),
        max_tokens: Some(512),
    };

    let response = orchestra.generate(request).await?;
    Ok(response.text)
}

pub async fn generate_world_description(
    orchestra: &LLMOrchestra,
    region_name: &str,
    harmony_level: f32,
    time_of_day: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let harmony_description = if harmony_level > 0.8 {
        "high harmony with vibrant colors and flourishing life"
    } else if harmony_level > 0.5 {
        "moderate harmony with gentle signs of the Song's presence"
    } else if harmony_level > 0.2 {
        "low harmony with muted colors and signs of the Silence's influence"
    } else {
        "very low harmony with corruption and decay from the Silence"
    };

    let prompt = format!(
        "Describe the region '{}' in Finalverse during {} with {}. \
        The description should capture the visual beauty or corruption, \
        the sounds of the Song or Silence, and the overall atmosphere. \
        Make it immersive and poetic, suitable for all ages.",
        region_name, time_of_day, harmony_description
    );

    let request = GenerationRequest {
        prompt,
        context: None,
        player_id: None,
        temperature: Some(0.9),
        max_tokens: Some(768),
    };

    let response = orchestra.generate(request).await?;
    Ok(response.text)
}