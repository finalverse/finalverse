-- File: migrations/2025_01_01_000001_initial_schema/up.sql
-- Path: finalverse/migrations/2025_01_01_000001_initial_schema/up.sql
-- Description: Initial database schema creation for Finalverse.
--              Creates all core tables with proper indexes and constraints.
--              This migration establishes the foundational data structure.

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "btree_gist";
CREATE EXTENSION IF NOT EXISTS "postgis"; -- For spatial data

-- Worlds table: Top-level container for everything
CREATE TABLE worlds (
                        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                        name VARCHAR(255) NOT NULL UNIQUE,
                        world_song JSONB NOT NULL,
                        global_harmony REAL NOT NULL DEFAULT 0.5,
                        global_discord REAL NOT NULL DEFAULT 0.0,
                        active_events JSONB[] DEFAULT '{}',
                        last_metabolism_tick TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                        statistics JSONB NOT NULL DEFAULT '{}',
                        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
                        CONSTRAINT harmony_range CHECK (global_harmony >= 0 AND global_harmony <= 1),
                        CONSTRAINT discord_range CHECK (global_discord >= 0 AND global_discord <= 1)
);

-- Regions table: Major areas within worlds
CREATE TABLE regions (
                         id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                         world_id UUID NOT NULL REFERENCES worlds(id) ON DELETE CASCADE,
                         name VARCHAR(255) NOT NULL,
                         boundaries JSONB NOT NULL,
                         harmony_level REAL NOT NULL DEFAULT 0.5,
                         discord_level REAL NOT NULL DEFAULT 0.0,
                         biome_type VARCHAR(50) NOT NULL,
                         environment_state JSONB NOT NULL DEFAULT '{}',
                         created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                         updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
                         CONSTRAINT region_harmony_range CHECK (harmony_level >= 0 AND harmony_level <= 1),
                         CONSTRAINT region_discord_range CHECK (discord_level >= 0 AND discord_level <= 1),
                         CONSTRAINT unique_region_name_per_world UNIQUE (world_id, name)
);

-- Grids table: Individual chunks within regions
CREATE TABLE grids (
                       id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                       region_id UUID NOT NULL REFERENCES regions(id) ON DELETE CASCADE,
                       coordinate_x INTEGER NOT NULL,
                       coordinate_y INTEGER NOT NULL,
                       coordinate_z INTEGER NOT NULL DEFAULT 0,
                       terrain_state JSONB NOT NULL DEFAULT '{}',
                       active_players INTEGER NOT NULL DEFAULT 0,
                       last_update TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       modifiers JSONB[] DEFAULT '{}',

    -- Unique constraint on coordinates within a region
                       CONSTRAINT unique_grid_coordinates UNIQUE (region_id, coordinate_x, coordinate_y, coordinate_z)
);

