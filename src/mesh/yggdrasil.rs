use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

use crate::core::NodeManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
pub struct YggdrasilSelf {
    pub address: Option<String>,
    pub coords: Option<Vec<i64>>,
    pub subnet: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YggdrasilPeer {
    pub address: Option<String>,
    pub coords: Option<Vec<i64>>,
    pub public_key: Option<String>,
    pub remote: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YggdrasilPeersResponse {
    pub peers: Option<Vec<YggdrasilPeer>>,
}

pub struct YggdrasilClient {
    client: Client,
    admin_url: String,
}

impl YggdrasilClient {
    pub fn new(admin_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            admin_url: admin_url.into(),
        }
    }

    pub async fn get_self(&self) -> Result<YggdrasilSelf, reqwest::Error> {
        let url = format!("{}/api/self", self.admin_url.trim_end_matches('/'));
        let resp = self.client.get(&url).send().await?;
        resp.json().await
    }

    pub async fn get_peers(&self) -> Result<Vec<YggdrasilPeer>, reqwest::Error> {
        let url = format!("{}/api/peers", self.admin_url.trim_end_matches('/'));
        let resp = self.client.get(&url).send().await?;
        let peers_resp: YggdrasilPeersResponse = resp.json().await?;
        Ok(peers_resp.peers.unwrap_or_default())
    }

    pub async fn sync_to_node_manager(&self, node_manager: Arc<Mutex<NodeManager>>) {
        if let Ok(self_info) = self.get_self().await {
            if let Some(addr) = &self_info.address {
                let mut nm = node_manager.lock().await;
                nm.register_or_update_node(addr.clone(), "online".to_string());
            }
        }

        match self.get_peers().await {
            Ok(peers) => {
                let mut nm = node_manager.lock().await;
                for peer in peers {
                    if let Some(addr) = peer.address {
                        let status = if peer.remote.unwrap_or(false) { "remote" } else { "connected" };
                        nm.register_or_update_node(addr, status.to_string());
                    }
                }
                info!("Synced Yggdrasil peers to NodeManager");
            }
            Err(e) => {
                warn!("Failed to fetch Yggdrasil peers: {}", e);
            }
        }
    }
}

pub fn default_client() -> YggdrasilClient {
    let url = std::env::var("SOLNET_YGGDRASIL_ADMIN")
        .unwrap_or_else(|_| "http://localhost:9001".to_string());
    YggdrasilClient::new(url)
}
