//harmony-service/src/main.rs
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tracing::info;
use finalverse_logging as logging;
use finalverse_events::{
    GameEventBus, LocalEventBus, NatsEventBus,
    Event, EventType, HarmonyEvent, ResonanceType, PlayerId,
    PlayerEvent, EventMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resonance {
    pub creative: f64,
    pub exploration: f64,
    pub restoration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgress {
    pub player_id: PlayerId,
    pub resonance: Resonance,
    pub attunement_tier: u32,
    pub unlocked_melodies: Vec<String>,
    pub unlocked_harmonies: Vec<String>,
}

pub struct HarmonyService {
    player_progress: Arc<RwLock<HashMap<PlayerId, PlayerProgress>>>,
    event_bus: Arc<dyn GameEventBus>,
    subscription_ids: Arc<RwLock<Vec<String>>>,
}

impl HarmonyService {
    pub fn new(event_bus: Arc<dyn GameEventBus>) -> Self {
        Self {
            player_progress: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
            subscription_ids: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start_event_listeners(&self) -> anyhow::Result<()> {
        // Subscribe to player events
        let progress = self.player_progress.clone();
        let player_sub_id = self
            .event_bus
            .subscribe("events.player", Box::new(move |event| {
                let progress = progress.clone();
                tokio::spawn(async move {
                    if let EventType::Player(player_event) = &event.event_type {
                    match player_event {
                        PlayerEvent::Connected { player_id } => {
                            info!("🎵 Player {} connected, initializing harmony data", player_id.0);
                            // Initialize player progress if needed
                            let mut progress_map = progress.write().await;
                            progress_map.entry(player_id.clone()).or_insert_with(|| {
                                PlayerProgress {
                                    player_id: player_id.clone(),
                                    resonance: Resonance {
                                        creative: 0.0,
                                        exploration: 0.0,
                                        restoration: 0.0,
                                    },
                                    attunement_tier: 0,
                                    unlocked_melodies: Vec::new(),
                                    unlocked_harmonies: Vec::new(),
                                }
                            });
                        }
                        PlayerEvent::Disconnected { player_id } => {
                            info!("👋 Player {} disconnected", player_id.0);
                        }
                        _ => {}
                    }
                }
            });
            }))
            .await?;

        self.subscription_ids.write().await.push(player_sub_id);

        // Subscribe to harmony events for logging
        let harmony_sub_id = self
            .event_bus
            .subscribe("events.harmony", Box::new(|event| {
                if let EventType::Harmony(harmony_event) = &event.event_type {
                    info!("🎼 Harmony Event: {:?}", harmony_event);
                }
            }))
            .await?;

        self.subscription_ids.write().await.push(harmony_sub_id);

        info!("✅ Harmony Service event listeners started");
        Ok(())
    }

    pub async fn add_resonance(&self, player_id: PlayerId, resonance_type: ResonanceType, amount: f64) -> anyhow::Result<()> {
        let mut progress_map = self.player_progress.write().await;

        let progress = progress_map.entry(player_id.clone()).or_insert_with(|| {
            PlayerProgress {
                player_id: player_id.clone(),
                resonance: Resonance {
                    creative: 0.0,
                    exploration: 0.0,
                    restoration: 0.0,
                },
                attunement_tier: 0,
                unlocked_melodies: Vec::new(),
                unlocked_harmonies: Vec::new(),
            }
        });

        // Update resonance
        match &resonance_type {
            ResonanceType::Creative => progress.resonance.creative += amount,
            ResonanceType::Exploration => progress.resonance.exploration += amount,
            ResonanceType::Restoration => progress.resonance.restoration += amount,
        }

        // Publish resonance gained event
        let event = Event::new(EventType::Harmony(HarmonyEvent::ResonanceGained {
            player_id: player_id.clone(),
            resonance_type: resonance_type.clone(),
            amount,
        })).with_metadata(EventMetadata {
            source: Some("harmony-service".to_string()),
            ..Default::default()
        });

        self.event_bus.publish(event).await?;

        // Check for attunement tier upgrade
        let total_resonance = progress.resonance.creative + progress.resonance.exploration + progress.resonance.restoration;
        let new_tier = (total_resonance / 100.0) as u32;

        if new_tier > progress.attunement_tier {
            let old_tier = progress.attunement_tier;
            progress.attunement_tier = new_tier;

            // Publish attunement achieved event
            let attunement_event = Event::new(EventType::Harmony(HarmonyEvent::AttunementAchieved {
                player_id: player_id.clone(),
                tier: new_tier,
                total_resonance,
            })).with_metadata(EventMetadata {
                source: Some("harmony-service".to_string()),
                ..Default::default()
            });

            self.event_bus.publish(attunement_event).await?;

            info!("⭐ Player {} achieved attunement tier {} (was {})", player_id.0, new_tier, old_tier);

            // Unlock new abilities based on tier
            self.unlock_tier_abilities(progress, new_tier).await?;
        }

        Ok(())
    }

    async fn unlock_tier_abilities(&self, progress: &mut PlayerProgress, tier: u32) -> anyhow::Result<()> {
        // Example melody unlocks
        let melodies = match tier {
            1 => vec![("Melody of Healing", 1), ("Melody of Light", 1)],
            2 => vec![("Melody of Discovery", 2), ("Melody of Growth", 2)],
            3 => vec![("Melody of Creation", 3), ("Melody of Harmony", 3)],
            4 => vec![("Melody of Transcendence", 4)],
            _ => vec![],
        };

        for (melody_name, required_tier) in melodies {
            if !progress.unlocked_melodies.contains(&melody_name.to_string()) {
                progress.unlocked_melodies.push(melody_name.to_string());

                let melody_event = Event::new(EventType::Harmony(HarmonyEvent::MelodyUnlocked {
                    player_id: progress.player_id.clone(),
                    melody: melody_name.to_string(),
                    tier_required: required_tier,
                })).with_metadata(EventMetadata {
                    source: Some("harmony-service".to_string()),
                    tags: vec!["ability_unlock".to_string(), format!("tier_{}", tier)],
                    ..Default::default()
                });

                self.event_bus.publish(melody_event).await?;
            }
        }

        // Example harmony unlocks
        let harmonies = match tier {
            2 => vec![("Harmony of Courage", 2), ("Harmony of Wisdom", 2)],
            3 => vec![("Harmony of Unity", 3)],
            4 => vec![("Harmony of Transcendence", 4), ("Harmony of Creation", 4)],
            5 => vec![("Harmony of the First Song", 5)],
            _ => vec![],
        };

        for (harmony_name, required_tier) in harmonies {
            if !progress.unlocked_harmonies.contains(&harmony_name.to_string()) {
                progress.unlocked_harmonies.push(harmony_name.to_string());

                let harmony_event = Event::new(EventType::Harmony(HarmonyEvent::HarmonyUnlocked {
                    player_id: progress.player_id.clone(),
                    harmony: harmony_name.to_string(),
                    tier_required: required_tier,
                })).with_metadata(EventMetadata {
                    source: Some("harmony-service".to_string()),
                    tags: vec!["ability_unlock".to_string(), format!("tier_{}", tier)],
                    ..Default::default()
                });

                self.event_bus.publish(harmony_event).await?;
            }
        }

        Ok(())
    }

    pub async fn get_progress(&self, player_id: &PlayerId) -> Option<PlayerProgress> {
        self.player_progress.read().await.get(player_id).cloned()
    }

    pub async fn shutdown(&self) -> anyhow::Result<()> {
        // Unsubscribe from all events
        let sub_ids = self.subscription_ids.read().await;
        for sub_id in sub_ids.iter() {
            self.event_bus.unsubscribe(sub_id).await?;
        }
        Ok(())
    }
}

// HTTP API handlers
async fn add_resonance_handler(
    player_id: String,
    resonance_type: String,
    amount: f64,
    service: Arc<HarmonyService>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resonance_type = match resonance_type.as_str() {
        "creative" => ResonanceType::Creative,
        "exploration" => ResonanceType::Exploration,
        "restoration" => ResonanceType::Restoration,
        _ => return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Invalid resonance type"})),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    };

    match service.add_resonance(PlayerId(player_id), resonance_type, amount).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"success": true})),
            warp::http::StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": e.to_string()})),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

async fn get_progress_handler(
    player_id: String,
    service: Arc<HarmonyService>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(progress) = service.get_progress(&PlayerId(player_id)).await {
        Ok(warp::reply::json(&progress))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "Player not found"})))
    }
}

