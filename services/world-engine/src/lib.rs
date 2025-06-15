// services/world-engine/src/lib.rs
pub mod grid_generation;
pub mod world;

pub mod server;

use serde::{Deserialize, Serialize};
use finalverse_core::{RegionId, TerrainType, WeatherType};

// Re-export RegionId for use by binaries depending on this crate
pub use finalverse_core::RegionId;

// Re-export the main types from world module
pub use world::{WorldEngine, WorldState, WorldUpdate, WorldTime};

// Re-export other important types
pub use finalverse_ecosystem::{EcosystemSimulator, Species, SpeciesProfile, MigrationPhase};
pub use finalverse_metobolism::{MetabolismSimulator, RegionState, WeatherState};


// Core types that are shared across modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    CreatureMigration {
        species: String,
        from: RegionId,
        to: RegionId,
    },
    CelestialEvent {
        event_type: CelestialEventType,
        duration: u64,
    },
    SilenceOutbreak {
        epicenter: Coordinates,
        radius: f64,
        intensity: f64,
    },
    HarmonyRestored {
        region_id: RegionId,
        amount: f64
    },
    SilenceManifested {
        location: GridCoordinate,
        intensity: f64
    },
    EchoAppeared {
        echo_type: EchoType,
        position: Position3D
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelestialEventType {
    Eclipse,
    MeteorShower,
    Aurora,
    Convergence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Move(Coordinates),
    Interact(String),
    UseAbility(String),
    Craft(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: PlayerId,
    pub action: ActionType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCoordinate {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EchoType {
    Lumi,
    Kai,
    Terra,
    Ignis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Observer trait for event notifications
#[async_trait::async_trait]
pub trait Observer: Send + Sync {
    async fn notify(&self, event: &WorldEvent);
}