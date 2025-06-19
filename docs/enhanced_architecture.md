# Finalverse Future Architecture

This document outlines the planned evolution of the Finalverse platform. It builds on the current service layout described in [architecture_overview.md](architecture_overview.md).

## Current Topology (Summary)

Finalverse today is organised in three main layers:

1. **Client Layer** – CLI tools, the webViewer and the upcoming FinalStorm client connect over HTTP or WebSocket to the gateways.
2. **API Gateway** – single HTTP entry point at `:8080` used by REST clients.
3. **WebSocket Gateway** – realtime endpoint on `:3000` forwarding events to and from the game services.
4. **Game Services** – domain specific microservices such as the Song Engine, World Engine, AI Orchestra and others. Each service registers itself with the Service Registry.
5. **Service Registry** – simple discovery service letting components resolve each other’s gRPC/HTTP addresses.
6. **Data Layer** – PostgreSQL, Redis, Qdrant and MinIO provide persistence, caching, vector search and object storage.

Refer to [architecture_overview.md](architecture_overview.md) for the complete diagram and role descriptions.

## Planned Services

- **Region hand-off & Teleport** – Allows seamless movement between world regions. The service coordinates grid ownership and supports instant player teleportation.
- **Asset & Inventory** – Extends the current Asset Service to manage user generated items and personal inventories.
- **Scripting Layer** – Gameplay extensions delivered as plugins or WebAssembly modules. See the [Plugin Development Guide](plugin_dev_guide.md) for current plugin capabilities.
- **FinalStorm Integration** – The 3D client communicates through the WebSocket Gateway (with QUIC support in the future) to subscribe to spatial streams from [`services/world3d-service`](../services/world3d-service/).

## Updated Architecture Diagram

The diagram below highlights the upcoming components (`*` marks future additions):

```
┌────────────────────────────────────────────┐
│              Client Layer                  │
│  CLI / webViewer / FinalStorm 3D           │
└────────────────┬───────────────────────────┘
                 │ HTTP / WS or QUIC
┌────────────────▼───────────────────────────┐
│              API Gateway                   │
└────────────────┬───────────────────────────┘
                 │
┌────────────────▼───────────────────────────┐
│            WebSocket Gateway               │
└────────────────┬───────────────────────────┘
                 │ gRPC / HTTP
┌────────────────▼───────────────────────────┐
│             Game Services                  │
├────────────────────────────────────────────┤
│ • Song Engine        – :3001               │
│ • World Engine       – :3002               │
│ • Echo Engine        – :3003               │
│ • AI Orchestra       – :3004               │
│ • Story Engine       – :3005               │
│ • Harmony Service    – :3006               │
│ • Asset Service      – :3007               │
│ • Community          – :3008               │
│ • Silence Service    – :3009               │
│ • Procedural Gen     – :3010               │
│ • Behavior AI        – :3011               │
│ • World3D Service    – :3012               │
│ • Region Teleport*   – :3013               │
│ • Inventory Service* – :3014               │
│ • Plugin/WASM Host*  – in-process          │
└────────────────┬───────────────────────────┘
                 │
┌────────────────▼───────────────────────────┐
│           Service Registry                 │
└────────────────┬───────────────────────────┘
                 │
┌────────────────▼───────────────────────────┐
│               Data Layer                   │
├────────────────────────────────────────────┤
│ • PostgreSQL – :5432                       │
│ • Redis      – :6379                       │
│ • Qdrant     – :6333                       │
│ • MinIO      – :9000                       │
└────────────────────────────────────────────┘
```

The FinalStorm client streams world data from the `world3d-service` and will eventually leverage QUIC for low latency updates.

