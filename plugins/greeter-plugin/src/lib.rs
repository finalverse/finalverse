// plugins/greeter-plugin/src/lib.rs
use async_trait::async_trait;
use fv_plugin::{Plugin, PluginManifest, PluginError, PluginCapability};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GreeterPlugin {
    manifest: PluginManifest,
    greeting_count: Arc<RwLock<u64>>,
    greeting_history: Arc<RwLock<Vec<GreetingRecord>>>,
}

#[derive(Clone, Debug, serde::Serialize)]
struct GreetingRecord {
    timestamp: chrono::DateTime<chrono::Utc>,
    name: String,
    message: String,
}

impl GreeterPlugin {
    pub fn new() -> Self {
        let manifest = PluginManifest {
            name: "greeter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Finalverse Team".to_string(),
            description: "A friendly greeter plugin that demonstrates the plugin system".to_string(),
            capabilities: vec![
                PluginCapability::Command("greet".to_string()),
                PluginCapability::Command("farewell".to_string()),
                PluginCapability::Command("stats".to_string()),
                PluginCapability::Command("history".to_string()),
                PluginCapability::Event("player_joined".to_string()),
            ],
            dependencies: vec![],
        };

        Self {
            manifest,
            greeting_count: Arc::new(RwLock::new(0)),
            greeting_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn record_greeting(&self, name: String, message: String) {
        let record = GreetingRecord {
            timestamp: chrono::Utc::now(),
            name,
            message: message.clone(),
        };

        self.greeting_history.write().await.push(record);

        // Keep only last 100 greetings
        let mut history = self.greeting_history.write().await;
        if history.len() > 100 {
            history.drain(0..history.len() - 100);
        }
    }
}

impl Default for GreeterPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for GreeterPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        println!("🎉 {} v{} initialized!", self.manifest.name, self.manifest.version);
        println!("   Author: {}", self.manifest.author);
        println!("   Description: {}", self.manifest.description);
        println!("   Capabilities: {} commands, {} events",
                 self.manifest.capabilities.iter()
                     .filter(|c| matches!(c, PluginCapability::Command(_)))
                     .count(),
                 self.manifest.capabilities.iter()
                     .filter(|c| matches!(c, PluginCapability::Event(_)))
                     .count()
        );
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        let count = *self.greeting_count.read().await;
        let history_count = self.greeting_history.read().await.len();
        println!("👋 {} shutting down after {} greetings ({} in history)!",
                 self.manifest.name, count, history_count);
        Ok(())
    }

    async fn handle_command(&self, command: &str, args: Value) -> Result<Value, PluginError> {
        match command {
            "greet" => {
                let name = args.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("World");

                let lang = args.get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("en");

                let style = args.get("style")
                    .and_then(|v| v.as_str())
                    .unwrap_or("normal");

                let greeting = match (lang, style) {
                    ("es", "formal") => format!("Estimado/a {}, es un honor saludarle.", name),
                    ("es", _) => format!("¡Hola, {}!", name),
                    ("fr", "formal") => format!("Bonjour {}, c'est un plaisir de vous rencontrer.", name),
                    ("fr", _) => format!("Salut, {} !", name),
                    ("de", "formal") => format!("Guten Tag, {}. Es ist mir eine Ehre.", name),
                    ("de", _) => format!("Hallo, {}!", name),
                    ("ja", "formal") => format!("{}様、はじめまして。", name),
                    ("ja", _) => format!("こんにちは、{}さん！", name),
                    ("zh", _) => format!("你好，{}！", name),
                    ("it", _) => format!("Ciao, {}!", name),
                    ("pt", _) => format!("Olá, {}!", name),
                    ("ru", _) => format!("Привет, {}!", name),
                    (_, "epic") => format!("Hail, {}! Your presence brings light to these digital realms!", name),
                    (_, "pirate") => format!("Ahoy there, {} ye scallywag!", name),
                    (_, "robot") => format!("GREETINGS, {}. SOCIAL PROTOCOL INITIATED.", name),
                    (_, "medieval") => format!("Well met, good {}! May thy journey be prosperous!", name),
                    _ => format!("Hello, {}!", name),
                };

                // Increment greeting count
                let mut count = self.greeting_count.write().await;
                *count += 1;
                let greeting_number = *count;

                // Record greeting
                self.record_greeting(name.to_string(), greeting.clone()).await;

                Ok(serde_json::json!({
                    "message": greeting,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "greeting_number": greeting_number,
                    "language": lang,
                    "style": style,
                }))
            }

            "farewell" => {
                let name = args.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Friend");

                let style = args.get("style")
                    .and_then(|v| v.as_str())
                    .unwrap_or("casual");

                let farewell = match style {
                    "formal" => format!("Farewell, {}. Until we meet again.", name),
                    "pirate" => format!("Fair winds and following seas, {} me hearty!", name),
                    "robot" => format!("GOODBYE, {}. TERMINATING SOCIAL INTERACTION.", name),
                    "medieval" => format!("Fare thee well, good {}! May fortune smile upon thee!", name),
                    "epic" => format!("May your path be ever lit by starlight, {}!", name),
                    "sad" => format!("I'll miss you, {}... Please come back soon!", name),
                    _ => format!("See you later, {}!", name),
                };

                self.record_greeting(name.to_string(), farewell.clone()).await;

                Ok(serde_json::json!({
                    "message": farewell,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }))
            }

            "stats" => {
                let count = *self.greeting_count.read().await;
                let history_count = self.greeting_history.read().await.len();

                Ok(serde_json::json!({
                    "total_greetings": count,
                    "greetings_in_history": history_count,
                    "plugin_version": self.manifest.version,
                    "plugin_name": self.manifest.name,
                    "uptime_message": format!("Greeter has said hello {} times!", count),
                    "capabilities": self.manifest.capabilities.len(),
                }))
            }

            "history" => {
                let limit = args.get("limit")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(10) as usize;

                let history = self.greeting_history.read().await;
                let recent: Vec<_> = history.iter()
                    .rev()
                    .take(limit)
                    .map(|record| serde_json::json!({
                        "timestamp": record.timestamp.to_rfc3339(),
                        "name": record.name,
                        "message": record.message,
                    }))
                    .collect();

                Ok(serde_json::json!({
                    "recent_greetings": recent,
                    "total_in_history": history.len(),
                }))
            }

            _ => Err(PluginError::InvalidCommand(command.to_string())),
        }
    }

    async fn handle_event(&self, event: Value) -> Result<(), PluginError> {
        if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
            match event_type {
                "player_joined" => {
                    if let Some(player_name) = event.get("player_name").and_then(|v| v.as_str()) {
                        println!("🎮 Player {} joined! Auto-greeting...", player_name);

                        // Simulate sending a greeting
                        let greeting = format!("Welcome to Finalverse, {}! Type /help for commands.", player_name);
                        self.record_greeting(player_name.to_string(), greeting.clone()).await;

                        // In a real implementation, this would send the greeting through the game's chat system
                        println!("   Sent: {}", greeting);
                    }
                }
                _ => {
                    // Ignore other events
                }
            }
        }
        Ok(())
    }

    fn supports_capability(&self, capability: &PluginCapability) -> bool {
        self.manifest.capabilities.contains(capability)
    }
}

// Plugin entry point for dynamic loading
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    let plugin = GreeterPlugin::new();
    Box::into_raw(Box::new(plugin) as Box<dyn Plugin>)
}

// Cargo.toml for greeter-plugin:
/*
[package]
name = "greeter-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
fv-plugin = { path = "../../crates/fv-plugin" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["sync", "rt"] }

[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
*/