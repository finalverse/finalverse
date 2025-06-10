#!/bin/bash
# cleanup_and_restart.sh - Complete cleanup and fresh start for Finalverse

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${PURPLE}🧹 Finalverse Complete Cleanup & Restart${NC}"
echo "======================================="

# Function to confirm action
confirm() {
    read -p "$1 (y/n): " -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Cancelled."
        exit 1
    fi
}

# Warning
echo -e "${YELLOW}⚠️  WARNING: This will:${NC}"
echo "  - Stop all Finalverse services"
echo "  - Remove all Docker containers, volumes, and networks"
echo "  - Clean up all logs and temporary files"
echo "  - Remove all game data (can be backed up first)"
echo ""
confirm "Do you want to continue?"

# Backup option
read -p "Do you want to backup data first? (y/n): " -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    BACKUP_DIR="backups/$(date +%Y%m%d_%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    echo -e "${BLUE}📦 Creating backup in $BACKUP_DIR...${NC}"
    
    # Backup data directories
    if [ -d "data" ]; then
        cp -r data "$BACKUP_DIR/"
        echo "  ✅ Data backed up"
    fi
    
    # Backup logs
    if [ -d "logs" ]; then
        cp -r logs "$BACKUP_DIR/"
        echo "  ✅ Logs backed up"
    fi
    
    # Backup config
    if [ -d "config" ]; then
        cp -r config "$BACKUP_DIR/"
        echo "  ✅ Config backed up"
    fi
    
    echo -e "${GREEN}✅ Backup completed${NC}"
fi

echo ""
echo -e "${BLUE}🛑 Step 1: Stopping all services...${NC}"

# Stop Finalverse services
if [ -f "scripts/finalverse.sh" ]; then
    ./scripts/finalverse.sh stop 2>/dev/null || true
fi

# Kill any remaining processes on Finalverse ports
PORTS=(3000 3001 3002 3003 3004 3005 3006 3007 3008 3009 3010 3011 5432 6379 6333 6334 8080 8500 9000 9001)
for port in "${PORTS[@]}"; do
    if lsof -ti:$port >/dev/null 2>&1; then
        echo "  Killing processes on port $port..."
        lsof -ti:$port | xargs kill -9 2>/dev/null || true
    fi
done

echo -e "${GREEN}✅ Services stopped${NC}"

echo ""
echo -e "${BLUE}🐳 Step 2: Docker cleanup...${NC}"

# Stop all containers
echo "  Stopping Docker containers..."
docker-compose down -v 2>/dev/null || true

# Remove all Finalverse containers
echo "  Removing containers..."
docker ps -a | grep finalverse | awk '{print $1}' | xargs -r docker rm -f 2>/dev/null || true

# Remove all Finalverse images (optional)
read -p "Remove Docker images too? This will require re-downloading (y/n): " -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "  Removing images..."
    docker images | grep -E "(postgres|redis|qdrant|minio)" | awk '{print $3}' | xargs -r docker rmi -f 2>/dev/null || true
fi

# Remove networks
echo "  Removing networks..."
docker network rm finalverse-network 2>/dev/null || true

# Prune system
echo "  Pruning Docker system..."
docker system prune -f

echo -e "${GREEN}✅ Docker cleaned${NC}"

echo ""
echo -e "${BLUE}📁 Step 3: Cleaning directories...${NC}"

