// crates/world-engine/src/bin/world-engine.rs
use std::sync::Arc;
use tokio::time::{interval, Duration};
use world_engine::{WorldEngine, Observer, WorldEvent, RegionState, RegionId, TerrainType, WeatherState, WeatherType, ecosystem::Species};

// Example observer for logging events
struct LoggingObserver;

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

    // Register observer
    engine.register_observer(Arc::new(LoggingObserver)).await;

    // Initialize some test data
    let test_region = RegionState {
        id: RegionId("terra-nova-central".to_string()),
        harmony_level: 0.8,
        discord_level: 0.2,
        terrain_type: TerrainType::Forest,
        weather: WeatherState {
            weather_type: WeatherType::Clear,
            intensity: 0.5,
            wind_direction: 45.0,
            wind_speed: 10.0,
        },
        active_events: Vec::new(),
    };

    engine.metabolism().add_region(test_region).await;

    // Add some species
    let star_deer = Species {
        id: "star-deer".to_string(),
        name: "Star-Horned Deer".to_string(),
        population: 150,
        migration_pattern: vec![
            RegionId("terra-nova-central".to_string()),
            RegionId("whispering-woods".to_string()),
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
    let routes = world_engine::create_routes(engine);

    println!("🚀 World Engine HTTP API starting on port 3002");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3002))
        .await;
}