// services/world-engine/src/ecosystem.rs
use crate::{Observer, RegionId, TerrainType, WorldEvent};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: String,
    pub name: String,
    pub population: u64,
    pub migration_pattern: Vec<RegionId>,
    pub preferred_terrain: Vec<TerrainType>,
}

pub struct EcosystemSimulator {
    species: Arc<RwLock<HashMap<String, Species>>>,
    observers: Arc<RwLock<Vec<Arc<dyn Observer>>>>,
}

impl EcosystemSimulator {
    pub fn new() -> Self {
        Self {
            species: Arc::new(RwLock::new(HashMap::new())),
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_observer(&self, observer: Arc<dyn Observer>) {
        self.observers.write().await.push(observer);
    }

    pub async fn simulate_tick(&self) {
        let species_list = self.species.read().await;
        for (_, species) in species_list.iter() {
            if rand::random::<f64>() < 0.1 {
                if species.migration_pattern.len() >= 2 {
                    let from = species.migration_pattern[0].clone();
                    let to = species.migration_pattern[1].clone();

                    let event = WorldEvent::CreatureMigration {
                        species: species.name.clone(),
                        from,
                        to,
                    };

                    let observers = self.observers.read().await;
                    for observer in observers.iter() {
                        observer.notify(&event).await;
                    }
                }
            }
        }
    }

    pub async fn add_species(&self, species: Species) {
        self.species.write().await.insert(species.id.clone(), species);
    }
}