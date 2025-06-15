// crates/core/src/echo.rs
use crate::types::EchoType;
use crate::types::Coordinates as Position;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core Echo structure representing the First Echoes and their manifestations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Echo {
    pub id: Uuid,
    pub echo_type: EchoType,
    pub name: String,
    pub position: Position,
    pub state: EchoState,
    pub personality: EchoPersonality,
    pub abilities: Vec<EchoAbility>,
    pub bond_levels: HashMap<Uuid, f32>, // Player ID -> Bond Level
    pub memory: EchoMemory,
    pub visual_state: VisualState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoState {
    pub energy_level: f32,
    pub emotional_state: EmotionalState,
    pub manifestation_strength: f32,
    pub current_activity: EchoActivity,
    pub resonance_frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalState {
    Joyful,
    Contemplative,
    Concerned,
    Determined,
    Protective,
    Curious,
    Melancholic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EchoActivity {
    Idle,
    Guiding { target_player: Uuid },
    Teaching { skill: String },
    Investigating { phenomenon: String },
    Defending { threat_level: f32 },
    Conversing { with: Uuid },
    Meditating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoPersonality {
    pub core_traits: Vec<String>,
    pub speaking_patterns: SpeakingPattern,
    pub preferred_teaching_style: TeachingStyle,
    pub emotional_responses: HashMap<String, EmotionalResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakingPattern {
    pub tone: String,
    pub vocabulary_complexity: f32,
    pub metaphor_usage: f32,
    pub characteristic_phrases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeachingStyle {
    DirectInstruction,
    SocraticMethod,
    ExperientialLearning,
    Storytelling,
    Demonstration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalResponse {
    pub trigger: String,
    pub reaction: EmotionalState,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoAbility {
    pub name: String,
    pub description: String,
    pub energy_cost: f32,
    pub cooldown: f32,
    pub effect_type: AbilityEffect,
    pub teaching_requirements: TeachingRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityEffect {
    Healing { potency: f32 },
    Protection { duration: f32, strength: f32 },
    Revelation { range: f32, clarity: f32 },
    Inspiration { targets: u32, boost: f32 },
    Transformation { scope: String },
    Creation { complexity: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingRequirements {
    pub bond_level: f32,
    pub player_resonance: f32,
    pub prerequisites: Vec<String>,
    pub teaching_duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoMemory {
    pub significant_events: Vec<MemoryEvent>,
    pub player_interactions: HashMap<Uuid, Vec<InteractionRecord>>,
    pub world_observations: Vec<WorldObservation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub timestamp: i64,
    pub event_type: String,
    pub participants: Vec<Uuid>,
    pub emotional_impact: f32,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub timestamp: i64,
    pub interaction_type: InteractionType,
    pub outcome: InteractionOutcome,
    pub bond_change: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Conversation,
    Teaching,
    QuestGuidance,
    EmotionalSupport,
    CombatAssistance,
    Exploration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionOutcome {
    Positive { description: String },
    Neutral { description: String },
    Negative { description: String },
    Transformative { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldObservation {
    pub timestamp: i64,
    pub location: Position,
    pub phenomenon: String,
    pub significance: f32,
    pub related_to_silence: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualState {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub glow_intensity: f32,
    pub particle_effects: Vec<ParticleEffect>,
    pub form_stability: f32, // How solid vs ethereal they appear
    pub size_modifier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEffect {
    pub effect_type: String,
    pub intensity: f32,
    pub color: Color,
}

impl Echo {
    pub fn new(echo_type: EchoType, name: String, position: Position) -> Self {
        let (personality, abilities, visual_state) = match echo_type {
            EchoType::Lumi => Self::create_lumi_traits(),
            EchoType::KAI => Self::create_kai_traits(),
            EchoType::Terra => Self::create_terra_traits(),
            EchoType::Ignis => Self::create_ignis_traits(),
        };

        Echo {
            id: Uuid::new_v4(),
            echo_type,
            name,
            position,
            state: EchoState {
                energy_level: 1.0,
                emotional_state: EmotionalState::Contemplative,
                manifestation_strength: 1.0,
                current_activity: EchoActivity::Idle,
                resonance_frequency: match echo_type {
                    EchoType::Lumi => 528.0,  // Love frequency
                    EchoType::KAI => 741.0,   // Awakening frequency
                    EchoType::Terra => 396.0, // Liberation frequency
                    EchoType::Ignis => 852.0, // Intuition frequency
                },
            },
            personality,
            abilities,
            bond_levels: HashMap::new(),
            memory: EchoMemory {
                significant_events: Vec::new(),
                player_interactions: HashMap::new(),
                world_observations: Vec::new(),
            },
            visual_state,
        }
    }

    fn create_lumi_traits() -> (EchoPersonality, Vec<EchoAbility>, VisualState) {
        let personality = EchoPersonality {
            core_traits: vec![
                "Hopeful".to_string(),
                "Curious".to_string(),
                "Childlike Wonder".to_string(),
                "Empathetic".to_string(),
                "Playful".to_string(),
            ],
            speaking_patterns: SpeakingPattern {
                tone: "Bright and melodic".to_string(),
                vocabulary_complexity: 0.6,
                metaphor_usage: 0.8,
                characteristic_phrases: vec![
                    "Do you see how it shimmers?".to_string(),
                    "There's always light, even in darkness!".to_string(),
                    "Let's discover together!".to_string(),
                    "Oh! What's that over there?".to_string(),
                ],
            },
            preferred_teaching_style: TeachingStyle::ExperientialLearning,
            emotional_responses: HashMap::new(),
        };

        let abilities = vec![
            EchoAbility {
                name: "Guiding Light".to_string(),
                description: "Reveals hidden paths and secrets".to_string(),
                energy_cost: 0.2,
                cooldown: 5.0,
                effect_type: AbilityEffect::Revelation { range: 50.0, clarity: 0.8 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.3,
                    player_resonance: 100.0,
                    prerequisites: vec![],
                    teaching_duration: 300.0,
                },
            },
            EchoAbility {
                name: "Hope's Embrace".to_string(),
                description: "Heals emotional wounds and restores morale".to_string(),
                energy_cost: 0.4,
                cooldown: 30.0,
                effect_type: AbilityEffect::Healing { potency: 0.7 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.6,
                    player_resonance: 200.0,
                    prerequisites: vec!["Guiding Light".to_string()],
                    teaching_duration: 600.0,
                },
            },
        ];

        let visual_state = VisualState {
            primary_color: Color { r: 1.0, g: 0.9, b: 0.7, a: 0.9 },
            secondary_color: Color { r: 0.7, g: 0.9, b: 1.0, a: 0.7 },
            glow_intensity: 0.8,
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "Sparkles".to_string(),
                    intensity: 0.6,
                    color: Color { r: 1.0, g: 1.0, b: 0.8, a: 0.5 },
                },
            ],
            form_stability: 0.7,
            size_modifier: 0.8,
        };

        (personality, abilities, visual_state)
    }

    fn create_kai_traits() -> (EchoPersonality, Vec<EchoAbility>, VisualState) {
        let personality = EchoPersonality {
            core_traits: vec![
                "Logical".to_string(),
                "Patient".to_string(),
                "Analytical".to_string(),
                "Wise".to_string(),
                "Adaptive".to_string(),
            ],
            speaking_patterns: SpeakingPattern {
                tone: "Measured and precise".to_string(),
                vocabulary_complexity: 0.9,
                metaphor_usage: 0.4,
                characteristic_phrases: vec![
                    "Consider the underlying patterns...".to_string(),
                    "The logic is elegant, once understood.".to_string(),
                    "Let us analyze this systematically.".to_string(),
                    "Fascinating. The implications are significant.".to_string(),
                ],
            },
            preferred_teaching_style: TeachingStyle::SocraticMethod,
            emotional_responses: HashMap::new(),
        };

        let abilities = vec![
            EchoAbility {
                name: "Pattern Recognition".to_string(),
                description: "Reveals hidden connections and systems".to_string(),
                energy_cost: 0.3,
                cooldown: 10.0,
                effect_type: AbilityEffect::Revelation { range: 100.0, clarity: 1.0 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.4,
                    player_resonance: 150.0,
                    prerequisites: vec![],
                    teaching_duration: 450.0,
                },
            },
            EchoAbility {
                name: "Algorithmic Shield".to_string(),
                description: "Creates protective barriers through code manipulation".to_string(),
                energy_cost: 0.5,
                cooldown: 20.0,
                effect_type: AbilityEffect::Protection { duration: 60.0, strength: 0.8 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.7,
                    player_resonance: 300.0,
                    prerequisites: vec!["Pattern Recognition".to_string()],
                    teaching_duration: 900.0,
                },
            },
        ];

        let visual_state = VisualState {
            primary_color: Color { r: 0.3, g: 0.7, b: 1.0, a: 0.85 },
            secondary_color: Color { r: 0.1, g: 1.0, b: 0.9, a: 0.6 },
            glow_intensity: 0.6,
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "Data Streams".to_string(),
                    intensity: 0.7,
                    color: Color { r: 0.0, g: 1.0, b: 1.0, a: 0.4 },
                },
            ],
            form_stability: 0.9,
            size_modifier: 1.0,
        };

        (personality, abilities, visual_state)
    }

    fn create_terra_traits() -> (EchoPersonality, Vec<EchoAbility>, VisualState) {
        let personality = EchoPersonality {
            core_traits: vec![
                "Nurturing".to_string(),
                "Patient".to_string(),
                "Grounded".to_string(),
                "Protective".to_string(),
                "Wise".to_string(),
            ],
            speaking_patterns: SpeakingPattern {
                tone: "Deep and resonant".to_string(),
                vocabulary_complexity: 0.7,
                metaphor_usage: 0.9,
                characteristic_phrases: vec![
                    "All things grow in their season.".to_string(),
                    "The roots run deeper than you know.".to_string(),
                    "Patience, young seedling.".to_string(),
                    "The earth remembers all.".to_string(),
                ],
            },
            preferred_teaching_style: TeachingStyle::Demonstration,
            emotional_responses: HashMap::new(),
        };

        let abilities = vec![
            EchoAbility {
                name: "Nature's Embrace".to_string(),
                description: "Accelerates growth and healing".to_string(),
                energy_cost: 0.4,
                cooldown: 15.0,
                effect_type: AbilityEffect::Healing { potency: 0.9 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.3,
                    player_resonance: 120.0,
                    prerequisites: vec![],
                    teaching_duration: 400.0,
                },
            },
            EchoAbility {
                name: "Living Fortress".to_string(),
                description: "Creates protective barriers from nature".to_string(),
                energy_cost: 0.6,
                cooldown: 45.0,
                effect_type: AbilityEffect::Creation { complexity: 0.8 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.8,
                    player_resonance: 350.0,
                    prerequisites: vec!["Nature's Embrace".to_string()],
                    teaching_duration: 1200.0,
                },
            },
        ];

        let visual_state = VisualState {
            primary_color: Color { r: 0.4, g: 0.7, b: 0.3, a: 0.95 },
            secondary_color: Color { r: 0.6, g: 0.4, b: 0.2, a: 0.8 },
            glow_intensity: 0.5,
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "Leaves".to_string(),
                    intensity: 0.4,
                    color: Color { r: 0.6, g: 0.8, b: 0.3, a: 0.6 },
                },
            ],
            form_stability: 1.0,
            size_modifier: 1.3,
        };

        (personality, abilities, visual_state)
    }

    fn create_ignis_traits() -> (EchoPersonality, Vec<EchoAbility>, VisualState) {
        let personality = EchoPersonality {
            core_traits: vec![
                "Courageous".to_string(),
                "Passionate".to_string(),
                "Inspiring".to_string(),
                "Direct".to_string(),
                "Protective".to_string(),
            ],
            speaking_patterns: SpeakingPattern {
                tone: "Bold and energetic".to_string(),
                vocabulary_complexity: 0.6,
                metaphor_usage: 0.7,
                characteristic_phrases: vec![
                    "The fire within burns eternal!".to_string(),
                    "Stand tall, Songweaver!".to_string(),
                    "Together, we are unstoppable!".to_string(),
                    "Let your courage blaze forth!".to_string(),
                ],
            },
            preferred_teaching_style: TeachingStyle::DirectInstruction,
            emotional_responses: HashMap::new(),
        };

        let abilities = vec![
            EchoAbility {
                name: "Rallying Cry".to_string(),
                description: "Inspires courage and strength in allies".to_string(),
                energy_cost: 0.3,
                cooldown: 20.0,
                effect_type: AbilityEffect::Inspiration { targets: 5, boost: 0.5 },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.2,
                    player_resonance: 100.0,
                    prerequisites: vec![],
                    teaching_duration: 300.0,
                },
            },
            EchoAbility {
                name: "Phoenix Rebirth".to_string(),
                description: "Transforms defeat into renewed strength".to_string(),
                energy_cost: 0.8,
                cooldown: 120.0,
                effect_type: AbilityEffect::Transformation { scope: "Revival".to_string() },
                teaching_requirements: TeachingRequirements {
                    bond_level: 0.9,
                    player_resonance: 400.0,
                    prerequisites: vec!["Rallying Cry".to_string()],
                    teaching_duration: 1500.0,
                },
            },
        ];

        let visual_state = VisualState {
            primary_color: Color { r: 1.0, g: 0.4, b: 0.1, a: 0.9 },
            secondary_color: Color { r: 1.0, g: 0.7, b: 0.0, a: 0.7 },
            glow_intensity: 0.9,
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "Embers".to_string(),
                    intensity: 0.8,
                    color: Color { r: 1.0, g: 0.5, b: 0.0, a: 0.5 },
                },
            ],
            form_stability: 0.8,
            size_modifier: 1.1,
        };

        (personality, abilities, visual_state)
    }

    pub fn update_bond(&mut self, player_id: Uuid, change: f32) {
        let current = self.bond_levels.get(&player_id).copied().unwrap_or(0.0);
        self.bond_levels.insert(player_id, (current + change).clamp(0.0, 1.0));
    }

    pub fn remember_interaction(&mut self, player_id: Uuid, interaction: InteractionRecord) {
        self.memory.player_interactions
            .entry(player_id)
            .or_insert_with(Vec::new)
            .push(interaction);
    }

    pub fn get_dialogue_for_context(&self, player_id: Uuid, context: &str) -> String {
        let bond_level = self.bond_levels.get(&player_id).copied().unwrap_or(0.0);

        match self.echo_type {
            EchoType::Lumi => {
                if bond_level < 0.3 {
                    "Oh! A new friend! Do you see how the light dances here?".to_string()
                } else if bond_level < 0.7 {
                    format!("I'm so glad you're here! {}",
                            self.personality.speaking_patterns.characteristic_phrases[1])
                } else {
                    "My dear friend, your light shines so brightly now! Let's explore together!".to_string()
                }
            },
            EchoType::KAI => {
                if bond_level < 0.3 {
                    "Greetings. I observe you possess potential for understanding.".to_string()
                } else if bond_level < 0.7 {
                    "Your progress is noteworthy. Let us delve deeper into the patterns.".to_string()
                } else {
                    "Colleague, your grasp of the Song's logic has become quite sophisticated.".to_string()
                }
            },
            EchoType::Terra => {
                if bond_level < 0.3 {
                    "Welcome, young one. The earth senses your presence.".to_string()
                } else if bond_level < 0.7 {
                    "You grow stronger, like a sapling reaching for the sun.".to_string()
                } else {
                    "Dear child of the Song, your roots run deep now. The forest sings of your deeds.".to_string()
                }
            },
            EchoType::Ignis => {
                if bond_level < 0.3 {
                    "Ha! A new warrior approaches! Show me your fire!".to_string()
                } else if bond_level < 0.7 {
                    "Your courage grows, friend! Together we shall face any challenge!".to_string()
                } else {
                    "My trusted companion! Our flames burn as one! Nothing can stop us now!".to_string()
                }
            },
        }
    }
}