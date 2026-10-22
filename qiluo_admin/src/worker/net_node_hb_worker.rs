use async_trait::async_trait;
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::cache::CacheManager;
use crate::common::error::Result;
use crate::db::DB_WRITE;
use crate::model::net::net_node;
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NetNodeHbMsg {
    pub check_stale: bool,
}

#[derive(Clone)]
pub struct NetNodeHbWorker;

impl AppWorker<NetNodeHbMsg> for NetNodeHbWorker {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Worker<NetNodeHbMsg> for NetNodeHbWorker {
    fn opts() -> WorkerOpts<NetNodeHbMsg, Self> {
        WorkerOpts::new().queue("net_node_hb")
    }

    async fn perform(&self, _msg: NetNodeHbMsg) -> Result<()> {
        let db = DB_WRITE().await;
        let stale_threshold = Utc::now().naive_utc() - Duration::seconds(45);

        // Find nodes whose last_heartbeat is older than 45 seconds and currently marked online
        let stale_nodes = match net_node::Entity::find()
            .filter(net_node::Column::LastHeartbeat.lt(stale_threshold))
            .filter(net_node::Column::Status.eq("online"))
            .all(db)
            .await
        {
            Ok(nodes) => nodes,
            Err(e) => {
                error!("Failed to query stale nodes: {:?}", e);
                return Ok(());
            }
        };

        for node in stale_nodes {
            let node_id = node.id.clone();
            let mut active: net_node::ActiveModel = node.into();
            active.status = Set("offline".to_string());
            active.updated_at = Set(Utc::now().naive_utc());

            if let Ok(_) = active.update(db).await {
                info!("Marked stale edge node {} as offline", node_id);

                // Publish Node Status Change to DragonflyDB cache
                let cache = CacheManager::instance().await;
                let payload = serde_json::json!({
                    "event": "node_offline",
                    "node_id": node_id,
                    "timestamp": Utc::now().to_rfc3339()
                });
                let _ = cache.set_string_ex(&format!("node_status:{}", node_id), &payload.to_string(), 3600).await;
            }
        }

        Ok(())
    }
}
