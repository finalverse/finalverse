// Finalverse AI World Engine Core Modules: Metabolism Simulator & Observer Service

pub mod metabolism {
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RegionState {
        pub harmony: f32,
        pub dissonance: f32,
        pub resources: f32,
        pub political_tension: f32,
    }

    pub struct MetabolismSimulator {
        pub world_map: HashMap<String, RegionState>,
        pub last_tick: Instant,
        pub tick_interval: Duration,
    }

    impl MetabolismSimulator {
        pub fn new(tick_interval_secs: u64) -> Self {
            Self {
                world_map: HashMap::new(),
                last_tick: Instant::now(),
                tick_interval: Duration::from_secs(tick_interval_secs),
            }
        }

        pub fn tick(&mut self) {
            if self.last_tick.elapsed() >= self.tick_interval {
                for (_region, state) in self.world_map.iter_mut() {
                    // Example decay model: harmony slowly falls, dissonance rises
                    state.harmony *= 0.98;
                    state.dissonance *= 1.01;
                    state.resources *= 0.995;
                    state.political_tension *= 0.99;
                }
                self.last_tick = Instant::now();
            }
        }

        pub fn apply_effect(&mut self, region: &str, effect: RegionEffect) {
            let state = self.world_map.entry(region.to_string()).or_insert_with(|| RegionState {
                harmony: 0.0,
                dissonance: 0.0,
                resources: 0.0,
                political_tension: 0.0,
            });
            state.harmony += effect.harmony_delta;
            state.dissonance += effect.dissonance_delta;
            state.resources += effect.resource_delta;
            state.political_tension += effect.political_tension_delta;
        }

        pub fn get_state(&self, region: &str) -> Option<&RegionState> {
            self.world_map.get(region)
        }
    }

    impl Default for MetabolismSimulator {
        fn default() -> Self {
            Self {
                world_map: HashMap::new(),
                last_tick: Instant::now(),
                tick_interval: Duration::from_secs(60), // Default to 60 seconds
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RegionEffect {
        pub harmony_delta: f32,
        pub dissonance_delta: f32,
        pub resource_delta: f32,
        pub political_tension_delta: f32,
    }
}

pub mod observer {
    use super::metabolism::{MetabolismSimulator, RegionEffect};

    #[derive(Debug, Clone)]
    pub struct PlayerAction {
        pub player_id: String,
        pub action_type: ActionType,
        pub region: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub enum ActionType {
        CompleteQuest,
        BuildStructure,
        Ritual,
        PvPConflict,
    }

    pub struct Observer {
        pub metabolism: MetabolismSimulator,
    }

    impl Observer {
        pub fn new(metabolism: MetabolismSimulator) -> Self {
            Self { metabolism }
        }

        pub fn interpret_action(&mut self, action: PlayerAction) {
            let effect = match action.action_type {
                ActionType::CompleteQuest => RegionEffect {
                    harmony_delta: 5.0,
                    dissonance_delta: -1.0,
                    resource_delta: 0.0,
                    political_tension_delta: -0.2,
                },
                ActionType::BuildStructure => RegionEffect {
                    harmony_delta: 3.0,
                    dissonance_delta: -0.5,
                    resource_delta: -1.0,
                    political_tension_delta: -0.1,
                },
                ActionType::Ritual => RegionEffect {
                    harmony_delta: 7.0,
                    dissonance_delta: -2.0,
                    resource_delta: 0.0,
                    political_tension_delta: -0.3,
                },
                ActionType::PvPConflict => RegionEffect {
                    harmony_delta: -2.0,
                    dissonance_delta: 4.0,
                    resource_delta: -0.5,
                    political_tension_delta: 1.0,
                },
            };

            self.metabolism.apply_effect(&action.region, effect);
        }
    }
}

pub use metabolism::*;
pub use observer::*;
pub mod ecosystem;