use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;
use tokio::process::Command;
use tracing::{error, info};

use crate::cache::CacheManager;
use crate::common::error::Result;
use crate::db::DB_WRITE;
use crate::model::net::{net_job, net_job_result, net_scenario};
use crate::tools::registry::{GLOBAL_TOOL_REGISTRY, ToolContext};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NetScenarioJobMsg {
    pub execution_id: String,
    pub scenario_id: String,
    pub target: String,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioStep {
    pub step_id: String,
    pub tool_id: String,
    pub target: Option<String>,
    pub params: Option<serde_json::Value>,
    pub on_failure: Option<String>, // "abort" or "continue"
}

#[derive(Clone)]
pub struct NetScenarioWorker;

impl AppWorker<NetScenarioJobMsg> for NetScenarioWorker {
    fn new() -> Self {
        Self
    }
}

impl NetScenarioWorker {
    fn calculate_health_score(step_results: &[serde_json::Value]) -> u32 {
        let mut score: f64 = 100.0;

        for res in step_results {
            let tool = res.get("tool_id").and_then(|v| v.as_str()).unwrap_or("");
            let is_success = res.get("is_success").and_then(|v| v.as_bool()).unwrap_or(true);
            if !is_success {
                score -= 25.0;
                continue;
            }

            let metrics = res.get("metrics");
            if tool == "ping" {
                if let Some(loss) = metrics.and_then(|m| m.get("packet_loss_percent")).and_then(|v| v.as_f64()) {
                    score -= loss * 0.5; // Up to 50 pts deduction for loss
                }
                if let Some(avg_rtt) = metrics.and_then(|m| m.get("avg_rtt_ms")).and_then(|v| v.as_f64()) {
                    if avg_rtt > 100.0 {
                        score -= ((avg_rtt - 100.0) / 20.0).min(20.0);
                    }
                }
            } else if tool == "iperf3" || tool == "tr143" {
                if let Some(mbps) = metrics.and_then(|m| m.get("throughput_mbps").or_else(|| m.get("avg_recv_mbps"))).and_then(|v| v.as_f64()) {
                    if mbps < 10.0 {
                        score -= 20.0;
                    } else if mbps < 50.0 {
                        score -= 10.0;
                    }
                }
            }
        }

        score.max(0.0).min(100.0).round() as u32
    }
}

#[async_trait]
impl Worker<NetScenarioJobMsg> for NetScenarioWorker {
    fn opts() -> WorkerOpts<NetScenarioJobMsg, Self> {
        WorkerOpts::new().queue("net_scenario")
    }

    async fn perform(&self, msg: NetScenarioJobMsg) -> Result<()> {
        let execution_id = msg.execution_id.clone();
        info!("Executing Diagnostic Scenario: {}", execution_id);

        let db = DB_WRITE().await;

        // 1. Fetch Scenario Definition
        let scenario = match net_scenario::Entity::find()
            .filter(net_scenario::Column::Id.eq(&msg.scenario_id))
            .one(db)
            .await
        {
            Ok(Some(s)) => s,
            _ => {
                error!("Scenario {} not found in database", msg.scenario_id);
                return Ok(());
            }
        };

        // 2. Parse Execution Flow
        let steps: Vec<ScenarioStep> = serde_json::from_str(&scenario.execution_flow).unwrap_or_default();
        if steps.is_empty() {
            error!("Scenario {} has no executable steps", scenario.id);
            return Ok(());
        }

        let mut step_summaries = Vec::new();
        let mut scenario_failed = false;
        let start_time = Instant::now();

        // 3. Execute Steps
        for (index, step) in steps.iter().enumerate() {
            if scenario_failed && scenario.stop_on_failure {
                info!("Skipping step {} due to previous failure", step.step_id);
                break;
            }

            let tool_opt = GLOBAL_TOOL_REGISTRY.get(&step.tool_id);
            if tool_opt.is_none() {
                error!("Tool {} in step {} does not exist in registry", step.tool_id, step.step_id);
                continue;
            }
            let tool = tool_opt.unwrap();

            let target = step.target.clone().unwrap_or_else(|| msg.target.clone());
            let step_job_id = format!("{}_{}_{}", execution_id, index + 1, step.tool_id);
            let raw_params = step.params.clone().unwrap_or_else(|| json!({}));

            // Create step job record
            let new_job = net_job::ActiveModel {
                id: Set(step_job_id.clone()),
                tool_id: Set(step.tool_id.clone()),
                node_id: Set(msg.node_id.clone()),
                scenario_id: Set(Some(scenario.id.clone())),
                target_host: Set(target.clone()),
                parameters: Set(raw_params.to_string()),
                status: Set("running".to_string()),
                requested_by: Set("scenario_engine".to_string()),
                created_at: Set(Utc::now().naive_utc()),
                started_at: Set(Some(Utc::now().naive_utc())),
                finished_at: Set(None),
                duration_sec: Set(None),
            };
            let _ = new_job.insert(db).await;

            let ctx = ToolContext {
                job_id: step_job_id.clone(),
                target: target.clone(),
                timeout_sec: tool.max_timeout(),
                raw_params: raw_params.clone(),
            };

            let step_start = Instant::now();
            let args_res = tool.build_args(&ctx);

            let (stdout, stderr, exit_code) = match args_res {
                Ok(args) => {
                    if tool.binary() == "builtin:udpecho" {
                        let port = raw_params.get("port").and_then(|v| v.as_u64()).unwrap_or(37001) as u16;
                        let count = raw_params.get("count").and_then(|v| v.as_u64()).unwrap_or(50);
                        let size = raw_params.get("size").and_then(|v| v.as_u64()).unwrap_or(64) as usize;
                        let interval = raw_params.get("interval_ms").and_then(|v| v.as_u64()).unwrap_or(20);

                        match crate::tools::udpecho::UdpEchoTool::execute_native_udp_test(
                            &ctx.target, port, count, size, interval, 1000
                        ).await {
                            Ok(res) => (res.to_string(), String::new(), 0),
                            Err(e) => (String::new(), format!("UDP Echo error: {:?}", e), 1),
                        }
                    } else if tool.binary() == "builtin:tr143" {
                        let mode = raw_params.get("mode").and_then(|v| v.as_str()).unwrap_or("download");
                        let connections = raw_params.get("connections").and_then(|v| v.as_u64()).unwrap_or(4) as usize;
                        let size_mb = raw_params.get("size_mb").and_then(|v| v.as_u64()).unwrap_or(50);

                        if mode.eq_ignore_ascii_case("upload") {
                            match crate::tools::tr143::upload::execute_tr143_upload_client(
                                &ctx.target, size_mb, connections, tool.max_timeout()
                            ).await {
                                Ok(res) => (res.to_string(), String::new(), 0),
                                Err(e) => (String::new(), format!("TR-143 Upload error: {:?}", e), 1),
                            }
                        } else {
                            match crate::tools::tr143::download::execute_tr143_download_client(
                                &ctx.target, connections, tool.max_timeout()
                            ).await {
                                Ok(res) => (res.to_string(), String::new(), 0),
                                Err(e) => (String::new(), format!("TR-143 Download error: {:?}", e), 1),
                            }
                        }
                    } else {
                        if tool.id() == "tcpdump" {
                            let _ = tokio::fs::create_dir_all(crate::tools::tcpdump::TcpDumpTool::get_pcap_dir()).await;
                        }
                        match Command::new(tool.binary()).args(&args).output().await {
                            Ok(output) => {
                                let out_s = String::from_utf8_lossy(&output.stdout).to_string();
                                let err_s = String::from_utf8_lossy(&output.stderr).to_string();
                                let code = output.status.code().unwrap_or(-1);
                                (out_s, err_s, code)
                            }
                            Err(e) => (String::new(), format!("Execution error: {:?}", e), -1),
                        }
                    }
                }
                Err(e) => (String::new(), format!("Arg error: {:?}", e), -1),
            };

            let step_elapsed = step_start.elapsed().as_secs_f64();
            let metrics = tool.parse_output(&stdout, &stderr)
                .unwrap_or_else(|_| json!({ "raw": stdout }));

            let artifact_path = if tool.id() == "tcpdump" {
                Some(crate::tools::tcpdump::TcpDumpTool::get_pcap_path(&step_job_id))
            } else {
                None
            };

            // Save step result
            let result_id = format!("res_{}", step_job_id);
            let new_result = net_job_result::ActiveModel {
                id: Set(result_id),
                job_id: Set(step_job_id.clone()),
                structured_metrics: Set(Some(metrics.to_string())),
                raw_output: Set(Some(stdout)),
                artifact_path: Set(artifact_path),
                exit_code: Set(Some(exit_code)),
                error_message: Set(if stderr.is_empty() { None } else { Some(stderr) }),
                created_at: Set(Utc::now().naive_utc()),
            };
            let _ = new_result.insert(db).await;

            let is_step_success = exit_code == 0;
            if !is_step_success {
                let on_fail = step.on_failure.as_deref().unwrap_or("continue");
                if on_fail == "abort" {
                    scenario_failed = true;
                }
            }

            step_summaries.push(json!({
                "step_id": step.step_id,
                "tool_id": step.tool_id,
                "job_id": step_job_id,
                "is_success": is_step_success,
                "metrics": metrics,
                "duration_sec": step_elapsed
            }));
        }

        let total_duration = start_time.elapsed().as_secs_f64();
        let health_score = Self::calculate_health_score(&step_summaries);

        // 4. Publish Scenario Completion Event to Cache / WebSocket
        let cache = CacheManager::instance().await;
        let event_payload = json!({
            "event": "scenario_finished",
            "execution_id": execution_id,
            "scenario_id": scenario.id,
            "scenario_name": scenario.name,
            "health_score": health_score,
            "steps": step_summaries,
            "total_duration_sec": (total_duration * 100.0).round() / 100.0,
            "is_success": !scenario_failed
        });

        let _ = cache.set_string_ex(
            &format!("scenario_result:{}", execution_id),
            &event_payload.to_string(),
            3600,
        ).await;

        info!("Scenario {} completed with health score: {}/100", execution_id, health_score);
        Ok(())
    }
}
