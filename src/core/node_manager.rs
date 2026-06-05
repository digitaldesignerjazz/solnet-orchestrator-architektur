use crate::event_bus::{Event, EventBus};
use std::collections::HashMap;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub status: String,
    pub coords: Option<Vec<i64>>,
    pub public_key: Option<String>,
}

impl Node {
    pub fn new(id: String, status: String) -> Self {
        Self {
            id,
            status,
            coords: None,
            public_key: None,
        }
    }
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

    pub async fn register_or_update_node(&mut self, id: String, status: String) {
        if let Some(existing) = self.nodes.get_mut(&id) {
            existing.status = status;
        } else {
            let node = Node::new(id.clone(), status);
            self.nodes.insert(id.clone(), node.clone());
            self.event_bus
                .publish(Event::NodeJoined { node_id: id })
                .await;
        }
    }

    pub fn get_all_nodes(&self) -> Vec<Node> {
        self.nodes.values().cloned().collect()
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
}
