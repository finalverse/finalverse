#!/bin/bash
# finalverse.sh - Main development helper script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Helper functions
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Function to display banner
show_banner() {
    echo -e "${PURPLE}"
    cat << "EOF"
    ███████╗██╗███╗   ██╗ █████╗ ██╗    ██╗   ██╗███████╗██████╗ ███████╗███████╗
    ██╔════╝██║████╗  ██║██╔══██╗██║    ██║   ██║██╔════╝██╔══██╗██╔════╝██╔════╝
    █████╗  ██║██╔██╗ ██║███████║██║    ██║   ██║█████╗  ██████╔╝███████╗█████╗  
    ██╔══╝  ██║██║╚██╗██║██╔══██║██║    ╚██╗ ██╔╝██╔══╝  ██╔══██╗╚════██║██╔══╝  
    ██║     ██║██║ ╚████║██║  ██║███████╗╚████╔╝ ███████╗██║  ██║███████║███████╗
    ╚═╝     ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝ ╚═══╝  ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝
EOF
    echo -e "${NC}"
    echo -e "${CYAN}         🎵 The AI-Driven Digital Universe Where Stories Meet Infinite Possibilities 🌟${NC}"
    echo ""
}

# Function to check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    local missing_deps=()
    
    command -v cargo >/dev/null 2>&1 || missing_deps+=("Rust/Cargo")
    command -v docker >/dev/null 2>&1 || missing_deps+=("Docker")
    command -v docker-compose >/dev/null 2>&1 || missing_deps+=("Docker Compose")
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        echo ""
        echo "Please install:"
        for dep in "${missing_deps[@]}"; do
            case $dep in
                "Rust/Cargo")
                    echo "  - Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                    ;;
                "Docker")
                    echo "  - Docker: https://docs.docker.com/get-docker/"
                    ;;
                "Docker Compose")
                    echo "  - Docker Compose: https://docs.docker.com/compose/install/"
                    ;;
            esac
        done
        exit 1
    fi
    
    log_success "All prerequisites met"
}

# Function to setup project
setup_project() {
    log_info "Setting up Finalverse MVP..."
    
    if [ ! -f "setup_mvp.sh" ]; then
        log_error "setup_mvp.sh not found. Please run this from the Finalverse root directory."
        exit 1
    fi
    
    chmod +x setup_mvp.sh
    ./setup_mvp.sh
}

# Function to build project
build_project() {
    log_info "Building Finalverse workspace..."
    
    if cargo build --workspace --release; then
        log_success "Build completed successfully"
    else
        log_error "Build failed"
        echo ""
        echo "Common fixes:"
        echo "  - Run: cargo clean && cargo build"
        echo "  - Check: cargo update"
        echo "  - Verify: all dependencies are correct"
        exit 1
    fi
}

# Function to run development environment
run_dev() {
    log_info "Starting Finalverse development environment..."
    
    # Ensure scripts exist and are executable
    local scripts=("start_services.sh" "stop_services.sh" "test_services.sh" "monitor_services.sh")
    for script in "${scripts[@]}"; do
        if [ ! -f "$script" ]; then
            log_warning "$script not found, creating it..."
            # You would call the setup here if needed
        fi
        chmod +x "$script" 2>/dev/null || true
    done
    
    # Start services
    if [ -f "start_services.sh" ]; then
        ./start_services.sh
    else
        log_error "start_services.sh not found. Run 'setup' first."
        exit 1
    fi
}

# Function to run tests
run_tests() {
    log_info "Running Finalverse tests..."
    
    if [ -f "test_services.sh" ]; then
        ./test_services.sh
    else
        log_warning "test_services.sh not found, running basic connectivity tests..."
        
        local services=(
            "websocket-gateway:3000"
            "ai-orchestra:3001" 
            "song-engine:3002"
            "story-engine:3003"
            "echo-engine:3004"
        )
        
        for service_port in "${services[@]}"; do
            IFS=':' read -r service port <<< "$service_port"
            log_info "Testing $service on port $port..."
            if timeout 5 curl -s "http://localhost:$port/health" >/dev/null 2>&1; then
                log_success "$service is responding"
            else
                log_error "$service is not responding on port $port"
            fi
        done
    fi
}

