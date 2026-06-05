use crate::event_bus::{Event, EventBus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use tracing::info;
use ulid::Ulid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub priority: u8,           // 0 = highest
    pub description: String,
    pub status: String,         // pending, running, completed, failed
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(description: impl Into<String>, priority: u8) -> Self {
        Self {
            id: Ulid::new().to_string(),
            priority,
            description: description.into(),
            status: "pending".to_string(),
            created_at: Utc::now(),
        }
    }
}

// Simple wrapper for priority queue (BinaryHeap is max-heap, so we invert priority)
#[derive(Clone, Debug, Eq)]
struct PrioritizedTask {
    task: Task,
}

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower priority number = higher priority
        other.task.priority.cmp(&self.task.priority)
    }
}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PrioritizedTask {
    fn eq(&self, other: &Self) -> bool {
        self.task.id == other.task.id
    }
}

pub struct TaskScheduler {
    queue: BinaryHeap<PrioritizedTask>,
    event_bus: EventBus,
}

impl TaskScheduler {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            queue: BinaryHeap::new(),
            event_bus,
        }
    }

    pub async fn add_task(&mut self, task: Task) {
        info!("Adding task {} with priority {}", task.id, task.priority);
        self.queue.push(PrioritizedTask { task: task.clone() });
        self.event_bus
            .publish(Event::TaskCreated {
                task_id: task.id,
            })
            .await;
    }

    pub fn get_next_task(&mut self) -> Option<Task> {
        self.queue.pop().map(|pt| pt.task)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    // TODO: mark_completed, requeue, persistence, DAG dependencies
}
