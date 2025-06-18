use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use finalverse_health::HealthMonitor;
use service_registry::LocalServiceRegistry;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tracing::info;
use finalverse_logging as logging;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use mapleai_agent::Agent;
use finalverse_protocol::{BehaviorAction, ReasoningContext};

type Agents = Arc<RwLock<HashMap<String, Agent>>>;

#[derive(Clone)]
struct AppState {
    agents: Agents,
}

#[derive(Deserialize)]
struct SpawnRequest {
    id: String,
    region: String,
}

#[derive(Serialize)]
struct SpawnResponse {
    id: String,
}

async fn spawn_agent(
    State(state): State<AppState>,
    Json(req): Json<SpawnRequest>,
) -> Json<SpawnResponse> {
    let mut agents = state.agents.write().await;
    agents.insert(req.id.clone(), Agent::new(req.id.clone(), req.region));
    Json(SpawnResponse { id: req.id })
}

#[derive(Deserialize)]
struct ActRequest {
    location: String,
    nearby_entities: Vec<String>,
    harmony_level: f32,
    tension: f32,
    memory: Vec<String>,
}

#[derive(Serialize)]
struct ActResponse {
    action: ActionDto,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ActionDto {
    Wander,
    Rest,
    Flee { reason: String },
    Migrate { target_region: String },
    Interact { entity_id: String, action: String },
}

fn to_dto(action: BehaviorAction) -> ActionDto {
    match action {
        BehaviorAction::Wander => ActionDto::Wander,
        BehaviorAction::Rest => ActionDto::Rest,
        BehaviorAction::Flee(reason) => ActionDto::Flee { reason },
        BehaviorAction::Migrate { target_region } => ActionDto::Migrate { target_region },
        BehaviorAction::Interact { entity_id, action } => ActionDto::Interact { entity_id, action },
    }
}

async fn act_agent(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<ActRequest>,
) -> Option<Json<ActResponse>> {
    let mut agents = state.agents.write().await;
    let agent = agents.get_mut(&id)?;

    let ctx = ReasoningContext {
        location: req.location,
        nearby_entities: req.nearby_entities,
        harmony_level: req.harmony_level,
        tension: req.tension,
        memory: req.memory,
    };
    agent.update_context(ctx);
    agent.step().await;
    if let Some(action) = agent.state().last_action.clone() {
        Some(Json(ActResponse { action: to_dto(action) }))
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init(None);
    let monitor = Arc::new(HealthMonitor::new("behavior-ai", env!("CARGO_PKG_VERSION")));
    let registry = LocalServiceRegistry::new();
    registry
        .register_service("behavior-ai".to_string(), "http://localhost:3011".to_string())
        .await;

    let state = AppState {
        agents: Arc::new(RwLock::new(HashMap::new())),
    };
    let app = Router::new()
        .route("/agent/spawn", post(spawn_agent))
        .route("/agent/:id/act", post(act_agent))
        .with_state(state)
        .merge(monitor.clone().axum_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3011));
    info!("Behavior AI listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
