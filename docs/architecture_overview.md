# Finalverse Architecture Overview

This document provides an overview of the current microservice layout, data stores and shared crates used across the Finalverse project. It expands on the diagram in the main README and explains the role of each component.

## Service Topology

```
┌────────────────────────────────────┐
│            Client Layer            │
│  (CLI, webViewer, FinalStorm)      │
└────────────────┬───────────────────┘
                 │ HTTP / WS
┌────────────────▼───────────────────┐
│           API Gateway              │
│         http://localhost:8080      │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│        WebSocket Gateway           │
│             :3000                  │
└────────────────┬───────────────────┘
                 │ gRPC / HTTP
┌────────────────▼───────────────────┐
│            Game Services           │
├────────────────────────────────────┤
│ • Song Engine      – :3001         │
│ • World Engine     – :3002         │
│ • Echo Engine      – :3003         │
│ • AI Orchestra     – :3004         │
│ • Story Engine     – :3005         │
│ • Harmony Service  – :3006         │
│ • Asset Service    – :3007         │
│ • Community        – :3008         │
│ • Silence Service  – :3009         │
│ • Procedural Gen   – :3010         │
│ • Behavior AI      – :3011         │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│          Service Registry          │
│         http://localhost:8500      │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│            Data Layer              │
├────────────────────────────────────┤
│ • PostgreSQL    – :5432            │
│ • Redis         – :6379            │
│ • Qdrant        – :6333            │
│ • MinIO         – :9000            │
└────────────────────────────────────┘
```

Each service registers itself with the **Service Registry** on startup. The API Gateway forwards HTTP requests to the appropriate service and the WebSocket Gateway provides realtime connections. Services communicate with each other via gRPC using addresses discovered from the registry.

## Component Roles

- **API Gateway** – Main HTTP entry point. Performs routing, CORS and rate limiting.
- **WebSocket Gateway** – Handles realtime client traffic and broadcasts events from the engines.
- **AI Orchestra** – Manages connections to LLM providers and generates dynamic content.
- **Song Engine** – Core mechanics for harmony, melodies and resonance.
- **World Engine** – Simulates regional state, weather and ecosystem.
- **Story Engine** – Generates quests and chronicles world events.
- **Service Registry** – Simple discovery mechanism used by all services for address lookup.
- **Plugins** – Dynamically loaded modules extending server capabilities. Implement the `ServicePlugin` trait and can expose HTTP or gRPC endpoints.
- **Data Services** – PostgreSQL for persistence, Redis for caching, Qdrant for vector search and MinIO for object storage.

## Plugin System and WASM Runtime

Plugins are compiled as shared libraries and placed in `FINALVERSE_PLUGIN_DIR`. The server loads them at startup, calling their `init` function to register routes and gRPC services. For lightweight and safe extensions, WebAssembly modules can be executed through the `wasm-runtime` crate. It provides sandboxed execution and a small host API for logging and simple memory operations.

## Shared Crates

Common functionality lives in the `crates/` directory:

- `core` – basic utilities and types used by all services
- `config` – configuration loader and unified `finalverse.toml`
- `health` – standardized health and info endpoints
- `proto` & `grpc-client` – gRPC definitions and clients
- `plugin` – dynamic plugin interface
- `wasm-runtime` – sandbox for executing WASM plugins

## Current Capabilities

The MVP already includes WebSocket support, AI integration and a dynamic quest system. See [advance_features_impl.md](advance_features_impl.md) for the full list of implemented features.

## Roadmap

Future phases will expand multiplayer systems, procedural content and AI-driven events. Details are outlined in [advance_features_plan.md](advance_features_plan.md).

## Potential External 3D Client

An external 3D client (such as the planned **FinalStorm**) could connect through
WebSocket or QUIC via the WebSocket Gateway on port `3000`. It would subscribe
to spatial streams from `services/world3d-service`, which coordinates grids,
terrain generation and streaming through components like `WorldManager`,
`SpatialStreamManager` and `TerrainService`.

The client consumes structures from `crates/world3d` — including `Grid`,
`TerrainPatch`, `EchoEntity` and the `AssetManifest` for meshes and textures —
to render the world. AI-generated quests and characters are inserted into the 3D
space by `world3d-service`, allowing players to see new content appear in real
time.

