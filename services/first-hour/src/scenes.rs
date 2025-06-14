// services/first-hour/src/scenes.rs
use finalverse_world3d::{Position3D, GridCoordinate};

pub struct SceneDefinitions;

impl SceneDefinitions {
    pub fn memory_grotto_layout() -> SceneLayout {
        SceneLayout {
            name: "Memory Grotto".to_string(),
            grid: GridCoordinate::new(100, 100),
            key_positions: vec![
                ("grotto_center", Position3D::new(128.0, 128.0, 50.0)),
                ("crystal_north", Position3D::new(128.0, 110.0, 52.0)),
                ("crystal_east", Position3D::new(146.0, 128.0, 52.0)),
                ("crystal_south", Position3D::new(128.0, 146.0, 52.0)),
                ("crystal_west", Position3D::new(110.0, 128.0, 52.0)),
            ],
            ambient_effects: vec![
                ("grotto_mist", Position3D::new(128.0, 128.0, 50.0), 30.0),
                ("light_motes", Position3D::new(128.0, 128.0, 51.0), 50.0),
            ],
        }
    }

    pub fn weavers_landing_layout() -> SceneLayout {
        SceneLayout {
            name: "Weaver's Landing".to_string(),
            grid: GridCoordinate::new(101, 101),
            key_positions: vec![
                ("anyas_workshop", Position3D::new(180.0, 140.0, 52.0)),
                ("plaza_center", Position3D::new(150.0, 150.0, 51.0)),
                ("bridge_north", Position3D::new(140.0, 120.0, 50.5)),
                ("bridge_south", Position3D::new(160.0, 180.0, 50.5)),
            ],
            ambient_effects: vec![
                ("river_sounds", Position3D::new(150.0, 100.0, 50.0), 100.0),
                ("town_ambience", Position3D::new(150.0, 150.0, 51.0), 80.0),
            ],
        }
    }

    pub fn whisperwood_grove_layout() -> SceneLayout {
        SceneLayout {
            name: "Whisperwood Grove".to_string(),
            grid: GridCoordinate::new(102, 101),
            key_positions: vec![
                ("grove_entrance", Position3D::new(20.0, 128.0, 54.0)),
                ("resonant_blossom", Position3D::new(210.0, 190.0, 56.0)),
                ("ancient_tree_1", Position3D::new(100.0, 100.0, 56.0)),
                ("ancient_tree_2", Position3D::new(150.0, 90.0, 55.5)),
            ],
            ambient_effects: vec![
                ("whisperwood_fog", Position3D::new(128.0, 128.0, 55.0), 100.0),
                ("forest_whispers", Position3D::new(150.0, 150.0, 55.0), 120.0),
            ],
        }
    }
}

pub struct SceneLayout {
    pub name: String,
    pub grid: GridCoordinate,
    pub key_positions: Vec<(&'static str, Position3D)>,
    pub ambient_effects: Vec<(&'static str, Position3D, f32)>, // (name, position, radius)
}