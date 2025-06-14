use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use shared domain types from finalverse-core
pub use finalverse_core::{RegionId, TerrainType, WeatherType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub weather_type: WeatherType,
    pub intensity: f64,
    pub wind_direction: f64,
    pub wind_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionState {
    pub id: RegionId,
    pub harmony_level: f64,
    pub discord_level: f64,
    pub terrain_type: TerrainType,
    pub weather: WeatherState,
}

pub struct MetabolismSimulator {
    regions: Arc<RwLock<HashMap<RegionId, RegionState>>>,
    harmony_decay_rate: f64,
    discord_spread_rate: f64,
}

impl MetabolismSimulator {
    pub fn new() -> Self {
        Self {
            regions: Arc::new(RwLock::new(HashMap::new())),
            harmony_decay_rate: 0.01,
            discord_spread_rate: 0.02,
        }
    }

    pub async fn simulate_tick(&self) {
        let mut regions = self.regions.write().await;
        for (_, region) in regions.iter_mut() {
            region.harmony_level *= 1.0 - self.harmony_decay_rate;
            if region.discord_level > 0.1 {
                region.discord_level *= 1.0 + self.discord_spread_rate;
                if region.discord_level > 0.8 {
                    region.terrain_type = TerrainType::Corrupted;
                }
            }
            if region.discord_level > 0.5 && rand::random::<f64>() < 0.3 {
                region.weather.weather_type = WeatherType::DissonanceStorm;
            }
        }
    }

    pub async fn add_region(&self, region: RegionState) {
        self.regions.write().await.insert(region.id.clone(), region);
    }

    pub async fn get_region(&self, id: &RegionId) -> Option<RegionState> {
        self.regions.read().await.get(id).cloned()
    }
}
