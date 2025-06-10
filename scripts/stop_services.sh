#!/bin/bash
echo "🛑 Stopping Finalverse Services..."

# Stop tmux session if it exists
if tmux has-session -t finalverse 2>/dev/null; then
    echo "🖥️ Stopping tmux session..."
    tmux kill-session -t finalverse
    echo "✅ Tmux session stopped"
fi

# Stop background processes if they exist
if [ -d "logs" ]; then
    echo "🔄 Stopping background processes..."
    for pidfile in logs/*.pid; do
        if [ -f "$pidfile" ]; then
            pid=$(cat "$pidfile")
            service=$(basename "$pidfile" .pid)
            if kill "$pid" 2>/dev/null; then
                echo "✅ Stopped $service (PID $pid)"
            else
                echo "⚠️ Process $service (PID $pid) was not running"
            fi
            rm -f "$pidfile"
        fi
    done
fi

# Stop Docker services
echo "🐳 Stopping data services..."
docker-compose down

echo ""
echo "✅ All Finalverse services stopped"
echo "   Data is preserved in ./data/ directory"
echo "   Logs are preserved in ./logs/ directory"
