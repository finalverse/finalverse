# Finalverse Developer Overview

Finalverse is an AI‑driven metaverse where players and intelligent agents co‑create persistent worlds. The repository contains a collection of Rust microservices, configuration files and a CLI client that demonstrate the core gameplay loop.

## Architecture

```
┌────────────────────────────────────┐
│            Client Layer            │
│  (txtViewer CLI & upcoming FinalStorm)  │
└────────────────┬───────────────────┘
                 │ HTTP/WebSocket
┌────────────────▼───────────────────┐
│            API Gateway             │
│         http://localhost:8080      │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│            Game Services           │
├────────────────────────────────────┤
│ • Song Engine       – :3001        │
│ • World Engine      – :3002        │
│ • Echo Engine       – :3003        │
│ • AI Orchestra      – :3004        │
│ • Story Engine      – :3005        │
│ • Harmony Service   – :3006        │
│ • Asset Service     – :3007        │
│ • Community         – :3008        │
│ • Silence Service   – :3009        │
│ • Procedural Gen    – :3010        │
│ • Behavior AI       – :3011        │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│            Data Layer              │
├────────────────────────────────────┤
│ • PostgreSQL   – :5432             │
│ • Redis        – :6379             │
│ • Qdrant       – :6333             │
│ • MinIO        – :9000             │
└────────────────────────────────────┘
```

All services expose `/health` and `/info` endpoints and are automatically registered with the local service registry.

## Quick Start

1. **Prepare the environment**
   ```bash
   ./scripts/setup_mvp.sh       # builds services and starts the data layer
   ./fv start                   # alias for scripts/finalverse.sh
   ```
2. **Verify services**
   ```bash
   ./fv test
   ```
3. **Run the CLI client**
   ```bash
   cargo run --bin txtViewer
   ```

The upcoming **FinalStorm** 3D client will connect through the WebSocket gateway (`:3000`) using the same service APIs.

## Development Workflow

- Each service lives under `services/` and shares common types in `crates/`.
- After modifying a service run `cargo build -p <service>` and restart it via `./fv start-service <service>`.
- Use `./scripts/monitor_services.sh` to tail logs during development. The
  server loads dynamic plugins from `FINALVERSE_PLUGIN_DIR` and exposes all
  gRPC services on `FINALVERSE_GRPC_PORT`. See `.env.example` for defaults.

## Contributing

This MVP focuses on the core loop of songweaving, world simulation and AI interaction. Contributions that enhance interoperability with FinalStorm, improve AI behaviours or extend the service APIs are welcome. Please ensure code is formatted with `cargo fmt` and that all services compile with `cargo build --workspace`.

## License

MIT © 2025 Finalverse Team
