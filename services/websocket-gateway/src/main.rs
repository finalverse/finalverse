use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use fv_common::{
    events::{FinalverseEvent, HarmonyEvent, SongEvent},
    types::{Coordinates, EchoId, Melody, PlayerId, RegionId},
};
use futures::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use reqwest;
use health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WSMessage {
    // Player Actions
    SongweavingPerformed {
        melody: Melody,
        target: Coordinates,
    },
    EchoInteraction {
        echo_id: EchoId,
        interaction_type: String,
    },
    // Server Updates
    WorldUpdate {
        region: RegionId,
        harmony_level: f32,
    },
    EventNotification {
        event: FinalverseEvent,
    },
    // Connection
    Connected {
        player_id: PlayerId,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone)]
pub struct GameState {
    players: HashMap<PlayerId, PlayerSession>,
    harmony_levels: HashMap<RegionId, f32>,
}

#[derive(Debug, Clone)]
pub struct PlayerSession {
    player_id: PlayerId,
    current_region: RegionId,
    sender: Option<mpsc::UnboundedSender<WSMessage>>,
}

type SharedGameState = Arc<RwLock<GameState>>;

impl GameState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            harmony_levels: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
struct ServiceInfo {
    name: String,
    version: String,
    status: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedGameState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: SharedGameState) {
    let (sender, receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Generate a unique player ID
    let player_id = PlayerId(Uuid::new_v4().to_string());

    // Add player to game state
    {
        let mut game_state = state.write().unwrap();
        game_state.players.insert(
            player_id.clone(),
            PlayerSession {
                player_id: player_id.clone(),
                current_region: RegionId("terra_nova".to_string()),
                sender: Some(tx.clone()),
            },
        );
    }

    // Send connection confirmation
    let _ = tx.send(WSMessage::Connected {
        player_id: player_id.clone(),
    });

    // Spawn task to handle outgoing messages
    let mut sender = sender;
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json_msg) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json_msg)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages
    let mut receiver = receiver;
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ws_message) = serde_json::from_str::<WSMessage>(&text) {
                    handle_message(ws_message, &state, &player_id, &tx).await;
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            _ => {}
        }
    }

    // Remove player from state when disconnected
    {
        let mut game_state = state.write().unwrap();
        game_state.players.remove(&player_id);
    }
}

async fn handle_message(
    message: WSMessage,
    state: &SharedGameState,
    player_id: &PlayerId,
    tx: &mpsc::UnboundedSender<WSMessage>,
) {
    match message {
        WSMessage::SongweavingPerformed { melody, target } => {
            // Process songweaving action
            let harmony_event = HarmonyEvent::ResonanceGained {
                player_id: player_id.clone(),
                amount: 10.0,
                resonance_type: "creative".to_string(),
            };

            // Send to Song Engine
            send_to_song_engine(SongEvent::MelodyWoven {
                player_id: player_id.clone(),
                melody: melody.clone(),
                target,
            })
            .await;

            // Broadcast harmony update
            broadcast_harmony_update(state, &RegionId("terra_nova".to_string()), 0.75).await;

            // Send confirmation to player
            let _ = tx.send(WSMessage::WorldUpdate {
                region: RegionId("terra_nova".to_string()),
                harmony_level: 0.75,
            });
        }
        WSMessage::EchoInteraction {
            echo_id,
            interaction_type,
        } => {
            // Handle Echo interaction
            println!(
                "Player {} interacting with Echo {:?}: {}",
                player_id.0, echo_id, interaction_type
            );
        }
        _ => {}
    }
}

async fn send_to_song_engine(event: SongEvent) {
    // In a real implementation, this would send to the Song Engine service
    println!("Sending to Song Engine: {:?}", event);

    // For now, simulate with HTTP call
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3001/api/events")
        .json(&event)
        .send()
        .await;

    if let Ok(response) = response {
        if response.status().is_success() {
            if let Ok(data) = response.json::<serde_json::Value>().await {
                println!("Song Engine response: {:?}", data);
            }
        }
    }
}

async fn broadcast_harmony_update(state: &SharedGameState, region: &RegionId, level: f32) {
    let players = {
        let game_state = state.read().unwrap();
        game_state.players.clone()
    };

    let update_message = WSMessage::WorldUpdate {
        region: region.clone(),
        harmony_level: level,
    };

    for (_, player_session) in players {
        if let Some(sender) = &player_session.sender {
            let _ = sender.send(update_message.clone());
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(GameState::new()));
    let monitor = Arc::new(HealthMonitor::new("websocket-gateway", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("websocket-gateway".to_string(), "http://localhost:3000".to_string())
        .await;

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state.clone())
        .merge(monitor.clone().axum_routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("WebSocket Gateway listening on {}", addr);

    // Use axum::serve instead of the deprecated Server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}