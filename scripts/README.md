# Finalverse Scripts

## Main Script: finalverse.sh

The main development helper script with commands:

- `build` - Build all Rust services
- `start` - Start all services
- `stop` - Stop all services  
- `test` - Test connectivity
- `status` - Show status
- `logs [service]` - View logs
- `clean-ports` - Clean port conflicts

## Usage

```bash
# Build everything
./scripts/finalverse.sh build

# Start all services
./scripts/finalverse.sh start

# Test connectivity 
./scripts/finalverse.sh test

# View logs
./scripts/finalverse.sh logs

# Stop everything
./scripts/finalverse.sh stop
```

## Services

- WebSocket Gateway: Port 3000
- AI Orchestra: Port 3001
- Song Engine: Port 3002
- Story Engine: Port 3003
- Echo Engine: Port 3004

## Data Services

- PostgreSQL: Port 5432
- Redis: Port 6379
- Qdrant: Port 6333
- MinIO: Ports 9000/9001
