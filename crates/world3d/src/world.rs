use crate::{WorldId, RegionId};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    pub id: WorldId,
    pub regions: HashMap<RegionId, crate::region::Region>,
}

impl World {
    pub fn new(id: WorldId) -> Self {
        Self { id, regions: HashMap::new() }
    }
}
