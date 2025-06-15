use finalverse_wasm_runtime::{EventContext, WasmPlugin};
use std::path::Path;

#[test]
fn load_and_call() -> anyhow::Result<()> {
    let plugin_path = Path::new("tests/simple_plugin.wat");
    let mut plugin = WasmPlugin::load(plugin_path)?;
    let ctx = EventContext {
        entity_id: 1,
        event_type: 0,
        payload_ptr: std::ptr::null(),
        payload_len: 0,
    };
    plugin.call_on_event(&ctx)?;
    Ok(())
}
