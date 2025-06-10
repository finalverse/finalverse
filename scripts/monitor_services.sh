#!/bin/bash
# monitor_services.sh - Real-time monitoring for Finalverse services

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Service definitions
declare -A SERVICES=(
    ["websocket-gateway"]="3000"
    ["ai-orchestra"]="3001"
    ["song-engine"]="3002"
    ["story-engine"]="3003"
    ["echo-engine"]="3004"
)

declare -A DATA_SERVICES=(
    ["postgres"]="5432"
    ["redis"]="6379"
    ["qdrant"]="6333"
    ["minio"]="9000"
)

# Function to check if a port is open
check_port() {
    local host=$1
    local port=$2
    local timeout=${3:-5}
    
    timeout $timeout bash -c "</dev/tcp/$host/$port" >/dev/null 2>&1
    return $?
}

# Function to check HTTP health endpoint
check_http_health() {
    local url=$1
    local timeout=${2:-5}
    
    response=$(timeout $timeout curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null || echo "000")
    [ "$response" = "200" ]
}

# Function to check data service health
check_data_service() {
    local service=$1
    
    case $service in
        "postgres")
            docker-compose exec -T postgres pg_isready -U finalverse >/dev/null 2>&1
            ;;
        "redis")
            docker-compose exec -T redis redis-cli ping >/dev/null 2>&1
            ;;
        "qdrant")
            check_http_health "http://localhost:6333/health"
            ;;
        "minio")
            check_http_health "http://localhost:9000/minio/health/live"
            ;;
        *)
            return 1
            ;;
    esac
}

# Function to get service status
get_service_status() {
    local service=$1
    local port=$2
    
    if check_port "localhost" "$port" 2; then
        if check_http_health "http://localhost:$port/health" 2; then
            echo "HEALTHY"
        else
            echo "RUNNING"
        fi
    else
        echo "STOPPED"
    fi
}

# Function to get data service status
get_data_service_status() {
    local service=$1
    
    if ! docker-compose ps $service | grep -q "Up"; then
        echo "STOPPED"
    elif check_data_service "$service"; then
        echo "HEALTHY"
    else
        echo "UNHEALTHY"
    fi
}

# Function to print status with color
print_status() {
    local name=$1
    local status=$2
    local port=$3
    
    case $status in
        "HEALTHY")
            printf "   %-20s ${GREEN}●${NC} %-10s ${CYAN}:%s${NC}\n" "$name" "$status" "$port"
            ;;
        "RUNNING")
            printf "   %-20s ${YELLOW}●${NC} %-10s ${CYAN}:%s${NC}\n" "$name" "$status" "$port"
            ;;
        "UNHEALTHY")
            printf "   %-20s ${YELLOW}●${NC} %-10s ${CYAN}:%s${NC}\n" "$name" "$status" "$port"
            ;;
        "STOPPED")
            printf "   %-20s ${RED}●${NC} %-10s ${CYAN}:%s${NC}\n" "$name" "$status" "$port"
            ;;
    esac
}

# Function to display header
display_header() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    clear
    echo -e "${PURPLE}🎵 Finalverse Service Monitor${NC}"
    echo -e "${CYAN}=================================================${NC}"
    echo -e "Last updated: ${timestamp}"
    echo ""
}

# Function to display summary stats
display_summary() {
    local total_services=$((${#SERVICES[@]} + ${#DATA_SERVICES[@]}))
    local healthy_count=0
    local running_count=0
    local stopped_count=0
    
    # Count service statuses
    for service in "${!SERVICES[@]}"; do
        local status=$(get_service_status "$service" "${SERVICES[$service]}")
        case $status in
            "HEALTHY") ((healthy_count++)) ;;
            "RUNNING") ((running_count++)) ;;
            "STOPPED") ((stopped_count++)) ;;
        esac
    done
    
    for service in "${!DATA_SERVICES[@]}"; do
        local status=$(get_data_service_status "$service")
        case $status in
            "HEALTHY") ((healthy_count++)) ;;
            "RUNNING"|"UNHEALTHY") ((running_count++)) ;;
            "STOPPED") ((stopped_count++)) ;;
        esac
    done
    
    echo -e "${BLUE}📊 Service Summary${NC}"
    echo -e "   Total Services: $total_services"
    echo -e "   ${GREEN}Healthy: $healthy_count${NC}"
    echo -e "   ${YELLOW}Running: $running_count${NC}"
    echo -e "   ${RED}Stopped: $stopped_count${NC}"
    echo ""
}

# Function to display service details
display_services() {
    echo -e "${BLUE}🎮 Game Services${NC}"
    for service in "${!SERVICES[@]}"; do
        local port="${SERVICES[$service]}"
        local status=$(get_service_status "$service" "$port")
        print_status "$service" "$status" "$port"
    done
    echo ""
    
    echo -e "${BLUE}🗄️ Data Services${NC}"
    for service in "${!DATA_SERVICES[@]}"; do
        local port="${DATA_SERVICES[$service]}"
        local status=$(get_data_service_status "$service")
        print_status "$service" "$status" "$port"
    done
    echo ""
}

