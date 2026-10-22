use crate::common::codegen::generator::{self, CodeGenRequest};
use crate::common::codegen::parser;
use crate::common::codegen::parser::FieldConfig;
use crate::model::sys::model::msys_codegen_config::{CodegenConfigSave, SysCodegenConfigModel};
use crate::service::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// 保存字段配置请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SaveFieldConfigRequest {
    pub module_name: String,
    pub table_name: String,
    pub field_config: Vec<FieldConfig>,
}

/// 扫描所有可用的 Entity
pub async fn scan_entities() -> impl IntoResponse {
    // 获取项目 model 目录路径
    let model_path = get_model_base_path();
    let results = generator::scan_all_entities_json(&model_path);
    ApiResponse::ok(results)
}

/// 生成代码
pub async fn generate(VJson(arg): VJson<CodeGenRequest>) -> impl IntoResponse {
    let model_path = get_model_base_path();

    // 从 Entity 文件解析信息
    let entity_dir = std::path::Path::new(&model_path)
        .join(&arg.module_name)
        .join("entity");

    let entity_file = entity_dir.join(format!("{}.rs", arg.table_name));

    if !entity_file.exists() {
        return ApiResponse::not_found(format!(
            "Entity 文件不存在: model/{}/entity/{}.rs",
            arg.module_name, arg.table_name
        ));
    }

    let content = match std::fs::read_to_string(&entity_file) {
        Ok(c) => c,
        Err(e) => {
            return ApiResponse::internal_server_error(format!("读取 Entity 文件失败: {}", e));
        }
    };

    let entity = parser::parse_entity_content(&content, &arg.table_name, &arg.module_name);

    

    let result = match generator::generate_code(&entity, arg.field_config.as_deref(), arg.enable_data_scope) {
        Ok(r) => r,
        Err(e) => {
            return ApiResponse::internal_server_error(format!("代码生成失败: {}", e));
        }
    };

    ApiResponse::ok(result)
}

/// 保存字段配置
pub async fn save_config(VJson(arg): VJson<SaveFieldConfigRequest>) -> impl IntoResponse {
    let config_json = match serde_json::to_string(&arg.field_config) {
        Ok(s) => s,
        Err(e) => return ApiResponse::internal_server_error(format!("序列化配置失败: {}", e)),
    };

    let save_arg = CodegenConfigSave {
        module_name: arg.module_name,
        table_name: arg.table_name,
        config_json,
    };

    match SysCodegenConfigModel::save_config(save_arg).await {
        Ok(_) => ApiResponse::ok("配置保存成功"),
        Err(e) => ApiResponse::internal_server_error(format!("保存配置失败: {}", e)),
    }
}

/// 加载字段配置
pub async fn load_config(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let module_name = params.get("module_name").cloned().unwrap_or_default();
    let table_name = params.get("table_name").cloned().unwrap_or_default();
    if module_name.is_empty() || table_name.is_empty() {
        return ApiResponse::bad_request("缺少参数 module_name 或 table_name");
    }

    match SysCodegenConfigModel::get_config(&module_name, &table_name).await {
        Ok(Some(config_json)) => {
            match serde_json::from_str::<Vec<FieldConfig>>(&config_json) {
                Ok(configs) => ApiResponse::ok(configs),
                Err(e) => ApiResponse::internal_server_error(format!("反序列化配置失败: {}", e)),
            }
        }
        Ok(None) => ApiResponse::ok(Vec::<FieldConfig>::new()),
        Err(e) => ApiResponse::internal_server_error(format!("加载配置失败: {}", e)),
    }
}

/// 获取 model 目录的基础路径
fn get_model_base_path() -> String {
    // 尝试从当前工作目录获取
    if let Ok(current_dir) = std::env::current_dir() {
        let model_path = current_dir.join("src").join("model");
        if model_path.exists() {
            return model_path.to_string_lossy().to_string();
        }
    }

    // 回退：相对于 Cargo.toml 的路径
    "src/model".to_string()
}
