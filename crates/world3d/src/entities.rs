// crates/world3d/src/entities.rs
use crate::{EntityId, Position3D};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub enum Entity {
    Player(PlayerEntity),
    NPC(crate::interactive_objects::NPCEntity),
    Echo(crate::echo_entities::EchoEntity),
    Interactive(crate::interactive_objects::InteractiveObject),
    Creature(CreatureEntity),
}

impl Entity {
    pub fn get_id(&self) -> EntityId {
        match self {
            Entity::Player(e) => e.id,
            Entity::NPC(e) => e.id,
            Entity::Echo(e) => e.id,
            Entity::Interactive(e) => e.id,
            Entity::Creature(e) => e.id,
        }
    }

    pub fn get_position(&self) -> Position3D {
        match self {
            Entity::Player(e) => e.position,
            Entity::NPC(e) => e.position,
            Entity::Echo(e) => e.position,
            Entity::Interactive(e) => e.position,
            Entity::Creature(e) => e.position,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerEntity {
    pub id: EntityId,
    pub name: String,
    pub position: Position3D,
    pub resonance: ResonanceScore,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatureEntity {
    pub id: EntityId,
    pub creature_type: String,
    pub position: Position3D,
    pub behavior_state: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResonanceScore {
    pub creative: f32,
    pub exploration: f32,
    pub restoration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub model_id: String,
    pub materials: Vec<Material>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub shader: String,
    pub properties: HashMap<String, Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub loop_mode: AnimationLoop,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationLoop {
    Once,
    Loop,
    PingPong,
}