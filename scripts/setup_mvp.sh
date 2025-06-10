#!/bin/bash
# scripts/setup_mvp.sh - Updated for current Finalverse structure

set -e

echo "🎵 Setting up Finalverse MVP..."
echo "================================"

# Check prerequisites
echo "🔍 Checking prerequisites..."
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo is required but not installed. Visit https://rustup.rs/"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed."; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed."; exit 1; }

# Create directory structure
echo "📁 Creating directory structure..."
mkdir -p data/{postgres,redis,qdrant,minio}
mkdir -p logs
mkdir -p config

# Build all services
echo "🔨 Building Finalverse services..."
echo "   This may take a few minutes for the first build..."
cargo build --workspace --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the compilation errors above."
    exit 1
fi

echo "✅ All services built successfully!"

# Create docker-compose.yml for data layer
echo "🐳 Creating Docker services configuration..."
cat > docker-compose.yml << 'EOF'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: finalverse
      POSTGRES_USER: finalverse
      POSTGRES_PASSWORD: finalverse_secret
    ports:
      - "5432:5432"
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U finalverse"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - ./data/redis:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 5

  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
      - "6334:6334"
    volumes:
      - ./data/qdrant:/qdrant/storage
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:6333/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  minio:
    image: minio/minio:latest
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - ./data/minio:/data
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    command: server /data --console-address ":9001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

networks:
  default:
    name: finalverse-network
EOF

# Start data layer
echo "🗄️ Starting data layer services..."
docker-compose up -d

# Wait for services to be healthy
echo "⏳ Waiting for data services to be ready..."
sleep 15

# Check if services are healthy
echo "🔍 Checking service health..."
for service in postgres redis qdrant minio; do
    if docker-compose ps $service | grep -q "healthy\|Up"; then
        echo "✅ $service is ready"
    else
        echo "⚠️ $service may not be fully ready yet"
    fi
done

# Initialize database schema
echo "📊 Initializing database schema..."
cat > init_db.sql << 'EOF'
-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Players table
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- Player resonance tracking
CREATE TABLE IF NOT EXISTS player_resonance (
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    creative_resonance BIGINT DEFAULT 0,
    exploration_resonance BIGINT DEFAULT 0,
    restoration_resonance BIGINT DEFAULT 0,
    total_resonance BIGINT GENERATED ALWAYS AS (creative_resonance + exploration_resonance + restoration_resonance) STORED,
    attunement_tier INTEGER DEFAULT 1,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id)
);

-- Echo bonds
CREATE TABLE IF NOT EXISTS echo_bonds (
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    echo_id VARCHAR(50) NOT NULL,
    bond_level FLOAT DEFAULT 0.0,
    last_interaction TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    abilities_unlocked TEXT[] DEFAULT '{}',
    PRIMARY KEY (player_id, echo_id)
);

-- World regions
CREATE TABLE IF NOT EXISTS regions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    harmony_level FLOAT DEFAULT 50.0 CHECK (harmony_level >= 0 AND harmony_level <= 100),
    corruption_level FLOAT DEFAULT 0.0 CHECK (corruption_level >= 0 AND corruption_level <= 100),
    weather VARCHAR(50) DEFAULT 'clear',
    population INTEGER DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Event log for the living world
CREATE TABLE IF NOT EXISTS event_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    player_id UUID REFERENCES players(id),
    region_id UUID REFERENCES regions(id),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    processed BOOLEAN DEFAULT false
);

-- Melodies and songweaving
CREATE TABLE IF NOT EXISTS melodies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    melody_data JSONB NOT NULL,
    harmony_type VARCHAR(50) NOT NULL,
    power_level FLOAT DEFAULT 1.0,
    location_x FLOAT,
    location_y FLOAT, 
    location_z FLOAT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Player chronicles/achievements
CREATE TABLE IF NOT EXISTS player_achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    achievement_id VARCHAR(100) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    achieved_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    significance VARCHAR(50) DEFAULT 'personal'
);

-- Insert default regions
INSERT INTO regions (name, display_name, harmony_level, corruption_level, weather) VALUES
    ('terra_nova', 'Terra Nova', 75.0, 5.0, 'clear'),
    ('aethelgard', 'Aethelgard', 45.0, 25.0, 'misty'),
    ('technos_prime', 'Technos Prime', 60.0, 15.0, 'artificial'),
    ('whispering_wilds', 'Whispering Wilds', 80.0, 3.0, 'natural'),
    ('star_sailor_expanse', 'Star-Sailor Expanse', 55.0, 10.0, 'cosmic')
