// File: crates/core/src/database/schema.rs
// Path: finalverse/crates/core/src/database/schema.rs
// Description: Database schema definitions using Diesel ORM for PostgreSQL.
//              Defines all persistent storage tables and relationships.

use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

// Diesel table definitions
table! {
    worlds (id) {
        id -> Uuid,
        name -> Varchar,
        world_song -> Jsonb,
        global_harmony -> Float4,
        global_discord -> Float4,
        active_events -> Array<Jsonb>,
        last_metabolism_tick -> Timestamptz,
        statistics -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    regions (id) {
        id -> Uuid,
        world_id -> Uuid,
        name -> Varchar,
        boundaries -> Jsonb,
        harmony_level -> Float4,
        discord_level -> Float4,
        biome_type -> Varchar,
        environment_state -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    grids (id) {
        id -> Uuid,
        region_id -> Uuid,
        coordinate_x -> Int4,
        coordinate_y -> Int4,
        coordinate_z -> Int4,
        terrain_state -> Jsonb,
        active_players -> Int4,
        last_update -> Timestamptz,
        modifiers -> Array<Jsonb>,
    }
}

table! {
    entities (id) {
        id -> Uuid,
        entity_type -> Varchar,
        entity_data -> Jsonb,
        grid_id -> Nullable<Uuid>,
        transform -> Jsonb,
        appearance -> Jsonb,
        components -> Jsonb,
        state -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    players (id) {
        id -> Uuid,
        account_id -> Uuid,
        character_name -> Varchar,
        entity_id -> Uuid,
        resonance_data -> Jsonb,
        chronicle -> Jsonb,
        inventory -> Jsonb,
        skills -> Jsonb,
        last_login -> Timestamptz,
        total_playtime -> Int8, // seconds
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    events (id) {
        id -> Uuid,
        event_type -> Varchar,
        event_data -> Jsonb,
        source -> Jsonb,
        priority -> Int2,
        timestamp -> Timestamptz,
        processed -> Bool,
        processing_results -> Nullable<Jsonb>,
    }
}

table! {
    player_legends (id) {
        id -> Uuid,
        player_id -> Uuid,
        title -> Varchar,
        description -> Text,
        achieved_at -> Timestamptz,
        witnesses -> Array<Uuid>,
        world_effect -> Nullable<Text>,
        significance -> Float4,
    }
}

table! {
    npc_memories (id) {
        id -> Uuid,
        npc_id -> Uuid,
        memory_type -> Varchar,
        memory_data -> Jsonb,
        emotional_weight -> Float4,
        created_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
    }
}

table! {
    world_history (id) {
        id -> Uuid,
        world_id -> Uuid,
        event_type -> Varchar,
        event_description -> Text,
        participants -> Array<Uuid>,
        impact_level -> Float4,
        occurred_at -> Timestamptz,
        permanent_changes -> Jsonb,
    }
}

// Define foreign key relationships
joinable!(regions -> worlds (world_id));
joinable!(grids -> regions (region_id));
joinable!(entities -> grids (grid_id));
joinable!(players -> entities (entity_id));
joinable!(player_legends -> players (player_id));
joinable!(world_history -> worlds (world_id));

allow_tables_to_appear_in_same_query!(
    worlds,
    regions,
    grids,
    entities,
    players,
    events,
    player_legends,
    npc_memories,
    world_history,
);