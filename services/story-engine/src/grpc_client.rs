// services/story-engine/src/grpc_client.rs
use finalverse_grpc_client::FinalverseGrpcClient;
use finalverse_proto::world::*;

pub struct WorldEngineClient {
    client: FinalverseGrpcClient,
}

impl WorldEngineClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = FinalverseGrpcClient::connect(
            "http://[::1]:50051",  // World engine
            "http://[::1]:50052",  // Story engine (self)
        ).await?;

        Ok(Self { client })
    }

    pub async fn get_region_for_story(
        &mut self,
        region_id: &str,
    ) -> Result<Option<Region>, Box<dyn std::error::Error>> {
        let request = GetRegionRequest {
            region_id: region_id.to_string(),
        };

        match self.client.world.get_region(request).await {
            Ok(response) => Ok(response.into_inner().region),
            Err(status) if status.code() == tonic::Code::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn notify_harmony_change(
        &mut self,
        region_id: &str,
        delta: f32,
        source: &str,
    ) -> Result<UpdateHarmonyResponse, Box<dyn std::error::Error>> {
        let request = UpdateHarmonyRequest {
            region_id: region_id.to_string(),
            delta,
            source: source.to_string(),
        };

        let response = self.client.world.update_harmony(request).await?;
        Ok(response.into_inner())
    }
}