use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::debug;

/// 解析后的字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// 字段名 (snake_case)，如 dict_id, dict_name
    pub name: String,
    /// Rust 类型，如 i64, String, DateTime, Option<String>
    pub rust_type: String,
    /// 是否主键
    pub is_primary_key: bool,
    /// 是否 Optional (Option<T>)
    pub is_optional: bool,
    /// 去掉 Option 后的基础类型
    pub base_type: String,
    /// 字段名的 PascalCase 形式，如 DictId, DictName
    pub pascal_name: String,
    /// 字段名的 camelCase 形式，如 dictId, dictName
    pub camel_name: String,
    /// 是否是 id 类型 (i64 且为主键)
    pub is_id_field: bool,
    /// 是否是时间字段 (DateTime / created_at / updated_at)
    pub is_datetime: bool,
    /// 前端组件类型推断
    pub frontend_component: String,
    /// 是否在列表中显示
    pub show_in_list: bool,
    /// 是否可搜索
    pub show_in_search: bool,
    /// 是否在表单中
    pub show_in_form: bool,
    /// 用户自定义组件覆盖，如 Select, Radio 等
    pub component_override: Option<String>,
    /// 是否在详情页显示
    pub show_in_detail: bool,
    /// 是否可排序
    pub sortable: bool,
    /// 排序优先级（0=无优先级，值越高排序优先级越高）
    pub sort_priority: u32,
    /// 排序方向（"asc" 或 "desc"，默认 "desc"）
    pub sort_order: String,
    /// 用户自定义标签（覆盖自动推断的翻译标签），如 "字典名称"
    pub label: Option<String>,
    /// 关联关系信息（BelongsTo 等）
    pub relation: Option<RelationInfo>,
}

/// 关联关系类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationKind {
    BelongsTo,
    HasMany,
    ManyToMany,
}

/// 关联关系信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationInfo {
    pub kind: RelationKind,
    /// 关系变体名，如 SysRole
    pub variant_name: String,
    /// 目标模块名
    pub target_module: String,
    /// 目标实体表名（如 sys_role）
    pub target_entity: String,
    /// 目标实体 PascalCase 名（如 SysRole）
    pub target_model: String,
    /// 目标实体显示字段（如 role_name），仅 BelongsTo 有效
    pub display_field: Option<String>,
    /// 对于 BelongsTo，外键字段名（如 role_id）
    pub fk_field: Option<String>,
    /// 对于 HasMany，该关系在本实体中的字段名（即在对方实体中对应的外键）
    pub fk_in_target: Option<String>,
}

/// 用户对单个字段的配置（前端传入）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConfig {
    pub field_name: String,
    pub show_in_list: bool,
    pub show_in_search: bool,
    pub show_in_form: bool,
    /// 覆盖前端组件类型，如 Select, Radio, DatePicker 等
    pub component: Option<String>,
    /// 是否在详情页显示
    pub show_in_detail: bool,
    /// 是否可排序
    pub sortable: Option<bool>,
    /// 排序优先级（值越高排序越靠前）
    pub sort_priority: Option<i32>,
    /// 排序方向（"asc" 或 "desc"）
    pub sort_order: Option<String>,
    /// 用户自定义标签（覆盖自动推断的翻译标签），如 "字典名称"
    pub label: Option<String>,
    /// 是否必填（用于表单验证）
    pub required: Option<bool>,
    /// 最小长度（用于表单验证）
    pub min_length: Option<i32>,
    /// 最大长度（用于表单验证）
    pub max_length: Option<i32>,
    /// 正则表达式（用于表单验证）
    pub pattern: Option<String>,
    /// 正则验证失败提示
    pub pattern_message: Option<String>,
    /// 关联目标（BelongsTo 时可选覆盖）
    pub relation_target: Option<String>,
    /// 关联显示字段（BelongsTo 时可选覆盖）
    pub relation_display_field: Option<String>,
}

