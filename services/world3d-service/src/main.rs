// services/world3d-service/src/main.rs
mod spatial_streaming;
mod world_manager;
mod terrain_service;

use finalverse_world3d::{
    Position3D, GridCoordinate, PlayerId,
    world::World,
    region::Region,
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, error};

pub struct World3DService {
    world_manager: Arc<world_manager::WorldManager>,
    spatial_streamer: Arc<spatial_streaming::SpatialStreamManager>,
    terrain_service: Arc<terrain_service::TerrainService>,
}

impl World3DService {
    pub async fn new() -> anyhow::Result<Self> {
        let world_manager = Arc::new(world_manager::WorldManager::new().await?);
        let spatial_streamer = Arc::new(spatial_streaming::SpatialStreamManager::new());
        let terrain_service = Arc::new(terrain_service::TerrainService::new());

        Ok(Self {
            world_manager,
            spatial_streamer,
            terrain_service,
        })
    }

    pub async fn initialize_first_hour_world(&self) -> anyhow::Result<()> {
        info!("Initializing First Hour 3D world...");

        // Load Terra Nova world
        self.world_manager.create_terra_nova_world().await?;

        // Initialize first hour grids
        let first_hour_grids = vec![
            GridCoordinate::new(100, 100), // Memory Grotto
            GridCoordinate::new(101, 101), // Weaver's Landing
            GridCoordinate::new(102, 101), // Whisperwood Grove
        ];

        for grid_coord in first_hour_grids {
            self.world_manager.ensure_grid_loaded(grid_coord).await?;
        }

        info!("First Hour 3D world initialized successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let service = World3DService::new().await?;
    service.initialize_first_hour_world().await?;

    // Start gRPC server
    let addr = "[::1]:50053".parse()?;
    info!("World 3D Service listening on {}", addr);

    Server::builder()
        .add_service(world3d_server::World3DStreamServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}