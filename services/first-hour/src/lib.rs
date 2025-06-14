// services/first-hour/src/lib.rs
pub mod scenes;
pub mod first_hour_manager;
pub mod echo_spawner;
mod world_client;

use world_client::WorldEngineClient;
use first_hour_manager::FirstHourSceneManager;

use finalverse_world3d::{Position3D, GridCoordinate};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct FirstHourConfig {
    pub redis_url: String,
    pub world_engine_url: String,
    pub starting_grid: GridCoordinate,
}

impl FirstHourConfig {
    /// Load configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        let world_engine_url = std::env::var("WORLD_ENGINE_URL").unwrap_or_else(|_| "http://localhost:50052".to_string());
        Self {
            redis_url,
            world_engine_url,
            starting_grid: GridCoordinate::new(100, 100),
        }
    }
}

pub struct FirstHourService {
    config: FirstHourConfig,
    world_client: WorldEngineClient,
    scene_manager: Arc<RwLock<FirstHourSceneManager>>,
}

impl FirstHourService {
    pub async fn new(config: FirstHourConfig) -> anyhow::Result<Self> {
        let world_client = WorldEngineClient::connect(&config.world_engine_url).await?;
        let scene_manager = Arc::new(RwLock::new(FirstHourSceneManager::new()));

        Ok(Self {
            config,
            world_client,
            scene_manager,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        // Initialize first hour scenes
        self.initialize_scenes().await?;

        // Start event listeners
        self.start_event_listeners().await?;

        // Keep service running
        tokio::signal::ctrl_c().await?;
        Ok(())
    }

    async fn initialize_scenes(&self) -> anyhow::Result<()> {
        let mut manager = self.scene_manager.write().await;

        // Request world-engine to generate the first hour grids
        let grids = vec![
            GridCoordinate::new(100, 100), // Memory Grotto
            GridCoordinate::new(101, 101), // Weaver's Landing
            GridCoordinate::new(102, 101), // Whisperwood Grove
        ];

        for grid in grids {
            self.world_client.request_grid_generation(
                grid,
                "terra_nova",
                Some("first_hour_biome")
            ).await?;
        }

        // Set up specific first hour entities
        manager.setup_memory_grotto().await?;
        manager.setup_weavers_landing().await?;
        manager.setup_whisperwood_grove().await?;

        Ok(())
    }

    async fn start_event_listeners(&self) -> anyhow::Result<()> {
        // Placeholder for future integration with a message bus or websocket
        // events from the client.
        Ok(())
    }
}
