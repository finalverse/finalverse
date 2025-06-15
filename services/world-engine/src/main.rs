// crates/world-engine/src/bin/world-engine.rs
use std::sync::Arc;
use tokio::time::{interval, Duration};
use world_engine::{
    WorldEngine, Observer, WorldEvent, RegionState, RegionId, TerrainType,
    WeatherState, WeatherType, Species, SpeciesProfile, MigrationPhase,
};
use finalverse_audio_core::{AudioEvent, AudioEventType, AudioSource};
use nalgebra::Vector3;
use redis::Client as RedisClient;
use uuid::Uuid;
use chrono::Utc;
use serde_json;

// Example observer for logging events
struct LoggingObserver;

struct AudioObserver {
    redis_client: RedisClient,
}

#[async_trait::async_trait]
impl Observer for LoggingObserver {
    async fn notify(&self, event: &WorldEvent) {
        match event {
            WorldEvent::CreatureMigration { species, from, to } => {
                println!("🦌 {} migrating from {} to {}", species, from.0, to.0);
            }
            WorldEvent::CelestialEvent { event_type, duration } => {
                println!("✨ Celestial event: {:?} for {} seconds", event_type, duration);
            }
            WorldEvent::SilenceOutbreak { epicenter, radius, intensity } => {
                println!("🌑 Silence outbreak at ({:.2}, {:.2}, {:.2}), radius: {:.2}, intensity: {:.2}",
                         epicenter.x, epicenter.y, epicenter.z, radius, intensity);
            },
            &WorldEvent::HarmonyRestored { .. } | &WorldEvent::SilenceManifested { .. } | &WorldEvent::EchoAppeared { .. } => todo!()
        }
    }
}

#[async_trait::async_trait]
impl Observer for AudioObserver {
    async fn notify(&self, event: &WorldEvent) {
        let audio_event_opt = match event {
            WorldEvent::CelestialEvent { event_type, .. } => Some(AudioEvent {
                id: uuid::Uuid::new_v4(),
                event_type: AudioEventType::CelestialEvent { event_name: format!("{:?}", event_type) },
                position: None,
                source: AudioSource::World,
                timestamp: chrono::Utc::now().timestamp(),
            }),
            WorldEvent::SilenceOutbreak { epicenter, intensity, .. } => Some(AudioEvent {
                id: uuid::Uuid::new_v4(),
                event_type: AudioEventType::AmbientTrigger { trigger_id: "silence_outbreak".to_string(), intensity: *intensity as f32 },
                position: Some(Vector3::new(epicenter.x as f32, epicenter.y as f32, epicenter.z as f32)),
                source: AudioSource::Environment("silence".to_string()),
                timestamp: chrono::Utc::now().timestamp(),
            }),
            _ => None,
        };

        if let Some(audio_event) = audio_event_opt {
            if let Ok(mut con) = self.redis_client.get_async_connection().await {
                if let Ok(event_json) = serde_json::to_string(&audio_event) {
                    let _ : Result<(), _> = redis::cmd("PUBLISH")
                        .arg("world:events")
                        .arg(event_json)
                        .query_async(&mut con)
                        .await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("🌍 Starting World Engine...");

    // Create world engine
    let engine = Arc::new(WorldEngine::new());

    // Register observers
    engine.register_observer(Arc::new(LoggingObserver)).await;
    let redis_client = RedisClient::open("redis://127.0.0.1/").unwrap();
    engine.register_observer(Arc::new(AudioObserver { redis_client })).await;

    // Initialize some tests data
    let test_region = RegionState {
        id: RegionId(Uuid::new_v4()),
        harmony_level: 0.8,
        discord_level: 0.2,
        terrain_type: TerrainType::Forest,
        weather: WeatherState {
            weather_type: WeatherType::Clear,
            intensity: 0.5,
            wind_direction: 45.0,
            wind_speed: 10.0,
        },
    };

    engine.metabolism().add_region(test_region).await;

    // Add some species
    let star_deer = SpeciesProfile {
        id: "star-deer".to_string(),
        name: "Star-Horned Deer".to_string(),
        species: Species::StarHornedStag {
            herd_size: 3,
            migration_phase: MigrationPhase::Resting,
        },
        population: 150,
        migration_pattern: vec![
            RegionId(Uuid::new_v4()),
            RegionId(Uuid::new_v4()),
        ],
        preferred_terrain: vec![TerrainType::Forest, TerrainType::Plains],
    };

    engine.ecosystem().add_species(star_deer).await;

    // Start simulation loop
    let engine_sim = engine.clone();
    tokio::spawn(async move {
        let mut tick_interval = interval(Duration::from_secs(10));

        loop {
            tick_interval.tick().await;
            println!("⏰ Running world simulation tick...");
            engine_sim.simulate_tick().await;
        }
    });

    // Start HTTP server
    let routes = world_engine::server::create_routes(engine);

    println!("🚀 World Engine HTTP API starting on port 3002");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3002))
        .await;
}