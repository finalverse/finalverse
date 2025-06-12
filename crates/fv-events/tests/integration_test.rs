// crates/fv-events/tests/integration_test.rs
use fv_events::*;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_local_event_bus() {
    let event_bus = LocalEventBus::new();
    let received_count = Arc::new(AtomicU32::new(0));

    // Subscribe to player events
    let count_clone = received_count.clone();
    let sub_id = event_bus
        .subscribe(
            "events.player",
            Box::new(move |event| {
                println!("Received event: {:?}", event);
                count_clone.fetch_add(1, Ordering::SeqCst);
            }),
        )
        .await
        .unwrap();

    // Publish some events
    let player_id = PlayerId("test_player".to_string());

    let event1 = Event::new(EventType::Player(PlayerEvent::Connected {
        player_id: player_id.clone(),
    }));
    event_bus.publish(event1).await.unwrap();

    let event2 = Event::new(EventType::Player(PlayerEvent::LevelUp {
        player_id: player_id.clone(),
        new_level: 5,
    }));
    event_bus.publish(event2).await.unwrap();

    // Give time for events to be processed
    sleep(Duration::from_millis(100)).await;

    assert_eq!(received_count.load(Ordering::SeqCst), 2);

    // Unsubscribe
    event_bus.unsubscribe(&sub_id).await.unwrap();

    // Publish another event - should not be received
    let event3 = Event::new(EventType::Player(PlayerEvent::Disconnected {
        player_id,
    }));
    event_bus.publish(event3).await.unwrap();

    sleep(Duration::from_millis(100)).await;
    assert_eq!(received_count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_harmony_song_integration() {
    // This test demonstrates how harmony and song services interact through events
    let event_bus = Arc::new(LocalEventBus::new());

    let mut harmony_events = Vec::new();
    let mut song_events = Vec::new();

    // Subscribe to harmony events
    let harmony_events_clone = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let harmony_clone = harmony_events_clone.clone();
    event_bus
        .subscribe("events.harmony", Box::new(move |event| {
            if let EventType::Harmony(harmony_event) = &event.event_type {
                let harmony_clone = harmony_clone.clone();
                tokio::spawn(async move {
                    harmony_clone.lock().await.push(format!("{:?}", harmony_event));
                });
            }
        }))
        .await
        .unwrap();

    // Subscribe to song events
    let song_events_clone = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let song_clone = song_events_clone.clone();
    event_bus
        .subscribe("events.song", Box::new(move |event| {
            if let EventType::Song(song_event) = &event.event_type {
                let song_clone = song_clone.clone();
                tokio::spawn(async move {
                    song_clone.lock().await.push(format!("{:?}", song_event));
                });
            }
        }))
        .await
        .unwrap();

    // Simulate player gaining resonance
    let player_id = PlayerId("harmony_test_player".to_string());

    // Player gains resonance
    let resonance_event = Event::new(EventType::Harmony(HarmonyEvent::ResonanceGained {
        player_id: player_id.clone(),
        resonance_type: ResonanceType::Creative,
        amount: 150.0,
    }));
    event_bus.publish(resonance_event).await.unwrap();

    // Player achieves attunement
    let attunement_event = Event::new(EventType::Harmony(HarmonyEvent::AttunementAchieved {
        player_id: player_id.clone(),
        tier: 3,
        total_resonance: 300.0,
    }));
    event_bus.publish(attunement_event).await.unwrap();

    // High-tier player weaves a song
    let song_event = Event::new(EventType::Song(SongEvent::SongWoven {
        weaver_id: player_id.clone(),
        song_type: SongType::Protection,
        power: 30.0,
        location: Coordinates { x: 100.0, y: 50.0, z: 0.0 },
    }));
    event_bus.publish(song_event).await.unwrap();

    // Give time for events to be processed
    sleep(Duration::from_millis(200)).await;

    // Check events were received
    assert_eq!(harmony_events_clone.lock().await.len(), 2);
    assert_eq!(song_events_clone.lock().await.len(), 1);
}

#[tokio::test]
async fn test_event_metadata() {
    let event_bus = LocalEventBus::new();

    // Create an event with full metadata
    let event = Event::new(EventType::System(SystemEvent::ServiceStarted {
        service_name: "test-service".to_string(),
    })).with_metadata(EventMetadata {
        source: Some("test-suite".to_string()),
        correlation_id: Some("correlation-123".to_string()),
        causation_id: Some("cause-456".to_string()),
        tags: vec!["test".to_string(), "integration".to_string()],
    });

    // Subscribe and verify metadata
    let received_event = Arc::new(tokio::sync::Mutex::new(None));
    let received_clone = received_event.clone();

    event_bus
        .subscribe(
            "events.system",
            Box::new(move |event| {
                let received_clone = received_clone.clone();
                tokio::spawn(async move {
                    *received_clone.lock().await = Some(event);
                });
            }),
        )
        .await
        .unwrap();

    event_bus.publish(event.clone()).await.unwrap();
    sleep(Duration::from_millis(100)).await;

    let received = received_event.lock().await;
    assert!(received.is_some());

    let received_event = received.as_ref().unwrap();
    assert_eq!(received_event.metadata.source, Some("test-suite".to_string()));
    assert_eq!(received_event.metadata.correlation_id, Some("correlation-123".to_string()));
    assert_eq!(received_event.metadata.tags.len(), 2);
}

// Example of how to create a mock event bus for testing
pub struct MockEventBus {
    published_events: Arc<tokio::sync::Mutex<Vec<Event>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn get_published_events(&self) -> Vec<Event> {
        self.published_events.lock().await.clone()
    }
}

#[async_trait::async_trait]
impl GameEventBus for MockEventBus {
    async fn publish_raw(&self, _topic: &str, _payload: Vec<u8>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn publish(&self, event: Event) -> anyhow::Result<()> {
        self.published_events.lock().await.push(event);
        Ok(())
    }

    async fn subscribe_raw(
        &self,
        _topic: &str,
        _handler: Box<dyn Fn(Vec<u8>) + Send + Sync + 'static>,
    ) -> anyhow::Result<String> {
        Ok("mock-subscription".to_string())
    }

    async fn unsubscribe(&self, _subscription_id: &str) -> anyhow::Result<()> {
        Ok(())
    }
}