ON CONFLICT (name) DO NOTHING;

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_event_log_timestamp ON event_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_event_log_player ON event_log(player_id);
CREATE INDEX IF NOT EXISTS idx_event_log_type ON event_log(event_type);
CREATE INDEX IF NOT EXISTS idx_player_resonance_total ON player_resonance(total_resonance);
CREATE INDEX IF NOT EXISTS idx_melodies_player ON melodies(player_id);
CREATE INDEX IF NOT EXISTS idx_melodies_created ON melodies(created_at);

EOF

# Try to initialize the database
if docker exec $(docker-compose ps -q postgres) psql -U finalverse -d finalverse -f - < init_db.sql 2>/dev/null; then
    echo "✅ Database schema initialized successfully"
else
    echo "⚠️ Database initialization may have failed, but continuing..."
fi

rm -f init_db.sql

echo "🚀 Finalverse MVP data layer is ready!"
echo ""
echo "📍 Data Services Status:"
echo "   - PostgreSQL: localhost:5432 (finalverse/finalverse_secret)"
echo "   - Redis: localhost:6379"
echo "   - Qdrant: http://localhost:6333"
echo "   - MinIO: http://localhost:9001 (minioadmin/minioadmin)"
echo ""
echo "🎮 Ready to start Finalverse services!"
echo "   Run ./start_services.sh to start the game services"
echo "   Run ./stop_services.sh to stop everything"
echo ""

# Create service management scripts
cat > start_services.sh << 'EOF'
#!/bin/bash
echo "🎵 Starting Finalverse Services..."

# Check if data services are running
if ! docker-compose ps | grep -q "Up"; then
    echo "🗄️ Starting data services first..."
    docker-compose up -d
    sleep 10
fi

# Create logs directory
mkdir -p logs

# Start services based on what exists
services_to_start=""

# Check which services we can actually start
if [ -f "target/release/websocket-gateway" ] || [ -f "target/debug/websocket-gateway" ]; then
    services_to_start="$services_to_start websocket-gateway"
fi

if [ -f "target/release/ai-orchestra" ] || [ -f "target/debug/ai-orchestra" ]; then
    services_to_start="$services_to_start ai-orchestra"
fi

if [ -f "target/release/song-engine" ] || [ -f "target/debug/song-engine" ]; then
    services_to_start="$services_to_start song-engine"
fi

if [ -f "target/release/story-engine" ] || [ -f "target/debug/story-engine" ]; then
    services_to_start="$services_to_start story-engine"
fi

if [ -f "target/release/echo-engine" ] || [ -f "target/debug/echo-engine" ]; then
    services_to_start="$services_to_start echo-engine"
fi

if [ -z "$services_to_start" ]; then
    echo "❌ No services found to start. Run 'cargo build --workspace' first."
    exit 1
fi

# Use tmux if available for better service management
if command -v tmux >/dev/null 2>&1; then
    echo "🖥️ Starting services in tmux session 'finalverse'..."
    
    # Kill existing session if it exists
    tmux kill-session -t finalverse 2>/dev/null || true
    
    # Create new session
    tmux new-session -d -s finalverse -x 120 -y 30
    
    # Window counter
    window=0
    
    for service in $services_to_start; do
        if [ $window -eq 0 ]; then
            # Use the first window
            tmux rename-window -t finalverse:0 "$service"
            tmux send-keys -t finalverse:0 "RUST_LOG=info cargo run --bin $service" C-m
        else
            # Create new windows for other services
            tmux new-window -t finalverse:$window -n "$service"
            tmux send-keys -t finalverse:$window "RUST_LOG=info cargo run --bin $service" C-m
        fi
        window=$((window + 1))
        sleep 2
    done
    
    echo "✅ Services started in tmux session 'finalverse'"
    echo "   Run 'tmux attach -t finalverse' to view service logs"
    echo "   Use Ctrl+B then [ to scroll through logs"
    echo "   Use Ctrl+B then d to detach from session"
    
else
    echo "🖥️ Starting services in background (tmux not available)..."
    
    for service in $services_to_start; do
        echo "🚀 Starting $service..."
        RUST_LOG=info cargo run --bin $service > logs/$service.log 2>&1 &
        echo $! > logs/$service.pid
        sleep 2
    done
    
    echo "✅ Services started in background"
    echo "   Logs available in logs/ directory"
    echo "   PIDs stored in logs/*.pid files"
