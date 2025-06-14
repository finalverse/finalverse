// crates/world3d/src/terrain.rs
use noise::{NoiseFn, Perlin, SuperSimplex, Fbm, MultiFractal};
use serde::{Deserialize, Serialize};
use crate::{GridCoordinate, Position3D};

pub const GRID_SIZE: f32 = 256.0;
pub const GRID_RESOLUTION: usize = 256; // 256x256 heightmap per grid

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainPatch {
    pub heightmap: Vec<Vec<f32>>,
    pub textures: Vec<TerrainLayer>,
    pub vegetation_map: VegetationMap,
    pub water_bodies: Vec<WaterBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainLayer {
    pub texture_id: String,
    pub blend_map: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegetationMap {
    pub density: Vec<Vec<f32>>,
    pub types: Vec<VegetationType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegetationType {
    pub id: String,
    pub mesh_id: String,
    pub density_threshold: f32,
    pub max_slope: f32,
    pub min_height: f32,
    pub max_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterBody {
    pub level: f32,
    pub bounds: Vec<Position3D>,
}

pub struct TerrainGenerator {
    base_noise: Fbm<Perlin>,
    detail_noise: SuperSimplex,
    harmony_seed: u64,
}

impl TerrainGenerator {
    pub fn new(seed: u64) -> Self {
        let mut base_noise = Fbm::<Perlin>::new(seed as u32);
        base_noise.octaves = 6;
        base_noise.frequency = 0.001;
        base_noise.persistence = 0.5;
        base_noise.lacunarity = 2.0;

        Self {
            base_noise,
            detail_noise: SuperSimplex::new(seed as u32),
            harmony_seed: seed,
        }
    }

    pub fn generate_grid_terrain(
        &self,
        grid_coord: GridCoordinate,
        harmony_level: f32,
        biome: Biome,
    ) -> TerrainPatch {
        let mut heightmap = vec![vec![0.0; GRID_RESOLUTION]; GRID_RESOLUTION];

        // Generate base terrain
        for y in 0..GRID_RESOLUTION {
            for x in 0..GRID_RESOLUTION {
                let world_x = grid_coord.x as f64 * GRID_SIZE as f64 + x as f64;
                let world_y = grid_coord.y as f64 * GRID_SIZE as f64 + y as f64;

                // Multi-octave noise for base terrain
                let base_height = self.base_noise.get([world_x * 0.001, world_y * 0.001]) as f32;

                // Add detail noise
                let detail = self.detail_noise.get([world_x * 0.01, world_y * 0.01]) as f32;

                // Biome-specific modifications
                let biome_modifier = match biome {
                    Biome::WeaversLanding => {
                        // Gentle rolling hills with river valley
                        let river_distance = ((world_x - world_y).abs() / 100.0).min(1.0);
                        1.0 - (river_distance * 0.3)
                    },
                    Biome::WhisperwoodGrove => {
                        // More varied terrain for forest
                        1.2 + (detail * 0.3)
                    },
                    Biome::MemoryGrotto => {
                        // Bowl-shaped depression
                        let center_dist = ((world_x - grid_coord.x as f64 * GRID_SIZE as f64 - 128.0).powi(2) +
                            (world_y - grid_coord.y as f64 * GRID_SIZE as f64 - 128.0).powi(2)).sqrt() / 128.0;
                        1.0 - (center_dist * 0.5).min(0.5)
                    },
                    _ => 1.0,
                };

                // Apply harmony modifications
                let harmony_modifier = 1.0 + (harmony_level - 0.5) * 0.2;

                heightmap[y][x] = (base_height * 30.0 + detail * 5.0) * biome_modifier * harmony_modifier + 50.0;
            }
        }

        // Generate texture layers based on height and slope
        let textures = self.generate_texture_layers(&heightmap, biome);

        // Generate vegetation
        let vegetation_map = self.generate_vegetation(&heightmap, harmony_level, biome);

        // Detect water bodies
        let water_bodies = self.detect_water_bodies(&heightmap);

        TerrainPatch {
            heightmap,
            textures,
            vegetation_map,
            water_bodies,
        }
    }

    fn generate_texture_layers(&self, heightmap: &Vec<Vec<f32>>, biome: Biome) -> Vec<TerrainLayer> {
        let mut layers = Vec::new();

        // Base layer - grass/dirt
        let mut grass_blend = vec![vec![0.0; GRID_RESOLUTION]; GRID_RESOLUTION];
        let mut rock_blend = vec![vec![0.0; GRID_RESOLUTION]; GRID_RESOLUTION];
        let mut sand_blend = vec![vec![0.0; GRID_RESOLUTION]; GRID_RESOLUTION];

        for y in 1..GRID_RESOLUTION-1 {
            for x in 1..GRID_RESOLUTION-1 {
                let height = heightmap[y][x];
                let slope = self.calculate_slope(heightmap, x, y);

                // Rock on steep slopes
                if slope > 0.5 {
                    rock_blend[y][x] = (slope - 0.5) * 2.0;
                }

                // Sand near water level
                if height < 52.0 {
                    sand_blend[y][x] = (52.0 - height) / 2.0;
                }

                // Grass everywhere else
                grass_blend[y][x] = 1.0 - rock_blend[y][x] - sand_blend[y][x];
            }
        }

        layers.push(TerrainLayer {
            texture_id: "grass_verdant".to_string(),
            blend_map: grass_blend,
        });

        layers.push(TerrainLayer {
            texture_id: "rock_cliff".to_string(),
            blend_map: rock_blend,
        });

        layers.push(TerrainLayer {
            texture_id: "sand_river".to_string(),
            blend_map: sand_blend,
        });

        layers
    }

    fn calculate_slope(&self, heightmap: &Vec<Vec<f32>>, x: usize, y: usize) -> f32 {
        let dx = heightmap[y][x + 1] - heightmap[y][x - 1];
        let dy = heightmap[y + 1][x] - heightmap[y - 1][x];
        (dx * dx + dy * dy).sqrt() / 2.0
    }

    fn generate_vegetation(&self, heightmap: &Vec<Vec<f32>>, harmony_level: f32, biome: Biome) -> VegetationMap {
        let mut density = vec![vec![0.0; GRID_RESOLUTION]; GRID_RESOLUTION];

        for y in 1..GRID_RESOLUTION-1 {
            for x in 1..GRID_RESOLUTION-1 {
                let height = heightmap[y][x];
                let slope = self.calculate_slope(heightmap, x, y);

                // No vegetation on steep slopes or in water
                if slope < 0.3 && height > 51.0 {
                    let noise_val = self.detail_noise.get([x as f64 * 0.1, y as f64 * 0.1]) as f32;
                    density[y][x] = ((noise_val + 1.0) * 0.5 * harmony_level).min(1.0);
                }
            }
        }

        let types = match biome {
            Biome::WeaversLanding => vec![
                VegetationType {
                    id: "willow_tree".to_string(),
                    mesh_id: "tree_willow_01".to_string(),
                    density_threshold: 0.7,
                    max_slope: 0.2,
                    min_height: 52.0,
                    max_height: 80.0,
                },
                VegetationType {
                    id: "harmony_flower".to_string(),
                    mesh_id: "flower_glowing_01".to_string(),
                    density_threshold: 0.3,
                    max_slope: 0.3,
                    min_height: 51.0,
                    max_height: 70.0,
                },
            ],
            Biome::WhisperwoodGrove => vec![
                VegetationType {
                    id: "ancient_oak".to_string(),
                    mesh_id: "tree_oak_ancient".to_string(),
                    density_threshold: 0.6,
                    max_slope: 0.25,
                    min_height: 52.0,
                    max_height: 90.0,
                },
                VegetationType {
                    id: "resonant_blossom".to_string(),
                    mesh_id: "flower_resonant_01".to_string(),
                    density_threshold: 0.4,
                    max_slope: 0.3,
                    min_height: 51.0,
                    max_height: 75.0,
                },
            ],
            _ => vec![],
        };

        VegetationMap { density, types }
    }

    fn detect_water_bodies(&self, heightmap: &Vec<Vec<f32>>) -> Vec<WaterBody> {
        let water_level = 50.0;
        let mut water_bodies = Vec::new();

        // Simple flood fill to find connected water areas
        let mut visited = vec![vec![false; GRID_RESOLUTION]; GRID_RESOLUTION];

        for y in 0..GRID_RESOLUTION {
            for x in 0..GRID_RESOLUTION {
                if !visited[y][x] && heightmap[y][x] < water_level {
                    let bounds = self.flood_fill_water(heightmap, &mut visited, x, y, water_level);
                    if bounds.len() > 10 { // Minimum size for a water body
                        water_bodies.push(WaterBody {
                            level: water_level,
                            bounds,
                        });
                    }
                }
            }
        }

        water_bodies
    }

    fn flood_fill_water(
        &self,
        heightmap: &Vec<Vec<f32>>,
        visited: &mut Vec<Vec<bool>>,
        start_x: usize,
        start_y: usize,
        water_level: f32,
    ) -> Vec<Position3D> {
        let mut bounds = Vec::new();
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            if x >= GRID_RESOLUTION || y >= GRID_RESOLUTION || visited[y][x] {
                continue;
            }

            if heightmap[y][x] < water_level {
                visited[y][x] = true;
                bounds.push(Position3D::new(x as f32, y as f32, water_level));

                // Add neighbors
                if x > 0 { stack.push((x - 1, y)); }
                if x < GRID_RESOLUTION - 1 { stack.push((x + 1, y)); }
                if y > 0 { stack.push((x, y - 1)); }
                if y < GRID_RESOLUTION - 1 { stack.push((x, y + 1)); }
            }
        }

        bounds
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    WeaversLanding,
    WhisperwoodGrove,
    MemoryGrotto,
    PlazaOfEchoes,
    Other,
}