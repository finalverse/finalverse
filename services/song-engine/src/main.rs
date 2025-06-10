use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use finalverse_common::{
    events::{SongEvent, HarmonyEvent},
    types::{Coordinates, Melody, PlayerId, RegionId, HarmonyType, Note},
    FinalverseError, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use finalverse_health::HealthMonitor;
use finalverse_service_registry::LocalServiceRegistry;

#[derive(Debug, Clone)]
pub struct SongEngineState {
    global_harmony: f32,
    regional_harmony: HashMap<RegionId, f32>,
    active_melodies: HashMap<String, Melody>,
    silence_corruption: HashMap<RegionId, f32>,
}

type SharedSongState = Arc<RwLock<SongEngineState>>;

#[derive(Serialize)]
struct ServiceInfo {
    name: String,
    version: String,
    status: String,
    global_harmony: f32,
}

#[derive(Deserialize)]
struct PerformMelodyRequest {
    player_id: String,
    melody: MelodyRequest,
    target_location: CoordinatesRequest,
}

#[derive(Deserialize)]
struct MelodyRequest {
    notes: Vec<NoteRequest>,
    tempo: f32,
    harmony_type: String,
}

#[derive(Deserialize)]
struct NoteRequest {
    frequency: f32,
    duration: f32,
    intensity: f32,
}

#[derive(Deserialize)]
struct CoordinatesRequest {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize)]
struct PerformMelodyResponse {
    success: bool,
    resonance_gained: f32,
    harmony_impact: f32,
    message: String,
    effects: Vec<String>,
}

#[derive(Deserialize)]
struct HarmonyCheckRequest {
    region_id: String,
}

#[derive(Serialize)]
struct HarmonyCheckResponse {
    region_id: String,
    harmony_level: f32,
    corruption_level: f32,
    dominant_song_fragments: Vec<String>,
}

impl SongEngineState {
    pub fn new() -> Self {
        let mut regional_harmony = HashMap::new();
        regional_harmony.insert(RegionId("terra_nova".to_string()), 75.0);
        regional_harmony.insert(RegionId("aethelgard".to_string()), 45.0);
        regional_harmony.insert(RegionId("technos_prime".to_string()), 60.0);
        regional_harmony.insert(RegionId("whispering_wilds".to_string()), 80.0);
        regional_harmony.insert(RegionId("star_sailor_expanse".to_string()), 55.0);

        let mut silence_corruption = HashMap::new();
        silence_corruption.insert(RegionId("aethelgard".to_string()), 25.0);
        silence_corruption.insert(RegionId("technos_prime".to_string()), 15.0);

        Self {
            global_harmony: 65.0,
            regional_harmony,
            active_melodies: HashMap::new(),
            silence_corruption,
        }
    }

    pub fn perform_melody(&mut self, melody: Melody, location: Coordinates, player_id: PlayerId) -> PerformMelodyResponse {
        // Calculate melody power based on complexity and harmony
        let melody_power = self.calculate_melody_power(&melody);

        // Determine region from coordinates (simplified)
        let region = self.determine_region_from_coordinates(&location);

        // Apply harmony effects
        let harmony_impact = self.apply_harmony_effects(&region, melody_power, &melody.harmony_type);

        // Calculate resonance gained for the player
        let resonance_gained = melody_power * 2.0;

        // Generate effects based on harmony type and power
        let effects = self.generate_melody_effects(&melody.harmony_type, melody_power, &region);

        // Prepare message description before moving melody
        let harmony_desc = match melody.harmony_type {
            HarmonyType::Creative => "creative",
            HarmonyType::Restoration => "restorative",
            HarmonyType::Exploration => "exploratory",
            HarmonyType::Protection => "protective",
        };

        // Store the melody
        let melody_id = uuid::Uuid::new_v4().to_string();
        self.active_melodies.insert(melody_id, melody);

        PerformMelodyResponse {
            success: true,
            resonance_gained,
            harmony_impact,
            message: format!(
                "Your {} melody resonates through the Song of Creation!",
                harmony_desc
            ),
            effects,
        }
    }

