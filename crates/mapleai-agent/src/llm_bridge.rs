use std::sync::Arc;
use async_trait::async_trait;
use ai_orchestra::{LLMOrchestra, GenerationRequest, GenerationResponse};

#[async_trait]
pub trait LLMEngine: Send + Sync {
    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl LLMEngine for LLMOrchestra {
    async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.generate(request).await
    }
}

#[derive(Clone)]
pub struct LLMBridge {
    engine: Arc<dyn LLMEngine>
}

impl LLMBridge {
    pub fn new() -> Self {
        Self { engine: Arc::new(LLMOrchestra::new()) }
    }

    pub fn with_engine(engine: Arc<dyn LLMEngine>) -> Self {
        Self { engine }
    }

    pub async fn reason(&self, state: &finalverse_protocol::AgentState) -> String {
        let request = GenerationRequest {
            prompt: format!("What should agent {} do next?", state.id),
            context: None,
            player_id: None,
            temperature: Some(0.5),
            max_tokens: Some(32),
        };
        match self.engine.generate(request).await {
            Ok(res) => res.text,
            Err(_) => "".to_string(),
        }
    }
}
