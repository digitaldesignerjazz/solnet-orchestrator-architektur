use tokio::sync::broadcast;
use tracing::info;

#[derive(Clone, Debug)]
pub enum Event {
    NodeJoined { node_id: String },
    Heartbeat { node_id: String },
    TaskCreated { task_id: String },
    TaskCompleted { task_id: String },
    // Extend with more event types as architecture evolves
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub async fn publish(&self, event: Event) {
        // Ignore send errors if no receivers
        let _ = self.sender.send(event.clone());
        info!("Event published: {:?}", event);
    }
}