/// 解析后的 Entity 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    /// 表名，如 test_api
    pub table_name: String,
    /// 模块名，如 test
    pub module_name: String,
    /// Entity 结构体的 Model 名 (PascalCase)，如 TestApi
    pub model_name: String,
    /// 主键字段名，如 id 或 dict_id
    pub primary_key: String,
    /// 主键的 PascalCase
    pub primary_key_pascal: String,
    /// 所有字段
    pub fields: Vec<FieldInfo>,
    /// 可搜索字段 (String/Option<String> 类型，排除 id/时间)
    pub searchable_fields: Vec<FieldInfo>,
    /// 可在列表中展示的字段 (排除 created_at/updated_at)
    pub list_fields: Vec<FieldInfo>,
    /// 表单字段 (用于 Add/Edit，排除主键/created_at/updated_at)
    pub form_fields: Vec<FieldInfo>,
    /// 详情页字段 (show_in_detail == true)
    pub detail_fields: Vec<FieldInfo>,
    /// 所有关联关系
    pub relations: Vec<RelationInfo>,
    /// 适合做 Select 下拉的 BelongsTo 字段
    pub belongs_to_fields: Vec<FieldInfo>,
    /// 是否包含 updated_at 字段
    pub has_updated_at: bool,
    /// updated_at 是否是 Option<DateTime>
    pub is_updated_at_optional: bool,
    /// 是否有数据权限（同时包含 dept_id 和 owner_id 字段）
    pub has_data_scope: bool,
}

impl EntityInfo {
    /// Resp 结构体名，如 TestApiResp
    pub fn resp_name(&self) -> String {
        format!("{}Resp", self.model_name)
    }
    /// Add 结构体名
    pub fn add_name(&self) -> String {
        format!("{}Add", self.model_name)
    }
    /// Edit 结构体名
    pub fn edit_name(&self) -> String {
        format!("{}Edit", self.model_name)
    }
    /// Del 结构体名
    pub fn del_name(&self) -> String {
        format!("{}Del", self.model_name)
    }
    /// Search 结构体名
    pub fn search_name(&self) -> String {
        format!("{}Search", self.model_name)
    }
    /// Model alias，如 TestApiModel
    pub fn model_alias(&self) -> String {
        format!("{}Model", self.model_name)
    }
    /// Entity 文件中的 module 名 (snake_case 表名)
    pub fn entity_mod_name(&self) -> String {
        self.table_name.clone()
    }
    /// args 文件名
    pub fn args_file_name(&self) -> String {
        format!("a{}", self.table_name)
    }
    /// model 文件名
    pub fn model_file_name(&self) -> String {
        format!("m{}", self.table_name)
    }
    /// service 文件名
    pub fn service_file_name(&self) -> String {
        format!("s_{}", self.table_name)
    }
    /// API 路径前缀 (camelCase)，如 testApi
    pub fn api_path_name(&self) -> String {
        self.camel_case(&self.table_name)
    }
    /// 前端 API 目录名 (module/model_name，如 test/TestArticle)
    pub fn api_dir_name(&self) -> String {
        format!("{}/{}", self.module_name, self.model_name)
    }
    /// 前端 API 目录名 - 旧格式兼容 (去掉下划线，全小写)，如 testapi
    pub fn api_dir_name_flat(&self) -> String {
        self.table_name.replace("_", "")
    }
    /// 前端 View 目录名 (PascalCase)，如 TestApi
    pub fn view_dir_name(&self) -> String {
        self.model_name.clone()
    }

    fn pascal_case(&self, s: &str) -> String {
        s.split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect()
    }

    fn camel_case(&self, s: &str) -> String {
        let pascal = self.pascal_case(s);
        if pascal.is_empty() {
            return pascal;
        }
        let mut c = pascal.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }
}

