// services/first-hour/src/first_hour_manager.rs
use finalverse_world3d::{Position3D, GridCoordinate};
use crate::echo_spawner::{EchoSpawner, EchoType};
use crate::interactive_objects::{InteractiveObjectManager, InteractiveType, ObjectState, NPCState};
use crate::scenes::SceneDefinitions;
use crate::PlayerEvent;
use std::collections::HashMap;

pub struct FirstHourSceneManager {
    echo_spawner: EchoSpawner,
    object_manager: InteractiveObjectManager,
    scene_states: HashMap<String, SceneState>,
}

#[derive(Debug, Clone)]
pub enum SceneState {
    Uninitialized,
    Initialized,
    Active,
    Completed,
}

impl FirstHourSceneManager {
    pub fn new() -> Self {
        Self {
            echo_spawner: EchoSpawner::new(),
            object_manager: InteractiveObjectManager::new(),
            scene_states: HashMap::new(),
        }
    }

    pub async fn setup_memory_grotto(&mut self) -> anyhow::Result<()> {
        let layout = SceneDefinitions::memory_grotto_layout();

        // Add memory crystals for character creation
        for (name, pos) in &layout.key_positions {
            if name.starts_with("crystal_") {
                self.object_manager.spawn_memory_crystal(layout.grid, *pos).await?;
            }
        }

        // Lumi spawns here after character creation
        self.echo_spawner.prepare_lumi_spawn(
            layout.grid,
            Position3D::new(130.0, 130.0, 51.0)
        ).await?;

        self.scene_states.insert("memory_grotto".to_string(), SceneState::Initialized);
        tracing::info!("Memory Grotto scene initialized");
        Ok(())
    }

    pub async fn setup_weavers_landing(&mut self) -> anyhow::Result<()> {
        let layout = SceneDefinitions::weavers_landing_layout();

        // Spawn Anya NPC
        self.object_manager.spawn_npc(
            layout.grid,
            "anya",
            Position3D::new(182.0, 142.0, 52.0),
            NPCState::InitialSadness
        ).await?;

        // Spawn the faded Star Whale statue
        self.object_manager.spawn_interactive(
            layout.grid,
            InteractiveType::AnyaStatue,
            Position3D::new(185.0, 145.0, 52.5),
            ObjectState::Faded
        ).await?;

        // Prepare Ignis spawn (triggered later)
        self.echo_spawner.prepare_ignis_spawn(
            layout.grid,
            Position3D::new(150.0, 150.0, 51.5)
        ).await?;

        self.scene_states.insert("weavers_landing".to_string(), SceneState::Initialized);
        tracing::info!("Weaver's Landing scene initialized");
        Ok(())
    }

    pub async fn setup_whisperwood_grove(&mut self) -> anyhow::Result<()> {
        let layout = SceneDefinitions::whisperwood_grove_layout();

        // Spawn the Resonant Blossom
        self.object_manager.spawn_interactive(
            layout.grid,
            InteractiveType::ResonantBlossom,
            Position3D::new(210.0, 190.0, 56.0),
            ObjectState::Dormant
        ).await?;

        self.scene_states.insert("whisperwood_grove".to_string(), SceneState::Initialized);
        tracing::info!("Whisperwood Grove scene initialized");
        Ok(())
    }

    pub async fn handle_player_event(&mut self, event: PlayerEvent) -> anyhow::Result<()> {
        match event.event_type.as_str() {
            "character_creation_complete" => {
                if let Some(entity_id) = self.echo_spawner.trigger_spawn("lumi_first_appearance").await? {
                    tracing::info!("Lumi spawned: {:?}", entity_id);
                }
            },
            "statue_restored" => {
                // Trigger Gloom Shade appearance
                tracing::info!("Statue restored, preparing for Gloom Shade encounter");
            },
            "gloom_shade_defeated" => {
                if let Some(entity_id) = self.echo_spawner.trigger_spawn("ignis_arrival").await? {
                    tracing::info!("Ignis has arrived: {:?}", entity_id);
                }
            },
            _ => {
                tracing::debug!("Unhandled event type: {}", event.event_type);
            }
        }
        Ok(())
    }
}