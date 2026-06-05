use crate::event_bus::{Event, EventBus};
use std::collections::HashMap;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub status: String,
    // TODO: expand with capabilities, reputation, location, energy_profile etc.
}

pub struct NodeManager {
    nodes: HashMap<String, Node>,
    event_bus: EventBus,
}

impl NodeManager {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            nodes: HashMap::new(),
            event_bus,
        }
    }

    pub async fn register_node(&mut self, node: Node) {
        info!("Registering node: {}", node.id);
        self.nodes.insert(node.id.clone(), node.clone());
        self.event_bus
            .publish(Event::NodeJoined {
                node_id: node.id,
            })
            .await;
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    // TODO: heartbeat handling, health checks, decommissioning, quarantine logic
}
