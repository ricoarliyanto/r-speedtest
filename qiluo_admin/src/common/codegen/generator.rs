use super::parser::{camel_case, EntityInfo, FieldConfig, FieldInfo, RelationInfo, RelationKind, scan_entities};
use super::templates::*;
use crate::common::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tera::{Context, Tera};
use validator::Validate;


#[derive(Debug, Clone, Default)]
struct FieldValidation {
    required: Option<bool>,
    min_length: Option<i32>,
    max_length: Option<i32>,
    pattern: Option<String>,
    pattern_message: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CodeGenRequest {
    pub module_name: String,
    pub table_name: String,
    pub field_config: Option<Vec<crate::common::codegen::parser::FieldConfig>>,
    pub enable_data_scope: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
    pub file_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenResult {
    pub entity_info: EntityInfo,
    pub files: Vec<GeneratedFile>,
    pub mod_updates: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub module_name: String,
    pub entities: Vec<EntityInfo>,
}

pub fn scan_all_entities_json(model_base_path: &str) -> serde_json::Value {
    let results = scan_all_entities(model_base_path);
    let json_results: Vec<serde_json::Value> = results
        .into_iter()
        .map(|sr| {
            let entities_json: Vec<serde_json::Value> = sr
                .entities
                .into_iter()
                .map(|e| entity_to_json(&e))
                .collect();
            json!({
                "module_name": sr.module_name,
                "entities": entities_json,
            })
        })
        .collect();
    json!(json_results)
}


fn entity_to_json(e: &EntityInfo) -> serde_json::Value {
    let fields_json: Vec<serde_json::Value> = e.fields.iter().map(field_to_json).collect();
    let searchable_json: Vec<serde_json::Value> = e.searchable_fields.iter().map(field_to_json).collect();
    let list_json: Vec<serde_json::Value> = e.list_fields.iter().map(field_to_json).collect();
    let form_json: Vec<serde_json::Value> = e.form_fields.iter().map(field_to_json).collect();
    let detail_json: Vec<serde_json::Value> = e.detail_fields.iter().map(field_to_json).collect();
    let belongs_to_json: Vec<serde_json::Value> = e.belongs_to_fields.iter().map(field_to_json).collect();
    let relations_json: Vec<serde_json::Value> = e.relations.iter().map(|r| {
        json!({
            "kind": format!("{:?}", r.kind),
            "variant_name": r.variant_name,
            "target_module": r.target_module,
            "target_entity": r.target_entity,
            "target_model": r.target_model,
            "display_field": r.display_field,
            "fk_field": r.fk_field,
            "fk_in_target": r.fk_in_target,
        })
    }).collect();

    json!({
        "table_name": e.table_name,
        "module_name": e.module_name,
        "model_name": e.model_name,
        "primary_key": e.primary_key,
        "primary_key_pascal": e.primary_key_pascal,
        "fields": fields_json,
        "searchable_fields": searchable_json,
        "list_fields": list_json,
        "form_fields": form_json,
        "detail_fields": detail_json,
        "relations": relations_json,
        "belongs_to_fields": belongs_to_json,
    })
}

pub fn scan_all_entities(model_base_path: &str) -> Vec<ScanResult> {
    let mut results = Vec::new();

    if let Ok(entries) = std::fs::read_dir(model_base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let module_name = path.file_name().unwrap().to_string_lossy().to_string();
                if module_name == "prelude.rs" || module_name.starts_with('.') {
                    continue;
                }
                let entity_dir = path.join("entity");
                if entity_dir.exists() {
                    let entities = scan_entities(model_base_path, &module_name);
                    if !entities.is_empty() {
                        results.push(ScanResult {
                            module_name,
                            entities,
                        });
                    }
                }
            }
        }
    }

    results
}

