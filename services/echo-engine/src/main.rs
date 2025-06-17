// services/echo-engine/src/main.rs
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use finalverse_core::{
    echo::{Echo, EchoPersonality, EchoState},
    types::{EchoType, Coordinates as Position},
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use finalverse_logging as logging;

#[derive(Clone)]
struct AppState {
    echoes: Arc<Mutex<HashMap<Uuid, Echo>>>,
}

#[derive(Serialize, Deserialize)]
struct CreateEchoRequest {
    echo_type: EchoType,
    position: Position,
}

#[derive(Serialize, Deserialize)]
struct EchoResponse {
    id: Uuid,
    echo_type: EchoType,
    name: String,
    state: EchoState,
    position: Position,
}

impl From<&Echo> for EchoResponse {
    fn from(echo: &Echo) -> Self {
        EchoResponse {
            id: echo.id,
            echo_type: echo.echo_type,
            name: echo.name.clone(),
            state: echo.state.clone(),
            position: echo.position,
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    logging::init(Some("info"));

    let state = AppState {
        echoes: Arc::new(Mutex::new(HashMap::new())),
    };

    // Initialize the First Echoes
    initialize_first_echoes(&state);

    // Build our application with routes
    let app = Router::new()
        .route("/echoes", get(list_echoes))
        .route("/echoes", post(create_echo))
        .route("/echoes/:id", get(get_echo))
        .route("/echoes/:id/interact", post(interact_with_echo))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3004));
    info!("Echo Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app).await.unwrap();
}

fn initialize_first_echoes(state: &AppState) {
    let mut echoes = state.echoes.lock().unwrap();

    // Lumi - Echo of Hope and Discovery
    let lumi = Echo::new(
        EchoType::Lumi,
        "Lumi".to_string(),
        Position::new(0.0, 0.0, 0.0),
    );
    echoes.insert(lumi.id, lumi);
    info!("Initialized Lumi - Echo of Hope and Discovery");

    // KAI - Echo of Logic and Understanding
    let kai = Echo::new(
        EchoType::KAI,
        "KAI".to_string(),
        Position::new(100.0, 0.0, 0.0),
    );
    echoes.insert(kai.id, kai);
    info!("Initialized KAI - Echo of Logic and Understanding");

    // Terra - Echo of Resilience and Growth
    let terra = Echo::new(
        EchoType::Terra,
        "Terra".to_string(),
        Position::new(0.0, 100.0, 0.0),
    );
    echoes.insert(terra.id, terra);
    info!("Initialized Terra - Echo of Resilience and Growth");

    // Ignis - Echo of Courage and Creation
    let ignis = Echo::new(
        EchoType::Ignis,
        "Ignis".to_string(),
        Position::new(100.0, 100.0, 0.0),
    );
    echoes.insert(ignis.id, ignis);
    info!("Initialized Ignis - Echo of Courage and Creation");
}

async fn list_echoes(State(state): State<AppState>) -> Json<Vec<EchoResponse>> {
    let echoes = state.echoes.lock().unwrap();
    let responses: Vec<EchoResponse> = echoes.values().map(|e| e.into()).collect();
    Json(responses)
}

async fn create_echo(
    State(state): State<AppState>,
    Json(request): Json<CreateEchoRequest>,
) -> Json<EchoResponse> {
    let echo = Echo::new(
        request.echo_type,
        format!("{:?}", request.echo_type),
        request.position,
    );

    let response = EchoResponse::from(&echo);

    let mut echoes = state.echoes.lock().unwrap();
    echoes.insert(echo.id, echo);

    Json(response)
}

async fn get_echo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Option<EchoResponse>> {
    let echoes = state.echoes.lock().unwrap();
    Json(echoes.get(&id).map(|e| e.into()))
}

async fn interact_with_echo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<String> {
    let echoes = state.echoes.lock().unwrap();

    if let Some(echo) = echoes.get(&id) {
        match echo.echo_type {
            EchoType::Lumi => Json("Lumi's light brightens, filling you with hope!".to_string()),
            EchoType::KAI => Json("KAI analyzes the situation, revealing hidden patterns.".to_string()),
            EchoType::Terra => Json("Terra's presence strengthens your resolve.".to_string()),
            EchoType::Ignis => Json("Ignis ignites your courage!".to_string()),
        }
    } else {
        Json("Echo not found".to_string())
    }
}
