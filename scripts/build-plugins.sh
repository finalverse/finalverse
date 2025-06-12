#!/bin/bash
# scripts/build-plugins.sh

# Build and deploy plugins for Finalverse

set -e

echo "🔨 Building Finalverse plugins..."

# Create plugin output directory
PLUGIN_DIR="target/release/plugins"
mkdir -p "$PLUGIN_DIR"

# Build greeter plugin
echo "📦 Building greeter-plugin..."
cd plugins/greeter-plugin
cargo build --release

# Copy the built plugin to the plugin directory
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    cp ../../target/release/libgreeter_plugin.so "$PLUGIN_DIR/greeter_plugin.so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    cp ../../target/release/libgreeter_plugin.dylib "$PLUGIN_DIR/greeter_plugin.dylib"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    cp ../../target/release/greeter_plugin.dll "$PLUGIN_DIR/greeter_plugin.dll"
fi

cd ../..

echo "✅ Plugins built successfully!"
echo "📁 Plugins located in: $PLUGIN_DIR"

# Test the plugin with curl commands
echo ""
echo "🧪 Testing plugin API (make sure server is running)..."
echo ""

# List plugins
echo "📋 Listing loaded plugins:"
curl -s http://localhost:8091/plugins | jq .

echo ""
echo "🎉 Testing greet command:"
curl -s -X POST http://localhost:8091/plugins/greeter/greet \
  -H "Content-Type: application/json" \
  -d '{"name": "Finalverse Developer", "language": "en", "style": "epic"}' | jq .

echo ""
echo "👋 Testing farewell command:"
curl -s -X POST http://localhost:8091/plugins/greeter/farewell \
  -H "Content-Type: application/json" \
  -d '{"name": "Friend", "style": "pirate"}' | jq .

echo ""
echo "📊 Testing stats command:"
curl -s -X POST http://localhost:8091/plugins/greeter/stats \
  -H "Content-Type: application/json" \
  -d '{}' | jq .

echo ""
echo "📜 Testing history command:"
curl -s -X POST http://localhost:8091/plugins/greeter/history \
  -H "Content-Type: application/json" \
  -d '{"limit": 5}' | jq .