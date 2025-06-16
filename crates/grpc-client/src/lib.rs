// crates/grpc-client/src/lib.rs
use tonic::transport::{Channel, Endpoint};
use std::time::Duration;
use finalverse_proto::world::world_service_client::WorldServiceClient;
use finalverse_proto::story::story_service_client::StoryServiceClient;

#[derive(Clone)]
pub struct FinalverseGrpcClient {
    pub world: WorldServiceClient<Channel>,
    pub story: StoryServiceClient<Channel>,
}

impl FinalverseGrpcClient {
    pub async fn connect(
        world_addr: &str,
        story_addr: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let world_channel = create_channel(world_addr).await?;
        let story_channel = create_channel(story_addr).await?;

        Ok(Self {
            world: WorldServiceClient::new(world_channel),
            story: StoryServiceClient::new(story_channel),
        })
    }
}

async fn create_channel(addr: &str) -> Result<Channel, tonic::transport::Error> {
    Endpoint::from_shared(addr.to_string())?
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .connect()
        .await
}

// Convenience functions for common operations
pub mod helpers {
    use super::*;
    use finalverse_proto::world::*;

    pub async fn get_player_region(
        client: &mut WorldServiceClient<Channel>,
        player_id: &str,
    ) -> Result<Option<Region>, Box<dyn std::error::Error>> {
        // Implementation to get player's current region
        Ok(None)
    }

    pub async fn move_player(
        client: &mut WorldServiceClient<Channel>,
        player_id: &str,
        position: (f32, f32, f32),
    ) -> Result<ActionResponse, Box<dyn std::error::Error>> {
        let request = PlayerActionRequest {
            player_id: player_id.to_string(),
            action: Some(player_action_request::Action::Move(MoveAction {
                position: Some(Position3D {
                    x: position.0,
                    y: position.1,
                    z: position.2,
                }),
            })),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        let response = client.process_action(request).await?;
        Ok(response.into_inner())
    }
}