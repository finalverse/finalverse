// crates/finalverse-core/src/character.rs
use crate::types::{Position, Uuid};
use crate::echo::{Echo, InteractionRecord};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Human characters in Finalverse - both player characters and key NPCs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub character_type: CharacterType,
    pub position: Position,
    pub attributes: CharacterAttributes,
    pub songweaver_abilities: SongweaverAbilities,
    pub relationships: HashMap<Uuid, Relationship>,
    pub inventory: Vec<Item>,
    pub companion: Option<Companion>,
    pub personal_story: PersonalStory,
    pub appearance: CharacterAppearance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterType {
    Player,
    KeyNPC(KeyNPCRole),
    NPC(NPCRole),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyNPCRole {
    ElaraVayne,     // The Compassionate Harmonist
    Anya,           // The Sculptor
    MarcusStone,    // The Skeptical Scholar
    LyraWindsong,   // The Pragmatic Leader
    KaelDarkbane,   // The Fallen Hero
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCRole {
    Merchant,
    QuestGiver,
    Villager,
    Guard,
    Scholar,
    Artisan,
    Child,
    Elder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAttributes {
    pub health: f32,
    pub max_health: f32,
    pub resonance: ResonanceScore,
    pub emotional_state: EmotionalState,
    pub fatigue: f32,
    pub inspiration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceScore {
    pub creative: f32,
    pub exploration: f32,
    pub restoration: f32,
    pub total: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub primary: Emotion,
    pub secondary: Option<Emotion>,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Hope,
    Curiosity,
    Determination,
    Compassion,
    Fear,
    Sadness,
    Anger,
    Confusion,
    Wonder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongweaverAbilities {
    pub melodies_known: Vec<Melody>,
    pub harmonies_unlocked: Vec<Harmony>,
    pub symphonies_discovered: Vec<Symphony>,
    pub current_attunement: AttunementLevel,
    pub song_energy: f32,
    pub max_song_energy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Melody {
    pub name: String,
    pub description: String,
    pub effect: MelodyEffect,
    pub energy_cost: f32,
    pub learned_from: LearnedFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MelodyEffect {
    Healing { potency: f32 },
    Shielding { strength: f32, duration: f32 },
    Revealing { range: f32 },
    Soothing { target: TargetType },
    Energizing { boost: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
Self,
Single,
Area { radius: f32 },
Group { max_targets: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearnedFrom {
    Echo(crate::types::EchoType),
    Character(Uuid),
    Discovery,
    AncientText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harmony {
    pub name: String,
    pub description: String,
    pub required_melodies: Vec<String>,
    pub group_size: u32,
    pub effect: HarmonyEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarmonyEffect {
    AreaRestoration { radius: f32, duration: f32 },
    GroupProtection { strength: f32 },
    CombinedCreation { complexity: f32 },
    EmotionalResonance { range: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symphony {
    pub name: String,
    pub description: String,
    pub participants_required: u32,
    pub world_effect: WorldEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEffect {
    RegionalHarmonyBoost { amount: f32, duration: f32 },
    SilencePurge { radius: f32 },
    EnvironmentalTransformation { type_: String },
    CelestialEvent { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttunementLevel {
    pub tier: u32,
    pub name: String,
    pub abilities_unlocked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub bond_strength: f32,
    pub shared_experiences: Vec<SharedExperience>,
    pub current_status: RelationshipStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Mentor,
    Student,
    Companion,
    Rival,
    Romantic,
    Family,
    Acquaintance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedExperience {
    pub event_type: String,
    pub timestamp: i64,
    pub emotional_impact: f32,
    pub location: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStatus {
    Growing,
    Stable,
    Strained,
    Broken,
    Mending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub description: String,
    pub properties: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Instrument,
    Artifact,
    Journal,
    Gift,
    QuestItem,
    Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Companion {
    pub companion_type: CompanionType,
    pub name: String,
    pub bond_level: f32,
    pub abilities: Vec<CompanionAbility>,
    pub personality: CompanionPersonality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanionType {
    MelodySprite,          // Elara's companion
    GlimmerwingSongbird,   // Alternative companion
    WhisperbloomSprite,    // Plant-based companion
    ResonantCrystal,       // Mineral companion
    EchoFragment,          // Fragment of an Echo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionAbility {
    pub name: String,
    pub description: String,
    pub effect: CompanionEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanionEffect {
    EmotionalResonance { boost: f32 },
    HiddenPathRevealer,
    SongAmplifier { multiplier: f32 },
    DangerSense { range: f32 },
    MemoryKeeper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionPersonality {
    pub traits: Vec<String>,
    pub likes: Vec<String>,
    pub dislikes: Vec<String>,
    pub communication_style: CommunicationStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Musical,      // Communicates through notes
    Empathic,     // Emotional impressions
    Visual,       // Color changes
    Telepathic,   // Direct thoughts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalStory {
    pub origin: CharacterOrigin,
    pub motivations: Vec<String>,
    pub fears: Vec<String>,
    pub dreams: Vec<String>,
    pub defining_moments: Vec<DefiningMoment>,
    pub current_quest: Option<PersonalQuest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterOrigin {
    pub birthplace: String,
    pub family_background: String,
    pub early_life: String,
    pub catalyst_event: String, // What brought them to the adventure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiningMoment {
    pub description: String,
    pub impact: String,
    pub lesson_learned: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalQuest {
    pub name: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub emotional_stakes: String,
    pub potential_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    pub age: String,
    pub height: String,
    pub build: String,
    pub hair: HairStyle,
    pub eyes: EyeDescription,
    pub clothing: ClothingStyle,
    pub distinguishing_features: Vec<String>,
    pub cultural_markers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairStyle {
    pub color: String,
    pub length: String,
    pub style: String,
    pub decorations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeDescription {
    pub color: String,
    pub expression: String,
    pub notable_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClothingStyle {
    pub primary_outfit: String,
    pub colors: Vec<String>,
    pub materials: Vec<String>,
    pub accessories: Vec<String>,
}

impl Character {
    pub fn new_elara_vayne() -> Self {
        Character {
            id: Uuid::new_v4(),
            name: "Elara Vayne".to_string(),
            character_type: CharacterType::KeyNPC(KeyNPCRole::ElaraVayne),
            position: Position::new(180.0, 140.0, 52.0), // Near Anya's workshop
            attributes: CharacterAttributes {
                health: 100.0,
                max_health: 100.0,
                resonance: ResonanceScore {
                    creative: 150.0,
                    exploration: 200.0,
                    restoration: 300.0,
                    total: 650.0,
                },
                emotional_state: EmotionalState {
                    primary: Emotion::Compassion,
                    secondary: Some(Emotion::Curiosity),
                    intensity: 0.8,
                },
                fatigue: 0.2,
                inspiration: 0.9,
            },
            songweaver_abilities: SongweaverAbilities {
                melodies_known: vec![
                    Melody {
                        name: "Harmony's Touch".to_string(),
                        description: "A gentle melody that soothes discord".to_string(),
                        effect: MelodyEffect::Soothing {
                            target: TargetType::Area { radius: 10.0 }
                        },
                        energy_cost: 20.0,
                        learned_from: LearnedFrom::Discovery,
                    },
                    Melody {
                        name: "Empathic Resonance".to_string(),
                        description: "Connects hearts and minds".to_string(),
                        effect: MelodyEffect::Revealing { range: 50.0 },
                        energy_cost: 30.0,
                        learned_from: LearnedFrom::Echo(crate::types::EchoType::Lumi),
                    },
                ],
                harmonies_unlocked: vec![],
                symphonies_discovered: vec![],
                current_attunement: AttunementLevel {
                    tier: 3,
                    name: "Harmonist".to_string(),
                    abilities_unlocked: vec![
                        "Emotional Insight".to_string(),
                        "Harmonic Connection".to_string(),
                    ],
                },
                song_energy: 80.0,
                max_song_energy: 100.0,
            },
            relationships: HashMap::new(),
            inventory: vec![
                Item {
                    id: Uuid::new_v4(),
                    name: "Mother's Flute".to_string(),
                    item_type: ItemType::Instrument,
                    description: "A beautifully carved wooden flute from Terra Nova".to_string(),
                    properties: HashMap::from([
                        ("sentimental_value".to_string(), 1.0),
                        ("harmony_boost".to_string(), 0.1),
                    ]),
                },
                Item {
                    id: Uuid::new_v4(),
                    name: "Journal of Melodies".to_string(),
                    item_type: ItemType::Journal,
                    description: "Contains notes on discovered songs and their effects".to_string(),
                    properties: HashMap::new(),
                },
            ],
            companion: Some(Companion {
                companion_type: CompanionType::MelodySprite,
                name: "Lyra".to_string(),
                bond_level: 0.8,
                abilities: vec![
                    CompanionAbility {
                        name: "Harmonic Echo".to_string(),
                        description: "Amplifies Elara's melodies".to_string(),
                        effect: CompanionEffect::SongAmplifier { multiplier: 1.3 },
                    },
                    CompanionAbility {
                        name: "Emotional Attunement".to_string(),
                        description: "Senses the emotional state of others".to_string(),
                        effect: CompanionEffect::EmotionalResonance { boost: 0.2 },
                    },
                ],
                personality: CompanionPersonality {
                    traits: vec![
                        "Playful".to_string(),
                        "Empathetic".to_string(),
                        "Musical".to_string(),
                    ],
                    likes: vec![
                        "Harmonious music".to_string(),
                        "Flowering plants".to_string(),
                        "Gentle breezes".to_string(),
                    ],
                    dislikes: vec![
                        "Discord".to_string(),
                        "Loud noises".to_string(),
                        "Darkness".to_string(),
                    ],
                    communication_style: CommunicationStyle::Musical,
                },
            }),
            personal_story: PersonalStory {
                origin: CharacterOrigin {
                    birthplace: "Harmony Grove, Terra Nova".to_string(),
                    family_background: "Daughter of musicians and healers".to_string(),
                    early_life: "Showed early aptitude for sensing and harmonizing emotional discord".to_string(),
                    catalyst_event: "The Fading reached her village, stealing the music from her mother's voice".to_string(),
                },
                motivations: vec![
                    "Restore harmony to the fading world".to_string(),
                    "Understand the true nature of the Song".to_string(),
                    "Protect those who cannot protect themselves".to_string(),
                ],
                fears: vec![
                    "Losing those she loves to the Silence".to_string(),
                    "Being unable to help when needed".to_string(),
                    "The complete loss of music from the world".to_string(),
                ],
                dreams: vec![
                    "A world where all beings live in harmony".to_string(),
                    "To master the deepest mysteries of the Song".to_string(),
                    "To see her mother sing again".to_string(),
                ],
                defining_moments: vec![
                    DefiningMoment {
                        description: "First harmonizing with Lumi".to_string(),
                        impact: "Realized her true potential as a Songweaver".to_string(),
                        lesson_learned: "Hope can be found even in darkness".to_string(),
                        timestamp: 1625097600,
                    },
                ],
                current_quest: Some(PersonalQuest {
                    name: "The Songweaver's Journey".to_string(),
                    description: "Unite with the First Echoes to combat the growing Silence".to_string(),
                    objectives: vec![
                        "Learn from each of the First Echoes".to_string(),
                        "Master the three aspects of Resonance".to_string(),
                        "Discover the source of the Great Silence".to_string(),
                    ],
                    emotional_stakes: "The fate of all music and harmony in the universe".to_string(),
                    potential_outcomes: vec![
                        "Restore the Song to its full glory".to_string(),
                        "Find a balance between Song and Silence".to_string(),
                        "Sacrifice to save what remains".to_string(),
                    ],
                }),
            },
            appearance: CharacterAppearance {
                age: "Early twenties".to_string(),
                height: "Average height, graceful build".to_string(),
                build: "Slender but resilient".to_string(),
                hair: HairStyle {
                    color: "Warm auburn".to_string(),
                    length: "Mid-back length".to_string(),
                    style: "Often in a loose braid with escaping wisps".to_string(),
                    decorations: vec![
                        "Small musical note pins".to_string(),
                        "Flowers from Terra Nova".to_string(),
                    ],
                },
                eyes: EyeDescription {
                    color: "Deep hazel with golden flecks".to_string(),
                    expression: "Kind and perceptive".to_string(),
                    notable_features: vec![
                        "Seem to shimmer when using Song abilities".to_string(),
                    ],
                },
                clothing: ClothingStyle {
                    primary_outfit: "Practical traveler's tunic and comfortable trousers".to_string(),
                    colors: vec![
                        "Earth tones".to_string(),
                        "Deep greens".to_string(),
                        "Warm browns".to_string(),
                        "Sunset orange accents".to_string(),
                    ],
                    materials: vec![
                        "Durable linen".to_string(),
                        "Soft leather".to_string(),
                        "Woven grass from Terra Nova".to_string(),
                    ],
                    accessories: vec![
                        "Mother's flute on a leather cord".to_string(),
                        "Woven belt with pouches".to_string(),
                        "Comfortable walking boots".to_string(),
                        "Light cloak with hood".to_string(),
                    ],
                },
                distinguishing_features: vec![
                    "Gentle smile that puts others at ease".to_string(),
                    "Graceful, dancer-like movements".to_string(),
                    "Often humming softly to herself".to_string(),
                ],
                cultural_markers: vec![
                    "Terra Novan braiding patterns".to_string(),
                    "Musical notation tattoo on left wrist".to_string(),
                ],
            },
        }
    }

    pub fn new_player(name: String, starting_position: Position) -> Self {
        Character {
            id: Uuid::new_v4(),
            name,
            character_type: CharacterType::Player,
            position: starting_position,
            attributes: CharacterAttributes {
                health: 100.0,
                max_health: 100.0,
                resonance: ResonanceScore {
                    creative: 0.0,
                    exploration: 0.0,
                    restoration: 0.0,
                    total: 0.0,
                },
                emotional_state: EmotionalState {
                    primary: Emotion::Curiosity,
                    secondary: None,
                    intensity: 0.5,
                },
                fatigue: 0.0,
                inspiration: 0.5,
            },
            songweaver_abilities: SongweaverAbilities {
                melodies_known: vec![],
                harmonies_unlocked: vec![],
                symphonies_discovered: vec![],
                current_attunement: AttunementLevel {
                    tier: 0,
                    name: "Awakening".to_string(),
                    abilities_unlocked: vec![],
                },
                song_energy: 50.0,
                max_song_energy: 50.0,
            },
            relationships: HashMap::new(),
            inventory: vec![],
            companion: None,
            personal_story: PersonalStory {
                origin: CharacterOrigin {
                    birthplace: "Unknown".to_string(),
                    family_background: "To be discovered".to_string(),
                    early_life: "Fragments of memory".to_string(),
                    catalyst_event: "Awakening in the Memory Grotto".to_string(),
                },
                motivations: vec![],
                fears: vec![],
                dreams: vec![],
                defining_moments: vec![],
                current_quest: None,
            },
            appearance: CharacterAppearance {
                age: "Variable".to_string(),
                height: "Variable".to_string(),
                build: "Variable".to_string(),
                hair: HairStyle {
                    color: "Customizable".to_string(),
                    length: "Customizable".to_string(),
                    style: "Customizable".to_string(),
                    decorations: vec![],
                },
                eyes: EyeDescription {
                    color: "Customizable".to_string(),
                    expression: "Determined".to_string(),
                    notable_features: vec![],
                },
                clothing: ClothingStyle {
                    primary_outfit: "Simple traveler's garb".to_string(),
                    colors: vec!["Neutral tones".to_string()],
                    materials: vec!["Common cloth".to_string()],
                    accessories: vec![],
                },
                distinguishing_features: vec![],
                cultural_markers: vec![],
            },
        }
    }

    pub fn interact_with_echo(&mut self, echo: &Echo) -> InteractionRecord {
        // Implementation for character-echo interaction
        InteractionRecord {
            timestamp: chrono::Utc::now().timestamp(),
            interaction_type: crate::echo::InteractionType::Conversation,
            outcome: crate::echo::InteractionOutcome::Positive {
                description: format!("Learned from {}", echo.name),
            },
            bond_change: 0.1,
        }
    }
}