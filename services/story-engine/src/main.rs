// services/song-engine/src/main.rs
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use warp::Filter;
use fv_events::{
    GameEventBus, LocalEventBus, NatsEventBus,
    Event, EventType, SongEvent, SongType, PlayerId, Coordinates,
    HarmonyEvent, EventMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSong {
    pub id: String,
    pub weaver_id: PlayerId,
    pub song_type: SongType,
    pub power: f64,
    pub location: Coordinates,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub duration: u64, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symphony {
    pub id: String,
    pub symphony_type: String,
    pub participants: Vec<PlayerId>,
    pub required_power: f64,
    pub current_power: f64,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: SymphonyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymphonyStatus {
    Gathering,
    InProgress,
    Completed,
    Failed,
}

pub struct SongEngineService {
    active_songs: Arc<RwLock<HashMap<String, ActiveSong>>>,
    symphonies: Arc<RwLock<HashMap<String, Symphony>>>,
    event_bus: Arc<dyn GameEventBus>,
    subscription_ids: Arc<RwLock<Vec<String>>>,
}

impl SongEngineService {
    pub fn new(event_bus: Arc<dyn GameEventBus>) -> Self {
        Self {
            active_songs: Arc::new(RwLock::new(HashMap::new())),
            symphonies: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
            subscription_ids: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start_event_listeners(&self) -> anyhow::Result<()> {
        // Listen for harmony events to trigger automatic songs
        let songs = self.active_songs.clone();
        let event_bus = self.event_bus.clone();

        let harmony_sub_id = self.event_bus.subscribe("events.harmony", move |event| {
            let songs = songs.clone();
            let event_bus = event_bus.clone();

            tokio::spawn(async move {
                if let EventType::Harmony(harmony_event) = &event.event_type {
                    match harmony_event {
                        HarmonyEvent::AttunementAchieved { player_id, tier, .. } => {
                            if *tier >= 3 {
                                // High-tier players automatically create ambient songs
                                println!("🎵 Player {} achieved tier {}, creating ambient song", player_id.0, tier);

                                let song = ActiveSong {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    weaver_id: player_id.clone(),
                                    song_type: SongType::Protection,
                                    power: *tier as f64 * 10.0,
                                    location: Coordinates { x: 0.0, y: 0.0, z: 0.0 }, // Would get from player location
                                    started_at: chrono::Utc::now(),
                                    duration: 300, // 5 minutes
                                };

                                songs.write().await.insert(song.id.clone(), song.clone());

                                // Publish song woven event
                                let song_event = Event::new(EventType::Song(SongEvent::SongWoven {
                                    weaver_id: player_id.clone(),
                                    song_type: SongType::Protection,
                                    power: song.power,
                                    location: song.location,
                                })).with_metadata(EventMetadata {
                                    source: Some("song-engine".to_string()),
                                    causation_id: Some(event.id.clone()),
                                    ..Default::default()
                                });

                                let _ = event_bus.publish(song_event).await;
                            }
                        }
                        _ => {}
                    }
                }
            });
        }).await?;

        self.subscription_ids.write().await.push(harmony_sub_id);

        // Start cleanup task for expired songs
        let songs = self.active_songs.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

            loop {
                interval.tick().await;
                let now = chrono::Utc::now();
                let mut expired_songs = Vec::new();

                {
                    let songs_map = songs.read().await;
                    for (id, song) in songs_map.iter() {
                        let elapsed = (now - song.started_at).num_seconds() as u64;
                        if elapsed >= song.duration {
                            expired_songs.push(id.clone());
                        }
                    }
                }

                for id in expired_songs {
                    songs.write().await.remove(&id);
                    println!("🎵 Song {} expired and removed", id);
                }
            }
        });

        println!("✅ Song Engine event listeners started");
        Ok(())
    }

    pub async fn weave_song(
        &self,
        weaver_id: PlayerId,
        song_type: SongType,
        power: f64,
        location: Coordinates,
    ) -> anyhow::Result<String> {
        let song = ActiveSong {
            id: uuid::Uuid::new_v4().to_string(),
            weaver_id: weaver_id.clone(),
            song_type: song_type.clone(),
            power,
            location: location.clone(),
            started_at: chrono::Utc::now(),
            duration: match &song_type {
                SongType::Healing => 60,      // 1 minute
                SongType::Creation => 300,    // 5 minutes
                SongType::Protection => 600,  // 10 minutes
                SongType::Discovery => 120,   // 2 minutes
                SongType::Destruction => 30,  // 30 seconds
            },
        };

        let song_id = song.id.clone();
        self.active_songs.write().await.insert(song_id.clone(), song);

        // Publish song woven event
        let event = Event::new(EventType::Song(SongEvent::SongWoven {
            weaver_id,
            song_type,
            power,
            location,
        })).with_metadata(EventMetadata {
            source: Some("song-engine".to_string()),
            tags: vec!["player_action".to_string()],
            ..Default::default()
        });

        self.event_bus.publish(event).await?;

        Ok(song_id)
    }

    pub async fn start_symphony(
        &self,
        symphony_type: String,
        initiator: PlayerId,
        required_power: f64,
    ) -> anyhow::Result<String> {
        let symphony = Symphony {
            id: uuid::Uuid::new_v4().to_string(),
            symphony_type: symphony_type.clone(),
            participants: vec![initiator.clone()],
            required_power,
            current_power: 0.0,
            started_at: chrono::Utc::now(),
            status: SymphonyStatus::Gathering,
        };

        let symphony_id = symphony.id.clone();
        self.symphonies.write().await.insert(symphony_id.clone(), symphony);

        // Publish symphony started event
        let event = Event::new(EventType::Song(SongEvent::SymphonyStarted {
            participants: vec![initiator],
            symphony_type,
            required_power,
        })).with_metadata(EventMetadata {
            source: Some("song-engine".to_string()),
            correlation_id: Some(symphony_id.clone()),
            ..Default::default()
        });

        self.event_bus.publish(event).await?;

        Ok(symphony_id)
    }

    pub async fn join_symphony(
        &self,
        symphony_id: &str,
        player_id: PlayerId,
        contributed_power: f64,
    ) -> anyhow::Result<()> {
        let mut symphonies = self.symphonies.write().await;

        if let Some(symphony) = symphonies.get_mut(symphony_id) {
            if !symphony.participants.contains(&player_id) {
                symphony.participants.push(player_id);
            }

            symphony.current_power += contributed_power;

            // Check if symphony is ready to complete
            if symphony.current_power >= symphony.required_power && symphony.status == SymphonyStatus::Gathering {
                symphony.status = SymphonyStatus::InProgress;

                // Simulate symphony completion after some time
                let symphony_id = symphony_id.to_string();
                let participants = symphony.participants.clone();
                let symphony_type = symphony.symphony_type.clone();
                let event_bus = self.event_bus.clone();
                let symphonies_clone = self.symphonies.clone();

                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

                    // Complete the symphony
                    if let Some(symphony) = symphonies_clone.write().await.get_mut(&symphony_id) {
                        symphony.status = SymphonyStatus::Completed;
                    }

                    // Publish completion event
                    let event = Event::new(EventType::Song(SongEvent::SymphonyCompleted {
                        participants,
                        symphony_type,
                        success: true,
                    })).with_metadata(EventMetadata {
                        source: Some("song-engine".to_string()),
                        correlation_id: Some(symphony_id),
                        ..Default::default()
                    });

                    let _ = event_bus.publish(event).await;
                });
            }
        }

        Ok(())
    }

    pub async fn get_active_songs(&self) -> Vec<ActiveSong> {
        self.active_songs.read().await.values().cloned().collect()
    }

    pub async fn get_symphonies(&self) -> Vec<Symphony> {
        self.symphonies.read().await.values().cloned().collect()
    }

    pub async fn shutdown(&self) -> anyhow::Result<()> {
        let sub_ids = self.subscription_ids.read().await;
        for sub_id in sub_ids.iter() {
            self.event_bus.unsubscribe(sub_id).await?;
        }
        Ok(())
    }
}

// HTTP handlers
async fn weave_song_handler(
    body: WeaveRequest,
    service: Arc<SongEngineService>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match service.weave_song(
        PlayerId(body.player_id),
        body.song_type,
        body.power,
        body.location,
    ).await {
        Ok(song_id) => Ok(warp::reply::json(&serde_json::json!({
            "success": true,
            "song_id": song_id,
        }))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({
            "error": e.to_string(),
        }))),
    }
}

