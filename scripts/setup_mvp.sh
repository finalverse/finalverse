#!/bin/bash
# scripts/setup_mvp.sh

set -e

echo "🌟 Setting up Finalverse MVP..."

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo is required but not installed. Visit https://rustup.rs/"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed."; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed."; exit 1; }

# Create directory structure
echo "📁 Creating directory structure..."
# mkdir -p config
mkdir -p data/{postgres,redis,qdrant,minio}
mkdir -p logs

# Build all services
echo "🔨 Building Rust services..."
cargo build --workspace

# Start data layer
echo "🗄️ Starting data layer services..."
docker-compose up -d postgres redis qdrant minio

# Wait for services to be healthy
echo "⏳ Waiting for data services to be ready..."
sleep 10

# Initialize database schema (create a simple schema)
echo "📊 Initializing database schema..."
cat > init_db.sql << 'EOF'
-- Players table
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Resonance tracking
CREATE TABLE IF NOT EXISTS player_resonance (
    player_id UUID REFERENCES players(id),
    creative BIGINT DEFAULT 0,
    exploration BIGINT DEFAULT 0,
    restoration BIGINT DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id)
);

-- Echo bonds
CREATE TABLE IF NOT EXISTS echo_bonds (
    player_id UUID REFERENCES players(id),
    echo_id VARCHAR(50),
    bond_level INTEGER DEFAULT 0,
    last_interaction TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, echo_id)
);

-- World regions
CREATE TABLE IF NOT EXISTS regions (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    harmony_level FLOAT DEFAULT 50.0,
    discord_level FLOAT DEFAULT 0.0,
    weather VARCHAR(50),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Event log
CREATE TABLE IF NOT EXISTS event_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100),
    event_data JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
EOF

PGPASSWORD=finalverse_secret psql -h localhost -U finalverse -d finalverse -f init_db.sql || echo "⚠️ Database initialization skipped (may already exist)"
rm init_db.sql

# Start services in development mode
echo "🚀 Starting Finalverse services..."

# Create a tmux session for running services
if command -v tmux >/dev/null 2>&1; then
    tmux new-session -d -s finalverse
    
    # Song Engine
    tmux new-window -t finalverse:1 -n 'song-engine'
    tmux send-keys -t finalverse:1 'RUST_LOG=info cargo run --bin song-engine' C-m
    
    # World Engine
    tmux new-window -t finalverse:2 -n 'world-engine'
    tmux send-keys -t finalverse:2 'RUST_LOG=info cargo run --bin world-engine' C-m
    
    # Echo Engine
    tmux new-window -t finalverse:3 -n 'echo-engine'
    tmux send-keys -t finalverse:3 'RUST_LOG=info cargo run --bin echo-engine' C-m
    
    # AI Orchestra
    tmux new-window -t finalverse:4 -n 'ai-orchestra'
    tmux send-keys -t finalverse:4 'RUST_LOG=info cargo run --bin ai-orchestra' C-m
    
    echo "✅ Services started in tmux session 'finalverse'"
    echo "   Run 'tmux attach -t finalverse' to view logs"
else
    echo "ℹ️ tmux not found, starting services in background..."
    RUST_LOG=info cargo run --bin song-engine > logs/song-engine.log 2>&1 &
    RUST_LOG=info cargo run --bin world-engine > logs/world-engine.log 2>&1 &
    RUST_LOG=info cargo run --bin echo-engine > logs/echo-engine.log 2>&1 &
    RUST_LOG=info cargo run --bin ai-orchestra > logs/ai-orchestra.log 2>&1 &
    echo "✅ Services started, logs available in logs/"
fi

# Wait for services to start
echo "⏳ Waiting for services to start..."
sleep 5

# Start API Gateway
echo "🌐 Starting API Gateway..."
docker-compose up -d api-gateway

echo ""
echo "🎉 Finalverse MVP is ready!"
echo ""
echo "📍 Service Endpoints:"
echo "   - API Gateway: http://localhost:8080"
echo "   - Song Engine: http://localhost:3001"
echo "   - World Engine: http://localhost:3002"
echo "   - Echo Engine: http://localhost:3003"
echo "   - AI Orchestra: http://localhost:3004"
echo ""
echo "📊 Data Services:"
echo "   - PostgreSQL: localhost:5432 (user: finalverse, pass: finalverse_secret)"
echo "   - Redis: localhost:6379"
echo "   - Qdrant: http://localhost:6333"
echo "   - MinIO: http://localhost:9001 (user: minioadmin, pass: minioadmin)"
echo ""
echo "🎮 To start the client:"
echo "   cargo run --bin mock-client"
echo ""
echo "🛑 To stop all services:"
echo "   docker-compose down"
echo "   tmux kill-session -t finalverse (if using tmux)"
echo ""

# Create a quick test script
cat > test_services.sh << 'EOF'
#!/bin/bash
echo "Testing Finalverse services..."

echo "🎵 Song Engine:" && curl -s http://localhost:3001/info | jq .
echo "🌍 World Engine:" && curl -s http://localhost:3002/info | jq .
echo "✨ Echo Engine:" && curl -s http://localhost:3003/info | jq .
echo "🤖 AI Orchestra:" && curl -s http://localhost:3004/info | jq .

echo -e "\n📡 Testing via API Gateway:"
curl -s http://localhost:8080/api/song/info | jq .
EOF

chmod +x test_services.sh

echo "Run ./test_services.sh to test all services"