use async_trait::async_trait;
use serde_json::json;
use crate::common::error::{Error, Result};
use crate::tools::registry::{NetworkTool, ToolContext, ToolMetadata};

pub struct Tr143Tool;

impl Tr143Tool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NetworkTool for Tr143Tool {
    fn id(&self) -> &'static str {
        "tr143"
    }

    fn name(&self) -> &'static str {
        "TR-143 ISP Diagnostic Engine"
    }

    fn category(&self) -> &'static str {
        "throughput"
    }

    fn binary(&self) -> &'static str {
        "builtin:tr143"
    }

    fn max_timeout(&self) -> u64 {
        300
    }

    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>> {
        let mode = ctx.raw_params.get("mode").and_then(|v| v.as_str()).unwrap_or("download");
        let connections = ctx.raw_params.get("connections").and_then(|v| v.as_u64()).unwrap_or(4).min(16);
        let size_mb = ctx.raw_params.get("size_mb").and_then(|v| v.as_u64()).unwrap_or(50).min(500);

        Ok(vec![
            mode.to_string(),
            ctx.target.clone(),
            connections.to_string(),
            size_mb.to_string(),
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
            description: "Broadband Forum TR-143 Network-Side Diagnostic for HTTP Download/Upload throughput SLA verification.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Target TR-143 HTTP test URL" },
                    "mode": { "type": "string", "enum": ["download", "upload"], "default": "download" },
                    "connections": { "type": "integer", "default": 4, "minimum": 1, "maximum": 16 },
                    "size_mb": { "type": "integer", "default": 50, "minimum": 5, "maximum": 500 }
                },
                "required": ["target"]
            }),
            default_timeout_sec: 60,
            required_permission: "net:tool:execute".to_string(),
        }
    }
}
