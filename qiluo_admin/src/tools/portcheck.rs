use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use super::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct PortCheckTool;

impl PortCheckTool {
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
impl NetworkTool for PortCheckTool {
    fn id(&self) -> &'static str {
        "portcheck"
    }

    fn name(&self) -> &'static str {
        "TCP Port Check"
    }

    fn category(&self) -> &'static str {
        "connectivity"
    }

    fn binary(&self) -> &'static str {
        "/bin/nc"
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let target = Self::sanitize_target(&ctx.target)?;
        let port = ctx.raw_params.get("port").and_then(|v| v.as_u64()).unwrap_or(80);
        if port < 1 || port > 65535 {
            return Err(Error::Message("Port must be between 1 and 65535".to_string()));
        }

        let timeout = ctx.timeout_sec.max(1).min(30);

        let args = vec![
            "-z".to_string(),
            "-v".to_string(),
            "-w".to_string(),
            timeout.to_string(),
            target,
            port.to_string(),
        ];

        Ok(args)
    }

    fn parse_output(&self, stdout: &str, stderr: &str) -> Result<serde_json::Value> {
        let combined = format!("{}\n{}", stdout, stderr);
        let is_open = combined.contains("succeeded") || combined.contains("open") || combined.contains("Connected");
        
        Ok(json!({
            "is_open": is_open,
            "status": if is_open { "OPEN" } else { "CLOSED_OR_FILTERED" },
            "raw": combined.trim()
        }))
    }

    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().to_string(),
            binary: self.binary().to_string(),
            description: "Checks if a remote TCP port is open, listening, or blocked by a firewall.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Target IP or hostname" },
                    "port": { "type": "integer", "default": 80, "minimum": 1, "maximum": 65535 }
                },
                "required": ["target", "port"]
            }),
            default_timeout_sec: 10,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
