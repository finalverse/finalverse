#!/bin/bash
# fv_services.sh - Sophisticated service orchestration for Finalverse

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJ_HOME="$(dirname "$SCRIPT_DIR")"
LOG_DIR="$PROJ_HOME/logs"
PID_DIR="$PROJ_HOME/.pids"
CONFIG_DIR="$PROJ_HOME/config"

# Create directories
mkdir -p "$LOG_DIR" "$PID_DIR" "$CONFIG_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Service definitions with dependencies
declare -A SERVICE_PORTS=(
    ["registry"]="8500"
    ["api-gateway"]="8080"
    ["websocket-gateway"]="3000"
    ["song-engine"]="3001"
    ["world-engine"]="3002"
    ["echo-engine"]="3003"
    ["ai-orchestra"]="3004"
    ["story-engine"]="3005"
    ["harmony-service"]="3006"
    ["asset-service"]="3007"
    ["community-service"]="3008"
    ["silence-service"]="3009"
    ["procedural-gen"]="3010"
    ["behavior-ai"]="3011"
)

declare -A SERVICE_DEPS=(
    ["registry"]=""
    ["api-gateway"]="registry"
    ["websocket-gateway"]="registry api-gateway"
    ["song-engine"]="registry"
    ["world-engine"]="registry"
    ["echo-engine"]="registry"
    ["ai-orchestra"]="registry"
    ["story-engine"]="registry ai-orchestra"
    ["harmony-service"]="registry"
    ["asset-service"]="registry"
    ["community-service"]="registry"
    ["silence-service"]="registry world-engine"
    ["procedural-gen"]="registry world-engine"
    ["behavior-ai"]="registry ai-orchestra"
)

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_debug() { echo -e "${CYAN}[DEBUG]${NC} $1"; }

# Service health check
check_service_health() {
    local service=$1
    local port=${SERVICE_PORTS[$service]}
    local max_attempts=30
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        if curl -s -f "http://localhost:${port}/health" >/dev/null 2>&1; then
            return 0
        fi
        
        attempt=$((attempt + 1))
        if [ $attempt -lt $max_attempts ]; then
            sleep 1
        fi
    done
    
    return 1
}

# Wait for service dependencies
wait_for_deps() {
    local service=$1
    local deps=${SERVICE_DEPS[$service]}
    
    if [ -n "$deps" ]; then
        log_debug "Waiting for dependencies of $service: $deps"
        for dep in $deps; do
            if ! is_service_running "$dep"; then
                log_error "Dependency $dep is not running for $service"
                return 1
            fi
        done
    fi
    
    return 0
}

# Check if service is running
is_service_running() {
    local service=$1
    local pid_file="$PID_DIR/${service}.pid"
    
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            return 0
        fi
    fi
    
    return 1
}

# Start a single service
start_service() {
    local service=$1
    local port=${SERVICE_PORTS[$service]}
    
    if is_service_running "$service"; then
        log_warning "$service is already running"
        return 0
    fi
    
    # Wait for dependencies
    if ! wait_for_deps "$service"; then
        log_error "Dependencies not met for $service"
        return 1
    fi
    
    log_info "Starting $service on port $port..."
    
    # Set environment variables
    export SERVICE_NAME="$service"
    export SERVICE_PORT="$port"
    export RUST_LOG="info,${service}=debug"
    export RUST_BACKTRACE=1
    export DATABASE_URL="postgres://finalverse:finalverse_secret@localhost/finalverse"
    export REDIS_URL="redis://localhost:6379"
    export QDRANT_URL="http://localhost:6333"
    export REGISTRY_URL="http://localhost:8500"
    
    # Special handling for registry service
    if [ "$service" = "registry" ]; then
        # For now, we'll use the built-in service discovery in each service
        echo "{}" > "$CONFIG_DIR/registry.json"
        echo $$ > "$PID_DIR/${service}.pid"
        return 0
    fi
    
    # Start the service
    cd "$PROJ_HOME"
    
    if [ -f "target/release/${service}" ]; then
        nohup "target/release/${service}" \
            > "$LOG_DIR/${service}.log" \
            2>&1 &
        
        local pid=$!
        echo $pid > "$PID_DIR/${service}.pid"
        
        # Wait for health check
        log_debug "Waiting for $service to be healthy..."
        if check_service_health "$service"; then
            log_success "$service started successfully (PID: $pid)"
            return 0
        else
            log_error "$service failed health check"
            kill $pid 2>/dev/null || true
            rm -f "$PID_DIR/${service}.pid"
            return 1
        fi
    else
        log_error "Binary not found: target/release/${service}"
        return 1
    fi
}

# Stop a single service
stop_service() {
    local service=$1
    local pid_file="$PID_DIR/${service}.pid"
    
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "Stopping $service (PID: $pid)..."
            kill -TERM "$pid"
            
            # Wait for graceful shutdown
            local count=0
            while kill -0 "$pid" 2>/dev/null && [ $count -lt 10 ]; do
                sleep 1
                count=$((count + 1))
            done
            
            # Force kill if still running
            if kill -0 "$pid" 2>/dev/null; then
                log_warning "Force killing $service"
                kill -KILL "$pid"
            fi
            
            rm -f "$pid_file"
            log_success "$service stopped"
        else
            rm -f "$pid_file"
        fi
    fi
}

