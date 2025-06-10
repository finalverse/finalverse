#!/bin/bash

# health_check.sh - Comprehensive health check for Finalverse services

echo "🏥 Finalverse Health Check"
echo "========================="

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if service is responding
check_service() {
    local name=$1
    local url=$2
    local expected=$3
    
    printf "Checking %-20s " "$name..."
    
    response=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout 2 "$url" 2>/dev/null)
    
    if [ "$response" = "$expected" ]; then
        echo -e "${GREEN}✅ OK${NC} (HTTP $response)"
        return 0
    else
        echo -e "${RED}❌ FAIL${NC} (HTTP $response)"
        return 1
    fi
}

# Check data services
echo -e "\n📊 Data Services:"
check_service "PostgreSQL" "http://localhost:5432" "000" || pg_isready -h localhost -p 5432 >/dev/null 2>&1 && echo -e "  ${GREEN}✅ PostgreSQL is ready${NC}"
check_service "Redis" "http://localhost:6379" "000" || redis-cli -h localhost ping >/dev/null 2>&1 && echo -e "  ${GREEN}✅ Redis is ready${NC}"
check_service "Qdrant" "http://localhost:6333/health" "200"
check_service "MinIO" "http://localhost:9000/minio/health/live" "200"

# Check game services
echo -e "\n🎮 Game Services:"
check_service "API Gateway" "http://localhost:8080/health" "200"
check_service "WebSocket Gateway" "ws://localhost:3000/health" "101"
check_service "Song Engine" "http://localhost:3002/health" "200"
check_service "World Engine" "http://localhost:3005/health" "200"
check_service "Story Engine" "http://localhost:3003/health" "200"
check_service "Echo Engine" "http://localhost:3004/health" "200"
check_service "AI Orchestra" "http://localhost:3001/health" "200"

# Check service logs for errors
echo -e "\n📜 Recent Errors:"
for service in websocket-gateway song-engine world-engine story-engine echo-engine ai-orchestra; do
    if [ -f "logs/${service}.log" ]; then
        errors=$(tail -n 100 "logs/${service}.log" | grep -i "error" | tail -n 3)
        if [ -n "$errors" ]; then
            echo -e "${YELLOW}⚠️  ${service}:${NC}"
            echo "$errors" | sed 's/^/    /'
        fi
    fi
done

# Check tmux sessions
echo -e "\n📺 Service Sessions:"
if tmux has-session -t finalverse 2>/dev/null; then
    echo -e "${GREEN}✅ Tmux session 'finalverse' is active${NC}"
    tmux list-windows -t finalverse | sed 's/^/    /'
else
    echo -e "${RED}❌ No tmux session found${NC}"
fi

# Summary
echo -e "\n📊 Summary:"
echo "  Use './scripts/finalverse.sh logs <service>' to view detailed logs"
echo "  Use './scripts/finalverse.sh restart' to restart all services"