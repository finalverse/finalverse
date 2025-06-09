#!/bin/bash
# scripts/update_services.sh - Update all services with latest changes

echo "🔄 Updating Finalverse services..."

# Stop running services
echo "📦 Stopping existing services..."
docker-compose down
pkill -f "cargo run" || true

# Update dependencies
echo "📚 Updating dependencies..."
cargo update

# Fix any warnings
echo "🔧 Fixing warnings..."
cargo fix --lib -p finalverse-protocol --allow-dirty || true
cargo fix --bin -p harmony-service --allow-dirty || true
cargo fix --bin -p mock-client --allow-dirty || true

# Build all services
echo "🔨 Building all services..."
cargo build --workspace

# Update World Engine to include ecosystem endpoint
echo "🌍 Updating World Engine..."
cat >> services/world-engine/src/main.rs << 'EOF'

// Ecosystem endpoint
async fn get_region_ecosystem(
    State(state): State<WorldEngineState>,
    Path(region_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let region_uuid = uuid::Uuid::parse_str(&region_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let region_id = RegionId(region_uuid);
    
    let regions = state.regions.read().await;
    let region = regions.get(&region_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let notable_creatures = vec![
        serde_json::json!({
            "species": "Star-Horned Stag",
            "x": 100.0,
            "z": 200.0,
            "behavior": "Foraging"
        }),
        serde_json::json!({
            "species": "Melody Bird",
            "x": 250.0,
            "z": 150.0,
            "behavior": "Singing"
        }),
    ];
    
    Ok(Json(serde_json::json!({
        "region_id": region_id.0.to_string(),
        "biodiversity_index": 0.75,
        "creature_count": 12,
        "flora_count": 25,
        "harmony_influence": region.harmony_level / 100.0,
        "notable_creatures": notable_creatures,
        "ecosystem_health": if region.harmony_level > 70.0 { "Thriving" } else if region.harmony_level > 40.0 { "Stable" } else { "Declining" }
    })))
}
EOF

# Create test data setup script
cat > scripts/setup_test_data.sh << 'EOF'
#!/bin/bash
echo "🌱 Setting up test data..."

# Wait for services to be ready
sleep 5

# Create initial player progression
curl -X POST http://localhost:3006/grant \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "550e8400-e29b-41d4-a716-446655440000",
    "creative": 50,
    "exploration": 30,
    "restoration": 40
  }'

echo "✅ Test data setup complete!"
EOF

chmod +x scripts/setup_test_data.sh

# Restart services
echo "🚀 Restarting services..."
docker-compose up -d

echo "✅ Update complete!"
echo ""
echo "To test the updated services:"
echo "1. Run: cargo run --bin mock-client"
echo "2. Or use: ./scripts/test_services.sh"