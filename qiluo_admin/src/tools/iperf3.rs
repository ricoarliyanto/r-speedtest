use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct Iperf3Tool;

impl Iperf3Tool {
    pub fn new() -> Self {
        Self
    }

    fn sanitize_target(target: &str) -> Result<String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err(Error::Message("Target host cannot be empty".to_string()));
        }
        if trimmed.contains(|c: char| c.is_whitespace() || c == ';' || c == '&' || c == '|' || c == '`' || c == '$') {
            return Err(Error::Message("Target host contains invalid characters".to_string()));
        }
        Ok(trimmed.to_string())
    }
}

#[async_trait]
impl NetworkTool for Iperf3Tool {
    fn id(&self) -> &'static str {
        "iperf3"
    }

    fn name(&self) -> &'static str {
        "iPerf3 Throughput Tester"
    }

    fn category(&self) -> &'static str {
        "throughput"
    }

    fn binary(&self) -> &'static str {
        "/usr/bin/iperf3"
    }

    fn max_timeout(&self) -> u64 {
        3600
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let target = Self::sanitize_target(&ctx.target)?;
        let port = ctx.raw_params.get("port").and_then(|v| v.as_u64()).unwrap_or(5201);
        let duration = ctx.raw_params.get("duration").and_then(|v| v.as_u64()).unwrap_or(10).min(300);
        let parallel = ctx.raw_params.get("parallel").and_then(|v| v.as_u64()).unwrap_or(1).min(64);
        let is_reverse = ctx.raw_params.get("reverse").and_then(|v| v.as_bool()).unwrap_or(false);
        let protocol = ctx.raw_params.get("protocol").and_then(|v| v.as_str()).unwrap_or("tcp");

        let mut args = vec![
            "-c".to_string(),
            target,
            "-p".to_string(),
            port.to_string(),
            "-t".to_string(),
            duration.to_string(),
            "-P".to_string(),
            parallel.to_string(),
            "-J".to_string(), // Always request JSON format for reliable machine parsing
        ];

        if is_reverse {
            args.push("-R".to_string());
        }

        if protocol.eq_ignore_ascii_case("udp") {
            args.push("-u".to_string());
            if let Some(bw) = ctx.raw_params.get("bandwidth").and_then(|v| v.as_str()) {
                let sanitized_bw: String = bw.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
                if !sanitized_bw.is_empty() {
                    args.push("-b".to_string());
                    args.push(sanitized_bw);
                }
            } else {
                args.push("-b".to_string());
                args.push("1G".to_string());
            }
        }

        Ok(args)
    }

    fn parse_output(&self, stdout: &str, stderr: &str) -> Result<serde_json::Value> {
        // Attempt to parse JSON output
        if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(stdout) {
            let mut intervals_data = Vec::new();

            if let Some(intervals) = parsed_json.get("intervals").and_then(|v| v.as_array()) {
                for interval in intervals {
                    if let Some(sum) = interval.get("sum") {
                        let mbps = sum.get("bits_per_second")
                            .and_then(|v| v.as_f64())
                            .map(|bps| bps / 1_000_000.0)
                            .unwrap_or(0.0);
                        let bytes = sum.get("bytes").and_then(|v| v.as_u64()).unwrap_or(0);
                        let seconds = sum.get("seconds").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let retransmits = sum.get("retransmits").and_then(|v| v.as_u64()).unwrap_or(0);

                        intervals_data.push(json!({
                            "mbps": (mbps * 100.0).round() / 100.0,
                            "bytes": bytes,
                            "seconds": seconds,
                            "retransmits": retransmits,
                        }));
                    }
                }
            }

            let end_summary = parsed_json.get("end").cloned().unwrap_or(json!({}));
            let mut avg_send_mbps = 0.0;
            let mut avg_recv_mbps = 0.0;
            let mut total_bytes_sent = 0;
            let mut total_bytes_received = 0;
            let mut total_retransmits = 0;

            if let Some(sum_sent) = end_summary.get("sum_sent") {
                avg_send_mbps = sum_sent.get("bits_per_second").and_then(|v| v.as_f64()).unwrap_or(0.0) / 1_000_000.0;
                total_bytes_sent = sum_sent.get("bytes").and_then(|v| v.as_u64()).unwrap_or(0);
                total_retransmits = sum_sent.get("retransmits").and_then(|v| v.as_u64()).unwrap_or(0);
            }

            if let Some(sum_received) = end_summary.get("sum_received") {
                avg_recv_mbps = sum_received.get("bits_per_second").and_then(|v| v.as_f64()).unwrap_or(0.0) / 1_000_000.0;
                total_bytes_received = sum_received.get("bytes").and_then(|v| v.as_u64()).unwrap_or(0);
            }

            // In case of UDP tests, check sum object
            if let Some(sum) = end_summary.get("sum") {
                if avg_send_mbps == 0.0 {
                    avg_send_mbps = sum.get("bits_per_second").and_then(|v| v.as_f64()).unwrap_or(0.0) / 1_000_000.0;
                }
                let jitter_ms = sum.get("jitter_ms").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let lost_percent = sum.get("lost_percent").and_then(|v| v.as_f64()).unwrap_or(0.0);

                return Ok(json!({
                    "is_success": true,
                    "avg_send_mbps": (avg_send_mbps * 100.0).round() / 100.0,
                    "avg_recv_mbps": (avg_recv_mbps * 100.0).round() / 100.0,
                    "total_bytes_sent": total_bytes_sent,
                    "total_bytes_received": total_bytes_received,
                    "jitter_ms": jitter_ms,
                    "lost_percent": lost_percent,
                    "intervals": intervals_data,
                    "raw_json": parsed_json
                }));
            }

            return Ok(json!({
                "is_success": true,
                "avg_send_mbps": (avg_send_mbps * 100.0).round() / 100.0,
                "avg_recv_mbps": (avg_recv_mbps * 100.0).round() / 100.0,
                "total_bytes_sent": total_bytes_sent,
                "total_bytes_received": total_bytes_received,
                "total_retransmits": total_retransmits,
                "intervals": intervals_data,
                "raw_json": parsed_json
            }));
        }

        // Fallback if stdout wasn't valid JSON (e.g. error message from iperf3)
        let combined_err = format!("{}\n{}", stdout, stderr);
        Ok(json!({
            "is_success": false,
            "error": combined_err.trim(),
            "raw": combined_err
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "High-performance bandwidth, throughput, jitter, and packet loss speed testing using iPerf3.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "iPerf3 server IP or domain" },
                    "port": { "type": "integer", "default": 5201, "minimum": 1, "maximum": 65535 },
                    "duration": { "type": "integer", "default": 10, "minimum": 1, "maximum": 300 },
                    "parallel": { "type": "integer", "default": 1, "minimum": 1, "maximum": 64 },
                    "protocol": { "type": "string", "enum": ["tcp", "udp"], "default": "tcp" },
                    "reverse": { "type": "boolean", "default": false, "description": "Test server-to-client download instead of upload" },
                    "bandwidth": { "type": "string", "default": "1G", "description": "Target bandwidth limit for UDP tests (e.g. 100M, 1G, 10G)" }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 60,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
