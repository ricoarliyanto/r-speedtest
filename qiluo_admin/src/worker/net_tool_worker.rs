use crate::cache::CacheManager;
use crate::common::error::{Error, Result};
use crate::db::DB_WRITE;
use crate::model::net::{net_job, net_job_result};
use crate::tools::registry::{ToolContext, GLOBAL_TOOL_REGISTRY};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::process::Command;
use tracing::{error, info, warn};

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct NetToolJobMsg {
    pub job_id: String,
}

#[derive(Clone)]
pub struct NetToolWorker;

impl AppWorker<NetToolJobMsg> for NetToolWorker {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Worker<NetToolJobMsg> for NetToolWorker {
    fn opts() -> WorkerOpts<NetToolJobMsg, Self> {
        WorkerOpts::new().queue("net_tool")
    }

    async fn perform(&self, msg: NetToolJobMsg) -> Result<()> {
        let db = DB_WRITE().await;
        let job_id = msg.job_id.clone();

        info!("Starting execution of network job: {}", job_id);

        // 1. Fetch Job from database
        let job_opt = net_job::Entity::find()
            .filter(net_job::Column::Id.eq(&job_id))
            .one(db)
            .await
            .map_err(|e| Error::Message(format!("DB error fetching job {}: {:?}", job_id, e)))?;

        let job = match job_opt {
            Some(j) => j,
            None => {
                warn!("Job {} not found in database, skipping", job_id);
                return Ok(());
            }
        };

        // 2. Mark Job as Running
        let mut job_active: net_job::ActiveModel = job.clone().into();
        job_active.status = Set("running".to_string());
        job_active.started_at = Set(Some(Utc::now().naive_utc()));
        let _ = job_active.update(db).await;

        // 3. Resolve Tool from Registry
        let tool = match GLOBAL_TOOL_REGISTRY.get(&job.tool_id) {
            Some(t) => t,
            None => {
                error!("Tool {} not found in registry for job {}", job.tool_id, job_id);
                let mut failed_job: net_job::ActiveModel = job.into();
                failed_job.status = Set("failed".to_string());
                failed_job.finished_at = Set(Some(Utc::now().naive_utc()));
                let _ = failed_job.update(db).await;
                return Ok(());
            }
        };

        // 4. Parse Parameters & Build Args
        let raw_params: serde_json::Value = serde_json::from_str(&job.parameters)
            .unwrap_or_else(|_| serde_json::json!({}));

        let ctx = ToolContext {
            job_id: job_id.clone(),
            target: job.target_host.clone(),
            timeout_sec: tool.max_timeout(),
            raw_params,
        };

        let args = match tool.build_args(&ctx) {
            Ok(a) => a,
            Err(e) => {
                error!("Argument build error for job {}: {:?}", job_id, e);
                let mut failed_job: net_job::ActiveModel = job.into();
                failed_job.status = Set("failed".to_string());
                failed_job.finished_at = Set(Some(Utc::now().naive_utc()));
                let _ = failed_job.update(db).await;
                return Ok(());
            }
        };

        info!("Executing tool {} (binary: {}) with args: {:?}", tool.id(), tool.binary(), args);
        let start_time = Instant::now();

        let (stdout, stderr, exit_code) = if tool.binary() == "builtin:udpecho" {
            let port = ctx.raw_params.get("port").and_then(|v| v.as_u64()).unwrap_or(37001) as u16;
            let count = ctx.raw_params.get("count").and_then(|v| v.as_u64()).unwrap_or(50);
            let size = ctx.raw_params.get("size").and_then(|v| v.as_u64()).unwrap_or(64) as usize;
            let interval = ctx.raw_params.get("interval_ms").and_then(|v| v.as_u64()).unwrap_or(20);

            match crate::tools::udpecho::UdpEchoTool::execute_native_udp_test(
                &ctx.target, port, count, size, interval, 1000
            ).await {
                Ok(metrics_json) => (metrics_json.to_string(), String::new(), 0),
                Err(err) => (String::new(), format!("UDP Echo error: {:?}", err), 1),
            }
        } else if tool.binary() == "builtin:tr143" {
            let mode = ctx.raw_params.get("mode").and_then(|v| v.as_str()).unwrap_or("download");
            let connections = ctx.raw_params.get("connections").and_then(|v| v.as_u64()).unwrap_or(4) as usize;
            let size_mb = ctx.raw_params.get("size_mb").and_then(|v| v.as_u64()).unwrap_or(50);

            if mode.eq_ignore_ascii_case("upload") {
                match crate::tools::tr143::upload::execute_tr143_upload_client(
                    &ctx.target, size_mb, connections, tool.max_timeout()
                ).await {
                    Ok(metrics_json) => (metrics_json.to_string(), String::new(), 0),
                    Err(err) => (String::new(), format!("TR-143 Upload error: {:?}", err), 1),
                }
            } else {
                match crate::tools::tr143::download::execute_tr143_download_client(
                    &ctx.target, connections, tool.max_timeout()
                ).await {
                    Ok(metrics_json) => (metrics_json.to_string(), String::new(), 0),
                    Err(err) => (String::new(), format!("TR-143 Download error: {:?}", err), 1),
                }
            }
        } else {
            // 5. Execute external binary safely using Tokio Command (Zero arbitrary shell)
            match Command::new(tool.binary()).args(&args).output().await {
                Ok(output) => {
                    let out_s = String::from_utf8_lossy(&output.stdout).to_string();
                    let err_s = String::from_utf8_lossy(&output.stderr).to_string();
                    let code = output.status.code().unwrap_or(-1);
                    (out_s, err_s, code)
                }
                Err(err) => {
                    (String::new(), format!("Failed to spawn process: {:?}", err), -1)
                }
            }
        };

        // Ensure artifacts directory exists for tools generating file outputs
        if tool.id() == "tcpdump" {
            let _ = tokio::fs::create_dir_all(crate::tools::tcpdump::TcpDumpTool::get_pcap_dir()).await;
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        info!("Job {} completed with exit code: {}", job_id, exit_code);

        // Parse structured metrics
        let metrics = tool.parse_output(&stdout, &stderr)
            .unwrap_or_else(|_| serde_json::json!({ "raw": stdout }));

        let artifact_path = if tool.id() == "tcpdump" {
            Some(crate::tools::tcpdump::TcpDumpTool::get_pcap_path(&job_id))
        } else {
            None
        };

        // Insert Job Result
        let result_id = format!("res_{}", job_id);
        let new_result = net_job_result::ActiveModel {
            id: Set(result_id),
            job_id: Set(job_id.clone()),
            structured_metrics: Set(Some(metrics.to_string())),
            raw_output: Set(Some(stdout)),
            artifact_path: Set(artifact_path.clone()),
            exit_code: Set(Some(exit_code)),
            error_message: Set(if stderr.is_empty() { None } else { Some(stderr) }),
            created_at: Set(Utc::now().naive_utc()),
        };
        let _ = new_result.insert(db).await;

        // Update Job Status
        let mut completed_job: net_job::ActiveModel = job.into();
        completed_job.status = Set(if exit_code == 0 { "completed".to_string() } else { "failed".to_string() });
        completed_job.finished_at = Set(Some(Utc::now().naive_utc()));
        completed_job.duration_sec = Set(Some(elapsed));
        let _ = completed_job.update(db).await;

        // Publish Event to DragonflyDB Cache for WebSocket distribution
        let cache = CacheManager::instance().await;
        let event_payload = serde_json::json!({
            "event": "job_finished",
            "job_id": job_id,
            "status": if exit_code == 0 { "completed" } else { "failed" },
            "metrics": metrics,
            "duration_sec": elapsed
        });
        let _ = cache.set_string_ex(&format!("job_result:{}", job_id), &event_payload.to_string(), 3600).await;

        Ok(())
    }
}
