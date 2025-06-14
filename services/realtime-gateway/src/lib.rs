pub mod spatial_streaming;

use axum::extract::ws::WebSocket;
use std::future::Future;

#[async_trait::async_trait]
pub trait WebSocketPlugin: Send + Sync {
    fn register_ws_path(&self) -> &'static str;
    async fn handle(&self, socket: WebSocket);
}

#[cfg(feature = "dynamic")]
use libloading::{Library, Symbol};

pub struct LoadedPlugin {
    pub instance: Box<dyn WebSocketPlugin>,
    #[cfg(feature = "dynamic")]
    _lib: Library,
}

impl LoadedPlugin {
    pub fn take(&mut self) -> Box<dyn WebSocketPlugin> {
        std::mem::replace(&mut self.instance, Box::new(Dummy))
    }
}

struct Dummy;
#[async_trait::async_trait]
impl WebSocketPlugin for Dummy {
    fn register_ws_path(&self) -> &'static str { "_dummy" }
    async fn handle(&self, _socket: WebSocket) {}
}

pub async fn discover_plugins(dir: &std::path::Path) -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "so" || ext == "dylib" || ext == "dll" {
                    if let Ok(p) = unsafe { load_plugin(&path) } {
                        plugins.push(p);
                    }
                }
            }
        }
    }
    plugins
}

#[cfg(feature = "dynamic")]
unsafe fn load_plugin(path: &std::path::Path) -> anyhow::Result<LoadedPlugin> {
    let lib = Library::new(path)?;
    let constructor: Symbol<unsafe extern "C" fn() -> *mut dyn WebSocketPlugin> = lib.get(b"finalverse_ws_plugin")?;
    let raw = constructor();
    Ok(LoadedPlugin { instance: Box::from_raw(raw), _lib: lib })
}

#[cfg(not(feature = "dynamic"))]
unsafe fn load_plugin(_path: &std::path::Path) -> anyhow::Result<LoadedPlugin> {
    anyhow::bail!("dynamic plugin loading disabled")
}