fi

# Wait a moment for services to start
sleep 5

echo ""
echo "🌐 Service Status:"
for service in $services_to_start; do
    case $service in
        "websocket-gateway")
            port=3000
            echo "   📡 WebSocket Gateway: ws://localhost:$port/ws"
            ;;
        "ai-orchestra")
            port=3001
            echo "   🤖 AI Orchestra: http://localhost:$port/health"
            ;;
        "song-engine")
            port=3002
            echo "   🎵 Song Engine: http://localhost:$port/health"
            ;;
        "story-engine")
            port=3003
            echo "   📖 Story Engine: http://localhost:$port/health"
            ;;
        "echo-engine")
            port=3004
            echo "   🔮 Echo Engine: http://localhost:$port/health"
            ;;
    esac
done

echo ""
echo "🎮 Ready to test:"
echo "   ./test_services.sh - Test all service endpoints"
echo "   Open Client_WebSocket.html in your browser to test WebSocket"
echo ""
echo "🛑 To stop:"
echo "   ./stop_services.sh"
EOF

cat > stop_services.sh << 'EOF'
#!/bin/bash
echo "🛑 Stopping Finalverse Services..."

# Stop tmux session if it exists
if tmux has-session -t finalverse 2>/dev/null; then
    echo "🖥️ Stopping tmux session..."
    tmux kill-session -t finalverse
    echo "✅ Tmux session stopped"
fi

# Stop background processes if they exist
if [ -d "logs" ]; then
    echo "🔄 Stopping background processes..."
    for pidfile in logs/*.pid; do
        if [ -f "$pidfile" ]; then
            pid=$(cat "$pidfile")
            service=$(basename "$pidfile" .pid)
            if kill "$pid" 2>/dev/null; then
                echo "✅ Stopped $service (PID $pid)"
            else
                echo "⚠️ Process $service (PID $pid) was not running"
            fi
            rm -f "$pidfile"
        fi
    done
fi

# Stop Docker services
echo "🐳 Stopping data services..."
docker-compose down

echo ""
echo "✅ All Finalverse services stopped"
echo "   Data is preserved in ./data/ directory"
echo "   Logs are preserved in ./logs/ directory"
EOF

cat > test_services.sh << 'EOF'
#!/bin/bash
echo "🧪 Testing Finalverse Services..."
echo "================================="

# Test data services first
echo "📊 Testing Data Services:"

echo -n "   🐘 PostgreSQL: "
if docker-compose exec -T postgres pg_isready -U finalverse >/dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo -n "   🔴 Redis: "
if docker-compose exec -T redis redis-cli ping >/dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo -n "   🔍 Qdrant: "
if curl -s http://localhost:6333/health >/dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo -n "   📦 MinIO: "
if curl -s http://localhost:9000/minio/health/live >/dev/null 2>&1; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

echo ""
echo "🎵 Testing Game Services:"

# Test each service with timeout
test_service() {
    local name=$1
    local url=$2
    local port=$3
    
    echo -n "   $name: "
    if timeout 5 curl -s "$url" >/dev/null 2>&1; then
        echo "✅ Running (port $port)"
    else
        echo "❌ Not responding"
    fi
}

test_service "📡 WebSocket Gateway" "http://localhost:3000/health" "3000"
test_service "🤖 AI Orchestra" "http://localhost:3001/health" "3001"
test_service "🎵 Song Engine" "http://localhost:3002/health" "3002"
test_service "📖 Story Engine" "http://localhost:3003/health" "3003"
test_service "🔮 Echo Engine" "http://localhost:3004/health" "3004"

echo ""
echo "🌐 WebSocket Test:"
echo "   Open Client_WebSocket.html in your browser"
echo "   Or connect to: ws://localhost:3000/ws"

echo ""
echo "📈 Service Endpoints:"
echo "   http://localhost:3000/health - WebSocket Gateway"
echo "   http://localhost:3001/health - AI Orchestra"
echo "   http://localhost:3002/health - Song Engine"
echo "   http://localhost:3003/health - Story Engine"
echo "   http://localhost:3004/health - Echo Engine"
EOF

# Make scripts executable
chmod +x start_services.sh stop_services.sh test_services.sh

echo "📝 Management scripts created:"
echo "   ./start_services.sh - Start all Finalverse services"
echo "   ./stop_services.sh - Stop all services"
echo "   ./test_services.sh - Test service connectivity"
echo ""
echo "🎉 Finalverse MVP setup complete!"