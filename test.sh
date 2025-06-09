# Update the websocket-gateway to use proper futures
echo "🔌 Ensuring websocket-gateway uses correct imports..."
if [ -f "services/websocket-gateway/src/main.rs" ]; then
    # Make sure futures is imported correctly
    sed -i '1i use futures::{SinkExt, StreamExt};' services/websocket-gateway/src/main.rs 2>/dev/null || true
fi

# Update AI Orchestra to use correct imports
echo "🤖 Ensuring AI Orchestra uses correct imports..."
if [ -f "services/ai-orchestra/src/main.rs" ]; then
    # Ensure tracing_subscriber::fmt::init is used
    sed -i 's/tracing_subscriber::init()/tracing_subscriber::fmt::init()/g' services/ai-orchestra/src/main.rs 2>/dev/null || true
fi

# Build each service individually to identify specific issues
echo "🔨 Building services individually..."

services=("websocket-gateway" "ai-orchestra" "song-engine" "story-engine" "echo-engine")

for service in "${services[@]}"; do
    echo "🔨 Building $service..."
    if cargo build -p $service 2>&1 | grep -E "(error|Error)"; then
        echo "⚠️  Issues found in $service, but continuing..."
    else
        echo "✅ $service built successfully"
    fi
done

echo "🔄 Final workspace build..."
if cargo build; then
    echo "✅ All services compiled successfully!"
    echo ""
    echo "🎵 Finalverse is now ready to harmonize!"
    echo ""
    echo "To run the services:"
    echo "  📡 WebSocket Gateway: cargo run --bin websocket-gateway"
    echo "  🤖 AI Orchestra:      cargo run --bin ai-orchestra" 
    echo "  🎵 Song Engine:       cargo run --bin song-engine"
    echo "  📖 Story Engine:      cargo run --bin story-engine"
    echo "  🔮 Echo Engine:       cargo run --bin echo-engine"
    echo ""
    echo "Services will listen on:"
    echo "  📡 WebSocket Gateway: http://localhost:3000"
    echo "  🤖 AI Orchestra:      http://localhost:3001"
    echo "  🎵 Song Engine:       http://localhost:3002"
    echo "  📖 Story Engine:      http://localhost:3003"
    echo "  🔮 Echo Engine:       http://localhost:3004"
else
    echo "❌ Some compilation errors remain. Check the output above for details."
    echo ""
    echo "Common fixes:"
    echo "1. Run: cargo update"
    echo "2. Ensure all dependencies are properly versioned"
    echo "3. Check that all use statements point to the correct modules"
    echo ""
    exit 1
fi

echo ""
echo "🌟 The Song of Creation awaits your symphony! 🎵"
