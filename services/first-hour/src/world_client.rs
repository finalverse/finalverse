// services/first-hour/src/world_client.rs
use tonic::transport::Channel;
use finalverse_world_3d::GridCoordinate;

pub struct WorldEngineClient {
    client: world_proto::world_engine_client::WorldEngineClient<Channel>,
}

impl WorldEngineClient {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let client = world_proto::world_engine_client::WorldEngineClient::connect(url).await?;
        Ok(Self { client })
    }

    pub async fn request_grid_generation(
        &mut self,
        coord: GridCoordinate,
        world_id: &str,
        biome_hint: Option<&str>,
    ) -> anyhow::Result<()> {
        let request = world_proto::GenerateGridRequest {
            x: coord.x,
            y: coord.y,
            world_id: world_id.to_string(),
            biome_hint: biome_hint.map(|s| s.to_string()),
        };

        self.client.generate_grid(request).await?;
        Ok(())
    }
}