pub fn generate_code(entity: &EntityInfo, field_config: Option<&[FieldConfig]>, enable_data_scope: Option<bool>) -> Result<CodeGenResult> {
    let mut entity = entity.clone();
    let mut field_validation_map: HashMap<String, FieldValidation> = HashMap::new();
    if let Some(configs) = field_config {
        crate::common::codegen::parser::apply_field_config(&mut entity, configs);
        for c in configs {
            let v = FieldValidation {
                required: c.required,
                min_length: c.min_length,
                max_length: c.max_length,
                pattern: c.pattern.clone(),
                pattern_message: c.pattern_message.clone(),
            };
            field_validation_map.insert(c.field_name.clone(), v);
        }
    }


    if let Some(ds) = enable_data_scope {
        entity.has_data_scope = ds;
    }
    let data_scope_enabled = entity.has_data_scope;

 
    let mut files = Vec::new();
    let mut mod_updates = HashMap::new();


    let ctx = build_tera_context(&entity, &field_validation_map, data_scope_enabled);


    let model_template = if data_scope_enabled { TEMPLATE_MODEL_DATA_SCOPE } else { TEMPLATE_MODEL };
    let service_template = if data_scope_enabled { TEMPLATE_SERVICE_DATA_SCOPE } else { TEMPLATE_SERVICE };

    // 1. 生成 args 文件
    let args_content = render_template(TEMPLATE_ARGS, &ctx)?;
    files.push(GeneratedFile {
        path: format!(
            "src/model/{}/args/{}.rs",
            entity.module_name,
            entity.args_file_name()
        ),
        content: args_content,
        file_type: "rust".to_string(),
        description: format!("{} 参数定义（Resp/Search/Add/Edit/Del）", entity.model_name),
    });

    // 2. 生成 model 文件
    let model_content = render_template(model_template, &ctx)?;
    files.push(GeneratedFile {
        path: format!(
            "src/model/{}/model/{}.rs",
            entity.module_name,
            entity.model_file_name()
        ),
        content: model_content,
        file_type: "rust".to_string(),
        description: format!("{} 数据库操作", entity.model_name),
    });

    // 3. 生成 service 文件
    let service_content = render_template(service_template, &ctx)?;
    files.push(GeneratedFile {
        path: format!("src/service/{}/{}.rs", entity.module_name, entity.service_file_name()),
        content: service_content,
        file_type: "rust".to_string(),
        description: format!("{} Service 层", entity.model_name),
    });

    // 4. 生成 API 路由片段
    let api_route_content = render_template(TEMPLATE_API_ROUTE_FN, &ctx)?;
    files.push(GeneratedFile {
        path: format!("src/api/{}_route_fragment.txt", entity.table_name),
        content: api_route_content,
        file_type: "rust".to_string(),
        description: format!("{} API 路由函数片段（需手动合并到 api/{}.rs）", entity.model_name, entity.module_name),
    });

    // 5. 生成 API nest 片段
    let api_nest_content = render_template(TEMPLATE_API_NEST, &ctx)?;
    mod_updates.insert(
        "api_nest".to_string(),
        api_nest_content,
    );

    // 6. 记录 mod.rs 需要的更新
    let model_mod = render_template(TEMPLATE_MODEL_MOD, &ctx)?;
    mod_updates.insert(
        format!("model/{}/model/mod.rs", entity.module_name),
        model_mod,
    );

    let args_mod = render_template(TEMPLATE_ARGS_MOD, &ctx)?;
    mod_updates.insert(
        format!("model/{}/args/mod.rs", entity.module_name),
        args_mod,
    );

    let service_mod = render_template(TEMPLATE_SERVICE_MOD, &ctx)?;
    mod_updates.insert(
        format!("service/{}.rs", entity.module_name),
        service_mod,
    );

    // 7. 生成前端 API 文件
    let frontend_api_content = render_file_template("frontend_api.tera", &ctx)?;
    files.push(GeneratedFile {
        path: format!("src/api/{}.ts", entity.api_dir_name()),
        content: frontend_api_content,
        file_type: "typescript".to_string(),
        description: format!("{} 前端 API", entity.model_name),
    });

    // 8. 生成前端列表页
    let frontend_view_content = render_file_template("frontend_view.tera", &ctx)?;
    files.push(GeneratedFile {
        path: format!("src/views/{}/{}/{}.vue", capitalize_first(&entity.module_name), entity.view_dir_name(),entity.view_dir_name()),
        content: frontend_view_content,
        file_type: "vue".to_string(),
        description: format!("{} 列表页", entity.model_name),
    });

    // 9. 生成前端表单组件
    let frontend_write_content = render_file_template("frontend_write.tera", &ctx)?;
    files.push(GeneratedFile {
        path: format!(
            "src/views/{}/{}/components/Write.vue",
            capitalize_first(&entity.module_name),
            entity.view_dir_name()
        ),
        content: frontend_write_content,
        file_type: "vue".to_string(),
        description: format!("{} 表单组件", entity.model_name),
    });

    // 10. 生成前端详情组件
    let frontend_detail_content = render_file_template("frontend_detail.tera", &ctx)?;
    files.push(GeneratedFile {
        path: format!(
            "src/views/{}/{}/components/Detail.vue",
            capitalize_first(&entity.module_name),
            entity.view_dir_name()
        ),
        content: frontend_detail_content,
        file_type: "vue".to_string(),
        description: format!("{} 详情组件", entity.model_name),
    });

    // 11. 生成多语言翻译片段
    let (zh_locale, en_locale) = generate_locale_snippet(&entity);
    files.push(GeneratedFile {
        path: "src/locales/menuuser.zh-CN.snippet.json".to_string(),
        content: zh_locale,
        file_type: "json".to_string(),
        description: format!("{} 中文翻译片段（合并到 zh-CN.ts menuuser）", entity.model_name),
    });
    files.push(GeneratedFile {
        path: "src/locales/menuuser.en.snippet.json".to_string(),
        content: en_locale,
        file_type: "json".to_string(),
        description: format!("{} English translation snippet (merge into en.ts menuuser)", entity.model_name),
    });

    Ok(CodeGenResult {
        entity_info: entity.clone(),
        files,
        mod_updates,
    })
}