/// 从 Entity 文件内容解析字段信息
pub fn parse_entity_content(content: &str, table_name: &str, module_name: &str) -> EntityInfo {
    let mut fields = Vec::new();
    let mut primary_key = String::from("id");
    let mut primary_key_pascal = String::from("Id");

    // 解析 Model 结构体中的字段
    let in_model = find_model_struct(content);

    for line in in_model.lines() {
        let line = line.trim();

        // 检测主键
        if line.contains("#[sea_orm(primary_key") {
            // 下一行是主键字段
            if let Some(pk_field) = parse_field_line(get_next_line(&in_model, line)) {
                primary_key = pk_field.name.clone();
                primary_key_pascal = pk_field.pascal_name.clone();
                let mut pk = pk_field;
                pk.is_primary_key = true;
                pk.is_id_field = pk.base_type == "i64";
                fields.push(pk);
                continue;
            }
        }

        // 检测普通字段 (pub xxx: Type)，跳过已被主键分支处理的行
        if line.starts_with("pub ") && !line.contains("primary_key") {
            if let Some(field) = parse_field_line(line) {
                // 跳过与主键同名的字段（已在主键分支中添加）
                if field.name != primary_key {
                    fields.push(field);
                }
            }
        }
    }

    let model_name = pascal_case(table_name);

    // 为每个字段设置 show_in_list, show_in_search, show_in_form 默认值
    for field in &mut fields {
        if field.is_primary_key {
            field.frontend_component = "Input".to_string();
            field.show_in_list = true;
            field.show_in_search = false;
            field.show_in_form = false;
            field.show_in_detail = true;
        } else if field.is_datetime {
            field.frontend_component = "DatePicker".to_string();
            field.show_in_list = false;
            field.show_in_search = false;
            field.show_in_form = false;
            field.show_in_detail = true;
        } else {
            field.show_in_list = true;
            field.show_in_search = field.base_type == "String";
            field.show_in_form = true;
            field.show_in_detail = true;
            field.frontend_component = infer_frontend_component(&field.base_type, &field.name);
        }
    }

    // 解析 Relation 枚举
    let mut relations = parse_relation_enum(content, table_name, module_name);

    let mut belongs_to_fields: Vec<FieldInfo> = Vec::new();
    for field in &mut fields {
        if field.name.ends_with("_id") && !field.is_primary_key {
            let has_existing = relations.iter().any(|r| {
                matches!(r.kind, RelationKind::BelongsTo)
                    && r.fk_field.as_deref() == Some(&field.name)
            });
            if !has_existing {
                let target_name = field.name.strip_suffix("_id").unwrap_or(&field.name);
                let target_entity = target_name;
                let target_model = pascal_case(target_name);
                let display_field = infer_display_field(target_name);
                relations.push(RelationInfo {
                    kind: RelationKind::BelongsTo,
                    variant_name: format!("{}Relation", target_model),
                    target_module: module_name.to_string(),
                    target_entity: target_entity.to_string(),
                    target_model,
                    display_field: Some(display_field),
                    fk_field: Some(field.name.clone()),
                    fk_in_target: None,
                });
            }
            // 把关系信息挂到字段上
            if let Some(rel) = relations.iter().find(|r| {
                matches!(r.kind, RelationKind::BelongsTo)
                    && r.fk_field.as_deref() == Some(&field.name)
            }) {
                field.relation = Some(rel.clone());
                field.frontend_component = "Select".to_string();
            }
            belongs_to_fields.push(field.clone());
        }
    }

    // 为列表字段设置默认排序优先级：
    // - sort / order_num / sort_order 等排序专用字段优先级最高（1），方向 asc
    // - weight / priority 等权重字段优先级次高（2），方向 desc
    // - 其余 show_in_list 的非主键字段不设默认排序（priority=0），由 field_config 手动指定
    // - created_at / updated_at 也不设默认排序，作为 fallback 由模板处理
    // 这样默认排序链只包含真正需要的字段，而不是所有列表字段
    for field in &mut fields {
        if ["sort", "order_num", "sort_order"].contains(&field.name.as_str()) {
            field.sortable = true;
            field.sort_priority = 1;
            field.sort_order = "asc".to_string();
        } else if ["weight", "priority"].contains(&field.name.as_str()) {
            field.sortable = true;
            field.sort_priority = 2;
            field.sort_order = "desc".to_string();
        }
    }

    // 构建分类字段列表（此时 fields 已包含正确的 sort_priority）
    let searchable_fields: Vec<FieldInfo> = fields
        .iter()
        .filter(|f| f.show_in_search)
        .cloned()
        .collect();

    let list_fields: Vec<FieldInfo> = fields.iter().filter(|f| f.show_in_list).cloned().collect();

    let form_fields: Vec<FieldInfo> = fields.iter().filter(|f| f.show_in_form).cloned().collect();

    let detail_fields: Vec<FieldInfo> = fields
        .iter()
        .filter(|f| f.show_in_detail)
        .cloned()
        .collect();

    let has_updated_at = fields.iter().any(|f| f.name == "updated_at");
    let is_updated_at_optional = fields
        .iter()
        .find(|f| f.name == "updated_at")
        .map(|f| f.is_optional)
        .unwrap_or(false);

    // 检测是否包含数据隔离字段（dept_id + owner_id）
    let has_data_scope =
        fields.iter().any(|f| f.name == "dept_id") && fields.iter().any(|f| f.name == "owner_id");

    EntityInfo {
        table_name: table_name.to_string(),
        module_name: module_name.to_string(),
        model_name,
        primary_key,
        primary_key_pascal,
        fields,
        searchable_fields,
        list_fields,
        form_fields,
        detail_fields,
        relations,
        belongs_to_fields,
        has_updated_at,
        is_updated_at_optional,
        has_data_scope,
    }
}

