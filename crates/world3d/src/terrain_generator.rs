// crates/world-3d/src/terrain_generator.rs
use noise::{NoiseFn, Perlin, Seedable, SuperSimplex, Fbm, MultiFractal};
use nalgebra::{Vector2, Vector3};
use std::collections::HashMap;

pub struct TerrainGenerator {
    noise_engine: NoiseEngine,
    biome_mapper: BiomeMapper,
    harmony_modifier: HarmonyModifier,
}

pub struct NoiseEngine {
    height_noise: Fbm<Perlin>,
    moisture_noise: SuperSimplex,
    temperature_noise: Perlin,
    detail_noise: Perlin,
}

impl NoiseEngine {
    pub fn new(seed: u32) -> Self {
        let mut height_noise = Fbm::<Perlin>::new(seed);
        height_noise.octaves = 6;
        height_noise.frequency = 0.001;
        height_noise.lacunarity = 2.0;
        height_noise.persistence = 0.5;

        Self {
            height_noise,
            moisture_noise: SuperSimplex::new(seed + 1),
            temperature_noise: Perlin::new(seed + 2),
            detail_noise: Perlin::new(seed + 3),
        }
    }

    pub fn generate_heightmap(&self, grid_coord: GridCoordinate) -> HeightMap {
        let grid_size = 256;
        let mut heights = vec![vec![0.0; grid_size]; grid_size];

        let base_x = grid_coord.x as f64 * grid_size as f64;
        let base_z = grid_coord.z as f64 * grid_size as f64;

        for x in 0..grid_size {
            for z in 0..grid_size {
                let world_x = base_x + x as f64;
                let world_z = base_z + z as f64;

                // Multi-octave height generation
                let mut height = 0.0;

                // Continental shelf
                let continental = self.height_noise.get([world_x * 0.0001, world_z * 0.0001]) * 100.0;

                // Regional variations
                let regional = self.height_noise.get([world_x * 0.001, world_z * 0.001]) * 30.0;

                // Local details
                let detail = self.detail_noise.get([world_x * 0.01, world_z * 0.01]) * 5.0;

                height = continental + regional + detail + 50.0; // Base height at 50m

                // Apply erosion simulation
                height = self.apply_erosion(height, world_x, world_z);

                heights[x][z] = height;
            }
        }

        HeightMap {
            data: heights.clone(),
            min_height: heights.iter().flatten().fold(f64::INFINITY, |a, &b| a.min(b)),
            max_height: heights.iter().flatten().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
        }
    }

    fn apply_erosion(&self, height: f64, x: f64, z: f64) -> f64 {
        // Simple thermal erosion simulation
        let erosion_factor = 0.3;
        let slope_threshold = 30.0;

        // Calculate local slope (simplified)
        let dx = self.height_noise.get([x + 1.0, z]) - self.height_noise.get([x - 1.0, z]);
        let dz = self.height_noise.get([x, z + 1.0]) - self.height_noise.get([x, z - 1.0]);
        let slope = (dx * dx + dz * dz).sqrt();

        if slope > slope_threshold {
            height * (1.0 - erosion_factor * (slope / 100.0).min(1.0))
        } else {
            height
        }
    }
}

pub struct BiomeMapper {
    biome_definitions: HashMap<BiomeId, BiomeDefinition>,
}

#[derive(Debug, Clone)]
pub struct BiomeDefinition {
    pub id: BiomeId,
    pub name: String,
    pub temperature_range: (f64, f64),
    pub moisture_range: (f64, f64),
    pub height_range: (f64, f64),
    pub base_textures: Vec<TextureLayer>,
    pub vegetation_density: f32,
}

impl BiomeMapper {
    pub fn new() -> Self {
        let mut biome_definitions = HashMap::new();

        // Define biomes for Finalverse
        biome_definitions.insert(
            BiomeId::WhisperwoodGrove,
            BiomeDefinition {
                id: BiomeId::WhisperwoodGrove,
                name: "Whisperwood Grove".to_string(),
                temperature_range: (15.0, 25.0),
                moisture_range: (0.6, 0.9),
                height_range: (40.0, 80.0),
                base_textures: vec![
                    TextureLayer {
                        texture_id: "grass_lush".to_string(),
                        blend_height: 0.0,
                        blend_strength: 1.0,
                    },
                    TextureLayer {
                        texture_id: "moss".to_string(),
                        blend_height: 45.0,
                        blend_strength: 0.7,
                    },
                ],
                vegetation_density: 0.8,
            }
        );

        biome_definitions.insert(
            BiomeId::CrystallineHighlands,
            BiomeDefinition {
                id: BiomeId::CrystallineHighlands,
                name: "Crystalline Highlands".to_string(),
                temperature_range: (-5.0, 10.0),
                moisture_range: (0.2, 0.5),
                height_range: (100.0, 300.0),
                base_textures: vec![
                    TextureLayer {
                        texture_id: "stone_crystal".to_string(),
                        blend_height: 80.0,
                        blend_strength: 1.0,
                    },
                    TextureLayer {
                        texture_id: "snow".to_string(),
                        blend_height: 150.0,
                        blend_strength: 0.9,
                    },
                ],
                vegetation_density: 0.1,
            }
        );

        Self { biome_definitions }
    }

