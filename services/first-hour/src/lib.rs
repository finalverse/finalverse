// services/first-hour/src/lib.rs
pub mod scenes;
pub mod first_hour_manager;
pub mod echo_spawner;
pub mod interactive_objects;
pub mod world_client;

use finalverse_world3d::{Position3D, GridCoordinate};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::codegen::tokio_stream::StreamExt;
// Re-export for easier access
pub use first_hour_manager::FirstHourSceneManager;
pub use world_client::WorldEngineClient;

#[derive(Clone)]
pub struct FirstHourConfig {
    pub redis_url: String,
    pub world_engine_url: String,
    pub starting_grid: GridCoordinate,
}

impl FirstHourConfig {
    pub fn from_env() -> Self {
        Self {
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            world_engine_url: std::env::var("WORLD_ENGINE_URL")
                .unwrap_or_else(|_| "http://localhost:50051".to_string()),
            starting_grid: GridCoordinate::new(100, 100),
        }
    }
}

pub struct FirstHourService {
    config: FirstHourConfig,
    world_client: WorldEngineClient,
    scene_manager: Arc<RwLock<FirstHourSceneManager>>,
    redis_client: redis::Client,
}

impl FirstHourService {
    pub async fn new(config: FirstHourConfig) -> anyhow::Result<Self> {
        let world_client = WorldEngineClient::connect(&config.world_engine_url).await?;
        let scene_manager = Arc::new(RwLock::new(FirstHourSceneManager::new()));
        let redis_client = redis::Client::open(config.redis_url.clone())?;

        Ok(Self {
            config,
            world_client,
            scene_manager,
            redis_client,
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
            // For now, we'll just prepare the scenes
            // In production, this would communicate with world-engine
            tracing::info!("Preparing grid {:?}", grid);
        }

        // Set up specific first hour entities
        manager.setup_memory_grotto().await?;
        manager.setup_weavers_landing().await?;
        manager.setup_whisperwood_grove().await?;

        Ok(())
    }

    async fn start_event_listeners(&self) -> anyhow::Result<()> {
        // Start Redis event listener for player actions
        let scene_manager = self.scene_manager.clone();
        let redis_client = self.redis_client.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::listen_for_events(redis_client, scene_manager).await {
                tracing::error!("Event listener error: {}", e);
            }
        });

        Ok(())
    }

    async fn listen_for_events(
        redis_client: redis::Client,
        scene_manager: Arc<RwLock<FirstHourSceneManager>>,
    ) -> anyhow::Result<()> {
        use redis::AsyncCommands;

        let mut con = redis_client.get_async_connection().await?;
        let mut pubsub = con.into_pubsub();

        pubsub.subscribe("first_hour:events").await?;

        let mut stream = pubsub.into_on_message();

        while let Some(msg) = stream.next().await {
            // Process events
            let payload: String = msg.get_payload()?;
            tracing::debug!("Received event: {}", payload);

            // Parse and handle events
            if let Ok(event) = serde_json::from_str::<PlayerEvent>(&payload) {
                let mut manager = scene_manager.write().await;
                if let Err(e) = manager.handle_player_event(event).await {
                    tracing::error!("Error handling event: {}", e);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerEvent {
    pub event_type: String,
    pub player_id: String,
    pub data: serde_json::Value,
}