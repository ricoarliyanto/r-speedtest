/// Args 文件模板 (a{table_name}.rs)
pub const TEMPLATE_ARGS: &str = r#"use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct {{ model_name }}Resp {
{%- for field in resp_fields %}
{%- if field.base_type == "i64" %}
    {%- if "Vec<" in field.rust_type %}
    {%- if field.is_optional %}
    #[serde(with = "option_veci64_to_vecstring")]
    {%- else %}
    #[serde(with = "veci64_to_vecstring")]
    {%- endif %}
    {%- elif field.is_optional %}
    #[serde(with = "option_string_or_i64")]
    {%- else %}
    #[serde(with = "i64_to_string")]
    {%- endif %}
{%- endif %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endfor %}
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {{ model_name }}Search {
{%- for field in search_fields %}
{%- if field.base_type == "i64" %}
    #[serde(with = "option_string_or_i64")]
{%- endif %}
    pub {{ field.name }}: Option<{{ field.base_type }}>,
{%- endfor %}
{%- if search_fields | length == 0 %}
    // 无搜索字段时保留一个占位
{%- endif %}
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {{ model_name }}Add {
{%- for field in add_fields %}
{%- if field.base_type == "i64" %}
    {%- if "Vec<" in field.rust_type %}
    {%- if field.is_optional %}
    #[serde(with = "option_veci64_to_vecstring")]
    {%- else %}
    #[serde(with = "veci64_to_vecstring")]
    {%- endif %}
    {%- elif field.is_optional %}
    #[serde(with = "option_string_or_i64")]
    {%- else %}
    #[serde(with = "i64_to_string")]
    {%- endif %}
{%- endif %}
{%- if field.is_optional %}
    pub {{ field.name }}: Option<{{ field.base_type }}>,
{%- else %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endif %}
{%- endfor %}
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {{ model_name }}Edit {
    #[serde(with = "i64_to_string")]
    pub {{ primary_key }}: i64,
{%- for field in edit_fields %}
{%- if field.base_type == "i64" %}
    {%- if "Vec<" in field.rust_type %}
    {%- if field.is_optional %}
    #[serde(with = "option_veci64_to_vecstring")]
    {%- else %}
    #[serde(with = "veci64_to_vecstring")]
    {%- endif %}
    {%- elif field.is_optional %}
    #[serde(with = "option_string_or_i64")]
    {%- else %}
    #[serde(with = "i64_to_string")]
    {%- endif %}
{%- endif %}
{%- if field.is_optional %}
    pub {{ field.name }}: Option<{{ field.base_type }}>,
{%- else %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endif %}
{%- endfor %}
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {{ model_name }}Del {
    #[serde(with = "i64_to_string")]
    pub {{ primary_key }}: i64,
}
"#;

/// Model 文件模板 (m{table_name}.rs)
pub const TEMPLATE_MODEL: &str = r#"pub use super::args::{{ args_file_name }}::*;
pub use super::entity::{{ entity_mod_name }}::{self, ActiveModel, Model as {{ model_alias }}};
use crate::model::prelude::*;

impl {{ model_alias }} {
    pub async fn list(arg: PageParams, search: {{ search_name }}) -> Result<ListData<{{ resp_name }}>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = {{ entity_mod_name }}::Entity::find();
{%- for field in search_fields %}
        if let Some({{ field.name }}) = search.{{ field.name }} {
            rmodel = rmodel.filter({{ entity_mod_name }}::Column::{{ field.pascal_name }}{%- if field.base_type == "String" %}.contains({{ field.name }}){% else %}.eq({{ field.name }}){%- endif %});
        }
{%- endfor %}
        let total = rmodel.clone().count(db).await?;
        // 排序
        {% if has_default_sort_field %}{{default_sort_chain}}{% elif has_updated_at %}rmodel = rmodel.order_by_desc({{ entity_mod_name }}::Column::UpdatedAt);{% endif %}
        let paginator = rmodel
            .into_model::<{{ resp_name }}>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    pub async fn add(arg: {{ add_name }}) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let model = {{ entity_mod_name }}::ActiveModel {
            {{ primary_key }}: Set(id),
{%- for field in add_fields %}
            {{ field.name }}: Set(arg.{{ field.name }}),
{%- endfor %}
            ..Default::default()
        };
        {{ entity_mod_name }}::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with {{ primary_key }}: {}", id))
    }

    pub async fn edit(arg: {{ edit_name }}) -> Result<String> {
        let db = DB().await;
        let model = {{ entity_mod_name }}::Entity::find_by_id(arg.{{ primary_key }}).one(db).await?;
        if let Some(model) = model {
            let mut active_model: {{ entity_mod_name }}::ActiveModel = model.into();
{%- for field in edit_fields %}
            active_model.{{ field.name }} = Set(arg.{{ field.name }});
{%- endfor %}
{%- if has_updated_at %}
{%- if is_updated_at_optional %}
            active_model.updated_at = Set(Some(Local::now().naive_local()));
{%- else %}
            active_model.updated_at = Set(Local::now().naive_local());
{%- endif %}
{%- endif %}
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }

    pub async fn del(arg: {{ del_name }}) -> Result<String> {
        let db = DB().await;
        let result = {{ entity_mod_name }}::Entity::delete_by_id(arg.{{ primary_key }}).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
"#;

/// Service 文件模板 (s_{table_name}.rs)
pub const TEMPLATE_SERVICE: &str = r#"use crate::model::{{ module_name }}::model::{{ model_file_name }}::{
    {{ add_name }}, {{ del_name }}, {{ edit_name }}, {{ model_alias }}, {{ search_name }},
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<{{ search_name }}>,
) -> impl IntoResponse {
    let rlist = {{ model_alias }}::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<{{ edit_name }}>) -> impl IntoResponse {
    let r = {{ model_alias }}::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn add(VJson(arg): VJson<{{ add_name }}>) -> impl IntoResponse {
    let r = {{ model_alias }}::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg): VQuery<{{ del_name }}>) -> impl IntoResponse {
    let r = {{ model_alias }}::del(arg).await;
    ApiResponse::from_result(r)
}
"#;

/// Model 文件模板 (m{table_name}.rs) — with data scope
pub const TEMPLATE_MODEL_DATA_SCOPE: &str = r#"pub use super::args::{{ args_file_name }}::*;
pub use super::entity::{{ entity_mod_name }}::{self, ActiveModel, Model as {{ model_alias }}};
use crate::model::prelude::*;
use crate::model::sys::entity::sys_dept;
use crate::service::data_scope::{check_record_scope, DataScopeContext, RecordOp, RecordScope};

impl {{ model_alias }} {
    pub async fn list(
        arg: PageParams,
        search: {{ search_name }},
        userinfo: UserInfo,
    ) -> Result<ListData<{{ resp_name }}>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = {{ entity_mod_name }}::Entity::find();

        let scope = DataScopeContext::from_user_id(db, userinfo.uid).await?;

        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to({{ entity_mod_name }}::Entity)
                .from(sys_dept::Column::DeptId)
                .to({{ entity_mod_name }}::Column::DeptId)
                .into(),
        );
        if let Some(cond) = scope.to_scope_condition(
            Some({{ entity_mod_name }}::Column::DeptId),
            Some({{ entity_mod_name }}::Column::OwnerId),
            userinfo.uid,
        ) {
            rmodel = rmodel.filter(cond);
        }
{%- for field in search_fields %}
        if let Some({{ field.name }}) = search.{{ field.name }} {
            rmodel = rmodel.filter({{ entity_mod_name }}::Column::{{ field.pascal_name }}{%- if field.base_type == "String" %}.contains({{ field.name }}){% else %}.eq({{ field.name }}){%- endif %});
        }
{%- endfor %}
        // 排序
        {% if has_default_sort_field %}{{default_sort_chain}}{% elif has_updated_at %}rmodel = rmodel.order_by_desc({{ entity_mod_name }}::Column::UpdatedAt);{% endif %}
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .into_model::<{{ resp_name }}>()
            .paginate(db, page_per_size);

        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    pub async fn add(arg: {{ add_name }}, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();

        let rec_scope = check_record_scope(
            db,
            &userinfo,
            RecordOp::Create {
                req_dept_id: Some(arg.dept_id),
                req_owner_id: Some(arg.owner_id),
            },
        )
        .await?;

        let model = {{ entity_mod_name }}::ActiveModel {
            {{ primary_key }}: Set(id),
{%- for field in add_fields %}
{%- if field.name == "dept_id" %}
            dept_id: Set(rec_scope.dept_id),
{%- elif field.name == "owner_id" %}
            owner_id: Set(rec_scope.owner_id),
{%- else %}
            {{ field.name }}: Set(arg.{{ field.name }}),
{%- endif %}
{%- endfor %}
            ..Default::default()
        };
        {{ entity_mod_name }}::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with {{ primary_key }}: {}", id))
    }

    pub async fn edit(arg: {{ edit_name }}, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let model = {{ entity_mod_name }}::Entity::find_by_id(arg.{{ primary_key }}).one(db).await?;
        let Some(model) = model else {
            return Err("Record not found".into());
        };
        let old_scope = RecordScope {
            dept_id: model.dept_id,
            owner_id: model.owner_id,
        };

        let rec_scope = check_record_scope(
            db,
            &userinfo,
            RecordOp::Update {
                old: old_scope,
                new_dept_id: Some(arg.dept_id),
                new_owner_id: Some(arg.owner_id),
            },
        )
        .await?;

        let mut active_model: {{ entity_mod_name }}::ActiveModel = model.into();
{%- for field in edit_fields %}
{%- if field.name == "dept_id" %}
        active_model.dept_id = Set(rec_scope.dept_id);
{%- elif field.name == "owner_id" %}
        active_model.owner_id = Set(rec_scope.owner_id);
{%- else %}
        active_model.{{ field.name }} = Set(arg.{{ field.name }});
{%- endif %}
{%- endfor %}
{%- if has_updated_at %}
{%- if is_updated_at_optional %}
        active_model.updated_at = Set(Some(Local::now().naive_local()));
{%- else %}
        active_model.updated_at = Set(Local::now().naive_local());
{%- endif %}
{%- endif %}
        let _ = active_model.update(db).await?;
        Ok("Successfully updated record".to_string())
    }

    pub async fn del(arg: {{ del_name }}, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let model = {{ entity_mod_name }}::Entity::find_by_id(arg.{{ primary_key }}).one(db).await?;
        let Some(model) = model else {
            return Err("Record not found".into());
        };
        let old_scope = RecordScope {
            dept_id: model.dept_id,
            owner_id: model.owner_id,
        };
        check_record_scope(db, &userinfo, RecordOp::Delete { old: old_scope }).await?;
        {{ entity_mod_name }}::Entity::delete_by_id(arg.{{ primary_key }}).exec(db).await?;
        Ok("Successfully deleted record".to_string())
    }
}
"#;

/// Service 文件模板 (s_{table_name}.rs) — with data scope
pub const TEMPLATE_SERVICE_DATA_SCOPE: &str = r#"use crate::model::{{ module_name }}::model::{{ model_file_name }}::{
    {{ add_name }}, {{ del_name }}, {{ edit_name }}, {{ model_alias }}, {{ search_name }},
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<{{ search_name }}>,
    userinfo: UserInfo,
) -> impl IntoResponse {
    let rlist = {{ model_alias }}::list(arg, search, userinfo).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(
    userinfo: UserInfo,
    VJson(arg): VJson<{{ edit_name }}>,
) -> impl IntoResponse {
    let r = {{ model_alias }}::edit(arg, userinfo).await;
    ApiResponse::from_result(r)
}

pub async fn add(
    userinfo: UserInfo,
    VJson(arg): VJson<{{ add_name }}>,
) -> impl IntoResponse {
    let r = {{ model_alias }}::add(arg, userinfo).await;
    ApiResponse::from_result(r)
}

pub async fn delete(
    userinfo: UserInfo,
    VQuery(arg): VQuery<{{ del_name }}>,
) -> impl IntoResponse {
    let r = {{ model_alias }}::del(arg, userinfo).await;
    ApiResponse::from_result(r)
}
"#;

/// API 路由函数片段 (添加到 api/{module}.rs)
pub const TEMPLATE_API_ROUTE_FN: &str = r#"fn {{ module_name }}_{{ entity_mod_name }}() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取{{ model_name }}列表"), get({{ service_file_name }}::list))
        .route("/edit", WebPathType::Put, Some("编辑{{ model_name }}"), put({{ service_file_name }}::edit))
        .route("/add", WebPathType::Post, Some("添加{{ model_name }}"), post({{ service_file_name }}::add))
        .route("/del", WebPathType::Delete, Some("删除{{ model_name }}"), delete({{ service_file_name }}::delete))
}
"#;

/// API nest 路由片段
pub const TEMPLATE_API_NEST: &str = r#"            .nest("/{{ entity_mod_name }}", {{ module_name }}_{{ entity_mod_name }}())"#;

/// model/mod.rs 片段
pub const TEMPLATE_MODEL_MOD: &str = r#"pub mod {{ model_file_name }};
"#;

/// args/mod.rs 片段
pub const TEMPLATE_ARGS_MOD: &str = r#"pub mod {{ args_file_name }};
"#;

/// service/mod.rs 片段
pub const TEMPLATE_SERVICE_MOD: &str = r#"pub mod {{ service_file_name }};
"#;

/// model/{module}/mod.rs 完整模板
pub const TEMPLATE_MODULE_MOD: &str = r#"pub mod entity;
pub mod model;
pub mod args;
"#;

/// model/{module}/args/mod.rs 完整模板
pub const TEMPLATE_ARGS_MOD_FULL: &str = r#"pub mod {{ args_file_name }};
"#;

/// model/{module}/model/mod.rs 完整模板
pub const TEMPLATE_MODEL_MOD_FULL: &str = r#"pub use super::entity as entity;
pub use super::args as args;
pub mod {{ model_file_name }};
"#;

// ===================== 前端 Vue 模板 =====================
// 前端模板使用独立的 .tera 文件，通过 Tera::new() 从文件加载
// 在 .tera 文件中，Vue 的 {{ }} 用 {{ '{' }} 和 {{ '}' }} 输出，避免与 Tera 语法冲突
// 文件位于: src/common/codegen/tera_templates/
//   - frontend_view.tera  (列表页)
//   - frontend_write.tera (表单组件)
//   - frontend_api.tera   (API index.ts)
