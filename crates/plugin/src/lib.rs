// services/plugin/src/lib.rs
// Dynamic service plugin interface for Finalverse
use axum::Router as AxumRouter;
use tonic::transport::server::Router as GrpcRouter;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;

#[cfg(feature = "dynamic")]
use libloading::{Library, Symbol};

// Import the registry from the workspace `service_registry` crate (formerly
// `service_registry`)
use service_registry::LocalServiceRegistry;
// Use anyhow's Result for convenience in async plugin APIs
use anyhow::Result;

/// Trait implemented by optional service plugins.
/// Each plugin registers its own routes under the unified server.
#[async_trait::async_trait]
pub trait ServicePlugin: Send + Sync {
    /// Name of the plugin/service
    fn name(&self) -> &'static str;

    /// Build the router for this plugin.
    async fn routes(&self) -> AxumRouter;

    /// Initialize the plugin. Called after loading so the plugin can register
    /// itself with the service registry or load configuration.
    async fn init(&self, _registry: &LocalServiceRegistry) -> Result<()> {
        Ok(())
    }

    /// Optionally register gRPC services on the given `Server` builder.
    /// Implementations can add their own gRPC service definitions and return
    /// the updated builder. The default implementation simply returns the
    /// builder unchanged.
    fn register_grpc(self: Box<Self>, server: GrpcRouter) -> GrpcRouter {
        server
    }
}

/// Internal plugin used as a placeholder after moving plugin instances out.
pub struct NoopPlugin;

#[async_trait::async_trait]
impl ServicePlugin for NoopPlugin {
    fn name(&self) -> &'static str { "noop" }
    async fn routes(&self) -> AxumRouter { AxumRouter::new() }
    fn register_grpc(self: Box<Self>, server: GrpcRouter) -> GrpcRouter { server }
}

/// Discover available plugins on the filesystem at runtime.
/// Currently returns an empty list as a placeholder.
pub struct LoadedPlugin {
    pub instance: Box<dyn ServicePlugin>,
    #[cfg(feature = "dynamic")]
    _lib: Library,
}

/// Plugins discovered at startup.
pub static PLUGINS: Lazy<Vec<LoadedPlugin>> = Lazy::new(discover_plugins);

impl LoadedPlugin {
    pub fn take_instance(&mut self) -> Box<dyn ServicePlugin> {
        std::mem::replace(&mut self.instance, Box::new(NoopPlugin))
    }
}

pub fn discover_plugins() -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();
    if let Ok(dir) = std::env::var("FINALVERSE_PLUGIN_DIR") {
        let path = PathBuf::from(dir);
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext == "so" || ext == "dll" || ext == "dylib" {
                        tracing::info!("Discovered plugin candidate: {:?}", path);
                        if let Ok(plugin) = unsafe { load_plugin(&path) } {
                            plugins.push(plugin);
                        }
                    }
                }
            }
        }
    }
    plugins
}

unsafe fn load_plugin(path: &Path) -> Result<LoadedPlugin> {
    #[cfg(feature = "dynamic")]
    unsafe {
        let lib = Library::new(path)?;
        let constructor: Symbol<unsafe extern "C" fn() -> *mut dyn ServicePlugin> = lib.get(b"finalverse_plugin_entry")?;
        let boxed_raw = constructor();
        let instance = Box::from_raw(boxed_raw);
        Ok(LoadedPlugin { instance, _lib: lib })
    }

    #[cfg(not(feature = "dynamic"))]
    {
        let _ = path;
        Err(anyhow::anyhow!("dynamic plugin loading disabled"))
    }
}
