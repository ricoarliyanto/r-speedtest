use async_trait::async_trait;
use serde_json::json;
use std::path::Path;
use tokio::fs;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct TcpDumpTool;

impl TcpDumpTool {
    pub fn new() -> Self {
        Self
    }

    fn sanitize_interface(iface: &str) -> Result<String> {
        let trimmed = iface.trim();
        if trimmed.is_empty() {
            return Ok("any".to_string());
        }
        if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') {
            return Err(Error::Message("Interface contains invalid characters".to_string()));
        }
        Ok(trimmed.to_string())
    }

    fn sanitize_bpf_filter(filter: &str) -> Result<String> {
        let trimmed = filter.trim();
        if trimmed.is_empty() {
            return Ok(String::new());
        }
        // Strict allowlist of characters for BPF filter expression
        // Disallow shell operators: ;, &, |, `, $, <, >, (, ), \n, \r, \t
        for c in trimmed.chars() {
            if !c.is_ascii_alphanumeric() && c != ' ' && c != '.' && c != ':' && c != '-' && c != '_' && c != '/' {
                return Err(Error::Message(format!("BPF filter contains unsafe character '{}'", c)));
            }
        }
        Ok(trimmed.to_string())
    }

    pub fn get_pcap_dir() -> &'static str {
        "./artifacts/pcap"
    }

    pub fn get_pcap_path(job_id: &str) -> String {
        format!("{}/{}.pcap", Self::get_pcap_dir(), job_id)
    }
}

#[async_trait]
impl NetworkTool for TcpDumpTool {
    fn id(&self) -> &'static str {
        "tcpdump"
    }

    fn name(&self) -> &'static str {
        "TCPDump Packet Capture"
    }

    fn category(&self) -> &'static str {
        "deep_diagnostics"
    }

    fn binary(&self) -> &'static str {
        "/usr/sbin/tcpdump"
    }

    fn max_timeout(&self) -> u64 {
        60
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let iface = ctx.raw_params.get("interface").and_then(|v| v.as_str()).unwrap_or("any");
        let sanitized_iface = Self::sanitize_interface(iface)?;

        let packet_count = ctx.raw_params.get("count").and_then(|v| v.as_u64()).unwrap_or(100).min(5000);
        let bpf_filter = ctx.raw_params.get("filter").and_then(|v| v.as_str()).unwrap_or("");
        let sanitized_filter = Self::sanitize_bpf_filter(bpf_filter)?;

        let pcap_file = Self::get_pcap_path(&ctx.job_id);

        let mut args = vec![
            "-i".to_string(),
            sanitized_iface,
            "-c".to_string(),
            packet_count.to_string(),
            "-w".to_string(),
            pcap_file,
            "-n".to_string(),
            "-U".to_string(), // Packet-buffered output
        ];

        if !sanitized_filter.is_empty() {
            args.push(sanitized_filter);
        }

        Ok(args)
    }

    fn parse_output(&self, stdout: &str, stderr: &str) -> Result<serde_json::Value> {
        let mut packets_captured: u64 = 0;
        let mut packets_received: u64 = 0;
        let mut packets_dropped: u64 = 0;

        let combined = format!("{}\n{}", stdout, stderr);
        for line in combined.lines() {
            let lower = line.to_lowercase();
            if lower.contains("packet captured") || lower.contains("packets captured") {
                if let Some(first_word) = line.split_whitespace().next() {
                    packets_captured = first_word.parse().unwrap_or(0);
                }
            } else if lower.contains("packet received by filter") || lower.contains("packets received by filter") {
                if let Some(first_word) = line.split_whitespace().next() {
                    packets_received = first_word.parse().unwrap_or(0);
                }
            } else if lower.contains("packet dropped by kernel") || lower.contains("packets dropped by kernel") {
                if let Some(first_word) = line.split_whitespace().next() {
                    packets_dropped = first_word.parse().unwrap_or(0);
                }
            }
        }

        Ok(json!({
            "is_success": true,
            "packets_captured": packets_captured,
            "packets_received_by_filter": packets_received,
            "packets_dropped_by_kernel": packets_dropped,
            "raw_summary": combined.trim()
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "Deep packet inspection and capture engine generating downloadable PCAP artifacts.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "interface": { "type": "string", "default": "any", "description": "Network interface to capture on (e.g. eth0, any)" },
                    "count": { "type": "integer", "default": 100, "minimum": 1, "maximum": 5000, "description": "Maximum packets to capture" },
                    "filter": { "type": "string", "default": "", "description": "Safe BPF filter expression (e.g. tcp port 80 or icmp)" },
                    "duration_sec": { "type": "integer", "default": 10, "minimum": 1, "maximum": 60, "description": "Maximum capture duration in seconds" }
                }
            }),
            default_timeout_sec: 30,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