/// 将用户的字段配置应用到 EntityInfo，覆盖默认推断
pub fn apply_field_config(entity: &mut EntityInfo, configs: &[FieldConfig]) {
    for config in configs {
        if let Some(field) = entity
            .fields
            .iter_mut()
            .find(|f| f.name == config.field_name)
        {
            field.show_in_list = config.show_in_list;
            field.show_in_search = config.show_in_search;
            field.show_in_form = config.show_in_form;
            field.show_in_detail = config.show_in_detail;
            field.component_override = config.component.clone();
            if let Some(sortable) = config.sortable {
                field.sortable = sortable;
            }
            if let Some(priority) = config.sort_priority {
                field.sort_priority = priority as u32;
            }
            if let Some(ref order) = config.sort_order {
                field.sort_order = order.clone();
            }
            if let Some(ref label) = config.label {
                field.label = Some(label.clone());
            }
        } else {
            debug!(
                "[DEBUG apply_field_config] WARNING: field '{}' not found in entity!",
                config.field_name
            );
        }
    }
    // 重新根据 show_in_* 标记计算分类字段列表
    entity.searchable_fields = entity
        .fields
        .iter()
        .filter(|f| f.show_in_search)
        .cloned()
        .collect();
    entity.list_fields = entity
        .fields
        .iter()
        .filter(|f| f.show_in_list)
        .cloned()
        .collect();
    entity.form_fields = entity
        .fields
        .iter()
        .filter(|f| f.show_in_form)
        .cloned()
        .collect();
    entity.detail_fields = entity
        .fields
        .iter()
        .filter(|f| f.show_in_detail)
        .cloned()
        .collect();
}

/// 扫描指定模块目录下的所有 Entity 文件（递归遍历子目录）
pub fn scan_entities(model_base_path: &str, module_name: &str) -> Vec<EntityInfo> {
    let entity_dir = Path::new(model_base_path).join(module_name).join("entity");

    let mut entities = Vec::new();
    scan_entity_dir_recursive(&entity_dir, module_name, &mut entities);
    entities
}

fn scan_entity_dir_recursive(dir: &Path, module_name: &str, entities: &mut Vec<EntityInfo>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 递归子目录（如 extend/）
                scan_entity_dir_recursive(&path, module_name, entities);
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                    // 跳过 mod.rs 和 prelude.rs
                    if file_name == "mod" || file_name == "prelude" {
                        continue;
                    }
                    if let Ok(content) = fs::read_to_string(&path) {
                        let entity = parse_entity_content(&content, &file_name, module_name);
                        entities.push(entity);
                    }
                }
            }
        }
    }
}

