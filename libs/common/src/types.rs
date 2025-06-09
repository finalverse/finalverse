// # Fix the libs/common/src/types.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EchoId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Melody {
    pub notes: Vec<Note>,
    pub tempo: f32,
    pub harmony_type: HarmonyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub frequency: f32,
    pub duration: f32,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarmonyType {
    Creative,
    Restoration,
    Exploration,
    Protection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub regions: std::collections::HashMap<RegionId, RegionState>,
    pub global_harmony: f32,
    pub active_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionState {
    pub harmony_level: f32,
    pub population: u32,
    pub dominant_echo: Option<EchoId>,
    pub corruption_level: f32,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Coordinates {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Coordinates) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

impl Melody {
    pub fn new(harmony_type: HarmonyType) -> Self {
        Self {
            notes: Vec::new(),
            tempo: 120.0,
            harmony_type,
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn duration(&self) -> f32 {
        self.notes.iter().map(|n| n.duration).sum()
    }
}

impl Note {
    pub fn new(frequency: f32, duration: f32, intensity: f32) -> Self {
        Self {
            frequency,
            duration,
            intensity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EchoType {
    Lumi,
    KAI,
    Terra,
    Ignis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoState {
    pub echo_type: EchoType,
    pub current_region: Option<RegionId>,
    pub bond_levels: std::collections::HashMap<PlayerId, f32>,
    pub active_teachings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgress {
    pub player_id: PlayerId,
    pub total_resonance: f32,
    pub creative_resonance: f32,
    pub exploration_resonance: f32,
    pub restoration_resonance: f32,
    pub attunement_tier: u32,
    pub learned_melodies: Vec<String>,
    pub discovered_regions: Vec<RegionId>,
}

impl PlayerProgress {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            total_resonance: 0.0,
            creative_resonance: 0.0,
            exploration_resonance: 0.0,
            restoration_resonance: 0.0,
            attunement_tier: 1,
            learned_melodies: Vec::new(),
            discovered_regions: Vec::new(),
        }
    }

    pub fn add_resonance(&mut self, amount: f32, resonance_type: &str) {
        match resonance_type {
            "creative" => self.creative_resonance += amount,
            "exploration" => self.exploration_resonance += amount,
            "restoration" => self.restoration_resonance += amount,
            _ => {}
        }
        self.total_resonance = self.creative_resonance + self.exploration_resonance + self.restoration_resonance;
        
        let new_tier = (self.total_resonance / 100.0).floor() as u32 + 1;
        if new_tier > self.attunement_tier {
            self.attunement_tier = new_tier;
        }
    }
}