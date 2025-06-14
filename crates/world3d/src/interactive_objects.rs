use crate::{EntityId, Position3D, GridCoordinate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveObject {
    pub id: EntityId,
    pub object_type: ObjectType,
    pub position: Position3D,
    pub state: ObjectState,
    pub interaction_range: f32,
    pub required_harmony: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    AnyaStatue,
    ResonantBlossom,
    MemoryCrystal,
    HarmonyFountain,
    StoryStone,
    GloomShade,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectState {
    Pristine,
    Faded,
    Corrupted,
    Dormant,
    Active,
    Restored,
}

impl InteractiveObject {
    pub fn create_anya_statue(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            object_type: ObjectType::AnyaStatue,
            position,
            state: ObjectState::Faded,
            interaction_range: 5.0,
            required_harmony: Some(0.3),
        }
    }

    pub fn create_resonant_blossom(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            object_type: ObjectType::ResonantBlossom,
            position,
            state: ObjectState::Dormant,
            interaction_range: 3.0,
            required_harmony: Some(0.2),
        }
    }

    pub fn create_memory_crystal(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            object_type: ObjectType::MemoryCrystal,
            position,
            state: ObjectState::Active,
            interaction_range: 2.0,
            required_harmony: None,
        }
    }

    pub fn create_gloom_shade(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            object_type: ObjectType::GloomShade,
            position,
            state: ObjectState::Corrupted,
            interaction_range: 10.0,
            required_harmony: Some(0.5),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCEntity {
    pub id: EntityId,
    pub name: String,
    pub position: Position3D,
    pub dialogue_state: String,
    pub emotion: EmotionalState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalState {
    Neutral,
    Happy,
    Sad,
    Worried,
    Inspired,
    Fearful,
}

impl NPCEntity {
    pub fn create_anya(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            name: "Anya".to_string(),
            position,
            dialogue_state: "initial_sadness".to_string(),
            emotion: EmotionalState::Sad,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NPCState {
    InitialSadness,
    Neutral,
}

pub struct InteractiveObjectManager;

impl InteractiveObjectManager {
    pub fn new() -> Self { Self }

    pub async fn spawn_memory_crystal(&self, _grid: GridCoordinate, _pos: Position3D) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn spawn_npc(&self, _grid: GridCoordinate, _name: &str, _pos: Position3D, _state: NPCState) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn spawn_interactive(&self, _grid: GridCoordinate, _typ: ObjectType, _pos: Position3D, _state: ObjectState) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveObject3D {
    pub id: &'static str,
    pub position: Position3D,
    pub mesh: &'static str,
    pub interaction_type: InteractionType,
    pub current_state: ObjectState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Harmony,
    Songweave,
}
