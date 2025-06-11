#[derive(Debug, Clone)]
pub struct ReasoningContext {
    pub location: String,
    pub nearby_entities: Vec<String>,
    pub harmony_level: f32,
    pub tension: f32,
    pub memory: Vec<String>,
}