# Start all services in dependency order
start_all_services() {
    log_info "Starting Finalverse services..."
    
    # Start data services first
    if ! docker-compose ps | grep -q "Up"; then
        log_info "Starting data services..."
        docker-compose up -d
        log_info "Waiting for data services..."
        sleep 10
    fi
    
    # Build dependency order
    local services_ordered=()
    local started_services=()
    
    # Simple topological sort
    while [ ${#services_ordered[@]} -lt ${#SERVICE_PORTS[@]} ]; do
        for service in "${!SERVICE_PORTS[@]}"; do
            # Skip if already ordered
            if [[ " ${services_ordered[@]} " =~ " ${service} " ]]; then
                continue
            fi
            
            # Check if all deps are ordered
            local deps=${SERVICE_DEPS[$service]}
            local can_start=true
            
            if [ -n "$deps" ]; then
                for dep in $deps; do
                    if [[ ! " ${services_ordered[@]} " =~ " ${dep} " ]]; then
                        can_start=false
                        break
                    fi
                done
            fi
            
            if [ "$can_start" = true ]; then
                services_ordered+=("$service")
            fi
        done
    done
    
    # Start services in order
    local failed_services=()
    
    for service in "${services_ordered[@]}"; do
        if [ "$service" = "registry" ]; then
            continue  # Skip registry for now
        fi
        
        if [ -f "target/release/${service}" ]; then
            if start_service "$service"; then
                started_services+=("$service")
            else
                failed_services+=("$service")
                log_warning "Continuing without $service"
            fi
        fi
    done
    
    # Summary
    echo
    log_success "Started ${#started_services[@]} services:"
    for service in "${started_services[@]}"; do
        echo "  ✅ $service (port ${SERVICE_PORTS[$service]})"
    done
    
    if [ ${#failed_services[@]} -gt 0 ]; then
        echo
        log_warning "Failed to start ${#failed_services[@]} services:"
        for service in "${failed_services[@]}"; do
            echo "  ❌ $service"
        done
    fi
}

# Stop all services
stop_all_services() {
    log_info "Stopping all Finalverse services..."
    
    # Stop in reverse dependency order
    local services_ordered=()
    for service in "${!SERVICE_PORTS[@]}"; do
        services_ordered=("$service" "${services_ordered[@]}")
    done
    
    for service in "${services_ordered[@]}"; do
        stop_service "$service"
    done
    
    # Stop data services
    log_info "Stopping data services..."
    docker-compose down
}

# Show service status
show_status() {
    echo -e "${PURPLE}🎵 Finalverse Service Status${NC}"
    echo "==============================="
    
    # Data services
    echo -e "\n${CYAN}📊 Data Services:${NC}"
    if docker-compose ps 2>/dev/null | grep -q "Up"; then
        docker-compose ps --format "table {{.Service}}\t{{.State}}\t{{.Ports}}"
    else
        echo "  ❌ Docker services not running"
    fi
    
    # Game services
    echo -e "\n${CYAN}🎮 Game Services:${NC}"
    printf "%-20s %-10s %-20s %s\n" "SERVICE" "STATUS" "PORT" "HEALTH"
    printf "%-20s %-10s %-20s %s\n" "-------" "------" "----" "------"
    
    for service in "${!SERVICE_PORTS[@]}"; do
        local port=${SERVICE_PORTS[$service]}
        local status="❌ Stopped"
        local health="N/A"
        
        if is_service_running "$service"; then
            status="✅ Running"
            
            # Check health
            if curl -s -f "http://localhost:${port}/health" >/dev/null 2>&1; then
                health="✅ Healthy"
            else
                health="⚠️  Unhealthy"
            fi
        fi
        
        printf "%-20s %-10s %-20s %s\n" "$service" "$status" "localhost:$port" "$health"
    done
    
    # Show recent errors
    echo -e "\n${CYAN}📜 Recent Errors:${NC}"
    local error_count=0
    for log_file in "$LOG_DIR"/*.log; do
        if [ -f "$log_file" ]; then
            local service=$(basename "$log_file" .log)
            local errors=$(tail -n 100 "$log_file" 2>/dev/null | grep -i "error" | tail -n 1)
            if [ -n "$errors" ]; then
                echo "  $service: $errors"
                error_count=$((error_count + 1))
            fi
        fi
    done
    
    if [ $error_count -eq 0 ]; then
        echo "  ✅ No recent errors"
    fi
}

# Main command handling
case "${1:-help}" in
    "start")
        start_all_services
        ;;
    "stop")
        stop_all_services
        ;;
    "restart")
        stop_all_services
        sleep 2
        start_all_services
        ;;
    "status")
        show_status
        ;;
    "logs")
        if [ -n "$2" ]; then
            tail -f "$LOG_DIR/${2}.log"
        else
            tail -f "$LOG_DIR"/*.log
        fi
        ;;
    "start-service")
        if [ -n "$2" ]; then
            start_service "$2"
        else
            log_error "Please specify a service name"
        fi
        ;;
    "stop-service")
        if [ -n "$2" ]; then
            stop_service "$2"
        else
            log_error "Please specify a service name"
        fi
        ;;
    "help"|*)
        cat << EOF
${PURPLE}🎵 Finalverse Service Orchestrator${NC}

Commands:
  start              - Start all services in dependency order
  stop               - Stop all services
  restart            - Restart all services
  status             - Show service status and health
  logs [service]     - Tail logs (all or specific service)
  start-service <n>  - Start a specific service
  stop-service <n>   - Stop a specific service
  help               - Show this help

Services:
$(for service in "${!SERVICE_PORTS[@]}"; do
    echo "  $service (port ${SERVICE_PORTS[$service]})"
done | sort)

Examples:
  $0 start
  $0 status
  $0 logs song-engine
  $0 start-service world-engine
EOF
        ;;
esac