// services/world-engine/src/world.rs
use crate::{
    RegionId, RegionState, WorldEvent, PlayerAction, ActionType, Observer,
    GridCoordinate, Position3D, EchoType, CelestialEventType, EcosystemSimulator,
    MetabolismSimulator,
};
use finalverse_ecosystem::{EcosystemEvent, EcosystemObserver};

struct EcosystemAdapter {
    observer: Arc<dyn Observer>,
}

#[async_trait::async_trait]
impl EcosystemObserver for EcosystemAdapter {
    async fn notify(&self, event: &EcosystemEvent) {
        let world_event = match event {
            EcosystemEvent::CreatureMigration { species, from, to } => {
                WorldEvent::CreatureMigration {
                    species: species.clone(),
                    from: from.clone(),
                    to: to.clone(),
                }
            }
        };
        self.observer.notify(&world_event).await;
    }
}
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub regions: HashMap<RegionId, RegionState>,
    pub global_harmony: f32,
    pub active_events: Vec<WorldEvent>,
    pub time: WorldTime,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            global_harmony: 0.5,
            active_events: Vec::new(),
            time: WorldTime::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub day: u32,
    pub hour: f32,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            day: 1,
            hour: 6.0,
        }
    }
}

impl WorldTime {
    pub fn advance(&mut self, delta_hours: f32) {
        self.hour += delta_hours;
        while self.hour >= 24.0 {
            self.hour -= 24.0;
            self.day += 1;
        }
    }
}

pub enum WorldUpdate {
    HarmonyChange { region_id: RegionId, delta: f32 },
    EventTriggered { event: WorldEvent },
}

pub struct WorldEngine {
    state: Arc<RwLock<WorldState>>,
    metabolism: Arc<MetabolismSimulator>,
    ecosystem: Arc<EcosystemSimulator>,
    observers: Arc<RwLock<Vec<Arc<dyn Observer>>>>,
    update_queue: Arc<RwLock<Vec<WorldUpdate>>>,
}

impl WorldEngine {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(WorldState::new())),
            metabolism: Arc::new(MetabolismSimulator::new()),
            ecosystem: Arc::new(EcosystemSimulator::new()),
            observers: Arc::new(RwLock::new(Vec::new())),
            update_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn get_state(&self) -> WorldState {
        self.state.read().await.clone()
    }

    pub async fn register_observer(&self, observer: Arc<dyn Observer>) {
        self.observers.write().await.push(observer.clone());
        let adapter = Arc::new(EcosystemAdapter { observer });
        self.ecosystem.register_observer(adapter).await;
    }

    pub async fn process_action(&self, action: PlayerAction) {
        match action.action {
            ActionType::Move(coords) => {
                println!("Player {} moved to {:?}", action.player_id.0, coords);
            }
            ActionType::Interact(target) => {
                println!("Player {} interacted with {}", action.player_id.0, target);
            }
            ActionType::UseAbility(ability) => {
                println!("Player {} used ability {}", action.player_id.0, ability);
            }
            ActionType::Craft(item) => {
                println!("Player {} crafted {}", action.player_id.0, item);
            }
        }
    }

    pub async fn update(&self, delta_time: f32) {
        // Process queued updates
        let updates: Vec<WorldUpdate> = {
            let mut queue = self.update_queue.write().await;
            queue.drain(..).collect()
        };

        for update in updates {
            self.apply_update(update).await;
        }

        // Update world time
        let mut state = self.state.write().await;
        state.time.advance(delta_time);
    }

    async fn apply_update(&self, update: WorldUpdate) {
        let mut state = self.state.write().await;

        match update {
            WorldUpdate::HarmonyChange { region_id, delta } => {
                if let Some(region) = state.regions.get_mut(&region_id) {
                    region.harmony_level = (region.harmony_level + delta as f64).clamp(0.0, 1.0);
                }
            }
            WorldUpdate::EventTriggered { event } => {
                state.active_events.push(event);
            }
        }
    }

    pub async fn simulate_tick(&self) {
        // Run all simulations
        self.metabolism.simulate_tick().await;
        self.ecosystem.simulate_tick().await;

        // Check for celestial events
        if rand::random::<f64>() < 0.01 {
            let event = WorldEvent::CelestialEvent {
                event_type: match rand::random::<u8>() % 4 {
                    0 => CelestialEventType::Eclipse,
                    1 => CelestialEventType::MeteorShower,
                    2 => CelestialEventType::Aurora,
                    _ => CelestialEventType::Convergence,
                },
                duration: 3600,
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

    pub fn ecosystem(&self) -> Arc<EcosystemSimulator> {
        self.ecosystem.clone()
    }

    pub async fn update_region_harmony(
        &self,
        region_id: &RegionId,
        delta: f32,
    ) -> anyhow::Result<HarmonyUpdateResult> {
        let new_level = self
            .metabolism
            .update_harmony(region_id, delta as f64)
            .await
            .ok_or_else(|| anyhow::anyhow!("Region not found"))?;

        Ok(HarmonyUpdateResult {
            new_harmony_level: new_level as f32,
            triggered_events: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HarmonyUpdateResult {
    pub new_harmony_level: f32,
    pub triggered_events: Vec<WorldEvent>,
}