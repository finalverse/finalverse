finalverse-config
A comprehensive configuration management library split into logical modules:

lib.rs - Main entry point with convenience functions
config.rs - All configuration structures with defaults:

General settings (server name, logging, debug mode)
Network configuration (ports, timeouts, connections)
Service endpoints for all microservices
AI configuration (LLMs, procedural generation, behavior AI)
Database settings (PostgreSQL, TimescaleDB, Qdrant)
Cache configuration (Redis, in-memory)
Security settings (JWT, rate limiting, encryption)
Performance tuning
Monitoring and metrics
Game-specific settings (world, harmony, echoes, events)


loader.rs - Configuration loading utilities:

Load from TOML files
Support for environment-specific overrides
Sample config generation
Save configurations


validator.rs - Comprehensive validation:

Validates all configuration values
Checks for port conflicts
Ensures required fields are present
Validates ranges and constraints


environment.rs - Environment variable support:

Override any setting via environment variables
Generate .env template
Support for API keys and secrets