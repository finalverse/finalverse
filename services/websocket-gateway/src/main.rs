// services/websocket-gateway/src/main.rs

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use finalverse_common::*;
use finalverse_protocol::*;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum WSMessage {
    // Client -> Server
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    PlayerUpdate { position: Coordinates },
    MelodyPerformed { melody: Melody, target: Coordinates },
    
    // Server -> Client
    Welcome { player_id: String },
    HarmonyUpdate { region_id: String, harmony_level: f32 },
    PlayerPresence { player_id: String, position: Coordinates, action: String },
    EchoMovement { echo_id: String, position: Coordinates },
    WeatherChange { region_id: String, weather: String },
    EventNotification { event: FinalverseEvent },
    Error { message: String },
}

#[derive(Clone)]
struct WebSocketState {
    // Broadcast channels for different event types
    harmony_tx: broadcast::Sender<WSMessage>,
    presence_tx: broadcast::Sender<WSMessage>,
    event_tx: broadcast::Sender<WSMessage>,
    
    // Connected clients
    clients: Arc<RwLock<HashMap<String, ClientInfo>>>,
    
    // Service connections
    event_bus: Arc<dyn EventBus>,
}

struct ClientInfo {
    player_id: PlayerId,
    subscribed_channels: Vec<String>,
}

impl WebSocketState {
    fn new() -> Self {
        let (harmony_tx, _) = broadcast::channel(100);
        let (presence_tx, _) = broadcast::channel(100);
        let (event_tx, _) = broadcast::channel(100);
        
        Self {
            harmony_tx,
            presence_tx,
            event_tx,
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_bus: Arc::new(InMemoryEventBus::new()),
        }
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: WebSocketState) {
    let player_id = PlayerId(uuid::Uuid::new_v4());
    let client_id = player_id.0.to_string();
    
    // Send welcome message
    let welcome = WSMessage::Welcome {
        player_id: client_id.clone(),
    };
    
    if socket
        .send(Message::Text(serde_json::to_string(&welcome).unwrap()))
        .await
        .is_err()
    {
        return;
    }
    
    // Register client
    {
        let mut clients = state.clients.write().await;
        clients.insert(
            client_id.clone(),
            ClientInfo {
                player_id: player_id.clone(),
                subscribed_channels: vec!["global".to_string()],
            },
        );
    }
    
    // Set up broadcast receivers
    let mut harmony_rx = state.harmony_tx.subscribe();
    let mut presence_rx = state.presence_tx.subscribe();
    let mut event_rx = state.event_tx.subscribe();
    
    // Spawn task to handle broadcasts
    let broadcast_task = tokio::spawn({
        let mut socket_sender = socket.clone();
        async move {
            loop {
                tokio::select! {
                    Ok(msg) = harmony_rx.recv() => {
                        if let Ok(text) = serde_json::to_string(&msg) {
                            let _ = socket_sender.send(Message::Text(text)).await;
                        }
                    }
                    Ok(msg) = presence_rx.recv() => {
                        if let Ok(text) = serde_json::to_string(&msg) {
                            let _ = socket_sender.send(Message::Text(text)).await;
                        }
                    }
                    Ok(msg) = event_rx.recv() => {
                        if let Ok(text) = serde_json::to_string(&msg) {
                            let _ = socket_sender.send(Message::Text(text)).await;
                        }
                    }
                }
            }
        }
    });
    
    // Handle incoming messages
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(ws_msg) = serde_json::from_str::<WSMessage>(&text) {
                    handle_client_message(ws_msg, &state, &player_id).await;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
    
    // Cleanup
    broadcast_task.abort();
    state.clients.write().await.remove(&client_id);
    
    info!("WebSocket client {} disconnected", client_id);
}

async fn handle_client_message(msg: WSMessage, state: &WebSocketState, player_id: &PlayerId) {
    match msg {
        WSMessage::Subscribe { channels } => {
            let mut clients = state.clients.write().await;
            if let Some(client) = clients.get_mut(&player_id.0.to_string()) {
                client.subscribed_channels.extend(channels);
            }
        }
        
        WSMessage::PlayerUpdate { position } => {
            let presence_msg = WSMessage::PlayerPresence {
                player_id: player_id.0.to_string(),
                position,
                action: "moving".to_string(),
            };
            let _ = state.presence_tx.send(presence_msg);
        }
        
        WSMessage::MelodyPerformed { melody, target } => {
            // Publish to event bus
            let event = FinalverseEvent::MelodyPerformed {
                player: player_id.clone(),
                melody,
                target,
            };
            let _ = state.event_bus.publish(event).await;
            
            // Broadcast to nearby players
            let presence_msg = WSMessage::PlayerPresence {
                player_id: player_id.0.to_string(),
                position: target,
                action: "performing_melody".to_string(),
            };
            let _ = state.presence_tx.send(presence_msg);
        }
        
        _ => {}
    }
}

// Background task to monitor world changes
async fn world_monitor_task(state: WebSocketState) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    let client = reqwest::Client::new();
    
    loop {
        interval.tick().await;
        
        // Poll world engine for changes
        if let Ok(response) = client.get("http://localhost:3002/regions").send().await {
            if let Ok(data) = response.json::<serde_json::Value>().await {
                if let Some(regions) = data["regions"].as_array() {
                    for region in regions {
                        if let (Some(id), Some(harmony)) = (
                            region["id"].as_str(),
                            region["harmony_level"].as_f64(),
                        ) {
                            let msg = WSMessage::HarmonyUpdate {
                                region_id: id.to_string(),
                                harmony_level: harmony as f32,
                            };
                            let _ = state.harmony_tx.send(msg);
                        }
                    }
                }
            }
        }
    }
}

// Background task to monitor events
async fn event_monitor_task(state: WebSocketState) {
    let mut receiver = state.event_bus.subscribe("websocket-gateway").await.unwrap();
    
    while let Some(event) = receiver.recv().await {
        let msg = WSMessage::EventNotification { event };
        let _ = state.event_tx.send(msg);
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_service_info() -> impl IntoResponse {
    Json(ServiceInfo {
        name: "websocket-gateway".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: ServiceStatus::Healthy,
        uptime_seconds: 0, // TODO: Track uptime
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let state = WebSocketState::new();
    
    // Start background tasks
    let monitor_state = state.clone();
    tokio::spawn(world_monitor_task(monitor_state));
    
    let event_state = state.clone();
    tokio::spawn(event_monitor_task(event_state));
    
    // Build router
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/health", get(health_check))
        .route("/info", get(get_service_info))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3007));
    info!("WebSocket Gateway listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}