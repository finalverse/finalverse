# Finalverse Developer Overview

**Current Release: v0.1.2 – for Real Production Development**

Finalverse is an AI‑driven metaverse where players and intelligent agents co‑create persistent worlds. This repository hosts the microservices, CLI client and plugin SDK that power the prototype.

## Features

- Real‑time WebSocket gateway for future 3D clients
- AI Orchestra with pluggable LLM providers (Ollama or OpenAI)
- Dynamic quest and story generation
- Procedural world simulation and ecosystem services
- Extensible plugin system for adding gameplay modules

## Architecture

```
┌────────────────────────────────────┐
│            Client Layer            │
│  (Mock CLI & upcoming FinalStorm)  │
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

All services expose `/health` and `/info` endpoints and are automatically registered with the local service registry. The API Gateway on port `8080` provides the HTTP interface while the WebSocket Gateway on port `3000` handles realtime client traffic.

Service addresses are defined in `crates/config/finalverse.toml` so you can change the ports as needed. Environment variables can override any value at runtime.

Services can be launched either as local binaries or as Docker containers by setting the `USE_DOCKER` flag.

## Quick Start

### First-Time Setup

Run the helper script once to create directories, build binaries and prepare Docker images:

```bash
./scripts/setup_mvp.sh
```

### Starting the Stack

For daily development start the services (rebuild if you changed code):

```bash
./fv start                # launch data + game services
./fv tests                # check health endpoints
```

### Run the CLI client

```bash
cargo run --bin txtViewer
```

The upcoming **FinalStorm** 3D client will connect through the WebSocket gateway (`:3000`) using the same service APIs.

### Deployment

```bash
./scripts/setup_mvp.sh   # one-time setup
./fv build               # compile all services
./fv start               # start data + game services
./fv tests               # verify health endpoints
```

To run services inside Docker containers set `USE_DOCKER=true`:

```bash
USE_DOCKER=true ./fv start
```

Production deployments follow the same steps but with `--release` and persistent data directories.

### Server Maintenance

- `./fv status` – show running services
- `./fv monitor` – realtime health and log view
- `./fv restart <service>` – restart a single service
- `./fv backup` – snapshot data to `backups/`
- `./fv clean` – remove generated data and logs

Logs are stored in the `logs/` directory.

### Developing Plugins

1. Add `fv-plugin` to your plugin `Cargo.toml`.
2. Implement the `ServicePlugin` trait and export `finalverse_plugin_entry`.
3. Build with `cargo build --release`.
4. Copy the resulting library into `$FINALVERSE_PLUGIN_DIR`.
5. Restart the server for the plugin to load.

See [docs/plugin_dev_guide.md](docs/plugin_dev_guide.md) for details.

## Development Workflow

- Each service lives under `services/` and shares common types in `crates/`.
- After modifying a service run `cargo build -p <service>` and restart it via `./fv restart <service>`.
- Use `./fv monitor` to tail logs during development. The
  server loads dynamic plugins from `FINALVERSE_PLUGIN_DIR` and exposes all
  gRPC services on `FINALVERSE_GRPC_PORT`. See `.env.example` for defaults.

## Contributing

This MVP focuses on the core loop of songweaving, world simulation and AI interaction. Contributions that enhance interoperability with FinalStorm, improve AI behaviours or extend the service APIs are welcome. Please ensure code is formatted with `cargo fmt` and that all services compile with `cargo build --workspace`.

## Release History

- **0.1.2** - Improved monitoring and WebSocket integration.
- **0.1.1** - First official release with working setup scripts and Docker deployment.
- **0.1.0** - Initial proof-of-concept MVP. See `CHANGELOG.md` for details.

## License

© 2025 Finalverse Inc. All rights reserved.
