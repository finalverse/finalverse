// services/realtime-gateway/src/main.rs
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use warp::Filter;
use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessage {
    pub id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
    pub id: String,
    pub event: String,
    pub payload: serde_json::Value,
}

// WebSocket plugin trait - removed Clone requirement
#[async_trait::async_trait]
pub trait WebSocketPlugin: Send + Sync {
    fn name(&self) -> &str;
    async fn handle_message(&self, client_id: &str, message: ClientMessage) -> Option<ServerMessage>;
    async fn on_connect(&self, client_id: &str);
    async fn on_disconnect(&self, client_id: &str);
}

// Plugin registry using Arc instead of Box to avoid Clone issues
pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn WebSocketPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Arc<dyn WebSocketPlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn WebSocketPlugin>> {
        self.plugins.get(name).cloned()
    }
}

// Client connection manager
pub struct ConnectionManager {
    clients: Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, client_id: String, tx: tokio::sync::mpsc::UnboundedSender<Message>) {
        self.clients.write().await.insert(client_id, tx);
    }

    pub async fn remove_client(&self, client_id: &str) {
        self.clients.write().await.remove(client_id);
    }

    pub async fn send_to_client(&self, client_id: &str, message: Message) -> Result<(), String> {
        let clients = self.clients.read().await;
        if let Some(tx) = clients.get(client_id) {
            tx.send(message).map_err(|_| "Failed to send message".to_string())
        } else {
            Err("Client not found".to_string())
        }
    }

    pub async fn broadcast(&self, message: Message) {
        let clients = self.clients.read().await;
        for (_, tx) in clients.iter() {
            let _ = tx.send(message.clone());
        }
    }
}

async fn handle_websocket(
    ws: WebSocket,
    clients: Arc<ConnectionManager>,
    plugins: Arc<RwLock<PluginRegistry>>,
) {
    let client_id = Uuid::new_v4().to_string();
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Add client to connection manager
    clients.add_client(client_id.clone(), tx).await;

    // Notify plugins of new connection
    {
        let registry = plugins.read().await;
        for (_, plugin) in &registry.plugins {
            plugin.on_connect(&client_id).await;
        }
    }

    // Spawn task to handle outgoing messages
    let client_id_clone = client_id.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                        // Route message to appropriate plugin
                        let registry = plugins.read().await;
                        for (_, plugin) in &registry.plugins {
                            if let Some(response) = plugin.handle_message(&client_id, client_msg.clone()).await {
                                let response_text = serde_json::to_string(&response).unwrap();
                                let _ = clients.send_to_client(&client_id, Message::text(response_text)).await;
                            }
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }

    // Clean up on disconnect
    clients.remove_client(&client_id).await;

    // Notify plugins of disconnect
    {
        let registry = plugins.read().await;
        for (_, plugin) in &registry.plugins {
            plugin.on_disconnect(&client_id).await;
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let clients = Arc::new(ConnectionManager::new());
    let plugins = Arc::new(RwLock::new(PluginRegistry::new()));

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || clients.clone()))
        .and(warp::any().map(move || plugins.clone()))
        .map(|ws: warp::ws::Ws, clients, plugins| {
            ws.on_upgrade(move |websocket| handle_websocket(websocket, clients, plugins))
        });

    // Health check endpoint
    let health_route = warp::path("health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let routes = ws_route.or(health_route);

    println!("🌐 Realtime Gateway starting on port 3000");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3000))
        .await;
}

// Example plugin implementation
pub struct EchoPlugin;

#[async_trait::async_trait]
impl WebSocketPlugin for EchoPlugin {
    fn name(&self) -> &str {
        "echo"
    }

    async fn handle_message(&self, _client_id: &str, message: ClientMessage) -> Option<ServerMessage> {
        Some(ServerMessage {
            id: message.id,
            event: "echo".to_string(),
            payload: message.payload,
        })
    }

    async fn on_connect(&self, client_id: &str) {
        println!("Client {} connected to echo plugin", client_id);
    }

    async fn on_disconnect(&self, client_id: &str) {
        println!("Client {} disconnected from echo plugin", client_id);
    }
}