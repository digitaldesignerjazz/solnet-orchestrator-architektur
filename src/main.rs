use axum::serve;
use solnet_orchestrator::api::server::{create_router, AppState};
use solnet_orchestrator::core::{NodeManager, Task, TaskScheduler};
use solnet_orchestrator::event_bus::EventBus;
use solnet_orchestrator::mesh::yggdrasil::default_client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Initialize structured logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("\n=== Starting Solnet Orchestrator v0.1.0 ===");
    info!("Hybrid control plane for NovaNet / xMesh / QNET + AI Swarms + Hardware");

    // === Core Components ===
    let event_bus = EventBus::new(1024);
    let node_manager = Arc::new(Mutex::new(NodeManager::new(event_bus.clone())));
    let task_scheduler = Arc::new(Mutex::new(TaskScheduler::new(event_bus.clone())));

    // === Yggdrasil Client (real integration) ===
    let ygg_client = default_client();
    // Initial sync (non-blocking)
    let nm_clone = node_manager.clone();
    tokio::spawn(async move {
        ygg_client.sync_self_to_node_manager(nm_clone).await;
    });

    // === Example Task ===
    {
        let mut sched = task_scheduler.lock().await;
        sched
            .add_task(Task::new(
                "Initial Soilnova telemetry analysis",
                2, // high priority
            ))
            .await;
    }

    // === Build AppState for API ===
    let state = AppState {
        node_manager: node_manager.clone(),
        task_scheduler: task_scheduler.clone(),
        event_bus: event_bus.clone(),
    };

    // === Start REST + WebSocket API Server ===
    let app = create_router(state).await;
    let addr = "0.0.0.0:8080";
    info!("REST + WebSocket API listening on http://{}/", addr);
    info!("  - GET  /health");
    info!("  - GET  /nodes          (for egui dashboard)");
    info!("  - GET  /tasks");
    info!("  - POST /tasks          (create task from dashboard)");
    info!("  - WS   /ws/events      (real-time event stream for egui)");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Run server + core heartbeat loop concurrently
    tokio::select! {
        _ = serve(listener, app.into_make_service()) => {
            info!("API server stopped");
        }
        _ = core_heartbeat_loop(node_manager, task_scheduler, event_bus, ygg_client) => {
            info!("Core loop stopped");
        }
    }
}

/// Background core loop with real Yggdrasil sync
async fn core_heartbeat_loop(
    node_manager: Arc<Mutex<NodeManager>>,
    task_scheduler: Arc<Mutex<TaskScheduler>>,
    event_bus: EventBus,
    ygg_client: solnet_orchestrator::mesh::yggdrasil::YggdrasilClient,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
    loop {
        interval.tick().await;

        // === Real Yggdrasil Discovery + Heartbeat ===
        let nm_clone = node_manager.clone();
        ygg_client.sync_self_to_node_manager(nm_clone).await;

        // Process next high-priority task (skeleton)
        {
            let mut sched = task_scheduler.lock().await;
            if let Some(task) = sched.get_next_task() {
                info!("Processing high-priority task: {} - {}", task.id, task.description);
                // TODO: assign to nodes/agents, update status, publish Event
            }
        }

        info!("Orchestrator heartbeat - system nominal");
    }
}
