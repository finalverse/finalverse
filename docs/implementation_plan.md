# Finalverse Implementation Plan

This plan lists the steps for completing the outstanding features in [advanced_features_todo.md](advanced_features_todo.md). For architectural context see [enhanced_architecture.md](enhanced_architecture.md) and the individual feature guides in `docs/`.

## Short Term

### Player progress persistence
- Add new tables and migrations for player profiles and progress.
- Expose CRUD endpoints in the account/world services.
- Load/save progress during login and zone transitions.
- **After each change** run `cargo fmt --all` and `cargo build --workspace`.

### Echo evolution system
- Extend the Echo data model with evolution stages.
- Implement a background task that updates echoes based on world state.
- Provide API endpoints to query current evolution status.

### Basic multiplayer presence
- Track active sessions in the world engine.
- Broadcast join/leave events over WebSocket.
- Persist presence info for analytics.

### Dynamic weather system
- Add weather state to world regions and update at intervals.
- Send weather updates to clients via the WebSocket gateway.
- Store forecasts so they survive restarts.

### Melody combination system
- Implement melody crafting logic in the song engine.
- Add tests for harmony conflicts and combination results.

## Medium Term

### Region teleportation
- Implement region hand-off in `world3d-service`.
  - Add teleport API for clients.
  - Update grid management so players switch owners seamlessly.
- Persist last region in the player profile.

### Player inventory persistence
- Add `inventory-service` microservice.
  - Create service skeleton and integrate with the workspace.
  - Design database schema for items and inventories.
- Update asset service to interact with inventories via gRPC.

### Scripting support for in-world objects
- Integrate scripting via WASM plugins (see [plugin_dev_guide.md](plugin_dev_guide.md)).
  - Expose host functions for object state manipulation.
  - Provide an example plugin demonstrating an interactive object.

### Full ecosystem simulation
- Expand the procedural generation service with plant/creature lifecycles.
- Store ecosystem state in the database for consistency.

### Guild system
- Create guild data model and membership APIs.
- Implement basic guild chat and shared quests.

### Creative tools
- Develop melody composer and world building tools in the client.
- Store journal entries and creations in the asset service.

### AI-driven world events
- Add a scheduler service that spawns events based on harmony levels.
- Notify players in affected regions via the WebSocket gateway.

### Procedural content generation
- Leverage the procedural generation service for new areas and quests.

### Player trading & economy
- Add trading endpoints and a simple market service.
- Ensure trades update player inventories atomically.

### PvP harmony battles
- Build battle mechanics into the harmony service.
- Expose matchmaking and battle resolution APIs.

## Long Term

### Advanced AI features
- Experiment with additional models and fine-tuning approaches.
- Integrate with MapleAI framework enhancements.

### Full economy system with auctions and markets
- Expand the market service to support auctions and player shops.
- Connect economy data to analytics dashboards.

### Competitive features and tournaments
- Introduce ranked matchmaking and scheduled tournaments.
- Provide leaderboards stored in the community service.

### Performance monitoring and player analytics
- Add metrics collection across all services.
- Provide dashboards to visualize player behaviour trends.

### AI model optimization
- Profile inference workloads and cache common requests.
- Evaluate lightweight models for edge deployment.

### Database, service mesh and security enhancements
- Harden database configurations and add regular backups.
- Evaluate a service mesh for gRPC traffic.
- Implement role-based auth across services.

---

Remember to run `cargo fmt --all` and `cargo build --workspace` after each code change to keep the workspace consistent.
