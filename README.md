# Finalverse MVP

A revolutionary AI-driven metaverse platform where AI and humans co-create stories, worlds, and experiences through the power of the Song of Creation.

## 🌟 Overview

This MVP demonstrates the core architecture of Finalverse with:
- **Microservices Architecture**: Scalable, distributed services
- **AI Integration**: Mock AI services ready for real model integration
- **Event-Driven Design**: Reactive world that responds to player actions
- **Living World**: Dynamic systems that evolve even when players are offline

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│           Client Layer                  │
│      (Mock Client / FinalStorm)         │
└────────────────┬────────────────────────┘
                 │ HTTP/QUIC
┌────────────────▼────────────────────────┐
│          API Gateway (Envoy)            │
│         http://localhost:8080           │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│          Core Services                  │
├─────────────────────────────────────────┤
│ • Song Engine (Port 3001)               │
│ • World Engine (Port 3002)              │
│ • Echo Engine (Port 3003)               │
│ • AI Orchestra (Port 3004)              │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│           Data Layer                    │
├─────────────────────────────────────────┤
│ • PostgreSQL (Relational Data)          │
│ • Redis (Cache & Real-time State)       │
│ • Qdrant (Vector DB for AI)             │
│ • MinIO (Object Storage)                │
└─────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ (https://rustup.rs/)
- Docker & Docker Compose
- PostgreSQL client (optional, for direct DB access)

### Setup

1. **Clone and setup the workspace**:
```bash
# Copy all the provided code into the appropriate directory structure
./scripts/setup_mvp.sh
```

2. **Start all services**:
```bash
# This will:
# - Start data layer (PostgreSQL, Redis, Qdrant, MinIO)
# - Build and run all Rust services
# - Start the API Gateway
./scripts/setup_mvp.sh
```

3. **Run the client**:
```bash
cargo run --bin mock-client
```

## 🎮 Using the Mock Client

The mock client provides an interactive CLI to test all services:

1. **Check Service Status**: Verify all services are running
2. **Perform Melodies**: Use the Song of Creation to affect the world
   - Healing: Restore harmony to regions
   - Creation: Manifest new patterns
   - Discovery: Reveal hidden elements
   - Courage: Inspire and strengthen
3. **View World State**: See current harmony levels and weather
4. **Interact with Echoes**: Build bonds with Lumi, KAI, Terra, and Ignis
5. **View Echo Bonds**: Track your relationships with the First Echoes

## 🧪 Testing the Services

### Direct Service Testing
```bash
# Test individual services
curl http://localhost:3001/info  # Song Engine
curl http://localhost:3002/regions  # World Engine
curl http://localhost:3003/echoes  # Echo Engine
curl http://localhost:3004/models  # AI Orchestra
```

### API Gateway Testing
```bash
# Test via API Gateway
curl http://localhost:8080/api/song/info
curl http://localhost:8080/api/world/regions
curl http://localhost:8080/api/echo/echoes
curl http://localhost:8080/api/ai/models
```

### Example Interactions

**Perform a Melody**:
```bash
curl -X POST http://localhost:3001/melody \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "melody": {"Healing": {"power": 10.0}},
    "target": {"x": 100, "y": 50, "z": 200}
  }'
```

**Interact with an Echo**:
```bash
curl -X POST http://localhost:3003/interact \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "echo_id": "lumi"
  }'
```

**Generate NPC Dialogue**:
```bash
curl -X POST http://localhost:3004/npc/dialogue \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "npc_name": "Elder Sage",
      "emotion": "worried"
    }
  }'
```

## 🔧 Development

### Project Structure
```
finalverse-mvp/
├── services/          # Microservices
│   ├── song-engine/   # Manages the Song of Creation
│   ├── world-engine/  # Handles world state and dynamics
│   ├── echo-engine/   # Manages First Echoes interactions
│   └── ai-orchestra/  # Coordinates AI models
├── libs/              # Shared libraries
│   ├── common/        # Common types and utilities
│   └── protocol/      # Communication protocol
├── client/            # Client applications
│   └── mock-client/   # CLI testing client
├── config/            # Configuration files
├── docker/            # Docker configurations
└── scripts/           # Utility scripts
```

### Adding New Services

1. Create new service directory: `services/your-service/`
2. Add to workspace in root `Cargo.toml`
3. Implement the `FinalverseService` trait
4. Add health check endpoint
5. Update docker-compose and API gateway config

### Database Schema

The MVP includes basic tables for:
- `players`: Player accounts and metadata
- `player_resonance`: Progression tracking
- `echo_bonds`: Relationships with First Echoes
- `regions`: World state per region
- `event_log`: Event sourcing for world history

## 🎯 Next Steps

### Phase 1: Enhanced AI Integration
- [ ] Integrate real LLMs (GPT-4, Claude, Llama)
- [ ] Implement vector embeddings for semantic search
- [ ] Add procedural content generation
- [ ] Create behavior trees for NPCs

### Phase 2: Advanced World Systems
- [ ] Implement full ecosystem simulation
- [ ] Add weather and celestial event systems
- [ ] Create dynamic economy
- [ ] Build faction and politics systems

### Phase 3: Player Systems
- [ ] Implement full Harmony progression system
- [ ] Add Songweaving mechanics
- [ ] Create inventory and crafting
- [ ] Build social features

### Phase 4: Production Ready
- [ ] Migrate to Kubernetes
- [ ] Implement proper authentication
- [ ] Add monitoring and observability
- [ ] Performance optimization
- [ ] Security hardening

## 🐛 Troubleshooting

### Services not starting
- Check logs: `docker-compose logs [service-name]`
- Ensure ports aren't already in use
- Verify Docker daemon is running

### Database connection issues
- Check PostgreSQL is running: `docker ps`
- Verify credentials in connection strings
- Ensure database is initialized

### Client can't connect
- Verify services are running: `./test_services.sh`
- Check firewall settings
- Ensure correct ports are exposed

## 🤝 Contributing

This MVP is the foundation for building the full Finalverse vision. To contribute:

1. Focus on one of the Core Harmonies:
   - **Symbiotic Creation**: AI-human collaboration features
   - **Empathetic Exploration**: World discovery systems
   - **Living Wonder**: Dynamic world events

2. Follow the established patterns
3. Write tests for new features
4. Document your additions

## 📜 License

Copyright © 2025 Finalverse Team. All rights reserved.