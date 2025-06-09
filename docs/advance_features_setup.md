# Finalverse Advanced Features Setup Guide

## 🚀 Overview of New Features

We've implemented several advanced features to make Finalverse more dynamic and engaging:

1. **WebSocket Support** - Real-time updates for world events
2. **LLM Integration** - Dynamic content generation using AI
3. **Dynamic Quest System** - Procedurally generated quests based on player actions
4. **Enhanced AI Orchestra** - Real AI-powered NPC dialogue and world descriptions

## 📦 Installation & Setup

### 1. Update Workspace Configuration

Add the new services to your root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... existing services ...
    "services/websocket-gateway",
]
```

### 2. Create WebSocket Gateway Service

```bash
# Create service directory
mkdir -p services/websocket-gateway/src

# Add the WebSocket service code (from artifact: websocket-service)
# Add Cargo.toml for websocket-gateway
```

### 3. Install LLM Provider (Choose One)

#### Option A: Ollama (Local, Free)
```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Pull a model
ollama pull llama2

# Start Ollama server (runs on port 11434)
ollama serve
```

#### Option B: OpenAI API
```bash
# Set your API key
export OPENAI_API_KEY="your-api-key-here"
```

### 4. Update Docker Compose

Add the WebSocket gateway to `docker-compose.yml`:

```yaml
websocket-gateway:
  build:
    context: .
    dockerfile: docker/Dockerfile.service
    args:
      SERVICE: websocket-gateway
  ports:
    - "3007:3007"
  environment:
    RUST_LOG: info
  depends_on:
    - redis
```

### 5. Update Environment Variables

Create a `.env` file:

```bash
# LLM Configuration
ENABLE_OLLAMA=true
OLLAMA_BASE_URL=http://localhost:11434

# Optional: OpenAI Configuration
# OPENAI_API_KEY=your-key-here

# WebSocket Configuration
WS_PORT=3007
```

## 🧪 Testing the Advanced Features

### 1. Test WebSocket Connection

Open `websocket-client-example.html` in a browser:

```bash
# Save the HTML file locally
# Open in browser
# Click "Connect" to establish WebSocket connection
```

### 2. Test LLM Integration

```bash
# Generate NPC dialogue with AI
curl -X POST http://localhost:3004/npc/dialogue \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "npc_name": "Mystic Sage",
      "emotion": "mysterious",
      "player_name": "TestPlayer",
      "location": "Ancient Temple"
    }
  }'

# Generate a dynamic quest
curl -X POST http://localhost:3004/quest/generate \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "region": "Whispering Woods",
      "player_level": 15
    },
    "parameters": {
      "difficulty": "medium",
      "quest_type": "exploration"
    }
  }'
```

### 3. Test Real-Time Updates

With the WebSocket client connected:

1. Perform actions in another client
2. Watch real-time updates appear
3. Test harmony changes, player movements, etc.

## 🔧 Configuration Options

### AI Orchestra Configuration

```rust
// In services/ai-orchestra/src/main.rs
// Customize LLM providers:

// For Ollama with different model:
Box::new(OllamaProvider::new("mistral"))

// For Claude API:
Box::new(OpenAIProvider::new(
    "https://api.anthropic.com",
    &api_key,
    "claude-3-sonnet-20240229"
))
```

### WebSocket Event Types

Customize events in `websocket-gateway/src/main.rs`:

```rust
enum WSMessage {
    // Add custom event types
    CustomEvent { data: serde_json::Value },
    QuestUpdate { quest_id: String, status: String },
    // etc.
}
```

## 🚦 Running Everything Together

### Complete Startup Sequence

```bash
# 1. Start data services
docker-compose up -d postgres redis qdrant minio

# 2. Start Ollama (if using local LLM)
ollama serve &

# 3. Build all services
cargo build --workspace --release

# 4. Start all game services
docker-compose up -d

# 5. Verify all services
./scripts/test_services_improved.sh

# 6. Start WebSocket gateway
cargo run --bin websocket-gateway &

# 7. Run enhanced client
cargo run --bin mock-client
```

## 📊 Monitoring & Debugging

### Check Service Logs

```bash
# Docker services
docker-compose logs -f ai-orchestra
docker-compose logs -f websocket-gateway

# Local services
RUST_LOG=debug cargo run --bin websocket-gateway
```

### Performance Monitoring

```bash
# Monitor LLM response times
curl http://localhost:3004/metrics

# Check WebSocket connections
curl http://localhost:3007/info
```

## 🎮 Using Advanced Features in Game

### 1. Dynamic Quests
- Quests now adapt to player actions
- AI generates unique quest descriptions
- Objectives scale with player level

### 2. Real-Time World
- See other players' actions instantly
- World events broadcast to all connected clients
- Harmony changes reflect immediately

### 3. Smart NPCs
- NPCs generate contextual dialogue
- Responses based on world state
- Emotional reactions to player actions

## 🐛 Troubleshooting

### LLM Not Working
```bash
# Check Ollama is running
curl http://localhost:11434/api/tags

# Test direct generation
curl http://localhost:11434/api/generate -d '{
  "model": "llama2",
  "prompt": "Hello world"
}'
```

### WebSocket Connection Failed
- Check port 3007 is not in use
- Ensure CORS is enabled
- Check browser console for errors

### Slow AI Responses
- Consider using smaller models
- Implement response caching
- Use GPU acceleration for Ollama

## 🚀 Next Steps

1. **Implement Player Persistence**
   - Save/load player state
   - Track quest progress
   - Store generated content

2. **Add More AI Features**
   - Procedural dungeon generation
   - Dynamic item creation
   - Personality evolution for NPCs

3. **Enhance Real-Time Features**
   - Voice chat integration
   - Live music collaboration
   - Synchronized world events

4. **Scale for Production**
   - Add load balancing
   - Implement caching layers
   - Set up monitoring

The foundation is now in place for a truly dynamic, AI-driven world where every player's journey is unique!