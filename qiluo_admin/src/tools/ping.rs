use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct PingTool;

impl PingTool {
    pub fn new() -> Self {
        Self
    }

    fn sanitize_target(target: &str) -> Result<String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err(Error::Message("Target host cannot be empty".to_string()));
        }
        // Disallow dangerous shell characters
        if trimmed.contains(|c: char| c.is_whitespace() || c == ';' || c == '&' || c == '|' || c == '`' || c == '$') {
            return Err(Error::Message("Target host contains invalid characters".to_string()));
        }
        Ok(trimmed.to_string())
    }
}

#[async_trait]
impl NetworkTool for PingTool {
    fn id(&self) -> &'static str {
        "ping"
    }

    fn name(&self) -> &'static str {
        "ICMP Ping"
    }

    fn category(&self) -> &'static str {
        "connectivity"
    }

    fn binary(&self) -> &'static str {
        "/usr/bin/ping"
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let target = Self::sanitize_target(&ctx.target)?;
        let count = ctx.raw_params.get("count").and_then(|v| v.as_u64()).unwrap_or(4).min(100);
        let size = ctx.raw_params.get("size").and_then(|v| v.as_u64()).unwrap_or(56).min(65507);
        let timeout = ctx.timeout_sec.max(1).min(60);

        let mut args = vec![
            "-c".to_string(),
            count.to_string(),
            "-s".to_string(),
            size.to_string(),
            "-W".to_string(),
            timeout.to_string(),
        ];

        if let Some(interval) = ctx.raw_params.get("interval").and_then(|v| v.as_f64()) {
            if interval >= 0.2 && interval <= 10.0 {
                args.push("-i".to_string());
                args.push(format!("{:.1}", interval));
            }
        }

        args.push(target);
        Ok(args)
    }

    fn parse_output(&self, stdout: &str, _stderr: &str) -> Result<serde_json::Value> {
        let mut packets_transmitted: u64 = 0;
        let mut packets_received: u64 = 0;
        let mut packet_loss_percent: f64 = 100.0;
        let mut min_rtt: Option<f64> = None;
        let mut avg_rtt: Option<f64> = None;
        let mut max_rtt: Option<f64> = None;
        let mut mdev_rtt: Option<f64> = None;
        let mut rtt_series: Vec<f64> = Vec::new();

        for line in stdout.lines() {
            // Parse individual echo reply: "64 bytes from ... time=14.2 ms"
            if line.contains("time=") {
                if let Some(time_part) = line.split("time=").nth(1) {
                    if let Some(val_str) = time_part.split_whitespace().next() {
                        if let Ok(val) = val_str.parse::<f64>() {
                            rtt_series.push(val);
                        }
                    }
                }
            }

            // Parse packet stats: "4 packets transmitted, 4 received, 0% packet loss"
            if line.contains("packets transmitted") {
                let parts: Vec<&str> = line.split(',').collect();
                for part in parts {
                    let p = part.trim();
                    if p.contains("packets transmitted") {
                        if let Some(num) = p.split_whitespace().next() {
                            packets_transmitted = num.parse().unwrap_or(0);
                        }
                    } else if p.contains("received") {
                        if let Some(num) = p.split_whitespace().next() {
                            packets_received = num.parse().unwrap_or(0);
                        }
                    } else if p.contains("packet loss") {
                        if let Some(loss_str) = p.split('%').next() {
                            if let Some(num) = loss_str.split_whitespace().last() {
                                packet_loss_percent = num.parse().unwrap_or(100.0);
                            }
                        }
                    }
                }
            }

            // Parse RTT stats: "rtt min/avg/max/mdev = 12.341/14.512/18.231/1.942 ms"
            if line.contains("min/avg/max") {
                if let Some(stats_part) = line.split('=').nth(1) {
                    let parts: Vec<&str> = stats_part.trim().split('/').collect();
                    if parts.len() >= 4 {
                        min_rtt = parts[0].trim().parse().ok();
                        avg_rtt = parts[1].trim().parse().ok();
                        max_rtt = parts[2].trim().parse().ok();
                        mdev_rtt = parts[3].split_whitespace().next().and_then(|s| s.parse().ok());
                    }
                }
            }
        }

        Ok(json!({
            "packets_transmitted": packets_transmitted,
            "packets_received": packets_received,
            "packet_loss_percent": packet_loss_percent,
            "min_rtt_ms": min_rtt,
            "avg_rtt_ms": avg_rtt,
            "max_rtt_ms": max_rtt,
            "stddev_rtt_ms": mdev_rtt,
            "rtt_series": rtt_series,
            "is_reachable": packets_received > 0
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "Measures latency, round-trip time, and packet loss using ICMP echo requests.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Destination IP or domain" },
                    "count": { "type": "integer", "default": 4, "minimum": 1, "maximum": 100 },
                    "size": { "type": "integer", "default": 56, "minimum": 16, "maximum": 1500 },
                    "interval": { "type": "number", "default": 1.0, "minimum": 0.2, "maximum": 10.0 }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 15,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
