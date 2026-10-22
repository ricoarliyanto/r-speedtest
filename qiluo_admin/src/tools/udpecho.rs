use async_trait::async_trait;
use serde_json::json;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct UdpEchoTool;

impl UdpEchoTool {
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

    /// Performs native high-precision async UDP Echo test
    pub async fn execute_native_udp_test(
        target_addr: &str,
        port: u16,
        count: u64,
        payload_size: usize,
        interval_ms: u64,
        timeout_ms: u64,
    ) -> Result<serde_json::Value> {
        let target_full = format!("{}:{}", target_addr, port);
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| Error::Message(format!("Failed to bind local UDP socket: {:?}", e)))?;

        socket.connect(&target_full).await
            .map_err(|e| Error::Message(format!("Failed to connect UDP socket to {}: {:?}", target_full, e)))?;

        let mut rtt_series: Vec<f64> = Vec::new();
        let mut packets_sent: u64 = 0;
        let mut packets_received: u64 = 0;
        let mut jitter_acc: f64 = 0.0;
        let mut last_d: Option<f64> = None;

        let mut recv_buf = vec![0u8; 65535];

        for seq in 0..count {
            packets_sent += 1;
            let mut payload = vec![0u8; payload_size.max(8)];
            // Write sequence number into first 4 bytes
            payload[0..4].copy_from_slice(&(seq as u32).to_be_bytes());

            let send_time = Instant::now();
            let _ = socket.send(&payload).await;

            // Wait for echo response with timeout
            let timeout_fut = tokio::time::sleep(Duration::from_millis(timeout_ms));
            tokio::pin!(timeout_fut);

            tokio::select! {
                res = socket.recv(&mut recv_buf) => {
                    match res {
                        Ok(len) if len >= 4 => {
                            let rtt_ms = send_time.elapsed().as_secs_f64() * 1000.0;
                            rtt_series.push(rtt_ms);
                            packets_received += 1;

                            // RFC 3550 Jitter calculation
                            if let Some(prev_rtt) = last_d {
                                let diff = (rtt_ms - prev_rtt).abs();
                                jitter_acc = jitter_acc + (diff - jitter_acc) / 16.0;
                            }
                            last_d = Some(rtt_ms);
                        }
                        _ => {}
                    }
                }
                _ = &mut timeout_fut => {
                    // Packet timed out
                }
            }

            if interval_ms > 0 && seq + 1 < count {
                tokio::time::sleep(Duration::from_millis(interval_ms)).await;
            }
        }

        let loss_percent = if packets_sent > 0 {
            ((packets_sent - packets_received) as f64 / packets_sent as f64) * 100.0
        } else {
            100.0
        };

        let min_rtt = rtt_series.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_rtt = rtt_series.iter().cloned().fold(0.0, f64::max);
        let avg_rtt = if !rtt_series.is_empty() {
            rtt_series.iter().sum::<f64>() / rtt_series.len() as f64
        } else {
            0.0
        };

        Ok(json!({
            "target": target_full,
            "packets_sent": packets_sent,
            "packets_received": packets_received,
            "packet_loss_percent": (loss_percent * 100.0).round() / 100.0,
            "min_rtt_ms": if min_rtt.is_infinite() { 0.0 } else { (min_rtt * 100.0).round() / 100.0 },
            "avg_rtt_ms": (avg_rtt * 100.0).round() / 100.0,
            "max_rtt_ms": (max_rtt * 100.0).round() / 100.0,
            "jitter_ms": (jitter_acc * 100.0).round() / 100.0,
            "rtt_series": rtt_series,
            "is_reachable": packets_received > 0
        }))
    }
}

#[async_trait]
impl NetworkTool for UdpEchoTool {
    fn id(&self) -> &'static str {
        "udpecho"
    }

    fn name(&self) -> &'static str {
        "UDP Echo & Jitter Diagnostic"
    }

    fn category(&self) -> &'static str {
        "throughput"
    }

    fn binary(&self) -> &'static str {
        "builtin:udpecho"
    }

    fn max_timeout(&self) -> u64 {
        120
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let target = Self::sanitize_target(&ctx.target)?;
        let port = ctx.raw_params.get("port").and_then(|v| v.as_u64()).unwrap_or(37001).min(65535);
        let count = ctx.raw_params.get("count").and_then(|v| v.as_u64()).unwrap_or(50).min(5000);
        let size = ctx.raw_params.get("size").and_then(|v| v.as_u64()).unwrap_or(64).min(1472);

        Ok(vec![
            target,
            port.to_string(),
            count.to_string(),
            size.to_string(),
        ])
    }

    fn parse_output(&self, stdout: &str, _stderr: &str) -> Result<serde_json::Value> {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout) {
            return Ok(val);
        }
        Ok(json!({ "raw": stdout }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "High-precision microsecond UDP Echo test calculating RFC 3550 Jitter and packet loss.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Target UDP echo server IP or domain" },
                    "port": { "type": "integer", "default": 37001, "minimum": 1, "maximum": 65535 },
                    "count": { "type": "integer", "default": 50, "minimum": 1, "maximum": 5000 },
                    "size": { "type": "integer", "default": 64, "minimum": 16, "maximum": 1472 },
                    "interval_ms": { "type": "integer", "default": 20, "minimum": 1, "maximum": 1000 }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 20,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
