use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardKeyPair {
    pub private_key: String, // Base64 encoded
    pub public_key: String,  // Base64 encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardPeerConfig {
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardInterfaceConfig {
    pub private_key: String,
    pub address: String, // e.g. "10.200.0.2/24"
    pub listen_port: Option<u16>,
    pub peers: Vec<WireGuardPeerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelRequest {
    pub source_node_id: String,
    pub target_node_id: String,
    pub allowed_ips: Option<Vec<String>>,
}
