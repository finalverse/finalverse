# scripts/setup_mvp.sh - Simplified setup script
#!/bin/bash
set -e

echo "🎵 Setting up Finalverse MVP..."
echo "================================"

# Auto-detect project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo required. Visit https://rustup.rs/"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker required."; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose required."; exit 1; }

# Use the main CLI for setup
"$SCRIPT_DIR/finalverse.sh" setup
"$SCRIPT_DIR/finalverse.sh" build

echo ""
echo "🎉 Finalverse MVP setup complete!"
echo ""
echo "📋 Next steps:"
echo "  1. Start services: ./scripts/finalverse.sh start"
echo "  2. Test services: ./scripts/finalverse.sh test"
echo "  3. Monitor logs: ./scripts/finalverse.sh monitor"