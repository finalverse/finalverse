// services/first-hour/src/scenes.rs
use finalverse_world3d::{
    grid::Grid,
    entities::Entity,
    Position3D,
};
use crate::{
    echo_entities::{EchoEntity, EchoType},
    interactive_objects::{InteractiveObject, NPCEntity},
};

impl super::FirstHourManager {
    pub async fn add_memory_grotto_features(&self, grid: &mut Grid) -> anyhow::Result<()> {
        // Central pool with ethereal water
        let pool_center = Position3D::new(128.0, 128.0, 50.0);

        // Add glowing crystals around the grotto
        let crystal_positions = vec![
            Position3D::new(110.0, 110.0, 52.0),
            Position3D::new(146.0, 110.0, 52.0),
            Position3D::new(146.0, 146.0, 52.0),
            Position3D::new(110.0, 146.0, 52.0),
        ];

        for pos in crystal_positions {
            let crystal = InteractiveObject::create_memory_crystal(pos);
            grid.add_entity(Entity::Interactive(crystal));
        }

        // Add ambient particle effects
        grid.add_ambient_effect("grotto_mist", pool_center, 30.0);
        grid.add_ambient_effect("light_motes", pool_center, 50.0);

        Ok(())
    }

    pub async fn add_weavers_landing_structures(&self, grid: &mut Grid) -> anyhow::Result<()> {
        // Anya's Workshop
        let workshop_pos = Position3D::new(180.0, 140.0, 52.0);
        grid.add_structure("anyas_workshop", workshop_pos);

        // Add Anya NPC
        let anya = NPCEntity::create_anya(Position3D::new(182.0, 142.0, 52.0));
        grid.add_entity(Entity::NPC(anya));

        // Add the faded Star Whale statue
        let statue = InteractiveObject::create_anya_statue(
            Position3D::new(185.0, 145.0, 52.5)
        );
        grid.add_entity(Entity::Interactive(statue));

        // Plaza of Echoes
        let plaza_center = Position3D::new(150.0, 150.0, 51.0);
        grid.add_structure("plaza_echoes", plaza_center);

        // River and bridges
        grid.add_structure("wooden_bridge_01", Position3D::new(140.0, 120.0, 50.5));
        grid.add_structure("wooden_bridge_02", Position3D::new(160.0, 180.0, 50.5));

        // Other town buildings
        let buildings = vec![
            ("house_willow_01", Position3D::new(120.0, 130.0, 52.0)),
            ("house_willow_02", Position3D::new(130.0, 160.0, 52.0)),
            ("market_stall_01", Position3D::new(145.0, 155.0, 51.0)),
            ("market_stall_02", Position3D::new(155.0, 145.0, 51.0)),
        ];

        for (building_type, pos) in buildings {
            grid.add_structure(building_type, pos);
        }

        Ok(())
    }

    pub async fn add_whisperwood_features(&self, grid: &mut Grid) -> anyhow::Result<()> {
        // Ancient trees with special properties
        let ancient_tree_positions = vec![
            Position3D::new(50.0, 80.0, 55.0),
            Position3D::new(100.0, 100.0, 56.0),
            Position3D::new(150.0, 90.0, 55.5),
            Position3D::new(200.0, 120.0, 56.0),
        ];

        for pos in ancient_tree_positions {
            grid.add_structure("ancient_tree_whisperwood", pos);
        }

        // The Resonant Blossom location
        let blossom = InteractiveObject::create_resonant_blossom(
            Position3D::new(210.0, 190.0, 56.0)
        );
        grid.add_entity(Entity::Interactive(blossom));

        // Add mystical fog effect
        grid.add_ambient_effect("whisperwood_fog", Position3D::new(128.0, 128.0, 55.0), 100.0);

        // Story stones along the path
        let story_stone_positions = vec![
            Position3D::new(20.0, 30.0, 53.0),
            Position3D::new(60.0, 50.0, 53.5),
            Position3D::new(100.0, 70.0, 54.0),
        ];

        for (i, pos) in story_stone_positions.iter().enumerate() {
            grid.add_structure(&format!("story_stone_{:02}", i + 1), *pos);
        }

        Ok(())
    }

    pub async fn place_first_hour_entities(&mut self) -> anyhow::Result<()> {
        // Place Lumi in the Memory Grotto (appears after character creation)
        if let Some(grotto_grid) = self.scene_grids.get_mut("memory_grotto") {
            let lumi = EchoEntity::create_lumi(Position3D::new(130.0, 130.0, 51.0));
            grotto_grid.add_entity(Entity::Echo(lumi));
        }

        // Ignis appears in Weaver's Landing after the Gloom Shade encounter
        if let Some(landing_grid) = self.scene_grids.get_mut("weavers_landing") {
            // Pre-position but mark as inactive until triggered
            let ignis = EchoEntity::create_ignis(Position3D::new(150.0, 150.0, 51.5));
            landing_grid.add_entity_inactive(Entity::Echo(ignis));

            // Place the Gloom Shade (spawns after statue restoration)
            let shade = InteractiveObject::create_gloom_shade(
                Position3D::new(183.0, 143.0, 52.0)
            );
            landing_grid.add_entity_inactive(Entity::Interactive(shade));
        }

        Ok(())
    }
}