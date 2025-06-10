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