    pub fn get_biome(&self, coord: GridCoordinate, world_song: &WorldSong) -> Biome {
        // Calculate temperature and moisture at this location
        let temp_noise = Perlin::new(world_song.seed);
        let moisture_noise = Perlin::new(world_song.seed + 1000);

        let x = coord.x as f64 * 256.0 + 128.0;
        let z = coord.z as f64 * 256.0 + 128.0;

        let temperature = 15.0 + temp_noise.get([x * 0.0005, z * 0.0005]) * 20.0;
        let moisture = 0.5 + moisture_noise.get([x * 0.0003, z * 0.0003]) * 0.5;

        // Find best matching biome
        let mut best_biome = &self.biome_definitions[&BiomeId::WhisperwoodGrove];
        let mut best_score = f64::MAX;

        for (_, biome_def) in &self.biome_definitions {
            let temp_score = if temperature >= biome_def.temperature_range.0
                && temperature <= biome_def.temperature_range.1 {
                0.0
            } else {
                (temperature - (biome_def.temperature_range.0 + biome_def.temperature_range.1) / 2.0).abs()
            };

            let moisture_score = if moisture >= biome_def.moisture_range.0
                && moisture <= biome_def.moisture_range.1 {
                0.0
            } else {
                (moisture - (biome_def.moisture_range.0 + biome_def.moisture_range.1) / 2.0).abs()
            };

            let score = temp_score + moisture_score;
            if score < best_score {
                best_score = score;
                best_biome = biome_def;
            }
        }

        Biome {
            id: best_biome.id.clone(),
            definition: best_biome.clone(),
            local_temperature: temperature,
            local_moisture: moisture,
        }
    }
}

pub struct HarmonyModifier;

impl HarmonyModifier {
    pub fn apply_harmony_effects(
        &self,
        mut terrain: TerrainPatch,
        harmony_level: f32,
    ) -> TerrainPatch {
        // Harmony affects terrain generation
        if harmony_level > 0.7 {
            // High harmony: smoother terrain, more water features
            terrain.smooth_terrain(0.2);
            terrain.add_harmony_features();
        } else if harmony_level < 0.3 {
            // Low harmony: rougher terrain, corruption
            terrain.add_corruption_features();
            terrain.roughen_terrain(0.3);
        }

        terrain
    }
}

#[derive(Debug, Clone)]
pub struct TerrainPatch {
    pub heightmap: HeightMap,
    pub textures: Vec<TextureLayer>,
    pub vegetation_map: VegetationMap,
    pub water_bodies: Vec<WaterBody>,
}

impl TerrainPatch {
    fn smooth_terrain(&mut self, factor: f64) {
        // Apply smoothing filter
        let size = self.heightmap.data.len();
        let mut smoothed = vec![vec![0.0; size]; size];

        for x in 1..size-1 {
            for z in 1..size-1 {
                let mut sum = 0.0;
                for dx in -1..=1 {
                    for dz in -1..=1 {
                        sum += self.heightmap.data[(x as i32 + dx) as usize][(z as i32 + dz) as usize];
                    }
                }
                smoothed[x][z] = self.heightmap.data[x][z] * (1.0 - factor) + (sum / 9.0) * factor;
            }
        }

        self.heightmap.data = smoothed;
    }

    fn roughen_terrain(&mut self, factor: f64) {
        let noise = Perlin::new(42);
        let size = self.heightmap.data.len();

        for x in 0..size {
            for z in 0..size {
                let roughness = noise.get([x as f64 * 0.1, z as f64 * 0.1]) * factor * 10.0;
                self.heightmap.data[x][z] += roughness;
            }
        }
    }

    fn add_harmony_features(&mut self) {
        // Add peaceful water features
        self.water_bodies.push(WaterBody {
            center: Vector2::new(128.0, 128.0),
            radius: 30.0,
            water_type: WaterType::HarmonySpring,
            depth: 5.0,
        });
    }

    fn add_corruption_features(&mut self) {
        // Add corruption markers to vegetation map
        self.vegetation_map.add_corruption_zones();
    }
}

// Supporting types
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BiomeId {
    WhisperwoodGrove,
    CrystallineHighlands,
    AshenWastes,
    StarSailorExpanse,
}

#[derive(Debug, Clone)]
pub struct GridCoordinate {
    pub x: i32,
    pub z: i32,
}

impl GridCoordinate {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    pub data: Vec<Vec<f64>>,
    pub min_height: f64,
    pub max_height: f64,
}

#[derive(Debug, Clone)]
pub struct TextureLayer {
    pub texture_id: String,
    pub blend_height: f64,
    pub blend_strength: f32,
}

#[derive(Debug, Clone)]
pub struct VegetationMap {
    pub density_map: Vec<Vec<f32>>,
    pub vegetation_types: Vec<VegetationType>,
}

impl VegetationMap {
    fn add_corruption_zones(&mut self) {
        // Add areas with no vegetation (corruption)
        // Implementation details...
    }
}

#[derive(Debug, Clone)]
pub struct VegetationType {
    pub id: String,
    pub model_path: String,
    pub min_scale: f32,
    pub max_scale: f32,
}

#[derive(Debug, Clone)]
pub struct WaterBody {
    pub center: Vector2<f64>,
    pub radius: f64,
    pub water_type: WaterType,
    pub depth: f64,
}

#[derive(Debug, Clone)]
pub enum WaterType {
    River,
    Lake,
    Ocean,
    HarmonySpring,
    CorruptedPool,
}

#[derive(Debug, Clone)]
pub struct WorldSong {
    pub seed: u32,
    pub harmony_base: f32,
    pub theme: WorldTheme,
}

#[derive(Debug, Clone)]
pub enum WorldTheme {
    Natural,
    Mystical,
    Technological,
    Corrupted,
}

#[derive(Debug, Clone)]
pub struct Biome {
    pub id: BiomeId,
    pub definition: BiomeDefinition,
    pub local_temperature: f64,
    pub local_moisture: f64,
}