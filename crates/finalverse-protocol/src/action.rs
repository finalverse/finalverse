#[derive(Debug, Clone)]
pub enum BehaviorAction {
    Wander,
    Rest,
    Flee(String),
    Migrate { target_region: String },
    Interact { entity_id: String, action: String },
}