    fn calculate_melody_power(&self, melody: &Melody) -> f32 {
        let base_power = melody.notes.len() as f32 * 0.5;
        let complexity_bonus = melody.notes.iter()
            .map(|note| note.intensity * note.duration / note.frequency.max(1.0))
            .sum::<f32>() / melody.notes.len() as f32;

        base_power + complexity_bonus.min(10.0)
    }

    fn determine_region_from_coordinates(&self, _coordinates: &Coordinates) -> RegionId {
        // Simplified region determination - in a real implementation,
        // this would use spatial indexing
        RegionId("terra_nova".to_string())
    }

    fn apply_harmony_effects(&mut self, region: &RegionId, power: f32, harmony_type: &HarmonyType) -> f32 {
        let current_harmony = self.regional_harmony.get(region).unwrap_or(&50.0);
        let harmony_modifier = match harmony_type {
            HarmonyType::Restoration => power * 1.5,
            HarmonyType::Creative => power * 1.2,
            HarmonyType::Protection => power * 1.0,
            HarmonyType::Exploration => power * 0.8,
        };

        let new_harmony = (current_harmony + harmony_modifier).min(100.0);
        self.regional_harmony.insert(region.clone(), new_harmony);

        // Update global harmony
        let avg_harmony: f32 = self.regional_harmony.values().sum::<f32>() / self.regional_harmony.len() as f32;
        self.global_harmony = avg_harmony;

        // Reduce silence corruption if present
        if let Some(corruption) = self.silence_corruption.get_mut(region) {
            *corruption = (*corruption - harmony_modifier * 0.5).max(0.0);
        }

        harmony_modifier
    }

    fn generate_melody_effects(&self, harmony_type: &HarmonyType, power: f32, region: &RegionId) -> Vec<String> {
        let mut effects = Vec::new();

        match harmony_type {
            HarmonyType::Creative => {
                effects.push("Flowers bloom in your wake".to_string());
                if power > 5.0 {
                    effects.push("A small crystal formation appears".to_string());
                }
            },
            HarmonyType::Restoration => {
                effects.push("Wounded creatures are healed nearby".to_string());
                if power > 7.0 {
                    effects.push("The corruption in this area diminishes".to_string());
                }
            },
            HarmonyType::Protection => {
                effects.push("A protective aura surrounds the area".to_string());
                if power > 6.0 {
                    effects.push("Barriers of light form to ward off the Silence".to_string());
                }
            },
            HarmonyType::Exploration => {
                effects.push("Hidden paths become visible".to_string());
                if power > 4.0 {
                    effects.push("Ancient runes glow, revealing secrets".to_string());
                }
            },
        }

        effects
    }
}


async fn perform_melody(
    State(state): State<SharedSongState>,
    Json(request): Json<PerformMelodyRequest>,
) -> impl IntoResponse {
    // Parse and validate player ID
    let player_uuid = match uuid::Uuid::parse_str(&request.player_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid player ID format"
        }))),
    };

    let player_id = PlayerId(player_uuid.to_string());

    // Convert request to internal types
    let harmony_type = match request.melody.harmony_type.as_str() {
        "creative" => HarmonyType::Creative,
        "restoration" => HarmonyType::Restoration,
        "exploration" => HarmonyType::Exploration,
        "protection" => HarmonyType::Protection,
        _ => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid harmony type"
        }))),
    };

    let notes: Vec<Note> = request.melody.notes.into_iter().map(|n| Note {
        frequency: n.frequency,
        duration: n.duration,
        intensity: n.intensity,
    }).collect();

    let melody = Melody {
        notes,
        tempo: request.melody.tempo,
        harmony_type,
    };

    let coordinates = Coordinates {
        x: request.target_location.x,
        y: request.target_location.y,
        z: request.target_location.z,
    };

    // Perform the melody
    let mut song_state = state.write().unwrap();
    let response = song_state.perform_melody(melody, coordinates, player_id);
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn check_harmony(
    State(state): State<SharedSongState>,
    Json(request): Json<HarmonyCheckRequest>,
) -> impl IntoResponse {
    let song_state = state.read().unwrap();
    let region_id = RegionId(request.region_id.clone());

    let harmony_level = song_state.regional_harmony
        .get(&region_id)
        .copied()
        .unwrap_or(50.0);

    let corruption_level = song_state.silence_corruption
        .get(&region_id)
        .copied()
        .unwrap_or(0.0);

    // Get dominant song fragments (simplified)
    let dominant_fragments: Vec<String> = song_state.active_melodies
        .keys()
        .take(3)
        .cloned()
        .collect();

    let response = HarmonyCheckResponse {
        region_id: request.region_id,
        harmony_level,
        corruption_level,
        dominant_song_fragments: dominant_fragments,
    };
    let json_response = serde_json::to_value(response).unwrap();

    (StatusCode::OK, Json(json_response))
}

