use crate::common::error::{Error, Result};
use crate::common::result::ApiResponse;
use crate::common::validatedjson::VJson;
use crate::common::validatedquery::VQuery;
use crate::db::{DB_READ, DB_WRITE};
use crate::model::net::{net_job, net_job_result};
use crate::model::prelude::PageParams;
use crate::tools::registry::GLOBAL_TOOL_REGISTRY;
use crate::worker::net_tool_worker::{NetToolJobMsg, NetToolWorker};
use crate::worker::AppWorker;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct RunToolReq {
    pub tool_id: String,
    pub target: String,
    pub node_id: Option<String>,
    pub scenario_id: Option<String>,
    pub params: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct JobIdReq {
    pub job_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunToolRes {
    pub job_id: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JobDetailRes {
    pub job: net_job::Model,
    pub result: Option<net_job_result::Model>,
}

pub async fn list_tools() -> impl IntoResponse {
    let metadata = GLOBAL_TOOL_REGISTRY.list_metadata();
    ApiResponse::ok(metadata)
}

pub async fn run_tool(VJson(req): VJson<RunToolReq>) -> impl IntoResponse {
    let tool_id = req.tool_id.clone();
    if GLOBAL_TOOL_REGISTRY.get(&tool_id).is_none() {
        return ApiResponse::<Option<RunToolRes>>::from_result(Err(Error::Message(format!(
            "Tool '{}' does not exist in registry",
            tool_id
        ))));
    }

    let db = DB_WRITE().await;
    let job_id = format!("job_{}_{}", tool_id, Utc::now().timestamp_millis());
    let params_str = req
        .params
        .map(|p| p.to_string())
        .unwrap_or_else(|| "{}".to_string());

    let new_job = net_job::ActiveModel {
        id: Set(job_id.clone()),
        tool_id: Set(tool_id),
        node_id: Set(req.node_id),
        scenario_id: Set(req.scenario_id),
        target_host: Set(req.target),
        parameters: Set(params_str),
        status: Set("queued".to_string()),
        requested_by: Set("admin".to_string()),
        created_at: Set(Utc::now().naive_utc()),
        started_at: Set(None),
        finished_at: Set(None),
        duration_sec: Set(None),
    };

    if let Err(e) = new_job.insert(db).await {
        return ApiResponse::<Option<RunToolRes>>::from_result(Err(Error::Message(format!(
            "Failed to create job: {:?}",
            e
        ))));
    }

    // Enqueue to DragonflyDB
    let _ = NetToolWorker::enqueue_async(NetToolJobMsg {
        job_id: job_id.clone(),
    })
    .await;

    ApiResponse::ok(RunToolRes {
        job_id,
        status: "queued".to_string(),
    })
}

pub async fn list_jobs(VQuery(page_params): VQuery<PageParams>) -> impl IntoResponse {
    let db = DB_READ().await;
    let page = page_params.page_num.unwrap_or(1).max(1);
    let page_size = page_params.page_size.unwrap_or(20).min(100);

    let paginator = net_job::Entity::find()
        .order_by_desc(net_job::Column::CreatedAt)
        .paginate(db, page_size);

    let total = match paginator.num_items().await {
        Ok(t) => t,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    let list = match paginator.fetch_page(page - 1).await {
        Ok(l) => l,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page_num": page,
        "page_size": page_size
    }))
}

pub async fn get_job(VQuery(req): VQuery<JobIdReq>) -> impl IntoResponse {
    let db = DB_READ().await;
    let id = req.job_id;

    let job = match net_job::Entity::find()
        .filter(net_job::Column::Id.eq(&id))
        .one(db)
        .await
    {
        Ok(Some(j)) => j,
        Ok(None) => {
            return ApiResponse::<Option<JobDetailRes>>::from_result(Err(Error::Message(format!("Job {} not found", id))))
        }
        Err(e) => return ApiResponse::<Option<JobDetailRes>>::from_result(Err(Error::Message(e.to_string()))),
    };

    let result = net_job_result::Entity::find()
        .filter(net_job_result::Column::JobId.eq(&id))
        .one(db)
        .await
        .ok()
        .flatten();

    ApiResponse::ok(JobDetailRes { job, result })
}

pub async fn cancel_job(VJson(req): VJson<JobIdReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let id = req.job_id;

    let job_opt = match net_job::Entity::find()
        .filter(net_job::Column::Id.eq(&id))
        .one(db)
        .await
    {
        Ok(j) => j,
        Err(e) => return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string()))),
    };

    if let Some(job) = job_opt {
        if job.status == "queued" || job.status == "running" {
            let mut active: net_job::ActiveModel = job.into();
            active.status = Set("cancelled".to_string());
            active.finished_at = Set(Some(Utc::now().naive_utc()));
            let _ = active.update(db).await;
        }
    }

    ApiResponse::ok("Job cancelled")
}

pub async fn download_artifact(VQuery(req): VQuery<JobIdReq>) -> impl IntoResponse {
    let db = DB_READ().await;
    let id = req.job_id;

    let result = match net_job_result::Entity::find()
        .filter(net_job_result::Column::JobId.eq(&id))
        .one(db)
        .await
    {
        Ok(Some(r)) => r,
        _ => return (StatusCode::NOT_FOUND, "Job result not found").into_response(),
    };

    let artifact_path = match result.artifact_path {
        Some(p) if !p.is_empty() => p,
        _ => return (StatusCode::NOT_FOUND, "No artifact associated with this job").into_response(),
    };

    let file_content = match tokio::fs::read(&artifact_path).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::NOT_FOUND, "Artifact file not found on disk").into_response(),
    };

    let filename = std::path::Path::new(&artifact_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("artifact.pcap");

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/vnd.tcpdump.pcap"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).unwrap(),
    );

    (StatusCode::OK, headers, file_content).into_response()
}
