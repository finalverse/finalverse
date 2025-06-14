use crate::{GridCoordinate, PlayerId};
use std::collections::HashMap;

pub struct SpatialTracker {
    pub players: HashMap<PlayerId, GridCoordinate>,
}

impl SpatialTracker {
    pub fn new() -> Self {
        Self { players: HashMap::new() }
    }
}
