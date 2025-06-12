// server/src/plugin.rs
use anyhow::Result;
use std::sync::Arc;
use std::path::PathBuf;
use fv_plugin::{Plugin, PluginManager, PluginError, PluginRegistry};
use crate::service_registry::LocalServiceRegistry;
use tonic::transport::server::Router;

pub struct LoadedPlugin {
    pub plugin_id: String,
    pub plugin: Arc<dyn Plugin>,
}

impl LoadedPlugin {
    pub async fn init(&self, _registry: &LocalServiceRegistry) -> Result<()> {
        // Plugins are initialized through the PluginManager
        Ok(())
    }
}

pub async fn discover_plugins() -> Vec<LoadedPlugin> {
    let mut loaded_plugins = Vec::new();

    // Get plugin directory from environment or use default
    let plugin_dir = std::env::var("FINALVERSE_PLUGIN_DIR")
        .unwrap_or_else(|_| "target/release/plugins".to_string());

    let plugin_path = PathBuf::from(&plugin_dir);

    println!("🔌 Searching for plugins in: {}", plugin_dir);

    if !plugin_path.exists() {
        println!("⚠️  Plugin directory does not exist: {}", plugin_dir);
        return loaded_plugins;
    }

    // Create plugin manager
    let mut manager = PluginManager::new();

    // Look for plugin files
    let extension = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    if let Ok(entries) = std::fs::read_dir(&plugin_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                println!("📦 Found plugin file: {:?}", path);

                match manager.load_plugin(path.clone()) {
                    Ok(plugin_id) => {
                        println!("✅ Loaded plugin: {} (ID: {})", path.display(), plugin_id);

                        // Get the plugin instance
                        if let Some(plugin) = manager.get_plugin(&plugin_id) {
                            loaded_plugins.push(LoadedPlugin {
                                plugin_id: plugin_id.clone(),
                                plugin: plugin.clone(),
                            });

                            // Initialize the plugin
                            if let Err(e) = manager.initialize_plugin(&plugin_id).await {
                                eprintln!("❌ Failed to initialize plugin {}: {}", plugin_id, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to load plugin {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    // Store the manager globally so plugins stay loaded
    let _ = PLUGIN_MANAGER.set(Arc::new(tokio::sync::RwLock::new(manager)));

    loaded_plugins
}

// Global plugin manager to keep plugins alive
static PLUGIN_MANAGER: once_cell::sync::OnceCell<Arc<tokio::sync::RwLock<PluginManager>>> = once_cell::sync::OnceCell::new();

// Extension trait for plugins to work with gRPC
pub trait PluginGrpcExt {
    fn register_grpc(self: Box<Self>, server: Router) -> Router;
}

impl PluginGrpcExt for dyn Plugin {
    fn register_grpc(self: Box<Self>, server: Router) -> Router {
        // Plugins don't directly register gRPC services in this architecture
        // They handle commands through the plugin interface
        server
    }
}

// Add this to your main function after loading plugins:
/*
// Create plugin HTTP API
if let Some(manager) = PLUGIN_MANAGER.get() {
    let app_state = AppState {
        plugin_manager: manager.clone(),
    };
    
    let plugin_routes = AxumRouter::new()
        .route("/plugins", get(list_plugins))
        .route("/plugins/:name/:command", post(handle_plugin_command))
        .with_state(app_state);
    
    // Start plugin API server
    let plugin_api_port = 8091;
    println!("🔌 Plugin API starting on port {}", plugin_api_port);
    
    tokio::spawn(async move {
        axum::Server::bind(&format!("0.0.0.0:{}", plugin_api_port).parse().unwrap())
            .serve(plugin_routes.into_make_service())
            .await
            .unwrap();
    });
}
*/