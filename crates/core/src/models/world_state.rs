// File: crates/core/src/models/world_state.rs
// Path: finalverse/crates/core/src/models/world_state.rs
// Description: Core world state models representing the persistent state of the Finalverse.
//              These models form the foundation of the entire game world's data structure.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// The complete state of a Finalverse world instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// Unique identifier for this world
    pub id: Uuid,

    /// Human-readable name (e.g., "Terra Nova", "Aethelgard")
    pub name: String,

    /// All regions within this world
    pub regions: Vec<Region>,

    /// Global harmony level (0.0 to 1.0) - represents the Song's strength
    pub global_harmony: f32,

    /// Global discord level (0.0 to 1.0) - represents the Silence's influence
    pub global_discord: f32,

    /// Currently active world events
    pub active_events: Vec<WorldEvent>,

    /// Timestamp of the last metabolism simulation tick
    pub last_metabolism_tick: DateTime<Utc>,

    /// World-specific configuration and rules
    pub world_song: WorldSong,

    /// Statistical data for monitoring
    pub statistics: WorldStatistics,
}

/// Represents a major geographical area within a world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// Unique identifier for this region
    pub id: Uuid,

    /// Reference to parent world
    pub world_id: Uuid,

    /// Region name (e.g., "Weaver's Landing", "Crystal Spires")
    pub name: String,

    /// Grid coordinates defining the region's boundaries
    pub boundaries: RegionBoundaries,

    /// Current harmony level specific to this region
    pub harmony_level: f32,

    /// Current discord level specific to this region
    pub discord_level: f32,

    /// Biome type affects generation and behavior
    pub biome_type: BiomeType,

    /// Currently loaded grids in this region
    pub active_grids: HashMap<GridCoordinate, Grid>,

    /// Region-specific events and triggers
    pub local_events: Vec<RegionEvent>,

    /// Environmental conditions
    pub environment: EnvironmentState,
}

/// Individual grid cell within a region (typically 256x256 meters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    /// Grid position within the region
    pub coordinate: GridCoordinate,

    /// All entities currently in this grid
    pub entities: HashMap<Uuid, Entity>,

    /// Terrain configuration for this grid
    pub terrain_state: TerrainState,

    /// Dynamic objects (non-entity items)
    pub dynamic_objects: Vec<DynamicObject>,

    /// Timestamp of last update
    pub last_update: DateTime<Utc>,

    /// Active player count for load balancing
    pub active_players: u32,

    /// Grid-specific modifiers from events
    pub modifiers: Vec<GridModifier>,
}

/// Grid coordinate system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32, // Height layer for 3D worlds
}

/// Defines the boundaries of a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionBoundaries {
    pub min_coord: GridCoordinate,
    pub max_coord: GridCoordinate,
}

/// The unique "theme" or ruleset for a world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSong {
    /// Base frequency affecting all generation
    pub base_frequency: f32,

    /// Harmonic patterns that influence events
    pub harmonic_patterns: Vec<HarmonicPattern>,

    /// Melodic themes for different aspects
    pub melodies: HashMap<String, Melody>,

    /// How strongly this world responds to player actions
    pub responsiveness: f32,

    /// Unique generation rules
    pub generation_rules: GenerationRules,
}

/// World-wide statistics for monitoring and balancing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldStatistics {
    pub total_players: u64,
    pub active_players: u32,
    pub total_entities: u64,
    pub events_triggered: u64,
    pub average_harmony: f32,
    pub average_discord: f32,
    pub last_reset: DateTime<Utc>,
}

impl WorldState {
    /// Create a new world with default settings
    pub fn new(name: String, world_song: WorldSong) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            regions: Vec::new(),
            global_harmony: 0.5, // Start balanced
            global_discord: 0.0,
            active_events: Vec::new(),
            last_metabolism_tick: Utc::now(),
            world_song,
            statistics: WorldStatistics::default(),
        }
    }

    /// Calculate global metrics from all regions
    pub fn recalculate_global_metrics(&mut self) {
        if self.regions.is_empty() {
            return;
        }

        let (total_harmony, total_discord) = self.regions.iter()
            .fold((0.0, 0.0), |(h, d), region| {
                (h + region.harmony_level, d + region.discord_level)
            });

        let region_count = self.regions.len() as f32;
        self.global_harmony = total_harmony / region_count;
        self.global_discord = total_discord / region_count;

        // Update statistics
        self.statistics.average_harmony = self.global_harmony;
        self.statistics.average_discord = self.global_discord;
    }
}

impl GridCoordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Calculate distance to another coordinate
    pub fn distance_to(&self, other: &GridCoordinate) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let dz = (self.z - other.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get all adjacent coordinates (including diagonals)
    pub fn adjacent_coords(&self) -> Vec<GridCoordinate> {
        let mut coords = Vec::with_capacity(26); // 3x3x3 - 1 (self)

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    coords.push(GridCoordinate::new(
                        self.x + dx,
                        self.y + dy,
                        self.z + dz,
                    ));
                }
            }
        }

        coords
    }
}