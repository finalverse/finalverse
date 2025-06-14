// services/world-engine/src/grid_generation.rs
use finalverse_world3d::{
    terrain::{TerrainGenerator, TerrainPatch, Biome},
    grid::Grid,
    GridCoordinate,
};
use anyhow::Result;

/// Basic metabolic information for a region
#[derive(Debug, Clone, Copy)]
pub struct MetabolismState {
    pub harmony_level: f32,
}

/// Mock tracker providing metabolism data
pub struct MetabolismTracker;

impl MetabolismTracker {
    pub async fn get_region_state(&self, _coord: GridCoordinate) -> Result<MetabolismState> {
        Ok(MetabolismState { harmony_level: 0.5 })
    }
}

pub struct GridGenerationService {
    terrain_generator: TerrainGenerator,
    metabolism_tracker: MetabolismTracker,
}

impl GridGenerationService {
    pub async fn generate_grid(
        &self,
        coord: GridCoordinate,
        world_id: &str,
        biome_hint: Option<&str>,
    ) -> Result<Grid> {
        // Get current metabolism state for the region
        let metabolism = self.metabolism_tracker.get_region_state(coord).await?;

        // Determine biome
        let biome = match biome_hint {
            Some("first_hour_biome") => self.determine_first_hour_biome(coord),
            _ => self.determine_biome_from_world(world_id, coord),
        };

        // Generate terrain
        let terrain = self.terrain_generator.generate_grid_terrain(
            coord,
            metabolism.harmony_level,
            biome,
        );

        let grid = Grid::new(coord, terrain);

        Ok(grid)
    }

    fn determine_first_hour_biome(&self, coord: GridCoordinate) -> Biome {
        match (coord.x, coord.y) {
            (100, 100) => Biome::MemoryGrotto,
            (101, 101) => Biome::WeaversLanding,
            (102, 101) => Biome::WhisperwoodGrove,
            _ => Biome::Other,
        }
    }

    fn determine_biome_from_world(&self, _world_id: &str, _coord: GridCoordinate) -> Biome {
        Biome::Other
    }
}
