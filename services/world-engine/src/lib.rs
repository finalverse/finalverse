pub mod grid_generation;
pub mod terrain_generator;
pub mod providence_3d;

// crates/world-engine/src/lib.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Re-export for bin
pub use crate::server::*;

// Core types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegionId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionState {
    pub id: RegionId,
    pub harmony_level: f64,
    pub discord_level: f64,
    pub terrain_type: TerrainType,
    pub weather: WeatherState,
    pub active_events: Vec<WorldEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Forest,
    Desert,
    Mountain,
    Ocean,
    Plains,
    Corrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub weather_type: WeatherType,
    pub intensity: f64,
    pub wind_direction: f64,
    pub wind_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rain,
    Storm,
    DissonanceStorm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    CreatureMigration {
        species: String,
        from: RegionId,
        to: RegionId,
    },
    CelestialEvent {
        event_type: CelestialEventType,
        duration: u64,
    },
    SilenceOutbreak {
        epicenter: Coordinates,
        radius: f64,
        intensity: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelestialEventType {
    Eclipse,
    MeteorShower,
    Aurora,
    Convergence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Move(Coordinates),
    Interact(String),
    UseAbility(String),
    Craft(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: PlayerId,
    pub action: ActionType,
    pub timestamp: u64,
}

// Observer trait for event notifications
#[async_trait::async_trait]
pub trait Observer: Send + Sync {
    async fn notify(&self, event: &WorldEvent);
}

// Ecosystem module
pub mod ecosystem {
    use super::*;

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
            // Simulate ecosystem changes
            let species_list = self.species.read().await;
            for (_, species) in species_list.iter() {
                // Simple migration logic
                if rand::random::<f64>() < 0.1 {
                    // 10% chance of migration
                    if species.migration_pattern.len() >= 2 {
                        let from = species.migration_pattern[0].clone();
                        let to = species.migration_pattern[1].clone();

                        let event = WorldEvent::CreatureMigration {
                            species: species.name.clone(),
                            from,
                            to,
                        };

                        // Notify observers
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
}

// Metabolism simulator
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

// World Engine
pub struct WorldEngine {
    metabolism: Arc<MetabolismSimulator>,
    ecosystem: Arc<ecosystem::EcosystemSimulator>,
    observers: Arc<RwLock<Vec<Arc<dyn Observer>>>>,
}

impl WorldEngine {
    pub fn new() -> Self {
        Self {
            metabolism: Arc::new(MetabolismSimulator::new()),
            ecosystem: Arc::new(ecosystem::EcosystemSimulator::new()),
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_observer(&self, observer: Arc<dyn Observer>) {
        self.observers.write().await.push(observer.clone());
        self.ecosystem.register_observer(observer).await;
    }

    pub async fn process_action(&self, action: PlayerAction) {
        // Process player actions and update world state
        match action.action {
            ActionType::Move(coords) => {
                // Update player position
                println!("Player {} moved to {:?}", action.player_id.0, coords);
            }
            ActionType::Interact(target) => {
                // Handle interaction
                println!("Player {} interacted with {}", action.player_id.0, target);
            }
            ActionType::UseAbility(ability) => {
                // Process ability use
                println!("Player {} used ability {}", action.player_id.0, ability);
            }
            ActionType::Craft(item) => {
                // Handle crafting
                println!("Player {} crafted {}", action.player_id.0, item);
            }
        }
    }

    pub async fn simulate_tick(&self) {
        // Run all simulations
        self.metabolism.simulate_tick().await;
        self.ecosystem.simulate_tick().await;

        // Check for celestial events
        if rand::random::<f64>() < 0.01 {
            // 1% chance per tick
            let event = WorldEvent::CelestialEvent {
                event_type: match rand::random::<u8>() % 4 {
                    0 => CelestialEventType::Eclipse,
                    1 => CelestialEventType::MeteorShower,
                    2 => CelestialEventType::Aurora,
                    _ => CelestialEventType::Convergence,
                },
                duration: 3600, // 1 hour
            };

            let observers = self.observers.read().await;
            for observer in observers.iter() {
                observer.notify(&event).await;
            }
        }
    }

    pub fn metabolism(&self) -> Arc<MetabolismSimulator> {
        self.metabolism.clone()
    }

    pub fn ecosystem(&self) -> Arc<ecosystem::EcosystemSimulator> {
        self.ecosystem.clone()
    }
}

// Server module for HTTP API
pub mod server {
    use super::*;
    use warp::Filter;

    pub async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
        Ok(warp::reply::json(&serde_json::json!({"status": "healthy"})))
    }

    pub async fn region_handler(
        id: String,
        engine: Arc<WorldEngine>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(region) = engine.metabolism().get_region(&RegionId(id)).await {
            Ok(warp::reply::json(&region))
        } else {
            Ok(warp::reply::json(&serde_json::json!({"error": "Region not found"})))
        }
    }

    pub async fn action_handler(
        action: PlayerAction,
        engine: Arc<WorldEngine>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        engine.process_action(action).await;
        Ok(warp::reply::json(&serde_json::json!({"success": true})))
    }

    pub fn create_routes(engine: Arc<WorldEngine>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let health = warp::path!("health")
            .and(warp::get())
            .and_then(health_handler);

        let engine_get = engine.clone();
        let get_region = warp::path!("region" / String)
            .and(warp::get())
            .and(warp::any().map(move || engine_get.clone()))
            .and_then(region_handler);

        let engine_post = engine.clone();
        let post_action = warp::path!("action")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || engine_post.clone()))
            .and_then(action_handler);

        health.or(get_region).or(post_action)
    }
}