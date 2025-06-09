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
