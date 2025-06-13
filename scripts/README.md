# README.md - Updated documentation
# Finalverse Development Scripts

## Quick Start

```bash
# Initial setup (one time)
./scripts/setup_mvp.sh

# Daily development workflow
./scripts/finalverse.sh start    # Start all services
./scripts/finalverse.sh tests     # Test connectivity
./scripts/finalverse.sh monitor  # Watch services
./scripts/finalverse.sh stop     # Stop everything
```

## Main CLI: finalverse.sh

The consolidated development CLI with all functionality:

### Build & Setup
- `build` - Build all Rust services
- `setup` - Create directories and docker-compose.yml

### Service Management  
- `start` - Start all services (data + game)
- `stop` - Stop all services
- `restart [service]` - Restart all or specific service
- `status` - Show service status

### Testing & Monitoring
- `test` - Test service connectivity
- `monitor` - Real-time service monitoring
- `health` - Quick health check

### Logs & Debugging
- `logs [service] [lines]` - Show logs
- `follow [service]` - Follow logs in real-time

### Maintenance
- `clean-ports` - Kill processes on ports
- `clean` - Complete cleanup with options
- `backup` - Backup all data

## Service Architecture

### Game Services
- **WebSocket Gateway** (3000) - Real-time client connections
- **API Gateway** (8080) - HTTP API entry point
- **AI Orchestra** (3004) - AI coordination and LLM management
- **Song Engine** (3001) - Core harmony/dissonance mechanics
- **Story Engine** (3005) - Narrative and quest generation
- **Echo Engine** (3003) - First Echoes (Lumi, KAI, Terra, Ignis)
- **World Engine** (3002) - Dynamic world simulation
- **Harmony Service** (3006) - Player progression and resonance
- **Asset Service** (3007) - Content and media management
- **Community** (3008) - Social features and governance
- **Silence Service** (3009) - Corruption and antagonist systems
- **Procedural Gen** (3010) - AI-driven content generation
- **Behavior AI** (3011) - NPC and creature behaviors

### Data Services
- **PostgreSQL** (5432) - Primary database
- **Redis** (6379) - Caching and real-time data
- **Qdrant** (6333) - Vector database for AI
- **MinIO** (9000/9001) - Object storage

## Development Workflow

```bash
# Build and start everything
./scripts/finalverse.sh build
./scripts/finalverse.sh start

# During development - monitor logs
./scripts/finalverse.sh monitor

# Test individual services
./scripts/finalverse.sh tests
curl http://localhost:3001/health

# Debug specific service
./scripts/finalverse.sh logs song-engine 100
./scripts/finalverse.sh follow websocket-gateway

# Restart problematic service
./scripts/finalverse.sh restart harmony-service

# Clean restart
./scripts/finalverse.sh stop
./scripts/finalverse.sh clean-ports
./scripts/finalverse.sh start
```

These helper scripts were removed. Use `./scripts/finalverse.sh <command>` directly.
## Troubleshooting

1. **Services won't start**: Check if binaries exist with `ls target/release/`
2. **Port conflicts**: Run `./scripts/finalverse.sh clean-ports`
3. **Data issues**: Run `./scripts/finalverse.sh clean` for full reset
4. **Build failures**: Check Rust version and dependencies

## Advanced Usage

To run services inside Docker containers instead of local binaries, set the
`USE_DOCKER` environment variable:

```bash
export USE_DOCKER=true
./scripts/finalverse.sh start
```

This will build container images with `docker/Dockerfile.service` and run them
using the same port mappings as the local setup.