async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&serde_json::json!({
        "status": "healthy",
        "service": "song-engine",
        "version": env!("CARGO_PKG_VERSION"),
    })))
}

#[derive(Deserialize)]
struct WeaveRequest {
    player_id: String,
    song_type: SongType,
    power: f64,
    location: Coordinates,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Initialize event bus
    let event_bus: Arc<dyn GameEventBus> = if let Ok(nats_url) = std::env::var("NATS_URL") {
        println!("📡 Connecting to NATS at {}", nats_url);
        Arc::new(NatsEventBus::new(&nats_url).await?)
    } else {
        println!("📦 Using local event bus");
        Arc::new(LocalEventBus::new())
    };

    // Create service
    let service = Arc::new(SongEngineService::new(event_bus));

    // Start event listeners
    service.start_event_listeners().await?;

    // Define routes
    let service_filter = warp::any().map(move || service.clone());

    let weave_song = warp::path!("song" / "weave")
        .and(warp::post())
        .and(warp::body::json())
        .and(service_filter.clone())
        .and_then(weave_song_handler);

    let get_songs = warp::path!("songs")
        .and(warp::get())
        .and(service_filter.clone())
        .map(|service: Arc<SongEngineService>| {
            let service = service.clone();
            async move {
                let songs = service.get_active_songs().await;
                warp::reply::json(&songs)
            }
        });

    let health = warp::path!("health")
        .and(warp::get())
        .and_then(health_handler);

    let routes = weave_song
        .or(get_songs)
        .or(health);

    // Handle shutdown
    let service_shutdown = service.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("\n🛑 Shutting down Song Engine...");
        let _ = service_shutdown.shutdown().await;
        std::process::exit(0);
    });

    println!("🎵 Song Engine v{} starting on port 3001", env!("CARGO_PKG_VERSION"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3001))
        .await;

    Ok(())
}

// Add uuid to dependencies
// uuid = { version = "1.0", features = ["v4", "serde"] }