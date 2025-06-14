// services/first-hour/src/first_hour_manager.rs
use finalverse_world3d::{Position3D, GridCoordinate};
use std::collections::HashMap;
use crate::echo_spawner::EchoSpawner;
use finalverse_world3d::interactive_objects::{
    InteractiveObjectManager, NPCState, ObjectState, ObjectType as InteractiveType,
};

pub struct FirstHourSceneManager {
    echo_spawner: EchoSpawner,
    object_manager: InteractiveObjectManager,
    scene_states: HashMap<String, SceneState>,
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
        let grid = GridCoordinate::new(100, 100);

        // Add memory crystals for character creation
        let crystal_positions = vec![
            Position3D::new(110.0, 110.0, 52.0),
            Position3D::new(146.0, 110.0, 52.0),
            Position3D::new(146.0, 146.0, 52.0),
            Position3D::new(110.0, 146.0, 52.0),
        ];

        for pos in crystal_positions {
            self.object_manager.spawn_memory_crystal(grid, pos).await?;
        }

        // Lumi spawns here after character creation
        self.echo_spawner.prepare_lumi_spawn(
            grid,
            Position3D::new(130.0, 130.0, 51.0)
        ).await?;

        self.scene_states.insert("memory_grotto".to_string(), SceneState::Initialized);
        Ok(())
    }

    pub async fn setup_weavers_landing(&mut self) -> anyhow::Result<()> {
        let grid = GridCoordinate::new(101, 101);

        // Spawn Anya NPC
        self.object_manager.spawn_npc(
            grid,
            "anya",
            Position3D::new(182.0, 142.0, 52.0),
            NPCState::InitialSadness
        ).await?;

        // Spawn the faded Star Whale statue
        self.object_manager.spawn_interactive(
            grid,
            InteractiveType::AnyaStatue,
            Position3D::new(185.0, 145.0, 52.5),
            ObjectState::Faded
        ).await?;

        // Prepare Ignis spawn (triggered later)
        self.echo_spawner.prepare_ignis_spawn(
            grid,
            Position3D::new(150.0, 150.0, 51.5)
        ).await?;

        self.scene_states.insert("weavers_landing".to_string(), SceneState::Initialized);
        Ok(())
    }

    pub async fn setup_whisperwood_grove(&mut self) -> anyhow::Result<()> {
        let grid = GridCoordinate::new(102, 101);

        // Spawn the Resonant Blossom
        self.object_manager.spawn_interactive(
            grid,
            InteractiveType::ResonantBlossom,
            Position3D::new(210.0, 190.0, 56.0),
            ObjectState::Dormant
        ).await?;

        self.scene_states.insert("whisperwood_grove".to_string(), SceneState::Initialized);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum SceneState {
    Uninitialized,
    Initialized,
    Active,
    Completed,
}
