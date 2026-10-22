use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct TracerouteTool;

impl TracerouteTool {
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
impl NetworkTool for TracerouteTool {
    fn id(&self) -> &'static str {
        "traceroute"
    }

    fn name(&self) -> &'static str {
        "Traceroute"
    }

    fn category(&self) -> &'static str {
        "connectivity"
    }

    fn binary(&self) -> &'static str {
        "/usr/bin/traceroute"
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let target = Self::sanitize_target(&ctx.target)?;
        let max_hops = ctx.raw_params.get("max_hops").and_then(|v| v.as_u64()).unwrap_or(30).min(64);
        let timeout = ctx.timeout_sec.max(1).min(60);

        let args = vec![
            "-m".to_string(),
            max_hops.to_string(),
            "-w".to_string(),
            timeout.to_string(),
            "-n".to_string(), // Numerical IP format for faster parsing
            target,
        ];

        Ok(args)
    }

    fn parse_output(&self, stdout: &str, _stderr: &str) -> Result<serde_json::Value> {
        let mut hops: Vec<serde_json::Value> = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("traceroute to") {
                continue;
            }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            // First token is hop number
            if let Ok(hop_num) = tokens[0].parse::<u32>() {
                let mut ip = "*";
                let mut rtt_list: Vec<f64> = Vec::new();
                let mut is_timeout = true;

                for (idx, token) in tokens.iter().enumerate() {
                    if idx == 0 {
                        continue;
                    }
                    if *token != "*" && *token != "ms" {
                        if token.contains('.') || token.contains(':') {
                            ip = token;
                            is_timeout = false;
                        } else if let Ok(rtt) = token.parse::<f64>() {
                            rtt_list.push(rtt);
                            is_timeout = false;
                        }
                    }
                }

                let avg_rtt = if !rtt_list.is_empty() {
                    Some(rtt_list.iter().sum::<f64>() / rtt_list.len() as f64)
                } else {
                    None
                };

                hops.push(json!({
                    "hop": hop_num,
                    "ip": ip,
                    "rtts_ms": rtt_list,
                    "avg_rtt_ms": avg_rtt,
                    "is_timeout": is_timeout
                }));
            }
        }

        let total_hops = hops.len();
        let destination_reached = hops.last().map(|h| !h["is_timeout"].as_bool().unwrap_or(false)).unwrap_or(false);

        Ok(json!({
            "total_hops": total_hops,
            "destination_reached": destination_reached,
            "hops": hops
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "Traces the network path, intermediary routers, and latency to a destination host.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Destination IP or domain" },
                    "max_hops": { "type": "integer", "default": 30, "minimum": 1, "maximum": 64 }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 30,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
