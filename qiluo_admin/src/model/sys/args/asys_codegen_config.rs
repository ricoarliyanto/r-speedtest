use crate::model::prelude::*;

/// 保存字段配置请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CodegenConfigSave {
    pub module_name: String,
    pub table_name: String,
    /// JSON 序列化的字段配置
    pub config_json: String,
}
