use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use crate::common::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContext {
    pub job_id: String,
    pub target: String,
    pub timeout_sec: u64,
    pub raw_params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub binary: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub default_timeout_sec: u64,
    pub required_permission: String,
}

#[async_trait]
pub trait NetworkTool: Send + Sync {
    /// Unique identifier of the tool
    fn id(&self) -> &'static str;
    
    /// User-friendly name
    fn name(&self) -> &'static str;

    /// Category (connectivity, throughput, packet, dns, etc.)
    fn category(&self) -> &'static str;

    /// Executable binary path (must be whitelisted and exist on host)
    fn binary(&self) -> &'static str;

    /// Build strongly-typed arguments vector (Zero arbitrary shell injection)
    fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>>;

    /// Parse stdout & stderr into structured JSON metrics
    fn parse_output(&self, stdout: &str, stderr: &str) -> Result<serde_json::Value>;

    /// Maximum execution timeout in seconds
    fn max_timeout(&self) -> u64 {
        300
    }

    /// Tool metadata for frontend reflection and registration
    fn metadata(&self) -> ToolMetadata;
}

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn NetworkTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register(Arc::new(super::ping::PingTool::new()));
        registry.register(Arc::new(super::traceroute::TracerouteTool::new()));
        registry.register(Arc::new(super::dns::DnsTool::new()));
        registry.register(Arc::new(super::portcheck::PortCheckTool::new()));
        registry.register(Arc::new(super::iperf3::Iperf3Tool::new()));
        registry.register(Arc::new(super::udpecho::UdpEchoTool::new()));
        registry.register(Arc::new(super::tr143::Tr143Tool::new()));
        registry.register(Arc::new(super::tcpdump::TcpDumpTool::new()));
        registry
    }

    pub fn register(&mut self, tool: Arc<dyn NetworkTool>) {
        self.tools.insert(tool.id().to_string(), tool);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn NetworkTool>> {
        self.tools.get(id).cloned()
    }

    pub fn list_metadata(&self) -> Vec<ToolMetadata> {
        self.tools.values().map(|t| t.metadata()).collect()
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_TOOL_REGISTRY: ToolRegistry = ToolRegistry::new();
}
