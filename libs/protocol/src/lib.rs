// libs/protocol/src/lib.rs

use finalverse_common::*;
use serde::{Deserialize, Serialize};

pub mod event_bus;
pub use event_bus::{InMemoryEventBus, RedisEventBus};

// Client -> Server messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // Connection
    Connect { player_name: String },
    Disconnect,
    
    // Gameplay
    PerformMelody { melody: Melody, target: Coordinates },
    InteractWithEcho { echo_id: EchoId },
    Move { destination: Coordinates },
    
    // Query
    GetWorldState { region: RegionId },
    GetPlayerInfo,
}

// Server -> Client messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // Connection
    Connected { player_id: PlayerId, spawn_point: Coordinates },
    Disconnected { reason: String },
    
    // State updates
    WorldStateUpdate { region: RegionId, harmony: Harmony },
    PlayerStateUpdate { resonance: Resonance, position: Coordinates },
    EventNotification { event: FinalverseEvent },
    
    // Responses
    ActionResult { success: bool, message: String },
    Error { message: String },
}

// gRPC service definitions (proto would be generated, but for MVP we'll use Rust structs)
pub mod grpc {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConnectRequest {
        pub player_name: String,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConnectResponse {
        pub player_id: String,
        pub spawn_point: Coordinates,
        pub initial_resonance: Resonance,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PerformMelodyRequest {
        pub player_id: String,
        pub melody: Melody,
        pub target: Coordinates,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PerformMelodyResponse {
        pub success: bool,
        pub harmony_change: f32,
        pub resonance_gained: Resonance,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetServiceInfoRequest {}
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetServiceInfoResponse {
        pub info: ServiceInfo,
    }
}

// Service trait that all services will implement
#[async_trait::async_trait]
pub trait FinalverseService: Send + Sync + 'static {
    fn service_info(&self) -> ServiceInfo;
    async fn health_check(&self) -> ServiceStatus;
}

// Event bus trait for inter-service communication
#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: FinalverseEvent) -> Result<(), FinalverseError>;
    async fn subscribe(&self, service_name: &str) -> Result<tokio::sync::mpsc::Receiver<FinalverseEvent>, FinalverseError>;
}