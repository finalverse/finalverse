// services/world-engine/src/grpc/server.rs
use tonic::{Request, Response, Status};
use std::sync::Arc;
use std::collections::HashMap;
use std::pin::Pin;
use tokio::sync::RwLock;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use crate::{WorldEngine, RegionId, PlayerAction, ActionType, Coordinates};
use finalverse_proto::world::{
    world_service_server::WorldService,
    GetWorldStateRequest, WorldStateResponse,
    StreamUpdatesRequest, WorldUpdate,
    PlayerActionRequest, ActionResponse,
    GetRegionRequest, RegionResponse,
    UpdateHarmonyRequest, UpdateHarmonyResponse,
    Region as ProtoRegion, WeatherState as ProtoWeatherState,
    WorldTime as ProtoWorldTime,
    RegionUpdate,
    WorldEvent as ProtoWorldEvent,
    world_update,
    player_action_request,
};

pub struct WorldServiceImpl {
    engine: Arc<WorldEngine>,
    update_channels: Arc<RwLock<HashMap<String, tokio::sync::mpsc::Sender<WorldUpdate>>>>,
}

impl WorldServiceImpl {
    pub fn new(engine: Arc<WorldEngine>) -> Self {
        Self {
            engine,
            update_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl WorldService for WorldServiceImpl {
    type StreamWorldUpdatesStream = WorldUpdateStream;
    async fn get_world_state(
        &self,
        request: Request<GetWorldStateRequest>,
    ) -> Result<Response<WorldStateResponse>, Status> {
        let req = request.into_inner();
        let world_state = self.engine.get_state().await;

        let regions: Vec<ProtoRegion> = if req.region_ids.is_empty() {
            // Return all regions
            world_state.regions.values()
                .map(|r| region_to_proto(r))
                .collect()
        } else {
            // Return specific regions
            req.region_ids.iter()
                .filter_map(|id| {
                    uuid::Uuid::parse_str(id).ok()
                        .and_then(|u| world_state.regions.get(&RegionId(u)))
                        .map(|r| region_to_proto(r))
                })
                .collect()
        };

        let response = WorldStateResponse {
            regions,
            global_harmony: world_state.global_harmony,
            active_events: world_state.active_events.iter()
                .map(|e| event_to_proto(e))
                .collect(),
            time: Some(ProtoWorldTime {
                day: world_state.time.day,
                hour: world_state.time.hour,
            }),
        };

        Ok(Response::new(response))
    }

    async fn stream_world_updates(
        &self,
        request: Request<StreamUpdatesRequest>,
    ) -> Result<Response<Self::StreamWorldUpdatesStream>, Status> {
        let req = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // Store the channel for this player
        self.update_channels.write().await
            .insert(req.player_id.clone(), tx.clone());

        // Start update task
        let engine = self.engine.clone();
        let player_id = req.player_id.clone();
        let region_ids = req.region_ids;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                // Get current state
                let state = engine.get_state().await;

                // Send region updates
                for region_id in &region_ids {
                    if let Ok(uuid) = uuid::Uuid::parse_str(region_id) {
                        if let Some(region) = state.regions.get(&RegionId(uuid)) {
                            let update = WorldUpdate {
                                update: Some(world_update::Update::RegionUpdate(RegionUpdate {
                                region_id: region.id.0.to_string(),
                                harmony_level: region.harmony_level as f32,
                                discord_level: region.discord_level as f32,
                                weather: Some(weather_to_proto(&region.weather)),
                            })),
                            };

                            if tx.send(update).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        });

        let stream = ReceiverStream::new(rx).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::StreamWorldUpdatesStream))
    }

    async fn process_action(
        &self,
        request: Request<PlayerActionRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        let req = request.into_inner();

        let action = match req.action {
            Some(player_action_request::Action::Move(move_action)) => {
                ActionType::Move(Coordinates {
                    x: move_action.position.as_ref().map(|p| p.x as f64).unwrap_or(0.0),
                    y: move_action.position.as_ref().map(|p| p.y as f64).unwrap_or(0.0),
                    z: move_action.position.as_ref().map(|p| p.z as f64).unwrap_or(0.0),
                })
            }
            Some(player_action_request::Action::Interact(interact)) => {
                ActionType::Interact(interact.target_id)
            }
            Some(player_action_request::Action::Ability(ability)) => {
                ActionType::UseAbility(ability.ability_id)
            }
            Some(player_action_request::Action::Craft(craft)) => {
                ActionType::Craft(craft.item_id)
            }
            None => return Err(Status::invalid_argument("No action specified")),
        };

        let player_action = PlayerAction {
            player_id: crate::PlayerId(req.player_id),
            action,
            timestamp: req.timestamp,
        };

        self.engine.process_action(player_action).await;

        Ok(Response::new(ActionResponse {
            success: true,
            message: "Action processed".to_string(),
            effects: vec![],
        }))
    }

    async fn get_region(
        &self,
        request: Request<GetRegionRequest>,
    ) -> Result<Response<RegionResponse>, Status> {
        let id_str = request.into_inner().region_id;
        let uuid = uuid::Uuid::parse_str(&id_str)
            .map_err(|_| Status::invalid_argument("Invalid region id"))?;
        let region_id = RegionId(uuid);

        if let Some(region) = self.engine.metabolism().get_region(&region_id).await {
            Ok(Response::new(RegionResponse {
                region: Some(region_to_proto(&region)),
            }))
        } else {
            Err(Status::not_found("Region not found"))
        }
    }

    async fn update_harmony(
        &self,
        request: Request<UpdateHarmonyRequest>,
    ) -> Result<Response<UpdateHarmonyResponse>, Status> {
        let req = request.into_inner();
        let uuid = uuid::Uuid::parse_str(&req.region_id)
            .map_err(|_| Status::invalid_argument("Invalid region id"))?;
        let region_id = RegionId(uuid);

        // Update harmony through the engine
        let update_result = self.engine.update_region_harmony(&region_id, req.delta).await
            .map_err(|e| Status::internal(format!("Failed to update harmony: {}", e)))?;

        Ok(Response::new(UpdateHarmonyResponse {
            new_harmony_level: update_result.new_harmony_level,
            triggered_events: update_result.triggered_events.iter()
                .map(|e| event_to_proto(e))
                .collect(),
        }))
    }
}

// Conversion functions
fn region_to_proto(region: &crate::RegionState) -> ProtoRegion {
    ProtoRegion {
        id: region.id.0.to_string(),
        name: format!("Region {}", region.id.0), // You might want to add name to RegionState
        harmony_level: region.harmony_level as f32,
        discord_level: region.discord_level as f32,
        terrain_type: format!("{:?}", region.terrain_type),
        weather: Some(weather_to_proto(&region.weather)),
        grid_coords: vec![], // Add if needed
    }
}

fn weather_to_proto(weather: &crate::WeatherState) -> ProtoWeatherState {
    ProtoWeatherState {
        weather_type: format!("{:?}", weather.weather_type),
        intensity: weather.intensity as f32,
        wind_direction: weather.wind_direction as f32,
        wind_speed: weather.wind_speed as f32,
    }
}

fn event_to_proto(_event: &crate::WorldEvent) -> ProtoWorldEvent {
    // Convert internal event to proto event
    // This is a simplified version - expand based on your needs
    ProtoWorldEvent {
        event: None, // Implement full conversion
    }
}

pub type WorldUpdateStream = Pin<Box<dyn Stream<Item = Result<WorldUpdate, Status>> + Send + 'static>>;
