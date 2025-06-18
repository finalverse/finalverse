# Behavior AI Service

This microservice manages MapleAI agents and exposes a small HTTP API.

## Endpoints

### `POST /agent/spawn`
Creates a new agent and stores it in memory.
Request body:
```json
{ "id": "agent1", "region": "start" }
```
Response:
```json
{ "id": "agent1" }
```

### `POST /agent/{id}/act`
Updates the agent context, performs one reasoning step and returns its next action.
Request body example:
```json
{
  "location": "town",
  "nearby_entities": ["npc1"],
  "harmony_level": 0.8,
  "tension": 0.0,
  "memory": []
}
```
Response example:
```json
{ "action": { "kind": "rest" } }
```

Health information is available under `/health` and `/info`.