# Function to display recent logs
display_recent_logs() {
    echo -e "${BLUE}📜 Recent Activity${NC}"
    
    # Check for tmux session
    if tmux has-session -t finalverse 2>/dev/null; then
        echo "   Services running in tmux session 'finalverse'"
        echo "   Use 'tmux attach -t finalverse' to view detailed logs"
    else
        # Check for log files
        if [ -d "logs" ] && [ "$(ls -A logs/ 2>/dev/null)" ]; then
            echo "   Recent log entries:"
            for logfile in logs/*.log; do
                if [ -f "$logfile" ]; then
                    local service=$(basename "$logfile" .log)
                    local last_line=$(tail -n 1 "$logfile" 2>/dev/null | head -c 80)
                    if [ -n "$last_line" ]; then
                        echo "   $service: $last_line..."
                    fi
                fi
            done
        else
            echo "   No log files found"
        fi
    fi
    echo ""
}

# Function to display quick actions
display_actions() {
    echo -e "${BLUE}🚀 Quick Actions${NC}"
    echo "   ./start_services.sh  - Start all services"
    echo "   ./stop_services.sh   - Stop all services"
    echo "   ./test_services.sh   - Test service connectivity"
    echo ""
    echo "   tmux attach -t finalverse  - View service logs"
    echo "   docker-compose logs -f     - View data service logs"
    echo ""
    echo "   Press Ctrl+C to exit monitor"
    echo ""
}

# Function to check if docker-compose is running
check_docker_compose() {
    if ! docker-compose ps >/dev/null 2>&1; then
        echo -e "${RED}❌ Docker Compose not found or not running${NC}"
        echo "   Run 'docker-compose up -d' to start data services"
        return 1
    fi
    return 0
}

# Function to suggest actions based on status
suggest_actions() {
    local all_healthy=true
    
    # Check if any services are down
    for service in "${!SERVICES[@]}"; do
        local status=$(get_service_status "$service" "${SERVICES[$service]}")
        if [ "$status" != "HEALTHY" ]; then
            all_healthy=false
            break
        fi
    done
    
    if ! $all_healthy; then
        echo -e "${YELLOW}💡 Suggestions${NC}"
        echo "   Some services appear to be down."
        echo "   Try: ./start_services.sh"
        echo ""
    fi
}

# Main monitoring function
monitor_services() {
    local watch_mode=${1:-false}
    
    while true; do
        display_header
        
        # Check docker-compose availability
        if check_docker_compose; then
            display_summary
            display_services
            display_recent_logs
            suggest_actions
        fi
        
        display_actions
        
        if [ "$watch_mode" = "false" ]; then
            break
        fi
        
        sleep 5
    done
}

# Function to run interactive mode
interactive_mode() {
    echo -e "${PURPLE}🎵 Finalverse Interactive Monitor${NC}"
    echo "=================================="
    echo ""
    echo "Choose an option:"
    echo "1) Monitor services (one-time check)"
    echo "2) Watch services (continuous monitoring)"
    echo "3) Start all services"
    echo "4) Stop all services"
    echo "5) Test service connectivity"
    echo "6) View service logs"
    echo "7) Exit"
    echo ""
    read -p "Enter your choice (1-7): " choice
    
    case $choice in
        1)
            monitor_services false
            ;;
        2)
            echo "Starting continuous monitoring (Press Ctrl+C to stop)..."
            monitor_services true
            ;;
        3)
            echo "Starting services..."
            ./start_services.sh
            ;;
        4)
            echo "Stopping services..."
            ./stop_services.sh
            ;;
        5)
            echo "Testing services..."
            ./test_services.sh
            ;;
        6)
            if tmux has-session -t finalverse 2>/dev/null; then
                tmux attach -t finalverse
            else
                echo "No tmux session found. Showing recent logs:"
                if [ -d "logs" ]; then
                    tail -f logs/*.log
                else
                    echo "No log files found."
                fi
            fi
            ;;
        7)
            echo "Goodbye!"
            exit 0
            ;;
        *)
            echo "Invalid choice. Please try again."
            interactive_mode
            ;;
    esac
}

# Main script logic
case "${1:-interactive}" in
    "watch")
        monitor_services true
        ;;
    "check")
        monitor_services false
        ;;
    "interactive")
        interactive_mode
        ;;
    *)
        echo "Usage: $0 [watch|check|interactive]"
        echo ""
        echo "  watch       - Continuous monitoring (refreshes every 5s)"
        echo "  check       - One-time status check"
        echo "  interactive - Interactive menu (default)"
        exit 1
        ;;
esac