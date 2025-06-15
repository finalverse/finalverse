// services/world-engine/src/grpc/mod.rs
pub mod server;
pub mod client;

// Re-export proto types
pub use finalverse_proto::world::*;