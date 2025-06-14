// services/first-hour/src/weaver_landing_3d.rs

pub struct WeaverLanding3D {
    grid_coordinate: GridCoordinate,
    key_locations: HashMap<String, Position3D>,
    interactive_objects: Vec<InteractiveObject3D>,
}

impl WeaverLanding3D {
    pub fn new() -> Self {
        Self {
            grid_coordinate: GridCoordinate::new(100, 100), // Starting grid
            key_locations: hashmap! {
                "memory_grotto" => Position3D::new(128.0, 128.0, 50.0),
                "anyas_workshop" => Position3D::new(180.0, 140.0, 52.0),
                "plaza_of_echoes" => Position3D::new(150.0, 150.0, 51.0),
                "whisperwood_entrance" => Position3D::new(200.0, 180.0, 55.0),
            },
            interactive_objects: vec![
                InteractiveObject3D {
                    id: "anyas_statue",
                    position: Position3D::new(182.0, 142.0, 52.5),
                    mesh: "star_whale_statue",
                    interaction_type: InteractionType::Harmony,
                    current_state: ObjectState::Faded,
                },
                InteractiveObject3D {
                    id: "resonant_blossom",
                    position: Position3D::new(210.0, 190.0, 56.0),
                    mesh: "glowing_flower_closed",
                    interaction_type: InteractionType::Songweave,
                    current_state: ObjectState::Dormant,
                },
            ],
        }
    }
}