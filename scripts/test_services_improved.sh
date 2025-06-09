#!/bin/bash
# scripts/test_services_improved.sh

echo "🧪 Testing Finalverse Services (Improved)"
echo "========================================"

# Function to test a service
test_service() {
    local name=$1
    local url=$2
    local emoji=$3
    
    echo -n "$emoji $name: "
    
    # First try /info endpoint
    response=$(curl -s -w "\n%{http_code}" "$url/info" 2>/dev/null)
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [ "$http_code" = "200" ]; then
        echo "✅ Online"
        echo "$body" | jq '.' 2>/dev/null || echo "$body"
    else
        # Try /health endpoint as fallback
        health_response=$(curl -s -w "%{http_code}" "$url/health" 2>/dev/null)
        if [ "$health_response" = "OK200" ]; then
            echo "✅ Online (health check passed)"
        else
            echo "❌ Offline or unreachable"
        fi
    fi
    echo ""
}

# Test core services
echo "📡 Testing Core Services:"
echo "------------------------"
test_service "Song Engine" "http://localhost:3001" "🎵"
test_service "World Engine" "http://localhost:3002" "🌍"
test_service "Echo Engine" "http://localhost:3003" "✨"
test_service "AI Orchestra" "http://localhost:3004" "🤖"
test_service "Story Engine" "http://localhost:3005" "📜"
test_service "Harmony Service" "http://localhost:3006" "🎼"

# Test API Gateway
echo -e "\n🌐 Testing API Gateway:"
echo "----------------------"
test_service "API Gateway (Song)" "http://localhost:8080/api/song" "🎵"

# Test specific functionality
echo -e "\n🔬 Testing Specific Endpoints:"
echo "-----------------------------"

# Test world regions
echo -n "🗺️  World Regions: "
regions=$(curl -s http://localhost:3002/regions 2>/dev/null)
if [ $? -eq 0 ]; then
    region_count=$(echo "$regions" | jq '.regions | length' 2>/dev/null || echo "0")
    echo "✅ Found $region_count regions"
else
    echo "❌ Failed to fetch regions"
fi

# Test echo list
echo -n "👥 Echo Characters: "
echoes=$(curl -s http://localhost:3003/echoes 2>/dev/null)
if [ $? -eq 0 ]; then
    echo_count=$(echo "$echoes" | jq '.echoes | length' 2>/dev/null || echo "0")
    echo "✅ Found $echo_count echoes"
else
    echo "❌ Failed to fetch echoes"
fi

# Test AI models
echo -n "🧠 AI Models: "
models=$(curl -s http://localhost:3004/models 2>/dev/null)
if [ $? -eq 0 ]; then
    model_count=$(echo "$models" | jq '.models | length' 2>/dev/null || echo "0")
    echo "✅ Found $model_count AI models"
else
    echo "❌ Failed to fetch AI models"
fi

# Test data services
echo -e "\n💾 Testing Data Services:"
echo "------------------------"

# PostgreSQL
echo -n "🐘 PostgreSQL: "
if PGPASSWORD=finalverse_secret psql -h localhost -U finalverse -d finalverse -c "SELECT 1" &>/dev/null; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

# Redis
echo -n "🔴 Redis: "
if redis-cli -h localhost ping &>/dev/null; then
    echo "✅ Connected"
else
    echo "❌ Connection failed"
fi

# Summary
echo -e "\n📊 Summary:"
echo "----------"
online_count=0
total_count=6

for port in 3001 3002 3003 3004 3005 3006; do
    if curl -s -f "http://localhost:$port/health" &>/dev/null || curl -s -f "http://localhost:$port/info" &>/dev/null; then
        ((online_count++))
    fi
done

echo "Services Online: $online_count / $total_count"

if [ $online_count -eq $total_count ]; then
    echo "✅ All services are running!"
    echo ""
    echo "🎮 Ready to play! Run: cargo run --bin mock-client"
else
    echo "⚠️  Some services are not running properly."
    echo "   Run: docker-compose logs [service-name] to check logs"
fi