/// 查找内容中的 Relation 枚举块
fn find_relation_enum(content: &str) -> String {
    let mut result = String::new();
    let mut in_enum = false;
    let mut brace_count = 0;

    for line in content.lines() {
        if line.contains("enum Relation") {
            in_enum = true;
        }
        if in_enum {
            result.push_str(line);
            result.push('\n');
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;
            if brace_count <= 0 && result.contains('{') {
                break;
            }
        }
    }

    result
}

/// 解析 Relation 枚举，提取结构化关系信息
fn parse_relation_enum(content: &str, _table_name: &str, module_name: &str) -> Vec<RelationInfo> {
    let enum_content = find_relation_enum(content);
    if enum_content.is_empty() {
        return Vec::new();
    }

    let mut relations = Vec::new();
    let mut current_attrs = String::new();
    let mut in_attr_block = false;
    let mut paren_depth = 0;

    for line in enum_content.lines() {
        let trimmed = line.trim();

        // 进入属性块
        if trimmed.contains("#[sea_orm(") {
            in_attr_block = true;
            paren_depth += trimmed.matches('(').count() as i32;
            paren_depth -= trimmed.matches(')').count() as i32;
            current_attrs.push_str(trimmed);
            if paren_depth <= 0 {
                // 单行属性
                if let Some(rel) = parse_single_relation(&current_attrs, module_name) {
                    relations.push(rel);
                }
                current_attrs.clear();
                in_attr_block = false;
            }
            continue;
        }

        if in_attr_block {
            current_attrs.push_str(trimmed);
            paren_depth += trimmed.matches('(').count() as i32;
            paren_depth -= trimmed.matches(')').count() as i32;
            if paren_depth <= 0 {
                if let Some(rel) = parse_single_relation(&current_attrs, module_name) {
                    relations.push(rel);
                }
                current_attrs.clear();
                in_attr_block = false;
            }
            continue;
        }

        // 跳过属性块结束后的无关行（变体名由 attr 块中的 split 提取）
    }

    // 过滤掉空的关系
    relations.retain(|r| !r.variant_name.is_empty());
    relations
}

/// 解析单行/多行 #[sea_orm()] 属性，提取关系信息
fn parse_single_relation(attr: &str, module_name: &str) -> Option<RelationInfo> {
    let attr_owned = attr.replace('\n', "").replace("\\s+", " ");
    let attr_str = attr_owned.as_str();

    // 提取 variant name — 在 ] 后的下一行
    let variant = attr_str
        .split(']')
        .next_back()?
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string();

    if attr_str.contains("belongs_to") {
        // 提取目标 entity
        let target = extract_quoted_value(attr_str, "belongs_to = ")?;
        let target_entity = target
            .strip_prefix("super::")
            .unwrap_or(&target)
            .strip_suffix("::Entity")
            .unwrap_or(&target)
            .to_string();

        // 提取 from 字段
        let from = extract_quoted_value(attr_str, "from = ");
        let fk_field = from
            .as_ref()
            .and_then(|f| f.strip_prefix("Column::").map(to_snake_case));

        // 提取 to 字段（目标主键）
        let to = extract_quoted_value(attr_str, "to = ");
        let _target_pk = to.as_ref().and_then(|t| {
            let column_name = t
                .strip_prefix("super::")
                .and_then(|s| {
                    // super::entity::Column::Field → Column::Field
                    s.split("Column::").nth(1)
                })
                .or_else(|| t.strip_prefix("Column::"))
                .map(|c| c.to_string());
            column_name
        });

        Some(RelationInfo {
            kind: RelationKind::BelongsTo,
            target_module: module_name.to_string(),
            target_entity: target_entity.clone(),
            target_model: pascal_case(&target_entity),
            variant_name: if variant.is_empty() {
                pascal_case(&target_entity)
            } else {
                variant
            },
            display_field: None, // 需要额外推断
            fk_field,
            fk_in_target: None,
        })
    } else if attr_str.contains("has_many") {
        let target = extract_quoted_value(attr_str, "has_many = ")?;
        let target_entity = target
            .strip_prefix("super::")
            .unwrap_or(&target)
            .strip_suffix("::Entity")
            .unwrap_or(&target)
            .to_string();

        Some(RelationInfo {
            kind: RelationKind::HasMany,
            target_module: module_name.to_string(),
            target_entity: target_entity.clone(),
            target_model: pascal_case(&target_entity),
            variant_name: if variant.is_empty() {
                pascal_case(&target_entity)
            } else {
                variant
            },
            display_field: None,
            fk_field: None,
            fk_in_target: None,
        })
    } else {
        None
    }
}

