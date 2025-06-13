// crates/events/src/events.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Player types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlayerId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Main event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub metadata: EventMetadata,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            metadata: EventMetadata::default(),
        }
    }
    
    pub fn with_metadata(mut self, metadata: EventMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn topic(&self) -> String {
        match &self.event_type {
            EventType::Player(_) => "events.player".to_string(),
            EventType::World(_) => "events.world".to_string(),
            EventType::Harmony(_) => "events.harmony".to_string(),
            EventType::Song(_) => "events.song".to_string(),
            EventType::Echo(_) => "events.echo".to_string(),
            EventType::Silence(_) => "events.silence".to_string(),
            EventType::System(_) => "events.system".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventMetadata {
    pub source: Option<String>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub tags: Vec<String>,
}

// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Player(PlayerEvent),
    World(WorldEvent),
    Harmony(HarmonyEvent),
    Song(SongEvent),
    Echo(EchoEvent),
    Silence(SilenceEvent),
    System(SystemEvent),
}

// Player events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerEvent {
    Connected { player_id: PlayerId },
    Disconnected { player_id: PlayerId },
    Moved { player_id: PlayerId, from: Coordinates, to: Coordinates },
    ActionPerformed { player_id: PlayerId, action: PlayerAction },
    LevelUp { player_id: PlayerId, new_level: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    Move(Coordinates),
    Interact(String),
    UseAbility(String),
    Craft(String),
    Trade { with: PlayerId, items: Vec<String> },
}

// World events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    RegionChanged { region_id: RegionId, change: RegionChange },
    WeatherChanged { region_id: RegionId, weather: WeatherType },
    CreatureMigration { species: String, from: RegionId, to: RegionId },
    CelestialEvent { event_type: CelestialEventType, duration: u64 },
    GeologicalEvent { event_type: GeologicalEventType, location: Coordinates },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionChange {
    HarmonyIncreased(f64),
    DiscordIncreased(f64),
    TerrainChanged(TerrainType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rain,
    Storm,
    DissonanceStorm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Forest,
    Desert,
    Mountain,
    Ocean,
    Plains,
    Corrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelestialEventType {
    Eclipse,
    MeteorShower,
    Aurora,
    Convergence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeologicalEventType {
    Earthquake,
    Volcanic,
    Landslide,
    NewIsland,
}

// Harmony events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarmonyEvent {
    ResonanceGained {
        player_id: PlayerId,
        resonance_type: ResonanceType,
        amount: f64,
    },
    AttunementAchieved {
        player_id: PlayerId,
        tier: u32,
        total_resonance: f64,
    },
    MelodyUnlocked {
        player_id: PlayerId,
        melody: String,
        tier_required: u32,
    },
    HarmonyUnlocked {
        player_id: PlayerId,
        harmony: String,
        tier_required: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResonanceType {
    Creative,
    Exploration,
    Restoration,
}

// Song events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SongEvent {
    SongWoven {
        weaver_id: PlayerId,
        song_type: SongType,
        power: f64,
        location: Coordinates,
    },
    SymphonyStarted {
        participants: Vec<PlayerId>,
        symphony_type: String,
        required_power: f64,
    },
    SymphonyCompleted {
        participants: Vec<PlayerId>,
        symphony_type: String,
        success: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SongType {
    Healing,
    Creation,
    Destruction,
    Protection,
    Discovery,
}

// Echo events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EchoEvent {
    EchoBondFormed {
        player_id: PlayerId,
        echo_name: String,
        initial_level: u32,
    },
    EchoBondStrengthened {
        player_id: PlayerId,
        echo_name: String,
        new_level: u32,
    },
    EchoAbilityGranted {
        player_id: PlayerId,
        echo_name: String,
        ability: String,
    },
}

// Silence events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SilenceEvent {
    SilenceDetected {
        location: Coordinates,
        intensity: f64,
        radius: f64,
    },
    DiscordantSpawned {
        discordant_id: String,
        location: Coordinates,
        threat_level: u32,
    },
    CorruptionSpread {
        region_id: RegionId,
        corruption_level: f64,
    },
    SilencePurified {
        location: Coordinates,
        purifier_id: PlayerId,
        area_restored: f64,
    },
}

// System events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ServiceStarted { service_name: String },
    ServiceStopped { service_name: String },
    ServiceHealthChanged { service_name: String, healthy: bool },
    MaintenanceScheduled { start_time: DateTime<Utc>, duration: u64 },
    ServerRestart { reason: String, countdown: u64 },
}