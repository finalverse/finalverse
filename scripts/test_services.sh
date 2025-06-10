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
