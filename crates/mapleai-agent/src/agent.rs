use crate::{planner::Planner, llm_bridge::LLMBridge};
use finalverse_protocol::{AgentState, ReasoningContext, BehaviorAction};
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct Agent {
    state: AgentState,
    planner: Planner,
    bridge: LLMBridge,
}

pub struct AgentHandle {
    handle: JoinHandle<()>,
}

impl Agent {
    pub fn new(id: String, region: String) -> Self {
        Self {
            state: AgentState {
                id,
                current_region: region,
                last_action: None,
                context: ReasoningContext {
                    location: String::new(),
                    nearby_entities: vec![],
                    harmony_level: 0.5,
                    tension: 0.0,
                    memory: vec![],
                },
            },
            planner: Planner::default(),
            bridge: LLMBridge::new(),
        }
    }

    pub fn state(&self) -> &AgentState {
        &self.state
    }

    pub fn update_context(&mut self, ctx: ReasoningContext) {
        self.state.context = ctx;
    }

    pub async fn step(&mut self) {
        let action = self.planner.plan(&self.state.context);
        self.state.last_action = Some(action);
        let reasoning = self.bridge.reason(&self.state).await;
        self.state.context.memory.push(reasoning);
    }

    pub fn spawn(mut self) -> AgentHandle {
        let handle = tokio::spawn(async move {
            loop {
                self.step().await;
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        });
        AgentHandle { handle }
    }
}

impl AgentHandle {
    pub fn stop(self) {
        self.handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_bridge::LLMEngine;
    use ai_orchestra::{GenerationRequest, GenerationResponse};
    use std::sync::Arc;

    struct MockLLM;
    #[async_trait::async_trait]
    impl LLMEngine for MockLLM {
        async fn generate(&self, _request: GenerationRequest) -> Result<GenerationResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(GenerationResponse { text: "ok".into(), model_used: "mock".into(), tokens_used: 1 })
        }
    }

    #[tokio::test]
    async fn test_agent_step_updates_action() {
        let engine = Arc::new(MockLLM);
        let mut agent = Agent {
            state: AgentState {
                id: "a".into(),
                current_region: "r".into(),
                last_action: None,
                context: ReasoningContext { location: String::new(), nearby_entities: vec![], harmony_level: 1.0, tension: 0.0, memory: vec![] },
            },
            planner: Planner::default(),
            bridge: LLMBridge::with_engine(engine),
        };

        agent.step().await;
        assert!(agent.state.last_action.is_some());
        assert_eq!(agent.state.context.memory.len(), 1);
    }

    #[tokio::test]
    async fn test_update_context() {
        let mut agent = Agent::new("b".into(), "r".into());
        let ctx = ReasoningContext { location: "loc".into(), nearby_entities: vec![], harmony_level: 0.1, tension: 0.8, memory: vec![] };
        agent.update_context(ctx.clone());
        assert_eq!(agent.state.context.location, "loc");
        assert!((agent.state.context.harmony_level - 0.1).abs() < f32::EPSILON);
        assert!((agent.state.context.tension - 0.8).abs() < f32::EPSILON);
    }
}
