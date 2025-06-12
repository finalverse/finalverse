// crates/fv-events/src/lib.rs
pub mod event_bus;
pub mod events;
pub mod nats;
pub mod local;

pub use event_bus::GameEventBus;
pub use events::*;
pub use nats::NatsEventBus;
pub use local::LocalEventBus;

// Re-export commonly used types
pub use async_trait::async_trait;
pub use serde::{Deserialize, Serialize};