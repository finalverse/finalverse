// services/world-engine/src/metabolism.rs
use crate::{RegionId, RegionState, TerrainType, WeatherType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
            // Natural harmony decay
            region.harmony_level *= 1.0 - self.harmony_decay_rate;

            // Discord spreads if not countered
            if region.discord_level > 0.1 {
                region.discord_level *= 1.0 + self.discord_spread_rate;

                // Corrupt terrain if discord is too high
                if region.discord_level > 0.8 {
                    region.terrain_type = TerrainType::Corrupted;
                }
            }

            // Weather changes based on harmony/discord
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