#!/bin/bash
# scripts/finalverse.sh - Consolidated Finalverse Development CLI
# Compatible with bash 3.5+ and auto-detects project structure

set -e

# Auto-detect project structure
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Configuration
LOG_DIR="$PROJECT_ROOT/logs"
DATA_DIR="$PROJECT_ROOT/data"
CONFIG_DIR="$PROJECT_ROOT/config"
BACKUP_DIR="$PROJECT_ROOT/backups"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Service definitions (bash 3.5 compatible)
GAME_SERVICES="websocket-gateway:3000 api-gateway:8080 ai-orchestra:3004 song-engine:3001 story-engine:3005 echo-engine:3003 world-engine:3002 harmony-service:3006 asset-service:3007 community:3008 silence-service:3009 procedural-gen:3010 behavior-ai:3011"
DATA_SERVICES="postgres:5432 redis:6379 qdrant:6333 minio:9000"

# Logging functions
log() { echo -e "${GREEN}$(date '+%H:%M:%S')${NC} $1"; }
info() { echo -e "${BLUE}$(date '+%H:%M:%S')${NC} ℹ️  $1"; }
warn() { echo -e "${YELLOW}$(date '+%H:%M:%S')${NC} ⚠️  $1"; }
error() { echo -e "${RED}$(date '+%H:%M:%S')${NC} ❌ $1" >&2; }
success() { echo -e "${GREEN}$(date '+%H:%M:%S')${NC} ✅ $1"; }

# Banner
show_banner() {
    echo -e "${PURPLE}🎵 Finalverse Development CLI${NC}"
    echo -e "${CYAN}Project: $PROJECT_ROOT${NC}"
    echo ""
}

# Helper functions for service management
get_service_port() {
    local service=$1
    echo "$GAME_SERVICES" | tr ' ' '\n' | grep "^$service:" | cut -d: -f2
}

get_all_services() {
    echo "$GAME_SERVICES" | tr ' ' '\n' | cut -d: -f1
}

is_service_running() {
    local service=$1
    local port=$(get_service_port "$service")
    [ -n "$port" ] && lsof -i :$port >/dev/null 2>&1
}

get_service_pid() {
    local service=$1
    local port=$(get_service_port "$service")
    [ -n "$port" ] && lsof -ti :$port 2>/dev/null
}

is_data_service_healthy() {
    local service=$1
    case $service in
        "postgres")
            docker-compose exec -T postgres pg_isready -U finalverse >/dev/null 2>&1
            ;;
        "redis")
            docker-compose exec -T redis redis-cli ping >/dev/null 2>&1
            ;;
        "qdrant")
            curl -s http://localhost:6333/health >/dev/null 2>&1
            ;;
        "minio")
            curl -s http://localhost:9000/minio/health/live >/dev/null 2>&1
            ;;
        *)
            return 1
            ;;
    esac
}

# Directory setup
setup_directories() {
    info "Setting up directory structure..."
    mkdir -p "$LOG_DIR" "$DATA_DIR"/{postgres,redis,qdrant,minio} "$CONFIG_DIR" "$BACKUP_DIR"
    success "Directories created"
}

# Build function
build_services() {
    show_banner
    info "Building Finalverse workspace..."
    
    if cargo build --workspace --release; then
        success "Build completed successfully"
        echo ""
        echo "Built services:"
        for service in $(get_all_services); do
            if [ -x "target/release/$service" ]; then
                echo "  ✅ $service"
            fi
        done
    else
        error "Build failed"
        return 1
    fi
}

# Data services management
start_data_services() {
    info "Starting data services..."
    
    # Create docker-compose.yml if it doesn't exist
    if [ ! -f "docker-compose.yml" ]; then
        create_docker_compose
    fi
    
    docker-compose up -d
    info "Waiting for data services to be ready..."
    sleep 15
    
    # Check health
    for service in postgres redis qdrant minio; do
        if is_data_service_healthy "$service"; then
            success "$service is healthy"
        else
            warn "$service may not be fully ready"
        fi
    done
}

stop_data_services() {
    info "Stopping data services..."
    docker-compose down
}

create_docker_compose() {
    info "Creating docker-compose.yml..."
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
}

