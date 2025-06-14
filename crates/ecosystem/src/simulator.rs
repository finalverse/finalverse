use crate::Species;
use finalverse_metobolism::{RegionId, TerrainType};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesProfile {
    pub id: String,
    pub name: String,
    pub species: Species,
    pub population: u64,
    pub migration_pattern: Vec<RegionId>,
    pub preferred_terrain: Vec<TerrainType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemEvent {
    CreatureMigration { species: String, from: RegionId, to: RegionId },
}

#[async_trait::async_trait]
pub trait EcosystemObserver: Send + Sync {
    async fn notify(&self, event: &EcosystemEvent);
}

pub struct EcosystemSimulator {
    species: Arc<RwLock<HashMap<String, SpeciesProfile>>>,
    observers: Arc<RwLock<Vec<Arc<dyn EcosystemObserver>>>>,
}

impl EcosystemSimulator {
    pub fn new() -> Self {
        Self {
            species: Arc::new(RwLock::new(HashMap::new())),
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_observer(&self, observer: Arc<dyn EcosystemObserver>) {
        self.observers.write().await.push(observer);
    }

    pub async fn simulate_tick(&self) {
        let species_list = self.species.read().await;
        for (_, sp) in species_list.iter() {
            if rand::random::<f64>() < 0.1 {
                if sp.migration_pattern.len() >= 2 {
                    let from = sp.migration_pattern[0].clone();
                    let to = sp.migration_pattern[1].clone();
                    let event = EcosystemEvent::CreatureMigration {
                        species: sp.name.clone(),
                        from,
                        to,
                    };
                    let observers = self.observers.read().await;
                    for obs in observers.iter() {
                        obs.notify(&event).await;
                    }
                }
            }
        }
    }

    pub async fn add_species(&self, species: SpeciesProfile) {
        self.species.write().await.insert(species.id.clone(), species);
    }
}
