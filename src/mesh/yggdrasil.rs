use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

use crate::core::NodeManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct YggdrasilSelf {
    pub address: Option<String>,
    pub coords: Option<Vec<i64>>,
    pub subnet: Option<String>,
    pub public_key: Option<String>,
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

    /// Fetch basic self information from Yggdrasil admin API
    pub async fn get_self(&self) -> Result<YggdrasilSelf, reqwest::Error> {
        let url = format!("{}/api/self", self.admin_url.trim_end_matches('/'));
        info!("Querying Yggdrasil admin: {}", url);

        let resp = self.client.get(&url).send().await?;
        let self_info: YggdrasilSelf = resp.json().await?;
        Ok(self_info)
    }

    /// Register or update this node's information in the NodeManager
    pub async fn sync_self_to_node_manager(
        &self,
        node_manager: Arc<Mutex<NodeManager>>,
    ) {
        match self.get_self().await {
            Ok(self_info) => {
                let node_id = self_info
                    .address
                    .unwrap_or_else(|| "unknown-ygg".to_string());

                let mut nm = node_manager.lock().await;
                // In a real implementation we would update existing node or create new
                nm.register_node(crate::core::Node {
                    id: node_id,
                    status: "online".to_string(),
                })
                .await;

                info!("Synced Yggdrasil self info to NodeManager");
            }
            Err(e) => {
                warn!("Failed to query Yggdrasil admin API: {}. Using fallback.", e);
            }
        }
    }
}

/// Helper to create client from environment or default
pub fn default_client() -> YggdrasilClient {
    let url = std::env::var("SOLNET_YGGDRASIL_ADMIN")
        .unwrap_or_else(|_| "http://localhost:9001".to_string());
    YggdrasilClient::new(url)
}
