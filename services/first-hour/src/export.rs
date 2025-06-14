// services/first-hour/src/export.rs
use finalverse_world3d::{
    terrain::TerrainPatch,
    Position3D,
};
use image::{ImageBuffer, Rgb};
use std::fs;
use std::path::Path;

pub struct FirstHourExporter {
    export_path: String,
}

impl FirstHourExporter {
    pub fn new(export_path: String) -> Self {
        Self { export_path }
    }

    pub async fn export_first_hour_data(&self, scenes: &HashMap<String, Grid>) -> anyhow::Result<()> {
        fs::create_dir_all(&self.export_path)?;

        for (scene_name, grid) in scenes {
            self.export_scene(scene_name, grid).await?;
        }

        // Export metadata
        self.export_metadata().await?;

        Ok(())
    }

    async fn export_scene(&self, name: &str, grid: &Grid) -> anyhow::Result<()> {
        let scene_path = Path::new(&self.export_path).join(name);
        fs::create_dir_all(&scene_path)?;

        // Export heightmap as image
        self.export_heightmap(&grid.terrain, scene_path.join("heightmap.png")).await?;

        // Export entity positions
        let entities_json = serde_json::to_string_pretty(&grid.entities)?;
        fs::write(scene_path.join("entities.json"), entities_json)?;

        // Export structures
        let structures_json = serde_json::to_string_pretty(&grid.structures)?;
        fs::write(scene_path.join("structures.json"), structures_json)?;

        Ok(())
    }

    async fn export_heightmap(&self, terrain: &TerrainPatch, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let size = terrain.heightmap.len();
        let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(size as u32, size as u32);

        for (y, row) in terrain.heightmap.iter().enumerate() {
            for (x, &height) in row.iter().enumerate() {
                let normalized = ((height - 40.0) / 60.0 * 255.0).clamp(0.0, 255.0) as u8;
                img.put_pixel(x as u32, y as u32, Rgb([normalized, normalized, normalized]));
            }
        }

        img.save(path)?;
        Ok(())
    }

    async fn export_metadata(&self) -> anyhow::Result<()> {
        let metadata = serde_json::json!({
            "version": "0.1.0",
            "scenes": [
                {
                    "name": "memory_grotto",
                    "grid": [100, 100],
                    "description": "Starting area - peaceful grotto for character creation"
                },
                {
                    "name": "weavers_landing",
                    "grid": [101, 101],
                    "description": "Main town - home to Anya and the fading community"
                },
                {
                    "name": "whisperwood_grove",
                    "grid": [102, 101],
                    "description": "Mystical forest containing the Resonant Blossom"
                }
            ],
            "key_locations": {
                "memory_grotto_pool": [128.0, 128.0, 50.0],
                "anyas_workshop": [180.0, 140.0, 52.0],
                "plaza_of_echoes": [150.0, 150.0, 51.0],
                "resonant_blossom": [210.0, 190.0, 56.0]
            }
        });

        fs::write(
            Path::new(&self.export_path).join("first_hour_metadata.json"),
            serde_json::to_string_pretty(&metadata)?
        )?;

        Ok(())
    }
}