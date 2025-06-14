// File: crates/core/src/database/repositories/world_repository.rs
// Path: finalverse/crates/core/src/database/repositories/world_repository.rs
// Description: Repository for world data access and persistence.
//              Handles all database operations for worlds.

use super::{Repository, RepositoryError};
use crate::database::connection::DbConnection;
use crate::database::schema::worlds;
use crate::models::world_state::{WorldState, WorldSong, WorldStatistics};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use serde_json;

/// World repository for database operations
pub struct WorldRepository;

impl WorldRepository {
    pub fn new() -> Self {
        Self
    }

    /// Find world by name
    pub fn find_by_name(&self, conn: &mut DbConnection, name: &str) -> Result<WorldState, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let record = dsl::worlds
            .filter(dsl::name.eq(name))
            .first::<WorldRecord>(conn)?;

        Ok(self.record_to_model(record))
    }

    /// Get worlds with active events
    pub fn find_with_active_events(&self, conn: &mut DbConnection) -> Result<Vec<WorldState>, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let records = dsl::worlds
            .filter(dsl::active_events.is_not_null())
            .filter(diesel::dsl::sql("array_length(active_events, 1) > 0"))
            .load::<WorldRecord>(conn)?;

        Ok(records.into_iter().map(|r| self.record_to_model(r)).collect())
    }

    /// Update world harmony and discord levels
    pub fn update_harmony_discord(
        &self,
        conn: &mut DbConnection,
        world_id: Uuid,
        harmony: f32,
        discord: f32,
    ) -> Result<(), RepositoryError> {
        use crate::database::schema::worlds::dsl;

        diesel::update(dsl::worlds.find(world_id))
            .set((
                dsl::global_harmony.eq(harmony),
                dsl::global_discord.eq(discord),
                dsl::updated_at.eq(Utc::now()),
            ))
            .execute(conn)?;

        Ok(())
    }

    /// Convert database record to domain model
    fn record_to_model(&self, record: WorldRecord) -> WorldState {
        WorldState {
            id: record.id,
            name: record.name,
            world_song: serde_json::from_value(record.world_song).unwrap_or_default(),
            global_harmony: record.global_harmony,
            global_discord: record.global_discord,
            active_events: record.active_events
                .into_iter()
                .filter_map(|e| serde_json::from_value(e).ok())
                .collect(),
            last_metabolism_tick: record.last_metabolism_tick,
            statistics: serde_json::from_value(record.statistics).unwrap_or_default(),
            regions: Vec::new(), // Loaded separately
        }
    }

    /// Convert domain model to database record
    fn model_to_record(&self, model: &WorldState) -> NewWorldRecord {
        NewWorldRecord {
            name: model.name.clone(),
            world_song: serde_json::to_value(&model.world_song).unwrap(),
            global_harmony: model.global_harmony,
            global_discord: model.global_discord,
            active_events: model.active_events
                .iter()
                .map(|e| serde_json::to_value(e).unwrap())
                .collect(),
            last_metabolism_tick: model.last_metabolism_tick,
            statistics: serde_json::to_value(&model.statistics).unwrap(),
        }
    }
}

impl Repository for WorldRepository {
    type Entity = WorldState;
    type Id = Uuid;

    fn find_by_id(&self, conn: &mut DbConnection, id: Uuid) -> Result<WorldState, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let record = dsl::worlds
            .find(id)
            .first::<WorldRecord>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => RepositoryError::NotFound(format!("World {}", id)),
                _ => RepositoryError::DatabaseError(e),
            })?;

        Ok(self.record_to_model(record))
    }

    fn find_all(&self, conn: &mut DbConnection) -> Result<Vec<WorldState>, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let records = dsl::worlds
            .order(dsl::created_at.asc())
            .load::<WorldRecord>(conn)?;

        Ok(records.into_iter().map(|r| self.record_to_model(r)).collect())
    }

    fn create(&self, conn: &mut DbConnection, entity: WorldState) -> Result<WorldState, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let new_record = self.model_to_record(&entity);

        let record = diesel::insert_into(dsl::worlds)
            .values(&new_record)
            .get_result::<WorldRecord>(conn)?;

        Ok(self.record_to_model(record))
    }

    fn update(&self, conn: &mut DbConnection, entity: WorldState) -> Result<WorldState, RepositoryError> {
        use crate::database::schema::worlds::dsl;

        let record = diesel::update(dsl::worlds.find(entity.id))
            .set(&UpdateWorldRecord {
                name: entity.name.clone(),
                world_song: serde_json::to_value(&entity.world_song).unwrap(),
                global_harmony: entity.global_harmony,
                global_discord: entity.global_discord,
                active_events: entity.active_events
                    .iter()
                    .map(|e| serde_json::to_value(e).unwrap())
                    .collect(),
                last_metabolism_tick: entity.last_metabolism_tick,
                statistics: serde_json::to_value(&entity.statistics).unwrap(),
                updated_at: Utc::now(),
            })
            .get_result::<WorldRecord>(conn)?;

        Ok(self.record_to_model(record))
    }

    fn delete(&self, conn: &mut DbConnection, id: Uuid) -> Result<(), RepositoryError> {
        use crate::database::schema::worlds::dsl;

        diesel::delete(dsl::worlds.find(id))
            .execute(conn)?;

        Ok(())
    }
}

// Database record structs
#[derive(Queryable, Debug)]
struct WorldRecord {
    id: Uuid,
    name: String,
    world_song: serde_json::Value,
    global_harmony: f32,
    global_discord: f32,
    active_events: Vec<serde_json::Value>,
    last_metabolism_tick: chrono::DateTime<Utc>,
    statistics: serde_json::Value,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = worlds)]
struct NewWorldRecord {
    name: String,
    world_song: serde_json::Value,
    global_harmony: f32,
    global_discord: f32,
    active_events: Vec<serde_json::Value>,
    last_metabolism_tick: chrono::DateTime<Utc>,
    statistics: serde_json::Value,
}

#[derive(AsChangeset)]
#[diesel(table_name = worlds)]
struct UpdateWorldRecord {
    name: String,
    world_song: serde_json::Value,
    global_harmony: f32,
    global_discord: f32,
    active_events: Vec<serde_json::Value>,
    last_metabolism_tick: chrono::DateTime<Utc>,
    statistics: serde_json::Value,
    updated_at: chrono::DateTime<Utc>,
}