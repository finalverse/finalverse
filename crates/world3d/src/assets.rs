// crates/world3d/src/assets.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManifest {
    pub meshes: HashMap<String, MeshAsset>,
    pub textures: HashMap<String, TextureAsset>,
    pub shaders: HashMap<String, ShaderAsset>,
    pub animations: HashMap<String, AnimationAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAsset {
    pub id: String,
    pub path: String,
    pub format: MeshFormat,
    pub lod_levels: Vec<LODLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeshFormat {
    GLTF,
    OBJ,
    FBX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LODLevel {
    pub distance: f32,
    pub mesh_path: String,
    pub vertex_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureAsset {
    pub id: String,
    pub path: String,
    pub format: TextureFormat,
    pub resolution: (u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureFormat {
    PNG,
    JPEG,
    DDS,
    KTX2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderAsset {
    pub id: String,
    pub vertex_path: String,
    pub fragment_path: String,
    pub parameters: Vec<ShaderParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderParameter {
    pub name: String,
    pub param_type: ShaderParameterType,
    pub default_value: ShaderValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderParameterType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Texture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    TextureId(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationAsset {
    pub id: String,
    pub path: String,
    pub duration: f32,
    pub frame_rate: f32,
}

impl AssetManifest {
    pub fn first_hour_assets() -> Self {
        let mut manifest = AssetManifest {
            meshes: HashMap::new(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            animations: HashMap::new(),
        };

        // Echo meshes
        manifest.add_echo_assets();

        // Environment assets
        manifest.add_environment_assets();

        // Interactive object assets
        manifest.add_interactive_assets();

        manifest
    }

    fn add_echo_assets(&mut self) {
        // Lumi
        self.meshes.insert("echo_lumi".to_string(), MeshAsset {
            id: "echo_lumi".to_string(),
            path: "assets/meshes/echoes/lumi/lumi_base.gltf".to_string(),
            format: MeshFormat::GLTF,
            lod_levels: vec![
                LODLevel { distance: 0.0, mesh_path: "lumi_lod0.gltf".to_string(), vertex_count: 5000 },
                LODLevel { distance: 50.0, mesh_path: "lumi_lod1.gltf".to_string(), vertex_count: 2000 },
                LODLevel { distance: 100.0, mesh_path: "lumi_lod2.gltf".to_string(), vertex_count: 500 },
            ],
        });

        // Add glow shader for Lumi
        self.shaders.insert("glow_shader".to_string(), ShaderAsset {
            id: "glow_shader".to_string(),
            vertex_path: "shaders/glow.vert".to_string(),
            fragment_path: "shaders/glow.frag".to_string(),
            parameters: vec![
                ShaderParameter {
                    name: "emission_color".to_string(),
                    param_type: ShaderParameterType::Vec4,
                    default_value: ShaderValue::Vec4([0.8, 0.9, 1.0, 1.0]),
                },
                ShaderParameter {
                    name: "emission_strength".to_string(),
                    param_type: ShaderParameterType::Float,
                    default_value: ShaderValue::Float(2.0),
                },
            ],
        });

        // Similar entries for KAI, Terra, and Ignis...
    }

    fn add_environment_assets(&mut self) {
        // Memory Grotto crystals
        self.meshes.insert("memory_crystal".to_string(), MeshAsset {
            id: "memory_crystal".to_string(),
            path: "assets/meshes/environment/memory_crystal.gltf".to_string(),
            format: MeshFormat::GLTF,
            lod_levels: vec![
                LODLevel { distance: 0.0, mesh_path: "crystal_lod0.gltf".to_string(), vertex_count: 1000 },
            ],
        });

        // Willow trees for Weaver's Landing
        self.meshes.insert("tree_willow_01".to_string(), MeshAsset {
            id: "tree_willow_01".to_string(),
            path: "assets/meshes/environment/trees/willow_01.gltf".to_string(),
            format: MeshFormat::GLTF,
            lod_levels: vec![
                LODLevel { distance: 0.0, mesh_path: "willow_lod0.gltf".to_string(), vertex_count: 8000 },
                LODLevel { distance: 100.0, mesh_path: "willow_lod1.gltf".to_string(), vertex_count: 2000 },
            ],
        });
    }

    fn add_interactive_assets(&mut self) {
        // Anya's Star Whale statue
        self.meshes.insert("star_whale_statue".to_string(), MeshAsset {
            id: "star_whale_statue".to_string(),
            path: "assets/meshes/interactive/star_whale_statue.gltf".to_string(),
            format: MeshFormat::GLTF,
            lod_levels: vec![
                LODLevel { distance: 0.0, mesh_path: "statue_lod0.gltf".to_string(), vertex_count: 10000 },
            ],
        });

        // Resonant Blossom
        self.meshes.insert("resonant_blossom".to_string(), MeshAsset {
            id: "resonant_blossom".to_string(),
            path: "assets/meshes/interactive/resonant_blossom.gltf".to_string(),
            format: MeshFormat::GLTF,
            lod_levels: vec![
                LODLevel { distance: 0.0, mesh_path: "blossom_lod0.gltf".to_string(), vertex_count: 2000 },
            ],
        });
    }
}