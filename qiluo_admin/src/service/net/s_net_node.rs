use crate::common::error::{Error, Result};
use crate::common::result::ApiResponse;
use crate::common::validatedjson::VJson;
use crate::common::validatedquery::VQuery;
use crate::db::{DB_READ, DB_WRITE};
use crate::model::net::net_node;
use crate::model::prelude::PageParams;
use axum::response::IntoResponse;
use chrono::Utc;
use md5::Digest;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct RegisterNodeReq {
    pub name: String,
    pub ip_address: String,
    pub region: String,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterNodeRes {
    pub id: String,
    pub auth_token: String,
    pub name: String,
    pub ip_address: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct NodeHeartbeatReq {
    pub node_id: String,
    pub auth_token: String,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub active_jobs: Option<u64>,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct NodeIdReq {
    pub id: String,
}

pub async fn register_node(VJson(req): VJson<RegisterNodeReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let node_id = format!("node_{}", Utc::now().timestamp_millis());
    let raw_token = format!("tok_{}_{}", node_id, Utc::now().timestamp_micros());
    let token_hash = format!("{:x}", md5::Md5::digest(raw_token.as_bytes()));

    let capabilities_str = req
        .capabilities
        .map(|c| serde_json::to_string(&c).unwrap_or_else(|_| "[]".to_string()))
        .unwrap_or_else(|| "[\"ping\",\"traceroute\",\"dns\",\"portcheck\",\"iperf3\",\"udpecho\",\"tr143\"]".to_string());

    let new_node = net_node::ActiveModel {
        id: Set(node_id.clone()),
        name: Set(req.name.clone()),
        ip_address: Set(req.ip_address.clone()),
        region: Set(req.region.clone()),
        status: Set("online".to_string()),
        last_heartbeat: Set(Utc::now().naive_utc()),
        capabilities: Set(Some(capabilities_str)),
        auth_token_hash: Set(Some(token_hash)),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    if let Err(e) = new_node.insert(db).await {
        return ApiResponse::<Option<RegisterNodeRes>>::from_result(Err(Error::Message(format!(
            "Failed to register node: {:?}",
            e
        ))));
    }

    ApiResponse::ok(RegisterNodeRes {
        id: node_id,
        auth_token: raw_token,
        name: req.name,
        ip_address: req.ip_address,
        region: req.region,
    })
}

pub async fn node_heartbeat(VJson(req): VJson<NodeHeartbeatReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let node_opt = match net_node::Entity::find()
        .filter(net_node::Column::Id.eq(&req.node_id))
        .one(db)
        .await
    {
        Ok(n) => n,
        Err(e) => return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string()))),
    };

    if let Some(node) = node_opt {
        let expected_hash = format!("{:x}", md5::Md5::digest(req.auth_token.as_bytes()));
        if node.auth_token_hash.as_deref() != Some(&expected_hash) {
            return ApiResponse::<Option<String>>::from_result(Err(Error::Message(
                "Invalid node authentication token".to_string(),
            )));
        }

        let mut active: net_node::ActiveModel = node.into();
        active.status = Set("online".to_string());
        active.last_heartbeat = Set(Utc::now().naive_utc());
        active.updated_at = Set(Utc::now().naive_utc());

        if let Some(caps) = req.capabilities {
            active.capabilities = Set(Some(serde_json::to_string(&caps).unwrap_or_default()));
        }

        if let Err(e) = active.update(db).await {
            return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string())));
        }

        ApiResponse::ok(serde_json::json!({
            "status": "healthy",
            "server_time": Utc::now().to_rfc3339()
        }))
    } else {
        ApiResponse::<Option<String>>::from_result(Err(Error::Message("Node not found".to_string())))
    }
}

pub async fn list_nodes(VQuery(page_params): VQuery<PageParams>) -> impl IntoResponse {
    let db = DB_READ().await;
    let page = page_params.page_num.unwrap_or(1).max(1);
    let page_size = page_params.page_size.unwrap_or(20).min(100);

    let paginator = net_node::Entity::find()
        .order_by_desc(net_node::Column::CreatedAt)
        .paginate(db, page_size);

    let total = match paginator.num_items().await {
        Ok(t) => t,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    let list = match paginator.fetch_page(page - 1).await {
        Ok(l) => l,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    let now = Utc::now().naive_utc();
    let threshold_seconds = 45;

    let computed_list: Vec<serde_json::Value> = list
        .into_iter()
        .map(|node| {
            let is_online = (now - node.last_heartbeat).num_seconds() <= threshold_seconds;
            serde_json::json!({
                "id": node.id,
                "name": node.name,
                "ip_address": node.ip_address,
                "region": node.region,
                "status": if is_online { "online" } else { "offline" },
                "last_heartbeat": node.last_heartbeat,
                "capabilities": node.capabilities.and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok()).unwrap_or(serde_json::json!([])),
                "created_at": node.created_at
            })
        })
        .collect();

    ApiResponse::ok(serde_json::json!({
        "list": computed_list,
        "total": total,
        "page_num": page,
        "page_size": page_size
    }))
}

pub async fn delete_node(VJson(req): VJson<NodeIdReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let id = req.id;

    if let Err(e) = net_node::Entity::delete_many()
        .filter(net_node::Column::Id.eq(&id))
        .exec(db)
        .await
    {
        return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string())));
    }

    ApiResponse::ok("Node deleted")
}
