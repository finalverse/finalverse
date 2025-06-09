#!/bin/bash
echo "Testing Finalverse services..."

echo "🎵 Song Engine:" && curl -s http://localhost:3001/info | jq .
echo "🌍 World Engine:" && curl -s http://localhost:3002/info | jq .
echo "✨ Echo Engine:" && curl -s http://localhost:3003/info | jq .
echo "🤖 AI Orchestra:" && curl -s http://localhost:3004/info | jq .

echo -e "\n📡 Testing via API Gateway:"
curl -s http://localhost:8080/api/song/info | jq .