-- Entities table: All game objects
CREATE TABLE entities (
                          id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                          entity_type VARCHAR(50) NOT NULL,
                          entity_data JSONB NOT NULL,
                          grid_id UUID REFERENCES grids(id) ON DELETE SET NULL,
                          transform JSONB NOT NULL,
                          appearance JSONB NOT NULL DEFAULT '{}',
                          components JSONB NOT NULL DEFAULT '{}',
                          state JSONB NOT NULL DEFAULT '{"active": true}',
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Players table: Player character data
CREATE TABLE players (
                         id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                         account_id UUID NOT NULL,
                         character_name VARCHAR(100) NOT NULL UNIQUE,
                         entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
                         resonance_data JSONB NOT NULL DEFAULT '{"creative": 0, "exploration": 0, "restoration": 0}',
                         chronicle JSONB NOT NULL DEFAULT '{"legends": [], "relationships": {}}',
                         inventory JSONB NOT NULL DEFAULT '{"items": []}',
                         skills JSONB NOT NULL DEFAULT '{}',
                         last_login TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                         total_playtime BIGINT NOT NULL DEFAULT 0, -- in seconds
                         created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                         updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- One character per account for now
                         CONSTRAINT unique_account_character UNIQUE (account_id)
);

-- Events table: Event sourcing for all game events
CREATE TABLE events (
                        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                        event_type VARCHAR(100) NOT NULL,
                        event_data JSONB NOT NULL,
                        source JSONB NOT NULL,
                        priority SMALLINT NOT NULL DEFAULT 1,
                        timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                        processed BOOLEAN NOT NULL DEFAULT FALSE,
                        processing_results JSONB,

    -- Constraints
                        CONSTRAINT priority_range CHECK (priority >= 0 AND priority <= 3)
);

-- Player legends table: Major accomplishments
CREATE TABLE player_legends (
                                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
                                title VARCHAR(255) NOT NULL,
                                description TEXT NOT NULL,
                                achieved_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                witnesses UUID[] DEFAULT '{}',
                                world_effect TEXT,
                                significance REAL NOT NULL DEFAULT 1.0,

    -- Prevent duplicate legends
                                CONSTRAINT unique_legend_per_player UNIQUE (player_id, title)
);

-- NPC memories table: Dynamic NPC memory system
CREATE TABLE npc_memories (
                              id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                              npc_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
                              memory_type VARCHAR(50) NOT NULL,
                              memory_data JSONB NOT NULL,
                              emotional_weight REAL NOT NULL DEFAULT 0.5,
                              created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                              expires_at TIMESTAMPTZ,

    -- Memories decay over time
                              CONSTRAINT valid_expiry CHECK (expires_at IS NULL OR expires_at > created_at)
);

-- World history table: Permanent record of major events
CREATE TABLE world_history (
                               id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                               world_id UUID NOT NULL REFERENCES worlds(id) ON DELETE CASCADE,
                               event_type VARCHAR(100) NOT NULL,
                               event_description TEXT NOT NULL,
                               participants UUID[] DEFAULT '{}',
                               impact_level REAL NOT NULL DEFAULT 1.0,
                               occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                               permanent_changes JSONB DEFAULT '{}',

    -- Historical events should have meaningful impact
                               CONSTRAINT minimum_impact CHECK (impact_level >= 0.1)
);

-- Session tracking table for active connections
CREATE TABLE player_sessions (
                                 id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                 player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
                                 session_token VARCHAR(255) NOT NULL UNIQUE,
                                 ip_address INET,
                                 connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                 last_heartbeat TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                 disconnected_at TIMESTAMPTZ,

    -- Only one active session per player
                                 CONSTRAINT one_active_session_per_player EXCLUDE USING gist (
        player_id WITH =,
        tstzrange(connected_at, disconnected_at, '[)') WITH &&
    ) WHERE (disconnected_at IS NULL)
);

-- Create indexes for performance
CREATE INDEX idx_regions_world ON regions(world_id);
CREATE INDEX idx_regions_harmony ON regions(harmony_level);
CREATE INDEX idx_regions_discord ON regions(discord_level);

CREATE INDEX idx_grids_region ON grids(region_id);
CREATE INDEX idx_grids_coordinates ON grids(region_id, coordinate_x, coordinate_y, coordinate_z);
CREATE INDEX idx_grids_active_players ON grids(active_players) WHERE active_players > 0;

CREATE INDEX idx_entities_grid ON entities(grid_id) WHERE grid_id IS NOT NULL;
CREATE INDEX idx_entities_type ON entities(entity_type);
CREATE INDEX idx_entities_updated ON entities(updated_at);

CREATE INDEX idx_players_account ON players(account_id);
CREATE INDEX idx_players_name ON players(character_name);
CREATE INDEX idx_players_entity ON players(entity_id);

CREATE INDEX idx_events_timestamp ON events(timestamp);
CREATE INDEX idx_events_processed ON events(processed, priority) WHERE NOT processed;
CREATE INDEX idx_events_type ON events(event_type);

CREATE INDEX idx_legends_player ON player_legends(player_id);
CREATE INDEX idx_legends_achieved ON player_legends(achieved_at);

CREATE INDEX idx_npc_memories_npc ON npc_memories(npc_id);
CREATE INDEX idx_npc_memories_expires ON npc_memories(expires_at) WHERE expires_at IS NOT NULL;

CREATE INDEX idx_world_history_world ON world_history(world_id);
CREATE INDEX idx_world_history_occurred ON world_history(occurred_at);
CREATE INDEX idx_world_history_impact ON world_history(impact_level);

CREATE INDEX idx_sessions_player ON player_sessions(player_id);
CREATE INDEX idx_sessions_heartbeat ON player_sessions(last_heartbeat) WHERE disconnected_at IS NULL;

-- Create update timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply update trigger to all relevant tables
CREATE TRIGGER update_worlds_updated_at BEFORE UPDATE ON worlds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_regions_updated_at BEFORE UPDATE ON regions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_entities_updated_at BEFORE UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_players_updated_at BEFORE UPDATE ON players
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create views for common queries
CREATE VIEW active_players_view AS
SELECT
    p.id,
    p.character_name,
    p.last_login,
    e.transform->>'position' as position,
        g.coordinate_x,
        g.coordinate_y,
        g.coordinate_z,
        r.name as region_name,
        w.name as world_name
        FROM players p
        JOIN entities e ON p.entity_id = e.id
        LEFT JOIN grids g ON e.grid_id = g.id
        LEFT JOIN regions r ON g.region_id = r.id
        LEFT JOIN worlds w ON r.world_id = w.id
        WHERE p.last_login > NOW() - INTERVAL '30 minutes';

-- Create materialized view for world statistics
CREATE MATERIALIZED VIEW world_statistics_mv AS
SELECT
    w.id as world_id,
    w.name as world_name,
    COUNT(DISTINCT r.id) as region_count,
    COUNT(DISTINCT g.id) as grid_count,
    COUNT(DISTINCT p.id) as total_players,
    AVG(r.harmony_level) as avg_harmony,
    AVG(r.discord_level) as avg_discord,
    SUM(g.active_players) as active_players,
    MAX(p.last_login) as last_activity
FROM worlds w
         LEFT JOIN regions r ON w.id = r.world_id
         LEFT JOIN grids g ON r.id = g.region_id
         LEFT JOIN players p ON p.last_login > NOW() - INTERVAL '24 hours'
GROUP BY w.id, w.name;

-- Create index on materialized view
CREATE UNIQUE INDEX idx_world_stats_mv_world ON world_statistics_mv(world_id);

-- Comments for documentation
COMMENT ON TABLE worlds IS 'Top-level worlds in the Finalverse';
COMMENT ON TABLE regions IS 'Major geographical areas within each world';
COMMENT ON TABLE grids IS 'Individual 256x256 meter chunks that make up regions';
COMMENT ON TABLE entities IS 'All game objects using ECS pattern';
COMMENT ON TABLE players IS 'Player character data and progression';
COMMENT ON TABLE events IS 'Event sourcing table for all game events';
COMMENT ON TABLE player_legends IS 'Major player accomplishments that affect the world';
COMMENT ON TABLE npc_memories IS 'Dynamic memory system for NPCs';
COMMENT ON TABLE world_history IS 'Permanent record of world-changing events';
COMMENT ON TABLE player_sessions IS 'Active player connection tracking';