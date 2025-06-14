// services/first-hour/src/echo_spawner.rs
use finalverse_world3d::{Position3D, GridCoordinate, EntityId};
use uuid::Uuid;
use std::collections::HashMap;

pub struct EchoSpawner {
    prepared_spawns: HashMap<String, PreparedSpawn>,
}

struct PreparedSpawn {
    grid: GridCoordinate,
    position: Position3D,
    echo_type: EchoType,
    trigger_condition: TriggerCondition,
}

#[derive(Clone, Copy, Debug)]
pub enum EchoType {
    Lumi,
    KAI,
    Terra,
    Ignis,
}

#[derive(Clone)]
enum TriggerCondition {
    OnEvent(String),
    OnPlayerProgress(String),
    Immediate,
}

impl EchoSpawner {
    pub fn new() -> Self {
        Self {
            prepared_spawns: HashMap::new(),
        }
    }

    pub async fn prepare_lumi_spawn(
        &mut self,
        grid: GridCoordinate,
        position: Position3D,
    ) -> anyhow::Result<()> {
        self.prepared_spawns.insert(
            "lumi_first_appearance".to_string(),
            PreparedSpawn {
                grid,
                position,
                echo_type: EchoType::Lumi,
                trigger_condition: TriggerCondition::OnEvent("character_creation_complete".to_string()),
            },
        );
        Ok(())
    }

    pub async fn prepare_ignis_spawn(
        &mut self,
        grid: GridCoordinate,
        position: Position3D,
    ) -> anyhow::Result<()> {
        self.prepared_spawns.insert(
            "ignis_arrival".to_string(),
            PreparedSpawn {
                grid,
                position,
                echo_type: EchoType::Ignis,
                trigger_condition: TriggerCondition::OnEvent("gloom_shade_encounter".to_string()),
            },
        );
        Ok(())
    }

    pub async fn trigger_spawn(&mut self, spawn_id: &str) -> anyhow::Result<Option<EntityId>> {
        if let Some(spawn) = self.prepared_spawns.remove(spawn_id) {
            // In production, this would communicate with world-engine to spawn the entity
            let entity_id = EntityId(Uuid::new_v4());
            tracing::info!(
                "Spawning {:?} at {:?} in grid {:?}",
                spawn.echo_type,
                spawn.position,
                spawn.grid
            );
            Ok(Some(entity_id))
        } else {
            Ok(None)
        }
    }
}