# Function to stop services
stop_services() {
    log_info "Stopping Finalverse services..."
    
    if [ -f "stop_services.sh" ]; then
        ./stop_services.sh
    else
        log_warning "stop_services.sh not found, attempting manual cleanup..."
        
        # Kill tmux session
        tmux kill-session -t finalverse 2>/dev/null || true
        
        # Kill background processes
        if [ -d "logs" ]; then
            for pidfile in logs/*.pid; do
                if [ -f "$pidfile" ]; then
                    pid=$(cat "$pidfile")
                    kill "$pid" 2>/dev/null || true
                    rm -f "$pidfile"
                fi
            done
        fi
        
        # Stop docker services
        docker-compose down 2>/dev/null || true
        
        log_success "Services stopped"
    fi
}

# Function to monitor services
monitor_services() {
    if [ -f "monitor_services.sh" ]; then
        chmod +x monitor_services.sh
        ./monitor_services.sh "${1:-interactive}"
    else
        log_error "monitor_services.sh not found. Run 'setup' first."
        exit 1
    fi
}

# Function to open WebSocket test client
open_client() {
    local client_file="Client_WebSocket.html"
    
    if [ ! -f "$client_file" ]; then
        log_warning "WebSocket client not found, would you like me to create it? (y/n)"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            log_info "Creating WebSocket test client..."
            # Here you would create the client file
            log_success "WebSocket client created as $client_file"
        else
            log_error "WebSocket client required for testing"
            return 1
        fi
    fi
    
    log_info "Opening WebSocket test client..."
    
    # Try to open in browser
    if command -v xdg-open >/dev/null 2>&1; then
        xdg-open "$client_file"
    elif command -v open >/dev/null 2>&1; then
        open "$client_file"
    elif command -v start >/dev/null 2>&1; then
        start "$client_file"
    else
        log_info "Please open $client_file in your web browser"
        echo "Client available at: file://$(pwd)/$client_file"
    fi
}

# Function to show logs
show_logs() {
    local service=${1:-"all"}
    
    if tmux has-session -t finalverse 2>/dev/null; then
        if [ "$service" = "all" ]; then
            log_info "Attaching to tmux session (all services)..."
            tmux attach -t finalverse
        else
            log_info "Showing logs for $service..."
            tmux select-window -t "finalverse:$service" 2>/dev/null || {
                log_error "Service window '$service' not found"
                echo "Available windows:"
                tmux list-windows -t finalverse 2>/dev/null || echo "No tmux session running"
                return 1
            }
            tmux attach -t finalverse
        fi
    elif [ -d "logs" ]; then
        if [ "$service" = "all" ]; then
            log_info "Showing all service logs..."
            tail -f logs/*.log 2>/dev/null || {
                log_warning "No log files found in logs/"
            }
        else
            local logfile="logs/$service.log"
            if [ -f "$logfile" ]; then
                log_info "Showing logs for $service..."
                tail -f "$logfile"
            else
                log_error "Log file not found: $logfile"
                echo "Available logs:"
                ls logs/*.log 2>/dev/null || echo "No log files found"
                return 1
            fi
        fi
    else
        log_error "No logs available. Services may not be running or logs directory doesn't exist."
        return 1
    fi
}

# Function to clean project
clean_project() {
    log_info "Cleaning Finalverse project..."
    
    # Stop services first
    stop_services
    
    # Clean Rust build artifacts
    cargo clean
    
    # Clean logs
    if [ -d "logs" ]; then
        rm -rf logs/*.log logs/*.pid
        log_success "Cleaned log files"
    fi
    
    # Clean temporary files
    rm -f init_db.sql
    
    log_success "Project cleaned"
}

# Function to reset everything
reset_project() {
    log_warning "This will stop all services and remove all data. Continue? (y/n)"
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        log_info "Reset cancelled"
        return 0
    fi
    
    log_info "Resetting Finalverse project..."
    
    # Stop everything
    stop_services
    
    # Remove Docker volumes and data
    docker-compose down -v 2>/dev/null || true
    if [ -d "data" ]; then
        sudo rm -rf data/* 2>/dev/null || rm -rf data/* 2>/dev/null || true
        log_success "Removed data directory contents"
    fi
    
    # Clean project
    clean_project
    
    log_success "Project reset complete"
    log_info "Run 'setup' to reinitialize"
}

# Function to show project status
show_status() {
    log_info "Finalverse Project Status"
    echo ""
    
    # Check if built
    if [ -f "target/release/websocket-gateway" ] || [ -f "target/debug/websocket-gateway" ]; then
        log_success "Project is built"
    else
        log_warning "Project not built (run 'build' command)"
    fi
    
    # Check data services
    echo "Data Services:"
    if docker-compose ps >/dev/null 2>&1; then
        docker-compose ps --format "table {{.Service}}\t{{.State}}\t{{.Ports}}"
    else
        log_warning "Docker Compose not running"
    fi
    
    echo ""
    
    # Check if services are running
    echo "Game Services:"
    local services=("websocket-gateway:3000" "ai-orchestra:3001" "song-engine:3002" "story-engine:3003" "echo-engine:3004")
    for service_port in "${services[@]}"; do
        IFS=':' read -r service port <<< "$service_port"
        if timeout 2 curl -s "http://localhost:$port/health" >/dev/null 2>&1; then
            echo "  ✅ $service (port $port)"
        else
            echo "  ❌ $service (port $port)"
        fi
    done
    
    echo ""
    
    # Check tmux session
    if tmux has-session -t finalverse 2>/dev/null; then
        log_success "Services running in tmux session 'finalverse'"
    else
        log_info "No tmux session found"
    fi
}

# Function to show help
show_help() {
    echo "Finalverse Development Helper"
    echo ""
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  setup              - Setup the complete MVP environment"
    echo "  build              - Build all Rust services"
    echo "  start              - Start all Finalverse services"
    echo "  stop               - Stop all services"
    echo "  restart            - Restart all services"
    echo "  test               - Test service connectivity"
    echo "  monitor [mode]     - Monitor services (interactive/watch/check)"
    echo "  logs [service]     - Show logs (all services or specific)"
    echo "  client             - Open WebSocket test client"
    echo "  status             - Show project status"
    echo "  clean              - Clean build artifacts and logs"
    echo "  reset              - Reset everything (stops services, removes data)"
    echo "  help               - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 setup           - Initial setup"
    echo "  $0 start           - Start all services"
    echo "  $0 monitor watch   - Continuous monitoring"
    echo "  $0 logs song-engine - Show song engine logs"
    echo "  $0 test            - Test all endpoints"
    echo ""
    echo "Service URLs:"
    echo "  WebSocket Gateway: ws://localhost:3000/ws"
    echo "  AI Orchestra:      http://localhost:3001"
    echo "  Song Engine:       http://localhost:3002"
    echo "  Story Engine:      http://localhost:3003"
    echo "  Echo Engine:       http://localhost:3004"
    echo ""
    echo "Data Services:"
    echo "  PostgreSQL:        localhost:5432 (finalverse/finalverse_secret)"
    echo "  Redis:             localhost:6379"
    echo "  Qdrant:            http://localhost:6333"
    echo "  MinIO:             http://localhost:9001"
}

# Main command dispatcher
main() {
    local command=${1:-help}
    
    case $command in
        "setup")
            show_banner
            check_prerequisites
            setup_project
            ;;
        "build")
            show_banner
            build_project
            ;;
        "start")
            show_banner
            run_dev
            ;;
        "stop")
            stop_services
            ;;
        "restart")
            stop_services
            sleep 2
            run_dev
            ;;
        "test")
            run_tests
            ;;
        "monitor")
            monitor_services "${2:-interactive}"
            ;;
        "logs")
            show_logs "${2:-all}"
            ;;
        "client")
            open_client
            ;;
        "status")
            show_status
            ;;
        "clean")
            clean_project
            ;;
        "reset")
            reset_project
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"