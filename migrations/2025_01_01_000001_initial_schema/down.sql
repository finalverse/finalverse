-- File: migrations/2025_01_01_000001_initial_schema/down.sql
-- Path: finalverse/migrations/2025_01_01_000001_initial_schema/down.sql
-- Description: Rollback migration for initial schema.
--              Drops all tables and related objects in correct order.

-- Drop views first
DROP VIEW IF EXISTS active_players_view;
DROP MATERIALIZED VIEW IF EXISTS world_statistics_mv;

-- Drop triggers
DROP TRIGGER IF EXISTS update_worlds_updated_at ON worlds;
DROP TRIGGER IF EXISTS update_regions_updated_at ON regions;
DROP TRIGGER IF EXISTS update_entities_updated_at ON entities;
DROP TRIGGER IF EXISTS update_players_updated_at ON players;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS player_sessions;
DROP TABLE IF EXISTS world_history;
DROP TABLE IF EXISTS npc_memories;
DROP TABLE IF EXISTS player_legends;
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS entities;
DROP TABLE IF EXISTS grids;
DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS worlds;

-- Note: We don't drop extensions as they might be used by other schemas