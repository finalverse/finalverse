// services/plugin/src/lib.rs
// Dynamic service plugin interface for Finalverse
use axum::Router;
use tonic::transport::Server;
use std::path::PathBuf;

/// Trait implemented by optional service plugins.
/// Each plugin registers its own routes under the unified server.
#[async_trait::async_trait]
pub trait ServicePlugin: Send + Sync {
    /// Name of the plugin/service
    fn name(&self) -> &'static str;

    /// Build the router for this plugin.
    async fn routes(&self) -> Router;

    /// Optionally register gRPC services on the given `Server` builder.
    /// Implementations can add their own gRPC service definitions and return
    /// the updated builder. The default implementation simply returns the
    /// builder unchanged.
    fn register_grpc(self: Box<Self>, server: Server) -> Server {
        server
    }
}

/// Discover available plugins on the filesystem at runtime.
/// Currently returns an empty list as a placeholder.
pub async fn discover_plugins() -> Vec<Box<dyn ServicePlugin>> {
    // TODO: Implement dynamic discovery via configuration or directory scan.
    // For now, we read the `FINALVERSE_PLUGIN_DIR` environment variable and
    // look for dynamic libraries. Actual loading logic is left as a
    // placeholder to avoid unsafe code in this example.
    let mut plugins = Vec::new();
    if let Ok(dir) = std::env::var("FINALVERSE_PLUGIN_DIR") {
        let path = PathBuf::from(dir);
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "so" || ext == "dll" || ext == "dylib" {
                        tracing::info!("Discovered plugin candidate: {:?}", entry.path());
                        // Dynamic library loading would happen here
                    }
                }
            }
        }
    }
    plugins
}
