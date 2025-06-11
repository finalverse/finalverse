// crates/finalverse-wasm-runtime/src/lib.rs
// Runtime for loading and executing Wasm plugins safely
use std::path::Path;
use anyhow::{Context, Result};
use wasmtime::{Engine, Func, Instance, Linker, Module, Store};

/// Context passed to Wasm plugins on events
#[repr(C)]
pub struct EventContext {
    pub entity_id: u64,
    pub event_type: u32,
    pub payload_ptr: *const u8,
    pub payload_len: usize,
}

pub struct WasmPlugin {
    instance: Instance,
    store: Store<()>,
    call_on_event: Func,
}

impl WasmPlugin {
    /// Load a Wasm module from the given path and prepare it for execution
    pub fn load(path: &Path) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path)
            .with_context(|| format!("Failed to load module at {:?}", path))?;

        let mut store = Store::new(&engine, ());
        let mut linker = Linker::new(&engine);

        // TODO: register host functions (e.g. logging, memory access) here

        let instance = linker.instantiate(&mut store, &module)?;
        let call_on_event = instance
            .get_func(&mut store, "on_event")
            .context("Missing `on_event` function")?;

        Ok(Self {
            instance,
            store,
            call_on_event,
        })
    }

    /// Invoke the plugin's `on_event` function with the given `EventContext`
    pub fn call_on_event(&mut self, ctx: &EventContext) -> Result<()> {
        let ptr = ctx as *const EventContext as i64;
        self.call_on_event
            .call(&mut self.store, &[ptr.into()], &mut [])
            .context("Failed to invoke on_event")?;
        Ok(())
    }
}
