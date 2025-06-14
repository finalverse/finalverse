// services/first-hour/src/world_client.rs
use anyhow::Result;
use tracing::info;

pub struct WorldEngineClient {
    base_url: String,
}

impl WorldEngineClient {
    pub async fn connect(url: &str) -> Result<Self> {
        info!("Connecting to world engine at {}", url);
        Ok(Self {
            base_url: url.to_string(),
        })
    }

    pub async fn request_grid_generation(
        &mut self,
        coord: finalverse_world3d::GridCoordinate,
        world_id: &str,
        biome_hint: Option<&str>,
    ) -> Result<()> {
        info!(
            "Requesting grid generation for {:?} in world {} with biome hint {:?}",
            coord, world_id, biome_hint
        );

        // TODO: Implement actual gRPC/HTTP communication with world-engine
        // For now, this is a placeholder

        Ok(())
    }

    pub async fn spawn_entity(
        &mut self,
        entity_type: &str,
        position: finalverse_world3d::Position3D,
        grid: finalverse_world3d::GridCoordinate,
    ) -> Result<finalverse_world3d::EntityId> {
        info!(
            "Spawning {} at {:?} in grid {:?}",
            entity_type, position, grid
        );

        // TODO: Implement actual entity spawning via world-engine
        // For now, return a dummy ID
        Ok(finalverse_world3d::EntityId(uuid::Uuid::new_v4()))
    }
}