async fn get_global_harmony(State(state): State<SharedSongState>) -> impl IntoResponse {
    let song_state = state.read().unwrap();

    (StatusCode::OK, Json(serde_json::json!({
        "global_harmony": song_state.global_harmony,
        "regional_harmony": song_state.regional_harmony,
        "active_melodies_count": song_state.active_melodies.len(),
        "corrupted_regions": song_state.silence_corruption.len()
    })))
}

async fn process_song_event(
    State(state): State<SharedSongState>,
    Json(event): Json<SongEvent>,
) -> impl IntoResponse {
    let mut song_state = state.write().unwrap();

    match event {
        SongEvent::MelodyWoven { player_id, melody, target } => {
            let response = song_state.perform_melody(melody, target, player_id);
            (StatusCode::OK, Json(serde_json::json!({
                "event_processed": true,
                "result": response
            })))
        },
        SongEvent::HarmonyAchieved { participants, harmony_type, power_level } => {
            // Process collaborative harmony achievement
            let bonus_harmony = power_level * participants.len() as f32 * 0.5;
            song_state.global_harmony = (song_state.global_harmony + bonus_harmony).min(100.0);

            (StatusCode::OK, Json(serde_json::json!({
                "event_processed": true,
                "participants": participants.len(),
                "global_harmony_bonus": bonus_harmony,
                "new_global_harmony": song_state.global_harmony
            })))
        },
        SongEvent::DissonanceDetected { location, intensity, source } => {
            // Handle dissonance detection
            let region = song_state.determine_region_from_coordinates(&location);
            if let Some(harmony) = song_state.regional_harmony.get_mut(&region) {
                *harmony = (*harmony - intensity).max(0.0);
            }

            (StatusCode::OK, Json(serde_json::json!({
                "event_processed": true,
                "dissonance_location": location,
                "intensity": intensity,
                "source": source
            })))
        },
        SongEvent::SilenceCorruption { region, corruption_level, affected_entities } => {
            // Handle silence corruption
            song_state.silence_corruption.insert(region.clone(), corruption_level);
            if let Some(harmony) = song_state.regional_harmony.get_mut(&region) {
                *harmony = (*harmony - corruption_level * 0.5).max(0.0);
            }

            (StatusCode::OK, Json(serde_json::json!({
                "event_processed": true,
                "region": region,
                "corruption_applied": corruption_level,
                "affected_entities_count": affected_entities.len()
            })))
        }
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(SongEngineState::new()));
    let monitor = Arc::new(HealthMonitor::new("song-engine", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("song-engine".to_string(), "http://localhost:3001".to_string())
        .await;

    let app = Router::new()
        .with_state(state.clone())
        .merge(monitor.clone().axum_routes())
        .route("/api/melody/perform", post(perform_melody))
        .route("/api/harmony/check", post(check_harmony))
        .route("/api/harmony/global", get(get_global_harmony))
        .route("/api/events", post(process_song_event))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Song Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}