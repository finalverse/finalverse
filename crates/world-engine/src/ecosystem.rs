//! Ecosystem definitions for the World Engine.

use fv_common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ecosystem {
    pub region_id: RegionId,
    pub creatures: HashMap<CreatureId, Creature>,
    pub flora: HashMap<FloraId, Flora>,
    pub harmony_level: f32,
    pub biodiversity_index: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatureId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FloraId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    pub id: CreatureId,
    pub species: Species,
    pub position: Coordinates,
    pub health: f32,
    pub behavior_state: BehaviorState,
    pub migration_target: Option<Coordinates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Species {
    StarHornedStag {
        herd_size: u32,
        migration_phase: MigrationPhase,
    },
    StormSalamander {
        electric_charge: f32,
    },
    MelodyBird {
        song_complexity: u32,
    },
    GrottoTurtle {
        moss_growth: f32,
        sleeping: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationPhase {
    Resting,
    Preparing,
    Migrating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorState {
    Foraging,
    Resting,
    Migrating,
    Interacting,
    Fleeing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flora {
    pub id: FloraId,
    pub flora_type: FloraType,
    pub position: Coordinates,
    pub growth_stage: f32,
    pub harmony_influence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloraType {
    ResonantBlossom {
        bloom_state: bool,
        light_intensity: f32,
    },
    SunKissedMoss {
        coverage_area: f32,
    },
    WhisperTree {
        age: u32,
        memory_fragments: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    DissonanceStorm,
}

impl Ecosystem {
    pub fn new(region_id: RegionId) -> Self {
        let mut creatures = HashMap::new();
        let mut flora = HashMap::new();
        
        // Spawn initial creatures
        for i in 0..5 {
            let creature = Creature {
                id: CreatureId(Uuid::new_v4()),
                species: Species::StarHornedStag {
                    herd_size: 3,
                    migration_phase: MigrationPhase::Resting,
                },
                position: Coordinates {
                    x: (i as f64) * 100.0,
                    y: 50.0,
                    z: (i as f64) * 50.0,
                },
                health: 100.0,
                behavior_state: BehaviorState::Foraging,
                migration_target: None,
            };
            creatures.insert(creature.id.clone(), creature);
        }
        
        // Spawn initial flora
        for i in 0..10 {
            let flora_item = Flora {
                id: FloraId(Uuid::new_v4()),
                flora_type: FloraType::ResonantBlossom {
                    bloom_state: true,
                    light_intensity: 0.8,
                },
                position: Coordinates {
                    x: (i as f64) * 80.0,
                    y: 0.0,
                    z: (i as f64) * 60.0,
                },
                growth_stage: 1.0,
                harmony_influence: 0.1,
            };
            flora.insert(flora_item.id.clone(), flora_item);
        }
        
        Self {
            region_id,
            creatures,
            flora,
            harmony_level: 75.0,
            biodiversity_index: 0.8,
        }
    }
    
    pub fn update(&mut self, delta_time: f64, weather: &Weather) {
        // Update creatures
        let creatures_to_update: Vec<_> = self.creatures.keys().cloned().collect();
        for creature_id in creatures_to_update {
            if let Some(creature) = self.creatures.get_mut(&creature_id) {
                Self::update_creature(creature, delta_time, self.harmony_level, weather);
            }
        }
        
        // Update flora
        for flora in self.flora.values_mut() {
            Self::update_flora(flora, delta_time, self.harmony_level, weather);
        }
        
        // Update biodiversity based on population
        self.biodiversity_index = (self.creatures.len() as f32 * 0.1 + self.flora.len() as f32 * 0.05)
            .min(1.0)
            .max(0.1);
        
        // Harmony affects ecosystem health
        if self.harmony_level < 30.0 {
            // Low harmony causes ecosystem degradation
            self.biodiversity_index *= 0.99;
        } else if self.harmony_level > 80.0 {
            // High harmony promotes growth
            self.biodiversity_index = (self.biodiversity_index * 1.01).min(1.0);
        }
    }
    
    fn update_creature(creature: &mut Creature, delta_time: f64, harmony: f32, weather: &Weather) {
        match &mut creature.species {
            Species::StarHornedStag { migration_phase, .. } => {
                // Migration logic
                match migration_phase {
                    MigrationPhase::Resting => {
                        if rand::random::<f32>() < 0.001 {
                            *migration_phase = MigrationPhase::Preparing;
                            creature.migration_target = Some(Coordinates {
                                x: creature.position.x + 1000.0,
                                y: creature.position.y,
                                z: creature.position.z + 500.0,
                            });
                        }
                    }
                    MigrationPhase::Preparing => {
                        creature.behavior_state = BehaviorState::Migrating;
                        *migration_phase = MigrationPhase::Migrating;
                    }
                    MigrationPhase::Migrating => {
                        if let Some(target) = &creature.migration_target {
                            let dx = target.x - creature.position.x;
                            let dz = target.z - creature.position.z;
                            let distance = (dx * dx + dz * dz).sqrt();
                            
                            if distance > 10.0 {
                                let speed = 50.0 * delta_time;
                                creature.position.x += (dx / distance) * speed;
                                creature.position.z += (dz / distance) * speed;
                            } else {
                                *migration_phase = MigrationPhase::Resting;
                                creature.behavior_state = BehaviorState::Foraging;
                                creature.migration_target = None;
                            }
                        }
                    }
                }
            }
            Species::StormSalamander { electric_charge } => {
                // Charge builds during storms
                if matches!(weather, Weather::Storm | Weather::DissonanceStorm) {
                    *electric_charge = (*electric_charge + 0.1 * delta_time as f32).min(1.0);
                } else {
                    *electric_charge = (*electric_charge - 0.05 * delta_time as f32).max(0.0);
                }
            }
            Species::MelodyBird { song_complexity } => {
                // Song complexity increases with harmony
                if harmony > 70.0 && rand::random::<f32>() < 0.01 {
                    *song_complexity = (*song_complexity + 1).min(10);
                }
            }
            Species::GrottoTurtle { sleeping, moss_growth } => {
                if *sleeping {
                    *moss_growth = (*moss_growth + 0.01 * delta_time as f32).min(1.0);
                    if rand::random::<f32>() < 0.001 {
                        *sleeping = false;
                    }
                } else {
                    if rand::random::<f32>() < 0.002 {
                        *sleeping = true;
                    }
                }
            }
        }
        
        // General health updates
        creature.health = (creature.health + harmony * 0.001 * delta_time as f32).min(100.0);
    }
    
    fn update_flora(flora: &mut Flora, delta_time: f64, harmony: f32, weather: &Weather) {
        match &mut flora.flora_type {
            FloraType::ResonantBlossom { bloom_state, light_intensity } => {
                if harmony > 60.0 {
                    *bloom_state = true;
                    *light_intensity = (*light_intensity + 0.1 * delta_time as f32).min(1.0);
                } else {
                    *light_intensity = (*light_intensity - 0.05 * delta_time as f32).max(0.0);
                    if *light_intensity < 0.1 {
                        *bloom_state = false;
                    }
                }
            }
            FloraType::SunKissedMoss { coverage_area } => {
                let growth_rate = if matches!(weather, Weather::Clear) { 0.02 } else { 0.01 };
                *coverage_area = (*coverage_area + growth_rate * delta_time as f32).min(10.0);
            }
            FloraType::WhisperTree { age, .. } => {
                // Trees age slowly
                if rand::random::<f32>() < 0.0001 {
                    *age += 1;
                }
            }
        }
        
        // Update harmony influence
        flora.harmony_influence = flora.growth_stage * 0.1 * (harmony / 100.0);
    }
} 