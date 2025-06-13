// services/symphony-engine/src/world_audio_state.rs
use finalverse_audio_core::*;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct WorldAudioState {
    regions: HashMap<String, RegionAudioState>,
    active_events: Vec<AudioEvent>,
    global_harmony: f32,
    celestial_state: CelestialState,
}

impl WorldAudioState {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            active_events: Vec::new(),
            global_harmony: 0.5,
            celestial_state: CelestialState::default(),
        }
    }

    pub async fn process_event(&mut self, event: AudioEvent) {
        match event.event_type {
            AudioEventType::RegionHarmonyChanged { region_id, harmony_level } => {
                if let Some(region) = self.regions.get_mut(&region_id) {
                    region.harmony_level = harmony_level;
                    self.recalculate_global_harmony();
                }
            }
            AudioEventType::CelestialEvent { event_name } => {
                self.celestial_state.process_event(&event_name);
            }
            AudioEventType::EchoAppearance { echo_type } => {
                // Update active echoes in the region
                if let Some(position) = event.position {
                    let region_id = self.position_to_region(position);
                    if let Some(region) = self.regions.get_mut(&region_id) {
                        region.active_echoes.push(echo_type);
                    }
                }
            }
            _ => {
                // Store event for processing
                self.active_events.push(event);
            }
        }
    }

    pub fn get_active_regions(&self) -> Vec<&RegionAudioState> {
        self.regions.values().collect()
    }

    fn recalculate_global_harmony(&mut self) {
        let total_harmony: f32 = self.regions.values()
            .map(|r| r.harmony_level)
            .sum();
        self.global_harmony = total_harmony / self.regions.len() as f32;
    }

    fn position_to_region(&self, position: nalgebra::Vector3<f32>) -> String {
        // Simple grid-based region mapping
        // In production, this would use proper spatial indexing
        let grid_x = (position.x / 1000.0).floor() as i32;
        let grid_z = (position.z / 1000.0).floor() as i32;
        format!("region_{}_{}", grid_x, grid_z)
    }
}

#[derive(Default)]
pub struct CelestialState {
    pub time_of_day: f32, // 0.0 - 24.0
    pub moon_phase: f32,  // 0.0 - 1.0
    pub active_events: Vec<String>,
}

impl CelestialState {
    pub fn process_event(&mut self, event_name: &str) {
        self.active_events.push(event_name.to_string());
    }
}

pub use crate::music_ai::RegionAudioState;