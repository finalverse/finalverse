use finalverse_world3d::{WorldId, world::World, GridCoordinate};
use std::collections::HashMap;

pub struct WorldManager {
    worlds: HashMap<WorldId, World>,
}

impl WorldManager {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self { worlds: HashMap::new() })
    }

    pub async fn create_terra_nova_world(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn ensure_grid_loaded(&self, _coord: GridCoordinate) -> anyhow::Result<()> {
        Ok(())
    }
}