# Game services management
start_service() {
    local service=$1
    local port=$(get_service_port "$service")
    
    if is_service_running "$service"; then
        warn "$service already running on port $port"
        return 0
    fi
    
    if [ ! -f "target/release/$service" ]; then
        error "Binary not found: target/release/$service (run 'build' first)"
        return 1
    fi
    
    info "Starting $service on port $port..."
    RUST_LOG=info target/release/$service > "$LOG_DIR/${service}.log" 2>&1 &
    local pid=$!
    echo $pid > "$LOG_DIR/${service}.pid"
    
    sleep 2
    if kill -0 $pid 2>/dev/null && is_service_running "$service"; then
        success "$service started (PID: $pid, Port: $port)"
        return 0
    else
        error "$service failed to start"
        [ -f "$LOG_DIR/${service}.log" ] && tail -5 "$LOG_DIR/${service}.log" >&2
        return 1
    fi
}

stop_service() {
    local service=$1
    local pid_file="$LOG_DIR/${service}.pid"
    
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 $pid 2>/dev/null; then
            info "Stopping $service (PID: $pid)..."
            kill $pid
            sleep 2
            kill -0 $pid 2>/dev/null && kill -9 $pid 2>/dev/null
            rm -f "$pid_file"
            success "Stopped $service"
        else
            rm -f "$pid_file"
        fi
    fi
    
    # Also kill by port
    local port=$(get_service_port "$service")
    local port_pid=$(lsof -ti :$port 2>/dev/null || true)
    [ -n "$port_pid" ] && kill -9 $port_pid 2>/dev/null || true
}

start_all_services() {
    show_banner
    setup_directories
    start_data_services
    
    info "Starting game services..."
    failed_services=""
    
    for service in $(get_all_services); do
        if ! start_service "$service"; then
            failed_services="$failed_services $service"
        fi
    done
    
    if [ -z "$failed_services" ]; then
        success "All services started successfully"
    else
        error "Failed to start:$failed_services"
    fi
    
    show_service_urls
}

stop_all_services() {
    show_banner
    info "Stopping all services..."
    
    # Stop tmux session if it exists
    tmux kill-session -t finalverse 2>/dev/null || true
    
    # Stop game services
    for service in $(get_all_services); do
        stop_service "$service"
    done
    
    # Stop data services
    stop_data_services
    success "All services stopped"
}

# Status and monitoring
show_status() {
    show_banner
    echo -e "${BLUE}🎵 Service Status${NC}"
    echo "=================================="
    
    local running=0
    local total=0
    
    echo ""
    echo -e "${CYAN}📊 Data Services:${NC}"
    for service_port in $DATA_SERVICES; do
        local service=$(echo "$service_port" | cut -d: -f1)
        total=$((total + 1))
        if is_data_service_healthy "$service"; then
            echo -e "  ✅ ${GREEN}$service${NC}"
            running=$((running + 1))
        else
            echo -e "  ❌ ${RED}$service${NC}"
        fi
    done
    
    echo ""
    echo -e "${CYAN}🎮 Game Services:${NC}"
    for service in $(get_all_services); do
        local port=$(get_service_port "$service")
        total=$((total + 1))
        if is_service_running "$service"; then
            local pid=$(get_service_pid "$service")
            echo -e "  ✅ ${GREEN}$service${NC} (Port: $port, PID: $pid)"
            running=$((running + 1))
        else
            echo -e "  ❌ ${RED}$service${NC} (Port: $port)"
        fi
    done
    
    echo ""
    echo -e "${CYAN}📊 Summary: $running/$total services running${NC}"
    
    # Show tmux session if exists
    if tmux has-session -t finalverse 2>/dev/null; then
        echo ""
        echo -e "${CYAN}📺 Tmux session 'finalverse' is active${NC}"
    fi
}

show_service_urls() {
    echo ""
    echo -e "${CYAN}🌐 Service URLs:${NC}"
    for service in $(get_all_services); do
        local port=$(get_service_port "$service")
        if is_service_running "$service"; then
            case $service in
                "websocket-gateway") echo "  📡 WebSocket: ws://localhost:$port/ws" ;;
                "api-gateway") echo "  🚪 API Gateway: http://localhost:$port/health" ;;
                "ai-orchestra") echo "  🤖 AI Orchestra: http://localhost:$port/health" ;;
                "song-engine") echo "  🎵 Song Engine: http://localhost:$port/health" ;;
                "story-engine") echo "  📖 Story Engine: http://localhost:$port/health" ;;
                "echo-engine") echo "  🔮 Echo Engine: http://localhost:$port/health" ;;
                "world-engine") echo "  🌍 World Engine: http://localhost:$port/health" ;;
                "harmony-service") echo "  🎼 Harmony Service: http://localhost:$port/health" ;;
                "asset-service") echo "  📦 Asset Service: http://localhost:$port/health" ;;
                "community") echo "  👥 Community: http://localhost:$port/health" ;;
                "silence-service") echo "  🔇 Silence Service: http://localhost:$port/health" ;;
                "procedural-gen") echo "  ⚙️  Procedural Gen: http://localhost:$port/health" ;;
                "behavior-ai") echo "  🧠 Behavior AI: http://localhost:$port/health" ;;
                *) echo "  🎯 $service: http://localhost:$port/health" ;;
            esac
        fi
    done
}

