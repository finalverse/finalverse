// crates/world3d/src/grid.rs
use crate::{
    GridCoordinate, EntityId, Position3D,
    terrain::TerrainPatch,
    entities::Entity,
};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Grid {
    pub coordinate: GridCoordinate,
    pub terrain: TerrainPatch,
    pub entities: HashMap<EntityId, Entity>,
    pub inactive_entities: HashMap<EntityId, Entity>, // Entities waiting to be triggered
    pub structures: Vec<Structure>,
    pub ambient_effects: Vec<AmbientEffect>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Structure {
    pub structure_type: String,
    pub position: Position3D,
    pub rotation: f32,
    pub scale: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AmbientEffect {
    pub effect_type: String,
    pub position: Position3D,
    pub radius: f32,
}

impl Grid {
    pub fn new(coordinate: GridCoordinate, terrain: TerrainPatch) -> Self {
        Self {
            coordinate,
            terrain,
            entities: HashMap::new(),
            inactive_entities: HashMap::new(),
            structures: Vec::new(),
            ambient_effects: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let id = entity.get_id();
        self.entities.insert(id, entity);
    }

    pub fn add_entity_inactive(&mut self, entity: Entity) {
        let id = entity.get_id();
        self.inactive_entities.insert(id, entity);
    }

    pub fn activate_entity(&mut self, entity_id: EntityId) -> Option<Entity> {
        if let Some(entity) = self.inactive_entities.remove(&entity_id) {
            self.entities.insert(entity_id, entity.clone());
            Some(entity)
        } else {
            None
        }
    }

    pub fn add_structure(&mut self, structure_type: &str, position: Position3D) {
        self.structures.push(Structure {
            structure_type: structure_type.to_string(),
            position,
            rotation: 0.0,
            scale: 1.0,
        });
    }

    pub fn add_ambient_effect(&mut self, effect_type: &str, position: Position3D, radius: f32) {
        self.ambient_effects.push(AmbientEffect {
            effect_type: effect_type.to_string(),
            position,
            radius,
        });
    }
}