async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&serde_json::json!({
        "status": "healthy",
        "service": "harmony-service",
        "version": env!("CARGO_PKG_VERSION"),
    })))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init(None);

    // Initialize event bus - use NATS if URL provided, otherwise use local
    let event_bus: Arc<dyn GameEventBus> = if let Ok(nats_url) = std::env::var("NATS_URL") {
        info!("📡 Connecting to NATS at {}", nats_url);
        Arc::new(NatsEventBus::new(&nats_url).await?)
    } else {
        info!("📦 Using local event bus (no NATS_URL provided)");
        Arc::new(LocalEventBus::new())
    };

    // Create service
    let service = Arc::new(HarmonyService::new(event_bus));

    // Start event listeners
    service.start_event_listeners().await?;

    // Define routes
    let service_clone = service.clone();
    let service_filter = warp::any().map(move || service_clone.clone());

    let add_resonance = warp::path!("resonance" / String / String / f64)
        .and(warp::post())
        .and(service_filter.clone())
        .and_then(add_resonance_handler);

    let get_progress = warp::path!("progress" / String)
        .and(warp::get())
        .and(service_filter.clone())
        .and_then(get_progress_handler);

    let health = warp::path!("health")
        .and(warp::get())
        .and_then(health_handler);

    let routes = add_resonance
        .or(get_progress)
        .or(health);

    // Handle shutdown gracefully
    let service_shutdown = service.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        info!("\n🛑 Shutting down Harmony Service...");
        let _ = service_shutdown.shutdown().await;
        std::process::exit(0);
    });

    info!("🎵 Harmony Service v{} starting on port 3006", env!("CARGO_PKG_VERSION"));
    info!("   Event bus: {}", if std::env::var("NATS_URL").is_ok() { "NATS" } else { "Local" });

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3006))
        .await;

    Ok(())
}

// services