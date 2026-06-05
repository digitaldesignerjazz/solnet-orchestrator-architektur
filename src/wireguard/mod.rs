pub mod keys;
pub mod types;

// Re-exports
pub use keys::generate_keypair;
pub use types::{WireGuardKeyPair, WireGuardPeerConfig, WireGuardInterfaceConfig};