# Clean data directories
rm -rf data/postgres/* data/redis/* data/qdrant/* data/minio/*
echo "  ✅ Data directories cleaned"

# Clean logs
rm -rf logs/*
echo "  ✅ Logs cleaned"

# Clean PIDs
rm -rf .pids/*
echo "  ✅ PID files cleaned"

# Clean Rust target (optional)
read -p "Clean Rust build cache? This will require full rebuild (y/n): " -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "  Cleaning Rust target..."
    cargo clean
    echo "  ✅ Rust cache cleaned"
fi

echo -e "${GREEN}✅ Directories cleaned${NC}"

echo ""
echo -e "${BLUE}🚀 Step 4: Fresh setup...${NC}"

# Create fresh directories
mkdir -p data/{postgres,redis,qdrant,minio} logs .pids config

# Create fresh config
cat > config/finalverse.toml << 'EOF'
[general]
environment = "development"
log_level = "info"

[data]
postgres_url = "postgres://finalverse:finalverse_secret@localhost/finalverse"
redis_url = "redis://localhost:6379"
qdrant_url = "http://localhost:6333"
minio_url = "http://localhost:9000"
EOF

echo "  ✅ Fresh directories created"

echo ""
echo -e "${BLUE}🐳 Step 5: Starting Docker services...${NC}"

# Start Docker services
docker-compose up -d

# Wait for services to be ready
echo "  Waiting for services to be ready..."
sleep 10

# Check Docker services
docker-compose ps

echo -e "${GREEN}✅ Docker services started${NC}"

echo ""
echo -e "${BLUE}🔨 Step 6: Building Finalverse...${NC}"

# Build the project
cargo build --workspace --release

echo -e "${GREEN}✅ Build completed${NC}"

echo ""
echo -e "${BLUE}🎮 Step 7: Starting Finalverse services...${NC}"

# Copy the new orchestrator script
if [ -f "scripts/finalverse-orchestrator.sh" ]; then
    chmod +x scripts/finalverse-orchestrator.sh
    ./scripts/finalverse-orchestrator.sh start
else
    ./scripts/finalverse.sh start
fi

echo ""
echo -e "${BLUE}✨ Step 8: Initializing world data...${NC}"

# Initialize the database with some seed data
echo "  Creating initial regions..."
PGPASSWORD=finalverse_secret psql -h localhost -U finalverse -d finalverse << 'EOF' 2>/dev/null || true
-- Create initial regions
CREATE TABLE IF NOT EXISTS regions (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    harmony_level FLOAT DEFAULT 75.0,
    weather VARCHAR(50) DEFAULT 'clear',
    active_players INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO regions (id, name, harmony_level, weather) VALUES
    ('550e8400-e29b-41d4-a716-446655440000', 'Terra Nova', 85.0, 'clear'),
    ('550e8400-e29b-41d4-a716-446655440001', 'Aethelgard', 65.0, 'misty'),
    ('550e8400-e29b-41d4-a716-446655440002', 'Technos Prime', 90.0, 'digital_storm'),
    ('550e8400-e29b-41d4-a716-446655440003', 'Whispering Wilds', 45.0, 'corrupted')
ON CONFLICT (id) DO NOTHING;

-- Create player progression table
CREATE TABLE IF NOT EXISTS player_progression (
    player_id UUID PRIMARY KEY,
    player_name VARCHAR(255),
    creative_resonance INT DEFAULT 10,
    exploration_resonance INT DEFAULT 10,
    restoration_resonance INT DEFAULT 10,
    attunement_tier INT DEFAULT 0,
    total_actions INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create echo bonds table
CREATE TABLE IF NOT EXISTS echo_bonds (
    player_id UUID,
    echo_type VARCHAR(50),
    bond_level INT DEFAULT 0,
    last_interaction TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, echo_type)
);
EOF

echo "  ✅ Database initialized"

# Initialize Redis with some initial data
redis-cli << 'EOF' 2>/dev/null || true
SET global:harmony 75.0
SET global:active_players 0
SET global:total_melodies 0
HSET echo:average_bonds lumi 0 kai 0 terra 0 ignis 0
EOF

echo "  ✅ Redis initialized"

echo ""
echo -e "${BLUE}🧪 Step 9: Running health checks...${NC}"

# Run comprehensive health check
./scripts/finalverse-orchestrator.sh status || ./scripts/finalverse.sh test

echo ""
echo -e "${GREEN}🎉 Finalverse has been completely cleaned and restarted!${NC}"
echo ""
echo "📋 Next steps:"
echo "  1. Open the dashboard: http://localhost:8080/dashboard (or open dashboard.html)"
echo "  2. Test with the web client: open Client_WebSocket.html"
echo "  3. Test with the CLI client: cargo run --bin mock-client"
echo "  4. Monitor logs: ./scripts/finalverse-orchestrator.sh logs"
echo ""
echo "🎵 The Song of Creation begins anew! 🌟"