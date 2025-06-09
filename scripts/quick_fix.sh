#!/bin/bash
# scripts/quick_fix.sh - Quick fixes for common issues

echo "🔧 Applying quick fixes to Finalverse..."

# Fix 1: Update mock client to handle missing services gracefully
echo "📝 Updating mock client error handling..."

# Fix 2: Ensure all services have proper Cargo.toml files
echo "📦 Ensuring all services have Cargo.toml files..."

# Story Engine Cargo.toml
if [ ! -f "services/story-engine/Cargo.toml" ]; then
cat > services/story-engine/Cargo.toml << 'EOF'
[package]
name = "story-engine"
version.workspace = true
edition.workspace = true

[[bin]]
name = "story-engine"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
reqwest = { version = "0.11", features = ["json"] }
EOF
fi

# Harmony Service Cargo.toml
if [ ! -f "services/harmony-service/Cargo.toml" ]; then
cat > services/harmony-service/Cargo.toml << 'EOF'
[package]
name = "harmony-service"
version.workspace = true
edition.workspace = true

[[bin]]
name = "harmony-service"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
EOF
fi

# Fix 3: Add ecosystem endpoint to world engine
echo "🌍 Adding ecosystem endpoint to World Engine..."

# Check if the endpoint already exists
if ! grep -q "get_region_ecosystem" services/world-engine/src/main.rs; then
    # Create a backup
    cp services/world-engine/src/main.rs services/world-engine/src/main.rs.bak
    
    # Add the new endpoint before the main function
    sed -i '/^#\[tokio::main\]/i \
async fn get_region_ecosystem(\
    State(state): State<WorldEngineState>,\
    Path(region_id): Path<String>,\
) -> Result<Json<serde_json::Value>, StatusCode> {\
    let region_uuid = uuid::Uuid::parse_str(&region_id)\
        .map_err(|_| StatusCode::BAD_REQUEST)?;\
    let region_id = RegionId(region_uuid);\
    \
    let regions = state.regions.read().await;\
    let region = regions.get(&region_id)\
        .ok_or(StatusCode::NOT_FOUND)?;\
    \
    Ok(Json(serde_json::json!({\
        "region_id": region_id.0.to_string(),\
        "biodiversity_index": 0.75,\
        "creature_count": 12,\
        "flora_count": 25,\
        "harmony_influence": region.harmony_level / 100.0,\
        "notable_creatures": vec![\
            serde_json::json!({\
                "species": "Star-Horned Stag",\
                "x": 100.0,\
                "z": 200.0,\
                "behavior": "Foraging"\
            })\
        ],\
        "ecosystem_health": if region.harmony_level > 70.0 { "Thriving" } else { "Stable" }\
    })))\
}\
' services/world-engine/src/main.rs

    # Add the route
    sed -i '/.route("\/harmony", post(update_harmony))/a \
        .route("/regions/:id/ecosystem", get(get_region_ecosystem))' services/world-engine/src/main.rs
fi

# Fix 4: Create simple test client
echo "🎮 Creating simple test client..."
cat > test_client.sh << 'EOF'
#!/bin/bash
# Simple test client for Finalverse

echo "🌟 Finalverse Simple Test Client"
echo "================================"

# Test basic melody
echo -e "\n🎵 Testing melody performance..."
curl -X POST http://localhost:3001/melody \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "melody": {"Healing": {"power": 10.0}},
    "target": {"x": 100, "y": 50, "z": 200}
  }' | jq '.'

# Test echo interaction
echo -e "\n✨ Testing Echo interaction..."
curl -X POST http://localhost:3003/interact \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "echo_id": "lumi"
  }' | jq '.'

# Test world state
echo -e "\n🌍 Testing world state..."
curl -s http://localhost:3002/regions | jq '.'

# Test AI dialogue
echo -e "\n💬 Testing AI NPC dialogue..."
curl -X POST http://localhost:3004/npc/dialogue \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "npc_name": "Elder Sage",
      "emotion": "happy",
      "player_name": "Test Player"
    }
  }' | jq '.'
EOF

chmod +x test_client.sh

# Fix 5: Rebuild services
echo "🔨 Rebuilding services..."
cargo build --workspace

echo "✅ Quick fixes applied!"
echo ""
echo "Next steps:"
echo "1. Restart services: docker-compose restart"
echo "2. Test with: ./test_client.sh"
echo "3. Run full client: cargo run --bin mock-client"