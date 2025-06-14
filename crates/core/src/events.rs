use crate::types::{Coordinates, EchoId, Melody, PlayerId, RegionId, WeatherType};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinalverseEvent {
    // Song Events
    HarmonyRestored {
        region: RegionId,
        restorer: PlayerId,
        harmony_level: f32,
        timestamp: DateTime<Utc>,
    },
    SilenceManifested {
        location: Coordinates,
        intensity: f32,
        affected_area: f32,
        timestamp: DateTime<Utc>,
    },
    SymphonyInitiated {
        symphony_type: SymphonyType,
        participants: Vec<PlayerId>,
        region: RegionId,
        timestamp: DateTime<Utc>,
    },
    
    // World Events
    CreatureMigration {
        species: String,
        from: RegionId,
        to: RegionId,
        population: u32,
        timestamp: DateTime<Utc>,
    },
    CelestialEvent {
        event_type: CelestialEventType,
        affected_regions: Vec<RegionId>,
        duration_hours: f32,
        timestamp: DateTime<Utc>,
    },
    RegionDiscovered {
        region: RegionId,
        discoverer: PlayerId,
        region_type: String,
        timestamp: DateTime<Utc>,
    },
    
    // Player Events
    SongweavingPerformed {
        player: PlayerId,
        melody: Melody,
        target: Coordinates,
        success: bool,
        resonance_gained: f32,
        timestamp: DateTime<Utc>,
    },
    EchoBondIncreased {
        player: PlayerId,
        echo: EchoId,
        previous_level: f32,
        new_level: f32,
        milestone_reached: Option<String>,
        timestamp: DateTime<Utc>,
    },
    QuestCompleted {
        player: PlayerId,
        quest_id: String,
        quest_type: String,
        rewards: Vec<String>,
        impact_description: String,
        timestamp: DateTime<Utc>,
    },
    
    // AI Events
    NPCMemoryFormed {
        npc_id: String,
        memory_type: String,
        related_players: Vec<PlayerId>,
        emotional_impact: f32,
        timestamp: DateTime<Utc>,
    },
    QuestGenerated {
        quest_id: String,
        quest_type: String,
        target_players: Vec<PlayerId>,
        region: RegionId,
        generated_by: String, // AI system that generated it
        timestamp: DateTime<Utc>,
    },
    WorldStateChanged {
        region: RegionId,
        change_type: String,
        description: String,
        caused_by: Option<PlayerId>,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SongEvent {
    MelodyWoven {
        player_id: PlayerId,
        melody: Melody,
        target: Coordinates,
    },
    HarmonyAchieved {
        participants: Vec<PlayerId>,
        harmony_type: String,
        power_level: f32,
    },
    DissonanceDetected {
        location: Coordinates,
        intensity: f32,
        source: String,
    },
    SilenceCorruption {
        region: RegionId,
        corruption_level: f32,
        affected_entities: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarmonyEvent {
    ResonanceGained {
        player_id: PlayerId,
        amount: f32,
        resonance_type: String,
    },
    AttunementTierIncreased {
        player_id: PlayerId,
        new_tier: u32,
        abilities_unlocked: Vec<String>,
    },
    CollaborationBonus {
        participants: Vec<PlayerId>,
        bonus_multiplier: f32,
        activity: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    WeatherChanged {
        region: RegionId,
        new_weather: WeatherType,
        duration_hours: f32,
    },
    EcosystemShift {
        region: RegionId,
        shift_type: String,
        impact_level: f32,
    },
    ResourceDiscovered {
        location: Coordinates,
        resource_type: String,
        quantity: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymphonyType {
    CreationSymphony,
    RestorationSymphony,
    ProtectionSymphony,
    ExplorationSymphony,
    HarmonySymphony,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelestialEventType {
    Eclipse,
    MeteorShower,
    Aurora,
    StarAlignment,
    CosmicStorm,
    StarBirth,
    StarWhaleVisit,
}


impl FinalverseEvent {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            FinalverseEvent::HarmonyRestored { timestamp, .. } => timestamp,
            FinalverseEvent::SilenceManifested { timestamp, .. } => timestamp,
            FinalverseEvent::SymphonyInitiated { timestamp, .. } => timestamp,
            FinalverseEvent::CreatureMigration { timestamp, .. } => timestamp,
            FinalverseEvent::CelestialEvent { timestamp, .. } => timestamp,
            FinalverseEvent::RegionDiscovered { timestamp, .. } => timestamp,
            FinalverseEvent::SongweavingPerformed { timestamp, .. } => timestamp,
            FinalverseEvent::EchoBondIncreased { timestamp, .. } => timestamp,
            FinalverseEvent::QuestCompleted { timestamp, .. } => timestamp,
            FinalverseEvent::NPCMemoryFormed { timestamp, .. } => timestamp,
            FinalverseEvent::QuestGenerated { timestamp, .. } => timestamp,
            FinalverseEvent::WorldStateChanged { timestamp, .. } => timestamp,
        }
    }

    pub fn involves_player(&self, player_id: &PlayerId) -> bool {
        match self {
            FinalverseEvent::HarmonyRestored { restorer, .. } => restorer == player_id,
            FinalverseEvent::SymphonyInitiated { participants, .. } => participants.contains(player_id),
            FinalverseEvent::RegionDiscovered { discoverer, .. } => discoverer == player_id,
            FinalverseEvent::SongweavingPerformed { player, .. } => player == player_id,
            FinalverseEvent::EchoBondIncreased { player, .. } => player == player_id,
            FinalverseEvent::QuestCompleted { player, .. } => player == player_id,
            FinalverseEvent::NPCMemoryFormed { related_players, .. } => related_players.contains(player_id),
            FinalverseEvent::QuestGenerated { target_players, .. } => target_players.contains(player_id),
            FinalverseEvent::WorldStateChanged { caused_by, .. } => {
                if let Some(causer) = caused_by {
                    causer == player_id
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn event_type(&self) -> &'static str {
        match self {
            FinalverseEvent::HarmonyRestored { .. } => "harmony_restored",
            FinalverseEvent::SilenceManifested { .. } => "silence_manifested",
            FinalverseEvent::SymphonyInitiated { .. } => "symphony_initiated",
            FinalverseEvent::CreatureMigration { .. } => "creature_migration",
            FinalverseEvent::CelestialEvent { .. } => "celestial_event",
            FinalverseEvent::RegionDiscovered { .. } => "region_discovered",
            FinalverseEvent::SongweavingPerformed { .. } => "songweaving_performed",
            FinalverseEvent::EchoBondIncreased { .. } => "echo_bond_increased",
            FinalverseEvent::QuestCompleted { .. } => "quest_completed",
            FinalverseEvent::NPCMemoryFormed { .. } => "npc_memory_formed",
            FinalverseEvent::QuestGenerated { .. } => "quest_generated",
            FinalverseEvent::WorldStateChanged { .. } => "world_state_changed",
        }
    }
}