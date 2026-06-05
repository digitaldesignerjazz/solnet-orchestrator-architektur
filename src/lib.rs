pub mod core;
pub mod event_bus;

// Re-exports for convenience
pub use event_bus::EventBus;
pub use core::node_manager::NodeManager;
