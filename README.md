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


----


# Finalverse MVP - Complete Setup & Troubleshooting Guide

## 🚀 Quick Start (If Everything is Working)

```bash
# 1. Start all services
./scripts/setup_mvp.sh

# 2. Test services
./scripts/test_services.sh

# 3. Run the client
cargo run --bin mock-client
```

## 🔧 Troubleshooting Common Issues

### Issue 1: Services Not Found by Client

**Symptoms:**
- Client shows "Failed to perform melody"
- Client shows "Failed to interact with Echo"
- Check service status shows all offline

**Solution:**
```bash
# 1. Check if services are actually running
docker ps | grep finalverse

# 2. Check service logs
docker-compose logs song-engine
docker-compose logs world-engine
docker-compose logs echo-engine

# 3. Test services directly
curl http://localhost:3001/info
curl http://localhost:3002/regions
curl http://localhost:3003/echoes
```

### Issue 2: Story Engine and Harmony Service Not Running

**Symptoms:**
- Services 5 and 6 show as offline
- Quest generation fails
- Progression viewing fails

**Solution:**
```bash
# 1. Check if these services are built
ls services/story-engine/src/main.rs
ls services/harmony-service/src/main.rs

# 2. If missing, copy from the artifacts provided
# Then rebuild:
cargo build --bin story-engine
cargo build --bin harmony-service

# 3. Run them manually for testing:
RUST_LOG=info cargo run --bin story-engine &
RUST_LOG=info cargo run --bin harmony-service &
```

### Issue 3: Region Selection Skipped

**Symptoms:**
- Client doesn't show region selection at startup
- Ecosystem viewing shows "Select a region first"

**Solution:**
This happens when the world engine isn't ready yet. The fixed client handles this gracefully, but you can manually select a region using option 12 in the menu.

## 📝 Manual Testing Without Client

### Test Individual Services

```bash
# 1. Create a test player and perform a melody
curl -X POST http://localhost:3001/melody \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "melody": {"Healing": {"power": 10.0}},
    "target": {"x": 100, "y": 50, "z": 200}
  }'

# 2. Interact with an Echo
curl -X POST http://localhost:3003/interact \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "echo_id": "lumi"
  }'

# 3. Get world regions
curl http://localhost:3002/regions

# 4. Get AI-generated NPC dialogue
curl -X POST http://localhost:3004/npc/dialogue \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "npc_name": "Village Elder",
      "emotion": "happy"
    }
  }'

# 5. Grant resonance (if harmony service is running)
curl -X POST http://localhost:3006/grant \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "creative": 10,
    "exploration": 15,
    "restoration": 20
  }'

# 6. Generate a quest (if story engine is running)
curl -X POST http://localhost:3005/quest/generate \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "region": "Terra Nova"
  }'
```

## 🏗️ Complete Service Architecture

### Running Services (Ports)
- **3001**: Song Engine (Melody & Harmony management)
- **3002**: World Engine (Regions & World State)
- **3003**: Echo Engine (Character interactions)
- **3004**: AI Orchestra (NPC dialogue & content generation)
- **3005**: Story Engine (Quests & Chronicles)
- **3006**: Harmony Service (Player progression)
- **8080**: API Gateway (Unified access point)

### Data Services
- **5432**: PostgreSQL (Player data, progression)
- **6379**: Redis (Caching, pub/sub)
- **6333**: Qdrant (Vector database for AI)
- **9000/9001**: MinIO (Object storage)

## 🎮 Client Features Working Status

### ✅ Working Features
1. **Check service status** - Shows which services are online
2. **Perform melody** - Basic melodies (healing, creation, discovery, courage)
3. **View world state** - Shows regions and their harmony levels
4. **Interact with Echo** - Build bonds with Lumi, KAI, Terra, Ignis
9. **Interact with AI NPC** - Get dynamic dialogue based on emotion
11. **Initiate symphony** - Mock implementation for group events

### ⚠️ Requires Additional Services
5. **View progression & stats** - Needs harmony-service
6. **View chronicle** - Needs story-engine
7. **Request personal quest** - Needs story-engine and ai-orchestra
8. **View ecosystem** - Needs ecosystem endpoint in world-engine
10. **Perform advanced melody** - Needs harmony-service for unlock tracking

## 🔄 Development Workflow

### Making Changes
1. Edit the service code
2. Rebuild: `cargo build --bin [service-name]`
3. Restart: `docker-compose restart [service-name]`
4. Test: Use curl commands or the client

### Adding New Features
1. Update the common types in `libs/common/src/lib.rs`
2. Add protocol messages in `libs/protocol/src/lib.rs`
3. Implement in the relevant service
4. Update the client to use the new feature
5. Test thoroughly

## 🐛 Debugging Tips

### Check Service Logs
```bash
# Docker logs
docker-compose logs -f [service-name]

# Local development logs
RUST_LOG=debug cargo run --bin [service-name]
```

### Test Database Connection
```bash
PGPASSWORD=finalverse_secret psql -h localhost -U finalverse -d finalverse -c "SELECT * FROM players;"
```

### Test Redis Connection
```bash
redis-cli ping
redis-cli SUBSCRIBE finalverse_events
```

## 🎯 Next Steps for Full Functionality

1. **Implement Missing Endpoints**
   - Add `/regions/:id/ecosystem` to world-engine
   - Add quest completion endpoint to story-engine
   - Add symphony coordination to song-engine

2. **Enhance AI Integration**
   - Connect to real LLMs for dynamic content
   - Implement procedural quest generation
   - Add personality to Echo interactions

3. **Improve Data Persistence**
   - Save player state between sessions
   - Implement proper event sourcing
   - Add data migration tools

4. **Production Readiness**
   - Add authentication
   - Implement rate limiting
   - Add monitoring and metrics
   - Set up proper logging

Remember: This is an MVP. Focus on getting the core loop working first, then expand!