# Testing and health checks
test_services() {
    show_banner
    info "Testing service connectivity..."
    
    echo ""
    echo -e "${CYAN}📊 Data Services:${NC}"
    for service_port in $DATA_SERVICES; do
        local service=$(echo "$service_port" | cut -d: -f1)
        if is_data_service_healthy "$service"; then
            echo -e "  ✅ ${GREEN}$service${NC}"
        else
            echo -e "  ❌ ${RED}$service${NC}"
        fi
    done
    
    echo ""
    echo -e "${CYAN}🎮 Game Services:${NC}"
    for service in $(get_all_services); do
        local port=$(get_service_port "$service")
        if timeout 3 curl -s "http://localhost:$port/health" >/dev/null 2>&1; then
            echo -e "  ✅ ${GREEN}$service${NC} (http://localhost:$port/health)"
        else
            echo -e "  ❌ ${RED}$service${NC} (http://localhost:$port/health)"
        fi
    done
    
    echo ""
    echo -e "${CYAN}🌐 WebSocket Test:${NC}"
    echo "  Open Client_WebSocket.html in your browser"
    echo "  Connect to: ws://localhost:3000/ws"
}

# Log management
show_logs() {
    local service=${1:-"all"}
    local lines=${2:-50}
    
    if [ "$service" = "all" ]; then
        if tmux has-session -t finalverse 2>/dev/null; then
            tmux attach -t finalverse
        elif [ -d "$LOG_DIR" ] && [ "$(ls -A "$LOG_DIR"/*.log 2>/dev/null)" ]; then
            tail -f "$LOG_DIR"/*.log
        else
            error "No logs available"
        fi
    else
        local log_file="$LOG_DIR/${service}.log"
        if [ -f "$log_file" ]; then
            tail -n "$lines" "$log_file"
        else
            error "Log file not found: $log_file"
        fi
    fi
}

follow_logs() {
    local service=${1:-"all"}
    
    if [ "$service" = "all" ]; then
        if tmux has-session -t finalverse 2>/dev/null; then
            tmux attach -t finalverse
        elif [ -d "$LOG_DIR" ] && [ "$(ls -A "$LOG_DIR"/*.log 2>/dev/null)" ]; then
            tail -f "$LOG_DIR"/*.log
        else
            error "No logs available"
        fi
    else
        local log_file="$LOG_DIR/${service}.log"
        if [ -f "$log_file" ]; then
            tail -f "$log_file"
        else
            error "Log file not found: $log_file"
        fi
    fi
}

# Cleanup and maintenance
clean_ports() {
    show_banner
    info "Cleaning port conflicts..."
    
    # Extract all ports from services
    all_ports=""
    for service_port in $GAME_SERVICES $DATA_SERVICES; do
        port=$(echo "$service_port" | cut -d: -f2)
        all_ports="$all_ports $port"
    done
    
    for port in $all_ports; do
        local pid=$(lsof -ti:$port 2>/dev/null || true)
        if [ -n "$pid" ]; then
            warn "Killing process $pid on port $port"
            kill -9 "$pid" 2>/dev/null || true
        fi
    done
    
    success "Ports cleaned"
}

clean_all() {
    show_banner
    info "Performing complete cleanup..."
    
    # Stop all services
    stop_all_services
    
    # Clean logs and PIDs
    rm -rf "$LOG_DIR"/*.log "$LOG_DIR"/*.pid
    
    # Optional: Clean data
    read -p "Clean all data? This will remove databases and uploaded files (y/N): " -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$DATA_DIR"/*
        success "Data cleaned"
    fi
    
    # Optional: Clean build cache
    read -p "Clean Rust build cache? This will require full rebuild (y/N): " -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo clean
        success "Build cache cleaned"
    fi
    
    success "Cleanup complete"
}

# Backup and restore
backup_data() {
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_path="$BACKUP_DIR/$timestamp"
    
    info "Creating backup at $backup_path..."
    mkdir -p "$backup_path"
    
    # Backup data directories
    [ -d "$DATA_DIR" ] && cp -r "$DATA_DIR" "$backup_path/"
    [ -d "$LOG_DIR" ] && cp -r "$LOG_DIR" "$backup_path/"
    [ -d "$CONFIG_DIR" ] && cp -r "$CONFIG_DIR" "$backup_path/"
    [ -f "docker-compose.yml" ] && cp "docker-compose.yml" "$backup_path/"
    
    success "Backup created at $backup_path"
}

# Monitor mode
monitor_services() {
    info "Starting service monitor (Press Ctrl+C to exit)..."
    
    while true; do
        clear
        echo -e "${PURPLE}🎵 Finalverse Service Monitor - $(date)${NC}"
        echo "=============================================="
        
        local healthy=0
        local total=0
        
        # Check data services
        echo ""
        echo -e "${CYAN}📊 Data Services:${NC}"
        for service_port in $DATA_SERVICES; do
            local service=$(echo "$service_port" | cut -d: -f1)
            total=$((total + 1))
            if is_data_service_healthy "$service"; then
                echo -e "  ✅ ${GREEN}$service${NC}"
                healthy=$((healthy + 1))
            else
                echo -e "  ❌ ${RED}$service${NC}"
            fi
        done
        
        # Check game services
        echo ""
        echo -e "${CYAN}🎮 Game Services:${NC}"
        for service in $(get_all_services); do
            local port=$(get_service_port "$service")
            total=$((total + 1))
            if is_service_running "$service"; then
                echo -e "  ✅ ${GREEN}$service${NC} ($port)"
                healthy=$((healthy + 1))
            else
                echo -e "  ❌ ${RED}$service${NC} ($port)"
            fi
        done
        
        echo ""
        echo -e "${CYAN}📊 Health: $healthy/$total services running${NC}"
        
        # Show recent activity
        echo ""
        echo -e "${CYAN}📜 Recent Activity:${NC}"
        if [ -d "$LOG_DIR" ]; then
            find "$LOG_DIR" -name "*.log" -mmin -1 2>/dev/null | head -3 | while read -r log; do
                local service=$(basename "$log" .log)
                local last_line=$(tail -1 "$log" 2>/dev/null | head -c 60)
                [ -n "$last_line" ] && echo "  $service: $last_line..."
            done
        fi
        
        sleep 5
    done
}

# Help system
show_help() {
    show_banner
    echo "Usage: $0 <command> [options]"
    echo ""
    echo -e "${CYAN}🔨 Build & Setup:${NC}"
    echo "  build                 Build all Rust services"
    echo "  setup                 Setup directories and docker-compose"
    echo ""
    echo -e "${CYAN}🚀 Service Management:${NC}"
    echo "  start                 Start all services (data + game)"
    echo "  stop                  Stop all services"
    echo "  restart [service]     Restart all or specific service"
    echo "  status                Show service status"
    echo ""
    echo -e "${CYAN}🧪 Testing & Monitoring:${NC}"
    echo "  test                  Test service connectivity"
    echo "  monitor               Real-time service monitoring"
    echo "  health                Quick health check"
    echo ""
    echo -e "${CYAN}📜 Logs & Debugging:${NC}"
    echo "  logs [service] [n]    Show last n lines of logs"
    echo "  follow [service]      Follow logs in real-time"
    echo ""
    echo -e "${CYAN}🧹 Maintenance:${NC}"
    echo "  clean-ports           Kill processes on Finalverse ports"
    echo "  clean                 Complete cleanup"
    echo "  backup                Backup all data"
    echo ""
    echo -e "${CYAN}Examples:${NC}"
    echo "  $0 build && $0 start"
    echo "  $0 logs websocket-gateway 100"
    echo "  $0 follow song-engine"
    echo "  $0 restart harmony-service"
    echo ""
    echo -e "${CYAN}📋 Available services:${NC}"
    echo "  $(get_all_services | tr '\n' ' ')"
}

# Main command dispatcher
case "${1:-help}" in
    "build")
        build_services
        ;;
    "setup")
        setup_directories
        create_docker_compose
        ;;
    "start")
        start_all_services
        ;;
    "stop")
        stop_all_services
        ;;
    "restart")
        if [ -n "$2" ]; then
            stop_service "$2"
            sleep 2
            start_service "$2"
        else
            stop_all_services
            sleep 3
            start_all_services
        fi
        ;;
    "status")
        show_status
        ;;
    "test")
        test_services
        ;;
    "monitor")
        monitor_services
        ;;
    "health")
        test_services
        ;;
    "logs")
        show_logs "$2" "$3"
        ;;
    "follow")
        follow_logs "$2"
        ;;
    "clean-ports")
        clean_ports
        ;;
    "clean")
        clean_all
        ;;
    "backup")
        backup_data
        ;;
    "help"|*)
        show_help
        ;;
esac