use solnet_orchestrator::event_bus::{Event, EventBus};
use solnet_orchestrator::core::node_manager::NodeManager;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting Solnet Orchestrator v0.1.0");

    // Initialize core components (skeleton)
    let event_bus = EventBus::new(1024);
    let node_manager = NodeManager::new(event_bus.clone());

    // Example: simulate node join
    // In real implementation this would come from Yggdrasil discovery
    info!("Core components initialized. Entering main loop...");

    // Placeholder main loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        info!("Orchestrator heartbeat - system nominal (skeleton mode)");
        // Future: process events, run scheduler, check health, etc.
    }
}