/// 从属性字符串中提取引号内的值
fn extract_quoted_value(s: &str, prefix: &str) -> Option<String> {
    let start = s.find(prefix)?;
    let after_prefix = &s[start + prefix.len()..];
    // 查找引号
    let quote_start = after_prefix.find('"')?;
    let after_quote = &after_prefix[quote_start + 1..];
    let quote_end = after_quote.find('"')?;
    Some(after_quote[..quote_end].to_string())
}

/// 推断目标实体的显示字段名
fn infer_display_field(target_name: &str) -> String {
    // 常见的实体名称 → 显示字段映射
    let display_map = [
        ("dept", "dept_name"),
        ("role", "role_name"),
        ("user", "user_name"),
        ("menu", "name"),
        ("post", "post_name"),
        ("dict", "dict_label"),
        ("dict_type", "name"),
        ("job", "job_name"),
        ("category", "name"),
    ];
    for (key, field) in &display_map {
        if target_name == *key || target_name.ends_with(key) {
            return field.to_string();
        }
    }
    // 默认使用 name
    "name".to_string()
}

fn find_model_struct(content: &str) -> String {
    let mut result = String::new();
    let mut in_struct = false;
    let mut brace_count = 0;

    for line in content.lines() {
        if line.contains("pub struct Model") {
            in_struct = true;
        }
        if in_struct {
            result.push_str(line);
            result.push('\n');
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;
            if brace_count <= 0 && result.contains('{') {
                break;
            }
        }
    }

    result
}

fn parse_field_line(line: &str) -> Option<FieldInfo> {
    // 匹配 pub field_name: Type 或 pub field_name: Option<Type>
    let line = line.trim().trim_end_matches(',');

    if !line.starts_with("pub ") {
        return None;
    }

    // 去掉 pub 前缀
    let rest = line.strip_prefix("pub ")?;

    // 分割字段名和类型
    let parts: Vec<&str> = rest.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }

    let name = parts[0].trim().to_string();
    let type_str = parts[1].trim().to_string();

    let is_optional = type_str.starts_with("Option<");
    let base_type = if is_optional {
        type_str
            .strip_prefix("Option<")
            .unwrap_or(&type_str)
            .strip_suffix('>')
            .unwrap_or(&type_str)
            .to_string()
    } else {
        type_str.clone()
    };

    let is_datetime =
        base_type == "DateTime" || base_type == "DateTimeUtc" || base_type == "NaiveDateTime";
    let is_id_field = base_type == "i64";

    // 推断前端组件类型
    let frontend_component = infer_frontend_component(&base_type, &name);

    let pascal_name = pascal_case(&name);
    let camel_name = camel_case(&name);

    Some(FieldInfo {
        name,
        rust_type: type_str,
        is_primary_key: false,
        is_optional,
        base_type,
        pascal_name,
        camel_name,
        is_id_field,
        is_datetime,
        frontend_component,
        show_in_list: false,
        show_in_search: false,
        show_in_form: false,
        show_in_detail: false,
        sortable: false,
        sort_priority: 0,
        sort_order: String::from("desc"),
        component_override: None,
        label: None,
        relation: None,
    })
}

fn get_next_line<'a>(content: &'a str, current_line: &str) -> &'a str {
    let mut found = false;
    for line in content.lines() {
        if found {
            return line;
        }
        if line.trim() == current_line.trim() {
            found = true;
        }
    }
    ""
}

