// crates/world3d/src/lib.rs
pub mod world;
pub mod region;
pub mod grid;
pub mod terrain;
pub mod entities;
pub mod spatial;
pub mod interactive_objects;
pub mod echo_entities;

use serde::{Deserialize, Serialize};
use nalgebra::{Vector3, Point3};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GridCoordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<GridCoordinate> {
        vec![
            GridCoordinate::new(self.x - 1, self.y - 1),
            GridCoordinate::new(self.x, self.y - 1),
            GridCoordinate::new(self.x + 1, self.y - 1),
            GridCoordinate::new(self.x - 1, self.y),
            GridCoordinate::new(self.x + 1, self.y),
            GridCoordinate::new(self.x - 1, self.y + 1),
            GridCoordinate::new(self.x, self.y + 1),
            GridCoordinate::new(self.x + 1, self.y + 1),
        ]
    }
}

impl Position3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_grid_coordinate(&self) -> GridCoordinate {
        GridCoordinate::new(
            (self.x / 256.0).floor() as i32,
            (self.y / 256.0).floor() as i32,
        )
    }

    pub fn distance_to(&self, other: &Position3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}