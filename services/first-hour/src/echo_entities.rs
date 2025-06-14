// services/first-hour/src/echo_entities.rs
use finalverse_world3d::{
    entities::{Entity, EntityType, Mesh, Animation},
    Position3D, EntityId,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoEntity {
    pub id: EntityId,
    pub echo_type: EchoType,
    pub position: Position3D,
    pub mesh: Mesh,
    pub animations: Vec<Animation>,
    pub particle_effects: Vec<ParticleEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EchoType {
    Lumi,
    KAI,
    Terra,
    Ignis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEffect {
    pub effect_type: String,
    pub color: [f32; 4],
    pub emission_rate: f32,
}

impl EchoEntity {
    pub fn create_lumi(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            echo_type: EchoType::Lumi,
            position,
            mesh: Mesh {
                model_id: "echo_lumi".to_string(),
                materials: vec![
                    Material {
                        name: "lumi_body".to_string(),
                        shader: "glow_shader".to_string(),
                        properties: hashmap! {
                            "emission_color" => vec![0.8, 0.9, 1.0, 1.0],
                            "emission_strength" => vec![2.0],
                        },
                    },
                ],
            },
            animations: vec![
                Animation {
                    name: "idle_float".to_string(),
                    loop_mode: AnimationLoop::Loop,
                    duration: 3.0,
                },
                Animation {
                    name: "curious_tilt".to_string(),
                    loop_mode: AnimationLoop::Once,
                    duration: 1.5,
                },
            ],
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "sparkle_trail".to_string(),
                    color: [0.8, 0.9, 1.0, 0.6],
                    emission_rate: 20.0,
                },
            ],
        }
    }

    pub fn create_kai(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            echo_type: EchoType::KAI,
            position,
            mesh: Mesh {
                model_id: "echo_kai".to_string(),
                materials: vec![
                    Material {
                        name: "kai_core".to_string(),
                        shader: "hologram_shader".to_string(),
                        properties: hashmap! {
                            "base_color" => vec![0.2, 0.5, 0.9, 0.8],
                            "scan_lines" => vec![1.0],
                            "glitch_amount" => vec![0.05],
                        },
                    },
                ],
            },
            animations: vec![
                Animation {
                    name: "data_processing".to_string(),
                    loop_mode: AnimationLoop::Loop,
                    duration: 2.0,
                },
                Animation {
                    name: "analyzing".to_string(),
                    loop_mode: AnimationLoop::Once,
                    duration: 3.0,
                },
            ],
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "digital_particles".to_string(),
                    color: [0.2, 0.5, 0.9, 0.4],
                    emission_rate: 15.0,
                },
            ],
        }
    }

    pub fn create_terra(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            echo_type: EchoType::Terra,
            position,
            mesh: Mesh {
                model_id: "echo_terra".to_string(),
                materials: vec![
                    Material {
                        name: "terra_bark".to_string(),
                        shader: "nature_shader".to_string(),
                        properties: hashmap! {
                            "base_texture" => vec!["bark_ancient.png"],
                            "moss_coverage" => vec![0.3],
                            "wind_sway" => vec![0.1],
                        },
                    },
                    Material {
                        name: "terra_leaves".to_string(),
                        shader: "foliage_shader".to_string(),
                        properties: hashmap! {
                            "leaf_color" => vec![0.3, 0.7, 0.2, 1.0],
                            "subsurface" => vec![0.4],
                        },
                    },
                ],
            },
            animations: vec![
                Animation {
                    name: "gentle_sway".to_string(),
                    loop_mode: AnimationLoop::Loop,
                    duration: 6.0,
                },
                Animation {
                    name: "root_pulse".to_string(),
                    loop_mode: AnimationLoop::Loop,
                    duration: 4.0,
                },
            ],
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "falling_leaves".to_string(),
                    color: [0.3, 0.7, 0.2, 1.0],
                    emission_rate: 3.0,
                },
                ParticleEffect {
                    effect_type: "nature_spirits".to_string(),
                    color: [0.5, 0.9, 0.3, 0.3],
                    emission_rate: 5.0,
                },
            ],
        }
    }

    pub fn create_ignis(position: Position3D) -> Self {
        Self {
            id: EntityId(Uuid::new_v4()),
            echo_type: EchoType::Ignis,
            position,
            mesh: Mesh {
                model_id: "echo_ignis".to_string(),
                materials: vec![
                    Material {
                        name: "ignis_armor".to_string(),
                        shader: "fire_metal_shader".to_string(),
                        properties: hashmap! {
                            "base_color" => vec![0.8, 0.3, 0.1, 1.0],
                            "metallic" => vec![0.8],
                            "heat_glow" => vec![2.0],
                        },
                    },
                    Material {
                        name: "ignis_flames".to_string(),
                        shader: "fire_shader".to_string(),
                        properties: hashmap! {
                            "flame_color1" => vec![1.0, 0.6, 0.1, 1.0],
                            "flame_color2" => vec![1.0, 0.2, 0.0, 0.8],
                            "turbulence" => vec![0.5],
                        },
                    },
                ],
            },
            animations: vec![
                Animation {
                    name: "battle_ready".to_string(),
                    loop_mode: AnimationLoop::Loop,
                    duration: 2.5,
                },
                Animation {
                    name: "heroic_pose".to_string(),
                    loop_mode: AnimationLoop::Once,
                    duration: 3.0,
                },
            ],
            particle_effects: vec![
                ParticleEffect {
                    effect_type: "ember_trail".to_string(),
                    color: [1.0, 0.4, 0.1, 0.8],
                    emission_rate: 30.0,
                },
                ParticleEffect {
                    effect_type: "fire_aura".to_string(),
                    color: [1.0, 0.3, 0.0, 0.5],
                    emission_rate: 50.0,
                },
            ],
        }
    }
}