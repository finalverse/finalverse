# scripts/cleanup.sh - Complete cleanup script
#!/bin/bash
set -e

echo "🧹 Finalverse Complete Cleanup"
echo "==============================="

# Auto-detect project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Warning
echo "⚠️  WARNING: This will stop all services and optionally clean data"
read -p "Continue? (y/N): " -r
[[ ! $REPLY =~ ^[Yy]$ ]] && { echo "Cancelled."; exit 1; }

# Backup option
read -p "Create backup before cleanup? (Y/n): " -r
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    "$SCRIPT_DIR/finalverse.sh" backup
fi

# Use the main CLI for cleanup
"$SCRIPT_DIR/finalverse.sh" clean

echo ""
echo "🎉 Cleanup complete!"