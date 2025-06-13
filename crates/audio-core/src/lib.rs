// crates/finalverse-audio-core/src/lib.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use nalgebra::Vector3;

/// Core audio types shared across the Finalverse ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEvent {
    pub id: Uuid,
    pub event_type: AudioEventType,
    pub position: Option<Vector3<f32>>,
    pub source: AudioSource,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioEventType {
    // World Events
    RegionHarmonyChanged { region_id: String, harmony_level: f32 },
    CelestialEvent { event_name: String },
    WeatherChange { weather_type: WeatherType },

    // Character Events
    CharacterSpeak { character_id: String, emotion: EmotionalState, text: String },
    EchoAppearance { echo_type: EchoType },

    // Player Events
    SongweavingStart { player_id: String, melody_type: MelodyType },
    SongweavingComplete { success: bool, harmony_gained: f32 },
    UIInteraction { interaction_type: UISound },

    // Environmental
    AmbientTrigger { trigger_id: String, intensity: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioSource {
    World,
    Player(String),
    NPC(String),
    Echo(EchoType),
    Environment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EchoType {
    Lumi,
    KAI,
    Terra,
    Ignis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalState {
    Joyful,
    Sad,
    Hopeful,
    Fearful,
    Determined,
    Curious,
    Melancholic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MelodyType {
    Restoration,
    Discovery,
    Protection,
    Creation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Storm,
    DissonanceStorm,
    CelestialLight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UISound {
    MenuOpen,
    MenuClose,
    ItemSelect,
    ItemEquip,
    QuestAccept,
    QuestComplete,
}

/// Musical components for the Song of Creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicalTheme {
    pub id: String,
    pub base_scale: Scale,
    pub tempo: f32, // BPM
    pub mood: MoodDescriptor,
    pub instrumentation: Vec<Instrument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scale {
    Major,
    Minor,
    Pentatonic,
    Lydian,
    Dorian,
    Phrygian,
    Chromatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodDescriptor {
    pub valence: f32,    // -1.0 (sad) to 1.0 (happy)
    pub energy: f32,     // 0.0 (calm) to 1.0 (energetic)
    pub tension: f32,    // 0.0 (relaxed) to 1.0 (tense)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instrument {
    // Lumi's instruments
    CrystalBells,
    EtherealChimes,
    CelestialHarp,

    // KAI's instruments
    DigitalSynth,
    AlgorithmicPulse,
    DataStream,

    // Terra's instruments
    DeepWoodwind,
    EarthDrum,
    NatureAmbience,

    // Ignis's instruments
    HeroicBrass,
    FireCrackle,
    BattleDrum,

    // General
    StringSection,
    Choir,
    Piano,
}

/// Audio streaming protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStreamRequest {
    pub stream_id: Uuid,
    pub stream_type: StreamType,
    pub quality: AudioQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    Ambient,
    Character,
    Effect,
    Music,
    Voice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioQuality {
    Low,     // 64kbps
    Medium,  // 128kbps
    High,    // 256kbps
    Lossless, // FLAC
}