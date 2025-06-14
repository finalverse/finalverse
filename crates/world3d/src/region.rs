use crate::{RegionId, GridCoordinate, grid::Grid};
use std::collections::HashMap;

#[derive(Default)]
pub struct Region {
    pub id: RegionId,
    pub grids: HashMap<GridCoordinate, Grid>,
}

impl Region {
    pub fn new(id: RegionId) -> Self {
        Self { id, grids: HashMap::new() }
    }
}
