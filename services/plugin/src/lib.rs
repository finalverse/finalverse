// services/plugin/src/lib.rs
// Dynamic service plugin interface for Finalverse
use axum::Router;

/// Trait implemented by optional service plugins.
/// Each plugin registers its own routes under the unified server.
#[async_trait::async_trait]
pub trait ServicePlugin: Send + Sync {
    /// Name of the plugin/service
    fn name(&self) -> &'static str;

    /// Build the router for this plugin.
    async fn routes(&self) -> Router;
}

/// Discover available plugins on the filesystem at runtime.
/// Currently returns an empty list as a placeholder.
pub async fn discover_plugins() -> Vec<Box<dyn ServicePlugin>> {
    // TODO: Implement dynamic discovery via configuration or directory scan
    Vec::new()
}
