// plugins/greeter-plugin/src/lib.rs
use async_trait::async_trait;
use finalverse_plugin::ServicePlugin;
use service_registry::LocalServiceRegistry;
use axum::Router as AxumRouter;
use tonic::transport::server::Router as GrpcRouter;
use serde_json::Value;
use serde::de::Error as SerdeError;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GreeterPlugin {
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
        Self {
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
        let len = history.len();
        if len > 100 {
            history.drain(0..len - 100);
        }
    }
}

impl Default for GreeterPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ServicePlugin for GreeterPlugin {
    fn name(&self) -> &'static str {
        "greeter"
    }

    async fn routes(&self) -> AxumRouter {
        // In a real plugin we would expose HTTP routes here.
        AxumRouter::new()
    }

    async fn init(&self, _registry: &LocalServiceRegistry) -> anyhow::Result<()> {
        println!("🎉 greeter plugin initialized");
        Ok(())
    }

    fn register_grpc(self: Box<Self>, server: GrpcRouter) -> GrpcRouter {
        server
    }
}

impl GreeterPlugin {
    async fn handle_command_internal(&self, command: &str, args: Value) -> serde_json::Result<Value> {
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
                    "uptime_message": format!("Greeter has said hello {} times!", count)
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
            _ => Err(SerdeError::custom("invalid command")),
        }
    }
}

// Plugin entry point for dynamic loading
#[no_mangle]
pub extern "C" fn finalverse_plugin_entry() -> *mut dyn ServicePlugin {
    let plugin = GreeterPlugin::new();
    Box::into_raw(Box::new(plugin) as Box<dyn ServicePlugin>)
}

// Cargo.toml for greeter-plugin:
/*
[package]
name = "greeter-plugin"
version = "0.1.1"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
finalverse-plugin = { path = "../../crates/plugin" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["sync", "rt"] }

[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
*/