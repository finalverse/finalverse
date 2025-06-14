use finalverse_world3d::{GridCoordinate, Position3D};

pub struct EchoSpawner;

impl EchoSpawner {
    pub fn new() -> Self { Self }

    pub async fn prepare_lumi_spawn(&self, _grid: GridCoordinate, _pos: Position3D) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn prepare_ignis_spawn(&self, _grid: GridCoordinate, _pos: Position3D) -> anyhow::Result<()> {
        Ok(())
    }
}

