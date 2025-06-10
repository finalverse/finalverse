#!/bin/bash
# quick_setup.sh - One-time setup for Finalverse project structure

set -e

echo "🎵 Finalverse Quick Setup"
echo "========================"

# Check if we're in the right place
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this from your Finalverse project root (where Cargo.toml is)"
    exit 1
fi

PROJ_HOME="$(pwd)"
echo "📁 Project root: $PROJ_HOME"

# Create scripts directory if it doesn't exist
mkdir -p scripts

# Check for port conflicts and offer to clean them
echo "🔍 Checking for port conflicts..."
ports_in_use=()
for port in 3000 3001 3002 3003 3004 5432 6379 6333 9000 9001; do
    if lsof -ti:$port >/dev/null 2>&1; then
        ports_in_use+=($port)
    fi
done

if [ ${#ports_in_use[@]} -gt 0 ]; then
    echo "⚠️  The following ports are in use: ${ports_in_use[*]}"
    echo "These ports are needed for Finalverse services."
    read -p "Would you like to kill processes on these ports? (y/n): " -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        for port in "${ports_in_use[@]}"; do
            echo "🔫 Killing processes on port $port..."
            lsof -ti:$port | xargs kill -9 2>/dev/null || true
        done
        echo "✅ Ports cleaned"
    else
        echo "⚠️  You may encounter port conflicts. You can run './scripts/finalverse.sh clean-ports' later."
    fi
fi

# Clean up any existing Docker containers/networks
echo "🧹 Cleaning up existing Docker resources..."
docker-compose down -v 2>/dev/null || true
docker network rm finalverse-network 2>/dev/null || true

# Copy the main finalverse.sh script to scripts directory
echo "📝 Creating management scripts..."

# Create the main finalverse.sh script in scripts/
cat > scripts/finalverse.sh << 'EOF'
#!/bin/bash
set -e

# Determine project structure
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJ_HOME="$(dirname "$SCRIPT_DIR")"
cd "$PROJ_HOME"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

show_banner() {
    echo -e "${PURPLE}🎵 Finalverse Development Helper${NC}"
    echo -e "${BLUE}Project: $PROJ_HOME${NC}"
    echo ""
}

build_project() {
    show_banner
    log_info "Building Finalverse workspace..."
    
    if cargo build --workspace --release; then
        log_success "Build completed successfully"
        echo ""
        echo "Built services:"
        for binary in target/release/*-*; do
            if [ -x "$binary" ] && [ ! -d "$binary" ]; then
                echo "  ✅ $(basename "$binary")"
            fi
        done
    else
        log_error "Build failed"
        exit 1
    fi
}

start_services() {
    show_banner
    log_info "Starting Finalverse Services..."
    
    # Start data services first
    if ! docker-compose ps postgres | grep -q "Up"; then
        log_info "Starting data services..."
        docker-compose up -d
        log_info "Waiting for data services to be ready..."
        sleep 15
    fi
    
    # Create logs directory
    mkdir -p logs
    
    # Find available services
    services_found=()
    
    if [ -f "target/release/websocket-gateway" ]; then
        services_found+=("websocket-gateway")
    fi
    
    if [ -f "target/release/ai-orchestra" ]; then
        services_found+=("ai-orchestra")
    fi
    
    if [ -f "target/release/song-engine" ]; then
        services_found+=("song-engine")
    fi
    
    if [ -f "target/release/story-engine" ]; then
        services_found+=("story-engine")
    fi
    
    if [ -f "target/release/echo-engine" ]; then
        services_found+=("echo-engine")
    fi
    
    if [ ${#services_found[@]} -eq 0 ]; then
        log_error "No services found. Run 'build' first."
        exit 1
    fi
    
    log_info "Found services: ${services_found[*]}"
    
    # Use tmux if available
    if command -v tmux >/dev/null 2>&1; then
        log_info "Starting services in tmux session 'finalverse'..."
        
        # Kill existing session
        tmux kill-session -t finalverse 2>/dev/null || true
        
        # Create new session
        tmux new-session -d -s finalverse
        
        # Start each service in a window
        window=0
        for service in "${services_found[@]}"; do
            if [ $window -eq 0 ]; then
                tmux rename-window -t finalverse:0 "$service"
                tmux send-keys -t finalverse:0 "RUST_LOG=info cargo run --bin $service" C-m
            else
                tmux new-window -t finalverse:$window -n "$service"
                tmux send-keys -t finalverse:$window "RUST_LOG=info cargo run --bin $service" C-m
            fi
            window=$((window + 1))
            sleep 2
        done
        
        log_success "Services started in tmux session 'finalverse'"
        echo ""
        echo "🎮 To view logs: tmux attach -t finalverse"
        echo "🎮 To test: ./scripts/finalverse.sh test"
        
    else
        log_info "Starting services in background..."
        
        for service in "${services_found[@]}"; do
            log_info "Starting $service..."
            RUST_LOG=info cargo run --bin $service > logs/$service.log 2>&1 &
            echo $! > logs/$service.pid
            sleep 2
        done
        
        log_success "Services started in background"
        echo "📜 Logs in logs/ directory"
    fi
    
    # Show service URLs
    echo ""
    echo "🌐 Service URLs:"
    [ -f "target/release/websocket-gateway" ] && echo "  📡 WebSocket: ws://localhost:3000/ws"
    [ -f "target/release/ai-orchestra" ] && echo "  🤖 AI Orchestra: http://localhost:3001/health"
    [ -f "target/release/song-engine" ] && echo "  🎵 Song Engine: http://localhost:3002/health"
    [ -f "target/release/story-engine" ] && echo "  📖 Story Engine: http://localhost:3003/health"
    [ -f "target/release/echo-engine" ] && echo "  🔮 Echo Engine: http://localhost:3004/health"
}

stop_services() {
    show_banner
    log_info "Stopping Finalverse services..."
    
    # Stop tmux session
    if tmux has-session -t finalverse 2>/dev/null; then
        tmux kill-session -t finalverse
        log_success "Stopped tmux session"
    fi
    
    # Stop background processes
    if [ -d "logs" ]; then
        for pidfile in logs/*.pid; do
            if [ -f "$pidfile" ]; then
                pid=$(cat "$pidfile")
                service=$(basename "$pidfile" .pid)
                if kill "$pid" 2>/dev/null; then
                    log_success "Stopped $service"
                fi
                rm -f "$pidfile"
            fi
        done
    fi
    
    # Stop data services
    docker-compose down
    log_success "All services stopped"
}

test_services() {
    show_banner
    log_info "Testing Finalverse services..."
    
    echo ""
    echo "📊 Data Services:"
    
    # Test PostgreSQL
    if docker-compose exec -T postgres pg_isready -U finalverse >/dev/null 2>&1; then
        echo "  ✅ PostgreSQL"
    else
        echo "  ❌ PostgreSQL"
    fi
    
    # Test Redis
    if docker-compose exec -T redis redis-cli ping >/dev/null 2>&1; then
        echo "  ✅ Redis"
    else
        echo "  ❌ Redis"
    fi
    
    # Test Qdrant
    if curl -s http://localhost:6333/health >/dev/null 2>&1; then
        echo "  ✅ Qdrant"
    else
        echo "  ❌ Qdrant"
    fi
    
    # Test MinIO
    if curl -s http://localhost:9000/minio/health/live >/dev/null 2>&1; then
        echo "  ✅ MinIO"
    else
        echo "  ❌ MinIO"
    fi
    
    echo ""
    echo "🎵 Game Services:"
    
    # Test each game service
    services=(
        "websocket-gateway:3000:WebSocket Gateway"
        "ai-orchestra:3001:AI Orchestra"
        "song-engine:3002:Song Engine"
        "story-engine:3003:Story Engine"
        "echo-engine:3004:Echo Engine"
    )
    
    for service_info in "${services[@]}"; do
        IFS=':' read -r service port name <<< "$service_info"
        if timeout 3 curl -s "http://localhost:$port/health" >/dev/null 2>&1; then
            echo "  ✅ $name (port $port)"
        else
            echo "  ❌ $name (port $port)"
        fi
    done
    
    echo ""
    echo "🌐 WebSocket Test:"
    echo "  Open Client_WebSocket.html in your browser"
    echo "  Connect to: ws://localhost:3000/ws"
}

clean_ports() {
    show_banner
    log_info "Cleaning port conflicts..."
    
    ports=(3000 3001 3002 3003 3004 5432 6379 6333 9000 9001)
    
    for port in "${ports[@]}"; do
        pid=$(lsof -ti:$port 2>/dev/null || true)
        if [ -n "$pid" ]; then
            log_warning "Killing process $pid on port $port"
            kill -9 $pid 2>/dev/null || true
        fi
    done
    
    log_success "Ports cleaned"
}

show_status() {
    show_banner
    log_info "Finalverse Status"
    
    echo ""
    echo "📊 Data Services:"
    if docker-compose ps 2>/dev/null | grep -q "Up"; then
        docker-compose ps --format "table {{.Service}}\t{{.State}}\t{{.Ports}}"
    else
        echo "  ❌ Docker services not running"
    fi
    
    echo ""
    echo "🎵 Game Services:"
    if tmux has-session -t finalverse 2>/dev/null; then
        echo "  ✅ Running in tmux session 'finalverse'"
        tmux list-windows -t finalverse 2>/dev/null
    else
        echo "  ❌ No tmux session found"
    fi
    
    echo ""
    echo "📁 Built Services:"
    for binary in target/release/*-*; do
        if [ -x "$binary" ] && [ ! -d "$binary" ]; then
            echo "  ✅ $(basename "$binary")"
        fi
    done
}

show_logs() {
    local service=${1:-"all"}
    
    if tmux has-session -t finalverse 2>/dev/null; then
        if [ "$service" = "all" ]; then
            tmux attach -t finalverse
        else
            tmux select-window -t "finalverse:$service" 2>/dev/null && tmux attach -t finalverse
        fi
    elif [ -d "logs" ]; then
        if [ "$service" = "all" ]; then
            tail -f logs/*.log 2>/dev/null || echo "No log files found"
        else
            tail -f "logs/$service.log" 2>/dev/null || echo "Log file not found: logs/$service.log"
        fi
    else
        log_error "No logs available"
    fi
}

show_help() {
    show_banner
    echo "Commands:"
    echo "  build         - Build all Rust services"
    echo "  start         - Start all services (data + game)"
    echo "  stop          - Stop all services"
    echo "  test          - Test service connectivity"
    echo "  status        - Show current status"
    echo "  logs [svc]    - Show logs (all or specific service)"
    echo "  clean-ports   - Kill processes on Finalverse ports"
    echo "  help          - Show this help"
    echo ""
    echo "Examples:"
    echo "  ./scripts/finalverse.sh build"
    echo "  ./scripts/finalverse.sh start"
    echo "  ./scripts/finalverse.sh logs song-engine"
    echo "  ./scripts/finalverse.sh test"
    echo ""
    echo "Service Ports:"
    echo "  WebSocket Gateway: 3000"
    echo "  AI Orchestra: 3001"
    echo "  Song Engine: 3002"
    echo "  Story Engine: 3003"
    echo "  Echo Engine: 3004"
}

# Main command dispatcher
case "${1:-help}" in
    "build")
        build_project
        ;;
    "start")
        start_services
        ;;
    "stop")
        stop_services
        ;;
    "test")
        test_services
        ;;
    "status")
        show_status
        ;;
    "logs")
        show_logs "${2:-all}"
        ;;
    "clean-ports")
        clean_ports
        ;;
    "help"|*)
        show_help
        ;;
esac
EOF

chmod +x scripts/finalverse.sh

# Create a proper docker-compose.yml in project root
echo "🐳 Creating Docker Compose configuration..."

cat > docker-compose.yml << 'EOF'

services:
  postgres:
    image: postgres:15
    container_name: finalverse-postgres
    environment:
      POSTGRES_DB: finalverse
      POSTGRES_USER: finalverse
      POSTGRES_PASSWORD: finalverse_secret
    ports:
      - "5432:5432"
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
    networks:
      - finalverse-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U finalverse"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: finalverse-redis
    ports:
      - "6379:6379"
    volumes:
      - ./data/redis:/data
    networks:
      - finalverse-network
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 5

  qdrant:
    image: qdrant/qdrant:latest
    container_name: finalverse-qdrant
    ports:
      - "6333:6333"
      - "6334:6334"
    volumes:
      - ./data/qdrant:/qdrant/storage
    networks:
      - finalverse-network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:6333/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  minio:
    image: minio/minio:latest
    container_name: finalverse-minio
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - ./data/minio:/data
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    command: server /data --console-address ":9001"
    networks:
      - finalverse-network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

networks:
  finalverse-network:
    driver: bridge
    name: finalverse-network
EOF

# Create necessary directories
echo "📁 Creating directory structure..."
mkdir -p {data/{postgres,redis,qdrant,minio},logs,config}

# Create a simple README for the scripts
cat > scripts/README.md << 'EOF'
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
EOF

echo ""
echo "✅ Finalverse setup complete!"
echo ""
echo "📁 Project structure:"
echo "   $PROJ_HOME/"
echo "   ├── scripts/finalverse.sh    # Main management script"
echo "   ├── docker-compose.yml      # Data services"
echo "   ├── data/                   # Persistent data"
echo "   └── logs/                   # Service logs"
echo ""
echo "🎮 Next steps:"
echo "   1. Build services:  ./scripts/finalverse.sh build"
echo "   2. Start services:  ./scripts/finalverse.sh start"
echo "   3. Test services:   ./scripts/finalverse.sh test"
echo ""
echo "🎵 The Song of Creation awaits! 🌟"