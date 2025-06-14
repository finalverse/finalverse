// services/first-hour/src/interactive_objects.rs
use finalverse_world3d::{Position3D, GridCoordinate, EntityId};
use uuid::Uuid;
use std::collections::HashMap;

pub struct InteractiveObjectManager {
    objects: HashMap<EntityId, InteractiveObject>,
    npcs: HashMap<String, NPCData>,
}

#[derive(Clone)]
struct InteractiveObject {
    id: EntityId,
    grid: GridCoordinate,
    position: Position3D,
    object_type: InteractiveType,
    state: ObjectState,
}

#[derive(Clone)]
pub enum InteractiveType {
    MemoryCrystal,
    AnyaStatue,
    ResonantBlossom,
    GloomShade,
}

#[derive(Debug, Clone)]
pub enum ObjectState {
    Active,
    Dormant,
    Faded,
    Corrupted,
    Restored,
}

#[derive(Clone)]
struct NPCData {
    id: EntityId,
    name: String,
    grid: GridCoordinate,
    position: Position3D,
    state: NPCState,
}

#[derive(Clone)]
pub enum NPCState {
    InitialSadness,
    Hopeful,
    Inspired,
    Grateful,
}

impl InteractiveObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            npcs: HashMap::new(),
        }
    }

    pub async fn spawn_memory_crystal(
        &mut self,
        grid: GridCoordinate,
        position: Position3D,
    ) -> anyhow::Result<EntityId> {
        let id = EntityId(Uuid::new_v4());
        let crystal = InteractiveObject {
            id,
            grid,
            position,
            object_type: InteractiveType::MemoryCrystal,
            state: ObjectState::Active,
        };

        self.objects.insert(id, crystal);
        Ok(id)
    }

    pub async fn spawn_npc(
        &mut self,
        grid: GridCoordinate,
        npc_name: &str,
        position: Position3D,
        initial_state: NPCState,
    ) -> anyhow::Result<EntityId> {
        let id = EntityId(Uuid::new_v4());
        let npc = NPCData {
            id,
            name: npc_name.to_string(),
            grid,
            position,
            state: initial_state,
        };

        self.npcs.insert(npc_name.to_string(), npc);
        Ok(id)
    }

    pub async fn spawn_interactive(
        &mut self,
        grid: GridCoordinate,
        object_type: InteractiveType,
        position: Position3D,
        initial_state: ObjectState,
    ) -> anyhow::Result<EntityId> {
        let id = EntityId(Uuid::new_v4());
        let object = InteractiveObject {
            id,
            grid,
            position,
            object_type,
            state: initial_state,
        };

        self.objects.insert(id, object);
        Ok(id)
    }

    pub async fn update_object_state(
        &mut self,
        id: EntityId,
        new_state: ObjectState,
    ) -> anyhow::Result<()> {
        if let Some(object) = self.objects.get_mut(&id) {
            object.state = new_state.clone();
            tracing::info!("Updated object {:?} to state {:?}", id, new_state);
        }
        Ok(())
    }
}