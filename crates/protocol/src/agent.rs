#[derive(Debug, Clone)]
pub struct AgentState {
    pub id: String,
    pub current_region: String,
    pub last_action: Option<super::BehaviorAction>,
    pub context: super::ReasoningContext,
}
