use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::extract::ws::{Message, WebSocket};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};

use crate::core::{NodeManager, Task, TaskScheduler};
use crate::event_bus::EventBus;

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub node_manager: Arc<Mutex<NodeManager>>,
    pub task_scheduler: Arc<Mutex<TaskScheduler>>,
    pub event_bus: EventBus,
}

pub async fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/nodes", get(list_nodes))
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/ws/events", get(ws_events_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // Allow egui / browser dashboard connections
}

async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "solnet-orchestrator" }))
}

async fn list_nodes(State(state): State<AppState>) -> impl IntoResponse {
    let manager = state.node_manager.lock().await;
    // For skeleton we return empty or sample; real impl would query manager
    Json(json!({ "nodes": [] }))
}

async fn list_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let scheduler = state.task_scheduler.lock().await;
    Json(json!({ "tasks_count": scheduler.len() }))
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed task")
        .to_string();
    let priority = payload.get("priority").and_then(|v| v.as_u64()).unwrap_or(5) as u8;

    let task = Task::new(description, priority);
    {
        let mut scheduler = state.task_scheduler.lock().await;
        scheduler.add_task(task.clone()).await;
    }
    Json(json!({ "status": "created", "task_id": task.id }))
}

// WebSocket handler for real-time events (perfect for egui Dashboard)
async fn ws_events_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.event_bus.clone()))
}

async fn handle_socket(mut socket: WebSocket, event_bus: EventBus) {
    let mut rx = event_bus.subscribe();
    info!("New WebSocket client connected to /ws/events");

    loop {
        tokio::select! {
            // Forward events from bus to client
            event = rx.recv() => {
                match event {
                    Ok(ev) => {
                        if let Ok(text) = serde_json::to_string(&ev) {
                            if socket.send(Message::Text(text)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            // Or receive messages from client (e.g. commands from dashboard)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        info!("WS received from dashboard: {}", text);
                        // TODO: parse commands (e.g. "spawn_agent", "approve_task")
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
    warn!("WebSocket client disconnected");
}
