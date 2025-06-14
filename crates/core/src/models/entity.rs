// File: crates/core/src/models/entity.rs
// Path: finalverse/crates/core/src/models/entity.rs
// Description: Entity system models for all game objects including players, NPCs, and items.
//              Implements a flexible component-based entity system.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use nalgebra::{Vector3, Quaternion};
use std::collections::HashMap;

/// Base entity that can exist in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier
    pub id: Uuid,

    /// Entity type determines behavior and capabilities
    pub entity_type: EntityType,

    /// Current position in world space
    pub transform: Transform,

    /// Visual representation
    pub appearance: Appearance,

    /// Component data for ECS pattern
    pub components: HashMap<String, ComponentData>,

    /// Current state flags
    pub state: EntityState,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
}

/// Transform component for spatial data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

/// Visual appearance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub mesh_id: String,
    pub material_id: String,
    pub animations: Vec<String>,
    pub current_animation: Option<String>,
    pub custom_colors: HashMap<String, Color>,
}

/// Entity type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Player(PlayerData),
    NPC(NPCData),
    Creature(CreatureData),
    Item(ItemData),
    Environmental(EnvironmentalData),
    Echo(EchoData), // Special type for First Echoes
}

/// Player-specific data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerData {
    pub account_id: Uuid,
    pub character_name: String,
    pub resonance: ResonanceData,
    pub inventory: Inventory,
    pub skills: SkillSet,
    pub chronicle: PlayerChronicle,
}

/// Player's resonance (progression) data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResonanceData {
    /// Creative activities (building, crafting, art)
    pub creative_resonance: f32,

    /// Exploration and discovery
    pub exploration_resonance: f32,

    /// Healing and restoration activities
    pub restoration_resonance: f32,

    /// Total resonance level
    pub total_resonance: f32,

    /// Tier of attunement (progression milestones)
    pub attunement_tier: u32,
}

/// Player's personal story record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerChronicle {
    /// Major accomplishments
    pub legends: Vec<Legend>,

    /// Relationships with NPCs and Echoes
    pub relationships: HashMap<Uuid, Relationship>,

    /// Places discovered
    pub discovered_locations: Vec<DiscoveredLocation>,

    /// Current active quests
    pub active_quests: Vec<QuestProgress>,

    /// Choices that affected the world
    pub world_impacts: Vec<WorldImpact>,
}

/// A legendary deed recorded in player's chronicle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Legend {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub achieved_at: DateTime<Utc>,
    pub witnesses: Vec<Uuid>, // NPCs or players who witnessed
    pub world_effect: Option<String>,
}

/// NPC-specific data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NPCData {
    pub npc_id: String,
    pub display_name: String,
    pub personality: PersonalityProfile,
    pub dialogue_state: DialogueState,
    pub memory: NPCMemory,
    pub behavior_tree: String, // Reference to behavior definition
}

/// NPC personality configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonalityProfile {
    pub traits: Vec<String>,
    pub voice_style: String,
    pub emotional_baseline: EmotionalState,
    pub interests: Vec<String>,
    pub fears: Vec<String>,
}

/// NPC memory system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NPCMemory {
    /// Memories of specific players
    pub player_interactions: HashMap<Uuid, Vec<InteractionMemory>>,

    /// General event memories
    pub event_memories: Vec<EventMemory>,

    /// Current goals and motivations
    pub active_goals: Vec<NPCGoal>,

    /// Emotional reactions to recent events
    pub emotional_memory: Vec<EmotionalMemory>,
}

/// State flags for entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub active: bool,
    pub visible: bool,
    pub interactable: bool,
    pub physics_enabled: bool,
    pub ai_enabled: bool,
    pub custom_flags: HashMap<String, bool>,
}

/// Generic component data for ECS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentData {
    Health(HealthComponent),
    Combat(CombatComponent),
    Dialogue(DialogueComponent),
    Merchant(MerchantComponent),
    Songweaver(SongweaverComponent),
    Custom(serde_json::Value),
}

/// Health component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthComponent {
    pub current: f32,
    pub maximum: f32,
    pub regeneration_rate: f32,
    pub is_essential: bool, // Cannot die
}

/// Songweaving abilities component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongweaverComponent {
    pub known_melodies: Vec<String>,
    pub active_harmonies: Vec<ActiveHarmony>,
    pub song_power: f32,
    pub instrument_bonuses: HashMap<String, f32>,
}

impl Entity {
    /// Create a new entity with basic components
    pub fn new(entity_type: EntityType, position: Vector3<f32>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            entity_type,
            transform: Transform {
                position,
                rotation: Quaternion::identity(),
                scale: Vector3::new(1.0, 1.0, 1.0),
                velocity: Vector3::zeros(),
            },
            appearance: Appearance::default(),
            components: HashMap::new(),
            state: EntityState {
                active: true,
                visible: true,
                interactable: true,
                physics_enabled: true,
                ai_enabled: matches!(entity_type, EntityType::NPC(_) | EntityType::Creature(_)),
                custom_flags: HashMap::new(),
            },
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a component to this entity
    pub fn add_component(&mut self, name: String, component: ComponentData) {
        self.components.insert(name, component);
        self.updated_at = Utc::now();
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&ComponentData> {
        self.components.get(name)
    }

    /// Check if entity has a specific component
    pub fn has_component(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            mesh_id: "default_cube".to_string(),
            material_id: "default_material".to_string(),
            animations: Vec::new(),
            current_animation: None,
            custom_colors: HashMap::new(),
        }
    }
}