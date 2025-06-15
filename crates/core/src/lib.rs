// libs/common/src/lib.rs

pub mod events;
pub mod types;
pub mod error;

pub use events::*;
pub use types::*;
pub use error::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Core domain types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegionId(pub Uuid);

/// Unique identifier for an Echo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EchoId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Song-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harmony {
    pub level: f32,
    pub region: RegionId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resonance {
    pub creative: u64,
    pub exploration: u64,
    pub restoration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Melody {
    Healing { power: f32 },
    Creation { pattern: String },
    Discovery { range: f32 },
    Courage { intensity: f32 },
}

// Echo types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EchoType {
    Lumi,   // Hope & Discovery
    KAI,    // Logic & Understanding
    Terra,  // Resilience & Growth
    Ignis,  // Courage & Creation
}

impl EchoType {
    /// Deterministically generate the unique [`EchoId`] for this variant
    pub fn id(&self) -> EchoId {
        let uuid = match self {
            EchoType::Lumi => Uuid::new_v5(&Uuid::NAMESPACE_OID, b"lumi"),
            EchoType::KAI => Uuid::new_v5(&Uuid::NAMESPACE_OID, b"kai"),
            EchoType::Terra => Uuid::new_v5(&Uuid::NAMESPACE_OID, b"terra"),
            EchoType::Ignis => Uuid::new_v5(&Uuid::NAMESPACE_OID, b"ignis"),
        };
        EchoId(uuid)
    }
}

// Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinalverseEvent {
    // Song Events
    HarmonyRestored {
        region: RegionId,
        restorer: PlayerId,
        amount: f32,
    },
    SilenceManifested {
        location: Coordinates,
        intensity: f32,
    },
    MelodyPerformed {
        player: PlayerId,
        melody: Melody,
        target: Coordinates,
    },
    
    // Player Events
    PlayerConnected {
        player: PlayerId,
        timestamp: DateTime<Utc>,
    },
    PlayerDisconnected {
        player: PlayerId,
        timestamp: DateTime<Utc>,
    },
    
    // Echo Events
    EchoBondIncreased {
        player: PlayerId,
        echo: EchoId,
        new_level: u32,
    },
    
    // World Events
    RegionStateChanged {
        region: RegionId,
        harmony: f32,
        discord: f32,
    },
}

// Service health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: ServiceStatus,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum FinalverseError {
    #[error("Service communication error: {0}")]
    ServiceError(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("AI processing error: {0}")]
    AIError(String),
}

// Utilities
pub mod utils {
    use super::*;
    
    pub fn new_player_id() -> PlayerId {
        PlayerId(Uuid::new_v5(&Uuid::NAMESPACE_OID, b"finalverse.com"))
    }
    
    pub fn new_region_id() -> RegionId {
        RegionId(Uuid::new_v5(&Uuid::NAMESPACE_OID, b"finalverse.com"))
    }
}