fn infer_frontend_component(base_type: &str, name: &str) -> String {
    if name.ends_with("_at") || name == "created_at" || name == "updated_at" {
        return "DatePicker".to_string();
    }
    // 字段名包含图片相关关键字时返回 Upload 组件
    let image_keywords = [
        "image",
        "img",
        "avatar",
        "cover",
        "pic",
        "photo",
        "thumbnail",
    ];
    if image_keywords.iter().any(|kw| name.contains(kw)) {
        return "Upload".to_string();
    }
    // 字段名为 password 时返回 InputPassword 组件
    if name == "password" {
        return "InputPassword".to_string();
    }
    // 外键字段（以 _id 结尾）使用 Select 下拉选择组件
    if name.ends_with("_id") {
        return "Select".to_string();
    }
    // 常见枚举/字典字段 → Select
    let enum_names = [
        "status", "type", "type_", "kind", "category", "level", "state", "mode", "priority",
        "severity", "grade", "class",
    ];
    if enum_names
        .iter()
        .any(|kw| name == *kw || name.ends_with(kw))
    {
        return "Select".to_string();
    }
    // 性别 → Radio
    if name == "gender" || name == "sex" {
        return "Radio".to_string();
    }
    // 排序/权重 → InputNumber
    if name == "sort"
        || name == "sort_order"
        || name == "order_num"
        || name == "weight"
        || name == "priority"
    {
        return "InputNumber".to_string();
    }
    match base_type {
        "i32" | "u32" => "InputNumber".to_string(),
        "i64" | "u64" => "Input".to_string(),
        "f32" | "f64" => "InputNumber".to_string(),
        "bool" => "Switch".to_string(),
        "String" => {
            let comp = if ["remark", "content", "description"]
                .iter()
                .any(|k| name.contains(k))
            {
                "InputTextarea"
            } else {
                "Input"
            }
            .to_string();
            comp
        }
        "DateTime" | "NaiveDateTime" | "DateTimeUtc" => "DatePicker".to_string(),
        _ => "Input".to_string(),
    }
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

pub fn camel_case(s: &str) -> String {
    let pascal = pascal_case(s);
    if pascal.is_empty() {
        return pascal;
    }
    let mut c = pascal.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

/// PascalCase → snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entity() {
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
        assert_eq!(entity.table_name, "test_api");
        assert_eq!(entity.model_name, "TestApi");
        assert_eq!(entity.primary_key, "id");
        assert_eq!(entity.fields.len(), 6); // id 只出现一次（已去重）
        assert!(entity.fields[0].is_primary_key);
        assert_eq!(entity.fields[1].name, "name");
    }

    #[test]
    fn test_parse_entity_with_relation() {
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
    pub content: String,
    pub author: Option<String>,
    pub created_at: Option<DateTime>,
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

        // 验证字段数量 (id 只出现一次，已去重)
        assert_eq!(entity.fields.len(), 6);
        tracing::info!(
            "fields: {:?}",
            entity.fields.iter().map(|f| &f.name).collect::<Vec<_>>()
        );

        // 验证 id 字段
        let id_field = entity.fields.iter().find(|f| f.name == "id").unwrap();
        assert!(id_field.is_primary_key);
        assert!(id_field.is_id_field);

        // 验证 category_id 字段 → 应有 Relation
        let cat_field = entity
            .fields
            .iter()
            .find(|f| f.name == "category_id")
            .unwrap();
        assert!(cat_field.relation.is_some(), "category_id 应该有关联关系");
        let rel = cat_field.relation.as_ref().unwrap();
        assert!(matches!(rel.kind, RelationKind::BelongsTo));
        assert_eq!(rel.target_entity, "test_category");
        assert_eq!(rel.target_model, "TestCategory");
        assert_eq!(
            rel.fk_field.as_deref(),
            Some("category_id"),
            "fk_field 应该是 category_id"
        );

        // 验证 belongs_to_fields
        assert_eq!(entity.belongs_to_fields.len(), 1);
        assert_eq!(entity.belongs_to_fields[0].name, "category_id");
        assert!(entity.belongs_to_fields[0].relation.is_some());

        // 验证 frontend_component 被设为 Select
        assert_eq!(cat_field.frontend_component, "Select");
    }
}
