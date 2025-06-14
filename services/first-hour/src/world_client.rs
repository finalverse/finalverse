// services/first-hour/src/world_client.rs
use finalverse_world3d::GridCoordinate;
use tracing::info;

/// Thin client used by the first hour service to request grid generation from
/// the world engine. The actual gRPC client is omitted here to keep the
/// example self‑contained.
pub struct WorldEngineClient {
    url: String,
}

impl WorldEngineClient {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        Ok(Self { url: url.to_string() })
    }

    pub async fn request_grid_generation(
        &mut self,
        coord: GridCoordinate,
        world_id: &str,
        biome_hint: Option<&str>,
    ) -> anyhow::Result<()> {
        info!(
            "requesting grid ({}, {}) in world {} biome {:?}",
            coord.x,
            coord.y,
            world_id,
            biome_hint
        );
        // Stub call
        Ok(())
    }
}
