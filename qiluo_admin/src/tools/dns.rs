use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct DnsTool;

impl DnsTool {
    pub fn new() -> Self {
        Self
    }

    fn sanitize_domain(domain: &str) -> Result<String> {
        let trimmed = domain.trim();
        if trimmed.is_empty() {
            return Err(Error::Message("Domain query cannot be empty".to_string()));
        }
        if trimmed.contains(|c: char| c.is_whitespace() || c == ';' || c == '&' || c == '|' || c == '`' || c == '$') {
            return Err(Error::Message("Domain query contains invalid characters".to_string()));
        }
        Ok(trimmed.to_string())
    }
}

#[async_trait]
impl NetworkTool for DnsTool {
    fn id(&self) -> &'static str {
        "dns"
    }

    fn name(&self) -> &'static str {
        "DNS Lookup"
    }

    fn category(&self) -> &'static str {
        "dns"
    }

    fn binary(&self) -> &'static str {
        "/usr/bin/dig"
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let domain = Self::sanitize_domain(&ctx.target)?;
        let record_type = ctx.raw_params.get("record_type").and_then(|v| v.as_str()).unwrap_or("A");
        
        let mut args = vec![
            domain,
            record_type.to_string(),
            "+noall".to_string(),
            "+answer".to_string(),
            "+stats".to_string(),
        ];

        if let Some(server) = ctx.raw_params.get("server").and_then(|v| v.as_str()) {
            let sanitized_server = Self::sanitize_domain(server)?;
            args.push(format!("@{}", sanitized_server));
        }

        Ok(args)
    }

    fn parse_output(&self, stdout: &str, _stderr: &str) -> Result<serde_json::Value> {
        let mut answers = Vec::new();
        let mut query_time_ms: Option<u64> = None;
        let mut resolver_server: Option<String> = None;

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.contains("Query time:") {
                if let Some(time_str) = trimmed.split("Query time:").nth(1) {
                    if let Some(val) = time_str.split_whitespace().next() {
                        query_time_ms = val.parse().ok();
                    }
                }
            } else if trimmed.contains("SERVER:") {
                if let Some(server_str) = trimmed.split("SERVER:").nth(1) {
                    resolver_server = Some(server_str.trim().to_string());
                }
            } else if !trimmed.starts_with(';') {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 5 {
                    answers.push(json!({
                        "name": parts[0],
                        "ttl": parts[1].parse::<u64>().unwrap_or(0),
                        "class": parts[2],
                        "type": parts[3],
                        "data": parts[4..].join(" ")
                    }));
                }
            }
        }

        Ok(json!({
            "answers": answers,
            "query_time_ms": query_time_ms,
            "resolver_server": resolver_server,
            "records_count": answers.len()
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "Performs DNS record queries (A, AAAA, CNAME, MX, TXT) and measures resolution time.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Domain name to resolve" },
                    "record_type": { "type": "string", "enum": ["A", "AAAA", "CNAME", "MX", "TXT", "NS", "SOA", "PTR"], "default": "A" },
                    "server": { "type": "string", "description": "Optional custom DNS server IP (e.g. 1.1.1.1)" }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 10,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
