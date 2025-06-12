# Plugin Development Guide

This guide covers the basics of creating, testing and deploying a plugin for the Finalverse server.

## 1. Project Setup

1. Enable the `fv-plugin` crate in your `Cargo.toml`:
   ```toml
   [dependencies]
   fv-plugin = { path = "../../crates/fv-plugin" }
   ```
2. Add other dependencies as needed (e.g. `async-trait`, `tokio`).
3. Ensure your library is built as a dynamic library:
   ```toml
   [lib]
   crate-type = ["cdylib"]
   ```

## 2. Implement `ServicePlugin`

Create a struct implementing the `ServicePlugin` trait. At minimum you must provide:

- `name()` – returns a unique plugin name.
- `routes()` – returns an Axum `Router` with any HTTP routes.
- `init()` – perform initialization (optional).
- `register_grpc()` – register gRPC services if needed.

Export an entry point that constructs your plugin:

```rust
#[no_mangle]
pub extern "C" fn finalverse_plugin_entry() -> *mut dyn ServicePlugin {
    Box::into_raw(Box::new(MyPlugin::default()) as Box<dyn ServicePlugin>)
}
```

## 3. Building

Run `cargo build -p my-plugin --release` to produce a shared library (`.so`, `.dll`, or `.dylib`). Copy the resulting file into the directory defined by the `FINALVERSE_PLUGIN_DIR` environment variable (see `.env.example`).

Example:
```bash
export FINALVERSE_PLUGIN_DIR=./target/plugins
mkdir -p "$FINALVERSE_PLUGIN_DIR"
cp target/release/libmy_plugin.so "$FINALVERSE_PLUGIN_DIR"
```

## 4. Testing

You can test plugin logic with standard Rust tests. Integration with the server requires starting the server with the plugin directory set. During development run:

```bash
cargo run -p finalverse-server
```

The server will discover plugins in `FINALVERSE_PLUGIN_DIR` and initialize them. Use any exposed HTTP or gRPC endpoints to verify behaviour.

## 5. Deployment

1. Build the plugin in release mode.
2. Place the compiled library in the production plugin directory on the server.
3. Restart the Finalverse server so it loads the new plugin.

Plugins can be hot-swapped by replacing the library file and restarting the server.