/// 为单个字段生成独立 rules 对象中的一条规则
/// 例如: `  name: [{ required: true, message: t('menuuser.name') + t('button.required'), trigger: 'blur' }],`
/// 如果无规则则返回 None
fn build_field_rules_entry(f: &FieldInfo, field_validation_map: &HashMap<String, FieldValidation>) -> Option<String> {
    let mut rules: Vec<String> = Vec::new();

    let fc = field_validation_map.get(&f.name);
    let is_required = fc.and_then(|c| c.required).unwrap_or(!f.is_optional);
    let min_len = fc.and_then(|c| c.min_length);
    let max_len = fc.and_then(|c| c.max_length);
    let pat = fc.and_then(|c| c.pattern.as_deref());
    let pat_msg = fc.and_then(|c| c.pattern_message.as_deref());

    if is_required {
        rules.push(format!("{{ required: true, message: t('menuuser.{}') + t('button.required'), trigger: 'blur' }}", f.name));
    }
    if let Some(min) = min_len {
        rules.push(format!("{{ min: {}, message: t('menuuser.{}') + '不能少于{}个字符', trigger: 'blur' }}", min, f.name, min));
    }
    if let Some(max) = max_len {
        rules.push(format!("{{ max: {}, message: t('menuuser.{}') + '不能超过{}个字符', trigger: 'blur' }}", max, f.name, max));
    }
    if let Some(pattern) = pat {
        let msg = pat_msg.unwrap_or("格式不正确");
        rules.push(format!("{{ pattern: /{}/, message: '{}', trigger: 'blur' }}", pattern, msg));
    }

    if rules.is_empty() {
        None
    } else {
        Some(format!("  {}: [{}],", f.name, rules.join(", ")))
    }
}
fn build_tera_context(entity: &EntityInfo, field_validation_map: &HashMap<String, FieldValidation>, data_scope_enabled: bool) -> serde_json::Value {
    let resp_fields: Vec<serde_json::Value> = entity
        .fields
        .iter()
        .map(field_to_json)
        .collect();

    // 搜索字段
    let search_fields: Vec<serde_json::Value> = entity
        .searchable_fields
        .iter()
        .map(field_to_json)
        .collect();

    // Add 字段（排除主键和时间）
    let add_fields: Vec<serde_json::Value> = entity
        .form_fields
        .iter()
        .filter(|f| !f.is_primary_key)
        .map(field_to_json)
        .collect();

    // Edit 字段（排除主键，模板已单独硬编码 primary_key）
    let edit_fields: Vec<serde_json::Value> = entity
        .form_fields
        .iter()
        .filter(|f| !f.is_primary_key)
        .map(field_to_json)
        .collect();

    // 列表展示字段（排除主键在 action 列展示）
    let list_fields: Vec<serde_json::Value> = entity
        .list_fields
        .iter()
        .map(field_to_json)
        .collect();

  
    let mut sort_fields: Vec<&FieldInfo> = entity.fields.iter()
        .filter(|f| f.sortable && f.sort_priority > 0)
        .collect();
    sort_fields.sort_by(|a, b| b.sort_priority.cmp(&a.sort_priority));

   

    let has_default_sort_field = !sort_fields.is_empty();

    // default_sort_chain: 生成的 Rust 默认排序代码块（支持每个字段独立 asc/desc）
    let default_sort_chain = if sort_fields.is_empty() {
        String::new()
    } else {
        sort_fields.iter()
            .map(|f| {
                let col = format!("{}::Column::{}", entity.entity_mod_name(), f.pascal_name);
                if f.sort_order == "asc" {
                    format!("rmodel = rmodel.order_by_asc({});", col)
                } else {
                    format!("rmodel = rmodel.order_by_desc({});", col)
                }
            })
            .collect::<Vec<_>>()
            .join("\n\t")
    };

    // 表单字段（前端 Write.vue）
    let form_fields: Vec<serde_json::Value> = entity
        .form_fields
        .iter()
        .map(field_to_json)
        .collect();

    // ========== 前端模板展开的纯文本变量 ==========

    // 辅助函数：获取字段的实际组件类型（component_override 优先）
    let resolve_component = |f: &FieldInfo| -> String {
        if let Some(ref override_comp) = f.component_override {
            override_comp.clone()
        } else {
            f.frontend_component.clone()
        }
    };

    // list_columns: 表格列定义（前端不显示主键，由 index 列代替）
    let list_columns_text = entity.list_fields.iter()
        .filter(|f| !f.is_primary_key)
        .map(|f| {
            let label = format!("t('menuuser.{}')", f.name);
            let comp = resolve_component(f);
            let is_image = comp == "Upload" && ["image", "img", "avatar", "pic", "cover", "photo", "thumbnail"].iter().any(|kw| f.name.contains(kw));
            let sort_attr = if f.sortable { ",\n    sortable: true" } else { "" };
            if is_image {
                format!("  {{\n    field: '{}',\n    label: {},\n    width: 100{},{}    slots: {{\n      default: (data: any) => data.row.{} ? <ElImage style=\"width: 50px; height: 50px; border-radius: 4px\" src={{data.row.{}}} fit=\"cover\" /> : null \n    }}\n  }},", f.name, label, sort_attr, if f.sortable { "" } else { "\n    " }, f.name, f.name)
            } else {
                match comp.as_str() {
                    "DatePicker" => format!("  {{\n    field: '{}',\n    label: {},\n    width: 180{}\n  }},", f.name, label, sort_attr),
                    "Switch" => format!("  {{\n    field: '{}',\n    label: {},\n    width: 100{},{}    slots: {{\n      default: (data: any) => data.row.{} ? <el-tag type=\"success\">启用</el-tag> : <el-tag type=\"info\">禁用</el-tag>\n    }}\n  }},", f.name, label, sort_attr, if f.sortable { "" } else { "\n    " }, f.name),
                    "Select" | "Radio" => format!("  {{\n    field: '{}',\n    label: {},\n    width: 120{}\n  }},", f.name, label, sort_attr),
                    _ => format!("  {{\n    field: '{}',\n    label: {}{}\n  }},", f.name, label, sort_attr),
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // ========== BelongsTo 相关代码生成 ==========

    // 收集所有 BelongsTo 关系（去重）
    let mut seen_targets = Vec::new();
    let belongs_to_rels: Vec<&RelationInfo> = entity.belongs_to_fields.iter()
        .filter_map(|f| f.relation.as_ref())
        .filter(|r| matches!(r.kind, RelationKind::BelongsTo))
        .filter(|r| {
            if seen_targets.contains(&r.target_entity) {
                false
            } else {
                seen_targets.push(r.target_entity.clone());
                true
            }
        })
        .collect();

    // belongs_to_api_imports: 远程搜索 API import 语句
    let belongs_to_api_imports = belongs_to_rels.iter()
        .map(|r| {
            let target_api_dir = format!("{}/{}", r.target_module, r.target_model);
            format!("import {{ Get{}List }} from '@/api/{}'", r.target_model, target_api_dir)
        })
        .collect::<Vec<_>>()
        .join("\n");

    // belongs_to_setup: remote-method 函数和响应式状态
    let belongs_to_setup = belongs_to_rels.iter()
        .map(|r| {
            let target_var = camel_case(&r.target_entity);
            let target_model = &r.target_model;
            let display_field = r.display_field.as_deref().unwrap_or("name");
            format!(
                r#"const {targetVar}List = ref<any[]>([])
const loading{targetModel} = ref(false)
const search{targetModel} = async (query: string) => {{
  loading{targetModel}.value = true
  try {{
    const res = await Get{targetModel}List({{ page_size: 50, {display_field}: query || undefined }})
    {targetVar}List.value = res.data.list || []
  }} finally {{
    loading{targetModel}.value = false
  }}
}}"#,
                targetVar = target_var,
                targetModel = target_model,
                display_field = display_field,
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    // search_columns: 搜索表单 schema
    let search_columns_text = entity.searchable_fields.iter()
        .map(|f| {
            let comp = resolve_component(f);
            let component = match comp.as_str() {
                "DatePicker" => "DatePicker",
                "InputNumber" => "InputNumber",
                "Switch" => "Switch",
                "Select" => "Select",
                "Radio" => "Radio",
                _ => "Input",
            };
            if component == "Select" || component == "Radio" {
                let opts = sample_options_for_field(&f.name);
                format!("  {{\n    field: '{}',\n    label: t('menuuser.{}'),\n    component: '{}',\n    componentProps: {{\n      options: {}\n    }}\n  }},", f.name, f.name, component, opts)
            } else {
                format!("  {{\n    field: '{}',\n    label: t('menuuser.{}'),\n    component: '{}'\n  }},", f.name, f.name, component)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Upload 组件的 componentProps 模板（复用于必填/选填）
    let upload_comp_props = r#"{
      action: '/api/sys/upload',
      accept: 'image/*',
      limit: 1,
      headers: { Authorization: userStore.getTokenType + ' ' + userStore.getToken },
      on: {
        success: (response: any, uploadFile: any) => {
          uploadFile.url = response.data
        },
        exceed: () => {
          ElMessage.warning('最多只能上传1个文件')
        },
        beforeUpload: (rawFile: any) => {
          if (rawFile.size / 1024 / 1024 > 2) {
            ElMessage.error('文件大小不能超过2MB')
            return false
          }
          return true
        }
      },
      slots: {
        trigger: () => <el-button type='primary'>点击上传</el-button>,
        tip: () => <div style='color: #999; font-size: 12px'>只能上传图片文件，且不超过2MB</div>
      }
    }"#;

    // form_columns: 表单字段 schema
    let form_columns_text = entity.form_fields.iter()
        .map(|f| {
            let comp = resolve_component(f);
            let component = match comp.as_str() {
                "DatePicker" => "DatePicker",
                "InputNumber" => "InputNumber",
                "InputTextarea" => "InputTextarea",
                "Switch" => "Switch",
                "Select" => "Select",
                "Radio" => "Radio",
                "InputPassword" => "InputPassword",
                "Checkbox" => "Checkbox",
                "Upload" => "Upload",
                "Textarea" => "InputTextarea",
                "TimePicker" => "TimePicker",
                "DateTimePicker" => "DateTimePicker",
                _ => "Input",
            };
            let has_options = component == "Select" || component == "Radio";
            let is_belongs_to = has_options && f.relation.is_some() && matches!(f.relation.as_ref().unwrap().kind, RelationKind::BelongsTo);
            let is_upload = component == "Upload";

            if is_belongs_to {
                // BelongsTo 字段 → Select 远程搜索
                let rel = f.relation.as_ref().unwrap();
                let target_var = camel_case(&rel.target_entity);
                let target_model = &rel.target_model;
                format!("  {{\
    field: '{}',\
    label: t('menuuser.{}'),\
    component: 'Select',\
    componentProps: {{\
      remote: true,\
      remoteMethod: search{},\
      options: {}List\
    }}\
  }},", f.name, f.name, target_model, target_var)
            } else if is_upload {
                // Upload 字段
                format!("  {{\
    field: '{}',\
    label: t('menuuser.{}'),\
    component: 'Upload',\
    componentProps: {}\
  }},", f.name, f.name, upload_comp_props)
            } else if has_options {
                let opts = sample_options_for_field(&f.name);
                format!("  {{\
    field: '{}',\
    label: t('menuuser.{}'),\
    component: '{}',\
    componentProps: {{\
      options: {}\
    }}\
  }},", f.name, f.name, component, opts)
            } else {
                format!("  {{\
    field: '{}',\
    label: t('menuuser.{}'),\
    component: '{}'\
  }},", f.name, f.name, component)
            }
        })
         .collect::<Vec<_>>()
        .join("\n");

    // form_rules: 独立验证规则对象
    let form_rules_entries: Vec<String> = entity.form_fields.iter()
        .filter_map(|f| build_field_rules_entry(f, field_validation_map))
        .collect();
    let form_rules_text = if form_rules_entries.is_empty() {
        String::new()
    } else {
        format!("const rules = reactive({{\n{}\n}})", form_rules_entries.join("\n"))
    };

    // add_defaults: 新增时的默认值
    let add_defaults_text = entity.form_fields.iter()
        .map(|f| {
            let default = match f.base_type.as_str() {
                "i32" | "i64" | "u32" | "u64" => "0",
                "f32" | "f64" => "0",
                "bool" => "false",
                _ => "''",
            };
            format!("    {}: {},", f.name, default)
        })
        .collect::<Vec<_>>()
        .join("\n");

    // detail_fields: 详情展示字段
    let detail_fields_text = entity.detail_fields.iter()
        .map(|f| {
            let label = format!("t('menuuser.{}')", f.name);
            let comp = resolve_component(f);
            let name = &f.name;
            if comp == "Upload" && ["image", "img", "avatar", "pic", "cover", "photo", "thumbnail"].iter().any(|kw| name.contains(kw)) {
                format!(
                    "      <div class=\"detail-item\">\n        <span class=\"label\">{{{{ {} }}}}</span>\n        <span class=\"value\">\n          <ElImage v-if=\"currentRow.{}\" style=\"width: 100px; height: 100px; border-radius: 4px\" :src=\"currentRow.{}\" fit=\"cover\" />\n          <span v-else>-</span>\n        </span>\n      </div>",
                    label, name, name
                )
            } else if comp == "DatePicker" || comp == "DateTimePicker" {
                format!(
                    "      <div class=\"detail-item\">\n        <span class=\"label\">{{{{ {} }}}}</span>\n        <span class=\"value\">{{{{ formatToDateTime(currentRow.{}) }}}}</span>\n      </div>",
                    label, name
                )
            } else {
                format!(
                    "      <div class=\"detail-item\">\n        <span class=\"label\">{{{{ {} }}}}</span>\n        <span class=\"value\">{{{{ currentRow.{} }}}}</span>\n      </div>",
                    label, name
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Upload 字段检测
    let upload_field_names: Vec<String> = entity.form_fields.iter()
        .filter(|f| {
            let comp = resolve_component(f);
            comp == "Upload"
        })
        .map(|f| f.name.clone())
        .collect();
    let has_upload_field = !upload_field_names.is_empty();

    // upload_watch_text: watch 中将字符串转为 Upload file-list 的代码
    let upload_watch_text = if has_upload_field {
        upload_field_names.iter()
            .map(|name| format!(
                "    if (currentRow.{name} && typeof currentRow.{name} === 'string') {{\n      currentRow.{name} = currentRow.{name}.split(',').filter(Boolean).map((url: string) => ({{ url, name: url.split('/').pop() || url, status: 'success' }}))\n    }}",
                name = name
            ))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    // upload_submit_text: submit 中将 file-list 转回字符串的代码
    let upload_submit_text = if has_upload_field {
        upload_field_names.iter()
            .map(|name| format!(
                "  if (formData.{name} && Array.isArray(formData.{name})) {{\n    formData.{name} = formData.{name}.map((f: any) => f.url || f.response?.data || f.name).filter(Boolean).join(',')\n  }}",
                name = name
            ))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    // upload_imports_text: Upload 需要的额外导入
    let upload_imports_text = if has_upload_field {
        "import { ElMessage } from 'element-plus'\nimport { useUserStore } from '@/store/modules/user'".to_string()
    } else {
        String::new()
    };

    // upload_setup_text: Upload 需要的 setup 代码
    let upload_setup_text = if has_upload_field {
        "const userStore = useUserStore()".to_string()
    } else {
        String::new()
    };

    json!({
        "module_name": entity.module_name,
        "table_name": entity.table_name,
        "model_name": entity.model_name,
        "primary_key": entity.primary_key,
        "primary_key_pascal": entity.primary_key_pascal,
        "entity_mod_name": entity.entity_mod_name(),
        "args_file_name": entity.args_file_name(),
        "model_file_name": entity.model_file_name(),
        "service_file_name": entity.service_file_name(),
        "model_alias": entity.model_alias(),
        "resp_name": entity.resp_name(),
        "add_name": entity.add_name(),
        "edit_name": entity.edit_name(),
        "del_name": entity.del_name(),
        "search_name": entity.search_name(),
        "api_dir_name": entity.api_dir_name(),
        "view_dir_name": entity.view_dir_name(),
        "resp_fields": resp_fields,
        "search_fields": search_fields,
        "add_fields": add_fields,
        "edit_fields": edit_fields,
        "list_fields": list_fields,
        "form_fields": form_fields,
        // 前端模板展开的纯文本
        "list_columns": list_columns_text,
        "search_columns": search_columns_text,
        "form_columns": form_columns_text,
        "form_rules": form_rules_text,
        "add_defaults": add_defaults_text,
        "detail_fields": detail_fields_text,
        "data_scope_enabled": data_scope_enabled,
        "has_updated_at": entity.has_updated_at,
        "is_updated_at_optional": entity.is_updated_at_optional,
        "belongs_to_api_imports": belongs_to_api_imports,
        "belongs_to_setup": belongs_to_setup, 
        "has_default_sort_field": has_default_sort_field,
        "default_sort_chain": default_sort_chain,
        // Upload 相关
        "has_upload_field": has_upload_field,
        "upload_watch_text": upload_watch_text,
        "upload_submit_text": upload_submit_text,
        "upload_imports_text": upload_imports_text,
        "upload_setup_text": upload_setup_text,
    })
}

fn field_to_json(f: &FieldInfo) -> serde_json::Value {
    json!({
        "name": f.name,
        "rust_type": f.rust_type,
        "is_primary_key": f.is_primary_key,
        "is_optional": f.is_optional,
        "base_type": f.base_type,
        "pascal_name": f.pascal_name,
        "camel_name": f.camel_name,
        "is_id_field": f.is_id_field,
        "is_datetime": f.is_datetime,
        "frontend_component": f.frontend_component,
        "show_in_list": f.show_in_list,
        "show_in_search": f.show_in_search,
        "show_in_form": f.show_in_form,
        "show_in_detail": f.show_in_detail,
        "component_override": f.component_override,
        "sortable": f.sortable,
        "sort_priority": f.sort_priority,
        "sort_order": f.sort_order,
        "has_relation": f.relation.is_some(),
        "relation_kind": f.relation.as_ref().map(|r| format!("{:?}", r.kind)),
        "relation_target": f.relation.as_ref().map(|r| r.target_model.clone()),
        "relation_target_entity": f.relation.as_ref().map(|r| r.target_entity.clone()),
        "relation_display_field": f.relation.as_ref().and_then(|r| r.display_field.clone()),
        "fk_field": f.relation.as_ref().and_then(|r| r.fk_field.clone()),
    })
}

/// 根据字段名生成示例 options（仅用于 Select/Radio 的 demo 数据）
fn sample_options_for_field(name: &str) -> &'static str {
    if name.contains("status") || name.contains("type") || name.contains("level") || name.contains("category") {
        "[\n      { label: '选项一', value: 1 },\n      { label: '选项二', value: 2 }\n    ]"
    } else {
        "[\n      { label: '选项一', value: 1 },\n      { label: '选项二', value: 2 },\n      { label: '选项三', value: 3 }\n    ]"
    }
}

/// 根据字段名推断中文标签（用于翻译文件生成）
fn infer_field_label_zh(name: &str) -> String {
    // 常见字段名映射
    match name {
        "id" => "ID".to_string(),
        "title" | "name" => "名称".to_string(),
        "username" => "用户名".to_string(),
        "password" => "密码".to_string(),
        "nickname" => "昵称".to_string(),
        "email" => "邮箱".to_string(),
        "phone" | "phonenumber" | "mobile" => "电话".to_string(),
        "avatar" | "head_img" | "headimg" => "头像".to_string(),
        "status" => "状态".to_string(),
        "remark" | "description" | "desc" | "content" => "备注".to_string(),
        "sort" | "order_num" | "order" | "sort_order" => "排序".to_string(),
        "create_time" | "created_at" => "创建时间".to_string(),
        "update_time" | "updated_at" => "更新时间".to_string(),
        "create_by" | "created_by" => "创建人".to_string(),
        "update_by" | "updated_by" => "更新人".to_string(),
        "is_deleted" | "deleted" => "是否删除".to_string(),
        "sex" | "gender" => "性别".to_string(),
        "age" => "年龄".to_string(),
        "address" => "地址".to_string(),
        "url" | "link" | "href" => "链接".to_string(),
        "image" | "img" | "pic" | "picture" | "cover" | "photo" | "thumbnail" => "图片".to_string(),
        "file" | "attachment" => "文件".to_string(),
        "price" | "amount" | "cost" => "金额".to_string(),
        "count" | "num" | "quantity" | "qty" => "数量".to_string(),
        "total" => "合计".to_string(),
        "type" | "category" => "类型".to_string(),
        "level" => "级别".to_string(),
        "parent_id" | "parentid" | "pid" => "上级".to_string(),
        "key" | "code" => "编码".to_string(),
        "value" | "val" => "值".to_string(),
        "label" | "text" => "标签".to_string(),
        "icon" => "图标".to_string(),
        "path" | "route" => "路径".to_string(),
        "method" => "方法".to_string(),
        "permission" => "权限".to_string(),
        "role" => "角色".to_string(),
        "department" | "dept" => "部门".to_string(),
        "leader" => "领导".to_string(),
        "enabled" | "is_enabled" => "是否启用".to_string(),
        "disabled" | "is_disabled" => "是否禁用".to_string(),
        "visible" | "is_visible" => "是否可见".to_string(),
        "required" | "is_required" => "是否必填".to_string(),
        "start_time" | "begin_time" => "开始时间".to_string(),
        "end_time" | "finish_time" => "结束时间".to_string(),
        "expire_time" => "过期时间".to_string(),
        "duration" => "时长".to_string(),
        "rate" | "ratio" => "比率".to_string(),
        "weight" => "权重".to_string(),
        "size" => "大小".to_string(),
        "color" => "颜色".to_string(),
        "version" => "版本".to_string(),
        "source" => "来源".to_string(),
        "target" => "目标".to_string(),
        "action" => "操作".to_string(),
        "index" => "序号".to_string(),
        _ => {
            // snake_case → 去掉 _id/_code 后缀，按 _ 分割，尝试推断
            let clean = name.trim_end_matches("_id")
                           .trim_end_matches("_code")
                           .trim_end_matches("_type")
                           .trim_end_matches("_name")
                           .trim_end_matches("_time")
                           .trim_end_matches("_at");
            if clean != name {
                // 递归推断去掉后缀后的部分
                let base = infer_field_label_zh(clean);
                if name.ends_with("_id") { format!("{}ID", base) }
                else if name.ends_with("_code") { format!("{}编码", base) }
                else if name.ends_with("_type") { format!("{}类型", base) }
                else if name.ends_with("_name") { format!("{}名称", base) }
                else if name.ends_with("_time") || name.ends_with("_at") { format!("{}时间", base) }
                else { base }
            } else {
                // 无法推断，把 snake_case 转为首字母大写作为兜底
                name.split('_')
                    .map(|s| {
                        let mut c = s.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
        }
    }
}

/// 根据字段名推断英文标签（用于翻译文件生成）
fn infer_field_label_en(name: &str) -> String {
    match name {
        "id" => "ID".to_string(),
        _ => {
            // snake_case → Title Case
            name.split('_')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

/// 生成翻译片段内容（zh-CN 和 en）
fn generate_locale_snippet(entity: &EntityInfo) -> (String, String) {
    // 已知的 menuuser 通用 key（不需要重复生成）
    let known_keys = [
        "index", "action", "username", "password", "role", "remark",
        "email", "createTime", "department", "status", "enable", "disable",
        "nickname", "phone", "sex", "leader", "account",
    ];
    
    let mut zh_lines = Vec::new();
    let mut en_lines = Vec::new();
    
    for f in &entity.fields {
        // 跳过已知通用 key 和主键
        if known_keys.contains(&f.name.as_str()) || f.is_primary_key {
            continue;
        }
        let zh_label = infer_field_label_zh(&f.name);
        let en_label = infer_field_label_en(&f.name);
        zh_lines.push(format!("    {}: '{}',", f.name, zh_label));
        en_lines.push(format!("    {}: '{}',", f.name, en_label));
    }
    
    let zh_content = format!(
        "// ====== {} 翻译片段 ======\n// 请将以下内容合并到 src/locales/zh-CN.ts\n\n{}\n",
        entity.model_name,
        zh_lines.join("\n")
    );
    let en_content = format!(
        "// ====== {} Translation Snippet ======\n// Merge the following into src/locales/en.ts\n\n{}\n",
        entity.model_name,
        en_lines.join("\n")
    );
    
    (zh_content, en_content)
}

/// 渲染 Tera 内联模板（用于后端 Rust 模板，不含 Vue {{ }} 冲突）
fn render_template(template_str: &str, ctx: &serde_json::Value) -> Result<String> { 

    let context = Context::from_serialize(ctx)
        .map_err(|e| {
            tracing::error!("[CodeGen] Tera context 构建失败: {}", e);
            crate::common::error::Error::Message(format!("Tera context error: {}", e))
        })?; 

    let result = Tera::one_off(template_str, &context, false)
        .map_err(|e| {  
            crate::common::error::Error::Message(format!("Tera render error: {}", e))
        })?; 

    let cleaned = clean_output(&result);
    Ok(cleaned)
}

/// 渲染前端模板文件（Vue/TS/TSX）
/// 不使用 Tera 引擎（Tera 无法解析 Vue/TSX 中的 HTML 标签和 JSX 语法），
/// 改用简单字符串替换：将 {{var}} 占位符替换为 ctx 中对应的值。
/// 循环/条件逻辑在 Rust 代码中展开，模板文件只包含纯文本 + {{var}} 占位符。
fn render_file_template(template_name: &str, ctx: &serde_json::Value) -> Result<String> {
 

    // 获取模板文件路径
    let template_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/common/codegen/tera_templates");

    let template_path = template_dir.join(template_name);

 

    let mut template_content = std::fs::read_to_string(&template_path)
        .map_err(|e| {
            tracing::error!("[CodeGen] 读取模板文件失败: {:?} => {}", template_path, e);
            crate::common::error::Error::Message(
                format!("Failed to read template file {:?}: {}", template_path, e)
            )
        })?;
 

    // 从 ctx 中提取所有简单变量（字符串值），执行字符串替换
    if let Some(obj) = ctx.as_object() {
        for (key, value) in obj {
            // 只替换字符串值，跳过数组/对象（它们通过循环在 Rust 代码中展开为文本）
            if let Some(s) = value.as_str() {
                let placeholder = format!("{{{{{}}}}}", key); // {{key}}
                template_content = template_content.replace(&placeholder, s);
            }
        }
    }

    let cleaned = clean_output(&template_content);
    Ok(cleaned)
}

/// 清理模板输出中的多余空行
fn clean_output(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let mut result = Vec::new();
    let mut empty_count = 0;

    for line in lines {
        if line.trim().is_empty() {
            empty_count += 1;
            if empty_count <= 2 {
                result.push(line);
            }
        } else {
            empty_count = 0;
            result.push(line);
        }
    }

    result.join("\n")
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::codegen::parser::parse_entity_content;

    #[test]
    fn test_field_to_json_fk_field() {
        let content = r#"
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "test_article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub category_id: i64,
    pub title: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::test_category::Entity",
        from = "Column::CategoryId",
        to = "super::test_category::Column::Id"
    )]
    TestCategory,
}

impl ActiveModelBehavior for ActiveModel {}
"#;
        let entity = parse_entity_content(content, "test_article", "test");
        let cat_field = entity.fields.iter().find(|f| f.name == "category_id").unwrap();
        let json_val = field_to_json(cat_field);

        // 验证扁平字段
        assert_eq!(json_val["fk_field"].as_str(), Some("category_id"), "fk_field 应为 'category_id'，实际: {:?}", json_val["fk_field"]);
        assert_eq!(json_val["has_relation"].as_bool(), Some(true), "has_relation 应为 true");
        assert_eq!(json_val["relation_target"].as_str(), Some("TestCategory"), "relation_target 应为 'TestCategory'");
        // 确保不是嵌套的 relation 对象
        assert!(json_val.get("relation").is_none(), "不应存在嵌套的 'relation' 字段");
    }

    #[test]
    fn test_tera_render_inline_templates() {
        let content = r#"
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.19

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "test_api")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
"#;
        let entity = parse_entity_content(content, "test_api", "test");
        let empty_map = std::collections::HashMap::new();
        let ctx = build_tera_context(&entity, &empty_map, false);

        // Test inline (Rust) templates
        let templates = [
            ("TEMPLATE_ARGS", TEMPLATE_ARGS),
            ("TEMPLATE_MODEL", TEMPLATE_MODEL),
            ("TEMPLATE_SERVICE", TEMPLATE_SERVICE),
            ("TEMPLATE_API_ROUTE_FN", TEMPLATE_API_ROUTE_FN),
            ("TEMPLATE_API_NEST", TEMPLATE_API_NEST),
            ("TEMPLATE_MODEL_MOD", TEMPLATE_MODEL_MOD),
            ("TEMPLATE_ARGS_MOD", TEMPLATE_ARGS_MOD),
            ("TEMPLATE_SERVICE_MOD", TEMPLATE_SERVICE_MOD),
            ("TEMPLATE_MODULE_MOD", TEMPLATE_MODULE_MOD),
            ("TEMPLATE_ARGS_MOD_FULL", TEMPLATE_ARGS_MOD_FULL),
            ("TEMPLATE_MODEL_MOD_FULL", TEMPLATE_MODEL_MOD_FULL),
        ];

        for (name, tpl) in &templates {
            let context = tera::Context::from_serialize(&ctx)
                .unwrap_or_else(|e| panic!("Context error for {}: {}", name, e));
            let result = Tera::one_off(tpl, &context, false)
                .unwrap_or_else(|e| panic!("Render error for {}: {}", name, e));
            println!("=== {} ===\n{}\n", name, result);
        }
    }

    #[test]
    fn test_tera_render_file_templates() {
        let content = r#"
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.19

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "test_api")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
"#;
        let entity = parse_entity_content(content, "test_api", "test");
        let empty_map = std::collections::HashMap::new();
        let ctx = build_tera_context(&entity, &empty_map, false);

        // Test file-based (frontend) templates
        let file_templates = [
            "frontend_api.tera",
            "frontend_view.tera",
            "frontend_write.tera",
        ];

        for name in &file_templates {
            let result = render_file_template(name, &ctx)
                .unwrap_or_else(|e| panic!("Render error for {}: {}", name, e));
            println!("=== {} ===\n{}\n", name, result);
        }
    }

    #[test]
    fn test_tera_render_file_templates_with_upload() {
        let content = r#"
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.19

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "test_photo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub cover: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
"#;
        let entity = parse_entity_content(content, "test_photo", "test");
        let empty_map = std::collections::HashMap::new();
        let ctx = build_tera_context(&entity, &empty_map, false);

        let result = render_file_template("frontend_write.tera", &ctx)
            .unwrap_or_else(|e| panic!("Render error: {}", e));
        println!("=== frontend_write.tera (with upload) ===\n{}\n", result);

        // 验证没有残留 Tera 语法
        assert!(!result.contains("{% if"), "不应包含 Tera 条件语法: {}", result);
        assert!(!result.contains("{% endif"), "不应包含 Tera endif 语法: {}", result);
        // 验证 Upload 相关代码存在
        assert!(result.contains("ElMessage"), "应包含 ElMessage 导入");
        assert!(result.contains("useUserStore"), "应包含 useUserStore 导入");
        assert!(result.contains("userStore"), "应包含 userStore 变量");
        assert!(result.contains("component: 'Upload'"), "应包含 Upload 组件");
        // 验证逗号正确：不应出现 'Upload'componentProps（缺少逗号）
        assert!(!result.contains("'Upload'componentProps"), "Upload 后缺少逗号");
    }

    /// 重新生成指定模块下所有实体的代码（用于模板变更后刷新生成文件）
    /// 运行: cargo test regenerate_test_entities -- --nocapture
    #[test]
    fn regenerate_test_entities() {
        let model_base = std::path::Path::new("src/model");
        let modules = ["test"];
        let entity_dirs = [
            "test_api", "test_article", "test_category", "test_data_scope",
        ];

        for module in &modules {
            for table in &entity_dirs {
                let entity_path = model_base
                    .join(module)
                    .join("entity")
                    .join(format!("{}.rs", table));
                if !entity_path.exists() {
                    println!("  [跳过] 实体文件不存在: {}", entity_path.display());
                    continue;
                }
                let content = std::fs::read_to_string(&entity_path)
                    .expect(&format!("读取失败: {}", entity_path.display()));
                let entity = parse_entity_content(&content, table, module);

                // 生成代码
                let result = generate_code(&entity, None, None)
                    .expect(&format!("生成失败: {}/{}", module, table));

                // 写入文件（service 文件如果已存在则跳过，避免覆盖手写代码）
                for file in &result.files {
                    let abs_path = std::path::Path::new(&file.path);
                    // service 文件可能包含手写路由函数，已存在时跳过
                    let is_service_file = file.path.contains("/service/");
                    if is_service_file && abs_path.exists() {
                        println!("  [跳过] service 文件已存在，不覆盖: {}", file.path);
                        continue;
                    }
                    if let Some(parent) = abs_path.parent() {
                        std::fs::create_dir_all(parent)
                            .expect(&format!("创建目录失败: {}", parent.display()));
                    }
                    std::fs::write(&abs_path, &file.content)
                        .expect(&format!("写入失败: {}", abs_path.display()));
                    println!("  [OK] {} ({})", file.path, file.description);
                }
                println!("  => {} 生成完成，共 {} 个文件", table, result.files.len());
            }
        }
    }
}
